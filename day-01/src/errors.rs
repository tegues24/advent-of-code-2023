use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("unknown error happened: {0}")]
    ErrorUnknown(String),
}