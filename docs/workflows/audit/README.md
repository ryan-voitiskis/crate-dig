# Audit Workflow

Single-prompt workflow that audits a codebase subsystem end-to-end.
The agent orchestrates subagents through discovery, audit, triage,
implementation, and review — you just specify what to audit.

## Usage

Open a Claude Code instance and `@`-include `audit.md` followed by
a description of the subsystem to audit:

```
@docs/workflows/audit/audit.md audit the database access layer —
SQLCipher decryption, query construction, search filters, connection
management, and the internal SQLite store
```

The agent handles everything from there. It will pause after triage
to show you what it plans to fix and what it plans to defer, then
wait for your confirmation before implementing.

## Parallel audits

Run independent subsystems in separate Claude Code instances:

```
Terminal 1:  claude  →  @prompt.md  [DB access layer]
Terminal 2:  claude  →  @prompt.md  [enrichment pipeline]
Terminal 3:  claude  →  @prompt.md  [XML export / write path]
```

If two audits touch overlapping files and both reach the implement
phase, run them sequentially or in separate worktrees.

## What to include in the subsystem description

More context produces better audits. Good descriptions include:

- The subsystem name and what it does
- Key modules or file names to start from (the agent will find more)
- Specific concerns you want examined (security surface, concurrency,
  correctness of a specific algorithm)
- What severity ordering matters most (e.g., "data corruption risks
  matter more than code quality here")

## Dependencies

The review phase (Phase 5) uses these Claude Code agent plugins:

- `pr-review-toolkit:code-reviewer`
- `pr-review-toolkit:silent-failure-hunter`
- `pr-review-toolkit:pr-test-analyzer`

These must be installed and available in your Claude Code instance.
The workflow will still function without them, but the review phase
will fall back to generic subagent review rather than the specialized
reviewers.
