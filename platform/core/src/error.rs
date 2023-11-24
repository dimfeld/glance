use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to intialize database")]
    DbInit,
    #[error("Database error")]
    Db,
    #[error("Failed to read app data")]
    ReadAppData,
    #[error("Failed to build {0}")]
    Builder(&'static str),
}
