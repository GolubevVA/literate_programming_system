#![forbid(unsafe_code)]

use std::path::PathBuf;

use crate::config::constants::SYSTEM_FILES_EXTENSION;

// eliminates the spec's extension
pub fn prepare_module_file_extension(path: &PathBuf) -> PathBuf {
    let mut result = path.clone();
    if let Some(extension) = path.extension() {
        if extension == SYSTEM_FILES_EXTENSION {
            result.set_extension("");
        }
    }
    result
}
