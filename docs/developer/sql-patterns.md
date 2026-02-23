# SQL Patterns

All queries use dynamic building with indexed positional parameters (`?1`, `?2`, ...)
and `Vec<Box<dyn ToSql>>` for bind values.

The base `TRACK_SELECT` joins 7 tables via `LEFT JOIN`.

Every query filters deleted tracks and sampler samples.
