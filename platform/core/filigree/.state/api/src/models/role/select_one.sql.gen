SELECT
  id AS "id: RoleId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  name,
  description
FROM
  public.roles tb
WHERE
  id = $1
  AND tb.organization_id = $2
