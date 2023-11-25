INSERT INTO items (
  id,
  app_id,
  html,
  data,
  dismissible)
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
    dismissible = EXCLUDED.dismissible,
    updated_at = NOW(),
    active = TRUE;
