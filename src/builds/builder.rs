#![forbid(unsafe_code)]

use std::sync::Arc;

use crate::{config::config::Config, error::LPError};

use super::{
    code::{self, code_builder::CodeBuilder},
    docs::docs_builder::DocsBuilder,
    spec::structs::Project,
};

pub struct Builder {
    config: Config,
    code_builder: CodeBuilder,
    docs_builder: DocsBuilder,
}

impl Builder {
    pub fn new(config: Config) -> Self {
        let project = Project::new(&config.source_dir);
        let shared_project = Arc::new(project);
        let code_builder = CodeBuilder::new(
            code::config::Config::new(config.code_dir.clone(), config.source_dir.clone()),
            Arc::clone(&shared_project),
        );
        let docs_builder = DocsBuilder::new(config.docs_dir.clone(), Arc::clone(&shared_project));

        Builder {
            config: config.clone(),
            code_builder: code_builder,
            docs_builder: docs_builder,
        }
    }

    fn init(&self) -> Result<(), LPError> {
        std::fs::create_dir_all(&self.config.docs_dir).map_err(|e| LPError::Io(e))?;
        std::fs::create_dir_all(&self.config.code_dir).map_err(|e| LPError::Io(e))?;
        Ok(())
    }

    pub fn build(&self) -> Result<(), LPError> {
        println!(
            "Building from source directory: {:?}",
            self.config.source_dir
        );
        println!("Bulding documentation to: {:?}", self.config.docs_dir);
        println!("Bulding code to: {:?}", self.config.code_dir);

        self.init()?;

        self.code_builder.build()?;
        self.docs_builder.build()?;
        Ok(())
    }
}
