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

    /// Information only used to render the UI of the app
    pub ui: Option<AppUiInfo>,

    /// A version number for the app metadata. If this is present, the metadata will only be
    /// updated if the version number in the submitted data is greater than or equal to the number
    /// in the database.
    ///
    /// The primary purpose of this is to allow independent instances of the same app to submit updates
    /// without old versions rolling back the app metadata.
    ///
    /// Note also that `version` does not apply to the `items` array.
    #[serde(default = "zero")]
    pub version: u32,
}

fn zero() -> u32 {
    0
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

/// An item published by the app
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppItem {
    /// An ID that uniquely identifies this item among others published by the app
    pub id: String,

    /// Display information for the item
    pub data: AppItemData,

    /// An ID that can be compared to a previous copy of the item to see if it should be considered
    /// changed. On an item change, the data will be updated regardless, but the "dismissed" state
    /// will be reset only if state_key has changed, so this can be used to skip resurfacing an
    /// item when only small changes have been made.
    ///
    /// If state_key is not used, the platform will compare individual fields of the item.
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    pub state_key: Option<String>,

    /// Whether the item can be dismissed by the viewer
    #[cfg_attr(feature = "sqlx", sqlx(default))]
    #[serde(default)]
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

    /// A URL to open when the title is clicked
    pub url: Option<String>,

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

    /// How long to wait, in seconds, for the app to execute before killing it and retrying.
    /// Defaults to 5 minutes, or 300 seconds.
    /// This uses an int instead of a [Duration] for better interoperability with non-Rust apps.
    pub timeout: Option<u32>,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(AppSchedule);

/// A notification from the app
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Notification {
    /// A unique ID among other notifications for this app
    pub id: String,

    /// Data for the notification
    pub data: NotificationData,
}

#[cfg(feature = "sqlx")]
sqlx_json_decode!(Notification);

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
