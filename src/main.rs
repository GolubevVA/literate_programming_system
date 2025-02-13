#![forbid(unsafe_code)]

mod builds;
mod cli;
mod config;
mod error;

use anyhow::Result;
use builds::builder::Builder;
use config::config::Config;

fn main() -> Result<()> {
    let params_parser = cli::args_processor::ParamsProcessor::new();
    let params = params_parser.process_cli_params();

    if let Err(e) = params {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    let params = params.unwrap();
    let config = Config::new(&params.target_dir);

    let builder = Builder::new(params.src_dir, config);
    builder.build();

    Ok(())
}
