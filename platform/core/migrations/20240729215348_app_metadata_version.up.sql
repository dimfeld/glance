ALTER TABLE apps
  ADD COLUMN version bigint NOT NULL DEFAULT 0;

COMMENT ON COLUMN apps.version IS 'Version of the app metadata schema';
