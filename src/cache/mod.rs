mod providers;
mod trait_def;

pub use providers::{NoneCache, RocksDBCache, SQLiteCache};
pub use trait_def::{Cache, CacheConfig};

use crate::error::Result;
use std::path::PathBuf;
use std::fs;

/// Get the cache directory path
pub fn get_cache_dir(cache_dir: Option<PathBuf>) -> PathBuf {
    cache_dir.unwrap_or_else(|| {
        dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from(".cache"))
            .join("codemerge")
    })
}

/// Factory function to create a cache instance based on the provider name
pub fn create_cache(provider: &str, cache_dir: Option<PathBuf>) -> Result<Box<dyn Cache>> {
    let cache_dir = get_cache_dir(cache_dir);
    
    // Ensure the cache directory exists
    fs::create_dir_all(&cache_dir).map_err(|e| {
        crate::error::Error::Cache(format!(
            "Failed to create cache directory {}: {}",
            cache_dir.display(),
            e
        ))
    })?;
    
    let config = CacheConfig {
        cache_dir,
    };

    match provider.to_lowercase().as_str() {
        "sqlite" => Ok(Box::new(SQLiteCache::new(config)?)),
        "rocksdb" => Ok(Box::new(RocksDBCache::new(config)?)),
        "none" => Ok(Box::new(NoneCache::new(config)?)),
        _ => Err(crate::error::Error::Config(format!(
            "Unknown cache provider: {}",
            provider
        ))),
    }
}
