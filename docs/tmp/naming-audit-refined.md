# Naming Audit — Refined

Merged from two independent passes, cross-verified against source.
3 phantom entries removed; serde/API impact annotated.

Mark each: **Y** (accept), **N** (reject), **M** (modify — add a note).

---

## Section A — Methods & Functions

High-value renames: these are called frequently, appear in stack traces, or form
the internal API surface of `ServerState`.

| # | File | Line | Current | Proposed | Notes |
|---|------|------|---------|----------|-------|
| A1 | tools/mod.rs | 67 | `conn()` | `rekordbox_conn()` | Ambiguous next to `internal_conn()` |
| A2 | tools/mod.rs | 90 | `internal_conn()` | `cache_store_conn()` | "Internal" doesn't describe the cache/enrichment store |
| A3 | tools/mod.rs | 109 | `internal_store_path()` | `cache_store_path()` | Follows A2 |
| A4 | tools/mod.rs | 43 | `internal()` | `mcp_internal_error()` | Shadows keyword concept; doesn't convey "construct error" |
| A5 | tools/mod.rs | 400 | `read_library()` | `get_library_summary()` | Returns stats, not the library itself. **MCP tool name**: `#[tool]` has no explicit `name =`, so renaming the fn renames the tool. Must pin with `#[tool(name = "read_library")]`. |
| A6 | tools/mod.rs | 126 | `lookup_discogs_live()` | `lookup_discogs_remote()` | "Live" is informal; function has broker + legacy-creds fallback paths, so "via_broker" would be inaccurate |
| A7 | tools/mod.rs | 299 | `lookup_beatport_live()` | `lookup_beatport_remote()` | Consistency; "live" is informal |
| A8 | xml.rs | 35 | `path_to_location()` | `path_to_rekordbox_location_uri()` | More specific output contract |
| A9 | xml.rs | 77 | `write_track()` | `write_collection_track()` | Disambiguates from playlist writing |
| A10 | audio.rs | 471 | `to_camelot()` | `stratum_notation_to_camelot()` | Makes conversion source explicit |
| A11 | audio.rs | 489 | `analyze()` | `analyze_with_stratum()` | Generic name; stratum-dsp is the specific backend |
| A12 | audio.rs | 442 | `mix_to_mono()` | `downmix_to_mono()` | "Downmix" is standard audio term |
| A13 | color.rs | 23 | `canonical_casing()` | `canonical_color_name()` | Domain-specific clarity |
| A14 | genre.rs | 54 | `canonical_casing()` | `canonical_genre_name()` | Same pattern as A13 |
| A15 | genre.rs | 129 | `alias_map()` | `genre_alias_map()` | Qualifies what kind of aliases |
| A16 | genre.rs | 135 | `normalize_genre()` | `canonical_genre_from_alias()` | Behaviour is alias resolution, not normalization |
| A17 | normalize.rs | 2 | `normalize()` | `normalize_for_matching()` | Only used for fuzzy-match folding |
| A18 | corpus.rs | 233 | `consulted_paths()` | `matched_paths()` | Return value represents matches |
| A19 | corpus.rs | 353 | `touch_manifest_metadata()` | `mark_manifest_metadata_used()` | "Touch" is vague |
| A20 | discogs.rs | 454 | `nonce()` | `generate_oauth_nonce()` | Intent is security-specific |
| A21 | discogs.rs | 570 | `to_result()` | `to_discogs_result()` | Original too verbose; this is idiomatic Rust conversion naming |
| A22 | scoring.rs | 752 | `round_score()` | `round_to_3_decimals()` | "3dp" abbreviation unclear; full word is unambiguous. Alternatively keep `round_score()` if it stays scoring-specific. |
| A23 | scoring.rs | 1191 | `standard_key_to_camelot()` | `musical_key_to_camelot()` | "Standard" is ambiguous (standard key? standard function?) |
| ~~A24~~ | | | | | **REMOVED**: `to_percent()` is idiomatic Rust (`to_*` convention). Rounding precision is an implementation detail. |
| A25 | tags.rs | 250 | `format_name()` | `file_type_name()` | Narrower, more accurate semantic |
| A26 | changes.rs | 7 | `lock_or_recover()` | `acquire_or_recover_lock()` | Slightly more explicit |

