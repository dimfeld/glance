use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx_transparent_json_decode::sqlx_json_decode;
use sqlx_transparent_json_decode::BoxedRawValue;

/// The top-level data for the app
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppData {
    /// The name of the app
    pub name: String,

    /// The path at which this app is installed
    pub path: String,

    /// An array of data items that the app is publishing
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AppItem>,

    /// Request that the platform run the app at the specified schedule, if it does not have its own methods of scheduling updates
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<AppSchedule>,

    /// If false, the app does not keep its own state, so the platform should do a closer diff to see if an item has changed since the last write
    /// If true, the app can just check the updated timestamp to see if an item has changed
    #[serde(default)]
    pub stateful: bool,

    /// Information only used to render the UI of the app
    pub ui: Option<AppUiInfo>,
}

/// Information only used to render the UI of the app
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppUiInfo {
    /// The icon that the app should show (exact format TBD)
    pub icon: Option<String>,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(AppUiInfo);

fn bool_true() -> bool {
    true
}

/// An item published by the app
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppItem {
    /// An ID that uniquely identifies this item among others published by the app
    pub id: String,

    /// Display information for the item
    pub data: AppItemData,

    /// Whether the item can be dismissed by the viewer
    #[serde(default = "bool_true")]
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub persistent: bool,

    /// Notifications for this item
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "sqlx", sqlx(json))]
    pub notify: Vec<Notification>,

    /// When the item was last updated
    pub updated: chrono::DateTime<chrono::offset::Utc>,
}

/// Information for an app item
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppItemData {
    /// The title at the top of the card
    pub title: String,
    /// A subtitle to display below the title
    pub subtitle: Option<String>,
    /// Extra information which can be shown
    pub detail: Option<String>,

    /// An icon to show with this item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Extra structured data for use by chart or other formatters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BoxedRawValue>,
    // /// Charts to display for this item
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub charts: Vec<Chart>,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(AppItemData);

/// A schedule on which to run this app. This is not implemented yet.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppSchedule {
    /// The cron schedule for the app
    pub cron: String,

    /// Arguments to pass to the app
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(AppSchedule);

/// A notification from the app
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Notification {
    /// A unique ID among other notifications for this app
    pub id: String,

    /// Data for the notification
    pub data: NotificationData,
}

/// Data for a notification
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotificationData {
    /// The title at the top of the card
    pub title: String,
    /// A subtitle to display below the title
    pub subtitle: Option<String>,

    /// An icon to show with the notification
    pub icon: Option<String>,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(NotificationData);
