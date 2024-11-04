use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "codemerge")]
pub enum CodeMerge {
    #[structopt(
        name = "merge",
        about = "Merge multiple code files into a single output file"
    )]
    Merge {
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        #[structopt(short, long,)]
        ignores: Vec<String>,
        #[structopt(short = "f", long = "filter")]
        filters: Vec<String>,
        #[structopt(short, long)]
        verbose: bool,
        #[structopt(short = "n", long = "file-names-only", help = "Print only file names")]
        file_name: bool,
    },
    #[structopt(
        name = "tokens",
        about = "Calculate token counts for multiple code files"
    )]
    Tokens {
        #[structopt(short, long, default_value = "10")]
        count: usize,
        #[structopt(short, long, )]
        ignores: Vec<String>,
        #[structopt(short = "f", long = "filter")]
        filters: Vec<String>,
        #[structopt(short, long)]
        verbose: bool,
    },
} 