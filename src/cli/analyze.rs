use std::time::Instant;

use crate::{audio, db, store, tools};

use super::{CliCacheWriteMsg, cache_probe_for_path, cache_status_for_track};

#[derive(clap::Args)]
pub(crate) struct AnalyzeArgs {
    /// Filter by playlist ID
    #[arg(long)]
    playlist: Option<String>,
    /// Filter by artist name (partial match)
    #[arg(long)]
    artist: Option<String>,
    /// Filter by genre name (partial match)
    #[arg(long)]
    genre: Option<String>,
    /// Minimum BPM
    #[arg(long)]
    bpm_min: Option<f64>,
    /// Maximum BPM
    #[arg(long)]
    bpm_max: Option<f64>,
    /// Filter by musical key
    #[arg(long)]
    key: Option<String>,
    /// Filter by label name (partial match)
    #[arg(long)]
    label: Option<String>,
    /// Filter by file path/folder (partial match)
    #[arg(long)]
    path: Option<String>,
    /// Search query matching title or artist
    #[arg(long)]
    query: Option<String>,
    /// Only tracks added on or after this date (ISO date)
    #[arg(long)]
    added_after: Option<String>,
    /// Only tracks added on or before this date (ISO date)
    #[arg(long)]
    added_before: Option<String>,
    /// Minimum star rating (1-5)
    #[arg(long)]
    rating_min: Option<u8>,
    /// Max tracks to process
    #[arg(long, default_value = "200")]
    max_tracks: u32,
    /// Don't skip already-cached tracks
    #[arg(long)]
    no_skip_cached: bool,
    /// Skip Essentia analysis, only run stratum-dsp
    #[arg(long)]
    stratum_only: bool,
    /// Max concurrent track analyses (default: half CPU cores, min 2, max 16)
    #[arg(long, short = 'j')]
    concurrency: Option<u32>,
}

