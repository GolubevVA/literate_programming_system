#![forbid(unsafe_code)]

use std::path::Path;

use clap::Parser;

use super::structs::Params;

use crate::error::LPError;

/// Parses CLI aguments, prepares and validates them.
pub struct ParamsProcessor {}

impl ParamsProcessor {
    pub fn new() -> Self {
        Self {}
    }

    fn validate_params(&self, params: &Params) -> Option<LPError> {
        if !Path::new(&params.src_dir).is_dir() {
            return Some(LPError::SourceDirectoryNotFound(
                params.src_dir.to_string_lossy().to_string(),
            ));
        }
        // may add checks with force flag and target already exists
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
