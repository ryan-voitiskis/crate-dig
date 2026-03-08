use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;

use super::mcp_internal_error;

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct HelpParams {
    #[schemars(
        description = "Optional topic filter: 'genre', 'set', 'audit', 'import'. Omit for the full menu."
    )]
    pub topic: Option<String>,
}

struct Workflow {
    name: &'static str,
    keywords: &'static [&'static str],
    summary: &'static str,
    url: &'static str,
    key_tools: &'static [&'static str],
}

const WORKFLOWS: &[Workflow] = &[
    Workflow {
        name: "Batch Import",
        keywords: &["import", "batch", "download", "purchase"],
        summary: "Prepare newly acquired music for Rekordbox import: tag, rename, embed cover art.",
        url: "https://reklawdbox.com/agent/batch-import/",
        key_tools: &[
            "read_file_tags",
            "write_file_tags",
            "lookup_discogs",
            "lookup_beatport",
            "embed_cover_art",
            "extract_cover_art",
        ],
    },
    Workflow {
        name: "Genre Classification",
        keywords: &["genre", "classify", "classification"],
        summary: "Classify genres using Discogs, Beatport, and audio evidence.",
        url: "https://reklawdbox.com/agent/genre-classification/",
        key_tools: &[
            "cache_coverage",
            "enrich_tracks",
            "resolve_tracks_data",
            "suggest_normalizations",
            "update_tracks",
        ],
    },
    Workflow {
        name: "Set Building",
        keywords: &["set", "mix", "sequence", "transition", "playlist"],
        summary: "Build transition-scored DJ set sequences.",
        url: "https://reklawdbox.com/agent/set-building/",
        key_tools: &[
            "search_tracks",
            "resolve_tracks_data",
            "build_set",
            "score_transition",
        ],
    },
    Workflow {
        name: "Collection Audit",
        keywords: &["audit", "fix", "naming", "tags", "convention"],
        summary: "Detect and fix naming, tagging, and convention violations.",
        url: "https://reklawdbox.com/agent/collection-audit/",
        key_tools: &["audit_state", "read_file_tags", "write_file_tags"],
    },
];

fn workflow_to_json(w: &Workflow) -> serde_json::Value {
    serde_json::json!({
        "name": w.name,
        "summary": w.summary,
        "url": w.url,
        "key_tools": w.key_tools,
    })
}

pub(super) fn handle_help(params: HelpParams) -> Result<CallToolResult, McpError> {
    let result = if let Some(ref topic) = params.topic {
        let topic_lower = topic.to_lowercase();
        let matched = WORKFLOWS
            .iter()
            .find(|w| w.keywords.iter().any(|k| topic_lower.contains(k)));

        match matched {
            Some(w) => serde_json::json!({
                "workflow": workflow_to_json(w),
                "tip": "Use WebFetch on the URL above for the full step-by-step SOP.",
            }),
            None => serde_json::json!({
                "error": format!("No workflow matching '{topic}'. Try: import, genre, set, audit."),
                "workflows": WORKFLOWS.iter().map(|w| w.name).collect::<Vec<_>>(),
            }),
        }
    } else {
        serde_json::json!({
            "workflows": WORKFLOWS.iter().map(workflow_to_json).collect::<Vec<_>>(),
            "getting_started": "https://reklawdbox.com/getting-started/",
            "reference": "https://reklawdbox.com/reference/tools/",
            "llms_txt": "https://reklawdbox.com/llms.txt",
            "tip": "Run cache_coverage to check enrichment and analysis readiness before starting a workflow.",
        })
    };

    let json =
        serde_json::to_string_pretty(&result).map_err(|e| mcp_internal_error(format!("{e}")))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}
