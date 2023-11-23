CREATE TABLE apps (
  id text primary key,
  name text not null,
  path text not null,
  stateful boolean not null default false
);

CREATE TABLE schedules (
  app_id text references apps(id) ON DELETE CASCADE,
  cron text not null,
  arguments jsonb,
  primary key (app_id, cron)
);

CREATE TABLE items (
  id text,
  app_id text references apps(id) ON DELETE CASCADE,
  html text not null,
  data jsonb not null,
  charts jsonb not null,
  updated bigint not null,
  dismissible boolean default false,
  active boolean default true,
  primary key (id, app_id)
);

CREATE INDEX ON items (app_id);

CREATE TABLE item_notifications (
  id text primary key,
  item_id text,
  app_id text references apps(id) ON DELETE CASCADE,
  html text not null,
  icon text,
  active boolean default true,
  foreign key (item_id, app_id) references items(id, app_id) ON DELETE CASCADE
);

CREATE INDEX ON item_notifications (app_id, item_id);
