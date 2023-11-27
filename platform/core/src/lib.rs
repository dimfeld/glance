#![warn(missing_docs)]
//! Glance platform core

use std::path::PathBuf;

use db::Db;
use glance_app::{App, AppData};

mod api;
/// Database implementation
pub mod db;
/// Define errors
pub mod error;
#[cfg(feature = "fs-source")]
mod fs_source;
mod handle_changes;
mod items;

/// An app data update
pub enum AppFileContents {
    /// There was no input, indicating that the app should be removed.
    Empty,
    /// The raw input before parsing. This should be used when the data is written and read
    /// asynchronously, as when the data is written to disk and detected via a watcher.
    Raw(String),
    /// The fully-parsed input. This can be used when the data is read synchronously and errors can
    /// be returned directly to the submitter, as when interacting through an HTTP interface.
    Parsed(Box<AppData>),
}

impl AppFileContents {
    /// Return true if there is no data
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

/// Input to the platform of an app's data, to be reconciled against the existing data.
pub struct AppFileInput {
    app_id: String,
    contents: AppFileContents,
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
    #[cfg(feature = "fs-source")]
    fs_source: fs_source::FsSource,
    change_handler: tokio::task::JoinHandle<()>,
    /// The database for the platform
    pub db: Db,
}

impl Platform {
    /// Create a new platform
    pub async fn new(config: PlatformOptions) -> Self {
        let base_dir = config.base_dir.unwrap_or_else(App::base_data_dir);
        std::fs::create_dir_all(&base_dir).expect("creating data directory");
        let (change_tx, change_rx) = flume::bounded(16);

        let db_url = config
            .database_url
            .unwrap_or_else(|| std::env::var("GLANCE_DATABASE_URL").unwrap_or_default());

        let db = Db::new(&db_url).await.expect("creating database");

        let change_handler =
            tokio::task::spawn(handle_changes::handle_changes(db.clone(), change_rx));

        Self {
            #[cfg(feature = "fs-source")]
            fs_source: fs_source::FsSource::new(base_dir, change_tx).expect("creating FsSource"),
            change_handler,
            db,
        }
    }

    /// Wait for everything to settle and then shut down.
    pub async fn shutdown(self) {
        let Self {
            fs_source,
            change_handler,
            ..
        } = self;
        tokio::task::spawn_blocking(|| fs_source.close()).await.ok();
        change_handler.await.ok();
    }
}
