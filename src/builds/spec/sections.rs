#![forbid(unsafe_code)]

use serde::Deserialize;

use super::structs::Section;


#[derive(Debug, Deserialize)]
pub struct LiterateFile {
    pub sections: Vec<Section>,
}

impl LiterateFile {
    pub fn new(content: &str) -> Self {
        serde_yaml::from_str(content).unwrap()
    }
}
