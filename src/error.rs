#![forbid(unsafe_code)]

use thiserror::Error;

/// Errors that can occur during the execution of the utility for literate programming
#[derive(Error, Debug)]
pub enum LPError {
    /// IO related errors.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error when source directory does not exist
    #[error("Source directory not found: {0}")]
    SourceDirectoryNotFound(String),

    // / Other errors.
    //#[error("{0}")]
    //Other(String),
}
