use std::path::PathBuf;
use codemerge::{cli::{CodeMerge, Commands}, token_counter, TokenManager, TokenConfig};
use clap::Parser;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use codemerge::file_ops::create_walk_builder;
use chrono::Local;
use codemerge::output_format::{create_formatter, AnalysisReport, TokenAnalysis};
use codemerge::config::{ProjectConfig, CliConfig};
use indicatif::{ProgressBar, ProgressStyle};

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
            config,
            no_config,
        } => {
            // Load and merge configuration
            let project_config = if *no_config {
                ProjectConfig::default()
            } else if let Some(config_path) = config {
                ProjectConfig::from_file(config_path)?
            } else {
                // Look for config files in current directory
                let config_files = [
                    "codemerge.yaml",
                    "codemerge.yml",
                    "codemerge.toml",
                    "codemerge.json",
                ];
                
                config_files
                    .iter()
                    .find(|&file| PathBuf::from(file).exists())
                    .map(|file| ProjectConfig::from_file(&PathBuf::from(file)))
                    .transpose()?
                    .unwrap_or_default()
            };

            let base_path = path.as_ref().map_or_else(
                || PathBuf::from("."),
                |p| p.clone()
            );

            // Use configuration, but let CLI arguments override
            let effective_ignores = if ignores.is_empty() {
                &project_config.ignore_patterns
            } else {
                ignores
            };

            let effective_filters = if filters.is_empty() {
                &project_config.file_filters
            } else {
                filters
            };

            let effective_output = output.clone().or_else(|| {
                project_config.merge.output_path.as_ref().map(|p| PathBuf::from(p))
            });
            

            let effective_file_names = *file_name || project_config.merge.file_names_only;
            let effective_verbose = *verbose || project_config.verbose;

            token_counter::merge_files(
                &base_path,
                effective_output.as_deref(),
                effective_ignores,
                effective_filters,
                effective_verbose,
                effective_file_names,
            )?;
        }
        Commands::Tokens { 
            model,
            budget,
            warning_threshold,
            path: _,
            count,
            ignores,
            filters,
            verbose,
            format,
            output,
            metadata,
            config,
            no_config,
        } => {
            // Load and merge configuration
            let mut project_config = if *no_config {
                ProjectConfig::default()
            } else if let Some(config_path) = config {
                ProjectConfig::from_file(config_path)?
            } else {
                // Look for config files in current directory
                let config_files = [
                    "codemerge.yaml",
                    "codemerge.yml",
                    "codemerge.toml",
                    "codemerge.json",
                ];
                
                config_files
                    .iter()
                    .find(|&file| PathBuf::from(file).exists())
                    .map(|file| ProjectConfig::from_file(&PathBuf::from(file)))
                    .transpose()?
                    .unwrap_or_default()
            };

            // Create CLI config from arguments
            let cli_config = CliConfig {
                model: Some(model.clone()),
                budget: *budget,
                warning_threshold: Some(*warning_threshold),
                format: Some(format.clone()),
                metadata: parse_metadata(metadata),
                ignore_patterns: ignores.clone(),
                file_filters: filters.clone(),
                verbose: *verbose,
                top_count: Some(*count),
                output_path: output.as_ref().map(|p| p.to_string_lossy().to_string()),
                file_names_only: false,
            };

            // Merge CLI arguments with config file
            project_config.merge_with_cli(cli_config);
            project_config.validate()?;

            // Use the merged configuration
            let token_config = TokenConfig {
                model: project_config.tokenizer.model.clone(),
                budget: project_config.tokenizer.budget,
                warning_threshold: project_config.tokenizer.warning_threshold,
            };

            let token_manager = Arc::new(Mutex::new(TokenManager::new(token_config)));
            let file_tokens = Arc::new(Mutex::new(HashMap::new()));
            
            let walker = create_walk_builder(&project_config.ignore_patterns, &project_config.file_filters);
            let entries: Vec<_> = walker
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
                .collect();

            // Initialize progress bar
            let pb = ProgressBar::new(entries.len() as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{msg} [{bar:40}] {percent:>3}% | ETA: {eta} | {pos}/{len} files")
                .progress_chars("##-"));

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
                pb.inc(1); // Increment progress bar
            });

            pb.finish_with_message("Token counting complete.");

            // Sort and display top N files by token count
            let tokens_guard = file_tokens.lock().unwrap();
            let mut files: Vec<(&PathBuf, &usize)> = tokens_guard.iter().collect();
            files.sort_by(|a, b| b.1.cmp(a.1));
            
            println!("\nTop {} files by token count:", project_config.top_count);
            for (path, tokens) in files.iter().take(project_config.top_count) {
                println!("{}: {} tokens", path.display(), tokens);
            }
            
            let total_usage = token_manager.lock().unwrap().get_usage();
            println!("\nTotal token usage: {}", total_usage);
            
            if let Some(budget_val) = project_config.tokenizer.budget {
                println!("Budget remaining: {}", 
                    budget_val.saturating_sub(total_usage));
            }
            
            // Create analysis report
            let report = AnalysisReport {
                files: tokens_guard.iter()
                    .map(|(path, &count)| TokenAnalysis {
                        file_path: path.clone(),
                        token_count: count,
                        metadata: project_config.output.metadata.clone(),
                    })
                    .collect(),
                total_tokens: total_usage,
                budget: project_config.tokenizer.budget,
                budget_remaining: project_config.tokenizer.budget.map(|b| b.saturating_sub(total_usage)),
                model: project_config.tokenizer.model,
                timestamp: Local::now().to_rfc3339(),
            };
            
            // Format and output
            let formatter = create_formatter(&project_config.output.format)?;
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

fn parse_metadata(metadata: &[String]) -> HashMap<String, String> {
    metadata.iter()
        .filter_map(|s| {
            let parts: Vec<&str> = s.split('=').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect()
}
