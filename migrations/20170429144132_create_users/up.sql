CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL,
    name VARCHAR(50),
    CONSTRAINT users_username_unique UNIQUE (username)
)
