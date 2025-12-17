INSERT INTO Sessions (user_id, device_name, expires_at)
VALUES (?, ?, ?)
RETURNING id
