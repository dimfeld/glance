use serde::{Deserialize, Serialize};
use sqlx_transparent_json_decode::{sqlx_json_decode, BoxedRawValue};

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppData {
    /// The name of the app
    pub name: String,

    /// The path at which this app is installed
    pub path: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AppDataItem>,

    /// Request that the platform run the app at the specified schedule, if it does not have its own methods of scheduling updates
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<AppDataSchedule>,

    /// If false, the app does not keep its own state, so the platform should do a closer diff to see if an item has changed since the last write
    /// If true, the app can just check the updated timestamp to see if an item has changed
    #[serde(default)]
    pub stateful: bool,
}

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppDataItem {
    pub id: String,

    /// HTML to display for the item's label
    pub html: String,

    /// Whether the item can be dismissed by the viewer
    #[serde(default)]
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub dismissible: bool,

    /// Extra structured data for use by chart or other formatters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BoxedRawValue>,

    // /// Charts to display for this item
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub charts: Vec<Chart>,
    /// Notifications for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "sqlx", sqlx(json))]
    pub notify: Vec<Notification>,

    /// When the item was last updated
    pub updated: chrono::DateTime<chrono::offset::Utc>,
}

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppDataSchedule {
    /// The cron schedule for the app
    pub cron: String,

    /// Arguments to pass to the app
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(AppDataSchedule);

#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "sql", derive(sqlx::Type))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Notification {
    pub id: String,
    pub html: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(Notification);
