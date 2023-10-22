mod migrations;

use std::path::Path;

use error_stack::{Report, ResultExt};

use self::migrations::run_migrations;
use crate::error::Error;

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
    pub fn read_items(app_id: &str) {
        todo!()
    }
}
