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
      *
    FROM
      tasks
    WHERE
      id = $1
  ) AS T
  INNER JOIN status AS S ON T.status_id = S.id