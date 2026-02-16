use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use rmcp::handler::server::tool::ToolRouter;
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};
use rusqlite::Connection;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::changes::ChangeManager;
use crate::db;
use crate::genre;
use crate::types::TrackChange;
use crate::xml;

/// Inner shared state (not Clone).
struct ServerState {
    db: OnceLock<Result<Mutex<Connection>, String>>,
    db_path: Option<String>,
    changes: ChangeManager,
}

/// MCP server for Rekordbox library management.
#[derive(Clone)]
pub struct CrateDigServer {
    state: Arc<ServerState>,
    tool_router: ToolRouter<Self>,
}

impl CrateDigServer {
    fn conn(&self) -> Result<std::sync::MutexGuard<'_, Connection>, McpError> {
        let result = self.state.db.get_or_init(|| {
            let path = match self.state.db_path.clone().or_else(db::resolve_db_path) {
                Some(p) => p,
                None => {
                    return Err(
                        "Rekordbox database not found. Set REKORDBOX_DB_PATH env var.".into(),
                    )
                }
            };
            match db::open(&path) {
                Ok(conn) => Ok(Mutex::new(conn)),
                Err(e) => Err(format!("Failed to open Rekordbox database: {e}")),
            }
        });
        match result {
            Ok(mutex) => mutex
                .lock()
                .map_err(|_| McpError::internal_error("Database lock poisoned", None)),
            Err(msg) => Err(McpError::internal_error(msg.clone(), None)),
        }
    }
}

fn err(msg: String) -> McpError {
    McpError::internal_error(msg, None)
}

