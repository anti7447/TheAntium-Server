CREATE TABLE IF NOT EXISTS users (
    id              INTEGER         PRIMARY KEY AUTOINCREMENT NOT NULL,
    tag             TEXT,
    username        TEXT            UNIQUE
    avatar_url      TEXT
    banner_url      TEXT
    password_hash   TEXT
    token           TEXT UNIQUE
    telegram_id     INTEGER
);

CREATE TABLE IF NOT EXISTS posts (
    id              INTEGER         PRIMARY KEY AUTOINCREMENT NOT NULL,
    author_id       INTEGER,
    content         TEXT
);
