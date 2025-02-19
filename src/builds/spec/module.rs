#![forbid(unsafe_code)]

use std::path::Path;

use crate::config::constants::SYSTEM_FILES_EXTENSION;

use super::{sections::LiterateFile, structs::Module};

fn clean_path(source_dir: &Path, path: &Path) -> std::path::PathBuf {
    if let Ok(stripped_path) = path.strip_prefix(source_dir) {
        stripped_path.to_path_buf()
    } else {
        path.to_path_buf()
    }
}

impl Module {
    pub fn new(source_dir: &Path, path: &Path) -> Self {
        if path.extension().and_then(|ext| ext.to_str()) != Some(SYSTEM_FILES_EXTENSION) {
            return Module {
                sections: None,
                path: clean_path(source_dir, path),
            };
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => {
                return Module {
                    sections: None,
                    path: clean_path(source_dir, path),
                }
            }
        };

        Module {
            sections: Some(LiterateFile::new(&content).sections),
            path: clean_path(source_dir, path),
        }
    }
}
