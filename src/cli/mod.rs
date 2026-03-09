mod analyze;
mod hydrate;
mod tags;

use std::path::{Path, PathBuf};

use clap::Parser;

use crate::{audio, store};

#[derive(Parser)]
#[command(name = "reklawdbox", version)]
enum Cli {
    /// Batch audio analysis (stratum-dsp + Essentia)
    Analyze(analyze::AnalyzeArgs),
    /// Batch enrichment + analysis (Discogs, Beatport, audio analysis)
    Hydrate(hydrate::HydrateArgs),
    /// Read metadata tags from audio files
    ReadTags(tags::ReadTagsArgs),
    /// Write metadata tags to audio files
    WriteTags(tags::WriteTagsArgs),
    /// Extract embedded cover art from an audio file
    ExtractArt(tags::ExtractArtArgs),
    /// Embed cover art into audio files
    EmbedArt(tags::EmbedArtArgs),
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli {
        Cli::Analyze(args) => analyze::run_analyze(args).await,
        Cli::Hydrate(args) => hydrate::run_hydrate(args).await,
        Cli::ReadTags(args) => tags::run_read_tags(args),
        Cli::WriteTags(args) => tags::run_write_tags(args),
        Cli::ExtractArt(args) => tags::run_extract_art(args),
        Cli::EmbedArt(args) => tags::run_embed_art(args),
    }
}

// ---------------------------------------------------------------------------
// Shared helpers used by analyze + hydrate
// ---------------------------------------------------------------------------

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
    schema_version: &str,
) -> bool {
    matches!(
        cached,
        Some(entry) if entry.file_size == file_size
            && entry.file_mtime == file_mtime
            && entry.analysis_version == schema_version
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
    schema_version: &str,
) -> Result<bool, rusqlite::Error> {
    if let Some((cache_key, file_size, file_mtime)) = cache_probe {
        let cached = store::get_audio_analysis(store_conn, cache_key, analyzer)?;
        Ok(is_cache_fresh(
            cached.as_ref(),
            *file_size,
            *file_mtime,
            schema_version,
        ))
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
        has_fresh_cache_entry(
            store_conn,
            cache_probe,
            audio::ANALYZER_STRATUM,
            audio::STRATUM_SCHEMA_VERSION,
        )?
    } else {
        false
    };

    let has_essentia = if !essentia_available {
        true
    } else if skip_cached {
        has_fresh_cache_entry(
            store_conn,
            cache_probe,
            audio::ANALYZER_ESSENTIA,
            audio::ESSENTIA_SCHEMA_VERSION,
        )?
    } else {
        false
    };

    Ok((has_stratum, has_essentia))
}

/// Cache write message for audio analysis (used by analyze + hydrate)
pub(crate) struct CliCacheWriteMsg {
    pub file_path: String,
    pub analyzer: String,
    pub file_size: i64,
    pub file_mtime: i64,
    pub analyzer_version: String,
    pub features_json: String,
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

#[cfg(test)]
mod tests {
    use super::analyze::{handle_analysis_result, handle_decode_result, mark_track_outcome};
    use super::{cache_status_for_track, file_mtime_unix, is_cache_fresh};
    use crate::{
        audio,
        audio::AudioError,
        audio::StratumResult,
        store,
        store::CachedAudioAnalysis,
    };
    use std::time::Duration;

    fn cached(file_size: i64, file_mtime: i64) -> CachedAudioAnalysis {
        CachedAudioAnalysis {
            file_path: "/tmp/a.flac".to_string(),
            analyzer: "stratum-dsp".to_string(),
            file_size,
            file_mtime,
            analysis_version: audio::STRATUM_SCHEMA_VERSION.to_string(),
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
            mod_centroid: Some(12.5),
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
    fn cache_is_fresh_only_when_size_mtime_and_version_match() {
        let entry = cached(123, 456);
        let v = audio::STRATUM_SCHEMA_VERSION;
        assert!(is_cache_fresh(Some(&entry), 123, 456, v));
        assert!(!is_cache_fresh(Some(&entry), 999, 456, v));
        assert!(!is_cache_fresh(Some(&entry), 123, 999, v));
        assert!(!is_cache_fresh(Some(&entry), 123, 456, "outdated"));
    }

    #[test]
    fn missing_cache_is_not_fresh() {
        assert!(!is_cache_fresh(None, 123, 456, audio::STRATUM_SCHEMA_VERSION));
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
            audio::STRATUM_SCHEMA_VERSION,
            "{}",
        )
        .expect("set stratum");
        store::set_audio_analysis(
            &conn,
            &cache_key,
            "essentia",
            file_size,
            file_mtime,
            audio::ESSENTIA_SCHEMA_VERSION,
            "{}",
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
            audio::STRATUM_SCHEMA_VERSION,
            "{}",
        )
        .expect("set stale stratum");
        store::set_audio_analysis(
            &conn,
            &cache_key,
            "essentia",
            file_size,
            file_mtime,
            audio::ESSENTIA_SCHEMA_VERSION,
            "{}",
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
