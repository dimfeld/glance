mod migrations;

use std::path::Path;

use error_stack::{Report, ResultExt};
use glance_app::AppDataItemsItem;
use rusqlite::{types::FromSql, Row};
use serde::{de::DeserializeOwned, Deserialize};

use self::migrations::run_migrations;
use crate::{error::Error, items::Item};

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

    /// Read the active items for the given app from the database
    pub fn read_app_items(&self, app_id: &str) -> Result<Vec<Item>, Report<Error>> {
        let conn = self.pool.get().change_context(Error::Db)?;
        let items = conn
            .prepare_cached(
                r##"SELECT id, html, data, charts, updated, dismissible, active,
                    json_group_array(json_object(
                        'id', noti.id,
                        'html', noti.html,
                        'icon', noti.icon,
                        'active', noti.active
                    )) as notifications
                FROM items
                LEFT JOIN item_notifications noti
                    ON items.id = noti.item_id AND items.app_id == noti.app_id AND noti.active
                WHERE app_id = ?
                GROUP BY items.id"##,
            )
            .change_context(Error::Db)?
            .query_and_then([app_id], |row| {
                let data = Item {
                    item: AppDataItemsItem {
                        id: get_column(row, 0, "id")?,
                        html: get_column(row, 1, "html")?,
                        data: get_json_column(row, 2, "data")?,
                        charts: get_json_column(row, 3, "charts")?,
                        updated: get_column(row, 4, "updated")?,
                        dismissible: get_column(row, 5, "dismissible")?,
                        notify: get_json_column(row, 7, "notifications")?,
                    },
                    active: get_column(row, 6, "active")?,
                };

                Ok::<_, Error>(data)
            })
            .change_context(Error::Db)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(items)
    }

    pub fn read_active_items(&self) -> Result<Vec<AppItems>, Report<Error>> {
        todo!()
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
