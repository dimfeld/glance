INSERT INTO apps (
  id,
  name,
  path,
  stateful)
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
    stateful = EXCLUDED.stateful,
    updated_at = NOW();
