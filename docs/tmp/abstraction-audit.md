# Abstraction Audit

Two independent agents scanned the codebase for pointless abstractions.
This document records the verdict on each finding after manual verification.

Findings are grouped into **fix** (worth changing) and **accept** (keep as-is).

---

## Fix

### F1. `analyze_essentia` / `cache_analysis` pass-throughs (`tools/analysis.rs:40-61`)

`analyze_essentia` is a one-liner forwarding to `audio::run_essentia().await.map_err(|e| e.to_string())`.
`cache_analysis` is a one-liner forwarding to `store::set_audio_analysis().map_err(...)`.

`check_analysis_cache` (line 7) and `analyze_stratum` (line 25) in the same file
do real work (cache-hit validation and `spawn_blocking` orchestration) so they
earn their keep. The other two don't.

**Action:** Inline `analyze_essentia` and `cache_analysis` at their ~6 call sites
in `mod.rs`. The `.map_err(|e| e.to_string())` / `.map_err(|e| format!("Cache write error: {e}"))`
can go inline. Delete the functions. If `analysis.rs` becomes just `check_analysis_cache` +
`analyze_stratum`, that's fine — those are non-trivial.

### F2. `resolve_file_path` pass-through (`tools/audio_scan.rs:78-80`)

```rust
pub(super) fn resolve_file_path(raw_path: &str) -> Result<String, McpError> {
    audio::resolve_audio_path(raw_path).map_err(|e| super::resolve::err(e.to_string()))
}
```

Used 10+ times across `mod.rs` and `scoring.rs`, but the function body is just
error-type conversion. The `err()` helper (F3) is already a one-liner itself, so
this is a wrapper calling a wrapper.

**Action:** Inline `audio::resolve_audio_path(raw).map_err(|e| err(e.to_string()))`
at call sites — or keep `resolve_file_path` but drop the `err()` indirection
inside it (use `McpError::internal_error(...)` directly). Pick one layer, not two.

### F3. `err()` helper (`tools/resolve.rs:8-10`)

```rust
pub(super) fn err(msg: String) -> McpError {
    McpError::internal_error(msg, None)
}
```

Used ~165 times in `mod.rs` plus a handful elsewhere. It saves typing but hides
the error constructor behind a 3-letter name with zero semantic signal.

**Action:** Replace with a descriptive name like `internal(msg)` or keep
`McpError::internal_error(msg, None)` inline. The repetition argument is real at
~165 call sites so a helper is justified — just give it a better name and move it
to `mod.rs` where it's used, rather than living in `resolve.rs`.

### F4. `get_alias_map()` clones on every call (`genre.rs:142-149`)

Builds a fresh `Vec<(String, String)>` by cloning the HashMap's key-value pairs
every invocation. Called from `mod.rs:407` (MCP `get_genre_taxonomy` tool) and one test.

**Action:** Return `&'static HashMap<String, &'static str>` (i.e. expose `alias_map()`
directly or rename it to `get_alias_map`). The caller in `mod.rs` can iterate
the reference. Eliminates allocation on every tool call.

### F5. `get_taxonomy()` allocates needlessly (`genre.rs:52-54`)

```rust
pub fn get_taxonomy() -> Vec<String> {
    GENRES.iter().map(|s| s.to_string()).collect()
}
```

Called from `mod.rs:405` and tests. `GENRES` is `&[&str]` — callers can iterate
it directly without allocating owned `String`s.

**Action:** Expose `GENRES` (already `pub`) and remove `get_taxonomy()`. The MCP
tool handler can serialize `GENRES` directly. Tests can use `GENRES` too.

### F6. `IssueType::as_str()` duplicates strum (`audit.rs:56-75`)

The enum derives `strum::EnumString` (parse from string) and has manual `as_str()`
for the reverse direction. strum's `IntoStaticStr` or `AsRefStr` derive would
generate `as_str()` automatically from the same `#[strum(serialize = "...")]`
attributes already present.

The manual `Display` impl (lines 104-108) delegates to `as_str()`, which would also
be handled by `strum::Display`.

**Action:** Add `strum::Display` derive and delete the manual `Display` impl.
Note: strum's `IntoStaticStr` generates `From<T> for &'static str` / `AsRef<str>`,
not an inherent `as_str()` method — so the ~6 `.as_str()` call sites would need
migration to `.as_ref()` or `.into()`. Alternatively, keep the manual `as_str()`
and only replace `Display` with the strum derive. Same pattern applies to
`IssueType` only — `AuditStatus` and `Resolution` don't derive strum (see A4 below).

---

## Accept

### A1. `SearchFilterParams` wrapper (`tools/params.rs:10-67`)

Flagged as "double-hop" since it converts 1:1 to `db::SearchParams`. But it
exists for a good reason: it carries `#[schemars(description)]` annotations for
MCP JSON schema generation, which `db::SearchParams` shouldn't know about.
`db::SearchParams` also has `playlist`, `exclude_samples`, `limit`, and `offset`
fields that aren't user-facing filter params. The separation is a real boundary
(MCP presentation vs DB query construction). A previous audit already validated
this extraction as correct (see `docs/developer/code-quality-audit.md:58`).

**Verdict:** Keep.

### A2. `ResolveTracksOpts` config struct (`tools/resolve.rs:12-21`)

