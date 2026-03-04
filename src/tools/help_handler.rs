use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData as McpError;

use super::mcp_internal_error;

pub(super) fn handle_help() -> Result<CallToolResult, McpError> {
    let result = serde_json::json!({
        "workflows": [
            {
                "name": "Genre Classification",
                "summary": "Classify genres using Discogs, Beatport, and audio evidence.",
                "url": "https://reklawdbox.com/agent/genre-classification/",
                "key_tools": ["cache_coverage", "enrich_tracks", "resolve_tracks_data", "suggest_normalizations", "update_tracks"]
            },
            {
                "name": "Set Building",
                "summary": "Build transition-scored DJ set sequences.",
                "url": "https://reklawdbox.com/agent/set-building/",
                "key_tools": ["search_tracks", "resolve_tracks_data", "build_set", "score_transition"]
            },
            {
                "name": "Collection Audit",
                "summary": "Detect and fix naming, tagging, and convention violations.",
                "url": "https://reklawdbox.com/agent/collection-audit/",
                "key_tools": ["audit_state", "read_file_tags", "write_file_tags"]
            }
        ],
        "getting_started": "https://reklawdbox.com/getting-started/",
        "reference": "https://reklawdbox.com/reference/tools/",
        "llms_txt": "https://reklawdbox.com/llms.txt",
        "tip": "Run cache_coverage to check enrichment and analysis readiness before starting a workflow."
    });
    let json = serde_json::to_string_pretty(&result)
        .map_err(|e| mcp_internal_error(format!("{e}")))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}
