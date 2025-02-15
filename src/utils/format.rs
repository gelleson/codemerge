use crate::core::file::FileData;
use anyhow::Result;
use std::path::PathBuf;

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
