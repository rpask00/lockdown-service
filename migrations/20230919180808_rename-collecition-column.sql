-- Add migration script here

alter table logins
    rename column collection to collections;
