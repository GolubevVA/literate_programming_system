#![forbid(unsafe_code)]

use crate::builds::spec::structs::Section;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use super::spec::structs::Project;
use super::spec::utils;

/// Index for the project sections.
///
/// It's used to quickly find a section by its header and it's module path.
pub struct ProjectIndex {
    sections: HashMap<PathBuf, HashMap<String, Arc<Section>>>,
}

impl ProjectIndex {
    /// Creates a new project index from the project.
    pub fn new(project: Arc<Project>) -> ProjectIndex {
        let mut sections = HashMap::new();

        for module in project.modules.iter() {
            let path = utils::module_name(&module.path);

            if let Some(module_sections) = &module.sections {
                let mut header_map = HashMap::new();

                for section in module_sections {
                    if let Some(header) = &section.get_header() {
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::builds::spec::structs::Module;

    fn create_test_project() -> Arc<Project> {
        let section1 = Arc::new(Section {
            code: "code1".to_string(),
            docs: "docs1".to_string(),
            header: Some("Header 1".to_string()),
            references: vec![],
        });

        let section2 = Arc::new(Section {
            code: "code2".to_string(),
            docs: "docs2".to_string(),
            header: Some("Header 2".to_string()),
            references: vec![],
        });

        let section_no_header = Arc::new(Section {
            code: "code3".to_string(),
            docs: "docs3".to_string(),
            header: None,
            references: vec![],
        });

        let module1 = Arc::new(Module {
            path: PathBuf::from("module1.rs.lpnb"),
            sections: Some(vec![section1.clone(), section2.clone()]),
        });

        let module2 = Arc::new(Module {
            path: PathBuf::from("subdir/module2.rs.lpnb"),
            sections: Some(vec![
                Arc::new(Section {
                    code: "code4".to_string(),
                    docs: "docs4".to_string(),
                    header: Some("Header 3".to_string()),
                    references: vec![],
                }),
                section_no_header,
            ]),
        });

        let module_no_sections = Arc::new(Module {
            path: PathBuf::from("empty.rs.lpnb"),
            sections: None,
        });

        Arc::new(Project {
            modules: vec![module1, module2, module_no_sections],
        })
    }

    #[test]
    fn test_new_project_index() {
        let project = create_test_project();
        let index = ProjectIndex::new(project);

        assert!(index.sections.contains_key(&PathBuf::from("module1")));
        assert!(index
            .sections
            .contains_key(&PathBuf::from("subdir/module2")));
        assert!(!index.sections.contains_key(&PathBuf::from("empty")));

        assert_eq!(
            index.sections.get(&PathBuf::from("module1")).unwrap().len(),
            2
        );
        assert_eq!(
            index
                .sections
                .get(&PathBuf::from("subdir/module2"))
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn test_get_section() {
        let project = create_test_project();
        let index = ProjectIndex::new(project);

        let section = index.get_section(&PathBuf::from("module1"), "Header-1");
        assert!(section.is_some());
        assert_eq!(section.unwrap().code, "code1");

        let section = index.get_section(&PathBuf::from("subdir/module2"), "Header-3");
        assert!(section.is_some());
        assert_eq!(section.unwrap().code, "code4");

        let section = index.get_section(&PathBuf::from("nonexistent"), "Header-1");
        assert!(section.is_none());

        let section = index.get_section(&PathBuf::from("module1"), "Nonexistent-Header");
        assert!(section.is_none());

        let section = index.get_section(&PathBuf::from("empty"), "Any-Header");
        assert!(section.is_none());
    }
}
