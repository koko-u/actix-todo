-- Add up migration script here
CREATE TABLE IF NOT EXISTS tasks
(
    id          BIGSERIAL    NOT NULL,
    summary     VARCHAR(255) NOT NULL,
    description TEXT,
    status      VARCHAR(100) NOT NULL DEFAULT 'NotStarted',
    created_at  TIMESTAMP    NOT NULL DEFAULT 'now()',
    updated_at  TIMESTAMP    NOT NULL DEFAULT 'now()',
    CONSTRAINT pk_tasks PRIMARY KEY (id)
);

CREATE TRIGGER tgr_tasks_updated_at
    BEFORE UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime(updated_at);