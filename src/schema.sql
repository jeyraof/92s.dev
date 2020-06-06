CREATE TABLE IF NOT EXISTS records
(
    id         int auto_increment                 primary key,
    slug       varchar(255)                       not null,
    url        text                               null,
    created_at datetime default CURRENT_TIMESTAMP null,
    constraint records_slug_uindex
        unique (slug)
);
