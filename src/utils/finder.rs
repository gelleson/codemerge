//! File traversal and discovery.
//!
//! Provides functions to scan directories in parallel while respecting
//! include and ignore glob patterns, as well as helpers to read file
//! lists from standard input.

use crate::error::{Error, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::WalkBuilder;
use std::io::IsTerminal;
use std::path::Path;

/// Build a GlobSet from a list of string patterns
fn build_glob_set(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob = Glob::new(pattern).map_err(|e| {
            Error::Filter(format!("Invalid glob pattern '{}': {}", pattern, e))
        })?;
        builder.add(glob);
    }
    builder
        .build()
        .map_err(|e| Error::Filter(format!("Failed to build glob set: {}", e)))
}

/// Find files using the 'ignore' crate for fast, correct directory traversal
pub fn find_files(
    root: &Path,
    include_patterns: &[String],
    ignore_patterns: &[String],
) -> Result<Vec<String>> {
    let include_set = build_glob_set(include_patterns)?;
    let ignore_set = build_glob_set(ignore_patterns)?;

    let (tx, rx) = std::sync::mpsc::channel();
    
    let mut builder = WalkBuilder::new(root);
    builder.standard_filters(true); // Respect .gitignore, etc.
    builder.hidden(false); // Do not skip hidden files by default, let globs decide
    
    let walker = builder.build_parallel();
    
    walker.run(|| {
        let tx = tx.clone();
        let include_set = &include_set;
        let ignore_set = &ignore_set;
        let root = root.to_path_buf();
        
        Box::new(move |entry| {
            if let Ok(entry) = entry {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    let path = entry.path();
                    let relative = path.strip_prefix(&root).unwrap_or(path);
                    
                    if include_set.is_match(relative) && !ignore_set.is_match(relative) {
                        let _ = tx.send(path.to_string_lossy().to_string());
                    }
                }
            }
            ignore::WalkState::Continue
        })
    });

    drop(tx);
    let files: Vec<String> = rx.into_iter().collect();

    Ok(files)
}

/// Read file names from standard input
pub fn read_from_stdin() -> Result<Vec<String>> {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut files = Vec::new();

    for line in stdin.lock().lines() {
        // Map standard IO errors to our custom error type.
        let line = line.map_err(crate::error::Error::Io)?;
        if !line.trim().is_empty() {
            files.push(line);
        }
    }

    Ok(files)
}

/// Checks if data is being piped into stdin
///
/// Returns `true` if stdin is connected to a pipe rather than an interactive terminal.
/// This is used to automatically detect when input should be read from stdin
/// rather than scanning the filesystem.
pub fn has_stdin_pipe() -> bool {
    !std::io::stdin().is_terminal()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_build_glob_set() -> Result<()> {
        let patterns = vec!["*.rs".to_string(), "src/**/*.txt".to_string()];
        let glob_set = build_glob_set(&patterns)?;

        assert!(glob_set.is_match("test.rs"));
        assert!(glob_set.is_match("src/foo/bar.txt"));
        assert!(!glob_set.is_match("test.js"));
        Ok(())
    }

    #[test]
    fn test_find_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let root = temp_dir.path();

        // Create test files
        fs::create_dir_all(root.join("src"))?;
        fs::write(root.join("src/test.rs"), "")?;
        fs::write(root.join("src/test.txt"), "")?;
        fs::write(root.join("test.js"), "")?;

        let include_patterns = vec!["**/*.rs".to_string()];
        let ignore_patterns = vec!["test.js".to_string()];

        let files = find_files(root, &include_patterns, &ignore_patterns)?;

        assert_eq!(files.len(), 1);
        assert!(files[0].contains("test.rs"));
        Ok(())
    }

    #[test]
    #[cfg_attr(not(ci), ignore)]
    fn test_read_from_stdin() {
        // Note: Testing stdin requires more complex setup with mock stdin
        // This would be implementation-specific based on how you want to test stdin
        assert_eq!(has_stdin_pipe(), true);
    }

    #[test]
    fn test_invalid_glob_pattern() {
        let patterns = vec!["[".to_string()]; // Invalid glob pattern
        assert!(build_glob_set(&patterns).is_err());
    }
}
