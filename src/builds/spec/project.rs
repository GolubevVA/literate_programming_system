#![forbid(unsafe_code)]

use std::path::Path;
use std::sync::Arc;

use walkdir::WalkDir;

use crate::error::LPError;

use super::structs::{Module, Project};

impl Project {
    /// Creates a new Project instance.
    pub fn new(source_dir: &Path) -> Result<Self, LPError> {
        let modules = WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| Module::new(source_dir, entry.path()).map(Arc::new))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Project { modules })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::constants::SYSTEM_FILES_EXTENSION;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_project_new_empty_dir() {
        let dir = tempdir().unwrap();
        let project = Project::new(dir.path()).unwrap();
        assert!(project.modules.is_empty());
    }

    #[test]
    fn test_project_new_non_literate_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("file1.txt"), "content").unwrap();
        fs::write(dir.path().join("file2.rs"), "fn main() {}").unwrap();

        let project = Project::new(dir.path()).unwrap();
        assert_eq!(project.modules.len(), 2);
        for module in &project.modules {
            assert!(module.sections.is_none());
        }
    }

    #[test]
    fn test_project_new_literate_files() {
        let dir = tempdir().unwrap();
        let content = r#"
sections:
  - code: |
        fn hello() {}
    docs: |
        # Hello Function
        This function says hello.
"#;
        fs::write(
            dir.path()
                .join(format!("test.rs.{}", SYSTEM_FILES_EXTENSION)),
            content,
        )
        .unwrap();

        let project = Project::new(dir.path()).unwrap();
        assert_eq!(project.modules.len(), 1);
        let module = &project.modules[0];
        assert!(module.sections.is_some());
        assert_eq!(module.sections.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_project_new_mixed_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("file.txt"), "content").unwrap();

        let content = r#"
sections:
  - code: |
        fn hello() {}
    docs: |
        # Hello Function
        This function says hello.
"#;
        fs::write(
            dir.path()
                .join(format!("test.rs.{}", SYSTEM_FILES_EXTENSION)),
            content,
        )
        .unwrap();

        let project = Project::new(dir.path()).unwrap();
        assert_eq!(project.modules.len(), 2);

        let with_sections = project
            .modules
            .iter()
            .filter(|m| m.sections.is_some())
            .count();
        let without_sections = project
            .modules
            .iter()
            .filter(|m| m.sections.is_none())
            .count();

        assert_eq!(with_sections, 1);
        assert_eq!(without_sections, 1);
    }

    #[test]
    fn test_project_new_nested_directories() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("subdir")).unwrap();
        fs::create_dir_all(dir.path().join("another/nested")).unwrap();

        fs::write(dir.path().join("file.txt"), "content").unwrap();
        fs::write(dir.path().join("subdir/file.txt"), "content").unwrap();
        fs::write(dir.path().join("another/nested/file.txt"), "content").unwrap();

        let project = Project::new(dir.path()).unwrap();
        assert_eq!(project.modules.len(), 3);
    }

    #[test]
    fn test_project_new_error_propagation() {
        let dir = tempdir().unwrap();
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
        fs::write(
            dir.path().join(format!("test.{}", SYSTEM_FILES_EXTENSION)),
            content,
        )
        .unwrap();

        let result = Project::new(dir.path());
        assert!(result.is_err());
        match result {
            Err(LPError::DuplicateHeader(_)) => {}
            _ => panic!("Expected DuplicateHeader error"),
        }
    }
}
