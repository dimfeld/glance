UPDATE
  public.roles
SET
  name = $1,
  description = $2,
  updated_at = NOW()
WHERE
  id = $3
  AND organization_id = $4
