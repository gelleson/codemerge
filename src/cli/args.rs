use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "codemerge")]
#[command(about = "Ultra fast CLI for merging and analyzing code", long_about = None)]
#[command(version)]
pub struct Cli {
    /// Global config override
    #[arg(long)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Merge file contents into one output
    Merge {
        /// Path to files or directories
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Glob filters to include
        #[arg(short = 'f', long, num_args = 1.., default_value = "**")]
        filters: Vec<String>,

        /// Patterns to ignore
        #[arg(short = 'i', long, num_args = 1..)]
        ignores: Vec<String>,

        /// Output file path
        #[arg(long)]
        output: Option<PathBuf>,

        /// Output format
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,

        /// Maximum token budget
        #[arg(long = "max-budget", short = 'M', default_value_t = 10000)]
        max_budget: usize,

        /// Minimum token budget
        #[arg(long = "min-budget", short = 'm', default_value_t = 0)]
        min_budget: usize,

        /// Enable high-budget filtering
        #[arg(long = "limit-by-high-budget")]
        limit_by_high_budget: bool,

        /// Enable low-budget filtering
        #[arg(long = "limit-by-low-budget")]
        limit_by_low_budget: bool,

        /// Use specific config context
        #[arg(long)]
        context: Option<String>,

        /// Ignore config file
        #[arg(long = "ignore-config")]
        ignore_config: bool,

        /// Alternative config path
        #[arg(long = "config-path")]
        config_path: Option<PathBuf>,

        /// Read from stdin
        #[arg(long)]
        input: bool,
    },

    /// Display file tree structure
    Tree {
        /// Root path
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Glob filters
        #[arg(short = 'f', long, num_args = 1.., default_value = "**")]
        filters: Vec<String>,

        /// Ignore patterns
        #[arg(short = 'i', long, num_args = 1..)]
        ignores: Vec<String>,

        /// Maximum token budget
        #[arg(long = "max-budget", short = 'M', default_value_t = 10000)]
        max_budget: usize,

        /// Minimum token budget
        #[arg(long = "min-budget", short = 'm', default_value_t = 0)]
        min_budget: usize,

        /// Enable high-budget filtering
        #[arg(long = "limit-by-high-budget")]
        limit_by_high_budget: bool,

        /// Enable low-budget filtering
        #[arg(long = "limit-by-low-budget")]
        limit_by_low_budget: bool,

        /// Use specific config context
        #[arg(long)]
        context: Option<String>,

        /// Ignore config file
        #[arg(long = "ignore-config")]
        ignore_config: bool,

        /// Alternative config path
        #[arg(long = "config-path")]
        config_path: Option<PathBuf>,

        /// Read from stdin
        #[arg(long)]
        input: bool,
    },

    /// Calculate token counts
    Tokens {
        /// Root path
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Glob filters
        #[arg(short = 'f', long, num_args = 1.., default_value = "**")]
        filters: Vec<String>,

        /// Ignore patterns
        #[arg(short = 'i', long, num_args = 1..)]
        ignores: Vec<String>,

        /// Number of files to display
        #[arg(short = 'n', default_value_t = 10)]
        total: usize,

        /// Maximum token budget
        #[arg(long = "max-budget", short = 'M', default_value_t = 10000)]
        max_budget: usize,

        /// Minimum token budget
        #[arg(long = "min-budget", short = 'm', default_value_t = 0)]
        min_budget: usize,

        /// Enable high-budget filtering
        #[arg(long = "limit-by-high-budget")]
        limit_by_high_budget: bool,

        /// Enable low-budget filtering
        #[arg(long = "limit-by-low-budget")]
        limit_by_low_budget: bool,

        /// Use specific config context
        #[arg(long)]
        context: Option<String>,

        /// Ignore config file
        #[arg(long = "ignore-config")]
        ignore_config: bool,

        /// Alternative config path
        #[arg(long = "config-path")]
        config_path: Option<PathBuf>,

        /// Read from stdin
        #[arg(long)]
        input: bool,

        /// Output format
        #[arg(long, default_value = "plain", value_parser = ["plain", "json"])]
        format: String,
    },

    /// Initialize configuration
    Init {
        /// Configuration filename
        #[arg(long = "file-name", default_value = ".codemerge.yaml")]
        file_name: String,

        /// Force overwrite
        #[arg(short, long)]
        force: bool,
    },
}
