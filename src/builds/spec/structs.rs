#![forbid(unsafe_code)]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Section is the smallest unit of the literate programming system.
/// It's referencable.
/// 
/// Header must be a valid markdown header, starting from a few (may be 1) #s.
/// Only one header per section is allowed
/// All headers within the same module must be unique.
/// Header is needed if the section is exported.
/// if the markdown conatins more than one header, only the one on the first line is considered.
#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub code: String,
    pub docs: String,
    pub header: Option<String>
}

/// A file is a module, each module has sections if it's a literate programming file
/// If it's not a literate programming file, it has only a path.
/// It's referencable
#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    /// not whole paths, just what after the source directory
    pub path: PathBuf,

    pub sections: Option<Vec<Section>>,
}

pub struct Project {
    pub modules: Vec<Module>,
}