---

## Section B — Types, Enums, Structs

| # | File | Line | Current | Proposed | Serde Impact | Notes |
|---|------|------|---------|----------|-------------|-------|
| B1 | params.rs | 245 | `SetPriority` | `SequencingPriority` | Safe (`rename_all = "snake_case"`) | "Set" overloaded |
| B2 | params.rs | 254 | `HarmonicStyle` | `HarmonicMixingStyle` | Safe (`rename_all`) | Clarifies mixing context |
| B3 | params.rs | 460 | `AuditStateParams` | `AuditOperation` | Safe (`tag = "operation"` + variant renames) | It's an enum of operations |
| B4 | params.rs | 273 | `Flat` (variant) | `FlatEnergy` | **UNSAFE**: `rename_all = "snake_case"` would change wire value from `"flat"` to `"flat_energy"`; needs explicit `#[serde(rename = "flat")]` to pin | Context-free sibling of `WarmupBuildPeakRelease` |
| B5 | discogs.rs | 24 | `BrokerConfigResult` | `BrokerConfigStatus` | Internal only | Avoids confusion with `Result<T,E>` |
| B6 | store.rs | 96 | `CachedEnrichment` | `EnrichmentCacheEntry` | Internal only | Noun form clarifies struct role |
| B7 | corpus.rs | 115 | `IndexedDocument` | `CorpusDocument` | Internal only | "Entry" is redundant (items live in a `Vec`); "Indexed" is technically accurate but "Corpus" better scopes it for cross-module visibility |
| B8 | beatport.rs | 26 | `HttpStatusHandling` | `HttpStatusOutcome` | Internal only | Represents outcome, not strategy |

---

## Section C — Struct Fields (API-facing, need `#[serde(rename)]`)

These are MCP tool parameter fields. Renaming the Rust name requires adding
`#[serde(rename = "old_name")]` to preserve the JSON API contract.

**Policy**: Rust-side renames use `#[serde(rename = "old_name")]` so the wire
format is unchanged. No deprecation of old wire names — they remain the
canonical JSON keys indefinitely. The rename is purely for internal code
clarity; MCP callers see no change.

**Testing**: Each rename in this section must be verified with existing serde
round-trip tests (e.g. `flatten_json_round_trip_*` in tests.rs). If no
round-trip test covers the renamed field, add one before merging.

| # | File | Line | Current | Proposed | Notes |
|---|------|------|---------|----------|-------|
| C1 | params.rs | 329 | `pool_track_ids` | `candidate_track_ids` | Docs already call these "candidates" |
| C2 | params.rs | 327,355 | `from_track_id` / `to_track_id` | `source_track_id` / `target_track_id` | More explicit direction |
| C3 | params.rs | 156 | `min_count` | `min_genre_count` | Unclear what's counted |
| C4 | params.rs | 297 | `start_track_id` | `opening_track_id` | DJ-set semantics |
| C5 | params.rs | 452 | `targets` | `target_audio_files` | Too generic |
| C6 | params.rs | 464,478,508 | `scope` | `path_prefix` | `directory_path` would narrow meaning; actual usage is prefix-match via `LIKE '...%'` with `enforce_trailing_slash()` |

---

## Section D — Struct Fields (internal, safe to rename)

