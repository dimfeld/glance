use glance_app::{AppDataItem, Notification};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_transparent_json_decode::BoxedRawValue;
use tracing::instrument;

#[derive(Debug, Serialize)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub stateful: bool,
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
    pub html: String,
    pub persistent: bool,
    pub data: Option<BoxedRawValue>,
    pub notify: Option<Vec<Notification>>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,

    pub dismissed: bool,
}

impl Item {
    pub(crate) fn from_app_item(app_id: String, item: glance_app::AppDataItem) -> Self {
        Self {
            app_id,
            id: item.id,
            html: item.html,
            persistent: item.persistent,
            data: item.data,
            notify: Some(item.notify),
            updated_at: item.updated,
            created_at: chrono::Utc::now(),
            dismissed: false,
        }
    }

    /// Just check that the ID and the updated time of the item are the same.
    #[instrument(level = "trace")]
    pub fn equal_stateful(&self, other: &AppDataItem) -> bool {
        self.id == other.id && self.updated_at == other.updated
    }

    /// When the code that generated the item was not aware of the previous generated items,
    /// check all the data fields, except the updated timestamp.
    #[instrument(level = "trace")]
    pub fn equal_stateless(&self, other: &AppDataItem) -> bool {
        let data = self.data.as_ref().map(|s| s.get()).unwrap_or("");
        let other_data = other.data.as_ref().map(|s| s.get()).unwrap_or("");

        let notify_match = match &self.notify {
            Some(notify) => notify == &other.notify,
            None => other.notify.is_empty(),
        };

        self.id == other.id
            && self.html == other.html
            && notify_match
            && self.persistent == other.persistent
            // && self.charts == other.charts
            && data == other_data
    }
}
