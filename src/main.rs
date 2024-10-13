use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write, stdout};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use ignore::{WalkBuilder};
use regex::Regex;
use glob::Pattern;

// Import vfs crate traits
use vfs::{VfsPath, FileSystem, PhysicalFS};

#[derive(StructOpt, Debug)]
#[structopt(name = "codemerge")]
enum CodeMerge {
    #[structopt(name = "merge")]
    Merge {
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        #[structopt(short, long, parse(from_os_str))]
        ignores: Vec<PathBuf>,
        #[structopt(short, long)]
        verbose: bool,
    },
    #[structopt(name = "tokens")]
    Tokens {
        #[structopt(short, long, default_value = "10")]
        count: usize,
        #[structopt(short, long, parse(from_os_str))]
        ignores: Vec<PathBuf>,
        #[structopt(short, long)]
        verbose: bool,
    },
}

fn main() -> io::Result<()> {
    let opt = CodeMerge::from_args();

    match opt {
        CodeMerge::Merge { output, ignores, verbose } => {
            let fs = PhysicalFS::new(PathBuf::from(".")).into();
            merge_files(fs, output.as_deref(), &ignores, verbose)?;
        }
        CodeMerge::Tokens { count, ignores, verbose } => {
            calculate_tokens(count, &ignores, verbose)?;
        }
    }

    Ok(())
}

fn merge_files(fs: VfsPath, output: Option<&Path>, ignores: &[PathBuf], verbose: bool) -> io::Result<()> {
    let mut writer: Box<dyn Write> = match output {
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        None => Box::new(stdout()),
    };

    let mut total_tokens = 0;
    let walker = create_walk_builder(ignores);

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    if verbose {
                        eprintln!("Processing: {}", path.display());
                    }
                    writeln!(writer, "// File: {}", path.display())?;

                    let vfs_path = fs.join(entry.path().to_str().unwrap()).expect("failed to read");
                    let file = vfs_path.open_file().expect("failed to open file");
                    let reader = io::BufReader::new(file);
                    let mut file_tokens = 0;

                    for line in reader.lines() {
                        let line = line?;
                        writeln!(writer, "{}", line)?;
                        file_tokens += count_tokens(&line);
                    }

                    total_tokens += file_tokens;
                    if verbose {
                        eprintln!("Tokens in {}: {}", path.display(), file_tokens);
                    }
                }
            },
            Err(e) => eprintln!("Error processing entry: {}", e),
        }
    }

    writer.flush()?;
    eprintln!("Total tokens: {}", total_tokens);
    if let Some(path) = output {
        eprintln!("Merged files into: {}", path.display());
    }
    Ok(())
}

fn calculate_tokens(count: usize, ignores: &[PathBuf], verbose: bool) -> io::Result<()> {
    let mut file_tokens = Vec::new();
    let mut total_tokens = 0;

    let walker = create_walk_builder(ignores);

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let tokens = count_file_tokens(path)?;
                    total_tokens += tokens;
                    file_tokens.push((path.to_path_buf(), tokens));

                    if verbose {
                        println!("Tokens in {}: {}", path.display(), tokens);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error processing entry: {}", e);
            }
        }
    }

    file_tokens.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Top {} files by token count:", count);
    for (path, tokens) in file_tokens.iter().take(count) {
        println!("{}: {} tokens", path.display(), tokens);
    }
    println!("Total tokens: {}", total_tokens);

    Ok(())
}

fn count_file_tokens(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut tokens = 0;

    for line in reader.lines() {
        tokens += count_tokens(&line?);
    }

    Ok(tokens)
}

fn count_tokens(text: &str) -> usize {
    let re = Regex::new(r"\S+").unwrap();
    re.find_iter(text).count()
}

fn create_walk_builder(ignores: &[PathBuf]) -> ignore::Walk {
    let mut builder = WalkBuilder::new(".");
    let ignore_patterns: Vec<Pattern> = ignores
        .iter()
        .filter_map(|path| path.to_str().and_then(|s| Pattern::new(s).ok()))
        .collect();

    builder.filter_entry(move |entry| {
        let path = entry.path();
        !ignore_patterns.iter().any(|pattern| pattern.matches_path(path))
    });

    builder.build()
}