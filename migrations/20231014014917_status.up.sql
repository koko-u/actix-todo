-- Add up migration script here
CREATE TABLE IF NOT EXISTS status
(
    id         BIGSERIAL    NOT NULL,
    name       VARCHAR(255) NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT 'now()',
    updated_at TIMESTAMP    NOT NULL DEFAULT 'now()',
    CONSTRAINT pk_status PRIMARY KEY (id),
    CONSTRAINT uq_status_name UNIQUE (name)
);

CREATE TRIGGER tgr_status_updated_at
    BEFORE UPDATE
    ON status
    FOR EACH ROW
EXECUTE PROCEDURE moddatetime(updated_at);

ALTER TABLE tasks
    DROP COLUMN IF EXISTS status;

ALTER TABLE tasks
    ADD COLUMN status_id BIGINT NOT NULL;

ALTER TABLE tasks
    ADD CONSTRAINT fk_tasks_status FOREIGN KEY (status_id) REFERENCES status (id) ON DELETE RESTRICT;