use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tiktoken_rs::CoreBPE;
use glob::Pattern;
use crate::file_ops::create_walk_builder;

pub fn merge_files(
    base_path: &Path,
    output: Option<&Path>,
    ignores: &[String],
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
    
    let walker = create_walk_builder(ignores, filters);
    let entries: Vec<_> = walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .collect();

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
            // Read contents
            match read_file_contents(&path.to_string_lossy()) {
                Ok((contents, file_tokens)) => {
                    writeln!(writer.lock().unwrap(), "// File: {}", path.display()).unwrap();
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

fn read_file_contents(path: &str) -> Result<(String, usize), io::Error> {
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

    let walker = create_walk_builder(ignores, filters);
    let entries: Vec<_> = walker
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .collect();

    entries
        .into_par_iter()
        .for_each(|entry| {
            let path = entry.path();
            let cloned_path = path;
            let tokens = count_file_tokens(&path, &bpe).unwrap();
            let mut file_tokens = file_tokens.lock().unwrap();
            file_tokens.insert(path.to_path_buf(), tokens);

            let mut total = total_tokens.lock().unwrap();
            *total += tokens;

            if verbose {
                println!("Tokens in {}: {}", cloned_path.to_string_lossy(), tokens);
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