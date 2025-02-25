 use super::tokens::count_tokens;
use crate::cache::Cache;
use crate::error::Result;
use memmap2::MmapOptions;
use rayon::prelude::*;
use serde::Serialize;
use std::fs::{self, File};
use std::path::Path;
use std::time::SystemTime;

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

/// Process a list of files, using the cache if available
pub fn process_files(paths: &[String], cache: Option<&Box<dyn Cache>>) -> Vec<FileData> {
    paths
        .par_iter()
        .map(|path| {
            let path_obj = Path::new(path);
            
            // Get file modification time
            let mtime = match fs::metadata(path_obj) {
                Ok(metadata) => metadata.modified().unwrap_or_else(|_| SystemTime::now()),
                Err(_) => SystemTime::now(),
            };
            
            // Try to get from cache first if cache is available
            if let Some(cache) = cache {
                if let Some(cached_data) = cache.get_file_data(path, mtime) {
                    return cached_data;
                }
            }
            
            // Cache miss or no cache, read the file
            let file_data = read_file(path_obj).unwrap_or_else(|e| {
                FileData::with_error(path, format!("Failed to read file: {}", e))
            });
            
            // Store in cache if available
            if let Some(cache) = cache {
                let _ = cache.store_file_data(&file_data, mtime);
            }
            
            file_data
        })
        .collect()
}
