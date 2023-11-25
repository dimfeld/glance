INSERT INTO events (
  event_type,
  app_id,
  item_id,
  metadata)
VALUES (
  $1 ::event_type,
  $2,
  $3,
  $4)