| # | File | Line | Current | Proposed | Notes |
|---|------|------|---------|----------|-------|
| D1 | tools/resolve.rs | 13 | `default_max` | `default_max_tracks` | Scope unclear |
| D2 | tools/resolve.rs | 14 | `cap` | `max_tracks_cap` | Generic |
| ~~D3~~ | | | | | **REMOVED**: `key_camelot` is serialized into cached `features_json` blobs in SQLite and read back via `v.get("key_camelot")` in scoring.rs:568. Renaming breaks all existing caches. Would require `#[serde(alias)]` + cache migration — not worth the churn. |
| D4 | corpus.rs | 133 | `search_phrase` | `normalized_search_text` | Aligns with surrounding query naming |
| D5a | eval_tasks.rs | 21 | `min_prefix_matches` | `min_leading_matches` | "Prefix" misleads — refers to leading rank alignment, not string prefix |
| D5b | eval_tasks.rs | 29 | `expected_prefix_doc_ids` | `expected_leading_doc_ids` | Same as D5a |
| D5c | eval_tasks.rs | 43 | `prefix_matches` | `leading_matches` | Same as D5a |
| D5d | eval_tasks.rs | 47 | `expected_prefix_paths` | `expected_leading_paths` | Same as D5a |

---

## Section E — Bool Parameters (predicate readability)

| # | File | Line | Current | Proposed | Notes |
|---|------|------|---------|----------|-------|
| E1 | params.rs | 309,343,365 | `master_tempo` | `use_master_tempo` | **API-facing** — field appears in `BuildSetParams`, `QueryTransitionCandidatesParams`, `ScoreTransitionParams` with no serde rename. Needs `#[serde(rename = "master_tempo")]` on all three structs to preserve wire name. Same policy as Section C. |

---

## Section F — Constants

| # | File | Line | Current | Proposed | Notes |
|---|------|------|---------|----------|-------|
| F1 | db.rs | 10 | `DB_KEY` | `REKORDBOX_SQLCIPHER_KEY` | Makes domain explicit |
| F2 | beatport.rs | 6 | `BEATPORT_UA` | `BEATPORT_USER_AGENT` | Expands abbreviation |
| F3 | xml.rs | 8 | `REKORDBOX_VERSION` | `TARGET_REKORDBOX_VERSION` | Export target, not running version |
| F4 | eval_routing.rs | 7 | `SCORE_PASS_RATE_THRESHOLD` | `TOP1_SCORE_PASS_RATE_THRESHOLD` | Scope explicit |

---

## Section G — Magic Numbers → Named Constants (scoring.rs)

| # | Line(s) | Value(s) | Proposed Constant | Notes |
|---|---------|----------|--------------------|-------|
| G1 | 126-132 | `0.15`, `0.45`, `0.75` | `WARMUP_PHASE_END`, `BUILD_PHASE_END`, `PEAK_PHASE_END` | WarmupBuildPeakRelease preset boundaries |
| G2 | 138-143 | `0.10`, `0.85` | `PEAKONLY_BUILD_END`, `PEAKONLY_RELEASE_END` | PeakOnly preset boundaries |
| G3 | 265,427 | `0.7` | `BPM_DRIFT_PENALTY_FACTOR` | Applied in trajectory penalty |
| G4 | 720 | `0.5` | `HARMONIC_PENALTY_FACTOR` | Gate penalty for poor key match |
| G5 | 993-1003 | `300.0`, `800.0`, `1500.0` | `BRIGHTNESS_SIMILAR_HZ`, `BRIGHTNESS_SHIFT_HZ`, `BRIGHTNESS_JUMP_HZ` | Brightness axis thresholds |
| G6 | 1031-1041 | `0.1`, `0.25`, `0.5` | `RHYTHM_MATCHED_DELTA`, `RHYTHM_MANAGEABLE_DELTA`, `RHYTHM_CHALLENGING_DELTA` | Rhythm regularity thresholds |

---

## Section H — Local Variables (terse abbreviations)

Grouped by impact. Single-letter vars in short closures omitted.

### H1 — Scoring math (`scoring.rs`)

