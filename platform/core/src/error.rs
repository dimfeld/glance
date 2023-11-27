use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use error_stack::Report;
use thiserror::Error;

use crate::server::error::HttpError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to intialize database")]
    DbInit,
    #[error("Database error")]
    Db,
    #[error("Failed to read app data")]
    ReadAppData,
    #[error("Failed to start server")]
    Server,
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
            Error::DbInit => "db_init",
            Error::Db => "db",
            Error::ReadAppData => "read_app_data",
            Error::Server => "server",
            Error::NotFound(_) => "not_found",
            Error::WrapReport(e) => e.current_context().error_kind(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::DbInit => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Db => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ReadAppData => StatusCode::BAD_REQUEST,
            Error::Server => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::WrapReport(e) => e.current_context().status_code(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.to_response()
    }
}
