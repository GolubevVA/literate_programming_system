#![forbid(unsafe_code)]

use std::{path::PathBuf, rc::Rc};

use mockall::predicate::str;

use crate::{
    builds::{
        docs::config::Config,
        spec::{
            structs::{Project, Section},
            utils,
        },
    },
    error::LPError,
};

/// DocsBuilder is a struct that is responsible for building the documentation from the source project.
pub struct DocsBuilder {
    config: Config,
    project: Rc<Project>,
}

impl DocsBuilder {
    /// Creates a new DocsBuilder instance.
    pub fn new(config: Config, project: Rc<Project>) -> Self {
        Self { config, project }
    }

    /// returns target path and an original extension, "" if no extension
    fn prepare_target_path(&self, path: &PathBuf) -> (PathBuf, String) {
        let mut result = self.config.target_docs_dir.clone();
        result.push(path);
        let cleaned_res = utils::prepare_module_file_extension(&result);
        let extension = cleaned_res
            .extension()
            .unwrap_or(std::ffi::OsStr::new(""))
            .to_str()
            .unwrap_or("")
            .to_string();
        if cleaned_res != result {
            (cleaned_res.with_extension("md"), extension)
        } else {
            (cleaned_res, extension)
        }
    }

    fn get_module_source_path(&self, module: &PathBuf) -> PathBuf {
        let mut result = self.config.source_dir.clone();
        result.push(module);
        result
    }

    fn prepare_final_docs(&self, sections: &[Rc<Section>], extension: &str) -> String {
        sections
            .iter()
            .map(|s| format!("{}\n```{}\n{}\n```", s.docs, extension, s.code))
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// The main method of the DocsBuilder that builds the documentation.
    /// It prepares the markdown files from the source project and writes them to the target directory.
    /// If the module has no sections, it just copies the source file to the target directory.
    pub fn build(&self) -> Result<(), LPError> {
        for module in &self.project.modules {
            let source_path = self.get_module_source_path(&module.path);
            let (target_path, extension) = self.prepare_target_path(&module.path);
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            if let Some(sections) = &module.sections {
                std::fs::write(
                    target_path,
                    self.prepare_final_docs(sections, extension.as_str()),
                )?;
            } else {
                std::fs::copy(source_path, target_path)?;
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::config::constants::SYSTEM_FILES_EXTENSION;

    use super::*;

    #[test]
    fn test_prepare_target_path_with_extension() {
        let config = Config::new(PathBuf::from("/target"), PathBuf::from("/source"));
        let project = Rc::new(Project { modules: vec![] });
        let builder = DocsBuilder::new(config, project);

        let path = PathBuf::from(format!("module.rs.{}", SYSTEM_FILES_EXTENSION));
        let (target_path, extension) = builder.prepare_target_path(&path);

        assert_eq!(target_path, PathBuf::from("/target/module.md"));
        assert_eq!(extension, "rs");
    }

    #[test]
    fn test_prepare_target_path_without_extension() {
        let config = Config::new(PathBuf::from("/target"), PathBuf::from("/source"));
        let project = Rc::new(Project { modules: vec![] });
        let builder = DocsBuilder::new(config, project);

        let path = PathBuf::from("README");
        let (target_path, extension) = builder.prepare_target_path(&path);

        assert_eq!(target_path, PathBuf::from("/target/README"));
        assert_eq!(extension, "");
    }

    #[test]
    fn test_prepare_target_path_nested_path() {
        let config = Config::new(PathBuf::from("/target"), PathBuf::from("/source"));
        let project = Rc::new(Project { modules: vec![] });
        let builder = DocsBuilder::new(config, project);

        let path = PathBuf::from(format!("dir/subdir/module.py.{}", SYSTEM_FILES_EXTENSION));
        let (target_path, extension) = builder.prepare_target_path(&path);

        assert_eq!(target_path, PathBuf::from("/target/dir/subdir/module.md"));
        assert_eq!(extension, "py");
    }

    #[test]
    fn test_get_module_source_path() {
        let config = Config::new(PathBuf::from("/target"), PathBuf::from("/source"));
        let project = Rc::new(Project { modules: vec![] });
        let builder = DocsBuilder::new(config, project);

        let module_path = PathBuf::from(format!("dir/module.rs.{}", SYSTEM_FILES_EXTENSION));
        let source_path = builder.get_module_source_path(&module_path);

        assert_eq!(
            source_path,
            PathBuf::from(format!("/source/dir/module.rs.{}", SYSTEM_FILES_EXTENSION))
        );
    }

    #[test]
    fn test_prepare_final_docs_single_section() {
        let config = Config::new(PathBuf::from("/target"), PathBuf::from("/source"));
        let project = Rc::new(Project { modules: vec![] });
        let builder = DocsBuilder::new(config, project);

        let section = Rc::new(Section {
            code: "fn hello() {}".to_string(),
            docs: "# Hello Function".to_string(),
            header: Some("Hello Function".to_string()),
            references: vec![],
        });

        let result = builder.prepare_final_docs(&[section], "rs");
        let expected = "# Hello Function\n```rs\nfn hello() {}\n```";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_prepare_final_docs_multiple_sections() {
        let config = Config::new(PathBuf::from("/target"), PathBuf::from("/source"));
        let project = Rc::new(Project { modules: vec![] });
        let builder = DocsBuilder::new(config, project);

        let section1 = Rc::new(Section {
            code: "fn hello() {}".to_string(),
            docs: "# Hello Function".to_string(),
            header: Some("Hello Function".to_string()),
            references: vec![],
        });

        let section2 = Rc::new(Section {
            code: "fn world() {}".to_string(),
            docs: "# World Function".to_string(),
            header: Some("World Function".to_string()),
            references: vec![],
        });

        let result = builder.prepare_final_docs(&[section1, section2], "rs");
        let expected = "# Hello Function\n```rs\nfn hello() {}\n```\n# World Function\n```rs\nfn world() {}\n```";

        assert_eq!(result, expected);
    }
}
