use std::borrow::Cow;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use error_stack::Report;
use serde::Serialize;
use thiserror::Error;
use tracing::{event, Level};

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

impl Error {
    pub fn error_kind(&self) -> &'static str {
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

    pub fn response_tuple(&self) -> (StatusCode, ErrorResponseData) {
        (
            self.status_code(),
            ErrorResponseData::new(self.error_kind(), self.to_string()),
        )
    }
}

impl From<Report<Error>> for Error {
    fn from(value: Report<Error>) -> Self {
        todo!()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (code, json) = self.response_tuple();
        (code, Json(json)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseData {
    error: ErrorDetails,
}

#[derive(Debug, Serialize)]
struct ErrorDetails {
    kind: Cow<'static, str>,
    message: Cow<'static, str>,
}

impl ErrorResponseData {
    pub fn new(
        kind: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> ErrorResponseData {
        let ret = ErrorResponseData {
            error: ErrorDetails {
                kind: kind.into(),
                message: message.into(),
            },
        };

        event!(Level::ERROR, kind=%ret.error.kind, message=%ret.error.message);

        ret
    }
}