pub(crate) async fn run_analyze(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Open Rekordbox DB
    let db_path = db::resolve_db_path().ok_or(
        "Cannot find Rekordbox database. Set REKORDBOX_DB_PATH or ensure Rekordbox is installed.",
    )?;
    let conn = db::open(&db_path)?;

    // Open internal store (cache)
    let store_path = store::default_path();
    let store_path_str = store_path
        .to_str()
        .ok_or("Invalid store path encoding")?
        .to_string();
    let store_conn = store::open(&store_path_str)?;

    // Probe essentia
    let essentia_python = if args.stratum_only {
        None
    } else {
        tools::probe_essentia_python_path()
    };

    tracing::info!(
        "Essentia: {}",
        if args.stratum_only {
            "skipped (--stratum-only)".to_string()
        } else {
            match &essentia_python {
                Some(p) => format!("available ({p})"),
                None => "not found (stratum-dsp only)".to_string(),
            }
        }
    );

    // Search tracks
    let params = db::SearchParams {
        query: args.query,
        artist: args.artist,
        genre: args.genre,
        rating_min: args.rating_min,
        bpm_min: args.bpm_min,
        bpm_max: args.bpm_max,
        key: args.key,
        playlist: args.playlist,
        has_genre: None,
        has_label: None,
        label: args.label,
        path: args.path,
        path_prefix: None,
        added_after: args.added_after,
        added_before: args.added_before,
        exclude_samples: true,
        limit: Some(args.max_tracks),
        offset: None,
    };
    let tracks = db::search_tracks_unbounded(&conn, &params)?;

    if tracks.is_empty() {
        tracing::info!("No tracks match the given filters.");
        return Ok(());
    }

    // Pre-filter: check cache for each track
    let skip_cached = !args.no_skip_cached;
    let mut to_analyze = Vec::new();
    let mut cached_count = 0;

    for track in &tracks {
        let cache_probe = cache_probe_for_path(&track.file_path, skip_cached);
        let (has_stratum, has_essentia) = cache_status_for_track(
            &store_conn,
            cache_probe.as_ref(),
            skip_cached,
            essentia_python.is_some(),
        )?;

        if has_stratum && has_essentia {
            cached_count += 1;
        } else {
            to_analyze.push((track.clone(), !has_stratum, !has_essentia));
        }
    }

    let total = tracks.len();
    let pending = to_analyze.len();

    // Compute concurrency
    let concurrency = match args.concurrency {
        Some(n) => n.clamp(1, 16),
        None => {
            let cpus = std::thread::available_parallelism()
                .map(|n| n.get() as u32)
                .unwrap_or(4);
            (cpus.saturating_sub(2)).clamp(2, 16)
        }
    } as usize;

    tracing::info!(
        "Scanning {total} tracks ({cached_count} cached, {pending} to analyze, concurrency={concurrency})"
    );

    if to_analyze.is_empty() {
        tracing::info!("All tracks already cached. Nothing to do.");
        return Ok(());
    }

    // Drop pre-filter connection — writer task will open its own
    drop(store_conn);

    let batch_start = Instant::now();
    let analyzed = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
    let failed = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
    let completed_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    // Spawn cache writer task
    let (cache_tx, mut cache_rx) = tokio::sync::mpsc::channel::<CliCacheWriteMsg>(concurrency * 4);
    let writer_store_path = store_path_str.clone();
    let writer_handle = tokio::task::spawn_blocking(move || {
        let conn = match store::open(&writer_store_path) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Cache writer: failed to open store: {e}");
                return;
            }
        };
        while let Some(msg) = cache_rx.blocking_recv() {
            if let Err(e) = store::set_audio_analysis(
                &conn,
                &msg.file_path,
                &msg.analyzer,
                msg.file_size,
                msg.file_mtime,
                &msg.analyzer_version,
                &msg.features_json,
            ) {
                tracing::error!(
                    "Cache writer: failed to write {} for {}: {e}",
                    msg.analyzer,
                    msg.file_path
                );
            }
        }
    });

    // Spawn analysis tasks bounded by semaphore
    let sem = std::sync::Arc::new(tokio::sync::Semaphore::new(concurrency));
    let mut handles = Vec::with_capacity(pending);

    for (track, needs_stratum, needs_essentia) in to_analyze {
        let permit = sem.clone().acquire_owned().await?;
        let label = format!("{} - {}", track.artist, track.title);
        let essentia_python = essentia_python.clone();
        let cache_tx = cache_tx.clone();
        let analyzed = analyzed.clone();
        let failed = failed.clone();
        let completed_count = completed_count.clone();

        handles.push(tokio::spawn(async move {
            let result = cli_analyze_single_track(
                &track.file_path,
                needs_stratum,
                needs_essentia,
                essentia_python.as_deref(),
                &cache_tx,
            )
            .await;

            let idx = completed_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

            match result {
                Ok(outcome) => {
                    let elapsed = outcome.elapsed;
                    match outcome.kind {
                        CliTrackOutcome::StratumAndEssentia {
                            bpm,
                            key_camelot,
                            essentia_ok,
                        } => {
                            let essentia_status = if essentia_ok {
                                " +essentia"
                            } else {
                                " (essentia failed)"
                            };
                            tracing::info!(
                                "[{idx}/{pending}] {label} ... BPM={bpm:.1} Key={key_camelot}{essentia_status} ({elapsed:.1}s)"
                            );
                            if essentia_ok {
                                analyzed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            } else {
                                failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                        }
                        CliTrackOutcome::StratumOnly { bpm, key_camelot } => {
                            tracing::info!(
                                "[{idx}/{pending}] {label} ... BPM={bpm:.1} Key={key_camelot} ({elapsed:.1}s)"
                            );
                            analyzed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        CliTrackOutcome::EssentiaOnly { ok } => {
                            if ok {
                                tracing::info!(
                                    "[{idx}/{pending}] {label} ... +essentia ({elapsed:.1}s)"
                                );
                                analyzed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            } else {
                                tracing::error!(
                                    "[{idx}/{pending}] FAIL {label}: Essentia error ({elapsed:.1}s)"
                                );
                                failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                            }
                        }
                    }
                }
                Err(msg) => {
                    tracing::warn!("[{idx}/{pending}] SKIP {label}: {msg}");
                    failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            }

            drop(permit);
        }));
    }

    // Await all tasks
    for handle in handles {
        let _ = handle.await;
    }

    // Shut down writer
    drop(cache_tx);
    let _ = writer_handle.await;

    let analyzed = analyzed.load(std::sync::atomic::Ordering::Relaxed);
    let failed = failed.load(std::sync::atomic::Ordering::Relaxed);
    let total_time = batch_start.elapsed();
    let mins = total_time.as_secs() / 60;
    let secs = total_time.as_secs() % 60;
    tracing::info!("Done: {analyzed} analyzed, {failed} failed ({mins}m {secs}s)");

    Ok(())
}

enum CliTrackOutcome {
    StratumAndEssentia {
        bpm: f64,
        key_camelot: String,
        essentia_ok: bool,
    },
    StratumOnly {
        bpm: f64,
        key_camelot: String,
    },
    EssentiaOnly {
        ok: bool,
    },
}

struct CliTrackResult {
    kind: CliTrackOutcome,
    elapsed: f64,
}

async fn cli_analyze_single_track(
    raw_file_path: &str,
    needs_stratum: bool,
    needs_essentia: bool,
    essentia_python: Option<&str>,
    cache_tx: &tokio::sync::mpsc::Sender<CliCacheWriteMsg>,
) -> Result<CliTrackResult, String> {
    let file_path =
        audio::resolve_audio_path(raw_file_path).map_err(|_| "File not found".to_string())?;
    let metadata = std::fs::metadata(&file_path).map_err(|e| format!("Cannot stat file: {e}"))?;
    let file_size = metadata.len() as i64;
    let file_mtime = super::file_mtime_unix(&metadata);
    let track_start = Instant::now();

    if needs_stratum {
        // Decode + analyze stratum
        let path_clone = file_path.clone();
        let (samples, sample_rate) =
            tokio::task::spawn_blocking(move || audio::decode_to_samples(&path_clone))
                .await
                .map_err(|e| format!("Decode task failed: {e}"))?
                .map_err(|e| format!("Decode error: {e}"))?;

        let stratum_result =
            tokio::task::spawn_blocking(move || audio::analyze_with_stratum(&samples, sample_rate))
                .await
                .map_err(|e| format!("Analysis task failed: {e}"))?
                .map_err(|e| format!("Analysis error: {e}"))?;

        let features_json = serde_json::to_string(&stratum_result).unwrap_or_default();
        let _ = cache_tx
            .send(CliCacheWriteMsg {
                file_path: file_path.clone(),
                analyzer: audio::ANALYZER_STRATUM.to_string(),
                file_size,
                file_mtime,
                analyzer_version: audio::STRATUM_SCHEMA_VERSION.to_string(),
                features_json,
            })
            .await;

        if needs_essentia && let Some(python) = essentia_python {
            let essentia_ok =
                cli_run_and_send_essentia(python, &file_path, file_size, file_mtime, cache_tx)
                    .await;
            return Ok(CliTrackResult {
                kind: CliTrackOutcome::StratumAndEssentia {
                    bpm: stratum_result.bpm,
                    key_camelot: stratum_result.key_camelot,
                    essentia_ok,
                },
                elapsed: track_start.elapsed().as_secs_f64(),
            });
        }

        Ok(CliTrackResult {
            kind: CliTrackOutcome::StratumOnly {
                bpm: stratum_result.bpm,
                key_camelot: stratum_result.key_camelot,
            },
            elapsed: track_start.elapsed().as_secs_f64(),
        })
    } else if needs_essentia {
        if let Some(python) = essentia_python {
            let ok = cli_run_and_send_essentia(python, &file_path, file_size, file_mtime, cache_tx)
                .await;
            return Ok(CliTrackResult {
                kind: CliTrackOutcome::EssentiaOnly { ok },
                elapsed: track_start.elapsed().as_secs_f64(),
            });
        }
        Err("Essentia not available".to_string())
    } else {
        Err("Nothing to analyze".to_string())
    }
}

async fn cli_run_and_send_essentia(
    python: &str,
    file_path: &str,
    file_size: i64,
    file_mtime: i64,
    cache_tx: &tokio::sync::mpsc::Sender<CliCacheWriteMsg>,
) -> bool {
    match audio::run_essentia(python, file_path)
        .await
        .map_err(|e| e.to_string())
    {
        Ok(features) => {
            let features_json = serde_json::to_string(&features).unwrap_or_default();
            let _ = cache_tx
                .send(CliCacheWriteMsg {
                    file_path: file_path.to_string(),
                    analyzer: audio::ANALYZER_ESSENTIA.to_string(),
                    file_size,
                    file_mtime,
                    analyzer_version: audio::ESSENTIA_SCHEMA_VERSION.to_string(),
                    features_json,
                })
                .await;
            true
        }
        Err(e) => {
            tracing::error!("Essentia error for {file_path}: {e}");
            false
        }
    }
}

#[cfg(test)]
pub(super) fn handle_decode_result(
    decode_result: Result<Result<(Vec<f32>, u32), audio::AudioError>, tokio::task::JoinError>,
    track_index: usize,
    pending: usize,
    label: &str,
    failed: &mut u32,
) -> Option<(Vec<f32>, u32)> {
    match decode_result {
        Ok(Ok(value)) => Some(value),
        Ok(Err(e)) => {
            tracing::error!("[{track_index}/{pending}] FAIL {label}: Decode error: {e}");
            *failed += 1;
            None
        }
        Err(e) => {
            tracing::error!("[{track_index}/{pending}] FAIL {label}: Decode task failed: {e}");
            *failed += 1;
            None
        }
    }
}

#[cfg(test)]
pub(super) fn handle_analysis_result(
    analysis_result: Result<
        Result<audio::StratumResult, audio::AudioError>,
        tokio::task::JoinError,
    >,
    idx: usize,
    pending: usize,
    label: &str,
    failed: &mut u32,
) -> Option<audio::StratumResult> {
    match analysis_result {
        Ok(Ok(result)) => Some(result),
        Ok(Err(e)) => {
            tracing::error!("[{idx}/{pending}] FAIL {label}: Analysis error: {e}");
            *failed += 1;
            None
        }
        Err(e) => {
            tracing::error!("[{idx}/{pending}] FAIL {label}: Analysis task failed: {e}");
            *failed += 1;
            None
        }
    }
}

#[cfg(test)]
pub(super) fn mark_track_outcome(analyzed: &mut u32, failed: &mut u32, success: bool) {
    if success {
        *analyzed += 1;
    } else {
        *failed += 1;
    }
}
