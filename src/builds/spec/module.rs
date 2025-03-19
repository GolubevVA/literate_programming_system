#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::sync::Arc;

use path_clean::clean;

use crate::builds::spec::utils::module_name;
use crate::{config::constants::SYSTEM_FILES_EXTENSION, error::LPError};

use super::utils::clean_path;
use super::{sections::LiterateFile, structs::Module};

impl Module {
    pub fn new(source_dir: &Path, path: &Path) -> Result<Self, LPError> {
        let module_path = clean_path(source_dir, path);

        if path.extension().and_then(|ext| ext.to_str()) != Some(SYSTEM_FILES_EXTENSION) {
            return Ok(Module {
                sections: None,
                path: module_path,
            });
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => {
                return Ok(Module {
                    sections: None,
                    path: module_path,
                })
            }
        };

        let literate_file = match LiterateFile::new(&content) {
            Ok(lf) => lf,
            Err(e) => return Err(e),
        };

        Ok(Module {
            sections: Some(literate_file.sections.into_iter().map(Arc::new).collect()),
            path: module_path,
        })
    }

    /// Returns the path to the module which can be referred as a `path` from the current module
    ///
    /// E.g. if module.path is `dir/a.py.lpnb` and `path` is `../b.py.lpnb`, the result will be `b.py.lpnb`
    pub fn resolve_relative_module_path(&self, path: &Path) -> PathBuf {
        let mut combined = self.path.clone();
        if path.to_str() == Some("") {
            return module_name(&self.path);
        }
        combined.pop();
        combined.push(path);
        clean(combined)
    }
}
