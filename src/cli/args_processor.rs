#![forbid(unsafe_code)]

use std::path::Path;

use clap::Parser;

use super::structs::Params;

use crate::error::LPError;

/// Parses CLI aguments, prepares and validates them.
pub struct ParamsProcessor {}

impl ParamsProcessor {
    /// Creates a new instance of ParamsProcessor.
    pub fn new() -> Self {
        Self {}
    }

    fn validate_params(&self, params: &Params) -> Option<LPError> {
        if !Path::new(&params.src_dir).is_dir() {
            return Some(LPError::SourceDirectoryNotFound(
                params.src_dir.to_string_lossy().to_string(),
            ));
        }
        None
    }

    /// Returns processed params for the CLI App.
    pub fn process_cli_params(&self) -> Result<Params, LPError> {
        let params = Params::parse();

        let res = self.validate_params(&params);
        match res {
            Some(e) => Err(e),
            None => Ok(params),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempfile::tempdir;
    use super::*;

    #[test]
    fn test_validate_params() {
        let processor = ParamsProcessor::new();
        let temp_dir = tempdir().unwrap();
        let params = Params {
            src_dir: temp_dir.path().to_path_buf(),
            target_dir: PathBuf::from("target"),
            plugins_dir: PathBuf::from("plugins"),
            force: false,
        };

        assert!(processor.validate_params(&params).is_none());
    }

    #[test]
    fn test_validate_params_invalid() {
        let processor = ParamsProcessor::new();
        let params = Params {
            src_dir: PathBuf::from("tests_invalid"),
            target_dir: PathBuf::from("target"),
            plugins_dir: PathBuf::from("plugins"),
            force: false,
        };

        assert!(processor.validate_params(&params).is_some());
    }
}
