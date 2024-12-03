// gitignore_finder.rs

use std::path::Path;
use std::sync::LazyLock;
use walkdir::WalkDir;
use serde::Serialize;

/// A static flag for debugging.
/// This LazyLock boolean checks for the presence of the "CODEMERGE_DEBUG" environment variable.
/// If the environment variable is set, debugging messages will be printed.
static DEBUG: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("CODEMERGE_DEBUG").is_ok()
});


/// Represents the result of searching for `.gitignore` files.
///
/// Contains a list of `.gitignore` file paths and an optional error message if the search
/// encounters any issues.
#[derive(Serialize)]
pub struct GitignoreFilesResult {
    /// A vector of strings containing the relative paths to `.gitignore` files found.
    pub gitignore_files: Vec<String>,

    /// Optional error message. `None` means no errors occurred during the search.
    pub error: Option<String>,
}

/// Searches recursively for `.gitignore` files starting from a given directory path.
///
/// Utilizes the `walkdir` crate to traverse directories and identify `.gitignore` files.
/// Provides a result that includes the set of found file paths and any errors encountered if the path is invalid.
///
/// # Arguments
///
/// * `start_path` - A string slice representing the directory path to begin the search.
///
/// # Returns
///
/// * `GitignoreFilesResult` - A structure containing the list of found `.gitignore` files and any potential error message.
pub fn find_gitignore_files(start_path: &str) -> GitignoreFilesResult {
    let mut gitignore_files = Vec::new();
    let start_path = Path::new(start_path);

    // Check if the provided start path is a directory.
    if !start_path.is_dir() {
        if *DEBUG {
            eprintln!("DEBUG: Provided path is not a directory: {}", start_path.display());
        }
        return GitignoreFilesResult {
            gitignore_files: vec![],
            error: Some("Provided path is not a directory".to_string()),
        };
    }

    // Iterate through directory entries and find `.gitignore` files.
    if *DEBUG {
        eprintln!("DEBUG: Starting to walk directory: {}", start_path.display());
    }

    for entry in WalkDir::new(start_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == ".gitignore")
    {
        let path = entry.path().to_path_buf();
        if let Ok(relative_path) = path.strip_prefix(start_path) {
            if *DEBUG {
                eprintln!("DEBUG: Found .gitignore file at relative path: {}", relative_path.display());
            }
            gitignore_files.push(relative_path.to_string_lossy().to_string());
        } else {
            if *DEBUG {
                eprintln!("DEBUG: Found .gitignore file with path: {}", path.display());
            }
            gitignore_files.push(path.to_string_lossy().to_string());
        }
    }

    if *DEBUG {
        eprintln!("DEBUG: Completed directory walk. Total .gitignore files found: {}", gitignore_files.len());
    }

    GitignoreFilesResult {
        gitignore_files,
        error: None,
    }
}