# Code Quality Audit — AI Agent Perspective

Conducted 2026-02-26. Covers all 18 Rust files in `src/` (~21k lines).

Evaluates how easy this codebase is for an LLM coding agent to correctly read,
modify, extend, and avoid introducing bugs in. Not concerned with human
readability or style.

---

## Findings by Severity

### Critical

These will likely cause agent-introduced bugs on the next non-trivial change.

#### C1. `TrackChange` field set enumerated in 8+ locations with no compile-time enforcement

Adding a new editable field (e.g., `label: Option<String>`) requires updating
8 locations in perfect sync. None are compiler-enforced:

| # | Location | What it does |
|---|----------|-------------|
| 1 | `types.rs:45-48` | `TrackChange` struct definition |
| 2 | `changes.rs:239-244` | `has_any_staged_field` — `is_some()` per field |
| 3 | `changes.rs:246-258` | `merge_track_change` — overwrites per field |
| 4 | `changes.rs:261-273` | `merge_missing_fields` — fills per field |
| 5 | `changes.rs:276-301` | `apply_changes_with_map` — applies per field |
| 6 | `changes.rs:165-215` | `clear_fields` — string-match with `_ => {}` catch-all |
| 7 | `changes.rs:54-121` | `preview` — diff generation per field |
| 8 | `tools.rs:787-798` | `TrackChangeInput` — mirrors struct with schemars |
| 9 | `tools.rs:1654` | `VALID_FIELDS` — hardcoded string list |

`clear_fields` uses string dispatch — a misspelled field silently does nothing.
`changes.rs:205-210` has an inline all-fields-None check that also needs updating.

**Fix**: Macro or derive to enumerate fields once; or refactor fields into a
`BTreeMap<FieldName, Value>` with a `FieldName` enum.

#### C2. ~~`SearchParams` construction copy-pasted 6 times~~ ✅ Resolved

`SearchParams` now derives `Default`. All 6 tool-site constructions replaced
with `SearchFilterParams::into_search_params()`. All 18 test constructions use
`..Default::default()`. Adding a new filter field only requires updating
`SearchFilterParams` and `into_search_params`.

#### C3. `tools.rs` is 8138 lines — exceeds agent context windows

Contains logically unrelated concerns: MCP tool definitions, parameter structs,
transition scoring engine (Camelot math, energy curves), corpus helpers,
Essentia setup, audio scanning, genre family mapping, ~3100 lines of tests.

**Fix**: Split into modules: `tools/mod.rs`, `tools/params.rs`,
`tools/scoring.rs`, `tools/enrichment.rs`, `tools/analysis.rs`.

#### C4. `IssueType` enum: 7 update sites, only `as_str()` is compiler-enforced

| # | Location | Compiler-enforced? |
|---|----------|-------------------|
| 1 | `audit.rs:42-59` `as_str()` | Yes (exhaustive match) |
| 2 | `audit.rs:62-80` `from_str()` | No (`_ => None`) |
| 3 | `audit.rs:83-89` `safety_tier()` | No (`_ => Review`) |
| 4 | `audit.rs:396-540` / `545-693` detection logic | No |
| 5 | `audit.rs:1569-1585` round-trip test array | No |
| 6 | `audit.rs:1597-1611` safety_tiers test | No |
| 7 | `check_tags`/`check_filename` skip handling | No |

A new variant that only updates `as_str()` compiles and passes tests but never
detects anything and silently defaults to `Review` tier.

**Fix**: `strum::EnumString` derive for `from_str`; remove `_ =>` catch-all
from `safety_tier()`.

#### C5. `OnceLock` caches credential errors permanently

`discogs.rs:421-441` — `CREDENTIALS` is `static OnceLock<Result<..>>`. First
call failure is cached for process lifetime. Env vars set after startup have no
effect. Not retryable without restart.

Same pattern in `corpus.rs:11,299` — manifest load errors cached forever.

**Fix**: Use `tokio::sync::OnceCell` with a retry-capable initializer, or
return `Result` from a non-caching function and let callers cache.

---

### High

Significant friction; partial bugs likely on non-trivial changes.

#### H1. ~~4+ duplicate parameter structs with identical filter fields~~ ✅ Resolved

Extracted `SearchFilterParams` with 12 shared filter fields. All 4 param
structs now use `#[serde(flatten)] pub filters: SearchFilterParams`.
`SearchFilterParams::into_search_params()` provides the conversion to
`db::SearchParams`. MCP tool JSON schemas are unchanged (`schemars` inlines
flattened fields).

#### H2. Track resolution pattern duplicated 4 times with subtle differences

