CREATE TABLE apps (
  id text PRIMARY KEY,
  name text NOT NULL,
  path text NOT NULL,
  stateful boolean NOT NULL DEFAULT FALSE,
  updated_at timestamptz NOT NULL DEFAULT NOW(),
  error text
);

CREATE TABLE schedules (
  app_id text REFERENCES apps (id) ON DELETE CASCADE,
  cron text NOT NULL,
  arguments jsonb,
  PRIMARY KEY (app_id, cron)
);

CREATE TABLE items (
  id text,
  app_id text REFERENCES apps (id) ON DELETE CASCADE,
  html text NOT NULL,
  data jsonb,
  charts jsonb,
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL,
  dismissible boolean NOT NULL DEFAULT FALSE,
  active boolean NOT NULL DEFAULT TRUE,
  PRIMARY KEY (id, app_id)
);

CREATE INDEX ON items (app_id);

CREATE TABLE item_notifications (
  id text PRIMARY KEY,
  item_id text,
  app_id text REFERENCES apps (id) ON DELETE CASCADE,
  html text NOT NULL,
  icon text,
  active boolean NOT NULL DEFAULT TRUE,
  FOREIGN KEY (item_id, app_id) REFERENCES items (id, app_id) ON DELETE CASCADE
);

CREATE INDEX ON item_notifications (app_id, item_id);

CREATE TYPE event_type AS enum (
  'create_item',
  'update_item',
  'remove_item',
  'remove_app',
  'scheduled_run'
);

CREATE TABLE events (
  id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  event_type event_type NOT NULL,
  app_id text NOT NULL,
  item_id text,
  metadata jsonb,
  created_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE INDEX ON events (app_id, created_at DESC);

CREATE FUNCTION handle_item_trigger ()
  RETURNS TRIGGER
  AS $$
BEGIN
  IF (TG_OP = 'DELETE') THEN
    INSERT INTO events (
      event_type,
      app_id,
      item_id)
    VALUES (
      'remove_item',
      OLD.app_id,
      OLD.id);
  ELSIF (TG_OP = 'UPDATE') THEN
    INSERT INTO events (
      event_type,
      app_id,
      item_id)
    VALUES (
      'update_item',
      NEW.app_id,
      NEW.id);
  ELSIF (TG_OP = 'INSERT') THEN
    INSERT INTO events (
      event_type,
      app_id,
      item_id)
    VALUES (
      'create_item',
      NEW.app_id,
      NEW.id);
  END IF;
  RETURN NULL;
END;
$$
LANGUAGE plpgsql;

CREATE FUNCTION app_delete_trigger ()
  RETURNS TRIGGER
  AS $$
BEGIN
  INSERT INTO events (
    event_type,
    app_id)
  VALUES (
    'remove_app',
    OLD.id);
  RETURN NULL;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER item_event_trigger
  AFTER INSERT OR UPDATE OR DELETE ON items
  FOR EACH ROW
  EXECUTE FUNCTION handle_item_trigger ();

CREATE TRIGGER app_event_trigger
  AFTER DELETE ON apps
  FOR EACH ROW
  EXECUTE FUNCTION app_delete_trigger ();
