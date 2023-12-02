INSERT INTO apps (
  id,
  name,
  path,
  ui)
VALUES (
  $1,
  $2,
  $3,
  $4)
ON CONFLICT (
  id)
  DO UPDATE SET
    name = EXCLUDED.name,
    path = EXCLUDED.path,
    ui = EXCLUDED.ui,
    updated_at = NOW(),
    error = NULL;