`tools.rs` — "resolve tracks by priority: ids > playlist > search" in
`enrich_tracks`, `analyze_audio_batch`, `resolve_tracks_data`, `cache_coverage`.
The `cache_coverage` variant uses unbounded queries and sampler filtering.

**Fix**: Extract `resolve_tracks(params, conn, opts) -> Vec<Track>`.

#### H3. ~~`AUDIO_EXTENSIONS` defined identically in 3 files~~ ✅ Resolved

Single `pub(crate) const AUDIO_EXTENSIONS` in `audio.rs`, referenced by
`cli.rs`, `audit.rs`, and `tools.rs`.

#### H4. Audio analysis cache flow duplicated between single and batch tools

`tools.rs:2121-2226` (single) vs `tools.rs:2349-2549` (batch). Stratum-dsp
and Essentia flows (check cache → decode → analyze → cache → return) are
independent implementations with different error handling.

**Fix**: Extract `analyze_and_cache(path, store, analyzers) -> AnalysisResult`.

#### H5. Provider dispatch via raw strings instead of enum

`tools.rs:1891-1898,1957-2069` — `"discogs"` / `"beatport"` validated and
dispatched as bare strings. Adding a provider requires 3 manual updates.

**Fix**: `enum Provider { Discogs, Beatport }` with `Deserialize`.

#### H6. Genre-to-family mapping disconnected from genre taxonomy

`tools.rs:4649-4663` hardcodes genre → family mappings with no reference to
`genre.rs:6-50`. Verified missing families: `Gospel House`, `Progressive House`,
`Drone Techno`. String mismatches: `"drum and bass"` vs `"Drum & Bass"`,
`"rnb"` vs `"R&B"`. New taxonomy genres silently fall to `GenreFamily::Other`.

**Fix**: Derive family mapping from `genre.rs` data; or add a `family` field to
the genre taxonomy itself.

#### H7. `TRACK_SELECT` string replacement silently fragile

`db.rs:311-315` — Injects `Position` column via `.replace("\nFROM djmdContent c",
...)`. Whitespace change to `TRACK_SELECT` causes silent no-op. No assertion
that the replacement produced different output.

**Fix**: `assert_ne!(base_sql, TRACK_SELECT)` after replacement; or compose SQL
from parts instead of string surgery.

#### H8. Audit status/resolution strings cross module boundaries untyped

`audit.rs:952,1094` and `store.rs:524-528` — Statuses (`"open"`, `"resolved"`,
`"accepted"`, `"deferred"`) and resolutions (`"accepted_as_is"`, `"wont_fix"`,
`"deferred"`) are bare string literals. `store.rs` maps with `_ => "resolved"`
catch-all.

**Fix**: `enum AuditStatus` and `enum Resolution` with `as_str()` methods.

#### H9. Dynamic SQL parameter numbering in `get_audit_issues`

`store.rs:438-485` — Positional params (`?4`, `?5`) shift per filter
combination. 4-way match for binding. Third filter would need 8 arms.

**Fix**: Use `Vec<Box<dyn ToSql>>` and push params dynamically; or use
`rusqlite::named_params!`.

#### H10. Embedded 185-line Python script with no schema contract

`audio.rs:32-216` — Essentia script as raw string. Output parsed as untyped
`serde_json::Value`. No Rust struct defines expected keys. Errors are runtime-only.

**Fix**: Define `EssentiaOutput` struct with `Deserialize`; validate output
against it.

#### H11. Beatport HTML parsing silently fragile

`beatport.rs:102-192` — Scrapes `__NEXT_DATA__` via string search. Every
failure point returns `Ok(None)` — indistinguishable from "no results."
No logging at fallback points.

**Fix**: Add `tracing::debug!` at each fallback; consider returning a
`NotFound` vs `ParseError` distinction.

#### H12. String-typed closed sets: `FieldDiff.field`, `NormalizationSuggestion.confidence`

`types.rs:52-56,96` — `field` only takes 4 values, `confidence` only takes 3.
Both are `String`. Consumers match with catch-all fallbacks.

**Fix**: `enum EditableField { Genre, Comments, Rating, Color }` and
`enum Confidence { Alias, Unknown, Canonical }`.

#### H13. Blocking subprocess in async tool handler

`tools.rs:1546` — `std::process::Command` (backup script) runs synchronously
inside an async MCP tool handler. Can stall a Tokio worker thread.

**Fix**: Use `tokio::process::Command` or `spawn_blocking`.

---

### Medium

Notable issues. Manageable with awareness but easy to get wrong.

#### Duplication

