-- Add migration script here
-- Add migration script here
create table logins
(
    id              serial primary key,
    used_at         timestamp    not null default now(),
    username        varchar(255) not null,
    password        varchar(255) not null,
    email           varchar(255) not null,
    linked_websites varchar(255) not null,
    collection      varchar(255) not null
);
