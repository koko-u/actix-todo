SELECT
  id
, name
, email
, hashed_password
, created_at
, updated_at
FROM
  users
WHERE
  email = $1;