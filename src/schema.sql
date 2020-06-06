CREATE TABLE IF NOT EXISTS records
(
    id           int auto_increment                 primary key,
    slug         varchar(255)                       not null,
    url          text                               not null,
    created_at   datetime default CURRENT_TIMESTAMP null,
    last_used_at datetime                           null,
    constraint records_slug_uindex
        unique (slug)
);

CREATR INDEX records_last_used_at_id_index
    on records (last_used_at desc, id desc);