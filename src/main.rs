use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write, stdout};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use ignore::{WalkBuilder};
use glob::Pattern;
use tiktoken_rs::{o200k_base, CoreBPE};
use vfs::{VfsPath, FileSystem, PhysicalFS};
use rayon::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "codemerge")]
enum CodeMerge {
    #[structopt(name = "merge")]
    Merge {
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        #[structopt(short, long, parse(from_os_str))]
        ignores: Vec<PathBuf>,
        #[structopt(short = "f", long = "filter")]
        filters: Vec<String>,
        #[structopt(short, long)]
        verbose: bool,
    },
    #[structopt(name = "tokens")]
    Tokens {
        #[structopt(short, long, default_value = "10")]
        count: usize,
        #[structopt(short, long, parse(from_os_str))]
        ignores: Vec<PathBuf>,
        #[structopt(short = "f", long = "filter")]
        filters: Vec<String>,
        #[structopt(short, long)]
        verbose: bool,
    },
}

fn main() -> io::Result<()> {
    let opt = CodeMerge::from_args();
    let bpe = o200k_base().expect("Failed to load BPE model");

    match opt {
        CodeMerge::Merge { output, ignores, filters, verbose } => {
            let fs = PhysicalFS::new(PathBuf::from(".")).into();
            merge_files(fs, output.as_deref(), &ignores, &filters, verbose)?;
        }
        CodeMerge::Tokens { count, ignores, filters, verbose } => {
            calculate_tokens(count, &ignores, &filters, verbose, &bpe)?;
        }
    }

    Ok(())
}

fn merge_files(fs: VfsPath, output: Option<&Path>, ignores: &[PathBuf], filters: &[String], verbose: bool) -> io::Result<()> {
    let writer: Arc<Mutex<Box<dyn Write + Send + Sync>>> = Arc::new(Mutex::new(match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(stdout()),
    }));

    let total_tokens = Arc::new(Mutex::new(0));
    let walker = create_walk_builder(ignores, filters);
    let bpe = o200k_base().expect("Failed to load BPE model");

    walker.into_iter().par_bridge().for_each(|entry| {
        let writer = Arc::clone(&writer);

        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    if verbose {
                        eprintln!("Processing: {}", path.display());
                    }
                    writeln!(writer.lock().unwrap(), "// File: {}", path.display()).unwrap();

                    let vfs_path = fs.join(entry.path().to_str().unwrap()).expect("failed to read");
                    let file = vfs_path.open_file().expect("failed to open file");
                    let reader = io::BufReader::new(file);
                    let mut file_tokens = 0;

                    for line in reader.lines() {
                        let line = line.unwrap();
                        writeln!(writer.lock().unwrap(), "{}", line).unwrap();
                        file_tokens += count_tokens(&line, &bpe);
                    }

                    let mut total = total_tokens.lock().unwrap();
                    *total += file_tokens;
                    if verbose {
                        eprintln!("Tokens in {}: {}", path.display(), file_tokens);
                    }
                }
            }
            Err(e) => eprintln!("Error processing entry: {}", e),
        }
    });

    writer.lock().unwrap().flush()?;
    let total = total_tokens.lock().unwrap();
    eprintln!("Total tokens: {}", *total);
    if let Some(path) = output {
        eprintln!("Merged files into: {}", path.display());
    }
    Ok(())
}

fn calculate_tokens(count: usize, ignores: &[PathBuf], filters: &[String], verbose: bool, bpe: &CoreBPE) -> io::Result<()> {
    let file_tokens: Arc<Mutex<HashMap<PathBuf, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let total_tokens = Arc::new(Mutex::new(0));

    let walker = create_walk_builder(ignores, filters);

    walker.into_iter().par_bridge().for_each(|entry| {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    let tokens = count_file_tokens(path, &bpe).unwrap();
                    let mut file_tokens = file_tokens.lock().unwrap();
                    file_tokens.insert(path.to_path_buf(), tokens);

                    let mut total = total_tokens.lock().unwrap();
                    *total += tokens;

                    if verbose {
                        println!("Tokens in {}: {}", path.display(), tokens);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error processing entry: {}", e);
            }
        }
    });

    let mut file_tokens = file_tokens.lock().unwrap().clone().into_iter().collect::<Vec<_>>();
    file_tokens.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Top {} files by token count:", count);
    for (path, tokens) in file_tokens.iter().take(count) {
        println!("{}: {} tokens", path.display(), tokens);
    }

    let total = total_tokens.lock().unwrap();
    println!("Total tokens: {}", *total);

    Ok(())
}

fn count_file_tokens(path: &Path, bpe: &CoreBPE) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut tokens = 0;

    for line in reader.lines() {
        tokens += count_tokens(&line?, &bpe);
    }

    Ok(tokens)
}

fn count_tokens(text: &str, bpe: &CoreBPE) -> usize {
    bpe.encode(text, Default::default()).len()
}

