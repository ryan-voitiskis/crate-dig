use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;
use super::*;
use crate::db;
use crate::store;
use crate::types::TrackChange;

#[derive(Debug, Deserialize, JsonSchema)]
pub(super) struct BackfillLabelsParams {
    #[schemars(description = "Preview changes without staging (default false)")]
    pub dry_run: Option<bool>,
}

fn normalize_label(label: &str) -> String {
    if label.starts_with("Not On Label") {
        return "Self".to_string();
    }
    label.to_string()
}

pub(super) fn handle_backfill_labels(
    server: &ReklawdboxServer,
    params: BackfillLabelsParams,
) -> Result<CallToolResult, McpError> {
    let dry_run = params.dry_run.unwrap_or(false);

    let rb_conn = server.rekordbox_conn()?;
    let search_params = db::SearchParams {
        query: None,
        artist: None,
        genre: None,
        rating_min: None,
        bpm_min: None,
        bpm_max: None,
        key: None,
        playlist: None,
        has_genre: None,
        has_label: None,
        label: None,
        path: None,
        path_prefix: None,
        added_after: None,
        added_before: None,
        exclude_samples: true,
        limit: None,
        offset: None,
    };
    let tracks = db::search_tracks_unbounded(&rb_conn, &search_params)
        .map_err(|e| mcp_internal_error(format!("DB error: {e}")))?;
    drop(rb_conn);

    let store_conn = server.cache_store_conn()?;

    let mut filled = 0usize;
    let mut already_labeled = 0usize;
    let mut conflicts = Vec::new();
    let mut no_enrichment = 0usize;
    let mut to_stage = Vec::new();

    for track in &tracks {
        let discogs_cache = store::get_enrichment(
            &store_conn,
            "discogs",
            &track.artist,
            &track.title,
            Some(&track.album),
        )
        .map_err(|e| mcp_internal_error(format!("Store error: {e}")))?;

        let discogs_val = classify_handler::parse_response_json(discogs_cache.as_ref());
        let enrichment_label = discogs_val
            .as_ref()
            .and_then(|v| v.get("label"))
            .and_then(|v| v.as_str())
            .filter(|l| !l.is_empty())
            .map(normalize_label);

        let Some(enrich_label) = enrichment_label else {
            no_enrichment += 1;
            continue;
        };

        if track.label.is_empty() {
            filled += 1;
            to_stage.push(TrackChange {
                track_id: track.id.clone(),
                genre: None,
                comments: None,
                rating: None,
                color: None,
                label: Some(enrich_label.clone()),
            });
        } else if track.label == enrich_label {
            already_labeled += 1;
        } else {
            conflicts.push(serde_json::json!({
                "track_id": track.id,
                "artist": track.artist,
                "title": track.title,
                "current_label": track.label,
                "enrichment_label": enrich_label,
            }));
        }
    }

    drop(store_conn);

    let staged_count = if !dry_run && !to_stage.is_empty() {
        let (staged, _) = server.state.changes.stage(to_stage);
        staged
    } else {
        0
    };

    let pending = server.state.changes.pending_ids().len();

    let result = serde_json::json!({
        "summary": {
            "total_scanned": tracks.len(),
            "filled": filled,
            "already_labeled": already_labeled,
            "conflicts": conflicts.len(),
            "no_enrichment": no_enrichment,
        },
        "staged": staged_count,
        "total_pending": pending,
        "dry_run": dry_run,
        "conflicts": conflicts,
    });

    let json =
        serde_json::to_string_pretty(&result).map_err(|e| mcp_internal_error(format!("{e}")))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}
