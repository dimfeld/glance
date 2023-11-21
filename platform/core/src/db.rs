mod migrations;

use std::{collections::HashMap, path::Path, rc::Rc};

use error_stack::{Report, ResultExt};
use glance_app::AppDataItemsItem;
use itertools::Itertools;
use rusqlite::{types::FromSql, Connection, Params, Row};
use serde::{de::DeserializeOwned, Deserialize};

use self::migrations::run_migrations;
use crate::{
    error::Error,
    items::{AppInfo, AppItems, Item},
};

#[derive(Clone)]
pub struct Db {
    pub pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
}

impl Db {
    pub fn new(db_path: &Path) -> Result<Self, Report<Error>> {
        let manager = r2d2_sqlite::SqliteConnectionManager::file(db_path).with_init(|c| {
            c.execute_batch(
                r##"
              PRAGMA journal_mode = wal;
              PRAGMA synchronous = normal;
              PRAGMA foreign_keys = ON;
                "##,
            )
        });

        let pool = r2d2::Pool::new(manager).expect("creating database pool");

        let mut conn = pool.get().change_context(Error::DbInit)?;
        run_migrations(&mut conn).change_context(Error::DbInit)?;

        Ok(Self { pool })
    }

    fn get_items(
        &self,
        conn: &Connection,
        where_clause: &str,
        params: impl Params,
    ) -> Result<Vec<Item>, Report<Error>> {
        let query = format!(
            r##"SELECT id, app_id, html, data, charts, updated, dismissible, active,
                    json_group_array(json_object(
                        'id', noti.id,
                        'html', noti.html,
                        'icon', noti.icon,
                        'active', noti.active
                    )) as notifications
                FROM items
                LEFT JOIN item_notifications noti
                    ON items.id = noti.item_id AND items.app_id == noti.app_id AND noti.active
                {where_clause}
                GROUP BY items.id, items.app_id"##,
        );

        let items = conn
            .prepare_cached(&query)
            .change_context(Error::Db)?
            .query_and_then(params, |row| {
                let data = Item {
                    app_id: get_column(row, 1, "app_id")?,
                    item: AppDataItemsItem {
                        id: get_column(row, 0, "id")?,
                        html: get_column(row, 2, "html")?,
                        data: get_json_column(row, 3, "data")?,
                        charts: get_json_column(row, 4, "charts")?,
                        updated: get_column(row, 5, "updated")?,
                        dismissible: get_column(row, 6, "dismissible")?,
                        notify: get_json_column(row, 8, "notifications")?,
                    },
                    active: get_column(row, 7, "active")?,
                };

                Ok::<_, Error>(data)
            })
            .change_context(Error::Db)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    /// Read all the items for the given app from the database
    pub fn read_app_items(&self, app_id: &str) -> Result<Vec<Item>, Report<Error>> {
        let conn = self.pool.get().change_context(Error::Db)?;
        self.get_items(&conn, "WHERE app_id = ?", &[app_id])
    }

    /// Read all the active items for all apps from the database
    pub fn read_active_items(&self) -> Result<Vec<AppItems>, Report<Error>> {
        let conn = self.pool.get().change_context(Error::Db)?;
        let mut items = self.get_items(&conn, "active = true", [])?;
        items.sort_unstable_by(|i1, i2| i1.app_id.cmp(&i2.app_id));

        let app_ids = items
            .iter()
            .map(|item| &item.app_id)
            // todo itertools
            .dedup()
            .cloned()
            .map(rusqlite::types::Value::from)
            .collect::<Vec<_>>();

        let mut items_by_app_id = items
            .into_iter()
            .into_grouping_map_by(|item| item.app_id.clone())
            .collect::<Vec<_>>();

        let apps = conn
            .prepare_cached(
                r##"SELECT id, name, path, stateless
            FROM apps
            WHERE id IN rarray(?)"##,
            )
            .change_context(Error::Db)?
            .query_and_then([Rc::new(app_ids)], |row| {
                let app = AppInfo {
                    id: get_column(row, 0, "id")?,
                    name: get_column(row, 1, "name")?,
                    path: get_column(row, 2, "path")?,
                    stateless: get_column(row, 3, "stateless")?,
                };

                Ok::<_, Error>(app)
            })
            .change_context(Error::Db)?
            .collect::<Result<Vec<_>, _>>()?;

        let apps_with_items = apps
            .into_iter()
            .map(|app| {
                let items = items_by_app_id.remove(&app.id).unwrap_or_default();
                AppItems { app, items }
            })
            .collect::<Vec<_>>();

        Ok(apps_with_items)
    }
}

pub fn get_column<T: FromSql>(
    row: &Row,
    index: usize,
    col_name: &'static str,
) -> Result<T, Report<Error>> {
    row.get(index).change_context(Error::DbColumn(col_name))
}

pub fn get_json_column<T: DeserializeOwned>(
    row: &Row,
    index: usize,
    col_name: &'static str,
) -> Result<T, Report<Error>> {
    let blob = row
        .get_ref(index)
        .change_context(Error::DbColumn(col_name))?
        .as_bytes()
        .change_context(Error::DbColumn(col_name))?;
    serde_json::from_slice(blob).change_context(Error::DbColumn(col_name))
}

pub fn get_optional_json_column<T: DeserializeOwned>(
    row: &Row,
    index: usize,
    col_name: &'static str,
) -> Result<Option<T>, Report<Error>> {
    let blob = row
        .get_ref(index)
        .change_context(Error::DbColumn(col_name))?
        .as_bytes_or_null()
        .change_context(Error::DbColumn(col_name))?;

    let Some(blob) = blob else {
        return Ok(None);
    };

    let result = serde_json::from_slice(blob).change_context(Error::DbColumn(col_name))?;

    Ok(Some(result))
}
