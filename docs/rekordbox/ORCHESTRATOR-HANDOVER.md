# Rekordbox Knowledge Corpus — Handover

Date: 2026-02-17
Validator: `docs/rekordbox/validate-corpus.sh`
Plan: `docs/rekordbox/ORCHESTRATION-PLAN.md` / `ORCHESTRATOR-PROMPT.md`

## What This Is

A comprehensive rekordbox 7.x knowledge corpus built from 18 PDFs (~611 pages), 288 FAQ items, and 4 web pages. The corpus lives in `docs/rekordbox/` and is structured for agent consumption — each document has YAML frontmatter with standardized metadata (id, type, topics, modes, confidence).

The corpus exists to support the crate-dig MCP server project (a Rekordbox library management tool operated via Claude Code). It provides authoritative reference material for genre tagging workflows, XML import/export, and general Rekordbox knowledge.

## Current State

**Validation: 0 errors, 6 warnings** (run: `bash docs/rekordbox/validate-corpus.sh`)

### Completed

- **Phase A (Transcription):** All 37 agents across 4 waves finished. 63 documents total:
  - `manual/` — 33 files (259 pages of the rekordbox 7.2.8 instruction manual)
  - `guides/` — 17 files (16 operation guides covering DVS, lighting, streaming, cloud, video, etc.)
  - `faq/` — 9 files (288 Q&As split by topic from the official FAQ)
  - `features/` — 3 files (web page transcriptions: overview, what's new, cloud setup)
  - `reference/` — 1 file (developer/XML integration from rekordbox.com)

- **Frontmatter normalization:** All 63 files have topics mapped to the controlled vocabulary (57 terms) and modes normalized to lowercase. A Python fixer script was used: `docs/rekordbox/fix-frontmatter.py`.

- **Content fixes:**
  - Pages 81, 161-172, and 221 transcribed and merged into their respective manual files
  - All agent handoff markers removed
  - FAQ uncategorized sections fully redistributed across 9 topic files
  - Duplicate frontmatter ID (`introduction`) renamed to `guide-introduction` in guides/

### Remaining Warnings (acceptable)

- **4 page overlaps** at manual section boundaries (pages 60, 140, 200, 242 each appear in 2 files). These are boundary pages where content spans sections — both files include them for completeness. Not worth fixing.
- **2 missing artifacts:** `manifest.yaml` and `README.md` — these are Phase C deliverables.

## What's Left (Phase B + C)

The original orchestration plan (`ORCHESTRATOR-PROMPT.md`) defines three phases. Phase A is done. The remaining work:

### Phase B: Verification

The plan calls for verification agents that re-read source material and compare against transcriptions. This is the most expensive phase (63 agents, each reading source + transcription).

**Approach is up to you.** Options:
1. **Full verification** — spawn agents for every file as described in the prompt (thorough but costly)
2. **Spot-check verification** — verify a representative sample (e.g., 5-10 files from different types/waves)
3. **Skip verification** — the transcriptions were done by capable agents reading PDFs directly; confidence is already reasonable
4. **Selective verification** — focus on the most important files for the crate-dig use case (XML spec, preferences, collection management, genre-related content)

After verification, update each file's frontmatter: `confidence: verified`, `last_verified: "2026-02-17"`, `verified_by: agent`.

### Phase C: Organization & Mapping

5 sequential agents (each depends on the prior):

1. **C1 — manifest.yaml** — Read all frontmatter, generate `docs/rekordbox/manifest.yaml` with document inventory, taxonomy, and paths. Validate all paths exist and IDs are unique.

2. **C2 — README.md** — Generate `docs/rekordbox/README.md` with usage guide, priority consultation order for common tasks, topic→document cross-reference table, and source material summary.

3. **C3 — glossary** — Grep all files for bracketed terms (e.g., `[Collection]`, `[BPM]`), generate `docs/rekordbox/reference/glossary.md` with definitions and document references.

4. **C4 — XML import/export reference** — Consolidate XML knowledge from `guides/xml-format-spec.md`, `reference/developer-integration.md`, `manual/31-preferences.md`, and `docs/rekordbox-internals.md` into `docs/rekordbox/reference/xml-import-export.md`.

5. **C5 — Cross-reference links** — Add `## Related Documents` sections to each file based on shared topics.

See `ORCHESTRATOR-PROMPT.md` for full details on each agent's prompt.

## Key Files

| File | Purpose |
|------|---------|
| `validate-corpus.sh` | Bash validator — checks frontmatter schema, controlled vocabulary, page coverage, duplicate IDs |
| `fix-frontmatter.py` | Python script — bulk topic/mode normalization with mapping table |
| `ORCHESTRATOR-PROMPT.md` | Original orchestration plan with full agent prompts for all phases |
| `ORCHESTRATION-PLAN.md` | High-level plan document |
| `rekordbox7-faq.md` | Original FAQ source (288 items, ~3525 lines) |
| `source-material/` | 18 source PDFs |

## Controlled Vocabulary

**Topics** (57 terms): analysis, backup, beatgrid, browsing, cloud, collaborative-playlists, collection, color, comments, compatibility, connection, cue-points, devices, dvs, edit, effects, equipment, export, file-formats, genre, history, hot-cue, import, interface, key, library, lighting, link, memory-cue, metadata, midi, mixing, mobile, onelibrary, pads, performance, phrase, playback, playlists, preferences, pro-dj-link, rating, recording, sampler, search, sequencer, slicer, stems, streaming, subscription, system-requirements, track-suggestion, usb, video, waveform, xml

**Modes** (5): common, export, performance, lighting, edit

**Types** (5): manual, guide, faq, feature, reference

**Confidence**: pending | verified | high

## Notes

- All documents currently have `confidence: pending` — verification has not been done yet.
- The validator is strict on topics and will catch any drift. Run it after any bulk changes.
- The `fix-frontmatter.py` script is idempotent — safe to re-run after manual edits.
- Page overlap warnings are intentional and can be ignored.
- Some FAQ files may have minor content overlap where F1/F2/F3 agents independently categorized similar Q&As — deduplication could be a cleanup pass.
