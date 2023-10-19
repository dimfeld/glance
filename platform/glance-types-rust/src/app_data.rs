#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppData {
    pub app: AppDataApp,
    pub items: Vec<AppDataItemsItem>,
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
pub struct AppDataApp {
    pub name: String,
    #[doc = "If true, the app does not keep its own state, so the platform should do a closer diff to see if an item has changed since the last write"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stateless: Option<bool>,
}
impl From<&AppDataApp> for AppDataApp {
    fn from(value: &AppDataApp) -> Self {
        value.clone()
    }
}
impl AppDataApp {
    pub fn builder() -> builder::AppDataApp {
        builder::AppDataApp::default()
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
        app: Result<super::AppDataApp, String>,
        items: Result<Vec<super::AppDataItemsItem>, String>,
    }
    impl Default for AppData {
        fn default() -> Self {
            Self {
                app: Err("no value supplied for app".to_string()),
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl AppData {
        pub fn app<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AppDataApp>,
            T::Error: std::fmt::Display,
        {
            self.app = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for app: {}", e));
            self
        }
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
    }
    impl std::convert::TryFrom<AppData> for super::AppData {
        type Error = String;
        fn try_from(value: AppData) -> Result<Self, String> {
            Ok(Self {
                app: value.app?,
                items: value.items?,
            })
        }
    }
    impl From<super::AppData> for AppData {
        fn from(value: super::AppData) -> Self {
            Self {
                app: Ok(value.app),
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AppDataApp {
        name: Result<String, String>,
        stateless: Result<Option<bool>, String>,
    }
    impl Default for AppDataApp {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                stateless: Ok(Default::default()),
            }
        }
    }
    impl AppDataApp {
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
    impl std::convert::TryFrom<AppDataApp> for super::AppDataApp {
        type Error = String;
        fn try_from(value: AppDataApp) -> Result<Self, String> {
            Ok(Self {
                name: value.name?,
                stateless: value.stateless?,
            })
        }
    }
    impl From<super::AppDataApp> for AppDataApp {
        fn from(value: super::AppDataApp) -> Self {
            Self {
                name: Ok(value.name),
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
