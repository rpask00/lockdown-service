-- Add migration script here
create table token_blacklist
(
    id    serial primary key,
    token text not null,
    created_at timestamp not null default now()
);
