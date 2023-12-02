SELECT
  id,
  name,
  path
FROM
  apps
WHERE
  id = ANY ($1)
