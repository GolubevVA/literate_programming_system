#![forbid(unsafe_code)]

use std::sync::Arc;

use mlua::Lua;

use crate::{config::config::Config, error::LPError};

use super::{
    code::{self, code_builder::CodeBuilder},
    docs::{self, docs_builder::DocsBuilder},
    index::ProjectIndex,
    spec::structs::Project,
};

pub struct Builder {
    config: Config,
    code_builder: CodeBuilder,
    docs_builder: DocsBuilder,
}

impl Builder {
    pub fn new(config: Config, lua: Arc<Lua>) -> Result<Self, LPError> {
        let project = match Project::new(&config.source_dir) {
            Ok(project) => project,
            Err(e) => return Err(e),
        };
        let shared_project = Arc::new(project);
        let index = ProjectIndex::new(shared_project.clone());
        let code_builder = CodeBuilder::new(
            code::config::Config::new(
                config.code_dir.clone(),
                config.source_dir.clone(),
                config.code_plugins_dir.clone(),
            ),
            Arc::clone(&shared_project),
            Arc::new(index),
            lua,
        )?;
        let docs_builder = DocsBuilder::new(
            docs::config::Config::new(config.docs_dir.clone(), config.source_dir.clone()),
            Arc::clone(&shared_project),
        );

        Ok(Builder {
            config: config.clone(),
            code_builder,
            docs_builder,
        })
    }

    fn init(&self) -> Result<(), LPError> {
        if self.config.force {
            std::fs::remove_dir_all(&self.config.docs_dir)?;
            std::fs::remove_dir_all(&self.config.code_dir)?;
        }

        std::fs::create_dir_all(&self.config.docs_dir)?;
        std::fs::create_dir_all(&self.config.code_dir)?;

        Ok(())
    }

    pub fn build(&self) -> Result<(), LPError> {
        self.init()?;

        println!("Bulding code to: {:?}", self.config.code_dir);
        self.code_builder.build()?;

        println!("Bulding documentation to: {:?}", self.config.docs_dir);
        self.docs_builder.build()?;

        Ok(())
    }
}
