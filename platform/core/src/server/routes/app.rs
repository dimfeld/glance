use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use axum_extra::extract::Query;
use glance_app::AppData;
use http::StatusCode;
use serde::Deserialize;

use super::ServerState;
use crate::{error::Error, AppFileContents, AppFileInput};

async fn get_app(
    Path(app_id): Path<String>,
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, Error> {
    let app = state
        .orm
        .get_apps(&[app_id])
        .await?
        .pop()
        .ok_or_else(|| Error::NotFound("App"))?;
    Ok(Json(app))
}

#[derive(Debug, Deserialize)]
struct UpdateQuery {
    merge: Option<bool>,
}

async fn update_app(
    Path(app_id): Path<String>,
    State(state): State<ServerState>,
    Query(query): Query<UpdateQuery>,
    Json(app): Json<Box<AppData>>,
) -> Result<impl IntoResponse, Error> {
    let app_data = AppFileInput {
        app_id,
        contents: AppFileContents::Parsed(app),
        merge_items: query.merge.unwrap_or(false),
    };

    state.change_tx.send_async(app_data).await.ok();
    Ok(StatusCode::ACCEPTED)
}

/// Create the app routes
pub fn routes() -> Router<ServerState> {
    Router::new()
        .route("/apps/:app_id", get(get_app))
        .route("/apps/:app_id", put(update_app))
}
