# Rekordbox Gotchas

- DB column is `Commnt` not `Comment` (typo in schema)
- XML rating values: `0/51/102/153/204/255` (not `0-5`) — use `stars_to_rating()` / `rating_to_stars()` in `src/types.rs`
- `COLLECTION Entries` count in XML must match actual track count or import fails
- `DJPlayCount` is integer in real DB but sometimes text — `row_to_track()` handles both
- SQLCipher key: `PRAGMA key = '<passphrase>'` (not `x'<hex>'`)
- BPM stored as integer x100 (`12800` = `128.00 BPM`)
- All queries must filter `rb_local_deleted = 0` and exclude sampler path prefix
- LIKE wildcards (`%`, `_`) in user input must be escaped — use `escape_like()`
