SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  name,
  description
FROM
  public.roles tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
