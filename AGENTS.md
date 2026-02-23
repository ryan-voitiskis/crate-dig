# AGENTS.md

Repository-wide instructions for coding agents.

## MCP Runtime

- This project is an MCP server over stdio (`./target/release/reklawdbox`), not a flag-driven CLI app.
- Host-specific guidance lives in:
  - `CODEX.md` for Codex
  - `CLAUDE.md` for Claude Code
- Keep MCP credentials/secrets in local environment or untracked local config (`.mcp.json` is gitignored).

## Commit Messages

Use Conventional Commits for every commit:

```text
type(scope): short summary
```

Allowed `type` values include: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`, `build`.

Examples:

- `docs(rekordbox): add corpus manifest and cross-links`
- `fix(xml): handle empty playlist entries`
- `test(db): cover sqlcipher connection errors`
