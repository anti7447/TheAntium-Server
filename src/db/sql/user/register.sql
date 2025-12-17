INSERT INTO users (tag, username, password_hash)
VALUES (?, ?, ?)
RETURNING id;
