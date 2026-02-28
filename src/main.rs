//! `codemerge` is an ultra-fast CLI tool designed for merging and analyzing code.
//!
//! It provides functionalities to merge multiple source files into a single output,
//! analyze token counts, display directory trees, and manage caching for better performance.
//!
//! This is the main entry point for the application.

use clap::Parser;

mod cache;
mod cli;
mod config;
mod core;
mod error;
mod utils;

use error::Result;

use cli::args::Cli;

fn main() -> Result<()> {
    // Initialize logging if CODEMERGE_DEBUG is set
    if std::env::var("CODEMERGE_DEBUG").is_ok() {
        eprintln!("Debug mode enabled");
    }

    let cli = Cli::parse();
    cli::commands::execute(cli)
}
