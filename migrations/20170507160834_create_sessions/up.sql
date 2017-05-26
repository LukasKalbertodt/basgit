create table sessions (
    id bytea
        primary key
        check (octet_length(id) = 16),

    user_id bigint
        not null
        references users(id)
            on delete cascade
            on update cascade,

    birth timestamptz
        not null
        default now()
);
