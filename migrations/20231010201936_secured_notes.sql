CREATE TABLE IF NOT EXISTS secured_notes
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    content     VARCHAR(255) NOT NULL,
    created_at  timestamp    not null default now(),
    modified_at timestamp    not null default now(),
    color       varchar(255) NOT NULL default 'red'
);
