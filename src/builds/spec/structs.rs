#![forbid(unsafe_code)]
use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

/// A reference to another literate notebook's section from the same project
#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    /// Not the whole path, just what comes after the source directory. So, it's relative.
    /// It does not inlude the file extension.
    ///
    /// e.g. if the original path is `src/file.py.lpnb` then it would be `file`
    /// 
    /// As well, this path is relative to the current module's path.
    pub path: PathBuf,

    /// The header of the section, without the #s and the leading spaces.
    /// As well, it's a real header, in it's original form.
    ///
    /// E.g. if the header is `## Some header` then this would be `Some header`, not `## Some header` or `Some-header`
    pub header: String,
}

/// Section is the smallest unit of the literate programming system.
/// It's referencable.
///
/// Header must be a valid markdown header, starting from a few (may be 1) #s.
/// Only one header per section is allowed.
/// All headers within the same module must be unique.
/// Header is needed if the section is exported.
/// If the markdown contains more than one header, only the one on the first line is considered.
#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub code: String,
    pub docs: String,
    pub header: Option<String>,
    pub references: Vec<Reference>,
}

/// A file is a module, each module has sections if it's a literate programming file.
/// If it's not a literate programming file, it has only a path.
///
/// All the modules paths should be unique within the project, even if their extensions differ.
/// E.g. `main.py.lpnb`` can not coexist with `main.cpp.lpnb`
#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    /// Not whole paths, just what comes after the source directory.
    pub path: PathBuf,

    pub sections: Option<Vec<Arc<Section>>>,
}

/// A project is a collection of modules.
/// It's the main entity of the literate programming system.
pub struct Project {
    pub modules: Vec<Arc<Module>>,
}
