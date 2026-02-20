CREATE TABLE IF NOT EXISTS device_sessions (
    device_id TEXT PRIMARY KEY,
    pending_token TEXT NOT NULL,
    status TEXT NOT NULL,
    poll_interval_seconds INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    authorized_at INTEGER,
    oauth_access_token TEXT,
    oauth_access_token_secret TEXT,
    oauth_identity TEXT,
    session_token_hash TEXT,
    session_expires_at INTEGER,
    finalized_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_device_sessions_pending
    ON device_sessions (device_id, pending_token);

CREATE INDEX IF NOT EXISTS idx_device_sessions_session_hash
    ON device_sessions (session_token_hash);

CREATE TABLE IF NOT EXISTS oauth_request_tokens (
    oauth_token TEXT PRIMARY KEY,
    oauth_token_secret TEXT NOT NULL,
    device_id TEXT NOT NULL,
    pending_token TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_oauth_request_tokens_device
    ON oauth_request_tokens (device_id, pending_token);

CREATE TABLE IF NOT EXISTS discogs_search_cache (
    cache_key TEXT PRIMARY KEY,
    response_json TEXT NOT NULL,
    cached_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_discogs_search_cache_expiry
    ON discogs_search_cache (expires_at);

CREATE TABLE IF NOT EXISTS rate_limit_state (
    bucket TEXT PRIMARY KEY,
    last_request_at_ms INTEGER NOT NULL
);
