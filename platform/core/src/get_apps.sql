SELECT id, name, path, stateful
FROM apps
WHERE id = ANY($1)
