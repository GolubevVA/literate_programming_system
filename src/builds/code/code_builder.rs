#![forbid(unsafe_code)]

use std::{path::PathBuf, sync::Arc};

use crate::builds::format::structs::Project;

pub struct CodeBuilder {
    code_dir: PathBuf,
    project: Arc<Project>,
}

impl CodeBuilder {
    pub fn new(code_dir: PathBuf, project: Arc<Project>) -> Self {
        Self { code_dir, project }
    }

    pub fn build(&self) {
        for module in &self.project.modules {
            let module_path = self.code_dir.join(&module.path);
            println!("Building code module: {:?}", module_path);
        }
    }
}
