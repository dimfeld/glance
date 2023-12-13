use std::{path::PathBuf, sync::Arc};

use effectum::RunningJob;
use error_stack::{Report, ResultExt};
use glance_app::AppSchedule;
use serde::{Deserialize, Serialize};
use tracing::{event, Level};

use crate::{db::Db, error::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduledJobData {
    pub app_id: String,
    pub command: String,
    pub schedule: AppSchedule,
}

#[derive(Debug)]
pub struct ScheduledJobContext {
    log_dir: PathBuf,
    db: Db,
}

pub async fn create_scheduled_task_runner(
    db: Db,
    log_dir: PathBuf,
) -> Result<effectum::Worker, effectum::Error> {
    let schedule_runner = effectum::JobRunner::builder("scheduled-app", run_scheduled_app).build();
    effectum::Worker::builder(
        &db.task_queue,
        Arc::new(ScheduledJobContext {
            db: db.clone(),
            log_dir,
        }),
    )
    .max_concurrency(
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4) as u16,
    )
    .jobs([schedule_runner])
    .build()
    .await
}

async fn run_scheduled_app(
    job: RunningJob,
    context: Arc<ScheduledJobContext>,
) -> Result<(), Report<Error>> {
    let data: ScheduledJobData = job.json_payload().change_context(Error::ScheduledTask)?;

    event!(Level::INFO, cmd=%data.command, args=?data.schedule.arguments, "Running scheduled job");

    let stdout_fs_path = context.log_dir.join(format!("{}.stdout.log", data.app_id));
    let stdout_fs = std::fs::File::create(&stdout_fs_path)
        .change_context(Error::ScheduledTask)
        .attach_printable_lazy(|| format!("Creating {}", stdout_fs_path.display()))?;
    let stderr_fs_path = context.log_dir.join(format!("{}.stderr.log", data.app_id));
    let stderr_fs = std::fs::File::create(&stderr_fs_path)
        .change_context(Error::ScheduledTask)
        .attach_printable_lazy(|| format!("Creating {}", stderr_fs_path.display()))?;

    let mut cmd = tokio::process::Command::new(&data.command)
        .args(&data.schedule.arguments)
        .stdout(std::process::Stdio::from(stdout_fs))
        .stderr(std::process::Stdio::from(stderr_fs))
        .kill_on_drop(true)
        .spawn()
        .change_context(Error::ScheduledTask)?;

    let timeout = job.expires.load(std::sync::atomic::Ordering::Relaxed);
    let timeout = chrono::DateTime::from_timestamp(timeout, 0).unwrap();
    let duration = timeout - chrono::Utc::now();

    let res = tokio::time::timeout(duration.to_std().unwrap(), cmd.wait())
        .await
        .map_err(|_| Error::ScheduledTask)
        .attach_printable("Task timed out")?
        .change_context(Error::ScheduledTask)?;

    if !res.success() {
        return Err(Report::new(Error::ScheduledTask))
            .attach_printable(format!("Command failed with {}", res));
    }

    Ok(())
}
