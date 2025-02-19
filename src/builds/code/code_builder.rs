#![forbid(unsafe_code)]

use std::{path::PathBuf, sync::Arc};

use crate::{
    builds::spec::{
        structs::{Project, Section},
        utils,
    },
    error::LPError,
};

use super::config::Config;

pub struct CodeBuilder {
    config: Config,
    project: Arc<Project>,
}

impl CodeBuilder {
    pub fn new(config: Config, project: Arc<Project>) -> Self {
        Self { config, project }
    }

    fn prepare_target_path(&self, path: &PathBuf) -> PathBuf {
        let mut result = self.config.target_code_dir.clone();
        result.push(path);
        utils::prepare_module_file_extension(&result)
    }

    fn get_module_source_path(&self, module: &PathBuf) -> PathBuf {
        let mut result = self.config.source_dir.clone();
        result.push(module);
        result
    }

    fn prepare_final_code(&self, sections: &[Section]) -> String {
        sections
            .iter()
            .map(|s| s.code.as_str())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    pub fn build(&self) -> Result<(), LPError> {
        for module in &self.project.modules {
            let source_path = self.get_module_source_path(&module.path);
            let target_path = self.prepare_target_path(&module.path);
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| LPError::Io(e))?;
            }
            if let Some(sections) = &module.sections {
                std::fs::write(target_path, self.prepare_final_code(sections))
                    .map_err(|e| LPError::Io(e))?;
            } else {
                std::fs::copy(source_path, target_path).map_err(|e| LPError::Io(e))?;
            }
        }
        Ok(())
    }
}
