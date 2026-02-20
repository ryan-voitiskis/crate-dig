# Discogs Broker Test-First Handover

## Purpose

This runbook is for shipping broker auth changes with a strict order:

1. Build and test first.
2. Deploy only after all checks are green.

## Scope

Covers:

- Rust MCP server checks (`reklawdbox`)
- Broker Worker checks (`broker/`)
- Local smoke validation
- Cloudflare deploy + post-deploy verification

## Preconditions

- Cloudflare account access and `wrangler` auth (`wrangler login` complete).
- `broker/wrangler.toml` has real D1 `database_id`.
- Broker secrets are set:
  - `DISCOGS_CONSUMER_KEY`
  - `DISCOGS_CONSUMER_SECRET`
- Optional broker vars are set when needed:
  - `BROKER_PUBLIC_BASE_URL`
  - `BROKER_CLIENT_TOKEN`

## Phase 1: Rust Build/Test Gate (Must Pass)

From repo root:

```bash
cargo fmt --all
cargo test
```

Go/No-Go:

- Go only if `cargo test` has zero failures.
- No deploy if any Rust test fails.

## Phase 2: Broker Build/Test Gate (Must Pass)

From repo root:

```bash
cd broker
npm install
npm run d1:migrate:local
npm run dev
```

In a second terminal, run local endpoint smoke tests (replace token/header as needed):

```bash
curl -sS -X POST "http://127.0.0.1:8787/v1/device/session/start" \
  -H "content-type: application/json" \
  -H "x-reklawdbox-broker-token: <BROKER_CLIENT_TOKEN_IF_ENABLED>"
```

Expected:

- JSON with `device_id`, `pending_token`, `auth_url`, `poll_interval_seconds`, `expires_at`.

Optional status check:

```bash
curl -sS "http://127.0.0.1:8787/v1/device/session/status?device_id=<device_id>&pending_token=<pending_token>" \
  -H "x-reklawdbox-broker-token: <BROKER_CLIENT_TOKEN_IF_ENABLED>"
```

Go/No-Go:

- Go only if local broker starts and endpoint contract matches.
- No deploy if startup, migration, or endpoint contract fails.

## Phase 3: Integration Gate (Client + Broker)

In MCP host env, set:

- `REKLAWDBOX_DISCOGS_BROKER_URL` to local broker URL (for local smoke) or deployed URL.
- `REKLAWDBOX_DISCOGS_BROKER_TOKEN` if broker enforces client token.

Run `lookup_discogs` from MCP client.

Expected flow:

1. First call returns actionable auth remediation including `auth_url`.
2. Open `auth_url` and complete Discogs approval.
3. Re-run `lookup_discogs`; expect Discogs-normalized payload and caching behavior.

Go/No-Go:

- Go only if auth-remediation and post-auth lookup both work.
- No deploy if session finalization or proxy lookup fails.

## Phase 4: Deploy (Only After All Gates Pass)

From repo root:

```bash
cd broker
npm run d1:migrate:remote
npm run deploy
```

Post-deploy checks:

1. `POST /v1/device/session/start` returns contract payload.
2. OAuth callback marks session authorized.
3. `POST /v1/discogs/proxy/search` works with bearer `session_token`.
4. `lookup_discogs` in MCP works against deployed broker.

## Rollback

If deploy is bad:

1. Re-deploy previous known-good Worker version.
2. If schema-related issue, stop traffic first; do not hot-edit data manually.
3. Clear local MCP broker session row (`broker_discogs_session`) and re-auth.

## Known Notes

- Legacy `REKLAWDBOX_DISCOGS_*` remains a deprecated fallback path; default path is broker-based.
- Local session token persistence is in internal SQLite table: `broker_discogs_session`.
- Reference docs:
  - `docs/discogs-broker-auth.md`
  - `docs/discogs-broker-auth-plan.md`
