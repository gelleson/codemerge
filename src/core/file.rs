use super::tokens::count_tokens;
use anyhow::Result;
use memmap2::MmapOptions;
use rayon::prelude::*;
use serde::Serialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct FileData {
    pub path: String,
    pub content: String,
    pub tokens: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl FileData {
    pub fn new(path: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        let tokens = count_tokens(&content);

        // If content is empty or only whitespace, return with 0 tokens
        if content.trim().is_empty() {
            return Self {
                path: path.into(),
                content: String::new(),
                tokens: 0,
                error: None,
            };
        }

        Self {
            path: path.into(),
            tokens,
            content,
            error: None,
        }
    }

    pub fn with_error(path: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            content: String::new(),
            tokens: 0,
            error: Some(error.into()),
        }
    }
}

pub fn read_file(path: &Path) -> Result<FileData> {
    let file = File::open(path)?;
    let metadata = file.metadata()?;

    // For empty files, return with 0 tokens
    if metadata.len() == 0 {
        return Ok(FileData::new(
            path.to_string_lossy().to_string(),
            String::new(),
        ));
    }

    // For small files (< 8KB), use regular read
    if metadata.len() < 8192 {
        let content = std::fs::read_to_string(path)?;
        return Ok(FileData::new(path.to_string_lossy().to_string(), content));
    }

    // For larger files, use memory mapping
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    match std::str::from_utf8(&mmap) {
        Ok(content) => Ok(FileData::new(
            path.to_string_lossy().to_string(),
            content.to_string(),
        )),
        Err(e) => Ok(FileData::with_error(
            path.to_string_lossy().to_string(),
            format!("Invalid UTF-8: {}", e),
        )),
    }
}

pub fn process_files(paths: &[String]) -> Vec<FileData> {
    paths
        .par_iter()
        .map(|path| {
            read_file(Path::new(path)).unwrap_or_else(|e| {
                FileData::with_error(path, format!("Failed to read file: {}", e))
            })
        })
        .collect()
}