| ID | Issue | Files |
|----|-------|-------|
| M1 | ~~Duplicate `escape_like`~~ ✅ Resolved — `store.rs` now imports `crate::db::escape_like` | |
| M2 | ~~Duplicate `urlencoding`~~ ✅ Resolved — `beatport.rs` now imports `crate::discogs::urlencoding` | |
| M3 | Essentia storage code duplicated in two `cli.rs` branches | `cli.rs:461-475`, `cli.rs:492-507` |
| M4 | Status aggregation duplicated between `scan()` and `get_summary()` | `audit.rs:1009`, `audit.rs:1134` |
| M5 | Disc-subdir detection duplicated | `audit.rs:167`, `audit.rs:199` |

#### Module Boundaries & Coupling

| ID | Issue | Files |
|----|-------|-------|
| M6 | `normalize()` in `discogs.rs` used universally (28+ call sites in tools.rs) | `discogs.rs:151` |
| M7 | Inconsistent whitespace: `genre::canonical_casing` trims, `color::canonical_casing` doesn't | `genre.rs:57`, `color.rs:23` |
| M8 | Inconsistent normalization: Discogs strips punctuation, Beatport preserves it | `discogs.rs:151`, `beatport.rs:195` |
| M9 | Inconsistent matching: Discogs uses `contains`, Beatport uses exact equality | `discogs.rs:583`, `beatport.rs:207` |
| M10 | `color_code == 0` means "no color" — black (`0x000000`) unrepresentable | `xml.rs:126`, `changes.rs:293` |
| M11 | Rekordbox XML attribute names differ from struct fields (`Name`/`title`, `Tonality`/`key`) | `xml.rs:80-106` |

#### Error Handling & Silent Failures

| ID | Issue | Files |
|----|-------|-------|
| M12 | Reopened audit issues keep stale `resolution`/`resolved_at`/`note` | `store.rs:429,432,534` |
| M13 | `get_tracks_by_ids` doesn't preserve caller order, deduplicates | `db.rs:522,532` |
| M14 | Dry-run tag diff can claim RIFF changes that write path will skip | `tags.rs:864,874,748` |
| M15 | Beatport title matching: permissive bidirectional substring, false-positive prone | `beatport.rs:220` |
| M16 | Discogs short artist names (<3 chars) auto-match first result | `discogs.rs:580,547` |
| M17 | Audio decode tolerates frame errors with only stderr logging | `audio.rs:342,357` |
| M18 | Corrupt cache JSON becomes `null` while response still says "cache hit" | `tools.rs:1734,1740` |
| M19 | `DJPlayCount` dual-type parse failures collapse to 0 | `db.rs:63-70` |
| M20 | `TECH_SPEC_PATTERNS` lists `[FLAC]`/`[flac]` but misses mixed-case | `audit.rs:116-121` |
| M21 | `date` field in `check_tags` not in `tags::ALL_FIELDS` — no-op for non-WAV | `audit.rs:448` |
| M22 | Corpus manifest path is cwd-relative, creating non-local behavior | `corpus.rs:9,180` |

#### Type Safety

| ID | Issue | Files |
|----|-------|-------|
| M23 | `Track` struct is flat primitives with no newtypes (`rating: u8`, `file_type: i32`) | `types.rs:4-30` |
| M24 | Priority weights returned as anonymous 6-tuple | `tools.rs:4564-4571` |
| M25 | Energy computation uses undocumented magic numbers | `tools.rs:4605-4633` |
| M26 | `write_track` two-phase attribute writing (main write + conditional appends + close) | `xml.rs:75-131` |
| M27 | Migration mixes unconditional `CREATE TABLE IF NOT EXISTS` with version-gated blocks | `store.rs:55-113` |
| M28 | Migration logic assumes `user_version` implies audit tables exist | `store.rs:85,755` |
| M29 | All errors in `audio.rs`, `tags.rs`, `beatport.rs` are `Result<_, String>` | Multiple |
| M30 | `serde(untagged)` enums in `tags.rs` make deser errors opaque | `tags.rs:117-217` |
| M31 | Analyzer name strings (`"stratum-dsp"`, `"essentia"`) used as DB keys with no constant | `cli.rs:223,231,446,470` |

---

### Low

Minor issues. Documented for completeness.

