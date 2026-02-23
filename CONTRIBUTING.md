# Contributing

`reklawdbox` is primarily an MCP server over stdio (`./target/release/reklawdbox`).

## Local Setup

```bash
cargo build --release
cargo test
cargo test -- --ignored
# optional after docs/corpus edits
bash docs/rekordbox/validate-corpus.sh
python3 docs/rekordbox/verify-phase-b.py
```

## Expectations

- Keep changes scoped and reviewable.
- Add/update tests for behavior changes, or explain why not.
- Update docs for user-visible behavior changes.
- Use Conventional Commits: `type(scope): short summary`.
- Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`, `build`.
- Never commit secrets or local-only config (for example `.mcp.json`).

## Security Issues

Do not open public vulnerability issues. Follow `SECURITY.md` for private disclosure.
