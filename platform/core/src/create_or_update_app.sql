INSERT INTO apps (
  id,
  name,
  path,
  stateful,
  ui)
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
    stateful = EXCLUDED.stateful,
    ui = EXCLUDED.ui,
    updated_at = NOW(),
    error = NULL;
