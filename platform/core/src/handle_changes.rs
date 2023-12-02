use std::collections::HashMap;

use error_stack::{Report, ResultExt};
use glance_app::AppData;
use tracing::{event, instrument, Level};

use crate::{db::Db, error::Error, items::Item, AppFileContents, AppFileInput};

pub async fn handle_changes(db: Db, change_rx: flume::Receiver<AppFileInput>) {
    while let Ok(input) = change_rx.recv_async().await {
        handle_change_or_error(&db, input).await;
    }
}

#[instrument(skip(db, input), fields(app_id = %input.app_id, has_data = !input.contents.is_empty()))]
async fn handle_change_or_error(db: &Db, input: AppFileInput) {
    let AppFileInput { app_id, contents } = input;

    let result = match contents {
        AppFileContents::Raw(contents) => handle_raw_data(db, &app_id, &contents).await,
        AppFileContents::Parsed(data) => handle_change(db, &app_id, *data).await,
        AppFileContents::Empty => handle_remove(db, &app_id).await,
    };

    let result = result.attach_printable_lazy(|| format!("App ID: {}", app_id));

    if let Err(e) = result {
        let err_desc = format!("{e:?}");
        event!(Level::ERROR,  error = %err_desc , "Error handling app change");
        let err_result = db
            .update_app_status(&db.pool, &app_id, Some(&err_desc))
            .await;
        if let Err(e) = err_result {
            event!(Level::ERROR,  error = ?e , "Failed to record app error");
        }
    }
}

async fn handle_raw_data(db: &Db, app_id: &str, contents: &str) -> Result<(), Report<Error>> {
    let data = serde_json::from_str::<AppData>(contents).change_context(Error::ReadAppData)?;
    handle_change(db, app_id, data).await
}

async fn handle_change(db: &Db, app_id: &str, mut app: AppData) -> Result<(), Report<Error>> {
    let current_items = db
        .read_app_items(app_id)
        .await?
        .into_iter()
        .map(|item| (item.id.clone(), item))
        .collect::<HashMap<_, _>>();

    let item_ids = app
        .items
        .iter()
        .map(|item| item.id.clone())
        .collect::<Vec<_>>();

    let items = std::mem::replace(&mut app.items, vec![]);
    let changed_items = items.into_iter().map(|item| {
        let resurface = if let Some(current_item) = current_items.get(&item.id) {
            current_item.changed_from(&item)
        } else {
            true
        };

        let item = Item::from_app_item(app_id.to_string(), item);
        (item, resurface)
    });

    let mut tx = db.pool.begin().await.change_context(Error::Db)?;

    db.create_or_update_app(tx.as_mut(), app_id, &app).await?;

    for (item, resurface) in changed_items {
        db.create_or_update_item(tx.as_mut(), &item, resurface)
            .await?;
    }

    db.remove_unfound_items(tx.as_mut(), app_id, &item_ids)
        .await?;

    tx.commit().await.change_context(Error::Db)?;

    Ok(())
}

async fn handle_remove(db: &Db, app_id: &str) -> Result<(), Report<Error>> {
    db.remove_app(app_id).await
}
