# reklawdbox Project Audit

Date: 2026-03-09

Scope reviewed:
- Rust MCP server, cache/store layer, enrichment, audio analysis, XML export, and scoring paths
- Discogs broker Worker and D1 schema
- GitHub Actions / CI / release workflows

Validation run:
- `cargo test` -> 391 passed, 0 failed, 28 ignored
- `cd broker && npm test` -> 8 passed, 0 failed

## Findings

### 1. Critical: the Discogs broker client token is hard-coded into the Rust client and used automatically

Evidence:
- [src/discogs.rs#L8](/Users/vz/projects/reklawdbox/src/discogs.rs#L8)
- [src/discogs.rs#L9](/Users/vz/projects/reklawdbox/src/discogs.rs#L9)
- [src/discogs.rs#L35](/Users/vz/projects/reklawdbox/src/discogs.rs#L35)
- [src/discogs.rs#L36](/Users/vz/projects/reklawdbox/src/discogs.rs#L36)

Impact:
- Anyone with the source or compiled binary can extract the token and impersonate a trusted broker client.
- The fallback also makes the "broker not configured" path effectively unreachable, which hides deployment mistakes and weakens the broker's trust boundary.

Recommendation:
- Remove the compiled fallback token entirely.
- Require `REKLAWDBOX_DISCOGS_BROKER_TOKEN` for protected broker access, and fail closed when it is absent.

### 2. High: the broker stores Discogs OAuth secrets in plaintext in D1

Evidence:
- [broker/migrations/0001_initial.sql#L10](/Users/vz/projects/reklawdbox/broker/migrations/0001_initial.sql#L10)
- [broker/migrations/0001_initial.sql#L11](/Users/vz/projects/reklawdbox/broker/migrations/0001_initial.sql#L11)
- [broker/migrations/0001_initial.sql#L25](/Users/vz/projects/reklawdbox/broker/migrations/0001_initial.sql#L25)
- [broker/migrations/0001_initial.sql#L26](/Users/vz/projects/reklawdbox/broker/migrations/0001_initial.sql#L26)
- [broker/src/index.ts#L665](/Users/vz/projects/reklawdbox/broker/src/index.ts#L665)
- [broker/src/index.ts#L719](/Users/vz/projects/reklawdbox/broker/src/index.ts#L719)
- [broker/src/index.ts#L783](/Users/vz/projects/reklawdbox/broker/src/index.ts#L783)

Impact:
- Read access to the broker database is enough to replay user-linked Discogs credentials outside the service.
- This defeats the intended "broker-side only" protection for OAuth secrets.

Recommendation:
- Encrypt request-token secrets and access-token secrets before storing them, using an application-level key from a Worker secret or KMS-backed mechanism.
- Treat D1 as untrusted storage.

### 3. High: XML export cannot round-trip several existing Rekordbox fields and instead emits hard-coded blanks/zeros

Evidence:
- [src/xml.rs#L77](/Users/vz/projects/reklawdbox/src/xml.rs#L77)
- [src/xml.rs#L87](/Users/vz/projects/reklawdbox/src/xml.rs#L87)
- [src/xml.rs#L92](/Users/vz/projects/reklawdbox/src/xml.rs#L92)
- [src/xml.rs#L94](/Users/vz/projects/reklawdbox/src/xml.rs#L94)
- [src/xml.rs#L95](/Users/vz/projects/reklawdbox/src/xml.rs#L95)
- [src/xml.rs#L108](/Users/vz/projects/reklawdbox/src/xml.rs#L108)
- [src/types.rs#L76](/Users/vz/projects/reklawdbox/src/types.rs#L76)
- [src/db.rs#L27](/Users/vz/projects/reklawdbox/src/db.rs#L27)

Impact:
- The export path always writes `Composer=""`, `Grouping=""`, `Size="0"`, `DiscNumber="0"`, `TrackNumber="0"`, and `Mix=""` even when the source library may already contain real values.
- On Rekordbox reimport, this creates a metadata-loss risk for fields the server never loaded and therefore cannot preserve.

Recommendation:
- Extend the `Track` model and DB query to load every XML field the exporter writes, then preserve original values unless the user explicitly changed them.
- Until that is done, avoid emitting attributes whose original value is unknown.

### 4. High: Discogs enrichment cache keys ignore album, so different releases can overwrite each other

Evidence:
- [src/tools/enrich_handlers.rs#L15](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L15)
- [src/tools/enrich_handlers.rs#L41](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L41)
- [src/tools/enrich_handlers.rs#L63](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L63)
- [src/tools/enrich_handlers.rs#L222](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L222)
- [src/tools/enrich_handlers.rs#L323](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L323)
- [src/store.rs#L55](/Users/vz/projects/reklawdbox/src/store.rs#L55)
- [src/store.rs#L62](/Users/vz/projects/reklawdbox/src/store.rs#L62)
- [src/store.rs#L116](/Users/vz/projects/reklawdbox/src/store.rs#L116)
- [broker/src/index.ts#L758](/Users/vz/projects/reklawdbox/broker/src/index.ts#L758)

Impact:
- Two tracks with the same normalized artist/title but different albums, edits, or remix contexts can clobber each other's cached Discogs result.
- The Rust client can then return the wrong metadata or a wrong "no match" result for later lookups.

Recommendation:
- Add normalized album to the cache key and schema on the Rust side to match the broker query shape.
- Backfill or invalidate old rows during migration.

### 5. High: transient enrichment failures are cached as durable null hits

Evidence:
- [src/tools/enrich_handlers.rs#L44](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L44)
- [src/tools/enrich_handlers.rs#L132](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L132)
- [src/tools/enrich_handlers.rs#L262](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L262)
- [src/tools/enrich_handlers.rs#L388](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L388)
- [src/tools/enrich_handlers.rs#L489](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L489)
- [src/store.rs#L116](/Users/vz/projects/reklawdbox/src/store.rs#L116)

Impact:
- A temporary 429, broker outage, or network error becomes a sticky false negative because later reads only check for row existence, not whether the row represents an error.
- Batch runs will also skip those tracks as "cached" on future runs.

Recommendation:
- Do not treat `match_quality = "error"` as a cache hit.
- Prefer either not caching transient failures at all, or storing them with a short TTL and explicit retry semantics.

### 6. High: `write_xml` continues after backup failure even though backup is presented as part of the safety path

Evidence:
- [src/tools/staging_handlers.rs#L226](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L226)
- [src/tools/staging_handlers.rs#L239](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L239)
- [src/tools/staging_handlers.rs#L250](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L250)
- [src/tools/staging_handlers.rs#L254](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L254)
- [src/tools/staging_handlers.rs#L255](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L255)

Impact:
- Non-zero backup exit status, launch failure, and task failure are all downgraded to warnings, then export proceeds anyway.
- That makes the advertised "Runs backup first" protection advisory rather than enforced at the exact point where users most expect a hard stop.

Recommendation:
- Fail the export when the backup script exists but does not complete successfully.
- If backup is optional, surface that explicitly in the tool contract and make the mode user-selectable.

### 7. Medium: scoring and resolve paths consume stale audio-analysis rows without freshness checks

Evidence:
- [src/tools/analysis.rs#L5](/Users/vz/projects/reklawdbox/src/tools/analysis.rs#L5)
- [src/tools/analysis.rs#L15](/Users/vz/projects/reklawdbox/src/tools/analysis.rs#L15)
- [src/tools/resolve_handlers.rs#L28](/Users/vz/projects/reklawdbox/src/tools/resolve_handlers.rs#L28)
- [src/tools/resolve_handlers.rs#L90](/Users/vz/projects/reklawdbox/src/tools/resolve_handlers.rs#L90)
- [src/tools/scoring.rs#L613](/Users/vz/projects/reklawdbox/src/tools/scoring.rs#L613)
- [src/store.rs#L64](/Users/vz/projects/reklawdbox/src/store.rs#L64)
- [src/store.rs#L253](/Users/vz/projects/reklawdbox/src/store.rs#L253)

Impact:
- `score_transition`, `build_set`, `resolve_track_data`, and `resolve_tracks_data` can keep using obsolete descriptors after a file changes or a schema version is bumped.
- The dedicated analysis path already has the right freshness check, so behavior is inconsistent across tools.

Recommendation:
- Route all audio-cache reads through a single freshness validator that checks `file_size`, `file_mtime`, and `analysis_version`.
- Invalidate or ignore stale rows everywhere, not just in the analyze handlers.

### 8. Medium: batch audio analysis has a high-memory execution model

Evidence:
- [src/audio.rs#L377](/Users/vz/projects/reklawdbox/src/audio.rs#L377)
- [src/audio.rs#L417](/Users/vz/projects/reklawdbox/src/audio.rs#L417)
- [src/tools/analysis.rs#L29](/Users/vz/projects/reklawdbox/src/tools/analysis.rs#L29)
- [src/tools/analysis.rs#L32](/Users/vz/projects/reklawdbox/src/tools/analysis.rs#L32)
- [src/tools/audio_handlers.rs#L373](/Users/vz/projects/reklawdbox/src/tools/audio_handlers.rs#L373)
- [src/tools/audio_handlers.rs#L432](/Users/vz/projects/reklawdbox/src/tools/audio_handlers.rs#L432)

Impact:
- Each track is fully decoded into a `Vec<f32>`, then kept alive across multiple blocking phases.
- Batch mode can run up to 16 tracks concurrently while also invoking Essentia, so long WAV/AIFF inputs can drive memory usage into the GB range and destabilize the server.

Recommendation:
- Reduce default concurrency for analysis-heavy workloads.
- Stream or chunk decode where possible, or cap accepted duration/sample count before full decode.

### 9. Medium: the broker reflects raw exception messages back to callers

Evidence:
- [broker/src/index.ts#L147](/Users/vz/projects/reklawdbox/broker/src/index.ts#L147)
- [broker/src/index.ts#L150](/Users/vz/projects/reklawdbox/broker/src/index.ts#L150)
- [broker/src/index.ts#L1177](/Users/vz/projects/reklawdbox/broker/src/index.ts#L1177)
- [broker/src/index.ts#L864](/Users/vz/projects/reklawdbox/broker/src/index.ts#L864)
- [broker/src/index.ts#L1395](/Users/vz/projects/reklawdbox/broker/src/index.ts#L1395)

Impact:
- Callers can learn deployment and upstream details that should stay server-side, including missing secret names and Discogs error behavior.
- This increases information disclosure without helping legitimate clients recover.

Recommendation:
- Return stable public error messages to clients and log raw exceptions server-side only.

### 10. Medium: a failed XML export can resurrect fields that the user cleared while the export was in flight

Evidence:
- [src/tools/staging_handlers.rs#L213](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L213)
- [src/tools/staging_handlers.rs#L274](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L274)
- [src/tools/staging_handlers.rs#L320](/Users/vz/projects/reklawdbox/src/tools/staging_handlers.rs#L320)
- [src/changes.rs#L159](/Users/vz/projects/reklawdbox/src/changes.rs#L159)
- [src/changes.rs#L269](/Users/vz/projects/reklawdbox/src/changes.rs#L269)

Impact:
- `write_xml` drains staged changes up front and restores them on failure.
- The restore path treats `None` as "missing" rather than "explicitly cleared", so if a user clears a field during the export attempt, a later restore can silently put the old value back.

Recommendation:
- Track explicit clears separately from absent fields, or make the restore path compare against a write generation rather than blindly filling `None` slots.

### 11. Medium: local broker session tokens are stored in plaintext in the server-side cache DB

Evidence:
- [src/store.rs#L74](/Users/vz/projects/reklawdbox/src/store.rs#L74)
- [src/store.rs#L76](/Users/vz/projects/reklawdbox/src/store.rs#L76)
- [src/tools/discogs_auth.rs#L120](/Users/vz/projects/reklawdbox/src/tools/discogs_auth.rs#L120)
- [src/tools/discogs_auth.rs#L123](/Users/vz/projects/reklawdbox/src/tools/discogs_auth.rs#L123)
- [src/tools/discogs_auth.rs#L211](/Users/vz/projects/reklawdbox/src/tools/discogs_auth.rs#L211)
- [src/tools/discogs_auth.rs#L223](/Users/vz/projects/reklawdbox/src/tools/discogs_auth.rs#L223)

Impact:
- Anyone who can read `internal.sqlite3` can replay the broker session token for the local user until expiry.
- This is a smaller blast radius than the broker-side plaintext secret issue, but it is still a credential-at-rest problem.

Recommendation:
- Encrypt or OS-keychain-protect persisted broker session tokens, or accept the risk explicitly and document it as a local-secret requirement.

### 12. Medium: Beatport throttling is per-task sleep, not a shared rate limiter

Evidence:
- [src/beatport.rs#L40](/Users/vz/projects/reklawdbox/src/beatport.rs#L40)
- [src/beatport.rs#L46](/Users/vz/projects/reklawdbox/src/beatport.rs#L46)
- [src/tools/enrich_handlers.rs#L430](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L430)
- [src/tools/enrich_handlers.rs#L618](/Users/vz/projects/reklawdbox/src/tools/enrich_handlers.rs#L618)

Impact:
- Concurrent batch tasks wake up together and still hit Beatport in bursts.
- Under load this can still trigger 429s even though each task "slept".

Recommendation:
- Replace per-call sleep with a process-wide token bucket or serialized scheduler for Beatport requests.

### 13. Medium: the format workflow executes remote code via `curl | sh`

Evidence:
- [.github/workflows/format.yml#L24](/Users/vz/projects/reklawdbox/.github/workflows/format.yml#L24)
- [.github/workflows/format.yml#L25](/Users/vz/projects/reklawdbox/.github/workflows/format.yml#L25)

Impact:
- A compromised upstream install script gets arbitrary code execution on every PR and push that runs formatting checks.
- This is a avoidable CI supply-chain risk.

Recommendation:
- Pin and download a specific release artifact, or use a package manager / action with immutable provenance instead of piping a live script to `sh`.

### 14. Low: broker CI does not type-check or build the Worker

Evidence:
- [.github/workflows/broker-ci.yml#L17](/Users/vz/projects/reklawdbox/.github/workflows/broker-ci.yml#L17)
- [.github/workflows/broker-ci.yml#L37](/Users/vz/projects/reklawdbox/.github/workflows/broker-ci.yml#L37)
- [broker/package.json#L8](/Users/vz/projects/reklawdbox/broker/package.json#L8)

Impact:
- The broker CI job can stay green while TypeScript type errors, Wrangler bundle regressions, or deployment-time issues slip through.

Recommendation:
- Add at least `tsc --noEmit` and a Wrangler build or deploy dry-run step.

### 15. Low: several workflows still use floating major action tags instead of immutable SHAs

Evidence:
- [.github/workflows/broker-ci.yml#L25](/Users/vz/projects/reklawdbox/.github/workflows/broker-ci.yml#L25)
- [.github/workflows/broker-ci.yml#L28](/Users/vz/projects/reklawdbox/.github/workflows/broker-ci.yml#L28)
- [.github/workflows/ci.yml#L21](/Users/vz/projects/reklawdbox/.github/workflows/ci.yml#L21)
- [.github/workflows/ci.yml#L117](/Users/vz/projects/reklawdbox/.github/workflows/ci.yml#L117)
- [.github/workflows/docs-pages.yml#L28](/Users/vz/projects/reklawdbox/.github/workflows/docs-pages.yml#L28)
- [.github/workflows/docs-pages.yml#L65](/Users/vz/projects/reklawdbox/.github/workflows/docs-pages.yml#L65)

Impact:
- Workflow execution is less reproducible and more exposed to an upstream action release being compromised or changed unexpectedly.

Recommendation:
- Pin third-party actions to commit SHAs.

### 16. Low: CLI analyze and hydrate paths can silently poison or drop cache writes

Evidence:
- [src/cli/analyze.rs#L351](/Users/vz/projects/reklawdbox/src/cli/analyze.rs#L351)
- [src/cli/analyze.rs#L352](/Users/vz/projects/reklawdbox/src/cli/analyze.rs#L352)
- [src/cli/analyze.rs#L411](/Users/vz/projects/reklawdbox/src/cli/analyze.rs#L411)
- [src/cli/analyze.rs#L412](/Users/vz/projects/reklawdbox/src/cli/analyze.rs#L412)
- [src/cli/hydrate.rs#L558](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L558)
- [src/cli/hydrate.rs#L571](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L571)
- [src/cli/hydrate.rs#L637](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L637)
- [src/cli/hydrate.rs#L650](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L650)
- [src/cli/hydrate.rs#L948](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L948)
- [src/cli/hydrate.rs#L949](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L949)
- [src/cli/hydrate.rs#L975](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L975)
- [src/cli/hydrate.rs#L976](/Users/vz/projects/reklawdbox/src/cli/hydrate.rs#L976)

Impact:
- Several CLI cache-write paths use `serde_json::to_string(...).unwrap_or_default()`, which converts serialization failures into an empty string that later reads as corrupt cache data.
- The same paths ignore `cache_tx.send(...)` failures, so expensive analysis/enrichment results can be silently discarded if the writer task or channel fails.

Recommendation:
- Propagate serialization failures instead of substituting `""`.
- Log or surface channel-send failures so operators know when cache persistence was lost.

### 17. Low: one genre regression test is currently dead because it is missing `#[test]`

Evidence:
- [src/genre.rs#L585](/Users/vz/projects/reklawdbox/src/genre.rs#L585)
- [src/genre.rs#L586](/Users/vz/projects/reklawdbox/src/genre.rs#L586)
- [src/genre.rs#L604](/Users/vz/projects/reklawdbox/src/genre.rs#L604)

Impact:
- `label_genre_unknown_returns_none()` never runs, while the preceding test has a duplicated `#[test]` attribute that only produces a warning.
- That leaves one intended taxonomy regression check unenforced even though the suite appears fully green.

Recommendation:
- Move one of the duplicated `#[test]` attributes onto `label_genre_unknown_returns_none()`.

## Notes

- Both the Rust and broker test suites were green during the audit, so the issues above are mostly logic, safety-boundary, and operational-hardening defects rather than current red-test failures.
- The cache freshness logic is already implemented once in [src/tools/analysis.rs#L7](/Users/vz/projects/reklawdbox/src/tools/analysis.rs#L7); the main problem is that other consumers bypass it.

## Remediation Status (2026-03-10)

Fixed in the 2026-03-10 remediation pass:
- 1. Discogs broker client token fallback removed from the Rust client. The hosted/default broker now requires `REKLAWDBOX_DISCOGS_BROKER_TOKEN`; there is no compiled fallback token.
- 2. Broker-side Discogs OAuth request/access token secrets are now encrypted before D1 storage, with legacy plaintext read support and opportunistic re-encryption.
- 3. XML export no longer emits Rekordbox attributes for fields the server never loaded or preserved (`Composer`, `Grouping`, `Size`, `DiscNumber`, `TrackNumber`, `Mix`).
- 4. Rust enrichment cache keys now include normalized album for Discogs entries, with store migration support.
- 5. Transient enrichment failures are no longer persisted as durable cache misses; legacy `match_quality='error'` rows are filtered/cleared during migration.
- 6. `write_xml` now fails closed when a configured pre-op backup script fails, and restores staged changes before returning the error.
- 7. Resolve/scoring audio-cache consumers now use freshness validation (`file_size`, `file_mtime`, `analysis_version`) instead of consuming stale rows blindly.
- 8. MCP batch audio analysis concurrency was reduced to a safer bound and cache-writer queue failures are now surfaced explicitly.
- 9. Broker responses no longer reflect raw exception strings to clients; public errors are sanitized and raw details stay in logs.
- 13. The format workflow no longer uses `curl | sh`; it now uses pinned `npm exec dprint`.
- 14. Broker CI now runs `npm run typecheck`, `npm run build`, and `npm test`.
- 16. CLI analyze/hydrate cache-write paths now surface serialization/send failures instead of silently writing corrupt payloads or dropping results.
- 10. `ChangeManager` now tracks fields touched (staged or cleared) since the last `take()` and `restore()` skips those fields, preventing resurrection of explicitly-cleared values.
- 15. All workflows now pin GitHub Actions to commit SHAs (checkout bumped to v4.3.1 for consistency).
- 17. The dead genre regression test now runs again.

Still open after this pass:
- 11. Local broker session tokens in `internal.sqlite3` are still stored in plaintext.
- 12. Beatport throttling is still not globally coordinated across concurrent tasks.

Validation on 2026-03-10:
- `cargo test` -> passed (`395 passed, 0 failed, 28 ignored`)
- `cd broker && npm run typecheck` -> passed
- `cd broker && npm test` -> passed (`13 passed`)
- `cd broker && npm run build` -> passed (`wrangler deploy --dry-run`)
- `cargo build --release` -> passed

## Remediation Status (2026-03-11)

- 11. Broker session tokens are now stored in macOS Keychain (`com.reklawdbox.broker-session` service) instead of plaintext in SQLite. The `session_token` column stores an empty sentinel; legacy plaintext tokens are transparently migrated to Keychain on first read.
- 12. Beatport throttling now uses a process-wide rate limiter (`OnceLock<TokioMutex<Option<Instant>>>`) that serializes all callers and enforces minimum inter-request spacing.

All 17 findings are now resolved.

Validation on 2026-03-11:
- `cargo test` -> passed (`404 passed, 0 failed, 28 ignored`)
- `cargo build --release` -> passed
