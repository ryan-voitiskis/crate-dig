# 2026-03-10 Remediation Summary

## Scope Completed

This pass implemented and verified fixes for the highest-priority audit findings across the Rust MCP server, the Discogs broker Worker, CLI cache-write paths, and CI/workflow hardening.

Fixed:
- Discogs broker client token fallback removed from the Rust client.
- Broker-side Discogs OAuth request/access token secrets encrypted at rest.
- Broker public error handling sanitized.
- XML exporter no longer emits Rekordbox fields the server never loaded.
- `write_xml` now fails closed on configured backup failure and restores staged changes.
- Discogs enrichment cache keys now include normalized album.
- Transient enrichment failures are no longer persisted as durable cache misses.
- Resolve/scoring paths now enforce audio-cache freshness.
- MCP batch audio analysis concurrency reduced and cache writer failures surfaced.
- CLI analyze/hydrate cache-write paths now surface serialization/send failures.
- Broker CI now type-checks, builds, and tests the Worker.
- Format workflow no longer uses `curl | sh`.
- The dead genre regression test now runs again.

Partially addressed:
- Action SHA pinning was added to the touched workflows (`broker-ci.yml`, `format.yml`), but not yet to every workflow called out in the audit.

## Remaining Open Items

These findings were intentionally left open after the cut line:
- Failed XML export can still resurrect explicitly-cleared fields during restore.
- Local broker session tokens in `internal.sqlite3` are still plaintext.
- Beatport throttling is still per-task rather than globally coordinated.
- Some workflows outside the touched CI files still use floating action refs.

## Validation

Validated successfully on 2026-03-10:
- `cargo test`
- `cd broker && npm run typecheck`
- `cd broker && npm test`
- `cd broker && npm run build`
- `cargo build --release`

## Migration / Operator Notes

- The internal store schema moved to `user_version = 4`.
- `enrichment_cache` now includes `query_album`.
- Legacy cached enrichment rows with `match_quality = 'error'` are dropped/ignored during migration.
- The broker now expects `BROKER_STATE_ENCRYPTION_KEY` for secret-at-rest protection.
- The hosted/default broker now requires `REKLAWDBOX_DISCOGS_BROKER_TOKEN`; there is no compiled fallback token.
- Server code changed and `cargo build --release` was run, so the MCP client must reconnect to pick up the new binary.
