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
    , created_at
    , updated_at
  )
SELECT
  T.id AS "id!"
, T.summary AS "summary!"
, T.description
, T.status_id AS "status_id!"
, S.name AS "status_name!"
, T.created_at AS "created_at!"
, T.updated_at AS "updated_at!"
FROM
  deleted_row AS T
  LEFT OUTER JOIN status AS S ON T.status_id = S.id;