#![forbid(unsafe_code)]

use std::path::PathBuf;

use thiserror::Error;

/// Errors that can occur during the execution of the build system.
#[derive(Error, Debug)]
pub enum LPError {
    /// IO related errors.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error when source directory does not exist
    #[error("Source directory not found: {0}")]
    SourceDirectoryNotFound(String),

    /// Error when duplicate headers found in a literate file
    #[error("Duplicate header found: {0}")]
    DuplicateHeader(String),

    /// Error when it's impossible to read the file
    #[error("Cannot read file: {0}")]
    CannotReadFile(String),

    /// Error when such plugin not found
    #[error("No plugin for files extension: {0}")]
    PluginNotFound(String),

    /// Error when an incorrect reference is found
    #[error("Impossible to refer to the module: {0}, section: {1}")]
    IncorrectReference(PathBuf, String),

    /// Lua runtime errors.
    #[error("Lua error: {0}")]
    LuaRuntime(String),
}
