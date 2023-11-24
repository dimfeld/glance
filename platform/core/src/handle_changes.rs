use std::collections::HashMap;

use error_stack::{Report, ResultExt};
use glance_app::AppData;

use crate::{db::Db, error::Error, items::Item, AppFileInput};

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

    let current_items = db
        .read_app_items(app_id)
        .await?
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect::<HashMap<_, _>>();

    let mut changed_items = app
        .items
        .iter()
        .filter(|item| {
            if let Some(current_item) = current_items.get(&item.id) {
                if (app.stateful && current_item.equal_stateful(item))
                    || (!app.stateful && current_item.equal_stateless(item))
                {
                    return false;
                }
            }

            return true;
        })
        .map(|item| Item::from_app_item(app_id.to_string(), item.clone()));

    let mut tx = db.pool.begin().await.change_context(Error::Db)?;

    let mut changed_ids = Vec::new();
    for item in changed_items {
        changed_ids.push(item.id.clone());
        db.create_or_update_item(tx.as_mut(), &item).await?;
    }

    db.remove_unfound_items(tx.as_mut(), app_id, &changed_ids)
        .await?;

    tx.commit().await.change_context(Error::Db)?;

    Ok(())
}

async fn handle_remove(db: &Db, app_id: &str) -> Result<(), Report<Error>> {
    db.remove_app(app_id).await
}
