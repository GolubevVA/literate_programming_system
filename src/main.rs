#![forbid(unsafe_code)]

mod builds;
mod cli;
mod config;
mod error;

use builds::builder::Builder;
use cli::args_processor::ParamsProcessor;
use config::config::Config;
use error::LPError;
use std::process;

fn main() -> Result<(), LPError> {
    let params_parser = ParamsProcessor::new();
    let params = params_parser.process_cli_params();

    if let Err(e) = params {
        eprintln!("{}", e);
        process::exit(1);
    }

    let params = params.unwrap();
    let config = Config::new(&params.target_dir, &params.src_dir);

    let builder = Builder::new(config);
    builder.build();

    Ok(())
}
