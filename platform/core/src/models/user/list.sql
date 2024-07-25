SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  name,
  email,
  avatar_url
FROM
  public.users tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
