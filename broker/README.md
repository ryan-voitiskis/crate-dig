# Discogs Broker (Cloudflare Worker)

Maintainer-hosted broker for Discogs OAuth and proxied search.

## Endpoints

- `POST /v1/device/session/start`
- `GET /v1/device/session/status?device_id=...&pending_token=...`
- `POST /v1/device/session/finalize`
- `GET /v1/discogs/oauth/link`
- `GET /v1/discogs/oauth/callback`
- `POST /v1/discogs/proxy/search`

## Required secrets

Set with `wrangler secret put`:

- `DISCOGS_CONSUMER_KEY`
- `DISCOGS_CONSUMER_SECRET`

## Optional vars

Set in `wrangler.toml` `[vars]`:

- `BROKER_PUBLIC_BASE_URL`
- `BROKER_CLIENT_TOKEN`
- `DEVICE_SESSION_TTL_SECONDS` (default `900`)
- `SESSION_TOKEN_TTL_SECONDS` (default `2592000`)
- `SEARCH_CACHE_TTL_SECONDS` (default `604800`)
- `DISCOGS_MIN_INTERVAL_MS` (default `1100`)

## Local dev

```bash
cd broker
npm install
npm run d1:migrate:local
npm run dev
```

Or run:

```bash
cd broker
./scripts/dev.sh
```
