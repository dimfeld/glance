use error_stack::{Report, ResultExt};
use glance_app::AppData;

use crate::{db::Db, error::Error, AppFileInput};

pub async fn handle_changes(db: Db, change_rx: flume::Receiver<AppFileInput>) {
    while let Ok(input) = change_rx.recv_async().await {
        handle_change_or_error(&db, input).await;
    }
}

async fn handle_change_or_error(db: &Db, input: AppFileInput) {
    let AppFileInput { app_id, contents } = input;

    let result = match contents {
        Some(contents) => handle_change(db, &app_id, &contents).await,
        None => handle_remove(db, &app_id).await,
    };

    let result = result.attach_printable(app_id);

    if let Err(e) = result {
        let err_desc = e.to_string();
        // TODO set error status for app
    }
}

async fn handle_change(db: &Db, app_id: &str, contents: &str) -> Result<(), Report<Error>> {
    let app = serde_json::from_str::<AppData>(contents).change_context(Error::ReadAppData)?;

    let current_items = db.read_app_items(app_id).await?;
    Ok(())
}

async fn handle_remove(db: &Db, app_id: &str) -> Result<(), Report<Error>> {
    todo!("remove everything about the app from the DB")
}
