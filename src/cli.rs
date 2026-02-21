use std::time::Instant;

use clap::Parser;

use crate::{audio, db, store, tools};

#[derive(Parser)]
#[command(name = "reklawdbox")]
enum Cli {
    /// Batch audio analysis (stratum-dsp + Essentia)
    Analyze(AnalyzeArgs),
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
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli {
        Cli::Analyze(args) => analyze(args).await,
    }
}

async fn analyze(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
    // Open Rekordbox DB
    let db_path = db::resolve_db_path()
        .ok_or("Cannot find Rekordbox database. Set REKORDBOX_DB_PATH or ensure Rekordbox is installed.")?;
    let conn = db::open(&db_path)?;

    // Open internal store (cache)
    let store_path = store::default_path();
    let store_conn = store::open(
        store_path
            .to_str()
            .ok_or("Invalid store path encoding")?,
    )?;

    // Probe essentia
    let essentia_python = if args.stratum_only {
        None
    } else {
        tools::probe_essentia_python_path()
    };

    eprintln!(
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
        added_after: args.added_after,
        added_before: args.added_before,
        exclude_samples: true,
        limit: Some(args.max_tracks),
        offset: None,
    };
    let tracks = db::search_tracks_unbounded(&conn, &params)?;

    if tracks.is_empty() {
        eprintln!("No tracks match the given filters.");
        return Ok(());
    }

    // Pre-filter: check cache for each track
    let skip_cached = !args.no_skip_cached;
    let mut to_analyze = Vec::new();
    let mut cached_count = 0;

    for track in &tracks {
        let cache_key =
            audio::resolve_audio_path(&track.file_path).unwrap_or_else(|_| track.file_path.clone());

        let has_stratum = if skip_cached {
            store::get_audio_analysis(&store_conn, &cache_key, "stratum-dsp")?
                .is_some()
        } else {
            false
        };

        let has_essentia = if essentia_python.is_none() {
            true // no essentia available, so nothing to do
        } else if skip_cached {
            store::get_audio_analysis(&store_conn, &cache_key, "essentia")?
                .is_some()
        } else {
            false // --no-skip-cached: always re-run
        };

        if has_stratum && has_essentia {
            cached_count += 1;
        } else {
            to_analyze.push((track, !has_stratum, !has_essentia));
        }
    }

    let total = tracks.len();
    let pending = to_analyze.len();
    eprintln!(
        "Scanning {total} tracks ({cached_count} cached, {pending} to analyze)\n"
    );

    if to_analyze.is_empty() {
        eprintln!("All tracks already cached. Nothing to do.");
        return Ok(());
    }

    let batch_start = Instant::now();
    let mut analyzed = 0u32;
    let mut failed = 0u32;

    for (i, (track, needs_stratum, needs_essentia)) in to_analyze.iter().enumerate() {
        let idx = i + 1;
        let label = format!("{} - {}", track.artist, track.title);

        // Resolve file path
        let file_path = match audio::resolve_audio_path(&track.file_path) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("[{idx}/{pending}] SKIP {label}: File not found");
                failed += 1;
                continue;
            }
        };

        // Get file metadata for cache
        let metadata = match std::fs::metadata(&file_path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("[{idx}/{pending}] SKIP {label}: Cannot stat file: {e}");
                failed += 1;
                continue;
            }
        };

        let file_size = metadata.len() as i64;
        let file_mtime = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let track_start = Instant::now();

        // Stratum analysis
        if *needs_stratum {
            let path_clone = file_path.clone();
            let decode_result =
                tokio::task::spawn_blocking(move || audio::decode_to_samples(&path_clone)).await?;

            let (samples, sample_rate) = match decode_result {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("[{idx}/{pending}] FAIL {label}: Decode error: {e}");
                    failed += 1;
                    continue;
                }
            };

            let analysis_result =
                tokio::task::spawn_blocking(move || audio::analyze(&samples, sample_rate)).await?;

            match analysis_result {
                Ok(result) => {
                    let features_json = serde_json::to_string(&result)?;
                    store::set_audio_analysis(
                        &store_conn,
                        &file_path,
                        "stratum-dsp",
                        file_size,
                        file_mtime,
                        &result.analyzer_version,
                        &features_json,
                    )?;

                    eprint!(
                        "[{idx}/{pending}] {label} ... BPM={:.1} Key={}",
                        result.bpm, result.key_camelot,
                    );

                    // Essentia analysis
                    if *needs_essentia {
                        if let Some(ref python) = essentia_python {
                            match audio::run_essentia(python, &file_path).await {
                                Ok(essentia_result) => {
                                    let essentia_json = essentia_result.to_string();
                                    let essentia_version = essentia_result
                                        .get("analyzer_version")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("unknown");
                                    store::set_audio_analysis(
                                        &store_conn,
                                        &file_path,
                                        "essentia",
                                        file_size,
                                        file_mtime,
                                        essentia_version,
                                        &essentia_json,
                                    )?;
                                    eprint!(" +essentia");
                                }
                                Err(e) => {
                                    eprint!(" (essentia failed: {e})");
                                }
                            }
                        }
                    }

                    let elapsed = track_start.elapsed().as_secs_f64();
                    eprintln!(" ({elapsed:.1}s)");
                    analyzed += 1;
                }
                Err(e) => {
                    eprintln!("[{idx}/{pending}] FAIL {label}: Analysis error: {e}");
                    failed += 1;
                }
            }
        } else if *needs_essentia {
            // Only needs essentia (stratum already cached)
            if let Some(ref python) = essentia_python {
                let elapsed_start = Instant::now();
                match audio::run_essentia(python, &file_path).await {
                    Ok(essentia_result) => {
                        let essentia_json = essentia_result.to_string();
                        let essentia_version = essentia_result
                            .get("analyzer_version")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        store::set_audio_analysis(
                            &store_conn,
                            &file_path,
                            "essentia",
                            file_size,
                            file_mtime,
                            essentia_version,
                            &essentia_json,
                        )?;
                        let elapsed = elapsed_start.elapsed().as_secs_f64();
                        eprintln!(
                            "[{idx}/{pending}] {label} ... +essentia ({elapsed:.1}s)"
                        );
                        analyzed += 1;
                    }
                    Err(e) => {
                        eprintln!("[{idx}/{pending}] FAIL {label}: Essentia error: {e}");
                        failed += 1;
                    }
                }
            }
        }
    }

    let total_time = batch_start.elapsed();
    let mins = total_time.as_secs() / 60;
    let secs = total_time.as_secs() % 60;
    eprintln!("\nDone: {analyzed} analyzed, {failed} failed ({mins}m {secs}s)");

    Ok(())
}