| # | Line | Current | Proposed |
|---|------|---------|----------|
| H1a | 525 | `bs`, `be` | `build_start_idx`, `build_end_idx` |
| H1b | 535 | `rs`, `re` | `release_start_idx`, `release_end_idx` |
| H1c | 632-637 | `eff_from_key`, `eff_to_key` | `effective_from_key`, `effective_to_key` | Directional pair — coupled |
| H1c' | 643 | `eff_to_display` | `effective_to_key_display` | Independent display var |
| H1d | 1092 | `w` | `weights` |
| H1e | 859-892 | `met` | `phase_requirement_met` |
| H1f | 226 | `ctx` | `scoring_context` |
| H1g | 176 | `out` | `start_track_ids` |
| H1h | 913 | `lra` (in match pattern) | `loudness_range` |

### H2 — Handler params (`tools/mod.rs`)

| # | Line | Current | Proposed |
|---|------|---------|----------|
| H2a | 984 | `p` | `params` | Destructured from `Parameters<EnrichTracksParams>` — `params` matches the source type |
| H2b | 1970 | `p` | `params` | Destructured from `Parameters<BuildSetParams>` — same pattern |
| H2c | 658 | `o` | `backup_output` |
| H2d | 2109 | `seen_ids` | `seen_track_sequences` |
| H2e | 2216 | `id` | `candidate_label` |
| H2f | 2277 | `dc` | `discogs_cache` |
| H2f' | 2279 | `bc` | `beatport_cache` |
| H2f'' | 2283 | `sc` | `stratum_cache` |
| H2f''' | 2285 | `ec` | `essentia_cache` |
| H2g | 2577 | `fc` | `selected_fields` |

### H3 — Database / store / XML

| # | File | Line | Current | Proposed |
|---|------|------|---------|----------|
| H3a | db.rs | 154 | `q` | `query_text` |
| H3b | db.rs | 155 | `idx` | `bind_index` |
| H3c | db.rs | 277 | `refs` | `bind_params` |
| H3d | db.rs | 369 | `attr` | `playlist_attribute` |
| H3e | store.rs | 14 | `p` | `store_path` |
| H3f | store.rs | 557 | `count` | `deleted_count` |
| H3g | xml.rs | 185 | `key` | `xml_track_id` |
| H3h | db.rs | 419 | `sample_filter_c` | `content_sample_filter` | Cryptic `_c` suffix |
| H3i | store.rs | 422 | `it` | `issue_type_filter` | Opaque abbreviation in `if let Some(it)` |

### H4 — Tags / audit

| # | File | Line | Current | Proposed |
|---|------|------|---------|----------|
| H4a | tags.rs | 591 | `tt` | `tag_type_str` |
| H4b | tags.rs | 593 | `art` | `cover_art_meta` |
| H4c | audit.rs | 609-610 | `v2`, `ri` | `id3v2_value`, `riff_info_value` |
| H4d | audit.rs | 750-751 | `fn_a`, `t_a` | `filename_artist_folded`, `tag_artist_folded` | Artist comparison pair |
| H4d' | audit.rs | 764-765 | `fn_t`, `t_t` | `filename_title_folded`, `tag_title_folded` | Title comparison pair |
| H4e | audit.rs | 1137 | `i` | `issue` |
| H4f | audit.rs | 1183 | `res` | `parsed_resolution` |
| H4g | tags.rs | 796 | `val` | `new_value` | In write path; clarifies this is the value being written |

### H5 — Changes / discogs / corpus / cli

| # | File | Line | Current | Proposed |
|---|------|------|---------|----------|
| H5a | changes.rs | 27 | `map` | `staged_changes_by_track_id` |
| H5b | changes.rs | 73 | `fields` | `field_diffs` |
| H5c | changes.rs | 283 | `map` | `changes_by_track_id` |
| H5d | discogs.rs | 525 | `resp` | `response` |
| H5e | discogs.rs | 548 | `data` | `search_response` |
| H5f | discogs.rs | 559,570 | `r` | `search_result` |
| H5g | tools/corpus_helpers.rs | 22 | `out` | `deduped_paths` |
| H5h | cli.rs | 241 | `idx` | `track_index` |
| H5i | cli.rs | 306 | `json` | `analysis_json` |
| H5j | audio.rs | 334 | `mss` | `media_source_stream` |
| H5k | audio.rs | 456 | `scale` | `channel_weight` |
| H5l | normalize.rs | 2 | `s` | `input` |
| H5m | eval_routing.rs | 273 | `floors` | `score_thresholds` |

---

## Section I — Test Function Names (tools/tests.rs)

Low-risk; improves `cargo test` output.

| # | Line | Current | Proposed |
|---|------|---------|----------|
| I1 | 536 | `build_set_custom_curve_and_small_pool_are_handled` | `build_set_adapts_energy_curve_to_single_track_pool` |
| I2 | 609 | `build_set_handles_all_same_key_pool` | `build_set_produces_candidates_from_homogeneous_key_pool` |
| I3 | 761 | `probe_essentia_python_rejects_success_without_version_output` | `probe_essentia_python_fails_when_no_version_string` |
| I4 | 787 | `validate_essentia_python_times_out` | `probe_essentia_python_returns_error_on_timeout` |
| I5 | 817 | `lookup_output_with_cache_metadata_normalizes_non_object_payloads` | `lookup_output_wraps_non_object_in_result_envelope` |
| I6 | 1165 | `write_xml_with_playlists_and_staged_changes_exports_union_once` | `write_xml_deduplicates_playlist_and_staged_tracks` |
| I7 | 3091 | `harmonic_style_modulation_gate` | `harmonic_style_conservative_penalizes_poor_transitions` |
| I8 | 4021 | `beam_search_k_larger_than_pool` | `beam_search_width_exceeding_pool_size` |

---

## Removed from Earlier Lists (verified incorrect or impractical)

| Source | Claim | Reason Removed |
|--------|-------|----------------|
| List 2, #12 | `db.rs:419` — `sample_filter_c` | ~~Variable doesn't exist~~ Stale claim; line exists. Moved to H3h. |
| List 2, #68 | `tags.rs:796` — `val` → `new_value` | ~~Line 796 beyond file length~~ Stale claim; line exists. Moved to H4g. |
| List 2, #14 | `store.rs:422` — `it` → `issue_type_filter` | ~~Line doesn't match~~ Stale claim; line exists. Moved to H3i. |
| List 1, #23 | `types.rs:84` — `key` → `musical_key` | Field maps to Rekordbox XML `Tonality`; no serde rename; would break wire format and XML export |
| List 1, #24 | `types.rs:87` — `color` → `color_name` | Same: maps to XML `Colour`, no serde rename |
| List 1, #25 | `types.rs:92` — `length` → `duration_seconds` | Same: maps to XML `TotalTime`, no serde rename |
| List 1, #63 | `tags.rs:153` — `tag3_missing` → `riff_info_missing_fields` | Name matches established WAV tagging convention (ID3v2 + RIFF INFO = "tag3"); rename would introduce confusion |
| List 2, #57 | `discogs.rs:490` — `lookup_inner_legacy` → `lookup_legacy_inner` | Word order change adds no clarity |
| List 1, #52 | `beatport.rs:16` — `kind` → `error_class` | `kind` is idiomatic Rust for error classification (matches `io::Error::kind()`) |
| List 1 | `params.rs` — `skip_cached`/`force_refresh` inversion | API-breaking semantic change, not just a rename |
| List 1 | `params.rs:20,22` — `bpm_min`/`bpm_max` | Standard JSON API naming; no change needed |
| List 2, #55 | `beatport.rs:161` — `query` → `dehydrated_query` | "Dehydrated" is non-standard jargon |
