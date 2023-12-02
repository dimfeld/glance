ALTER TABLE items
  ADD COLUMN state_key TEXT;

ALTER TABLE apps
  DROP COLUMN stateful;