| ID | Issue | Files |
|----|-------|-------|
| L1 | Manual SQL `BEGIN`/`COMMIT` with `?` exits risks partial transactions | `audit.rs:891,996` |
| L2 | Unreadable directories in CLI expansion silently ignored | `cli.rs:546` |
| L3 | Malformed broker URL treated as missing config | `discogs.rs:24` |
| L4 | Legacy Discogs HTTP errors drop response-body diagnostics | `discogs.rs:530` |
| L5 | Issue detail JSON parse failure silently dropped | `audit.rs:1057` |
| L6 | Poisoned mutex silently recovered in staged changes | `changes.rs:20` |
| L7 | `file_type_to_kind` catch-all `_ => "Audio File"` | `types.rs:99-108` |
| L8 | Hardcoded Rekordbox version `"7.2.10"` in XML | `xml.rs:146-148` |
| L9 | Hardcoded User-Agent with Chrome 91 (2021) in Beatport scraper | `beatport.rs:4-5` |
| L10 | Double-negative CLI flag `--no-skip-cached` | `cli.rs:68` |
| L11 | `SAMPLER_PATH_PREFIX` hardcoded to `/Users/vz/...` | `db.rs:110` |
| L12 | Backup script discovery is cwd-relative | `tools.rs:1541-1558` |
| L13 | `stars_to_rating(6)` silently returns 255; `rating_to_stars(300)` returns 0 | `types.rs:112-135` |
| L14 | No-op `touch_cached_*` functions (dead scaffolding) | `store.rs:125,192,259` |
| L15 | No-op writes still trigger file rewrites in tags | `tags.rs:768` |

---

### Context-Dependent (Partially Confirmed)

These are technically present but mitigated by other code paths today. Worth
monitoring because the mitigation is in a different module — an agent modifying
one side could break the contract.

| ID | Issue | Mitigation | Risk if mitigation changes |
|----|-------|-----------|--------------------------|
| P1 | Unknown resolution coerces to `"resolved"` in store layer | Tool layer validates first (`audit.rs:1094`) | Direct store calls bypass validation |
| P2 | Duplicate track IDs overwrite in XML mapping | `write_xml` deduplicates first | Direct `generate_xml_with_playlists` calls don't |
| P3 | Hard-coded CLI/server dispatch string list | Currently aligned with declared commands | Adding CLI command without updating `main.rs:29` |
| P4 | Raw string date comparisons in SQL | Safe if ISO-8601 format invariant holds | User-supplied dates could violate |
| P5 | `mtime` fallbacks use epoch 0 | Only risky if `modified()` fails frequently | Platform-specific edge cases |

---

## Systemic Themes

### 1. String-typed closed sets instead of enums

The single most pervasive anti-pattern. Affects: editable fields, audit
statuses, resolutions, issue types (partially), provider names, confidence
levels, analyzer names, field diff names. Converting to enums makes the
compiler catch omissions when adding variants.

**Affected findings**: C1, C4, H5, H8, H12, M31

### 2. Copy-paste with subtle variation

SearchParams construction (C2), track resolution (H2), audio analysis caching
(H4), parameter structs (H1), status aggregation (M4), AUDIO_EXTENSIONS (H3),
escape_like (M1), urlencoding (M2). Each copy has minor differences that make
extraction tricky but not impossible.

**Affected findings**: C2, H1-H4, M1-M5

### 3. Cross-module implicit contracts

Color `0 = unset` (M10), Python Essentia output keys (H10), genre taxonomy →
family mapping (H6), TRACK_SELECT whitespace (H7), XML attribute names vs
struct fields (M11), status/resolution strings (H8). Modifying one side of
these contracts gives no signal about the other.

**Affected findings**: H6-H8, H10, M10, M11

### 4. Monolith file

`tools.rs` at 8k lines is the #1 barrier to agent productivity. All the
duplication findings (C2, H1, H2, H4) live here and would be more apparent
and fixable after splitting.

**Affected findings**: C3, and indirectly C2, H1, H2, H4, H5

### 5. Silent error swallowing

Multiple sites return `Ok(None)` or `Ok(0)` where an error occurred. This is
particularly dangerous for agents because there's no signal that something
went wrong — the agent assumes success and moves on.

**Affected findings**: H11, M12, M15-M19, L3-L6

---

## Suggested Fix Priority

Highest-impact changes ordered by (bug prevention * effort ratio):

1. **Derive `Default` for `SearchParams`** — trivial change, eliminates C2
2. **`enum EditableField`** — moderate change, eliminates C1 + H12
3. **Split `tools.rs`** — moderate effort, unlocks all other refactors (C3)
4. **`enum AuditStatus` + `enum Resolution`** — small change, eliminates H8
5. **`strum` derives for `IssueType`** — small change, partially fixes C4
6. **Extract `SearchFilterParams`** — moderate, eliminates H1
7. **Extract `resolve_tracks()` helper** — moderate, eliminates H2
8. **Shared `AUDIO_EXTENSIONS` constant** — trivial, eliminates H3
9. **`enum Provider`** — small, eliminates H5
10. **`EssentiaOutput` struct** — moderate, eliminates H10
