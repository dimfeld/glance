use error_stack::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to intialize database")]
    DbInit,
}
