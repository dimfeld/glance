WITH permissions AS (
  SELECT
    bool_or(permission IN ('org_admin', 'Organization::owner')) AS is_owner,
    bool_or(permission IN ('org_admin', 'Organization::owner', 'Organization::write')) AS is_user
  FROM
    permissions
  WHERE
    organization_id = $2
    AND actor_id = ANY ($3)
    AND permission IN ('org_admin', 'Organization::owner', 'Organization::write'))
UPDATE
  organizations
SET
  name = CASE WHEN permissions.is_owner THEN
    $4
  ELSE
    organizations.name
  END,
  OWNER = CASE WHEN permissions.is_owner THEN
    $5
  ELSE
    organizations.owner
  END,
  default_role = CASE WHEN permissions.is_owner THEN
    $6
  ELSE
    organizations.default_role
  END,
  updated_at = now()
FROM
  permissions
WHERE
  id = $1
  AND (permissions.is_owner
    OR permissions.is_user)
