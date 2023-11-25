DELETE FROM items
WHERE app_id = $1
  AND id <> ALL ($2)
