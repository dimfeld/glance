use std::ops::Deref;

use glance_app::AppDataItemsItem;
use serde::Deserialize;

pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub stateless: bool,
}

pub struct AppItems {
    pub app: AppInfo,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub app_id: String,
    #[serde(flatten)]
    pub item: AppDataItemsItem,
    pub active: bool,
}

impl Item {
    /// Check that the ID and the updated time of the item are the same.
    pub fn equal_shallow(&self, other: &AppDataItemsItem) -> bool {
        self.item.id == other.id && self.item.updated == other.updated
    }

    /// When the code that generated the item was not aware of the previous generated items,
    /// check all the data fields, except the updated timestamp.
    pub fn equal_deep(&self, other: &AppDataItemsItem) -> bool {
        self.item.id == other.id
            && self.item.data == other.data
            && self.item.html == other.html
            && self.item.charts == other.charts
            && self.item.notify == other.notify
            && self.item.dismissible == other.dismissible
    }
}

impl Deref for Item {
    type Target = AppDataItemsItem;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}
