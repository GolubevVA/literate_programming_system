#![forbid(unsafe_code)]

use std::path::PathBuf;

pub struct Config {
    pub target_code_dir: PathBuf,
    pub source_dir: PathBuf,
}

impl Config {
    pub fn new(target_code_dir: PathBuf, source_dir: PathBuf) -> Self {
        Config {
            target_code_dir,
            source_dir,
        }
    }
}
