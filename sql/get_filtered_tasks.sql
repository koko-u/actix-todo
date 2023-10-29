WITH
  filtered_tasks (id, summary, description, status_id, created_at, updated_at) AS (
    SELECT
      id
    , summary
    , description
    , status_id
    , created_at
    , updated_at
    , user_id
    FROM
      tasks
    WHERE
      user_id = $4
      AND summary ILIKE CASE
        WHEN $1::varchar IS NULL THEN summary
        ELSE '%' || $1::varchar || '%'
      END
  )
, joined_status (id, name, target, is_empty) AS (
    SELECT
      S.id
    , S.name
    , A.target
    , $3::boolean
    FROM
      status AS S
      LEFT OUTER JOIN (
        SELECT
          UNNEST AS id
        , TRUE AS target
        FROM
          UNNEST($2::bigint[])
      ) AS A ON S.id = A.id
  )
SELECT
  T.id
, T.summary
, T.description
, T.status_id
, S.name AS status_name
, T.user_id
, U.name AS user_name
, U.email AS user_email
, T.created_at
, T.updated_at
FROM
  filtered_tasks AS T
  INNER JOIN (
    SELECT
      id
    , name
    FROM
      joined_status
    WHERE
      target IS NOT NULL
      OR is_empty = TRUE
  ) AS S ON T.status_id = S.id
  LEFT OUTER JOIN users AS U ON T.user_id = U.id
ORDER BY
  T.id