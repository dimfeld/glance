use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error_stack::Report;
use thiserror::Error;

use crate::{server::error::HttpError, tracing_config};

/// The top-level error type from the platform
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to intialize database
    #[error("Failed to intialize database")]
    DbInit,
    /// Database error not otherwise handled
    #[error("Database error")]
    Db,
    /// Failure deserializing an app data file
    #[error("Failed to read app data")]
    ReadAppData,
    /// Failed to start the HTTP server
    #[error("Failed to start server")]
    ServerStart,
    /// Failure while shutting down
    #[error("Encountered error while shutting down")]
    Shutdown,
    /// The requested item was not found
    #[error("{0} not found")]
    NotFound(&'static str),
    /// A wrapper around a Report<Error> to let it be returned from an Axum handler
    #[error("{0}")]
    WrapReport(Report<Error>),
}

impl From<Report<Error>> for Error {
    fn from(value: Report<Error>) -> Self {
        Error::WrapReport(value)
    }
}

impl crate::server::error::HttpError for Error {
    fn error_kind(&self) -> &'static str {
        match self {
            Error::WrapReport(e) => e.current_context().error_kind(),
            Error::DbInit => "db_init",
            Error::Db => "db",
            Error::ReadAppData => "read_app_data",
            Error::ServerStart => "server",
            Error::NotFound(_) => "not_found",
            Error::Shutdown => "shutdown",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::WrapReport(e) => e.current_context().status_code(),
            Error::DbInit => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Db => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ReadAppData => StatusCode::BAD_REQUEST,
            Error::ServerStart => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Shutdown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.to_response()
    }
}
