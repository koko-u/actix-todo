-- Add down migration script here
ALTER TABLE tasks
    DROP COLUMN IF EXISTS status_id;

ALTER TABLE tasks
    ADD COLUMN IF NOT EXISTS status VARCHAR(100) DEFAULT 'NotStarted';

DROP TABLE IF EXISTS status;