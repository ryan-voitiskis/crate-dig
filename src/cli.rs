use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::Parser;

use crate::{audio, db, store, tags, tools};

#[derive(Parser)]
#[command(name = "reklawdbox")]
enum Cli {
    /// Batch audio analysis (stratum-dsp + Essentia)
    Analyze(AnalyzeArgs),
    /// Read metadata tags from audio files
    ReadTags(ReadTagsArgs),
    /// Write metadata tags to audio files
    WriteTags(WriteTagsArgs),
    /// Extract embedded cover art from an audio file
    ExtractArt(ExtractArtArgs),
    /// Embed cover art into audio files
    EmbedArt(EmbedArtArgs),
}

#[derive(clap::Args)]
struct AnalyzeArgs {
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

#[derive(clap::Args)]
struct ReadTagsArgs {
    /// Audio files or directories to read
    #[arg(required = true)]
    paths: Vec<String>,
    /// Only return these fields (comma-separated)
    #[arg(long, value_delimiter = ',')]
    fields: Option<Vec<String>>,
    /// Include cover art metadata
    #[arg(long)]
    cover_art: bool,
    /// Output as JSON (JSONL for multiple files)
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args)]
struct WriteTagsArgs {
    /// Audio file to write tags to
    #[arg(required = true)]
    path: String,
    #[arg(long)]
    artist: Option<String>,
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    album: Option<String>,
    #[arg(long)]
    album_artist: Option<String>,
    #[arg(long)]
    genre: Option<String>,
    #[arg(long)]
    year: Option<String>,
    #[arg(long)]
    track: Option<String>,
    #[arg(long)]
    disc: Option<String>,
    #[arg(long)]
    comment: Option<String>,
    #[arg(long)]
    publisher: Option<String>,
    #[arg(long)]
    bpm: Option<String>,
    #[arg(long)]
    key: Option<String>,
    #[arg(long)]
    composer: Option<String>,
    #[arg(long)]
    remixer: Option<String>,
    /// Preview changes without writing
    #[arg(long)]
    dry_run: bool,
    /// WAV tag targets: id3v2, riff_info (comma-separated, default: both)
    #[arg(long, value_delimiter = ',')]
    wav_targets: Option<Vec<String>>,
    /// Read tags from stdin as JSON (overrides individual field flags)
    #[arg(long)]
    json_input: bool,
    /// Output as JSON
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args)]
struct ExtractArtArgs {
    /// Audio file to extract art from
    #[arg(required = true)]
    path: String,
    /// Output path (default: cover.{ext} in same directory)
    #[arg(long)]
    output: Option<String>,
    /// Picture type (default: front_cover)
    #[arg(long, default_value = "front_cover")]
    picture_type: String,
    /// Output as JSON
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args)]
struct EmbedArtArgs {
    /// Image file to embed
    #[arg(required = true)]
    image: String,
    /// Audio files to embed art into
    #[arg(required = true)]
    targets: Vec<String>,
    /// Picture type (default: front_cover)
    #[arg(long, default_value = "front_cover")]
    picture_type: String,
    /// Output as JSON
    #[arg(long)]
    json: bool,
}

