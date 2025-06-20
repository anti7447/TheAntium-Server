CREATE TABLE IF NOT EXISTS users (
    id              INTEGER         PRIMARY KEY AUTOINCREMENT NOT NULL,
    username        TEXT            UNIQUE,
    display_name    TEXT,
    password_hash   TEXT,
    token           TEXT            UNIQUE
);

CREATE TABLE IF NOT EXISTS posts (
    id              INTEGER         PRIMARY KEY AUTOINCREMENT NOT NULL,
    author_id       INTEGER,
    content         TEXT
);
