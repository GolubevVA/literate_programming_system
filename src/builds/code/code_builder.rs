#![forbid(unsafe_code)]

use std::{path::PathBuf, sync::Arc};

use mlua::Lua;

use crate::{
    builds::{
        index::ProjectIndex,
        spec::{
            structs::{Module, Project /*, Section*/},
            utils,
        },
    },
    error::LPError,
};

use super::{config::Config, plugins::caller::PluginsCaller};

pub struct CodeBuilder {
    config: Config,
    project: Arc<Project>,
    plugins_caller: Arc<PluginsCaller>,
    index: Arc<ProjectIndex>,
}

impl CodeBuilder {
    pub fn new(
        config: Config,
        project: Arc<Project>,
        index: Arc<ProjectIndex>,
        lua: Arc<Lua>,
    ) -> Result<Self, LPError> {
        let plugins_caller = Arc::new(PluginsCaller::new(lua.clone(), &config.plugins_dir)?);
        Ok(Self {
            config,
            project,
            plugins_caller,
            index,
        })
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

    fn get_all_code(&self, module: Arc<Module>) -> String {
        module
            .sections
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| s.as_ref().code.as_str())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    /// module must have sections
    /// all the references should be valid
    fn get_all_imports(&self, module: Arc<Module>) -> Result<String, LPError> {
        let mut imports: Vec<String> = vec![];
        let current_path = utils::prepare_module_file_extension(&module.path);
        let current_extension = current_path
            .extension()
            .unwrap_or(std::ffi::OsStr::new(""))
            .to_str()
            .unwrap();
        for section in module.sections.as_ref().unwrap() {
            for reference in &section.references {
                let reference_path = reference.path.clone();
                if reference_path != current_path && reference_path != PathBuf::from("") {
                    let mut referenced_module_path = reference.path.clone();
                    let referenced_header = reference.header.clone();
                    println!(
                        "Referencing: {:?} -> {:?} and {:?}",
                        current_path, referenced_module_path, referenced_header
                    );
                    let referenced_code = self
                        .index
                        .get_section(&referenced_module_path, &referenced_header)
                        .unwrap()
                        .code
                        .clone();
                    if current_path.extension().is_some() {
                        referenced_module_path.set_extension(current_path.extension().unwrap());
                    }
                    let import = self.plugins_caller.call_plugin_func(
                        current_extension,
                        &current_path,
                        &referenced_module_path,
                        &referenced_code,
                    );
                    match import {
                        Ok(import) => imports.push(import),
                        Err(e) => return Err(e),
                    }
                }
            }
        }
        Ok(imports.join("\n"))
    }

    fn prepare_final_code(&self, module: Arc<Module>) -> Result<String, LPError> {
        let no_imports_code = self.get_all_code(module.clone());
        let imports = match self.get_all_imports(module.clone()) {
            Ok(imports) => imports,
            Err(e) => return Err(e),
        };
        Ok(format!("{}\n{}", imports, no_imports_code))
    }

    pub fn build(&self) -> Result<(), LPError> {
        for module in &self.project.modules {
            let source_path = self.get_module_source_path(&module.path);
            let target_path = self.prepare_target_path(&module.path);

            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| LPError::Io(e))?;
            }

            if module.sections.is_some() {
                let final_code = self.prepare_final_code(module.clone())?;
                std::fs::write(target_path, final_code).map_err(|e| LPError::Io(e))?;
            } else {
                std::fs::copy(source_path, target_path).map_err(|e| LPError::Io(e))?;
            }
        }
        Ok(())
    }
}
