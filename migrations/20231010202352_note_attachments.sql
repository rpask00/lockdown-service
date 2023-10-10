-- Add migration script here
create table note_attachments
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    size       INTEGER      NOT NULL,
    type       VARCHAR(255) NOT NULL,
    note_id    INT          NOT NULL,
    FOREIGN KEY (note_id) REFERENCES secured_notes (id) ON DELETE CASCADE
);
