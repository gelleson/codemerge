use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CodeMerge {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Merge code files
    Merge {
        #[arg(value_parser)]
        path: Option<PathBuf>,
        #[arg(short, long, value_parser)]
        output: Option<PathBuf>,
        #[clap(short, long)]
        ignores: Vec<String>,
        #[clap(short = 'f', long = "filter")]
        filters: Vec<String>,
        #[clap(short, long)]
        verbose: bool,
        #[clap(short = 'n', long = "file-names-only", help = "Print only file names")]
        file_name: bool,
    },
    /// Token management commands
    Tokens {
        /// Tokenizer model to use (gpt-3.5, gpt-4, claude)
        #[clap(long, default_value = "gpt-3.5")]
        model: String,

        /// Token budget limit
        #[clap(long)]
        budget: Option<usize>,

        /// Warning threshold as percentage of budget (0.0-1.0)
        #[clap(long, default_value = "0.8")]
        warning_threshold: f32,

        /// Base directory to analyze
        #[clap(value_parser, default_value = ".")]
        path: PathBuf,

        /// Number of top files to show
        #[clap(short = 'n', long, default_value = "10")]
        count: usize,

        /// Ignore patterns
        #[clap(short, long)]
        ignores: Vec<String>,

        /// File filters
        #[clap(short = 'f', long = "filter")]
        filters: Vec<String>,

        /// Show verbose output
        #[clap(short, long)]
        verbose: bool,
    },
} 