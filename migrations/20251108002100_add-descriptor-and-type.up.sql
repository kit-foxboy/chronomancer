-- Add up migration script here
PRAGMA foreign_keys=OFF;

CREATE TABLE timers_new (
    id INTEGER PRIMARY KEY,
    description TEXT NOT NULL DEFAULT '',
    is_recurring BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    paused_at INTEGER NOT NULL DEFAULT 0,
    ends_at INTEGER NOT NULL DEFAULT 0
);

INSERT INTO timers_new (id, description, is_recurring, created_at, paused_at, ends_at)
SELECT
    id,
    '' AS description,
    is_recurring,
    created_at,
    paused_at,
    ends_at
FROM timers;

DROP TABLE timers;
ALTER TABLE timers_new RENAME TO timers;

CREATE INDEX IF NOT EXISTS timers_created_at ON timers (created_at);

PRAGMA foreign_keys=ON;

CREATE TABLE timer_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);