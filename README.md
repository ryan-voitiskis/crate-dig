# crate-dig

AI-powered Rekordbox metadata tool for genre tagging, classification, and library management.

## Status

- [x] Phase 0: Backup & safety system
- [x] Phase 1: Rekordbox knowledge corpus
- [ ] Phase 2: TypeScript parser and data model
- [ ] Phase 3: AI-powered tagging
- [ ] Phase 4: MCP server / CLI

## Quick Start

```bash
# Back up your Rekordbox library (do this first!)
./backup.sh --db-only

# List backups
./backup.sh --list
```

## Documentation

- [`docs/rekordbox-internals.md`](docs/rekordbox-internals.md) — Comprehensive reference on Rekordbox file formats, database schema, XML structure, and ecosystem tools
- [`docs/backup-and-restore.md`](docs/backup-and-restore.md) — Backup usage and restore procedures

## Architecture

Phase 2 will use the **XML export/import** approach:
1. Export Rekordbox library to XML
2. Parse and analyze in TypeScript
3. Claude suggests genre tags, comments, ratings based on track metadata
4. Write modified XML for reimport into Rekordbox

See the [knowledge corpus](docs/rekordbox-internals.md) for the full technical rationale.
