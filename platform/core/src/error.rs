use error_stack::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to intialize database")]
    DbInit,
    #[error("Database error")]
    Db,
    #[error("Query error {0}")]
    DbQueryError(#[from] sqlx::Error),
    // /// Wrap a Report<Error> in another Error. Helpful for certain cases where
    // /// we need to be able to return a raw Error, such as in query_and_then.
    // #[error("{0:?}")]
    // ErrorReport(Report<Error>),
    // #[error("Failed getting column {0}")]
    // DbColumn(&'static str),
    #[error("Failed to build {0}")]
    Builder(&'static str),
}

impl From<Report<Error>> for Error {
    fn from(value: Report<Error>) -> Self {
        Self::ErrorReport(value)
    }
}
