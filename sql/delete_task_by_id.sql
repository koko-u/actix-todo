WITH
  deleted_row AS (
    DELETE FROM tasks
    WHERE
      id = $1
    RETURNING
      id
    , summary
    , description
    , status_id
    , user_id
    , created_at
    , updated_at
  )
SELECT
  T.id AS "id!"
, T.summary AS "summary!"
, T.description
, T.status_id AS "status_id!"
, S.name AS "status_name!"
, T.user_id
, U.name AS user_name
, U.email AS user_email
, T.created_at AS "created_at!"
, T.updated_at AS "updated_at!"
FROM
  deleted_row AS T
  LEFT OUTER JOIN status AS S ON T.status_id = S.id
  LEFT OUTER JOIN users AS U ON T.user_id = U.id;