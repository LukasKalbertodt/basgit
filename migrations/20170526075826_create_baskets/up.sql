create table baskets (
    id bigserial
        primary key,

    name sl_string
        not null,

    user_id bigint
        not null
        references users(id)
            on delete cascade
            on update cascade,

    description ml_string,

    private bool
        not null,

    kind sl_string
        not null,

    forked_from bigint
        references baskets(id)
            on delete set null
            on update cascade
);

create unique index baskets_unique_name_per_user_idx on baskets (name, user_id);
