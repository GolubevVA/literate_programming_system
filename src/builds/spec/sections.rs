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
            .trim_matches(|c| c == '#')
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
    /// Returns an error if the file's sections have duplicate headers.
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
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_raw_section_get_header() {
        let section = RawSection {
            code: "fn main() {}".to_string(),
            docs: "# Header\ntext".to_string(),
        };
        assert_eq!(section.get_header(), Some("# Header".to_string()));

        let section = RawSection {
            code: "".to_string(),
            docs: "Not header\ntext".to_string(),
        };
        assert_eq!(section.get_header(), None);

        let section = RawSection {
            code: "".to_string(),
            docs: "".to_string(),
        };
        assert_eq!(section.get_header(), None);

        let section = RawSection {
            code: "".to_string(),
            docs: "## Multiple hashes ##".to_string(),
        };
        assert_eq!(
            section.get_header(),
            Some("## Multiple hashes ##".to_string())
        );
    }

    #[test]
    fn test_raw_section_get_references() {
        let section = RawSection {
            code: "".to_string(),
            docs: "See [link](#header)".to_string(),
        };
        let refs = section.get_references();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].path, Path::new(""));
        assert_eq!(refs[0].header, "header");

        let section = RawSection {
            code: "".to_string(),
            docs: "See [link](file#header)".to_string(),
        };
        let refs = section.get_references();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].path, Path::new("file"));
        assert_eq!(refs[0].header, "header");

        let section = RawSection {
            code: "".to_string(),
            docs: "Multiple refs: [one](#header1) and [two](other#header2)".to_string(),
        };
        let refs = section.get_references();
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].path, Path::new(""));
        assert_eq!(refs[0].header, "header1");
        assert_eq!(refs[1].path, Path::new("other"));
        assert_eq!(refs[1].header, "header2");

        let section = RawSection {
            code: "".to_string(),
            docs: "No refs here".to_string(),
        };
        let refs = section.get_references();
        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn test_section_get_header() {
        let section = Section {
            code: "".to_string(),
            docs: "".to_string(),
            header: Some("# Main Header #".to_string()),
            references: vec![],
        };
        assert_eq!(section.get_header(), Some("Main-Header".to_string()));

        let section = Section {
            code: "".to_string(),
            docs: "".to_string(),
            header: Some("## Complex Header: With Symbols!".to_string()),
            references: vec![],
        };
        assert_eq!(
            section.get_header(),
            Some("Complex-Header:-With-Symbols!".to_string())
        );

        let section = Section {
            code: "".to_string(),
            docs: "".to_string(),
            header: None,
            references: vec![],
        };
        assert_eq!(section.get_header(), None);
    }

    #[test]
    fn test_literate_file_new() {
        let content = r#"
sections:
  - code: |
        fn hello() {}
    docs: |
        # Hello Function
        This function says hello.
  - code: |
        fn world() {}
    docs: |
        ## World Function ##
        This function says world.
"#;
        let result = LiterateFile::new(content);
        assert!(result.is_ok());
        let lit_file = result.unwrap();
        assert_eq!(lit_file.sections.len(), 2);
        assert_eq!(
            lit_file.sections[0].header,
            Some("# Hello Function".to_string())
        );
        assert_eq!(
            lit_file.sections[1].header,
            Some("## World Function ##".to_string())
        );
    }

    #[test]
    fn test_literate_file_duplicate_headers() {
        let content = r#"
sections:
  - code: |
        fn hello() {}
    docs: |
        # Duplicate Header
        This function says hello.
  - code: |
        fn world() {}
    docs: |
        # Duplicate Header
        This function says world.
"#;
        let result = LiterateFile::new(content);
        assert!(result.is_err());
        match result {
            Err(LPError::DuplicateHeader(header)) => {
                assert_eq!(header, "Duplicate-Header");
            }
            _ => panic!("Expected DuplicateHeader error"),
        }
    }

    #[test]
    fn test_literate_file_with_references() {
        let content = r#"
sections:
  - code: |
        fn main() {}
    docs: |
        # Main
        See [other section](#other)
  - code: |
        fn other() {}
    docs: |
        # Other
        This is another section.
"#;
        let result = LiterateFile::new(content);
        assert!(result.is_ok());
        let lit_file = result.unwrap();
        assert_eq!(lit_file.sections[0].references.len(), 1);
        assert_eq!(lit_file.sections[0].references[0].path, Path::new(""));
        assert_eq!(lit_file.sections[0].references[0].header, "other");
    }
}
