use std::string::FromUtf8Error;

use thiserror::Error;

/// Represents all the ways a method can fail within clipboard-stream.
#[derive(Debug, Error)]
#[error("error")]
pub enum Error {
    /// Error occurred while decode clipboard item as UTF-8
    #[error("failed to decode clipboard item as UTF-8: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
    /// Error occurred while system call
    #[error("failed to get item")]
    GetItem,
}
