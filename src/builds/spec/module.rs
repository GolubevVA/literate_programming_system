#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::rc::Rc;

use path_clean::clean;

use crate::{config::constants::SYSTEM_FILES_EXTENSION, error::LPError};

use super::utils::{clean_path, module_name};
use super::{sections::LiterateFile, structs::Module};

impl Module {
    /// Creates a new module instance.
    /// # Arguments
    /// * `source_dir` - a Path instance that points to the source directory.
    /// * `path` - a Path instance that points to the module file.
    pub fn new(source_dir: &Path, path: &Path) -> Result<Self, LPError> {
        let module_path = clean_path(source_dir, path);

        if path.extension().and_then(|ext| ext.to_str()) != Some(SYSTEM_FILES_EXTENSION) {
            return Ok(Module {
                sections: None,
                path: module_path,
            });
        }

        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => {
                return Ok(Module {
                    sections: None,
                    path: module_path,
                })
            }
        };

        let literate_file = LiterateFile::new(&content)?;

        Ok(Module {
            sections: Some(literate_file.sections.into_iter().map(Rc::new).collect()),
            path: module_path,
        })
    }

    /// Returns the path to the module which can be referred as a `path` from the current module
    ///
    /// E.g. if module.path is `dir/a.py.lpnb` and `path` is `../b.py.lpnb`, the result will be `b.py.lpnb`
    /// 
    /// In case of empty `path` argument, returns the module's name (not it's path!)
    pub fn resolve_relative_module_path(&self, path: &Path) -> PathBuf {
        let mut combined = self.path.clone();
        if path.to_str() == Some("") {
            return module_name(&combined);
        }
        combined.pop();
        combined.push(path);
        clean(combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_module_new_non_literate_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "some content").unwrap();

        let module = Module::new(dir.path(), &file_path).unwrap();
        assert!(module.sections.is_none());
        assert_eq!(module.path, Path::new("test.txt"));
    }

    #[test]
    fn test_module_new_literate_file() {
        let dir = tempdir().unwrap();
        let file_path = dir
            .path()
            .join(format!("test.rs.{}", SYSTEM_FILES_EXTENSION));

        let content = r#"
sections:
  - code: |
        fn hello() {}
    docs: |
        # Hello Function
        This function says hello.
"#;
        fs::write(&file_path, content).unwrap();

        let module = Module::new(dir.path(), &file_path).unwrap();
        assert!(module.sections.is_some());
        assert_eq!(module.sections.as_ref().unwrap().len(), 1);
        assert_eq!(
            module.path,
            PathBuf::from(format!("test.rs.{}", SYSTEM_FILES_EXTENSION))
        );
    }

    #[test]
    fn test_module_new_error_duplicate_headers() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(format!("test.{}", SYSTEM_FILES_EXTENSION));

        let content = r#"
sections:
  - code: |
        fn hello() {}
    docs: |
        # Duplicate Header
        This function says hello.
  - code: |
        fn world() {}
    docs: |
        # Duplicate Header
        This function says world.
"#;
        fs::write(&file_path, content).unwrap();

        let result = Module::new(dir.path(), &file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_relative_module_path_same_dir() {
        let module = Module {
            path: PathBuf::from(format!("dir/module.{}", SYSTEM_FILES_EXTENSION)),
            sections: None,
        };

        let formatted_path = format!("other.{}", SYSTEM_FILES_EXTENSION);
        let relative = Path::new(&formatted_path);
        let resolved = module.resolve_relative_module_path(relative);
        assert_eq!(
            resolved,
            PathBuf::from(format!("dir/other.{}", SYSTEM_FILES_EXTENSION))
        );
    }

    #[test]
    fn test_resolve_relative_module_path_parent_dir() {
        let module = Module {
            path: PathBuf::from(format!("dir/subdir/module.{}", SYSTEM_FILES_EXTENSION)),
            sections: None,
        };

        let relative = format!("../other.{}", SYSTEM_FILES_EXTENSION);
        let relative = Path::new(&relative);
        let resolved = module.resolve_relative_module_path(relative);
        assert_eq!(resolved, PathBuf::from("dir/other.lpnb"));
    }

    #[test]
    fn test_resolve_relative_module_path_empty() {
        let module = Module {
            path: PathBuf::from(format!("dir/module.py.{}", SYSTEM_FILES_EXTENSION)),
            sections: None,
        };

        let relative = Path::new("");
        let resolved = module.resolve_relative_module_path(relative);
        assert_eq!(resolved, PathBuf::from("dir/module"));
    }

    #[test]
    fn test_resolve_relative_module_path_subdirectory() {
        let module = Module {
            path: PathBuf::from(format!("dir/module.{}", SYSTEM_FILES_EXTENSION)),
            sections: None,
        };

        let relative = format!("subdir/other.{}", SYSTEM_FILES_EXTENSION);
        let relative = Path::new(&relative);
        let resolved = module.resolve_relative_module_path(relative);
        assert_eq!(
            resolved,
            PathBuf::from(format!("dir/subdir/other.{}", SYSTEM_FILES_EXTENSION))
        );
    }
}
