#![forbid(unsafe_code)]

use std::{path::PathBuf, sync::Arc};

use crate::config::config::Config;

use super::{
    code::code_builder::CodeBuilder, docs::docs_builder::DocsBuilder, format::structs::Project,
};

pub struct Builder {
    source_dir: PathBuf,
    config: Config,
    code_builder: CodeBuilder,
    docs_builder: DocsBuilder,
}

impl Builder {
    pub fn new(source_dir: PathBuf, config: Config) -> Self {
        let project = Project::new(&source_dir);
        let shared_project = Arc::new(project);
        let code_builder = CodeBuilder::new(config.code_dir.clone(), Arc::clone(&shared_project));
        let docs_builder = DocsBuilder::new(config.docs_dir.clone(), Arc::clone(&shared_project));

        Builder {
            source_dir: source_dir.clone(),
            config: config.clone(),
            code_builder: code_builder,
            docs_builder: docs_builder,
        }
    }

    pub fn build(&self) {
        println!("Building from source directory: {:?}", self.source_dir);
        println!("Bulding documentation to: {:?}", self.config.docs_dir);
        println!("Bulding code to: {:?}", self.config.code_dir);

        self.code_builder.build();
        self.docs_builder.build();
    }
}
