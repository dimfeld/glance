INSERT INTO items (
  id,
  app_id,
  data,
  state_key,
  persistent,
  updated_at)
VALUES (
  $1,
  $2,
  $3,
  $4,
  $5,
  $6)
ON CONFLICT (
  app_id,
  id)
  DO UPDATE SET
    data = EXCLUDED.data,
    persistent = EXCLUDED.persistent,
    state_key = EXCLUDED.state_key,
    updated_at = NOW(),
    dismissed = CASE WHEN $7 THEN
      FALSE
    ELSE
      items.dismissed
    END
