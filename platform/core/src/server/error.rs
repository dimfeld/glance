use std::borrow::Cow;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tracing::{event, Level};

pub trait HttpError: ToString {
    fn status_code(&self) -> StatusCode;
    fn error_kind(&self) -> &'static str;
    fn response_tuple(&self) -> (StatusCode, ErrorResponseData) {
        (
            self.status_code(),
            ErrorResponseData::new(self.error_kind(), self.to_string()),
        )
    }

    fn to_response(&self) -> Response {
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
