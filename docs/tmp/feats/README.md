# Feature Specs

Last updated: 2026-02-19.

## Core principle

MCP tools provide **deterministic data** (same cache state = same output). Claude Code does all genre reasoning. No scoring models or automated decisions in the server.

## Completed (specs removed)

- **001: Internal Store and Credentials** — SQLite cache store + Discogs credential externalization
- **002: Audio Analysis (Tier 1)** — stratum-dsp BPM/key confidence via Symphonia decode
- **003: Enrichment and Data Resolution** — Cached enrichment, batch tools, resolve tools, genre taxonomy mapping

## Active

| Spec | Summary | Status |
|---|---|---|
| [004: Essentia Audio Features](004-essentia-audio-features.md) | Python subprocess for danceability, energy, loudness, rhythmic descriptors | Ready for implementation |
| [005: Test Coverage Gaps](005-test-coverage-gaps.md) | Tool-level integration tests, golden dataset fixture | Ready for implementation |

## Workflow

```
search_tracks (find tracks needing genres)
  → enrich_tracks (fill Discogs/Beatport cache)
  → analyze_audio_batch (fill audio analysis cache — stratum-dsp now, Essentia via 004)
  → resolve_tracks_data (aggregate everything from cache)
  → Claude reasons about genre
  → update_tracks (stage changes)
  → preview_changes → write_xml
```