Three fields with doc comments controlling `resolve_tracks()` behavior. Used by
4 different callers with different combinations. Passing 3 extra positional
booleans/options would hurt readability at call sites — named fields are clearer
than `resolve_tracks(conn, ids, pid, filters, max, Some(50), Some(200), true)`.

**Verdict:** Keep.

### A3. `HttpStatusHandling` enum (`beatport.rs:26-29`)

Flagged as redundant with `Result<Option<T>, E>`. But `classify_http_status`
(line 85-106) is a pure classification function that doesn't have access to the
response body — it can only say "no match" or "error". The caller then maps
`NoMatch -> Ok(None)` and `Error(e) -> Err(e)`. The enum makes the classification
function's return type honest: it doesn't produce a success value, it classifies
HTTP status codes. It's also used in 4 test assertions. This is fine.

**Verdict:** Keep.

### A4. `AuditStatus` / `Resolution` manual `as_str()` + `from_str()` (`audit.rs:120-188`)

These enums don't derive strum. The string representations use snake_case
(`accepted_as_is`, `wont_fix`) which are DB column values — not the SCREAMING_CASE
used by `IssueType`. Adding strum derives here would require `#[strum(serialize)]`
on every variant anyway, and the manual implementations are 4 variants each.
The cognitive overhead of adding strum annotations for 8 lines of match
arms isn't worth it (strum is already a dependency via `IssueType`).

**Verdict:** Keep.

### A5. `CorpusQuery` + `NormalizedQuery` dual structs (`corpus.rs:89-136`)

`CorpusQuery` is the public API with borrowed `&str` for callers.
`NormalizedQuery` is the internal search representation with owned, lowercased,
split terms. The `From` conversion (line 138) does real work: lowercasing,
whitespace splitting, zero-filtering. This is a legitimate
parse-don't-validate pattern. The lifetime boundary (`'a` → owned) is intentional.

**Verdict:** Keep.

### A6. `IndexedDocument` pre-computed fields (`corpus.rs:114-126`)

Stores lowercased copies of all searchable fields for O(1) comparison during
search. The alternative (compute on every query) is worse for a corpus that's
built once and searched repeatedly. The "duplication" is a deliberate search
index, not accidental.

**Verdict:** Keep.

### A7. `PlaylistDef` struct (`xml.rs:10-14`)

A named struct with two fields: `name` and `track_ids`. Used in the XML export
API surface, test assertions, and the MCP tool handler. A tuple `(String, Vec<String>)`
would be less readable at every usage site (`.0` / `.1` vs `.name` / `.track_ids`).
Named fields are not over-abstraction.

**Verdict:** Keep.

### A8. `tools/` micro-module split

Flagged as over-split. Actual sizes: `analysis.rs` (61), `audio_scan.rs` (80),
`enrichment.rs` (125), `essentia.rs` (148), `corpus_helpers.rs` (178),
`resolve.rs` (164), `scoring.rs` (885), `params.rs` (444). The alternative is
a single `mod.rs` that's already 2817 lines — adding these back would make it
~5000 lines. The modules have clear domain boundaries (audio analysis, essentia
probing, enrichment test doubles, corpus retrieval, scoring/set-building).

The `use module::*` glob imports are a fair critique but a separate concern
(import style, not abstraction). Switching to explicit imports would be nice but
low-value given all items are `pub(super)`.

**Verdict:** Keep the split. Optionally switch to explicit imports later.

### A9. `should_run_cli` generic bounds (`main.rs:23-35`)

```rust
fn should_run_cli<I, S>(mut args: I) -> bool
where I: Iterator<Item = S>, S: AsRef<str>
```

The generic bounds let tests pass `vec!["reklawdbox", "analyze"].into_iter()`
(yields `&str`) while production passes `std::env::args()` (yields `String`).
This is a textbook use of generics for testability — not over-abstraction.

**Verdict:** Keep.

### A10. `StatusCounts` aggregation struct (`audit.rs:141-160`)

Named fields (`open`, `resolved`, `accepted`, `deferred`) are clearer than a
4-tuple. Used in one place but makes the aggregation loop readable. Tiny struct,
no methods beyond construction.

**Verdict:** Keep.

### A11. `Provider::as_str()` + `Display` (`types.rs:217-234`)

`Display` delegates to `as_str()`. This is a standard Rust pattern — `as_str()`
is useful independently (e.g. as a map key, in format strings without allocation)
while `Display` enables `{provider}` formatting. Not redundant.

**Verdict:** Keep.

### A12. `FileKind::as_kind_str()` (`types.rs:42-52`)

Returns Rekordbox XML `Kind` attribute values ("MP3 File", etc.). `Serialize`
uses `as_kind_str()` for JSON output. These are the same representation — not
duplication, just one calling the other.

**Verdict:** Keep.

---

## Summary

| ID | Location | Verdict | Effort |
|----|----------|---------|--------|
| F1 | `tools/analysis.rs:40-61` | Fix — inline 2 pass-throughs | S |
| F2 | `tools/audio_scan.rs:78-80` | Fix — collapse wrapper chain | S |
| F3 | `tools/resolve.rs:8-10` | Fix — rename or inline | S |
| F4 | `genre.rs:142-149` | Fix — return reference | S |
| F5 | `genre.rs:52-54` | Fix — use `GENRES` directly | S |
| F6 | `audit.rs:56-75` | Fix — use strum derive | S |
| A1-A12 | various | Accept | — |
