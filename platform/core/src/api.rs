use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json,
};
use error_stack::{Report, ResultExt};

use crate::{db::Db, error::Error};

#[derive(Clone)]
struct ServerState {
    db: Db,
}

async fn get_active_items(State(state): State<ServerState>) -> Result<impl IntoResponse, Error> {
    let items = state.db.read_active_items().await?;
    Ok(Json(items))
}

async fn dismiss_item(
    State(state): State<ServerState>,
    Path((app_id, item_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, Error> {
    state.db.set_item_dismissed(&app_id, &item_id, true).await?;
    Ok(())
}

async fn undismiss_item(
    State(state): State<ServerState>,
    Path((app_id, item_id)): Path<(String, String)>,
) -> Result<impl IntoResponse, Error> {
    state
        .db
        .set_item_dismissed(&app_id, &item_id, false)
        .await?;
    Ok(())
}

pub async fn start_server(bind: &str, db: Db) -> Result<(), Report<Error>> {
    let state = ServerState { db };
    let router = axum::Router::new()
        .route("/active_items", get(get_active_items))
        .route("/apps/:app_id/item/:item_id/dismiss", post(dismiss_item))
        .route(
            "/apps/:app_id/item/:item_id/undismiss",
            post(undismiss_item),
        )
        .with_state(state);

    axum::Server::bind(&bind.parse::<SocketAddr>().change_context(Error::Server)?)
        .serve(router.into_make_service())
        .await
        .change_context(Error::Server)
}
