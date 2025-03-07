#![forbid(unsafe_code)]

use std::sync::Arc;

use crate::{config::config::Config, error::LPError};

use super::{
    code::{self, code_builder::CodeBuilder},
    docs::{self, docs_builder::DocsBuilder},
    spec::structs::Project,
};

pub struct Builder {
    config: Config,
    code_builder: CodeBuilder,
    docs_builder: DocsBuilder,
}

impl Builder {
    pub fn new(config: Config) -> Result<Self, LPError> {
        let project = match Project::new(&config.source_dir) {
            Ok(project) => project,
            Err(e) => return Err(e),
        };
        let shared_project = Arc::new(project);
        let code_builder = CodeBuilder::new(
            code::config::Config::new(
                config.code_dir.clone(),
                config.source_dir.clone(),
                config.code_plugins_dir.clone(),
            ),
            Arc::clone(&shared_project),
        )?;
        let docs_builder = DocsBuilder::new(
            docs::config::Config::new(config.docs_dir.clone(), config.source_dir.clone()),
            Arc::clone(&shared_project),
        );

        Ok(Builder {
            config: config.clone(),
            code_builder: code_builder,
            docs_builder: docs_builder,
        })
    }

    fn init(&self) -> Result<(), LPError> {
        // force check in future
        let _ = std::fs::remove_dir_all(&self.config.docs_dir);
        let _ = std::fs::remove_dir_all(&self.config.code_dir);

        std::fs::create_dir_all(&self.config.docs_dir).map_err(|e| LPError::Io(e))?;
        std::fs::create_dir_all(&self.config.code_dir).map_err(|e| LPError::Io(e))?;
        Ok(())
    }

    pub fn build(&self) -> Result<(), LPError> {
        println!("Bulding documentation to: {:?}", self.config.docs_dir);
        println!("Bulding code to: {:?}", self.config.code_dir);

        self.init()?;

        self.code_builder.build()?;
        self.docs_builder.build()?;
        Ok(())
    }
}
