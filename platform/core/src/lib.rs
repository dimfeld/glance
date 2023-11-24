#![warn(missing_docs)]
//! Glance platform core

use std::path::PathBuf;

use db::Db;
use glance_app::App;

/// Database implementation
pub mod db;
/// Define errors
pub mod error;
#[cfg(feature = "fs-source")]
mod fs_source;
mod handle_changes;
mod items;

pub struct AppFileInput {
    app_id: String,
    /// The raw contents of the file. We don't serialize upon reading so that serialization error handling
    /// can be done by the change handler.
    /// If this is None, it means the app file was removed.
    contents: Option<String>,
}

/// Configuration for the platform
#[derive(Default)]
pub struct PlatformOptions {
    /// Override the data directory
    pub base_dir: Option<PathBuf>,
    /// Override the database URL
    /// Defaults to reading DATABASE_URL from the environment
    pub database_url: Option<String>,
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
    pub async fn new(config: PlatformOptions) -> Self {
        let base_dir = config.base_dir.unwrap_or_else(App::base_data_dir);
        let (change_tx, change_rx) = flume::bounded(16);

        let db_url = config
            .database_url
            .unwrap_or_else(|| std::env::var("DATABASE_URL").unwrap_or_default());

        let db = Db::new(&db_url).await.expect("creating database");

        let handler_task =
            tokio::task::spawn(handle_changes::handle_changes(db.clone(), change_rx));

        Self {
            #[cfg(feature = "fs-source")]
            fs_source: fs_source::FsSource::new(base_dir.clone(), change_tx)
                .expect("creating FsSource"),
            db,
            base_dir,
        }
    }
}
