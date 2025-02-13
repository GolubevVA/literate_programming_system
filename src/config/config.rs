#![forbid(unsafe_code)]

use std::path::PathBuf;

use super::constants::{CODE_SUBDIR_NAME, DOCS_SUBDIR_NAME};

#[derive(Clone)]
pub struct Config {
    pub docs_dir: PathBuf,
    pub code_dir: PathBuf,
}

impl Config {
    pub fn new(target_dir: &PathBuf) -> Self {
        Config {
            docs_dir: target_dir.join(DOCS_SUBDIR_NAME),
            code_dir: target_dir.join(CODE_SUBDIR_NAME),
        }
    }
}
