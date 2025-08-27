-- Users table
CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tag TEXT UNIQUE CHECK (
        length (tag) <= 32
        AND tag NOT GLOB '*[^a-zA-Z0-9]*'
    ),
    username TEXT CHECK (length (username) <= 32),
    avatar_url TEXT,
    banner_url TEXT,
    password_hash TEXT NOT NULL,
    -- token TEXT UNIQUE,
    telegram_id INTEGER,
    banned BOOLEAN NOT NULL DEFAULT 0,
    role TEXT NOT NULL DEFAULT 'default' CHECK (role IN ('default', 'moderator', 'admin')),
    is_legend BOOLEAN NOT NULL DEFAULT 0,
    permissions INTEGER NOT NULL DEFAULT 7,
    privacy INTEGER NOT NULL DEFAULT 3,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Posts table
CREATE TABLE IF NOT EXISTS Posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    author_id INTEGER NOT NULL,
    name TEXT CHECK (length (name) <= 256),
    content TEXT CHECK (length (content) <= 65536),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
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
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES Users (tag) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES Posts (id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES Comments (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rolename TEXT NOT NULL CHECK (length (rolename) <= 32),
    permissions INTEGER NOT NULL -- GlobalPermissions
);

CREATE TABLE IF NOT EXISTS UserRoles (
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES Roles(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS PostPermissions (
    user_id INTEGER NOT NULL,
    post_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL DEFAULT 10,
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES Posts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS UserPermission (
    owner_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL DEFAULT 3,
    PRIMARY KEY (owner_id, user_id),
    FOREIGN KEY (owner_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    CHECK (owner_id != user_id)
);

CREATE TABLE IF NOT EXISTS Sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    device_name TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
);
