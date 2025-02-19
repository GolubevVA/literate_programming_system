#![forbid(unsafe_code)]

use std::{path::PathBuf, sync::Arc};

use crate::{builds::spec::structs::Project, error::LPError};

pub struct DocsBuilder {
    docs_dir: PathBuf,
    project: Arc<Project>,
}

impl DocsBuilder {
    pub fn new(docs_dir: PathBuf, project: Arc<Project>) -> Self {
        Self { docs_dir, project }
    }

    pub fn build(&self) -> Result<(), LPError> {
        for module in &self.project.modules {
            let module_path = self.docs_dir.join(&module.path);
            println!("Building docs module: {:?}", module_path);
        }
        Ok(())
    }
}
