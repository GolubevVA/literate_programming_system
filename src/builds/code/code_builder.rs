#![forbid(unsafe_code)]

use std::{path::PathBuf, rc::Rc};

use mlua::Lua;

use crate::{
    builds::{
        index::ProjectIndex,
        spec::{
            structs::{Module, Project},
            utils::{self, get_module_extension},
        },
    },
    error::LPError,
};

use super::{config::Config, plugins::caller::PluginsCaller};

/// CodeBuilder is a struct that is responsible for building the code from the source project.
pub struct CodeBuilder {
    config: Config,
    project: Rc<Project>,
    plugins_caller: Rc<PluginsCaller>,
    index: Rc<ProjectIndex>,
}

impl CodeBuilder {
    /// Creates a new CodeBuilder instance.
    /// # Arguments
    /// * `config` - a Config instance that contains the configuration for the builder.
    /// * `project` - an Rc<Project> instance that contains the source project.
    /// * `index` - an Rc<ProjectIndex> instance that contains the index of the project.
    /// * `lua` - an Rc<Lua> instance that is used for running Lua plugins.
    pub fn new(
        config: Config,
        project: Rc<Project>,
        index: Rc<ProjectIndex>,
        lua: Rc<Lua>,
    ) -> Result<Self, LPError> {
        let plugins_caller = Rc::new(PluginsCaller::new(lua.clone(), &config.plugins_dir)?);
        Ok(Self {
            config,
            project,
            plugins_caller,
            index,
        })
    }

    fn validate_references(&self, module: Rc<Module>) -> Result<(), LPError> {
        for section in module.sections.as_ref().unwrap() {
            for reference in &section.references {
                let referenced_module_path = module.resolve_relative_module_path(&reference.path);
                if self
                    .index
                    .get_section(&referenced_module_path, &reference.header)
                    .is_none()
                {
                    return Err(LPError::IncorrectReference(
                        reference.path.clone(),
                        reference.header.clone(),
                    ));
                }
            }
        }
        Ok(())
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

    fn get_all_code(&self, module: Rc<Module>) -> String {
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
    fn get_all_imports(&self, module: Rc<Module>) -> Result<String, LPError> {
        let mut imports: Vec<String> = vec![];
        let current_path = utils::prepare_module_file_extension(&module.path);
        let current_extension = get_module_extension(&current_path);
        for section in module.sections.as_ref().unwrap() {
            for reference in &section.references {
                let reference_path = reference.path.clone();
                if reference_path != current_path && reference_path != PathBuf::from("") {
                    let referenced_module_relative_path = reference.path.clone();
                    let mut referenced_module_path =
                        module.resolve_relative_module_path(&referenced_module_relative_path);
                    let referenced_header = reference.header.clone();
                    let referenced_code = self
                        .index
                        .get_section(&referenced_module_path, &referenced_header)
                        .unwrap()
                        .code
                        .clone();
                    if current_path.extension().is_some() {
                        referenced_module_path.set_extension(current_path.extension().unwrap());
                    }
                    let import = self.plugins_caller.call_plugin_import_func(
                        current_extension.as_str(),
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

    fn prepare_final_code(&self, module: Rc<Module>) -> Result<String, LPError> {
        let no_imports_code = self.get_all_code(module.clone());
        let imports = match self.get_all_imports(module.clone()) {
            Ok(imports) => imports,
            Err(e) => return Err(e),
        };
        self.plugins_caller.call_plugin_cleaning_func(
            get_module_extension(&module.path).as_str(),
            &format!("{}\n{}", imports, no_imports_code),
        )
    }

    /// The main method of the CodeBuilder that builds the code.
    /// It validates the references, prepares the final code and writes it to the target directory.
    /// If the module has no sections, it just copies the source file to the target directory.
    /// Returns an error if any of the operations failed.
    pub fn build(&self) -> Result<(), LPError> {
        for module in &self.project.modules {
            if module.sections.is_some() {
                self.validate_references(Rc::clone(module))?
            }
        }

        for module in &self.project.modules {
            let source_path = self.get_module_source_path(&module.path);
            let target_path = self.prepare_target_path(&module.path);

            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            if module.sections.is_some() {
                let final_code = self.prepare_final_code(module.clone())?;
                std::fs::write(target_path, format!("{}\n", final_code))?;
            } else {
                std::fs::copy(source_path, target_path)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builds::spec::structs::Section;
    use crate::config::constants::SYSTEM_FILES_EXTENSION;

    #[test]
    fn test_prepare_target_path() {
        let config = Config::new(
            PathBuf::from("/target"),
            PathBuf::from("/source"),
            PathBuf::from("/plugins"),
        );
        let project = Rc::new(Project { modules: vec![] });
        let index = Rc::new(ProjectIndex::new(Rc::clone(&project)));
        let lua = Rc::new(Lua::new());

        let builder = CodeBuilder::new(config, project, index, Rc::clone(&lua)).unwrap();

        let path = PathBuf::from(format!("module.rs.{}", SYSTEM_FILES_EXTENSION));
        let target_path = builder.prepare_target_path(&path);

        assert_eq!(target_path, PathBuf::from("/target/module.rs"));
    }

    #[test]
    fn test_get_module_source_path() {
        let config = Config::new(
            PathBuf::from("/target"),
            PathBuf::from("/source"),
            PathBuf::from("/plugins"),
        );
        let project = Rc::new(Project { modules: vec![] });
        let index = Rc::new(ProjectIndex::new(Rc::clone(&project)));
        let lua = Rc::new(Lua::new());

        let builder = CodeBuilder::new(config, project, index, Rc::clone(&lua)).unwrap();

        let module_path = PathBuf::from(format!("dir/module.rs.{}", SYSTEM_FILES_EXTENSION));
        let source_path = builder.get_module_source_path(&module_path);

        assert_eq!(
            source_path,
            PathBuf::from(format!("/source/dir/module.rs.{}", SYSTEM_FILES_EXTENSION))
        );
    }

    #[test]
    fn test_get_all_code() {
        let config = Config::new(
            PathBuf::from("/target"),
            PathBuf::from("/source"),
            PathBuf::from("/plugins"),
        );

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

        let module = Rc::new(Module {
            path: PathBuf::from("test.rs.lpnb"),
            sections: Some(vec![section1, section2]),
        });

        let project = Rc::new(Project { modules: vec![] });
        let index = Rc::new(ProjectIndex::new(Rc::clone(&project)));
        let lua = Rc::new(Lua::new());

        let builder = CodeBuilder::new(config, project, index, Rc::clone(&lua)).unwrap();

        let code = builder.get_all_code(module);
        let expected = "fn hello() {}\nfn world() {}";

        assert_eq!(code, expected);
    }
}
