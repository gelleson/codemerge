// lib.rs

use rayon::prelude::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::LazyLock;

mod file_data;
mod file_processing;
mod gitignore;

use crate::file_data::FileData;
use crate::file_processing::process_file;
use crate::gitignore::find_gitignore_files;

/// A static flag for debugging.
/// This LazyLock boolean checks for the presence of the "CODEMERGE_DEBUG" environment variable.
/// If the environment variable is set, debugging messages will be printed.
static DEBUG: LazyLock<bool> = LazyLock::new(|| std::env::var("CODEMERGE_DEBUG").is_ok());

/// External C function to find `.gitignore` files starting from a given directory path provided as a C string.
///
/// This function exposes Rust functionality to C by taking a C string representing the path.
///
/// # Safety
///
/// This function is marked as `unsafe` since it involves raw pointer manipulation, which is inherent when interfacing with C.
/// The caller must guarantee that the provided `path_ptr` is a valid, null-terminated C string.
///
/// # Arguments
///
/// * `path_ptr` - A pointer to a C string which contains the directory path.
///
/// # Returns
///
/// * A pointer to a C string containing JSON data with found `.gitignore` files and any error message.
#[no_mangle]
pub extern "C" fn find_gitignore_files_json(path_ptr: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(path_ptr) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            if *DEBUG {
                eprintln!("DEBUG: Invalid UTF-8 in provided path");
            }
            let error =
                CString::new("{\"gitignore_files\":[],\"error\":\"Invalid UTF-8 in path\"}")
                    .unwrap();
            return error.into_raw();
        }
    };

    if *DEBUG {
        eprintln!(
            "DEBUG: Searching for .gitignore files in path: {}",
            path_str
        );
    }
    let result = find_gitignore_files(path_str);

    let json_string = serde_json::to_string(&result).unwrap_or_else(|e| {
        if *DEBUG {
            eprintln!("DEBUG: Failed to serialize result: {}", e);
        }
        format!(
            "{{\"gitignore_files\":[],\"error\":\"Failed to serialize result: {}\"}}",
            e
        )
    });
    let c_string = CString::new(json_string).unwrap();
    c_string.into_raw()
}

/// External C function to read multiple files and gather their metadata, provided as a newline-separated C string of paths.
///
/// Processes each file to gather metadata and encode it as JSON.
///
/// # Safety
///
/// This function is `unsafe` due to raw pointer manipulation with C strings.
/// The caller must ensure `paths_ptr` is valid and null-terminated.
///
/// # Arguments
///
/// * `paths_ptr` - A pointer to a C string containing newline-separated file paths.
///
/// # Returns
///
/// * A pointer to a C string containing JSON data with metadata for each file or an error message.
#[no_mangle]
pub extern "C" fn read_files_json(paths_ptr: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(paths_ptr) };
    let paths_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            if *DEBUG {
                eprintln!("DEBUG: Invalid UTF-8 in provided paths");
            }
            return CString::new("{\"error\":\"Invalid UTF-8 in paths\"}")
                .unwrap()
                .into_raw();
        }
    };

    if *DEBUG {
        eprintln!("DEBUG: Processing read request for paths.");
    }
    let paths: Vec<&str> = paths_str.lines().collect();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get_physical())
        .build()
        .unwrap();

    let results: Vec<FileData> = pool.install(|| {
        paths
            .par_iter()
            .map(|&path| {
                if *DEBUG {
                    eprintln!("DEBUG: Processing file at path: {}", path);
                }
                process_file(path)
            })
            .collect()
    });

    let json_string = serde_json::to_string(&results).unwrap_or_else(|e| {
        if *DEBUG {
            eprintln!("DEBUG: Failed to serialize file data results: {}", e);
        }
        format!("{{\"error\":\"Failed to serialize result: {}\"}}", e)
    });
    let c_string = CString::new(json_string).unwrap();
    c_string.into_raw()
}

/// Frees the memory allocated for a C string created by Rust functions.
///
/// # Safety
///
/// This function takes ownership of the pointer and deallocates the associated memory.
/// The caller must ensure that this pointer was originally allocated by Rust.
///
/// # Arguments
///
/// * `s` - A pointer to a C string allocated by a Rust function.
#[no_mangle]
pub extern "C" fn free_json_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            if *DEBUG {
                eprintln!("DEBUG: Freeing JSON string memory.");
            }
            let _ = CString::from_raw(s); // Will deallocate the memory once it goes out of scope
        }
    }
}
