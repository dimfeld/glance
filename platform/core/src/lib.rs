#![warn(missing_docs)]
//! Glance platform core

use std::path::PathBuf;

use db::{Db, DbInner};
use error::Error;
use error_stack::{Report, ResultExt};
use glance_app::{App, AppData};
use scheduled_task::create_scheduled_task_runner;
use tracing::{event, Level};

/// Database implementation
pub mod db;
/// Define errors
pub mod error;
#[cfg(feature = "fs-source")]
mod fs_source;
mod handle_changes;
mod items;
mod scheduled_task;
/// The HTTP server
pub mod server;
/// Tracing setup
pub mod tracing_config;

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
    scheduled_task_runner: effectum::Worker,
}

impl Platform {
    /// Create a new platform
    pub async fn new(config: PlatformOptions) -> Result<Self, Report<Error>> {
        let base_dir = config.base_dir.unwrap_or_else(App::base_data_dir);
        std::fs::create_dir_all(&base_dir).expect("creating data directory");
        let (change_tx, change_rx) = flume::bounded(16);

        let db_url = config
            .database_url
            .unwrap_or_else(|| std::env::var("GLANCE_DATABASE_URL").unwrap_or_default());

        let db = DbInner::new(&db_url, &base_dir)
            .await
            .expect("creating database");
        let db = std::sync::Arc::new(db);

        let log_dir = base_dir.join("logs");
        std::fs::create_dir_all(&log_dir).expect("creating logs directory");
        let scheduled_task_runner = create_scheduled_task_runner(db.clone(), log_dir)
            .await
            .change_context(Error::TaskQueue)?;

        let change_handler =
            tokio::task::spawn(handle_changes::handle_changes(db.clone(), change_rx));

        Ok(Self {
            #[cfg(feature = "fs-source")]
            fs_source: fs_source::FsSource::new(base_dir, change_tx).expect("creating FsSource"),
            change_handler,
            db,
            scheduled_task_runner,
        })
    }

    /// Wait for everything to settle and then shut down.
    pub async fn shutdown(self) {
        let Self {
            fs_source,
            change_handler,
            db,
            scheduled_task_runner,
        } = self;
        event!(Level::DEBUG, "Shutting down fs source");
        #[cfg(feature = "fs-source")]
        tokio::task::spawn_blocking(|| fs_source.close()).await.ok();
        event!(Level::DEBUG, "Shutting down change handler");
        change_handler.await.ok();
        event!(Level::DEBUG, "Shutting down scheduled task runner");
        scheduled_task_runner
            .unregister(Some(std::time::Duration::from_secs(10)))
            .await
            .ok();
        db.task_queue
            .close(std::time::Duration::from_secs(10))
            .await
            .ok();
        event!(Level::DEBUG, "Shutting down database");
        db.pool.close().await;
    }
}
