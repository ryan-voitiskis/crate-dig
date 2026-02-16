# crate-dig

MCP server for Rekordbox library management. Reads directly from the encrypted master.db,
stages metadata changes in memory, and writes Rekordbox-compatible XML for safe reimport.

Built as a single static Rust binary with zero runtime dependencies. Operated through
Claude Code — no web UI, no CLI flags, just MCP.

## Build

```bash
cargo build --release
```

The binary is at `./target/release/crate-dig` (~8.5 MB, arm64).

## Register with Claude Code

```bash
claude mcp add crate-dig ./target/release/crate-dig
```

The server auto-detects the Rekordbox database at `~/Library/Pioneer/rekordbox/master.db`.
To override, set `REKORDBOX_DB_PATH` in `.env`.

## Tools

| Tool | Description |
|------|-------------|
| `read_library` | Get library summary: track count, genre distribution, stats |
| `search_tracks` | Search and filter tracks in the Rekordbox library |
| `get_track` | Get full details for a specific track by ID |
| `get_playlists` | List all playlists with track counts |
| `get_playlist_tracks` | List tracks in a specific playlist |
| `get_genre_taxonomy` | Get the configured genre taxonomy |
| `update_tracks` | Stage changes to track metadata (genre, comments, rating, color) |
| `preview_changes` | Preview all staged changes, showing what will differ from current state |
| `write_xml` | Write staged changes to a Rekordbox-compatible XML file |
| `clear_changes` | Clear staged changes for specific tracks or all |

## Genre Taxonomy

Starter set for consistency (not a closed list — arbitrary genres are accepted):

Ambient, Ambient Techno, Bassline, Breakbeat, Deep House, Deep Techno, Disco,
Drum & Bass, Dub Techno, Dubstep, Garage, Grime, Hard Techno, House, IDM,
Jungle, Minimal, Speed Garage, Techno

## Workflow

1. **Search** — use `search_tracks` or `get_playlist_tracks` to find tracks to tag
2. **Update** — use `update_tracks` to stage genre, comments, rating, or color changes
3. **Preview** — use `preview_changes` to review what will change vs. current state
4. **Write** — use `write_xml` to generate the XML file (runs backup automatically)
5. **Import in Rekordbox** — File > Import > Import Playlist/Collection, select the XML

### Reimport Gotcha

Rekordbox 5.6.1+ does not update existing tracks on standard XML import. Use the two-step
workaround: right-click the imported playlist > "Import To Collection", then select all
tracks (Cmd+A), right-click > "Import To Collection" again to force overwrite.

## Documentation

- [`docs/rekordbox-internals.md`](docs/rekordbox-internals.md) — Rekordbox file formats, database schema, XML structure, ecosystem tools
- [`docs/backup-and-restore.md`](docs/backup-and-restore.md) — Backup usage and restore procedures
