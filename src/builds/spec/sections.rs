#![forbid(unsafe_code)]

use pulldown_cmark::{Parser, Event, Tag};
use serde::{Deserialize, Serialize};

use crate::error::LPError;

use super::structs::{Reference, Section};

#[derive(Debug, Serialize, Deserialize)]
struct RawSection {
    code: String,
    docs: String,
}

impl RawSection {
    /// returns raw header, with `#` symbols
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

    /// returns a list of references to other sections
    pub fn get_references(&self) -> Vec<Reference> {
        let parser = Parser::new(&self.docs);
        let mut references = Vec::new();
        
        for event in parser {
            if let Event::Start(Tag::Link(_, dest, _)) = event {
                let dest_str = dest.into_string();
                
                let path;
                let header;
                
                if let Some(hash_pos) = dest_str.find('#') {
                    path = dest_str[..hash_pos].to_string();
                    header = dest_str[hash_pos+1..].to_string();
                } else {
                    continue;
                }
                
                if !path.is_empty() || !header.is_empty() {
                    references.push(Reference {
                        path: path.into(),
                        header,
                    });
                }
            }
        }
        
        references
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
            let refs = raw_section.get_references();

            let section = Section {
                code: raw_section.code,
                docs: raw_section.docs,
                header: header,
                references: refs
            };

            let header = section.get_header();

            sections.push(section);
            
            if let Some(ref h) = header {
                if !seen_headers.insert(h.clone()) {
                    return Err(LPError::DuplicateHeader(h.clone()));
                }
            }
        }

        // references validation needed now
        
        Ok(LiterateFile { sections })
    }
}
