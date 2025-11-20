-- Revert to schema without timer_type_id and description columns
PRAGMA foreign_keys=OFF;

-- Drop the timer_types table
DROP TABLE IF EXISTS timer_types;

-- Recreate timers table without the new columns
CREATE TABLE timers_new (
    id INTEGER PRIMARY KEY,
    is_recurring BOOLEAN NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL,
    paused_at INTEGER NOT NULL DEFAULT 0,
    ends_at INTEGER NOT NULL DEFAULT 0
);

-- Copy data back, dropping the timer_type_id and description columns
INSERT INTO timers_new (id, is_recurring, created_at, paused_at, ends_at)
SELECT
    id,
    is_recurring,
    created_at,
    paused_at,
    ends_at
FROM timers;

DROP TABLE timers;
ALTER TABLE timers_new RENAME TO timers;

-- Recreate indexes
CREATE INDEX IF NOT EXISTS timers_ends_at_idx ON timers (ends_at);
CREATE INDEX IF NOT EXISTS timers_paused_at_idx ON timers (paused_at);

PRAGMA foreign_keys=ON;
