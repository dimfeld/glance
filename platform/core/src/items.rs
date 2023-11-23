use std::ops::Deref;

use glance_app::AppDataItem;
use serde::Deserialize;
use sqlx::FromRow;

pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub stateful: bool,
}

pub struct AppItems {
    pub app: AppInfo,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Item {
    pub app_id: String,
    #[serde(flatten)]
    #[sqlx(flatten)]
    pub item: AppDataItem,
    pub active: bool,
}

impl Deref for Item {
    type Target = AppDataItem;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}
