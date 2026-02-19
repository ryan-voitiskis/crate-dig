# 005: Test Coverage Gaps

Status: ready
Date: 2026-02-19

## Context

The crate-dig test suite has 73 unit tests and 23 integration tests (96 total). Store-level CRUD, type conversions, XML generation, genre taxonomy, and `resolve_single_track` logic are well covered. The gaps are at the tool level — the MCP tools that orchestrate caching, enrichment, and batch operations have minimal or no dedicated tests.

These gaps were identified during the phase 4-5 code review audit.

## Gap 1: `force_refresh` parameter behavior

**What's missing:** No test verifies that `force_refresh=true` bypasses cache and re-fetches from the API.

**Why it matters:** `force_refresh` is the only mechanism to update stale cache entries. If it silently stops working, the operator has no way to fix incorrect cached data without manually clearing the internal store.

**Test plan:**

```
test_force_refresh_bypasses_enrichment_cache (unit)
  1. Create in-memory internal store
  2. Pre-populate enrichment_cache with known entry for ("discogs", "artist", "title")
  3. Call lookup_discogs with force_refresh=false — assert returns cached data
  4. Call lookup_discogs with force_refresh=true — assert makes API call (mock or verify cache timestamp updated)
```

This needs either a mock HTTP client or a way to detect that the cache entry was overwritten (check `created_at` changed). Since the tools take `&self` with a real HTTP client, the cleanest approach is:

- **Option A:** Test at the store layer — verify that `set_enrichment` overwrites an existing entry and `created_at` updates. (Already covered by `test_enrichment_cache_upsert`.)
- **Option B:** Test the tool's cache-check logic by extracting it into a testable helper.
- **Option C:** Integration test (ignored) that calls the real tool with `force_refresh=true` against a pre-populated cache.

Recommended: **Option C** as an `#[ignore]` integration test. The upsert behavior is already tested; what's missing is the tool-level wiring that respects the flag.

## Gap 2: `enrich_tracks` batch tool

**What's missing:** No unit or integration test for the `enrich_tracks` tool.

**Why it matters:** The F2 bug (Beatport cache shape mismatch) lived in `enrich_tracks` undetected. Tool-level tests would have caught the schema divergence between batch and individual lookups.

**Test plan:**

```
test_enrich_tracks_caches_with_correct_schema (integration, ignored)
  1. Call enrich_tracks for 2-3 known tracks with providers=["discogs"]
  2. Verify summary counts (enriched + cached + skipped + failed = total)
  3. Call resolve_track_data for one of the tracks
  4. Verify discogs section is non-null and contains expected fields
  5. Call enrich_tracks again with skip_cached=true
  6. Verify all tracks report as cached (no re-enrichment)

test_enrich_tracks_beatport_schema_matches_individual (integration, ignored)
  1. Call lookup_beatport for a known track (caches native BeatportResult)
  2. Read cache entry, parse JSON, verify "genre" field (string)
  3. Clear cache for that track
  4. Call enrich_tracks with providers=["beatport"] for same track
  5. Read cache entry, parse JSON, verify same "genre" field (string, not "genres" array)

test_enrich_tracks_invalid_provider (unit)
  1. Call enrich_tracks with providers=["spotify"]
  2. Verify error response mentions invalid provider
```

## Gap 3: `resolve_track_data` / `resolve_tracks_data` tool-level tests

**What's missing:** `resolve_single_track` (the pure function) has 8 unit tests. The tool methods that call it have no tests.

**Why it matters:** The F1 bug (audio cache key mismatch) was in the tool-level wiring around `resolve_single_track`, not in the function itself. The cache key resolution logic is invisible to `resolve_single_track` unit tests.

**Test plan:**

```
test_resolve_track_data_returns_all_sections (integration, ignored)
  1. Pick a real track with known enrichment + audio cache
  2. Call resolve_track_data
  3. Verify all sections present: rekordbox, audio_analysis, discogs, beatport, staged_changes, data_completeness, genre_taxonomy
  4. Verify data_completeness flags match actual section presence

test_resolve_tracks_data_batch_consistency (integration, ignored)
  1. Call resolve_tracks_data for 5 tracks
  2. Call resolve_track_data individually for each
  3. Verify batch results match individual results (same sections, same values)

test_resolve_with_percent_encoded_path (integration, ignored)
  1. Find a track with percent-encoded characters in file_path (e.g., Aníbal)
  2. Run analyze_track_audio to populate audio cache
  3. Call resolve_track_data
  4. Verify data_completeness.stratum_dsp is true (not false due to cache key mismatch)
```

## Gap 4: Golden dataset

**What's missing:** `tests/fixtures/golden_genres.json` — a curated set of tracks with manually verified genres for validating genre tagging workflows.

**Why it matters:** Without a ground truth dataset, there's no way to measure whether genre tagging quality improves or regresses as the system evolves. This is the most impactful test artifact for the project's primary use case.

**Test plan:**

Not a code test — a fixture file that grows over time.

```json
[
  {
    "artist": "Burial",
    "title": "Archangel",
    "expected_genre": "Dubstep",
    "notes": "iconic dubstep, should never be tagged as anything else"
  },
  {
    "artist": "Kerri Chandler",
    "title": "Rain",
    "expected_genre": "Deep House",
    "notes": "classic deep house"
  }
]
```

**Format:**
- Key on `(artist, title)`, not `track_id` (IDs change on reimport)
- `expected_genre` must be a canonical genre from the taxonomy
- `notes` explains the reasoning for edge cases
- Start with 2-3 tracks per genre for the operator's top 15-20 genres (30-60 tracks)
- Expand over time as genre sessions surface ambiguous cases

**Validation test (integration, ignored):**
```
test_golden_dataset_genre_accuracy
  1. Load golden_genres.json
  2. For each entry, find track by artist+title in DB
  3. If track has a genre set, compare against expected_genre
  4. Report accuracy (% correct) and list mismatches
  5. No hard pass/fail threshold initially — this is a regression tracker
```

## Implementation priority

| # | Gap | Type | Effort | Impact |
|---|---|---|---|---|
| 1 | Golden dataset fixture | Fixture + integration test | Medium (manual curation) | High — validates primary use case |
| 2 | `enrich_tracks` batch tests | Integration (ignored) | Low | High — caught F2 bug class |
| 3 | `resolve_track_data` percent-encoded path test | Integration (ignored) | Low | High — caught F1 bug class |
| 4 | `resolve_tracks_data` batch consistency | Integration (ignored) | Low | Medium |
| 5 | `force_refresh` integration test | Integration (ignored) | Low | Medium |
| 6 | `enrich_tracks` invalid provider (unit) | Unit | Low | Low — defensive test |

## Notes

- All integration tests should be `#[ignore]` since they require the real Rekordbox DB backup
- The golden dataset is the most valuable but most effort — it requires manual curation by the operator
- Tool-level tests for `enrich_tracks` and `resolve_track_data` would have caught both high-severity bugs from the phase 4-5 review
