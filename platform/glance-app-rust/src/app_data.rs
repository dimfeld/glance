#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AppDataItemsItem>,
    #[doc = "The name of the app"]
    pub name: String,
    #[doc = "The path at which this app is installed"]
    pub path: String,
    #[doc = "Request that the platform run the app at the specified schedule, if it does not have its own methods of scheduling updates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<AppDataScheduleItem>,
    #[doc = "If true, the app does not keep its own state, so the platform should do a closer diff to see if an item has changed since the last write"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateless: Option<bool>,
}
impl From<&AppData> for AppData {
    fn from(value: &AppData) -> Self {
        value.clone()
    }
}
impl AppData {
    pub fn builder() -> builder::AppData {
        builder::AppData::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppDataItemsItem {
    #[doc = "Charts to display for this item"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub charts: Vec<Chart>,
    #[doc = "Extra structured data for use by chart or other formatters"]
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub data: serde_json::Map<String, serde_json::Value>,
    #[doc = "HTML to display for the item's label"]
    pub html: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notify: Vec<Notification>,
    #[doc = "Date the item was last updated"]
    pub updated: chrono::DateTime<chrono::offset::Utc>,
}
impl From<&AppDataItemsItem> for AppDataItemsItem {
    fn from(value: &AppDataItemsItem) -> Self {
        value.clone()
    }
}
impl AppDataItemsItem {
    pub fn builder() -> builder::AppDataItemsItem {
        builder::AppDataItemsItem::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppDataScheduleItem {
    #[doc = "Arguments to pass to the app"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
    #[doc = "The cron schedule for the app"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
}
impl From<&AppDataScheduleItem> for AppDataScheduleItem {
    fn from(value: &AppDataScheduleItem) -> Self {
        value.clone()
    }
}
impl AppDataScheduleItem {
    pub fn builder() -> builder::AppDataScheduleItem {
        builder::AppDataScheduleItem::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chart(pub serde_json::Map<String, serde_json::Value>);
impl std::ops::Deref for Chart {
    type Target = serde_json::Map<String, serde_json::Value>;
    fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.0
    }
}
impl From<Chart> for serde_json::Map<String, serde_json::Value> {
    fn from(value: Chart) -> Self {
        value.0
    }
}
impl From<&Chart> for Chart {
    fn from(value: &Chart) -> Self {
        value.clone()
    }
}
impl From<serde_json::Map<String, serde_json::Value>> for Chart {
    fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Notification {
    pub html: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub id: String,
}
impl From<&Notification> for Notification {
    fn from(value: &Notification) -> Self {
        value.clone()
    }
}
impl Notification {
    pub fn builder() -> builder::Notification {
        builder::Notification::default()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AppData {
        items: Result<Vec<super::AppDataItemsItem>, String>,
        name: Result<String, String>,
        path: Result<String, String>,
        schedule: Result<Vec<super::AppDataScheduleItem>, String>,
        stateless: Result<Option<bool>, String>,
    }
    impl Default for AppData {
        fn default() -> Self {
            Self {
                items: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                path: Err("no value supplied for path".to_string()),
                schedule: Ok(Default::default()),
                stateless: Ok(Default::default()),
            }
        }
    }
    impl AppData {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::AppDataItemsItem>>,
            T::Error: std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn schedule<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::AppDataScheduleItem>>,
            T::Error: std::fmt::Display,
        {
            self.schedule = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schedule: {}", e));
            self
        }
        pub fn stateless<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.stateless = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stateless: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AppData> for super::AppData {
        type Error = String;
        fn try_from(value: AppData) -> Result<Self, String> {
            Ok(Self {
                items: value.items?,
                name: value.name?,
                path: value.path?,
                schedule: value.schedule?,
                stateless: value.stateless?,
            })
        }
    }
    impl From<super::AppData> for AppData {
        fn from(value: super::AppData) -> Self {
            Self {
                items: Ok(value.items),
                name: Ok(value.name),
                path: Ok(value.path),
                schedule: Ok(value.schedule),
                stateless: Ok(value.stateless),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AppDataItemsItem {
        charts: Result<Vec<super::Chart>, String>,
        data: Result<serde_json::Map<String, serde_json::Value>, String>,
        html: Result<String, String>,
        id: Result<String, String>,
        notify: Result<Vec<super::Notification>, String>,
        updated: Result<chrono::DateTime<chrono::offset::Utc>, String>,
    }
    impl Default for AppDataItemsItem {
        fn default() -> Self {
            Self {
                charts: Ok(Default::default()),
                data: Ok(Default::default()),
                html: Err("no value supplied for html".to_string()),
                id: Err("no value supplied for id".to_string()),
                notify: Ok(Default::default()),
                updated: Err("no value supplied for updated".to_string()),
            }
        }
    }
    impl AppDataItemsItem {
        pub fn charts<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Chart>>,
            T::Error: std::fmt::Display,
        {
            self.charts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for charts: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn html<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.html = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for html: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn notify<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Notification>>,
            T::Error: std::fmt::Display,
        {
            self.notify = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for notify: {}", e));
            self
        }
        pub fn updated<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for updated: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AppDataItemsItem> for super::AppDataItemsItem {
        type Error = String;
        fn try_from(value: AppDataItemsItem) -> Result<Self, String> {
            Ok(Self {
                charts: value.charts?,
                data: value.data?,
                html: value.html?,
                id: value.id?,
                notify: value.notify?,
                updated: value.updated?,
            })
        }
    }
    impl From<super::AppDataItemsItem> for AppDataItemsItem {
        fn from(value: super::AppDataItemsItem) -> Self {
            Self {
                charts: Ok(value.charts),
                data: Ok(value.data),
                html: Ok(value.html),
                id: Ok(value.id),
                notify: Ok(value.notify),
                updated: Ok(value.updated),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AppDataScheduleItem {
        arguments: Result<Vec<String>, String>,
        cron: Result<Option<String>, String>,
    }
    impl Default for AppDataScheduleItem {
        fn default() -> Self {
            Self {
                arguments: Ok(Default::default()),
                cron: Ok(Default::default()),
            }
        }
    }
    impl AppDataScheduleItem {
        pub fn arguments<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.arguments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for arguments: {}", e));
            self
        }
        pub fn cron<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.cron = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cron: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AppDataScheduleItem> for super::AppDataScheduleItem {
        type Error = String;
        fn try_from(value: AppDataScheduleItem) -> Result<Self, String> {
            Ok(Self {
                arguments: value.arguments?,
                cron: value.cron?,
            })
        }
    }
    impl From<super::AppDataScheduleItem> for AppDataScheduleItem {
        fn from(value: super::AppDataScheduleItem) -> Self {
            Self {
                arguments: Ok(value.arguments),
                cron: Ok(value.cron),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Notification {
        html: Result<String, String>,
        icon: Result<Option<String>, String>,
        id: Result<String, String>,
    }
    impl Default for Notification {
        fn default() -> Self {
            Self {
                html: Err("no value supplied for html".to_string()),
                icon: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
            }
        }
    }
    impl Notification {
        pub fn html<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.html = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for html: {}", e));
            self
        }
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Notification> for super::Notification {
        type Error = String;
        fn try_from(value: Notification) -> Result<Self, String> {
            Ok(Self {
                html: value.html?,
                icon: value.icon?,
                id: value.id?,
            })
        }
    }
    impl From<super::Notification> for Notification {
        fn from(value: super::Notification) -> Self {
            Self {
                html: Ok(value.html),
                icon: Ok(value.icon),
                id: Ok(value.id),
            }
        }
    }
}
