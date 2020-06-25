-------------
-- Records --
-------------

-- records
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

--------------------
-- Authentication --
--------------------

-- refresh_token
CREATE TABLE IF NOT EXISTS refresh_tokens
(
    id         int auto_increment                 primary key,
    token      varchar(255)                       not null,
    expires_at datetime                           null,
    created_at datetime default CURRENT_TIMESTAMP null,
    updated_at datetime default CURRENT_TIMESTAMP null,
    constraint refresh_tokens_token_uindex
        unique (token)
);

-- access_token
CREATE TABLE IF NOT EXISTS access_tokens
(
    id               int auto_increment                 primary key,
    token            varchar(255)                       not null,
    expires_at       datetime                           null,
    created_at       datetime default CURRENT_TIMESTAMP null,
    updated_at       datetime default CURRENT_TIMESTAMP null,
    refresh_token_id int                                not null,
    constraint access_tokens_token_uindex
        unique (token),
    constraint access_tokens_refresh_tokens_id_fk
        foreign key (refresh_token_id) references refresh_tokens (id)
            on update cascade on delete cascade
);

CREATE INDEX access_tokens_refresh_token_id_index
    on access_tokens (refresh_token_id);
