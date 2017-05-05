create table users (
    id bigserial primary key,

    username sl_string not null,
    name sl_string,

    bio ml_string
);

create unique index users_unique_lower_username_idx on users (lower(username));


create table user_emails (
    email text primary key check (octet_length(email) <= 254),
    user_id bigint references users(id) on delete cascade on update cascade

    -- Valid emails can't be longer than 254 bytes. We can at least restrict
    -- the number of characters here.
    --
    -- [1]: http://stackoverflow.com/a/574698/2408867
);

create index user_emails_user_id_idx on user_emails (user_id);
create unique index user_emails_email_idx on user_emails (email);
