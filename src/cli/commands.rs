use anyhow::Result;
use std::path::Path;

use crate::cli::args::{Cli, Commands};
use crate::config::{self, Config};
use crate::core::{file, tokens, tree};
use crate::utils::{filters, format, finder};

pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
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

            let files = if input {
                finder::read_from_stdin()?
            } else {
                finder::find_files(
                    &path,
                    &merge_patterns(&filter_patterns, &config.filters),
                    &merge_patterns(&ignores, &config.ignores),
                )?
            };

            let processed = file::process_files(&files);
            let filtered = filters::apply_budget_filters(
                processed,
                min_budget,
                max_budget,
                limit_by_high_budget,
                limit_by_low_budget,
            );

            format::output_results(&filtered, &format_type, output)?;
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

            let files = if input {
                finder::read_from_stdin()?
            } else {
                finder::find_files(
                    &path,
                    &merge_patterns(&filter_patterns, &config.filters),
                    &merge_patterns(&ignores, &config.ignores),
                )?
            };

            let processed = file::process_files(&files);
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

            let files = if input {
                finder::read_from_stdin()?
            } else {
                finder::find_files(
                    &path,
                    &merge_patterns(&filter_patterns, &config.filters),
                    &merge_patterns(&ignores, &config.ignores),
                )?
            };

            let processed = file::process_files(&files);
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

        Commands::Init { file_name, force } => {
            init_config(&file_name, force)
        }
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
