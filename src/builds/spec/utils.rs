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

/// Eliminate the spec's extension together with an extra extension if exists
///
/// So, it prepares the module's path to act as a reference's path.
///
/// E.g. `dir/a.py.lpnb` -> `dir/a`
pub fn module_name(module_path: &PathBuf) -> PathBuf {
    let mut path = prepare_module_file_extension(&module_path);
    if path.extension().is_some() {
        path.set_extension("");
    }
    path
}

/// Returns module's real extension
///
/// E.g. `dir/a.py.lpnb` -> `py` or `Dockerfile.lpnb` -> `Dockefile`
pub fn get_module_extension(module: &PathBuf) -> String {
    let without_main_extension = prepare_module_file_extension(module);
    without_main_extension
        .extension()
        .unwrap_or(without_main_extension.as_os_str())
        .to_string_lossy()
        .to_string()
}

/// eliminates the directory prefix from the path
pub fn clean_path(source_dir: &Path, path: &Path) -> PathBuf {
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
/// E.g.: "My Header" -> "my-header"
pub fn header_to_anchor(header: &str) -> String {
    header.trim().replace(' ', "-")
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_prepare_module_file_extension() {
        let path = PathBuf::from("example.lpnb");
        assert_eq!(prepare_module_file_extension(&path), PathBuf::from("example"));

        let path = PathBuf::from("script.py.lpnb");
        assert_eq!(prepare_module_file_extension(&path), PathBuf::from("script.py"));

        let path = PathBuf::from("regular_file.txt");
        assert_eq!(prepare_module_file_extension(&path), PathBuf::from("regular_file.txt"));
    }

    #[test]
    fn test_module_name() {
        let path = PathBuf::from("example.py.lpnb");
        assert_eq!(module_name(&path), PathBuf::from("example"));

        let path = PathBuf::from("Dockerfile.lpnb");
        assert_eq!(module_name(&path), PathBuf::from("Dockerfile"));

        let path = PathBuf::from("script.lpnb");
        assert_eq!(module_name(&path), PathBuf::from("script"));
    }

    #[test]
    fn test_get_module_extension() {
        let path = PathBuf::from("script.py.lpnb");
        assert_eq!(get_module_extension(&path), "py");

        let path = PathBuf::from("Dockerfile.lpnb");
        assert_eq!(get_module_extension(&path), "Dockerfile");

        let path = PathBuf::from("file.txt.lpnb");
        assert_eq!(get_module_extension(&path), "txt");
    }

    #[test]
    fn test_clean_path() {
        let source_dir = Path::new("/projects/myapp");
        let path = Path::new("/projects/myapp/src/main.rs");
        assert_eq!(clean_path(source_dir, path), PathBuf::from("src/main.rs"));

        let unrelated_path = Path::new("/other/location/file.txt");
        assert_eq!(clean_path(source_dir, unrelated_path), PathBuf::from("/other/location/file.txt"));
    }

    #[test]
    fn test_header_to_anchor() {
        assert_eq!(header_to_anchor("Introduction"), "Introduction");
        assert_eq!(header_to_anchor("Getting Started"), "Getting-Started");
        assert_eq!(header_to_anchor("  Multiple   Spaces  "), "Multiple---Spaces");
    }
}
