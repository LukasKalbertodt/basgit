-- A simple type alias to limit the size of strings which are intended for
-- single line usage (think username, ...). I think a better description
-- would be: "a couple of words".
--
-- We don't want to semantically limit those field, we just do it for
-- security reasons (let the database server check for huge strings, too).
create domain sl_string as text
check (octet_length(value) <= 126);

-- As above, but for multi line strings which should be somewhat limited. I
-- prefer the description: "a couple of sentences".
create domain ml_string as text
check (octet_length(value) <= 32768);
