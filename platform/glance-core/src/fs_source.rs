use std::{
    path::{Path, PathBuf},
    thread::JoinHandle,
    time::Duration,
};

use error_stack::{Report, ResultExt};
use glance_app::{App, AppData, APP_DATA_SUBDIR};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Watcher error")]
pub struct WatcherError {}

pub struct FsSource {
    // Hold a reference to keep things open, until this is dropped
    shutdown_tx: flume::Sender<()>,
    watcher_thread: JoinHandle<Result<(), Report<WatcherError>>>,
}

impl FsSource {
    pub fn new(
        base_dir: PathBuf,
        change_tx: flume::Sender<(String, AppData)>,
    ) -> Result<Self, std::io::Error> {
        let (shutdown_tx, shutdown_rx) = flume::bounded(0);
        let data_dir = base_dir.join(APP_DATA_SUBDIR);

        std::fs::create_dir_all(&data_dir)?;
        let watcher_thread =
            std::thread::spawn(move || Self::watcher(shutdown_rx, data_dir, change_tx));

        Ok(Self {
            shutdown_tx,
            watcher_thread,
        })
    }

    /// Close the FsSource and wait for the watcher thread to close.
    /// It is also ok to just drop the FsSource if you don't care about waiting for it to shutdown.
    pub fn close(self) {
        let Self {
            shutdown_tx,
            watcher_thread,
        } = self;
        drop(shutdown_tx);
        match watcher_thread.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => eprintln!("{:?}", e),
            Err(e) => eprintln!("Watcher thread panicked {:?}", e),
        };
    }

    fn watcher(
        shutdown_rx: flume::Receiver<()>,
        data_dir: PathBuf,
        change_tx: flume::Sender<(String, AppData)>,
    ) -> Result<(), Report<WatcherError>> {
        // Start the watcher
        let watcher_change_tx = change_tx.clone();
        let mut watcher = new_debouncer(
            Duration::from_millis(500),
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    for event in events {
                        println!("updated file: {}", event.path.display());
                        match read_file(&event.path)
                            .attach_printable_lazy(|| event.path.clone().display().to_string())
                        {
                            Ok(Some(result)) => {
                                watcher_change_tx.send(result).ok();
                            }
                            Ok(None) => {}
                            Err(e) => eprintln!("{:?}", e),
                        };
                    }
                }
                Err(e) => eprintln!("watcher error: {:?}", e),
            },
        )
        .change_context(WatcherError {})?;

        watcher
            .watcher()
            .watch(&data_dir, RecursiveMode::NonRecursive)
            .change_context(WatcherError {})?;

        // Scan the directory and send change events for all the data that was already there.
        let dir = std::fs::read_dir(&data_dir)
            .change_context(WatcherError {})?
            .filter_map(Result::ok);
        for entry in dir {
            let path = entry.path();
            match read_file(&path) {
                Ok(Some(result)) => {
                    change_tx.send(result).ok();
                }
                Ok(None) => {}
                Err(e) => eprintln!("reading {}: {:?}", path.display(), e),
            }
        }

        // Block on the shutdown channel while the watcher runs in the background.
        shutdown_rx.recv().ok();
        Ok(())
    }
}

fn read_file(path: &Path) -> Result<Option<(String, AppData)>, std::io::Error> {
    let Some(app_id) = path.file_stem() else {
        return Ok(None);
    };

    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                return Ok(None);
            } else {
                return Err(e);
            }
        }
    };

    let data: AppData = serde_json::from_reader(&file)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(Some((app_id.to_string_lossy().to_string(), data)))
}
