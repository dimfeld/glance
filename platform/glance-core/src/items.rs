use std::ops::Deref;

use glance_app::AppDataItemsItem;

pub struct Item(AppDataItemsItem);

impl Item {
    pub fn equal_shallow(&self, other: &AppDataItemsItem) -> bool {
        self.0.id == other.id && self.0.updated == other.updated
    }

    pub fn equal_deep(&self, other: &AppDataItemsItem) -> bool {
        self.0.id == other.id && self.0.data == other.data
    }
}

impl Deref for Item {
    type Target = AppDataItemsItem;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
