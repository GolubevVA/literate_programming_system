#![forbid(unsafe_code)]

use std::{path::PathBuf, sync::Arc};

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
    project: Arc<Project>,
}

impl DocsBuilder {
    /// Creates a new DocsBuilder instance.
    pub fn new(config: Config, project: Arc<Project>) -> Self {
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

    fn prepare_final_docs(&self, sections: &[Arc<Section>], extension: &str) -> String {
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
                std::fs::create_dir_all(parent).map_err(|e| LPError::Io(e))?;
            }
            if let Some(sections) = &module.sections {
                std::fs::write(
                    target_path,
                    self.prepare_final_docs(sections, extension.as_str()),
                )
                .map_err(|e| LPError::Io(e))?;
            } else {
                std::fs::copy(source_path, target_path).map_err(|e| LPError::Io(e))?;
            }
        }
        Ok(())
    }
}
