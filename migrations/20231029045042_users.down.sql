-- Add down migration script here
ALTER TABLE tasks
DROP CONSTRAINT IF EXISTS fk_tasks_user;

ALTER TABLE tasks
DROP COLUMN IF EXISTS user_id;

DROP TRIGGER IF EXISTS tgr_users_updated_at ON users;

DROP TABLE IF EXISTS users;