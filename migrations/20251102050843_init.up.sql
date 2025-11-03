-- Add up migration script here
CREATE TABLE timers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    duration_seconds INTEGER NOT NULL,
    is_recurring TINYINT DEFAULT 0,
    created_at BIGINT DEFAULT CURRENT_TIMESTAMP,
    completed_at BIGINT DEFAULT NULL
);

CREATE INDEX idx_timers_completes ON timers (completed_at);
CREATE INDEX idx_timers_recurring ON timers (is_recurring);