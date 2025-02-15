// src/config/mod.rs

use crate::error::{Error, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub filters: Vec<String>,
    pub ignores: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    version: u8,
    contexts: Vec<Context>,
}

#[derive(Debug, Deserialize)]
struct Context {
    context: String,
    #[serde(default = "default_filters")]
    filters: Vec<String>,
    #[serde(default)]
    ignores: Vec<String>,
}

fn default_filters() -> Vec<String> {
    vec!["**".to_string()]
}

pub fn load_config(config_path: Option<&Path>, context_name: Option<&str>) -> Result<Config> {
    let config_path = config_path.unwrap_or_else(|| Path::new(".codemerge.yaml"));

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(config_path)?;
    let config: ConfigFile = serde_yaml::from_str(&content)?;

    if config.version != 1 {
        return Err(Error::Config(format!(
            "Unsupported config version: {}",
            config.version
        )));
    }

    let context_name = context_name.unwrap_or("default");
    let context = config
        .contexts
        .iter()
        .find(|c| c.context == context_name)
        .ok_or_else(|| Error::Config(format!("Context '{}' not found in config", context_name)))?;

    Ok(Config {
        filters: context.filters.clone(),
        ignores: context.ignores.clone(),
    })
}
