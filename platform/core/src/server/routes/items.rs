use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::{
    extract::Query,
    routing::{RouterExt, TypedPath},
};
use glance_app::AppItem;
use http::StatusCode;
use serde::Deserialize;

use super::ServerState;
use crate::{error::Error, items::Item};

async fn get_active_items(State(state): State<ServerState>) -> Result<impl IntoResponse, Error> {
    let items = state.orm.read_active_items().await?;
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
    state
        .orm
        .set_item_dismissed(&app_id, &item_id, true)
        .await?;
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
        .orm
        .set_item_dismissed(&app_id, &item_id, false)
        .await?;
    Ok(())
}

#[derive(Deserialize)]
struct PostItemQuery {
    /// If the item already exists and has been previously dismissed, undismiss it.
    resurface: Option<bool>,
}

async fn post_item(
    State(state): State<ServerState>,
    Path(app_id): Path<String>,
    Query(query): Query<PostItemQuery>,
    Json(data): Json<AppItem>,
) -> Result<impl IntoResponse, Error> {
    let item = Item::from_app_item(app_id, data);
    state
        .orm
        .create_or_update_item(&state.db, &item, query.resurface.unwrap_or(false))
        .await?;

    Ok(StatusCode::OK)
}

pub fn routes() -> Router<ServerState> {
    Router::new()
        .route("/active_items", get(get_active_items))
        .route("/apps/:app_id/item", post(post_item))
        .typed_post(dismiss_item)
        .typed_post(undismiss_item)
}
