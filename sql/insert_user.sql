INSERT INTO
  users (name, email, hashed_password, is_active)
VALUES
  ($1, $2, $3, $4)
RETURNING
  id
, name
, email
, created_at
, updated_at;