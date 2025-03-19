#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use crate::config::constants::SYSTEM_FILES_EXTENSION;

/// eliminates the spec's extension
pub fn prepare_module_file_extension(path: &PathBuf) -> PathBuf {
    let mut result = path.clone();
    if let Some(extension) = path.extension() {
        if extension == SYSTEM_FILES_EXTENSION {
            result.set_extension("");
        }
    }
    result
}

/// eliminates the directory prefix from the path
pub fn clean_path(source_dir: &Path, path: &Path) -> std::path::PathBuf {
    if let Ok(stripped_path) = path.strip_prefix(source_dir) {
        stripped_path.to_path_buf()
    } else {
        path.to_path_buf()
    }
}

/// Convert a header text to its anchor representation
/// 1. Trim whitespace
/// 2. Replace spaces with hyphens
///
/// For example: "My Header" -> "my-header"
pub fn header_to_anchor(header: &str) -> String {
    header.trim().replace(' ', "-")
}
