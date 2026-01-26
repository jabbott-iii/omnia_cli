pub mod ls;

use clap::Parser;
use std::path::PathBuf;

// Command-line interface definition
#[derive(Debug, Parser)]
#[command(version, author = "Joseph Abbott III", about = "An all in one CLI tool.")]
pub struct Cli {
    path: Option<PathBuf>,
}

pub fn core_run() {
    ls::ls_complete();
}