// --- Tool parameter types ---

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchTracksParams {
    #[schemars(description = "Search query matching title or artist")]
    pub query: Option<String>,
    #[schemars(description = "Filter by artist name (partial match)")]
    pub artist: Option<String>,
    #[schemars(description = "Filter by genre name (partial match)")]
    pub genre: Option<String>,
    #[schemars(description = "Minimum star rating (1-5)")]
    pub rating_min: Option<u8>,
    #[schemars(description = "Minimum BPM")]
    pub bpm_min: Option<f64>,
    #[schemars(description = "Maximum BPM")]
    pub bpm_max: Option<f64>,
    #[schemars(description = "Filter by musical key (e.g. 'Am', 'Cm')")]
    pub key: Option<String>,
    #[schemars(description = "Filter by playlist ID")]
    pub playlist: Option<String>,
    #[schemars(description = "Filter by whether track has a genre set")]
    pub has_genre: Option<bool>,
    #[schemars(description = "Max results (default 50, max 200)")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTrackParams {
    #[schemars(description = "Track ID")]
    pub track_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetPlaylistTracksParams {
    #[schemars(description = "Playlist ID")]
    pub playlist_id: String,
    #[schemars(description = "Max results (default 200)")]
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTracksParams {
    #[schemars(description = "Array of track changes to stage")]
    pub changes: Vec<TrackChangeInput>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TrackChangeInput {
    #[schemars(description = "Track ID")]
    pub track_id: String,
    #[schemars(description = "New genre")]
    pub genre: Option<String>,
    #[schemars(description = "New comments")]
    pub comments: Option<String>,
    #[schemars(description = "New star rating (1-5)")]
    pub rating: Option<u8>,
    #[schemars(description = "New color name")]
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteXmlParams {
    #[schemars(description = "Output file path (default: ./rekordbox-exports/crate-dig-{timestamp}.xml)")]
    pub output_path: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClearChangesParams {
    #[schemars(description = "Track IDs to clear (if empty, clears all)")]
    pub track_ids: Option<Vec<String>>,
}

// --- Tool implementations ---

use rmcp::handler::server::wrapper::Parameters;

#[tool_router]
impl CrateDigServer {
    pub fn new(db_path: Option<String>) -> Self {
        Self {
            state: Arc::new(ServerState {
                db: OnceLock::new(),
                db_path,
                changes: ChangeManager::new(),
            }),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Search and filter tracks in the Rekordbox library")]
    async fn search_tracks(&self, params: Parameters<SearchTracksParams>) -> Result<CallToolResult, McpError> {
        let conn = self.conn()?;
        let search = db::SearchParams {
            query: params.0.query,
            artist: params.0.artist,
            genre: params.0.genre,
            rating_min: params.0.rating_min,
            bpm_min: params.0.bpm_min,
            bpm_max: params.0.bpm_max,
            key: params.0.key,
            playlist: params.0.playlist,
            has_genre: params.0.has_genre,
            limit: params.0.limit,
        };
        let tracks = db::search_tracks(&conn, &search).map_err(|e| err(format!("DB error: {e}")))?;
        let json = serde_json::to_string_pretty(&tracks).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get full details for a specific track by ID")]
    async fn get_track(&self, params: Parameters<GetTrackParams>) -> Result<CallToolResult, McpError> {
        let conn = self.conn()?;
        let track =
            db::get_track(&conn, &params.0.track_id).map_err(|e| err(format!("DB error: {e}")))?;
        match track {
            Some(t) => {
                let json = serde_json::to_string_pretty(&t).map_err(|e| err(format!("{e}")))?;
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            None => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track '{}' not found",
                params.0.track_id
            ))])),
        }
    }

    #[tool(description = "List all playlists with track counts")]
    async fn get_playlists(&self) -> Result<CallToolResult, McpError> {
        let conn = self.conn()?;
        let playlists = db::get_playlists(&conn).map_err(|e| err(format!("DB error: {e}")))?;
        let json = serde_json::to_string_pretty(&playlists).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "List tracks in a specific playlist")]
    async fn get_playlist_tracks(
        &self,
        params: Parameters<GetPlaylistTracksParams>,
    ) -> Result<CallToolResult, McpError> {
        let conn = self.conn()?;
        let tracks = db::get_playlist_tracks(&conn, &params.0.playlist_id, params.0.limit)
            .map_err(|e| err(format!("DB error: {e}")))?;
        let json = serde_json::to_string_pretty(&tracks).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get library summary: track count, genre distribution, stats")]
    async fn read_library(&self) -> Result<CallToolResult, McpError> {
        let conn = self.conn()?;
        let stats = db::get_library_stats(&conn).map_err(|e| err(format!("DB error: {e}")))?;
        let json = serde_json::to_string_pretty(&stats).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Get the configured genre taxonomy")]
    async fn get_genre_taxonomy(&self) -> Result<CallToolResult, McpError> {
        let genres = genre::get_taxonomy();
        let result = serde_json::json!({
            "genres": genres,
            "description": "Flat genre taxonomy. Not a closed list — arbitrary genres are accepted. This list provides consistency suggestions."
        });
        let json = serde_json::to_string_pretty(&result).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Stage changes to track metadata (genre, comments, rating, color). Changes are held in memory until write_xml is called.")]
    async fn update_tracks(
        &self,
        params: Parameters<UpdateTracksParams>,
    ) -> Result<CallToolResult, McpError> {
        // Validate ratings before staging
        for c in &params.0.changes {
            if let Some(r) = c.rating {
                if r == 0 || r > 5 {
                    return Err(McpError::invalid_params(
                        format!("rating must be 1-5, got {r}"),
                        None,
                    ));
                }
            }
        }

        let changes: Vec<TrackChange> = params
            .0
            .changes
            .into_iter()
            .map(|c| TrackChange {
                track_id: c.track_id,
                genre: c.genre,
                comments: c.comments,
                rating: c.rating,
                color: c.color,
            })
            .collect();

        let (staged, total) = self.state.changes.stage(changes);
        let result = serde_json::json!({
            "staged": staged,
            "total_pending": total,
        });
        let json = serde_json::to_string_pretty(&result).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Preview all staged changes, showing what will differ from current state")]
    async fn preview_changes(&self) -> Result<CallToolResult, McpError> {
        let ids = self.state.changes.pending_ids();
        if ids.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "No changes staged.",
            )]));
        }

        let conn = self.conn()?;
        let current_tracks =
            db::get_tracks_by_ids(&conn, &ids).map_err(|e| err(format!("DB error: {e}")))?;

        let diffs = self.state.changes.preview(&current_tracks);
        if diffs.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "Changes staged but no fields actually differ from current values.",
            )]));
        }

        let json = serde_json::to_string_pretty(&diffs).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Write staged changes to a Rekordbox-compatible XML file. Runs backup first.")]
    async fn write_xml(&self, params: Parameters<WriteXmlParams>) -> Result<CallToolResult, McpError> {
        let ids = self.state.changes.pending_ids();
        if ids.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "No changes to write.",
            )]));
        }

        // Run backup before writing
        let backup_script = std::path::Path::new("backup.sh");
        if backup_script.exists() {
            eprintln!("[crate-dig] Running pre-op backup...");
            let output = std::process::Command::new("bash")
                .arg("backup.sh")
                .arg("--pre-op")
                .output();
            match output {
                Ok(o) if o.status.success() => eprintln!("[crate-dig] Backup completed."),
                Ok(o) => {
                    let stderr_out = String::from_utf8_lossy(&o.stderr);
                    eprintln!("[crate-dig] Backup warning: {stderr_out}");
                }
                Err(e) => eprintln!("[crate-dig] Backup skipped: {e}"),
            }
        }

        // Fetch current tracks and apply changes
        let conn = self.conn()?;
        let current_tracks =
            db::get_tracks_by_ids(&conn, &ids).map_err(|e| err(format!("DB error: {e}")))?;
        let modified_tracks = self.state.changes.apply_changes(&current_tracks);

        // Determine output path
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
        let output_path = params
            .0
            .output_path
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(format!("rekordbox-exports/crate-dig-{timestamp}.xml"))
            });

        xml::write_xml(&modified_tracks, &output_path)
            .map_err(|e| err(format!("Write error: {e}")))?;

        let track_count = modified_tracks.len();
        // Clear staged changes after successful write
        self.state.changes.clear(None);

        let result = serde_json::json!({
            "path": output_path.to_string_lossy(),
            "track_count": track_count,
            "changes_applied": track_count,
        });
        let json = serde_json::to_string_pretty(&result).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(description = "Clear staged changes for specific tracks or all")]
    async fn clear_changes(
        &self,
        params: Parameters<ClearChangesParams>,
    ) -> Result<CallToolResult, McpError> {
        let (cleared, remaining) = self.state.changes.clear(params.0.track_ids);
        let result = serde_json::json!({
            "cleared": cleared,
            "remaining": remaining,
        });
        let json = serde_json::to_string_pretty(&result).map_err(|e| err(format!("{e}")))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool_handler]
impl ServerHandler for CrateDigServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Rekordbox library management server. Search tracks, manage genres, \
                 stage metadata changes, and export to XML for reimport."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
