-- Add down migration script here
DROP TABLE IF EXISTS timers;

DROP INDEX IF EXISTS idx_timers_completed;
DROP INDEX IF EXISTS idx_timers_recurring;