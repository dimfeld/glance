SELECT
  id,
  updated_at,
  created_at,
  name,
  OWNER,
  default_role,
  active
FROM
  public.organizations tb
WHERE
  __insertion_point_filters
LIMIT $2 OFFSET $3
