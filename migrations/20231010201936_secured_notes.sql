CREATE TABLE IF NOT EXISTS secured_notes
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    content     VARCHAR(255) NOT NULL,
    created_at  TIMESTAMPTZ           DEFAULT NOW(),
    modified_at TIMESTAMPTZ           DEFAULT NOW(),
    color       varchar(255) not null default 'red'
);
