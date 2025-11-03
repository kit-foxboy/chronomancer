-- Add down migration script here
ALTER TABLE timers
    DROP COLUMN paused_at,
    DROP COLUMN ends_at,
    ADD COLUMN duration_seconds INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN completed_at BIGINT DEFAULT NULL;
    PRAGMA foreign_keys=OFF;

    CREATE TABLE timers_old (
        id INTEGER PRIMARY KEY,
        is_recurring BOOLEAN NOT NULL DEFAULT 0,
        duration_seconds INTEGER NOT NULL DEFAULT 0,
        created_at INTEGER NOT NULL,
        completed_at BIGINT DEFAULT NULL
    );

    INSERT INTO timers_old (id, is_recurring, duration_seconds, created_at, completed_at)
    SELECT
        id,
        is_recurring,
        CASE WHEN ends_at IS NOT NULL AND ends_at > 0 THEN (ends_at - created_at) ELSE 0 END AS duration_seconds,
        created_at,
        NULL AS completed_at
    FROM timers;

    DROP TABLE timers;
    ALTER TABLE timers_old RENAME TO timers;

    CREATE INDEX IF NOT EXISTS timers_ends_at_idx ON timers (completed_at);

ALTER INDEX timers_ends_at_idx RENAME TO idx_timers_completes;
PRAGMA foreign_keys=ON;