#![forbid(unsafe_code)]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Section is the smallest unit of the literate programming system
/// It's referencable
#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub code: String,
    pub docs: String,
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
