use rusqlite::Connection;

use crate::{audio, store};

pub(super) fn resolved_audio_cache_key(raw_file_path: &str) -> String {
    super::resolve_file_path(raw_file_path).unwrap_or_else(|_| raw_file_path.to_string())
}

fn file_timestamp_secs(metadata: &std::fs::Metadata) -> i64 {
    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub(super) fn get_fresh_analysis_entry(
    store: &Connection,
    raw_file_path: &str,
    analyzer: &str,
    schema_version: &str,
) -> Result<Option<store::CachedAudioAnalysis>, String> {
    let file_path = resolved_audio_cache_key(raw_file_path);
    let metadata = match std::fs::metadata(&file_path) {
        Ok(metadata) => metadata,
        Err(err) => {
            tracing::debug!(
                "Skipping stale {analyzer} cache lookup for {}: unable to stat file: {err}",
                file_path
            );
            return Ok(None);
        }
    };
    let file_size = metadata.len() as i64;
    let file_mtime = file_timestamp_secs(&metadata);

    let cached = store::get_audio_analysis(store, &file_path, analyzer)
        .map_err(|e| format!("Cache read error: {e}"))?;
    match cached {
        Some(entry)
            if entry.file_size == file_size
                && entry.file_mtime == file_mtime
                && entry.analysis_version == schema_version =>
        {
            Ok(Some(entry))
        }
        _ => Ok(None),
    }
}

/// Check analysis cache. Returns `Some(json_string)` on valid hit (matching
/// file_size, file_mtime, and schema_version), `None` on miss.
pub(super) fn check_analysis_cache(
    store: &Connection,
    raw_file_path: &str,
    analyzer: &str,
    file_size: i64,
    file_mtime: i64,
    schema_version: &str,
) -> Result<Option<String>, String> {
    let file_path = resolved_audio_cache_key(raw_file_path);
    let cached = store::get_audio_analysis(store, &file_path, analyzer)
        .map_err(|e| format!("Cache read error: {e}"))?;
    match cached {
        Some(entry)
            if entry.file_size == file_size
                && entry.file_mtime == file_mtime
                && entry.analysis_version == schema_version =>
        {
            Ok(Some(entry.features_json))
        }
        _ => Ok(None),
    }
}

/// Decode audio and run stratum-dsp analysis (uses `spawn_blocking`).
pub(super) async fn analyze_stratum(file_path: &str) -> Result<audio::StratumResult, String> {
    let path = file_path.to_string();
    let (samples, sample_rate) =
        tokio::task::spawn_blocking(move || audio::decode_to_samples(&path))
            .await
            .map_err(|e| format!("Decode task failed: {e}"))?
            .map_err(|e| format!("Decode error: {e}"))?;

    tokio::task::spawn_blocking(move || audio::analyze_with_stratum(&samples, sample_rate))
        .await
        .map_err(|e| format!("Analysis task failed: {e}"))?
        .map_err(|e| format!("Analysis error: {e}"))
}
