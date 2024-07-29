INSERT INTO apps (
  id,
  name,
  path,
  ui,
  version)
VALUES (
  $1,
  $2,
  $3,
  $4,
  $5)
ON CONFLICT (
  id)
  DO UPDATE SET
    name = EXCLUDED.name,
    path = EXCLUDED.path,
    ui = EXCLUDED.ui,
    version = EXCLUDED.version,
    updated_at = NOW(),
    error = NULL
  WHERE
    EXCLUDED.version >= apps.version;
