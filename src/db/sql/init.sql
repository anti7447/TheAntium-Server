-- Users table
CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tag TEXT UNIQUE CHECK (
        length (tag) <= 32
        AND tag GLOB '[0-1a-zA-Z]*'
    ),
    username TEXT CHECK (length (username) <= 32),
    avatar_url TEXT,
    banner_url TEXT,
    password_hash TEXT NOT NULL,
    token TEXT UNIQUE,
    telegram_id INTEGER,
    banned BOOLEAN NOT NULL DEFAULT 0,
    role TEXT NOT NULL DEFAULT 'default' CHECK (role IN ('default', 'moderator', 'admin')),
    is_legend BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Posts table
CREATE TABLE IF NOT EXISTS Posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    author_id INTEGER NOT NULL,
    name TEXT CHECK (length (name) <= 256),
    content TEXT CHECK (length (content) <= 65536),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Views table (for posts)
CREATE TABLE IF NOT EXISTS Views (
    post_id INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (post_id, user_id),
    FOREIGN KEY (post_id) REFERENCES Posts (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users (tag) ON DELETE CASCADE
);

-- Bookmark table
CREATE TABLE IF NOT EXISTS Bookmark (
    post_id INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (post_id, user_id),
    FOREIGN KEY (post_id) REFERENCES Posts (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users (tag) ON DELETE CASCADE
);

-- Comments table
CREATE TABLE IF NOT EXISTS Comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    post_id INTEGER NOT NULL,
    parent_id INTEGER,
    depth INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users (tag) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES Posts (id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES Comments (id) ON DELETE CASCADE
);
