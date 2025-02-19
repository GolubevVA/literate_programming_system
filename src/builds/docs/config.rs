#![forbid(unsafe_code)]

use std::path::PathBuf;

pub struct Config {
    pub target_docs_dir: PathBuf,
    pub source_dir: PathBuf,
}

impl Config {
    pub fn new(target_docs_dir: PathBuf, source_dir: PathBuf) -> Self {
        Self {
            target_docs_dir,
            source_dir,
        }
    }
}
