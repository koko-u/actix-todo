WITH
  inserted_row AS (
    INSERT INTO
      tasks (summary, description, status_id, user_id)
    VALUES
      ($1::varchar, $2::varchar, $3::bigint, $4::bigint)
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
, T.user_id
, U.name AS user_name
, U.email AS user_email
, S.name AS "status_name!"
, T.created_at AS "created_at!"
, T.updated_at AS "updated_at!"
FROM
  inserted_row AS T
  LEFT OUTER JOIN status AS S ON T.status_id = S.id
  LEFT OUTER JOIN users AS U ON T.user_id = U.id;