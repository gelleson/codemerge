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
    if paths.is_empty() {
        return Vec::new();
    }

    // 1. Get mtimes for all files
    let paths_with_mtime: Vec<(&String, SystemTime)> = paths
        .iter()
        .map(|path| {
            let mtime = match fs::metadata(path) {
                Ok(metadata) => metadata.modified().unwrap_or_else(|_| SystemTime::now()),
                Err(_) => SystemTime::now(),
            };
            (path, mtime)
        })
        .collect();

    // 2. Try to get from cache in batch
    let cached_results = if let Some(cache) = cache {
        let query_paths: Vec<(&str, SystemTime)> = paths_with_mtime
            .iter()
            .map(|(p, m)| (p.as_str(), *m))
            .collect();
        cache.get_file_data_batch(&query_paths)
    } else {
        vec![None; paths.len()]
    };

    // 3. Identify cache misses and their indices
    let mut results = vec![None; paths.len()];
    let mut misses = Vec::new();

    for (i, (cached, (path, mtime))) in cached_results
        .into_iter()
        .zip(paths_with_mtime.into_iter())
        .enumerate()
    {
        if let Some(data) = cached {
            results[i] = Some(data);
        } else {
            misses.push((i, path, mtime));
        }
    }

    if misses.is_empty() {
        return results.into_iter().flatten().collect();
    }

    // 4. Process misses in parallel
    let processed_misses: Vec<(usize, FileData, SystemTime)> = misses
        .into_par_iter()
        .map(|(i, path, mtime)| {
            let file_data = read_file(Path::new(path)).unwrap_or_else(|e| {
                FileData::with_error(path, format!("Failed to read file: {}", e))
            });
            (i, file_data, mtime)
        })
        .collect();

    // 5. Store misses in cache in batch
    if let Some(cache) = cache {
        let store_batch: Vec<(FileData, SystemTime)> = processed_misses
            .iter()
            .map(|(_, data, mtime)| (data.clone(), *mtime))
            .collect();
        let _ = cache.store_file_data_batch(&store_batch);
    }

    // 6. Merge results
    for (i, data, _) in processed_misses {
        results[i] = Some(data);
    }

    results.into_iter().flatten().collect()
}
