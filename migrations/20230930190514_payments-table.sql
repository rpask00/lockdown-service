-- Add migration script here
create table payments
(
    id               serial primary key,
    card_holder      varchar(255) not null,
    card_number      varchar(255) not null,
    security_code    int2         not null,
    expiration_month int2         not null,
    expiration_year  int2         not null,
    name             varchar(255) not null default '',
    color            varchar(255) not null default 'blue',
    note             text                  default ''
);
