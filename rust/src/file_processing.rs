use tiktoken_rs::{o200k_base, CoreBPE};
use std::fs::File;
use std::path::Path;
use std::sync::LazyLock;
use memmap2::MmapOptions;
use crate::file_data::FileData;

/// A static flag for debugging.
/// This LazyLock boolean checks for the presence of the "CODEMERGE_DEBUG" environment variable.
/// If the environment variable is set, debugging messages will be printed.
static DEBUG: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("CODEMERGE_DEBUG").is_ok()
});

/// A static, lazily-initialized CoreBPE model.
/// This biases the encoding model for tokenizing textual content for processing.
static O200K_BASE: LazyLock<CoreBPE> = LazyLock::new(|| {
    o200k_base().expect("Failed to load O200k base model")
});

/// Counts the number of encoded tokens in a given text string using the CoreBPE model.
///
/// This function utilizes the O200K_BASE static model to encode the text string,
/// returning the length of the encoded token stream.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text content for which the token count is needed.
///
/// # Returns
///
/// * `usize` - The total number of tokens in the provided text.
pub fn count_tokens(text: &str) -> usize {
    let tokens = O200K_BASE
        .encode_with_special_tokens(text)
        .len();
    tokens
}

/// Processes a file given its file path and returns its metadata encapsulated in a `FileData` structure.
///
/// Attempts to read the content of the file, count its tokens, and capture any encountered errors.
/// Utilizes memory-mapping for efficient file reading and performs UTF-8 validation on file content.
///
/// # Arguments
///
/// * `path` - A string slice representing the path of the file to be processed.
///
/// # Returns
///
/// * `FileData` - A structure containing the file path, content (if readable), token count,
///                and any potential error message describing issues encountered during processing.
pub fn process_file(path: &str) -> FileData {
    // Check if the given path points to a valid file.
    if !Path::new(path).is_file() {
        if *DEBUG {
            eprintln!("DEBUG: Path is not a valid file: {}", path);
        }
        return FileData {
            path: path.to_string(),
            content: None,
            tokens: 0,
            error: Some("Path is not a valid file".to_string()),
        };
    }

    // Attempt to open the file at the specified path.
    if *DEBUG {
        eprintln!("DEBUG: Attempting to open file: {}", path);
    }

    match File::open(path) {
        Ok(file) => {
            if *DEBUG {
                eprintln!("DEBUG: Successfully opened file: {}", path);
            }

            // Use memory mapping to load the file content.
            let mmap = unsafe { MmapOptions::new().map(&file) };
            match mmap {
                Ok(mmap) => {
                    if *DEBUG {
                        eprintln!("DEBUG: Successfully memory-mapped file: {}", path);
                    }

                    // Try to parse the memory-mapped content as a UTF-8 string.
                    let text = std::str::from_utf8(&mmap);
                    match text {
                        Ok(content_str) => {
                            if *DEBUG {
                                eprintln!("DEBUG: Successfully read file content as UTF-8");
                                eprintln!("DEBUG: Counting tokens for the file");
                            }

                            let tokens = count_tokens(content_str);
                            if *DEBUG {
                                eprintln!("DEBUG: Total tokens counted: {}", tokens);
                            }

                            FileData {
                                path: path.to_string(),
                                content: Some(content_str.to_string()),
                                tokens,
                                error: None,
                            }
                        }
                        Err(e) => {
                            if *DEBUG {
                                eprintln!("DEBUG: Failed to parse file content as UTF-8: {}", e);
                            }
                            FileData {
                                path: path.to_string(),
                                content: None,
                                tokens: 0,
                                error: Some(format!("Invalid UTF-8 data in file: {}", e)),
                            }
                        }
                    }
                }
                Err(e) => {
                    if *DEBUG {
                        eprintln!("DEBUG: Failed to memory-map file: {}", e);
                    }
                    FileData {
                        path: path.to_string(),
                        content: None,
                        tokens: 0,
                        error: Some(format!("Failed to memory-map file: {}", e)),
                    }
                }
            }
        }
        Err(e) => {
            if *DEBUG {
                eprintln!("DEBUG: Failed to open file: {}", e);
            }
            FileData {
                path: path.to_string(),
                content: None,
                tokens: 0,
                error: Some(format!("Failed to open file: {}", e)),
            }
        }
    }
}