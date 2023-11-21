CREATE TABLE apps (
  id text primary key,
  name text not null,
  path text not null,
  stateless boolean not null default false,
);

CREATE TABLE schedules (
  app_id text references apps(id) ON DELETE CASCADE,
  cron text not null,
  arguments text,
  primary key (app_id, cron)
);

CREATE TABLE items (
  id text,
  app_id text references apps(id) ON DELETE CASCADE,
  html text not null,
  data text not null,
  charts text not null,
  updated bigint not null,
  dismissible boolean default false,
  active boolean default true,
  (id, app_id) primary key
);

CREATE INDEX ON items (app_id);

CREATE TABLE item_notifications (
  id text primary key,
  item_id text references items(id) ON DELETE CASCADE,
  app_id text references apps(id) ON DELETE CASCADE,
  html text not null,
  icon text,
  active boolean default true
);

CREATE INDEX ON item_notifications (item_id);
CREATE INDEX ON item_notifications (app_id);
