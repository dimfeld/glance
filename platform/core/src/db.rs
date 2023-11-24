use error_stack::{Report, ResultExt};
use glance_app::{AppData, Notification};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnection, PgPool},
    Connection, Transaction,
};
use sqlx_transparent_json_decode::BoxedRawValue;

use crate::{
    error::Error,
    items::{AppInfo, AppItems, Item},
};

#[derive(Clone)]
pub struct Db {
    pub(crate) pool: sqlx::PgPool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    CreateItem,
    UpdateItem,
    RemoveItem,
    RemoveApp,
    ScheduledRun,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, Report<Error>> {
        let pool = PgPool::connect(database_url)
            .await
            .change_context(Error::DbInit)?;
        sqlx::migrate!()
            .run(&pool)
            .await
            .change_context(Error::DbInit)?;

        Ok(Self { pool })
    }

    async fn add_event(
        &self,
        tx: &mut PgConnection,
        event_type: EventType,
        app_id: &str,
        item_id: Option<&str>,
        data: Option<serde_json::Value>,
    ) -> Result<(), Report<Error>> {
        sqlx::query_file!("src/add_event.sql", event_type as _, app_id, item_id, data)
            .execute(tx)
            .await
            .change_context(Error::Db)?;
        Ok(())
    }

    pub async fn get_apps(&self, app_ids: &[String]) -> Result<Vec<AppInfo>, Report<Error>> {
        sqlx::query_file_as!(AppInfo, "src/get_apps.sql", app_ids)
            .fetch_all(&self.pool)
            .await
            .change_context(Error::Db)
    }

    /// Remove an app and all its associated items.
    pub async fn remove_app(&self, app_id: &str) -> Result<(), Report<Error>> {
        sqlx::query_file!("src/remove_app.sql", app_id)
            .execute(&self.pool)
            .await
            .change_context(Error::Db)?;

        Ok(())
    }

    pub async fn create_or_update_app(&self, app: &AppData) -> Result<(), Report<Error>> {
        todo!()
    }

    pub async fn create_or_update_item(
        &self,
        tx: &mut PgConnection,
        item: &Item,
    ) -> Result<(), Report<Error>> {
        sqlx::query_file!(
            "src/create_or_update_item.sql",
            item.id,
            item.app_id,
            item.html,
            item.data.as_deref() as _,
            item.dismissible
        )
        .execute(tx)
        .await
        .change_context(Error::Db)?;
        Ok(())
    }

    pub async fn remove_unfound_items(
        &self,
        tx: &mut PgConnection,
        app_id: &str,
        item_ids: &[String],
    ) -> Result<(), Report<Error>> {
        sqlx::query_file!("src/remove_unfound_items.sql", app_id, item_ids)
            .execute(tx)
            .await
            .change_context(Error::Db)?;
        Ok(())
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
