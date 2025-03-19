#![forbid(unsafe_code)]
//! structs for CLI arguments and parameters

use clap::Parser;
use std::path::PathBuf;

use crate::config::constants::{
    DEFAULT_PLUGINS_DIR_NAME, DEFAULT_SOURCE_DIR_NAME, DEFAULT_TARGETS_DIR_NAME,
};

/// CLI arguments
#[derive(Parser, Debug)]
#[clap(
    name = "lp",
    version = "1.0",
    about = "Utility for literate programming system.",
    long_about = "lp is a utility to build literate programming projects.\nIt is used to build source code and documentation from the same code."
)]
#[clap(version)]
pub struct Params {
    /// Source directory [default: DEFAULT_SOURCE_DIR_NAME]
    #[clap(short='s', long, default_value = DEFAULT_SOURCE_DIR_NAME)]
    pub src_dir: PathBuf,

    /// Target directory [default: DEFAULT_TARGETS_DIR_NAME]
    #[clap(short='t', long, default_value = DEFAULT_TARGETS_DIR_NAME)]
    pub target_dir: PathBuf,

    /// Plugins directory [default: DEFAULT_PLUGINS_DIR_NAME]
    #[clap(short='p', long, default_value = DEFAULT_PLUGINS_DIR_NAME)]
    pub plugins_dir: PathBuf,

    /// Clear the target directories before building.
    /// If not enabled, then existing files will be overwritten only if the same file appears after the build process.
    #[clap(short = 'f', long)]
    pub force: bool,
}
