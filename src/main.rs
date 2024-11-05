use std::path::PathBuf;
use codemerge::{cli::{CodeMerge, Commands}, token_counter, TokenManager, TokenConfig};
use clap::Parser;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use codemerge::file_ops::create_walk_builder;
use chrono::Local;
use codemerge::output_format::{create_formatter, AnalysisReport, TokenAnalysis};

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
            format,
            output,
            metadata,
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
            
            // Parse metadata
            let metadata_map: HashMap<String, String> = metadata.iter()
                .filter_map(|s| {
                    let parts: Vec<&str> = s.split('=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect();
            
            // Create analysis report
            let report = AnalysisReport {
                files: tokens_guard.iter()
                    .map(|(path, &count)| TokenAnalysis {
                        file_path: path.clone(),
                        token_count: count,
                        metadata: metadata_map.clone(),
                    })
                    .collect(),
                total_tokens: total_usage,
                budget: *budget,
                budget_remaining: budget.map(|b| b.saturating_sub(total_usage)),
                model: model.clone(),
                timestamp: Local::now().to_rfc3339(),
            };
            
            // Format and output
            let formatter = create_formatter(format)?;
            let output_str = formatter.format(&report)?;
            
            if let Some(output_path) = output {
                std::fs::write(output_path, output_str)?;
                println!("Report written to: {}", output_path.display());
            } else {
                println!("{}", output_str);
            }
        }
    }

    Ok(())
}
