use rusqlite::Connection;

use crate::{audio, store};

/// Check analysis cache. Returns `Some(json_string)` on valid hit (matching
/// file_size and file_mtime), `None` on miss.
pub(super) fn check_analysis_cache(
    store: &Connection,
    file_path: &str,
    analyzer: &str,
    file_size: i64,
    file_mtime: i64,
) -> Result<Option<String>, String> {
    let cached = store::get_audio_analysis(store, file_path, analyzer)
        .map_err(|e| format!("Cache read error: {e}"))?;
    match cached {
        Some(entry) if entry.file_size == file_size && entry.file_mtime == file_mtime => {
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

    tokio::task::spawn_blocking(move || audio::analyze(&samples, sample_rate))
        .await
        .map_err(|e| format!("Analysis task failed: {e}"))?
        .map_err(|e| format!("Analysis error: {e}"))
}

/// Run essentia analysis (async subprocess).
pub(super) async fn analyze_essentia(
    python_path: &str,
    file_path: &str,
) -> Result<audio::EssentiaOutput, String> {
    audio::run_essentia(python_path, file_path).await
}

/// Write analysis result to cache.
pub(super) fn cache_analysis(
    store: &Connection,
    file_path: &str,
    analyzer: &str,
    file_size: i64,
    file_mtime: i64,
    version: &str,
    features_json: &str,
) -> Result<(), String> {
    store::set_audio_analysis(store, file_path, analyzer, file_size, file_mtime, version, features_json)
        .map_err(|e| format!("Cache write error: {e}"))
}
