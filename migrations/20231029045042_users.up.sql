-- Add up migration script here
CREATE TABLE IF NOT EXISTS
  users (
    id BIGSERIAL NOT NULL
  , name VARCHAR(255) NOT NULL
  , email VARCHAR(255) NOT NULL
  , hashed_password VARCHAR(255) NOT NULL
  , is_active BOOLEAN NOT NULL DEFAULT FALSE
  , created_at TIMESTAMP NOT NULL DEFAULT now()
  , updated_at TIMESTAMP NOT NULL DEFAULT now()
  , CONSTRAINT pk_users PRIMARY KEY (id)
  , CONSTRAINT uq_users_email UNIQUE (email)
  );

CREATE TRIGGER tgr_users_updated_at BEFORE
UPDATE ON users FOR EACH ROW
EXECUTE PROCEDURE moddatetime (updated_at);

ALTER TABLE tasks
ADD COLUMN user_id BIGINT;

ALTER TABLE tasks
ADD CONSTRAINT fk_tasks_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE SET NULL;