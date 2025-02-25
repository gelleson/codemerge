use crate::cache;
use crate::cli::args::{CacheOperation, CacheProvider, Cli, Commands};
use crate::config::{self, Config};
use crate::core::{file, tokens, tree};
use crate::error::Result;
use crate::utils::{filters, finder, format};
use std::path::Path;

pub fn execute(cli: Cli) -> Result<()> {
    // Initialize cache if not disabled
    let cache = if !cli.no_cache {
        let provider = match cli.cache_provider {
            CacheProvider::Sqlite => "sqlite",
            CacheProvider::Rocksdb => "rocksdb",
            CacheProvider::None => "none",
        };
        
        let cache = cache::create_cache(provider, cli.cache_dir.clone())?;
        
        // Clear cache if requested
        if cli.clear_cache {
            cache.clear()?;
        }
        
        Some(cache)
    } else {
        None
    };
    
    match cli.command {
        Commands::Cache {
            operation,
            provider,
            dir,
        } => {
            let provider_str = match provider.unwrap_or(cli.cache_provider) {
                CacheProvider::Sqlite => "sqlite",
                CacheProvider::Rocksdb => "rocksdb",
                CacheProvider::None => "none",
            };
            
            // Clone the directory option to avoid ownership issues
            let dir_clone = dir.clone();
            let cache_dir_clone = cli.cache_dir.clone();
            
            let cache = cache::create_cache(provider_str, dir_clone.or(cache_dir_clone.clone()))?;
            
            match operation {
                CacheOperation::Clear => {
                    cache.clear()?;
                    println!("Cache cleared successfully");
                }
                CacheOperation::Info => {
                    println!("Cache provider: {}", provider_str);
                    println!("Cache directory: {}", cache::get_cache_dir(dir.or(cli.cache_dir)).display());
                }
            }
            
            Ok(())
        },
        Commands::Merge {
            path,
            filters: filter_patterns,
            ignores,
            output,
            format: format_type,
            max_budget,
            min_budget,
            limit_by_high_budget,
            limit_by_low_budget,
            context,
            ignore_config,
            config_path,
            input,
        } => {
            let config = if !ignore_config {
                config::load_config(config_path.as_deref(), context.as_deref())?
            } else {
                Config::default()
            };

            let files = if input || finder::has_stdin_pipe() {
                finder::read_from_stdin()?
            } else {
                finder::find_files(
                    &path,
                    &merge_patterns(&filter_patterns, &config.filters),
                    &merge_patterns(&ignores, &config.ignores),
                )?
            };

            let processed = file::process_files(&files, cache.as_ref());
            let filtered = filters::apply_budget_filters(
                processed,
                min_budget,
                max_budget,
                limit_by_high_budget,
                limit_by_low_budget,
            );

            format::output_results(&filtered, &format_type, output)
                .map_err(|e| crate::error::Error::Config(format!("Output error: {}", e)))?;
            Ok(())
        }

        Commands::Tree {
            path,
            filters: filter_patterns,
            ignores,
            max_budget,
            min_budget,
            limit_by_high_budget,
            limit_by_low_budget,
            context,
            ignore_config,
            config_path,
            input,
        } => {
            let config = if !ignore_config {
                config::load_config(config_path.as_deref(), context.as_deref())?
            } else {
                Config::default()
            };

            let files = if input || finder::has_stdin_pipe() {
                finder::read_from_stdin()?
            } else {
                finder::find_files(
                    &path,
                    &merge_patterns(&filter_patterns, &config.filters),
                    &merge_patterns(&ignores, &config.ignores),
                )?
            };

            let processed = file::process_files(&files, cache.as_ref());
            let filtered = filters::apply_budget_filters(
                processed,
                min_budget,
                max_budget,
                limit_by_high_budget,
                limit_by_low_budget,
            );

            let tree_structure = tree::build_tree(&filtered);
            println!("{}", tree::format_tree(&tree_structure, "", true));
            Ok(())
        }

        Commands::Tokens {
            path,
            filters: filter_patterns,
            ignores,
            total,
            max_budget,
            min_budget,
            limit_by_high_budget,
            limit_by_low_budget,
            context,
            ignore_config,
            config_path,
            input,
            format,
        } => {
            let config = if !ignore_config {
                config::load_config(config_path.as_deref(), context.as_deref())?
            } else {
                Config::default()
            };

            let files = if input || finder::has_stdin_pipe() {
                finder::read_from_stdin()?
            } else {
                finder::find_files(
                    &path,
                    &merge_patterns(&filter_patterns, &config.filters),
                    &merge_patterns(&ignores, &config.ignores),
                )?
            };

            let processed = file::process_files(&files, cache.as_ref());
            let filtered = filters::apply_budget_filters(
                processed,
                min_budget,
                max_budget,
                limit_by_high_budget,
                limit_by_low_budget,
            );

            match format.as_str() {
                "plain" => {
                    print!("{}", tokens::format_token_board(&filtered, total));
                }
                "json" => {
                    println!("{}", tokens::format_token_json(&filtered, total));
                }
                _ => unreachable!("Invalid format option"),
            }
            Ok(())
        }

        Commands::Init { file_name, force } => init_config(&file_name, force),
    }
}

fn init_config(file_name: &str, force: bool) -> Result<()> {
    let path = Path::new(file_name);
    if path.exists() && !force {
        println!("File {} already exists.", file_name);
        return Ok(());
    }

    let default_config = r#"version: 1  # CodeMerge configuration format version

contexts:
  - context: default
    filters:
      - "**"
    ignores:
      - ".git/"
      - "*.lock"
      - "node_modules/"
      - "target/"
"#;

    std::fs::write(path, default_config)?;
    println!("Created {}", file_name);
    Ok(())
}

fn merge_patterns(cli_patterns: &[String], config_patterns: &[String]) -> Vec<String> {
    if cli_patterns.is_empty() {
        config_patterns.to_vec()
    } else {
        cli_patterns.to_vec()
    }
}
