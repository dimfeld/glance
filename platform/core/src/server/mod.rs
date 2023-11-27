#[macro_use]
pub mod error;
mod obfuscate_errors;
mod panic_handler;
mod routes;

use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use axum::{routing::IntoMakeService, Router};
use error_stack::{Report, ResultExt};
use hyper::server::conn::AddrIncoming;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::{event, Level};

use crate::{db::Db, error::Error};

/// Shared state used by the server
pub struct InnerState {
    /// If the app is running in production mode. This should be used sparingly as there should be
    /// a minimum of difference between production and development.
    production: bool,
    db: Db,
}

pub(super) type ServerState = Arc<InnerState>;

/// The server and related information
pub struct Server {
    /// The host the server is bound to
    pub host: String,
    /// The port the server is bound to
    pub port: u16,
    /// The server. Await this to actually start running.
    pub server: axum::Server<AddrIncoming, IntoMakeService<Router>>,
    /// The server state.
    pub state: Arc<InnerState>,
}

impl Server {
    /// Run the server and wait for everything to close down once the server finishes.
    pub async fn run(self) -> Result<(), Report<Error>> {
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

        tokio::task::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for ctrl+c");
            shutdown_tx.send(()).ok();
        });

        self.run_with_shutdown_signal(shutdown_rx).await
    }

    /// Run the server, and shut it down when `shutdown_rx` closes.
    pub async fn run_with_shutdown_signal<T>(
        self,
        shutdown_rx: impl Future<Output = T> + Send + 'static,
    ) -> Result<(), Report<Error>> {
        let (internal_shutdown_tx, internal_shutdown_rx) = tokio::sync::oneshot::channel();

        tokio::task::spawn(async move {
            shutdown_rx.await;
            internal_shutdown_tx.send(()).ok();
        });

        self.server
            .with_graceful_shutdown(async move {
                internal_shutdown_rx.await.ok();
                event!(Level::INFO, "Shutting down server");
            })
            .await
            .change_context(Error::ServerStart)?;

        Ok(())
    }
}

/// Configuration for the server
pub struct Config<'a> {
    /// The environment we're running in. Currently this just distinguishes between
    /// "development" and any other value.
    pub env: &'a str,
    /// The host to bind to.
    pub host: String,
    /// The port to bind to
    pub port: u16,
    /// The database from the Platform
    pub db: Db,
}

/// Create the server and return it, ready to run.
pub async fn create_server(config: Config<'_>) -> Result<Server, Report<Error>> {
    let production = config.env != "development" && !cfg!(debug_assertions);

    let state = Arc::new(InnerState {
        production,
        db: config.db,
    });

    let app: Router<ServerState> = Router::new().merge(routes::items::routes()).layer(
        ServiceBuilder::new()
            .layer(CatchPanicLayer::custom(move |err| {
                panic_handler::handle_panic(production, err)
            }))
            .layer(obfuscate_errors::ObfuscateErrorLayer::new(
                production, false,
            ))
            .decompression()
            .compression()
            // .layer(tower_cookies::CookieManagerLayer::new())
            .set_x_request_id(MakeRequestUuid)
            .propagate_x_request_id()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::INFO)),
            )
            .into_inner(),
    );

    let app: Router<()> = app.with_state(state.clone());

    let bind_ip = config
        .host
        .parse::<IpAddr>()
        .change_context(Error::ServerStart)?;
    let bind_addr = SocketAddr::from((bind_ip, config.port));
    let builder = axum::Server::bind(&bind_addr);

    let server = builder.serve(app.into_make_service());
    let actual_addr = server.local_addr();
    let port = actual_addr.port();
    event!(Level::INFO, "Listening on {}:{port}", config.host);

    Ok(Server {
        host: config.host,
        port,
        server,
        state,
    })
}
