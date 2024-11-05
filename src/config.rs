use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use std::fs;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    ParseError(String),
    #[error("Invalid configuration: {0}")]
    ValidationError(String),
    #[error("Unsupported config format: {0}")]
    UnsupportedFormat(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenizerConfig {
    pub model: String,
    pub budget: Option<usize>,
    pub warning_threshold: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputConfig {
    pub format: String,
    pub template: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MergeConfig {
    #[serde(default)]
    pub output_path: Option<String>,
    #[serde(default)]
    pub file_names_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectConfig {
    pub tokenizer: TokenizerConfig,
    pub output: OutputConfig,
    pub ignore_patterns: Vec<String>,
    pub file_filters: Vec<String>,
    pub verbose: bool,
    #[serde(default = "default_top_count")]
    pub top_count: usize,
    #[serde(default)]
    pub merge: MergeConfig,
}

fn default_top_count() -> usize {
    10
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            tokenizer: TokenizerConfig {
                model: "gpt-3.5".to_string(),
                budget: None,
                warning_threshold: Some(0.8),
            },
            output: OutputConfig {
                format: "markdown".to_string(),
                template: None,
                metadata: HashMap::new(),
            },
            ignore_patterns: vec![],
            file_filters: vec![],
            verbose: false,
            top_count: 10,
            merge: MergeConfig::default(),
        }
    }
}

impl ProjectConfig {
    pub fn from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        
        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("yaml") | Some("yml") => {
                serde_yaml::from_str(&content)
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
            }
            Some("toml") => {
                toml::from_str(&content)
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
            }
            Some("json") => {
                serde_json::from_str(&content)
                    .map_err(|e| ConfigError::ParseError(e.to_string()))?
            }
            Some(ext) => return Err(ConfigError::UnsupportedFormat(ext.to_string())),
            None => return Err(ConfigError::UnsupportedFormat("unknown".to_string())),
        };

        Ok(config)
    }

    pub fn merge_with_cli(&mut self, cli_config: CliConfig) {
        // Override config values with CLI arguments if provided
        if let Some(model) = cli_config.model {
            self.tokenizer.model = model;
        }
        if let Some(budget) = cli_config.budget {
            self.tokenizer.budget = Some(budget);
        }
        if let Some(warning_threshold) = cli_config.warning_threshold {
            self.tokenizer.warning_threshold = Some(warning_threshold);
        }
        if let Some(format) = cli_config.format {
            self.output.format = format;
        }
        if !cli_config.metadata.is_empty() {
            self.output.metadata.extend(cli_config.metadata);
        }
        if !cli_config.ignore_patterns.is_empty() {
            self.ignore_patterns = cli_config.ignore_patterns;
        }
        if !cli_config.file_filters.is_empty() {
            self.file_filters = cli_config.file_filters;
        }
        if cli_config.verbose {
            self.verbose = true;
        }
        if let Some(count) = cli_config.top_count {
            self.top_count = count;
        }
        if let Some(output_path) = cli_config.output_path {
            self.merge.output_path = Some(output_path);
        }
        if cli_config.file_names_only {
            self.merge.file_names_only = true;
        }
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(threshold) = self.tokenizer.warning_threshold {
            if !(0.0..=1.0).contains(&threshold) {
                return Err(ConfigError::ValidationError(
                    "Warning threshold must be between 0.0 and 1.0".to_string()
                ));
            }
        }

        match self.output.format.as_str() {
            "json" | "csv" | "markdown" => Ok(()),
            format => Err(ConfigError::ValidationError(
                format!("Unsupported output format: {}", format).to_string()
            )),
        }
    }
}

#[derive(Debug, Default)]
pub struct CliConfig {
    pub model: Option<String>,
    pub budget: Option<usize>,
    pub warning_threshold: Option<f32>,
    pub format: Option<String>,
    pub metadata: HashMap<String, String>,
    pub ignore_patterns: Vec<String>,
    pub file_filters: Vec<String>,
    pub verbose: bool,
    pub top_count: Option<usize>,
    pub output_path: Option<String>,
    pub file_names_only: bool,
} 