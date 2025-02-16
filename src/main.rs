use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod core;
mod error;
mod utils;

use cli::args::Cli;

fn main() -> Result<()> {
    // Initialize logging if CODEMERGE_DEBUG is set
    if std::env::var("CODEMERGE_DEBUG").is_ok() {
        eprintln!("Debug mode enabled");
    }

    let cli = Cli::parse();
    cli::commands::execute(cli)
}
