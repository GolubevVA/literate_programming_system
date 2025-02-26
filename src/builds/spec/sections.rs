#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::error::LPError;

use super::structs::Section;

#[derive(Debug, Serialize, Deserialize)]
struct RawSection {
    code: String,
    docs: String,
}

impl RawSection {
    pub fn get_header(&self) -> Option<String> {
        let lines: Vec<&str> = self.docs.lines().collect();
        if lines.is_empty() {
            return None;
        }
        let first_line = lines[0].trim();
        if !first_line.starts_with('#') {
            return None;
        }
        Some(first_line.to_string())
    }
}

impl Section {
    /// returns referencable section's header if exists
    pub fn get_header(&self) -> Option<String> {
        if self.header.is_none() {
            return None;
        }
        Some(self.header.as_ref().unwrap().trim_matches(|c| c == '#' || c == ' ').to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct LiterateFile {
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize)]
struct RawLiterateFile {
    sections: Vec<RawSection>,
}

impl RawLiterateFile {
    pub fn new(content: &str) -> Self {
        serde_yaml::from_str(content).unwrap()
    }
}

impl LiterateFile {
    pub fn new(content: &str) -> Result<Self, LPError> {
        let raw_lit_file = RawLiterateFile::new(content);
        
        let mut sections = Vec::new();
        let mut seen_headers = std::collections::HashSet::new();
        
        for raw_section in raw_lit_file.sections {
            let header = raw_section.get_header();

            let section = Section {
                code: raw_section.code,
                docs: raw_section.docs,
                header: header,
            };

            let header = section.get_header();

            sections.push(section);
            
            if let Some(ref h) = header {
                if !seen_headers.insert(h.clone()) {
                    return Err(LPError::DuplicateHeader(h.clone()));
                }
            }
        }
        
        Ok(LiterateFile { sections })
    }
}
