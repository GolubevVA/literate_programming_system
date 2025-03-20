#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use super::constants::{CODE_SUBDIR_NAME, DOCS_SUBDIR_NAME};

#[derive(Clone)]
pub struct Config {
    /// the directory where the documentation will be generated
    pub docs_dir: PathBuf,
    /// the directory where the code will be generated
    pub code_dir: PathBuf,
    /// the directory where the code plugins are located
    pub code_plugins_dir: PathBuf,
    /// the source project directory
    pub source_dir: PathBuf,
    /// clear the target directory before building
    pub force: bool,
}

impl Config {
    pub fn new(target_dir: &Path, source_dir: &Path, code_plugins_dir: &Path, force: bool) -> Self {
        Config {
            docs_dir: target_dir.join(DOCS_SUBDIR_NAME),
            code_dir: target_dir.join(CODE_SUBDIR_NAME),
            code_plugins_dir: code_plugins_dir.to_path_buf(),
            source_dir: source_dir.to_path_buf(),
            force,
        }
    }
}
