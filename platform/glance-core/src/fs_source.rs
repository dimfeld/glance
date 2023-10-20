use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use glance_app::{App, AppData, APP_DATA_SUBDIR};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};

pub struct FsSource {
    shutdown_tx: flume::Sender<()>,
    watcher: std::thread::JoinHandle<()>,
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
            watcher: watcher_thread,
        })
    }

    pub fn close(self) {
        let Self {
            shutdown_tx,
            watcher,
        } = self;

        drop(shutdown_tx);
        watcher.join().expect("Shutting down");
    }

    fn watcher(
        shutdown_rx: flume::Receiver<()>,
        data_dir: PathBuf,
        change_tx: flume::Sender<(String, AppData)>,
    ) {
        // Start the watcher

        let watcher_change_tx = change_tx.clone();
        let mut watcher = new_debouncer(
            Duration::from_millis(500),
            move |res: DebounceEventResult| match res {
                Ok(events) => {
                    for event in events {
                        println!("updated file: {}", event.path.display());
                        match read_file(&event.path) {
                            Ok(Some(result)) => {
                                watcher_change_tx.send(result).ok();
                            }
                            Ok(None) => {}
                            Err(e) => eprintln!("reading {}: {:?}", event.path.display(), e),
                        };
                    }
                }
                Err(e) => eprintln!("watcher error: {:?}", e),
            },
        )
        .expect("Creating watcher");

        watcher
            .watcher()
            .watch(&data_dir, RecursiveMode::NonRecursive)
            .expect("watching data directory");

        // Scan the directory and send change events for all the data.
        let dir = std::fs::read_dir(&data_dir)
            .expect("Reading data directory")
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

        // Block on the shutdown channel
        shutdown_rx.recv().ok();
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
