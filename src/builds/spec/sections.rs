#![forbid(unsafe_code)]

use pulldown_cmark::{Event, Parser, Tag};
use serde::{Deserialize, Serialize};

use crate::error::LPError;

use super::{
    structs::{Reference, Section},
    utils::header_to_anchor,
};

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
                    header = dest_str[hash_pos + 1..].to_string();
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
    /// returns referencable section's header if exists, formatted as an anchor
    pub fn get_header(&self) -> Option<String> {
        if self.header.is_none() {
            return None;
        }

        let header_text = self
            .header
            .as_ref()
            .unwrap()
            .trim_start_matches(|c| c == '#')
            .trim();

        Some(header_to_anchor(header_text))
    }
}

/// Represents a literate file, which is a collection of sections.
/// 
/// It can be deserialized from a string. It's used to form a module.
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
    /// Returns a new LiterateFile instance.
    /// References are not validated.
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
                header,
                references: refs,
            };

            let section_header = section.get_header();

            sections.push(section);

            if let Some(ref h) = section_header {
                if !seen_headers.insert(h.clone()) {
                    return Err(LPError::DuplicateHeader(h.clone()));
                }
            }
        }

        Ok(LiterateFile { sections })
    }
}
