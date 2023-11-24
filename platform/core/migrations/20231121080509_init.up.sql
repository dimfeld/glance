CREATE TABLE apps (
  id text PRIMARY KEY,
  name text NOT NULL,
  path text NOT NULL,
  stateful boolean NOT NULL DEFAULT false,
  error text
);

CREATE TABLE schedules (
  app_id text REFERENCES apps(id) ON DELETE CASCADE,
  cron text NOT NULL,
  arguments jsonb,
  PRIMARY KEY (app_id, cron)
);

CREATE TABLE items (
  id text,
  app_id text REFERENCES apps(id) ON DELETE CASCADE,
  html text NOT NULL,
  data jsonb,
  charts jsonb,
  created_at timestamptz NOT NULL DEFAULT NOW(),
  updated_at timestamptz NOT NULL,
  dismissible boolean NOT NULL DEFAULT false,
  active boolean NOT NULL DEFAULT TRUE,
  PRIMARY KEY (id, app_id)
);

CREATE INDEX ON items (app_id);

CREATE TABLE item_notifications (
  id text PRIMARY KEY,
  item_id text,
  app_id text REFERENCES apps(id) ON DELETE CASCADE,
  html text NOT NULL,
  icon text,
  active boolean NOT NULL DEFAULT TRUE,
  FOREIGN KEY (item_id, app_id) REFERENCES items(id, app_id) ON DELETE CASCADE
);

CREATE INDEX ON item_notifications (app_id, item_id);

CREATE TYPE event_type AS enum (
  'create_item',
  'update_item',
  'delete_item',
  'scheduled_run'
);

CREATE TABLE EVENTS (
  id bigint PRIMARY KEY generated always AS identity,
  event_type event_type NOT NULL,
  app_id text NOT NULL,
  item_id text,
  metadata jsonb,
  created_at timestamptz NOT NULL DEFAULT NOW()
);

CREATE INDEX ON EVENTS (app_id, created_at DESC);
