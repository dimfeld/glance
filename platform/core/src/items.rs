use glance_app::Notification;
use serde::Deserialize;
use sqlx::FromRow;
use sqlx_transparent_json_decode::BoxedRawValue;

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

    // Until https://github.com/launchbadge/sqlx/issues/514 is resolved we can't use flatten with
    // the query_as macro, so just duplicate the fields here.
    pub id: String,
    pub html: String,
    pub dismissible: bool,
    pub data: Option<BoxedRawValue>,
    pub notify: Option<Vec<Notification>>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,

    pub active: bool,
}