fn create_walk_builder(ignores: &[PathBuf], filters: &[String]) -> ignore::Walk {
    let mut builder = WalkBuilder::new(".");
    let ignore_patterns: Vec<Pattern> = ignores
        .iter()
        .filter_map(|path| path.to_str().and_then(|s| Pattern::new(s).ok()))
        .collect();

    let filter_patterns: Vec<glob::Pattern> = filters
        .iter()
        .filter_map(|f| glob::Pattern::new(f).ok())
        .collect();

    builder.filter_entry(move |entry| {
        let path = entry.path();

        // Always include directories to traverse into them
        if entry.file_type().map_or(false, |ft| ft.is_dir()) {
            return true;
        }

        // For files, apply ignore and filter patterns
        let not_ignored = !ignore_patterns.iter().any(|pattern| pattern.matches_path(path));
        let passes_filter = filter_patterns.is_empty() || filter_patterns.iter().any(|pattern| pattern.matches_path(path));

        not_ignored && passes_filter
    });

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_count_tokens() {
        // Initialize the BPE model
        let bpe = o200k_base().expect("Failed to load BPE model");
        let text = "Hello, world!";

        // Count tokens in the text
        let tokens = count_tokens(text, &bpe);

        // Assert the expected number of tokens
        // Note: The expected token count may vary based on the BPE model
        assert_eq!(tokens, 4); // Adjust as necessary
    }

    #[test]
    fn test_count_file_tokens() -> io::Result<()> {
        // Initialize the BPE model
        let bpe = o200k_base().expect("Failed to load BPE model");

        // Create a temporary directory
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.txt");

        // Write sample content to a temporary file
        let mut file = File::create(&file_path)?;
        writeln!(file, "Hello, world!")?;
        writeln!(file, "This is a test file.")?;

        // Count tokens in the file
        let tokens = count_file_tokens(&file_path, &bpe)?;

        // Assert that tokens were counted
        assert!(tokens > 0);
        Ok(())
    }

    #[test]
    fn test_create_walk_builder() -> io::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        // Create files and directories
        let file_rs = dir_path.join("file.rs");
        File::create(&file_rs)?;

        let file_txt = dir_path.join("file.txt");
        File::create(&file_txt)?;

        let sub_dir = dir_path.join("subdir");
        fs::create_dir(&sub_dir)?;
        let file_in_subdir = sub_dir.join("subfile.rs");
        File::create(&file_in_subdir)?;

        // Set up filters and ignores
        let filters = vec!["**/*.rs".to_string()];
        let ignores: Vec<PathBuf> = vec![];

        // Create the walk builder
        let walker = WalkBuilder::new(dir_path)
            .filter_entry(move |entry| {
                let path = entry.path();

                // Always include directories to traverse into them
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    return true;
                }

                let not_ignored = true; // No ignore patterns for this test
                let filter_patterns: Vec<glob::Pattern> = filters
                    .iter()
                    .filter_map(|f| glob::Pattern::new(f).ok())
                    .collect();
                let passes_filter = filter_patterns.is_empty()
                    || filter_patterns.iter().any(|pattern| pattern.matches_path(path));

                not_ignored && passes_filter
            })
            .build();

        // Collect the files found by the walker
        let files_found: Vec<_> = walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().strip_prefix(dir_path).unwrap().to_path_buf())
            .collect();

        let expected_files = vec![
            PathBuf::from("file.rs"),
            PathBuf::from("subdir/subfile.rs"),
        ];

        // Assert the expected files are found
        assert_eq!(files_found.len(), expected_files.len());
        for expected in expected_files {
            assert!(files_found.contains(&expected));
        }

        Ok(())
    }


    #[test]
    fn test_calculate_tokens() -> io::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let dir_path = temp_dir.path();

        // Create some test files
        let file1 = dir_path.join("file1.rs");
        let mut f1 = File::create(&file1)?;
        writeln!(f1, "fn main() {{ println!(\"Hello from file1\"); }}")?;

        let file2 = dir_path.join("file2.rs");
        let mut f2 = File::create(&file2)?;
        writeln!(f2, "fn helper() {{ println!(\"Hello from file2\"); }}")?;

        // Set up filters
        let filters = vec!["**/*.rs".to_string()];
        let ignores: Vec<PathBuf> = vec![];        let verbose = false;
        let count = 2;
        let bpe = o200k_base().expect("Failed to load BPE model");

        // Prepare the data structures
        let file_tokens: Arc<Mutex<HashMap<PathBuf, usize>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let total_tokens = Arc::new(Mutex::new(0));

        // Create the walker
        let walker = WalkBuilder::new(dir_path)
            .filter_entry(move |entry| {
                let path = entry.path();

                // Always include directories to traverse into them
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    return true;
                }

                let not_ignored = true; // No ignore patterns for this test
                let filter_patterns: Vec<glob::Pattern> = filters
                    .iter()
                    .filter_map(|f| glob::Pattern::new(f).ok())
                    .collect();
                let passes_filter = filter_patterns.is_empty()
                    || filter_patterns.iter().any(|pattern| pattern.matches_path(path));

                not_ignored && passes_filter
            })
            .build();

        // Process the entries
        walker.into_iter().for_each(|entry| {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        let tokens = count_file_tokens(path, &bpe).unwrap();
                        let mut file_tokens_map = file_tokens.lock().unwrap();
                        file_tokens_map.insert(path.to_path_buf(), tokens);

                        let mut total = total_tokens.lock().unwrap();
                        *total += tokens;
                    }
                }
                Err(e) => eprintln!("Error processing entry: {}", e),
            }
        });

        // Assertions
        let file_tokens_map = file_tokens.lock().unwrap();
        assert_eq!(file_tokens_map.len(), 2);

        // Check that tokens are counted for each file
        for (path, tokens) in file_tokens_map.iter() {
            assert!(tokens > &0);
            assert!(path == &file1 || path == &file2);
        }

        // Check total tokens
        let total = total_tokens.lock().unwrap();
        assert!(*total > 0);

        Ok(())
    }
}