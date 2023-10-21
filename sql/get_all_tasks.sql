SELECT
  T.id
, T.summary
, T.description
, T.status_id
, S.name AS status_name
, T.created_at
, T.updated_at
FROM
  tasks AS T
  INNER JOIN status AS S ON T.status_id = S.id
ORDER BY
  T.id