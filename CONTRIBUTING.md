# Contributing

Thanks for contributing to `reklawdbox`.

## Project Model

`reklawdbox` is primarily operated as an MCP server over stdio (`./target/release/reklawdbox`).
The repository also includes supporting scripts and CLI-style workflows for setup, backup, and validation.

## Local Setup

1. Build:

```bash
cargo build --release
```

2. Run tests:

```bash
cargo test
cargo test -- --ignored
```

3. Optional corpus validation after docs/corpus edits:

```bash
bash docs/rekordbox/validate-corpus.sh
python3 docs/rekordbox/verify-phase-b.py
```

## Coding Expectations

- Keep changes scoped and reviewable.
- Add tests for behavior changes when practical.
- Avoid committing secrets or local host credentials (`.mcp.json` is local-only).
- Keep generated or machine-local artifacts out of commits.

## Commit Messages

Use Conventional Commits:

```text
type(scope): short summary
```

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`, `build`.

## Pull Request Checklist

- [ ] Change is scoped to a clear problem.
- [ ] Tests updated or rationale provided if tests were not added.
- [ ] Documentation updated for user-visible behavior changes.
- [ ] Commit messages follow Conventional Commits.
- [ ] No secrets or local-only config files included.

## Security Issues

Do not open public issues for vulnerabilities. Follow `SECURITY.md` for private disclosure.
