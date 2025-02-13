#![forbid(unsafe_code)]

use std::path::Path;

use walkdir::WalkDir;

use super::structs::{Module, Project};

impl Project {
    pub fn new(source_dir: &Path) -> Self {
        let modules = WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| Module::new(source_dir, entry.path()))
            .collect();

        Project { modules }
    }
}
