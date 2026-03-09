# Orchestrator Handover Prompt

You are the lead orchestrator agent for `reklawdbox`. Your job is to take the existing audit findings and drive fixes from start to finish, using subagents aggressively, reviewing work at milestones, and leaving the repo in a verified, releasable state.

## Repo Context

- Project: `reklawdbox`, an MCP server for Rekordbox 7.x.
- Main runtime: Rust 2024 single binary.
- Companion service: Discogs broker in TypeScript on Cloudflare Workers + D1.
- Critical rule: Rekordbox DB is read-only. Human-approved write path is XML export only.
- Use Conventional Commits.
- If you modify server code, you must run `cargo build --release` at the end and tell the user to run `/mcp` to reconnect, because the running server will still be the old binary.

## Source Materials

Start from these audit reports:

- Consolidated audit: `/Users/vz/projects/reklawdbox/docs/tmp/2026-03-09-project-audit.md`

Treat the consolidated audit as the source of truth unless you re-verify and overturn a finding with code evidence.

## Primary Objective

Work through the real findings end-to-end. Do not stop at planning. Implement fixes, add or update tests, run verification, and leave a concise status summary for the user with any remaining risks.

## Operating Mode

You are an orchestrator, not just an implementer.

- Split work across subagents in parallel where write scopes do not overlap.
- Keep one central execution plan updated.
- Review subagent output before accepting it.
- Require milestone reviews before moving from one tranche of fixes to the next.
- Prefer landing a smaller set of high-confidence fixes completely rather than partially touching every finding.

## Priority Order

Work in this order unless code review during implementation reveals a dependency inversion:

1. Discogs broker client token fallback in Rust client
2. Broker-side plaintext OAuth secret storage
3. XML export safety issues
4. Enrichment cache correctness issues
5. Audio cache freshness and batch analysis robustness
6. Broker error-handling and credential-at-rest improvements
7. CI hardening and broker CI coverage
8. Lower-severity CLI/test hygiene items

## Findings To Address

At minimum, review and fix as many of these as feasible in one pass:

1. Hardcoded Discogs broker token fallback in `src/discogs.rs`
2. Plaintext Discogs OAuth secrets in `broker` D1 storage
3. XML export emitting hard-coded blank/zero fields for values it never loaded
4. `write_xml` continuing after backup failure
5. Discogs enrichment cache key missing `album`
6. Transient enrichment failures cached as durable misses
7. Stale audio-analysis cache reads in scoring/resolve paths
8. Batch audio analysis memory/concurrency risk
9. Broker reflecting raw exception messages to clients
10. Plaintext local broker session tokens in internal SQLite
11. Beatport throttling not being globally coordinated
12. `curl | sh` in format workflow
13. Broker CI missing type-check/build coverage
14. Floating action tags instead of SHAs
15. CLI cache-write paths silently poisoning or dropping cache writes
16. Dead test in `src/genre.rs`

You do not need to fix every item if time or complexity makes that impractical, but you must:

- explicitly choose a cut line
- explain why
- finish everything above that line properly

## Milestones

### Milestone 1: Re-verify and scope

- Re-check each targeted finding against current code before editing.
- Downgrade or drop anything you can disprove with concrete evidence.
- Produce a grouped execution plan:
  - auth/secrets
  - XML/export safety
  - cache correctness
  - broker/CI hardening

Review gate:
- Stop and self-review whether each chosen finding is still real and whether the fix strategy is minimally invasive and correct.

### Milestone 2: High-severity auth and XML fixes

Target:
- hardcoded broker token fallback
- broker secret storage issue, if feasible in one safe migration
- XML export safety defects
- backup failure behavior

Requirements:
- preserve compatibility intentionally or document breakage precisely
- add tests for each fixed behavior

Review gate:
- Perform a focused review of changed code for credential leakage, migration safety, and rollback behavior.

### Milestone 3: Cache correctness and audio freshness

Target:
- album-aware enrichment cache
- no caching of transient errors as durable misses
- freshness validation for scoring/resolve cache consumers
- CLI cache write robustness

Requirements:
- add migration logic if schema changes
- add regression tests around cache hits, misses, and stale rows

Review gate:
- Review for cache invalidation correctness, backward compatibility, and performance regressions.

### Milestone 4: Broker/CI hardening and cleanup

Target:
- broker error-message sanitization
- broker/local credential-at-rest improvements if feasible
- Beatport throttle coordination
- CI hardening
- dead test cleanup

Review gate:
- Review for operational risk, deployability, and whether each change is worth its complexity.

## Subagent Strategy

Use subagents aggressively, but keep ownership disjoint.

Suggested split:

- Worker A: Rust auth/enrichment/cache fixes
  - `src/discogs.rs`
  - `src/tools/enrich_handlers.rs`
  - `src/store.rs`
  - related tests

- Worker B: Rust XML/export/staging fixes
  - `src/xml.rs`
  - `src/changes.rs`
  - `src/tools/staging_handlers.rs`
  - related tests

- Worker C: Rust audio/scoring/resolve fixes
  - `src/tools/analysis.rs`
  - `src/tools/audio_handlers.rs`
  - `src/tools/resolve_handlers.rs`
  - `src/tools/scoring.rs`
  - CLI analysis/hydrate paths

- Worker D: Broker security/hardening
  - `broker/src/index.ts`
  - `broker/migrations/*.sql`
  - broker tests

- Worker E: CI/workflow hardening and low-risk hygiene
  - `.github/workflows/*`
  - `src/genre.rs`
  - possibly docs updates

Rules for subagents:

- Tell each worker they are not alone in the repo.
- Tell them not to revert unrelated edits.
- Require them to report exact files changed and any unresolved risks.

## Implementation Standards

- Prefer minimal, correct fixes over broad refactors.
- Use `apply_patch` for manual file edits.
- Add targeted tests for each bug fixed.
- Keep error handling explicit; do not silently swallow failures.
- If a fix changes behavior for operators or configuration, update docs.
- If a migration is risky, stage it carefully and document downgrade/compatibility implications.

## Review Standards

At each milestone, do a code-review pass yourself before continuing.

For each changed area, check:

- Is the bug actually fixed?
- Did the change create a regression path?
- Are tests covering the fixed behavior?
- Is the failure mode now explicit and diagnosable?
- Is the operational behavior acceptable?

If a subagent’s patch is weak, incomplete, or too invasive:

- revise it yourself or send the work back with a narrower instruction

## Verification Checklist

Minimum verification before final handoff:

- `cargo test`
- `cd broker && npm test`
- `cargo build --release`

Also run targeted checks as needed, for example:

- focused Rust test subsets for changed modules
- broker type-check/build if you add those scripts
- any migration or schema verification you add

If you change the broker significantly, verify its tests still reflect the intended auth posture.

## Final Deliverable

When done, provide:

1. A concise summary of what was fixed
2. Which audit findings remain open and why
3. Validation results
4. Any migration/config/deploy notes
5. A note to the user to run `/mcp` to reconnect if server code changed

Also update `/Users/vz/projects/reklawdbox/docs/tmp/2026-03-09-project-audit.md` to mark fixed items or append a remediation status section, rather than creating yet another competing audit file.

## Start Here

1. Read `/Users/vz/projects/reklawdbox/docs/tmp/2026-03-09-project-audit.md`
2. Re-verify the top 6 findings against live code
3. Open a plan with milestones and owners
4. Spawn workers for disjoint areas
5. Review at Milestone 1 before allowing broad edits
