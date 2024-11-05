use std::path::PathBuf;
use codemerge::{cli::{CodeMerge, Commands}, token_counter, TokenManager, TokenConfig};
use clap::Parser;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use codemerge::file_ops::create_walk_builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CodeMerge::parse();

    match &cli.command {
        Commands::Merge {
            path,
            output,
            ignores,
            filters,
            verbose,
            file_name,
        } => {
            let base_path = path.as_ref().map_or_else(
                || PathBuf::from("."),
                |p| p.clone()
            );
            token_counter::merge_files(
                &base_path,
                output.as_deref(),
                ignores,
                filters,
                *verbose,
                *file_name,
            )?;
        }
        Commands::Tokens { 
            model, 
            budget, 
            warning_threshold,
            path: _,  // Ignoring path as we're using current directory
            count,
            ignores,
            filters,
            verbose,
        } => {
            let config = TokenConfig {
                model: model.clone(),
                budget: *budget,
                warning_threshold: Some(*warning_threshold),
            };
            
            let token_manager = Arc::new(Mutex::new(TokenManager::new(config)));
            let file_tokens = Arc::new(Mutex::new(HashMap::new()));
            
            let walker = create_walk_builder(ignores, filters);
            let entries: Vec<_> = walker
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
                .collect();

            entries.into_par_iter().for_each(|entry| {
                let path = entry.path().to_path_buf();
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let mut token_manager = token_manager.lock().unwrap();
                    match token_manager.count_tokens(&content) {
                        Ok(count) => {
                            if *verbose {
                                println!("{}: {} tokens", path.display(), count);
                            }
                            file_tokens.lock().unwrap().insert(path, count);
                        },
                        Err(e) => eprintln!("Error processing {}: {}", path.display(), e),
                    }
                }
            });
            
            // Sort and display top N files by token count
            let tokens_guard = file_tokens.lock().unwrap();
            let mut files: Vec<(&PathBuf, &usize)> = tokens_guard.iter().collect();
            files.sort_by(|a, b| b.1.cmp(a.1));
            
            println!("\nTop {} files by token count:", count);
            for (path, tokens) in files.iter().take(*count) {
                println!("{}: {} tokens", path.display(), tokens);
            }
            
            let total_usage = token_manager.lock().unwrap().get_usage();
            println!("\nTotal token usage: {}", total_usage);
            
            if let Some(budget_val) = budget {
                println!("Budget remaining: {}", 
                    budget_val.saturating_sub(total_usage));
            }
        }
    }

    Ok(())
}
