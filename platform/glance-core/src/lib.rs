#![warn(missing_docs)]
//! Glance platform core

use std::path::PathBuf;

use db::Db;
use glance_app::App;

pub mod db;
pub mod error;
#[cfg(feature = "fs-source")]
mod fs_source;
mod items;

/// Configuration for the platform
#[derive(Default)]
pub struct PlatformOptions {
    /// Override the data directory
    pub base_dir: Option<PathBuf>,
}

/// The platform data
pub struct Platform {
    base_dir: PathBuf,
    #[cfg(feature = "fs-source")]
    fs_source: fs_source::FsSource,
    db: Db,
}

impl Platform {
    /// Create a new platform
    pub fn new(config: PlatformOptions) -> Self {
        let base_dir = config.base_dir.unwrap_or_else(App::base_data_dir);
        let (change_tx, change_rx) = flume::bounded(16);

        // TODO run migrations

        Self {
            #[cfg(feature = "fs-source")]
            fs_source: fs_source::FsSource::new(base_dir.clone(), change_tx)
                .expect("creating FsSource"),
            db: Db::new(&base_dir.join("glance.sqlite")).expect("creating database"),
            base_dir,
        }
    }
}
