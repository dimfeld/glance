SELECT
  id AS "id: UserId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  name,
  email,
  avatar_url
FROM
  public.users tb
WHERE
  id = $1
  AND tb.organization_id = $2
