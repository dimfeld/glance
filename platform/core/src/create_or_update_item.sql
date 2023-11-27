INSERT INTO items (
  id,
  app_id,
  html,
  data,
  persistent)
VALUES (
  $1,
  $2,
  $3,
  $4,
  $5)
ON CONFLICT (
  app_id,
  id)
  DO UPDATE SET
    html = EXCLUDED.html,
    data = EXCLUDED.data,
    persistent = EXCLUDED.persistent,
    updated_at = NOW(),
    dismissed = FALSE;
