#![forbid(unsafe_code)]

use crate::builds::spec::structs::Section;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use super::spec::structs::Project;
use super::spec::utils;

pub struct ProjectIndex {
    sections: HashMap<PathBuf, HashMap<String, Arc<Section>>>,
}

impl ProjectIndex {
    pub fn new(project: Arc<Project>) -> ProjectIndex {
        let mut sections = HashMap::new();

        for module in project.modules.iter() {
            let mut path = utils::prepare_module_file_extension(&module.path);
            if path.extension().is_some() {
                path.set_extension("");
            }

            if let Some(module_sections) = &module.sections {
                let mut header_map = HashMap::new();

                for section in module_sections {
                    if let Some(header) = &section.get_header() {
                        println!("Adding {}/{}", path.display(), header);
                        header_map.insert(header.clone(), section.clone());
                    }
                }

                if !header_map.is_empty() {
                    sections.insert(path, header_map);
                }
            }
        }

        ProjectIndex { sections }
    }

    /// The path is treated as a module path without any extension
    ///
    /// e.g. cmd/api/main, but not ~/projects/my-project/cmd/api/main.rs.lpnb
    pub fn get_section(&self, path: &PathBuf, header: &str) -> Option<&Arc<Section>> {
        self.sections.get(path)?.get(header)
    }
}
