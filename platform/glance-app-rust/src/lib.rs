#![warn(missing_docs)]
//! Types for mini-apps that are built for the Glance dashboard platform

// app_data is autogenerated so won't have all docs, but many are generated from the JSON schema so
// it's not too bad
#[allow(missing_docs)]
mod app_data;

use std::path::PathBuf;

pub use app_data::*;

/// Common logic useful for mini-apps
pub struct App {
    /// The name of the application
    pub app_id: String,
}

impl App {
    /// Create a new `Paths` for the given application
    pub fn new(app_id: String) -> Self {
        Self { app_id }
    }

    /// The base data directory for the Glance platform
    pub fn base_data_dir() -> PathBuf {
        [
            dirs::data_local_dir().unwrap().to_string_lossy().as_ref(),
            "glance-dashboards",
            #[cfg(os = "windows")]
            "Data",
        ]
        .iter()
        .collect()
    }

    /// The directory that holds the app data files
    pub fn data_dir() -> PathBuf {
        Self::base_data_dir().join("app_data")
    }

    /// The directory that holds temporary data when it needs to be on the same fileysystem.
    pub fn tmp_data_dir() -> PathBuf {
        Self::base_data_dir().join("tmp")
    }

    /// The JSON file that the app should write to
    pub fn data_file(&self) -> PathBuf {
        Self::data_dir().join(format!("{}.json", self.app_id))
    }

    /// A directory that the app can optionally use to store its internal state.
    pub fn state_dir(&self) -> PathBuf {
        Self::base_data_dir().join("app_state")
    }

    /// Write the app data to the appropridate location
    pub fn write_data(&self, data: AppData) -> Result<(), std::io::Error> {
        let tmp_path = Self::tmp_data_dir().join(format!(
            "{}-{}.json",
            self.app_id,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        ));

        std::fs::write(
            &tmp_path,
            serde_json::to_string(&data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
        )?;

        let final_path = self.data_file();
        std::fs::rename(&tmp_path, &final_path)?;

        Ok(())
    }
}