fn file_mtime_unix(metadata: &std::fs::Metadata) -> i64 {
    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn is_cache_fresh(
    cached: Option<&store::CachedAudioAnalysis>,
    file_size: i64,
    file_mtime: i64,
) -> bool {
    matches!(
        cached,
        Some(entry) if entry.file_size == file_size && entry.file_mtime == file_mtime
    )
}

fn cache_probe_for_path(file_path: &str, skip_cached: bool) -> Option<(String, i64, i64)> {
    if !skip_cached {
        return None;
    }

    match audio::resolve_audio_path(file_path) {
        Ok(path) => match std::fs::metadata(&path) {
            Ok(metadata) => Some((path, metadata.len() as i64, file_mtime_unix(&metadata))),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

fn has_fresh_cache_entry(
    store_conn: &rusqlite::Connection,
    cache_probe: Option<&(String, i64, i64)>,
    analyzer: &str,
) -> Result<bool, rusqlite::Error> {
    if let Some((cache_key, file_size, file_mtime)) = cache_probe {
        let cached = store::get_audio_analysis(store_conn, cache_key, analyzer)?;
        Ok(is_cache_fresh(cached.as_ref(), *file_size, *file_mtime))
    } else {
        Ok(false)
    }
}

fn cache_status_for_track(
    store_conn: &rusqlite::Connection,
    cache_probe: Option<&(String, i64, i64)>,
    skip_cached: bool,
    essentia_available: bool,
) -> Result<(bool, bool), rusqlite::Error> {
    let has_stratum = if skip_cached {
        has_fresh_cache_entry(store_conn, cache_probe, audio::ANALYZER_STRATUM)?
    } else {
        false
    };

    let has_essentia = if !essentia_available {
        true
    } else if skip_cached {
        has_fresh_cache_entry(store_conn, cache_probe, audio::ANALYZER_ESSENTIA)?
    } else {
        false
    };

    Ok((has_stratum, has_essentia))
}

#[cfg(test)]
fn handle_decode_result(
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
fn handle_analysis_result(
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
fn mark_track_outcome(analyzed: &mut u32, failed: &mut u32, success: bool) {
    if success {
        *analyzed += 1;
    } else {
        *failed += 1;
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli {
        Cli::Analyze(args) => run_analyze(args).await,
        Cli::ReadTags(args) => run_read_tags(args),
        Cli::WriteTags(args) => run_write_tags(args),
        Cli::ExtractArt(args) => run_extract_art(args),
        Cli::EmbedArt(args) => run_embed_art(args),
    }
}

async fn run_analyze(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
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
            (cpus / 2).clamp(2, 16)
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
    let (cache_tx, mut cache_rx) =
        tokio::sync::mpsc::channel::<CliCacheWriteMsg>(concurrency * 4);
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

struct CliCacheWriteMsg {
    file_path: String,
    analyzer: String,
    file_size: i64,
    file_mtime: i64,
    analyzer_version: String,
    features_json: String,
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
    let metadata =
        std::fs::metadata(&file_path).map_err(|e| format!("Cannot stat file: {e}"))?;
    let file_size = metadata.len() as i64;
    let file_mtime = file_mtime_unix(&metadata);
    let track_start = Instant::now();

    if needs_stratum {
        // Decode + analyze stratum
        let path_clone = file_path.clone();
        let (samples, sample_rate) =
            tokio::task::spawn_blocking(move || audio::decode_to_samples(&path_clone))
                .await
                .map_err(|e| format!("Decode task failed: {e}"))?
                .map_err(|e| format!("Decode error: {e}"))?;

        let stratum_result = tokio::task::spawn_blocking(move || {
            audio::analyze_with_stratum(&samples, sample_rate)
        })
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
                analyzer_version: stratum_result.analyzer_version.clone(),
                features_json,
            })
            .await;

        if needs_essentia
            && let Some(python) = essentia_python
        {
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
            let ok =
                cli_run_and_send_essentia(python, &file_path, file_size, file_mtime, cache_tx)
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
            let version = if features.analyzer_version.is_empty() {
                "unknown".to_string()
            } else {
                features.analyzer_version.clone()
            };
            let features_json = serde_json::to_string(&features).unwrap_or_default();
            let _ = cache_tx
                .send(CliCacheWriteMsg {
                    file_path: file_path.to_string(),
                    analyzer: audio::ANALYZER_ESSENTIA.to_string(),
                    file_size,
                    file_mtime,
                    analyzer_version: version,
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

// ---------------------------------------------------------------------------
// Audio file extension matching for directory scanning
// ---------------------------------------------------------------------------

fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| {
            crate::audio::AUDIO_EXTENSIONS.contains(&ext.to_ascii_lowercase().as_str())
        })
}

fn expand_paths(paths: &[String]) -> Vec<PathBuf> {
    let mut result = Vec::new();
    for p in paths {
        let path = PathBuf::from(p);
        if path.is_dir() {
            match std::fs::read_dir(&path) {
                Ok(entries) => {
                    let mut files: Vec<PathBuf> = entries
                        .filter_map(|e| e.ok())
                        .map(|e| e.path())
                        .filter(|p| p.is_file() && is_audio_file(p))
                        .collect();
                    files.sort();
                    result.extend(files);
                }
                Err(e) => {
                    tracing::warn!("Skipping unreadable directory {p}: {e}");
                }
            }
        } else {
            result.push(path);
        }
    }
    result
}

// ---------------------------------------------------------------------------
// read-tags
// ---------------------------------------------------------------------------

fn print_tags_human(result: &tags::FileReadResult) {
    match result {
        tags::FileReadResult::Single {
            path,
            format,
            tag_type,
            tags,
            cover_art,
        } => {
            tracing::info!("=== {} ({}) ===", path, format.to_uppercase());
            println!("{}:", tag_type);
            print_tag_map(tags, 2);
            if let Some(art) = cover_art {
                println!("  Cover Art    {} ({} bytes)", art.format, art.size_bytes);
            }
        }
        tags::FileReadResult::Wav {
            path,
            format,
            id3v2,
            riff_info,
            tag3_missing,
            cover_art,
        } => {
            tracing::info!("=== {} ({}) ===", path, format.to_uppercase());
            println!("ID3v2:");
            print_tag_map(id3v2, 2);
            println!();
            println!("RIFF INFO:");
            print_tag_map(riff_info, 2);
            if !tag3_missing.is_empty() {
                println!("  (not in RIFF INFO: {})", tag3_missing.join(", "));
            }
            if let Some(art) = cover_art {
                println!("  Cover Art    {} ({} bytes)", art.format, art.size_bytes);
            }
        }
        tags::FileReadResult::Error { path, error } => {
            tracing::error!("=== {} ===", path);
            tracing::error!("Error: {}", error);
        }
    }
}

/// Title-case a canonical field name for human output.
/// "album_artist" → "Album Artist", "bpm" → "Bpm", etc.
fn display_field_name(field: &str) -> String {
    field
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{}", chars.as_str())
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn print_tag_map(tags: &HashMap<String, Option<String>>, indent: usize) {
    let pad = " ".repeat(indent);
    // Print in canonical field order
    for &field in tags::ALL_FIELDS {
        if let Some(value) = tags.get(field) {
            let display = match value {
                Some(v) => v.as_str(),
                None => "(missing)",
            };
            println!("{pad}{:<14}{display}", display_field_name(field));
        }
    }
}

fn run_read_tags(args: ReadTagsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let files = expand_paths(&args.paths);
    if files.is_empty() {
        return Err("No audio files found.".into());
    }

    let fields_ref = args.fields.as_deref();
    let mut had_errors = false;

    for (i, file) in files.iter().enumerate() {
        let result = tags::read_file_tags(file, fields_ref, args.cover_art);
        if matches!(&result, tags::FileReadResult::Error { .. }) {
            had_errors = true;
        }
        if args.json {
            println!("{}", serde_json::to_string(&result)?);
        } else {
            if i > 0 {
                println!();
            }
            print_tags_human(&result);
        }
    }

    if had_errors {
        std::process::exit(1);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// write-tags
// ---------------------------------------------------------------------------

fn build_tags_from_args(args: &WriteTagsArgs) -> HashMap<String, Option<String>> {
    let mut tags = HashMap::new();
    let fields: &[(&str, &Option<String>)] = &[
        ("artist", &args.artist),
        ("title", &args.title),
        ("album", &args.album),
        ("album_artist", &args.album_artist),
        ("genre", &args.genre),
        ("year", &args.year),
        ("track", &args.track),
        ("disc", &args.disc),
        ("comment", &args.comment),
        ("publisher", &args.publisher),
        ("bpm", &args.bpm),
        ("key", &args.key),
        ("composer", &args.composer),
        ("remixer", &args.remixer),
    ];
    for &(name, value) in fields {
        if let Some(v) = value {
            if v.is_empty() {
                // Empty string means delete the field
                tags.insert(name.to_string(), None);
            } else {
                tags.insert(name.to_string(), Some(v.clone()));
            }
        }
    }
    tags
}

fn parse_wav_targets(
    raw: &Option<Vec<String>>,
) -> Result<Vec<tags::WavTarget>, Box<dyn std::error::Error>> {
    match raw {
        Some(targets) => {
            let mut valid = Vec::new();
            let mut invalid = Vec::new();
            for t in targets {
                match t.as_str() {
                    "id3v2" => valid.push(tags::WavTarget::Id3v2),
                    "riff_info" => valid.push(tags::WavTarget::RiffInfo),
                    _ => invalid.push(t.as_str()),
                }
            }
            if !invalid.is_empty() {
                tracing::warn!("Unknown WAV target(s): {}", invalid.join(", "));
            }
            if valid.is_empty() {
                return Err("No valid WAV targets. Valid values: id3v2, riff_info"
                    .to_string()
                    .into());
            }
            Ok(valid)
        }
        None => Ok(vec![tags::WavTarget::Id3v2, tags::WavTarget::RiffInfo]),
    }
}

fn print_write_human(result: &tags::FileWriteResult) {
    match result {
        tags::FileWriteResult::Ok {
            path,
            fields_written,
            fields_deleted,
            wav_targets,
            ..
        } => {
            tracing::info!("=== {} ===", path);
            if !fields_written.is_empty() {
                println!("Written: {}", fields_written.join(", "));
            }
            if !fields_deleted.is_empty() {
                println!("Deleted: {}", fields_deleted.join(", "));
            }
            if fields_written.is_empty() && fields_deleted.is_empty() {
                println!("No changes.");
            }
            if let Some(targets) = wav_targets {
                println!("WAV targets: {}", targets.join(", "));
            }
        }
        tags::FileWriteResult::Error { path, error, .. } => {
            tracing::error!("=== {} ===", path);
            tracing::error!("Error: {}", error);
        }
    }
}

fn print_dry_run_human(result: &tags::FileDryRunResult) {
    match result {
        tags::FileDryRunResult::Preview {
            path,
            changes,
            wav_targets,
            ..
        } => {
            tracing::info!("=== {} (dry run) ===", path);
            if changes.is_empty() {
                println!("No changes.");
                return;
            }
            for &field in tags::ALL_FIELDS {
                if let Some(change) = changes.get(field) {
                    let old = change.old.as_deref().unwrap_or("(missing)");
                    let new = change.new.as_deref().unwrap_or("(delete)");
                    println!("  {:<14}{} -> {}", display_field_name(field), old, new);
                }
            }
            if let Some(targets) = wav_targets {
                println!("WAV targets: {}", targets.join(", "));
            }
        }
        tags::FileDryRunResult::Error { path, error, .. } => {
            tracing::error!("=== {} ===", path);
            tracing::error!("Error: {}", error);
        }
    }
}

fn run_write_tags(args: WriteTagsArgs) -> Result<(), Box<dyn std::error::Error>> {
    let tag_map = if args.json_input {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        let parsed: HashMap<String, Option<String>> = serde_json::from_str(&input)?;
        parsed
    } else {
        build_tags_from_args(&args)
    };

    if tag_map.is_empty() {
        return Err("No tags specified. Use --field flags or --json-input.".into());
    }

    let wav_targets = parse_wav_targets(&args.wav_targets)?;
    let entry = tags::WriteEntry {
        path: PathBuf::from(&args.path),
        tags: tag_map,
        wav_targets,
        comment_mode: tags::CommentMode::default(),
    };

    let had_errors;

    if args.dry_run {
        let result = tags::write_file_tags_dry_run(&entry);
        had_errors = matches!(&result, tags::FileDryRunResult::Error { .. });
        if args.json {
            println!("{}", serde_json::to_string(&result)?);
        } else {
            print_dry_run_human(&result);
        }
    } else {
        let result = tags::write_file_tags(&entry);
        had_errors = matches!(&result, tags::FileWriteResult::Error { .. });
        if args.json {
            println!("{}", serde_json::to_string(&result)?);
        } else {
            print_write_human(&result);
        }
    }

    if had_errors {
        std::process::exit(1);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// extract-art
// ---------------------------------------------------------------------------

fn run_extract_art(args: ExtractArtArgs) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(&args.path);
    let output = args.output.as_deref().map(Path::new);

    match tags::extract_cover_art(path, output, &args.picture_type) {
        Ok(result) => {
            if args.json {
                println!("{}", serde_json::to_string(&result)?);
            } else {
                println!(
                    "Extracted {} ({}, {} bytes) -> {}",
                    result.picture_type, result.image_format, result.size_bytes, result.output_path
                );
            }
        }
        Err(e) => {
            tracing::error!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// embed-art
// ---------------------------------------------------------------------------

fn run_embed_art(args: EmbedArtArgs) -> Result<(), Box<dyn std::error::Error>> {
    let image_path = Path::new(&args.image);
    let mut had_errors = false;

    for target in &args.targets {
        let target_path = Path::new(target);
        let result = tags::embed_cover_art(image_path, target_path, &args.picture_type);

        if matches!(&result, tags::FileEmbedResult::Error { .. }) {
            had_errors = true;
        }

        if args.json {
            println!("{}", serde_json::to_string(&result)?);
        } else {
            match &result {
                tags::FileEmbedResult::Ok { path, .. } => {
                    println!("Embedded cover art into {}", path);
                }
                tags::FileEmbedResult::Error { path, error, .. } => {
                    tracing::error!("Error ({}): {}", path, error);
                }
            }
        }
    }

    if had_errors {
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        cache_status_for_track, file_mtime_unix, handle_analysis_result, handle_decode_result,
        is_cache_fresh, mark_track_outcome,
    };
    use crate::{audio::AudioError, audio::StratumResult, store, store::CachedAudioAnalysis};
    use std::time::Duration;

    fn cached(file_size: i64, file_mtime: i64) -> CachedAudioAnalysis {
        CachedAudioAnalysis {
            file_path: "/tmp/a.flac".to_string(),
            analyzer: "stratum-dsp".to_string(),
            file_size,
            file_mtime,
            analysis_version: "1.0.0".to_string(),
            features_json: "{}".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    fn sample_stratum_result() -> StratumResult {
        StratumResult {
            bpm: 120.0,
            bpm_confidence: 0.9,
            key: "Am".to_string(),
            key_camelot: "8A".to_string(),
            key_confidence: 0.8,
            key_clarity: 0.7,
            grid_stability: 0.95,
            duration_seconds: 180.0,
            processing_time_ms: 42.0,
            analyzer_version: "1.0.0".to_string(),
            flags: vec![],
            warnings: vec![],
        }
    }

    fn open_temp_store_with_probe() -> (tempfile::TempDir, rusqlite::Connection, (String, i64, i64))
    {
        let dir = tempfile::tempdir().expect("temp dir");

        let audio_path = dir.path().join("track.wav");
        std::fs::write(&audio_path, b"not-a-real-audio-file").expect("write audio fixture");

        let metadata = std::fs::metadata(&audio_path).expect("metadata");
        let probe = (
            audio_path.to_string_lossy().to_string(),
            metadata.len() as i64,
            file_mtime_unix(&metadata),
        );

        let store_path = dir.path().join("cache.sqlite3");
        let conn = store::open(store_path.to_str().expect("utf-8 path")).expect("open store");
        (dir, conn, probe)
    }

    #[test]
    fn cache_is_fresh_only_when_size_and_mtime_match() {
        let entry = cached(123, 456);
        assert!(is_cache_fresh(Some(&entry), 123, 456));
        assert!(!is_cache_fresh(Some(&entry), 999, 456));
        assert!(!is_cache_fresh(Some(&entry), 123, 999));
    }

    #[test]
    fn missing_cache_is_not_fresh() {
        assert!(!is_cache_fresh(None, 123, 456));
    }

    #[test]
    fn file_mtime_unix_returns_non_negative_timestamp_for_real_file() {
        let dir = tempfile::tempdir().expect("temp dir");
        let path = dir.path().join("x.txt");
        std::fs::write(&path, "a").expect("write");
        let metadata = std::fs::metadata(path).expect("metadata");
        assert!(file_mtime_unix(&metadata) >= 0);
    }

    #[test]
    fn cache_status_skips_track_when_both_fresh_entries_exist() {
        let (_dir, conn, probe) = open_temp_store_with_probe();
        let (cache_key, file_size, file_mtime) = probe.clone();

        store::set_audio_analysis(
            &conn,
            &cache_key,
            "stratum-dsp",
            file_size,
            file_mtime,
            "1.0.0",
            "{}",
        )
        .expect("set stratum");
        store::set_audio_analysis(
            &conn, &cache_key, "essentia", file_size, file_mtime, "1.0.0", "{}",
        )
        .expect("set essentia");

        let (has_stratum, has_essentia) =
            cache_status_for_track(&conn, Some(&probe), true, true).expect("cache status");
        assert!(has_stratum);
        assert!(has_essentia);
    }

    #[test]
    fn cache_status_only_skips_fresh_analyzers() {
        let (_dir, conn, probe) = open_temp_store_with_probe();
        let (cache_key, file_size, file_mtime) = probe.clone();

        store::set_audio_analysis(
            &conn,
            &cache_key,
            "stratum-dsp",
            file_size + 1,
            file_mtime,
            "1.0.0",
            "{}",
        )
        .expect("set stale stratum");
        store::set_audio_analysis(
            &conn, &cache_key, "essentia", file_size, file_mtime, "1.0.0", "{}",
        )
        .expect("set fresh essentia");

        let (has_stratum, has_essentia) =
            cache_status_for_track(&conn, Some(&probe), true, true).expect("cache status");
        assert!(!has_stratum, "stale stratum cache must be re-analyzed");
        assert!(has_essentia, "fresh essentia cache should still be skipped");
    }

    #[tokio::test]
    async fn decode_join_error_marks_failed_and_allows_next_track() {
        let handle = tokio::spawn(async {
            tokio::time::sleep(Duration::from_secs(60)).await;
            Ok::<(Vec<f32>, u32), AudioError>((vec![0.0], 44_100))
        });
        handle.abort();
        let join_err = handle
            .await
            .expect_err("aborted task should produce JoinError");

        let mut failed = 0;
        assert!(handle_decode_result(Err(join_err), 1, 2, "a - b", &mut failed).is_none());
        assert_eq!(failed, 1);

        let next = handle_decode_result(Ok(Ok((vec![0.0], 44_100))), 2, 2, "c - d", &mut failed);
        assert!(
            next.is_some(),
            "next track should continue after prior join error"
        );
        assert_eq!(failed, 1);
    }

    #[tokio::test]
    async fn analysis_join_error_marks_failed_and_allows_next_track() {
        let handle = tokio::spawn(async {
            tokio::time::sleep(Duration::from_secs(60)).await;
            Ok::<StratumResult, AudioError>(sample_stratum_result())
        });
        handle.abort();
        let join_err = handle
            .await
            .expect_err("aborted task should produce JoinError");

        let mut failed = 0;
        assert!(handle_analysis_result(Err(join_err), 1, 2, "a - b", &mut failed).is_none());
        assert_eq!(failed, 1);

        let next =
            handle_analysis_result(Ok(Ok(sample_stratum_result())), 2, 2, "c - d", &mut failed);
        assert!(
            next.is_some(),
            "next track should continue after prior analysis join error"
        );
        assert_eq!(failed, 1);
    }

    #[test]
    fn mark_track_outcome_counts_success_and_failure_consistently() {
        let mut analyzed = 0;
        let mut failed = 0;

        mark_track_outcome(&mut analyzed, &mut failed, true);
        mark_track_outcome(&mut analyzed, &mut failed, false);

        assert_eq!(analyzed, 1);
        assert_eq!(failed, 1);
    }
}
