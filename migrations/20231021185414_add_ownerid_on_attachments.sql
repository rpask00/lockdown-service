-- Add migration script here
alter table note_attachments
    add column owner_id integer;
