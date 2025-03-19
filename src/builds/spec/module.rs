#![forbid(unsafe_code)]

use std::path::Path;
use std::sync::Arc;

use crate::{config::constants::SYSTEM_FILES_EXTENSION, error::LPError};

use super::{sections::LiterateFile, structs::Module};

/// eliminate's the directory prefix from the path
fn clean_path(source_dir: &Path, path: &Path) -> std::path::PathBuf {
    if let Ok(stripped_path) = path.strip_prefix(source_dir) {
        stripped_path.to_path_buf()
    } else {
        path.to_path_buf()
    }
}

impl Module {
    pub fn new(source_dir: &Path, path: &Path) -> Result<Self, LPError> {
        if path.extension().and_then(|ext| ext.to_str()) != Some(SYSTEM_FILES_EXTENSION) {
            return Ok(Module {
                sections: None,
                path: clean_path(source_dir, path),
            });
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => {
                return Ok(Module {
                    sections: None,
                    path: clean_path(source_dir, path),
                })
            }
        };

        let literate_file = match LiterateFile::new(&content) {
            Ok(lf) => lf,
            Err(e) => return Err(e),
        };
        Ok(Module {
            sections: Some(literate_file.sections.into_iter().map(Arc::new).collect()),
            path: clean_path(source_dir, path),
        })
    }
}
