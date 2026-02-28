//! Output formatting functionality.
//!
//! Handles taking processed file data and formatting it as plain text
//! or JSON, either printing it to stdout or saving it to a file.

use crate::core::file::FileData;
use anyhow::Result;
use std::path::PathBuf;

/// Output the final processed file data to the console or a file.
///
/// # Arguments
///
/// * `files` - A slice of `FileData` representing the files to output.
/// * `format` - A string defining the output format (e.g., "text", "json").
/// * `output` - An optional path to a file where the results should be written.
///
/// # Returns
///
/// * `Result<()>` - Returns success if writing completes, or an error.
pub fn output_results(files: &[FileData], format: &str, output: Option<PathBuf>) -> Result<()> {
    let content = match format {
        "text" => format_text(files),
        "json" => serde_json::to_string_pretty(files)?,
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
    };

    match output {
        Some(path) => {
            std::fs::write(path, content)?;
        }
        None => {
            println!("{}", content);
        }
    }

    Ok(())
}

fn format_text(files: &[FileData]) -> String {
    let mut output = String::from("=== Result ===\n");
    for file in files {
        output.push_str(&format!("File: {}\n{}", file.path, file.content));
    }
    output
}
