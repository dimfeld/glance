SELECT
  id AS "id: OrganizationId",
  updated_at,
  created_at,
  name,
  OWNER AS "owner: crate::models::user::UserId",
  default_role AS "default_role: crate::models::role::RoleId",
  active
FROM
  public.organizations tb
WHERE
  id = $1
