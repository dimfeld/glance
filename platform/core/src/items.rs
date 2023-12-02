use glance_app::{AppItem, AppItemData, Notification};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::instrument;

#[derive(Debug, Serialize)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct AppItems {
    pub app: AppInfo,
    pub items: Vec<Item>,
}

/// An Item as stored in the database
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub app_id: String,

    // Until https://github.com/launchbadge/sqlx/issues/514 is resolved we can't use flatten with
    // the query_as macro, so just duplicate the fields here.
    pub id: String,
    pub persistent: bool,
    pub state_key: Option<String>,
    pub data: AppItemData,
    pub notify: Option<Vec<Notification>>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,

    pub dismissed: bool,
}

impl Item {
    pub(crate) fn from_app_item(app_id: String, item: glance_app::AppItem) -> Self {
        Self {
            app_id,
            id: item.id,
            persistent: item.persistent,
            data: item.data,
            notify: Some(item.notify),
            state_key: item.state_key,
            updated_at: item.updated,
            created_at: chrono::Utc::now(),
            dismissed: false,
        }
    }

    /// Check if this item is considered changed from another item, using the state key if it is
    /// set and comparing individual fields otherwise.
    #[instrument(level = "trace")]
    pub fn changed_from(&self, other: &AppItem) -> bool {
        match (self.state_key.as_ref(), other.state_key.as_ref()) {
            (Some(a), Some(b)) => a != b,
            (Some(_), None) => true,
            (None, Some(_)) => true,
            (None, None) => self.equal_stateless(other),
        }
    }

    /// When the code that generated the item was not aware of the previous generated items,
    /// check all the data fields, except the updated timestamp.
    #[instrument(level = "trace")]
    pub fn equal_stateless(&self, other: &AppItem) -> bool {
        // let notify_match = match &self.notify {
        //     Some(notify) => notify == &other.notify,
        //     None => other.notify.is_empty(),
        // };

        self.id == other.id
            && self.data.title == other.data.title
            && self.data.subtitle == other.data.subtitle
            && self.data.detail == other.data.detail
            && self.data.icon == other.data.icon
            && self.persistent == other.persistent
        // && self.charts == other.charts
    }
}
