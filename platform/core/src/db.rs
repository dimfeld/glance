use error_stack::{Report, ResultExt};
use glance_app::Notification;
use itertools::Itertools;
use sqlx::postgres::PgPool;
use sqlx_transparent_json_decode::BoxedRawValue;

use crate::{
    error::Error,
    items::{AppInfo, AppItems, Item},
};

#[derive(Clone)]
pub struct Db {
    pub(crate) pool: sqlx::PgPool,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, Report<Error>> {
        let pool = PgPool::connect(database_url)
            .await
            .change_context(Error::Db)?;
        sqlx::migrate!()
            .run(&pool)
            .await
            .change_context(Error::Db)?;

        Ok(Self { pool })
    }

    pub async fn get_apps(&self, app_ids: &[String]) -> Result<Vec<AppInfo>, Report<Error>> {
        sqlx::query_file_as!(AppInfo, "src/get_apps.sql", app_ids)
            .fetch_all(&self.pool)
            .await
            .change_context(Error::Db)
    }

    /// Read all the items for the given app from the database
    pub async fn read_app_items(&self, app_id: &str) -> Result<Vec<Item>, Report<Error>> {
        let items = sqlx::query_file_as!(Item, "src/get_items_by_app_id.sql", app_id)
            .fetch_all(&self.pool)
            .await
            .change_context(Error::Db)?;

        Ok(items)
    }

    /// Read all the active items for all apps from the database
    pub async fn read_active_items(&self) -> Result<Vec<AppItems>, Report<Error>> {
        let mut items = sqlx::query_file_as!(Item, "src/get_active_items.sql")
            .fetch_all(&self.pool)
            .await
            .change_context(Error::Db)?;

        items.sort_unstable_by(|i1, i2| i1.app_id.cmp(&i2.app_id));

        let mut items_by_app_id = items
            .into_iter()
            .into_grouping_map_by(|item| item.app_id.to_string())
            .collect::<Vec<_>>();
        let app_ids = items_by_app_id.keys().cloned().collect::<Vec<_>>();
        let apps = self.get_apps(&app_ids).await?;

        let apps_with_items = apps
            .into_iter()
            .map(|app| {
                let items = items_by_app_id.remove(app.id.as_str()).unwrap_or_default();
                AppItems { app, items }
            })
            .collect::<Vec<_>>();

        Ok(apps_with_items)
    }
}
