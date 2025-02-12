#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::config::config::Config;

pub struct Builder {
    pub source_dir: PathBuf,
    pub config: Config,
}

impl Builder {
    pub fn new(source_dir: PathBuf, config: Config) -> Self {
        Builder {
            source_dir,
            config,
        }
    }

    pub fn build(&self) {
        println!("Building from source directory: {:?}", self.source_dir);
        println!("Bulding documentation to: {:?}", self.config.docs_dir);
        println!("Bulding code to: {:?}", self.config.code_dir);
    }
}
