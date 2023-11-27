use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;

use super::ServerState;
use crate::{db::Db, error::Error, server::error::HttpError};

async fn get_active_items(State(state): State<ServerState>) -> Result<impl IntoResponse, Error> {
    let items = state.db.read_active_items().await?;
    Ok(Json(items))
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/apps/:app_id/items/:item_id/dismiss")]
struct DismissItemPath {
    app_id: String,
    item_id: String,
}

async fn dismiss_item(
    DismissItemPath { app_id, item_id }: DismissItemPath,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, Error> {
    state.db.set_item_dismissed(&app_id, &item_id, true).await?;
    Ok(())
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/apps/:app_id/items/:item_id/undismiss")]
struct UndismissItemPath {
    app_id: String,
    item_id: String,
}

async fn undismiss_item(
    UndismissItemPath { app_id, item_id }: UndismissItemPath,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, Error> {
    state
        .db
        .set_item_dismissed(&app_id, &item_id, false)
        .await?;
    Ok(())
}

pub fn routes() -> Router<ServerState> {
    Router::new()
        .route("/active_items", get(get_active_items))
        .typed_post(dismiss_item)
        .typed_post(undismiss_item)
}
