use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tiktoken_rs::CoreBPE;
use glob::Pattern;

pub fn merge_files(
    base_path: &Path,
    output: Option<&Path>,
    ignores: &[String], // Change to String for glob patterns
    filters: &[String],
    verbose: bool,
    file_name: bool,
) -> io::Result<()> {
    if !base_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Base path does not exist"));
    }

    let writer: Arc<Mutex<Box<dyn Write + Send + Sync>>> = Arc::new(Mutex::new(match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(io::stdout()),
    }));

    let total_tokens = Arc::new(Mutex::new(0));
    let entries = get_filtered_entries(base_path, ignores, filters)?;

    // Process files in parallel
    entries.into_par_iter().for_each(|entry| {
        let writer = Arc::clone(&writer);
        let path = entry.path();
        if verbose {
            eprintln!("Processing: {}", path.display());
        }

        if file_name {
            writeln!(writer.lock().unwrap(), "{}", path.display()).unwrap();
        } else {
            // Write file header
            writeln!(writer.lock().unwrap(), "// File: {}", path.display()).unwrap();

            // Get relative path and read contents
            let relative_path = path.to_string_lossy().to_string();

            match read_file_contents(&relative_path) {
                Ok((contents, file_tokens)) => {
                    // Write file contents immediately after header
                    writeln!(writer.lock().unwrap(), "{}", contents).unwrap();
                    
                    let mut total = total_tokens.lock().unwrap();
                    *total += file_tokens;
                    if verbose {
                        eprintln!("Tokens in {}: {}", path.display(), file_tokens);
                    }
                }
                Err(e) => eprintln!("Error reading file {}: {}", path.display(), e),
            }
        }
    });

    writer.lock().unwrap().flush()?;
    let total = total_tokens.lock().unwrap();
    if verbose && !file_name {
        eprintln!("Total tokens: {}", *total);
    }
    if let Some(path) = output {
        eprintln!("Merged files into: {}", path.display());
    }
    Ok(())
}

fn get_filtered_entries(
    base_path: &Path,
    ignores: &[String],
    filters: &[String],
) -> io::Result<Vec<fs::DirEntry>> {
    let mut entries = Vec::new();

    if base_path.exists() {
        for entry in fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Recursively search subdirectories
                let mut subdir_entries = get_filtered_entries(&path, ignores, filters)?;
                entries.append(&mut subdir_entries);
            } else if path.is_file() {
                // Use the full path string for pattern matching
                let path_str = path.to_string_lossy().to_string();
                
                // Check if the path matches any of the filters using glob patterns
                let matches_filter = filters.is_empty() || filters.iter().any(|f| {
                    let pattern = Pattern::new(f).unwrap();
                    pattern.matches(&path_str)
                });

                // Check if the path is ignored using glob patterns
                let matches_ignore = ignores.iter().any(|i| {
                    let pattern = Pattern::new(i).unwrap();
                    pattern.matches(&path_str)
                });

                if matches_filter && !matches_ignore {
                    entries.push(entry);
                }
            }
        }
    }

    Ok(entries)
}

fn read_file_contents(path: &str) -> Result<(String, usize), io::Error> {
    println!("Attempting to read file at path: {}", path);

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();
    let mut tokens = 0;

    let bpe = tiktoken_rs::o200k_base().expect("Failed to load BPE model");
    for line in reader.lines() {
        let line = line?;
        contents.push_str(&line);
        contents.push('\n');
        tokens += count_tokens(&line, &bpe);
    }

    Ok((contents, tokens))
}

pub fn calculate_tokens(
    count: usize,
    ignores: &[String],
    filters: &[String],
    verbose: bool,
    bpe: &CoreBPE,
) -> io::Result<()> {
    let file_tokens: Arc<Mutex<HashMap<PathBuf, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let total_tokens = Arc::new(Mutex::new(0));

    let entries = get_filtered_entries(Path::new("."), ignores, filters)?;

    entries
        .into_par_iter()
        .for_each(|entry| {
            let path = entry.path();
            let cloned_path = path.clone();
            let tokens = count_file_tokens(&path, &bpe).unwrap();
            let mut file_tokens = file_tokens.lock().unwrap();
            file_tokens.insert(path, tokens);

            let mut total = total_tokens.lock().unwrap();
            *total += tokens;

            if verbose {
                println!("Tokens in {}: {}", cloned_path.clone().to_string_lossy(), tokens);
            }
        });

    let mut file_tokens = file_tokens
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .collect::<Vec<_>>();
    file_tokens.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Top {} files by token count:", count);
    for (path, tokens) in file_tokens.iter().take(count) {
        println!("{}: {} tokens", path.display(), tokens);
    }

    let total = total_tokens.lock().unwrap();
    println!("Total tokens: {}", *total);

    Ok(())
}

pub fn count_file_tokens(path: &Path, bpe: &CoreBPE) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut tokens = 0;

    for line in reader.lines() {
        tokens += count_tokens(&line?, &bpe);
    }

    Ok(tokens)
}

pub fn count_tokens(text: &str, bpe: &CoreBPE) -> usize {
    bpe.encode(text, Default::default()).len()
} 