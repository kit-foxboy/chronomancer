-- Add up migration script here
PRAGMA foreign_keys=OFF;

CREATE TABLE timers_new (
    id INTEGER PRIMARY KEY,
    is_recurring BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    paused_at INTEGER NOT NULL DEFAULT 0,
    ends_at INTEGER NOT NULL DEFAULT 0
);

INSERT INTO timers_new (id, is_recurring, created_at, paused_at, ends_at)
SELECT
    id,
    is_recurring,
    created_at,
    0 AS paused_at,
    COALESCE(created_at + duration_seconds, 0) AS ends_at
FROM timers;

DROP TABLE timers;
ALTER TABLE timers_new RENAME TO timers;

DROP INDEX IF EXISTS idx_timers_completes;
CREATE INDEX IF NOT EXISTS timers_ends_at_idx ON timers (ends_at);
CREATE INDEX IF NOT EXISTS timers_paused_at_idx ON timers (paused_at);

PRAGMA foreign_keys=ON;