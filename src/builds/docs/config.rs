#![forbid(unsafe_code)]

use std::path::PathBuf;

/// Configuration for the documentation build.
pub struct Config {
    pub target_docs_dir: PathBuf,
    pub source_dir: PathBuf,
}

impl Config {
    /// Create a new configuration.
    /// # Arguments
    /// * `target_docs_dir` - the directory where the documentation will be generated.
    /// * `source_dir` - the source project directory.
    pub fn new(target_docs_dir: PathBuf, source_dir: PathBuf) -> Self {
        Self {
            target_docs_dir,
            source_dir,
        }
    }
}
