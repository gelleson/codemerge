use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use tiktoken_rs::o200k_base;

use codemerge::{cli::CodeMerge, token_counter};

fn main() -> io::Result<()> {
    let opt = CodeMerge::from_args();
    let bpe = o200k_base().expect("Failed to load BPE model");

    match opt {
        CodeMerge::Merge {
            path,
            output,
            ignores,
            filters,
            verbose,
            file_name,
        } => {
            let base_path = path.unwrap_or_else(|| PathBuf::from("."));
            token_counter::merge_files(
                &base_path,
                output.as_deref(),
                &ignores,
                &filters,
                verbose,
                file_name,
            )?;
        }
        CodeMerge::Tokens {
            count,
            ignores,
            filters,
            verbose,
        } => {
            token_counter::calculate_tokens(count, &ignores, &filters, verbose, &bpe)?;
        }
    }

    Ok(())
}
