SELECT
  T.id
, T.summary
, T.description
, T.status_id
, S.name AS status_name
, T.created_at
, T.updated_at
FROM
  (
    SELECT
      id
    , summary
    , description
    , status_id
    , created_at
    , updated_at
    FROM
      tasks
    WHERE
      summary ILIKE CASE
        WHEN $1::varchar IS NULL THEN summary
        ELSE '%' || $1::varchar || '%'
      END
  ) AS T
  INNER JOIN status AS S ON T.status_id = S.id
ORDER BY
  T.id