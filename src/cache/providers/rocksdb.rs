use crate::cache::trait_def::{Cache, CacheConfig};
use crate::core::file::FileData;
use crate::error::{Error, Result};
use rocksdb::{Options, DB, IteratorMode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// RocksDB-based cache implementation
pub struct RocksDBCache {
    db: Arc<DB>,
}

/// Cached file entry for RocksDB
#[derive(Serialize, Deserialize)]
struct CachedFileEntry {
    content: String,
    tokens: usize,
    mtime: u64,
    error: Option<String>,
}

impl RocksDBCache {
    /// Convert SystemTime to seconds since UNIX epoch
    fn system_time_to_timestamp(time: SystemTime) -> u64 {
        time.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Convert seconds since UNIX epoch to SystemTime
    #[allow(dead_code)]
    fn timestamp_to_system_time(timestamp: u64) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::from_secs(timestamp)
    }

    /// Create a cache key for a file path
    fn create_key(path: &str) -> Vec<u8> {
        format!("file:{}", path).into_bytes()
    }
}

impl Cache for RocksDBCache {
    fn new(config: CacheConfig) -> Result<Self> {
        // Cache directory is already created by the factory function

        let db_path = config.cache_dir.join("rocksdb");
        
        // Configure RocksDB options
        let mut options = Options::default();
        options.create_if_missing(true);
        options.set_keep_log_file_num(10);
        options.set_max_total_wal_size(64 * 1024 * 1024); // 64MB
        options.set_write_buffer_size(64 * 1024 * 1024); // 64MB
        
        // Open the database
        let db = DB::open(&options, &db_path).map_err(|e| {
            Error::Config(format!(
                "Failed to open RocksDB database {}: {}",
                db_path.display(),
                e
            ))
        })?;

        Ok(RocksDBCache {
            db: Arc::new(db),
        })
    }

    fn get_file_data(&self, path: &str, mtime: SystemTime) -> Option<FileData> {
        let key = Self::create_key(path);
        let mtime_ts = Self::system_time_to_timestamp(mtime);

        // Try to get the cached entry
        let cached_data = match self.db.get(&key) {
            Ok(Some(data)) => data,
            _ => return None,
        };

        // Deserialize the cached entry
        let cached_entry: CachedFileEntry = match serde_json::from_slice(cached_data.as_ref()) {
            Ok(entry) => entry,
            Err(_) => return None,
        };

        // Check if the file has been modified since it was cached
        if cached_entry.mtime < mtime_ts {
            return None;
        }

        // Return the cached file data
        Some(FileData {
            path: path.to_string(),
            content: cached_entry.content,
            tokens: cached_entry.tokens,
            error: cached_entry.error,
        })
    }

    fn store_file_data(&self, file_data: &FileData, mtime: SystemTime) -> Result<()> {
        let key = Self::create_key(&file_data.path);
        let mtime_ts = Self::system_time_to_timestamp(mtime);

        // Create a cached entry
        let cached_entry = CachedFileEntry {
            content: file_data.content.clone(),
            tokens: file_data.tokens,
            mtime: mtime_ts,
            error: file_data.error.clone(),
        };

        // Serialize the cached entry
        let serialized = serde_json::to_vec(&cached_entry).map_err(|e| {
            Error::Config(format!("Failed to serialize cached entry: {}", e))
        })?;

        // Store the cached entry
        self.db.put(&key, &serialized).map_err(|e| {
            Error::Config(format!("Failed to store file data in cache: {}", e))
        })?;

        Ok(())
    }

    fn clear(&self) -> Result<()> {
        // Iterate over all keys and delete them
        let iter = self.db.iterator(IteratorMode::Start);
        for result in iter {
            let (key, _) = result.map_err(|e| {
                Error::Config(format!("Failed to iterate over cache entries: {}", e))
            })?;
            
            self.db.delete(&key).map_err(|e| {
                Error::Config(format!("Failed to delete cache entry: {}", e))
            })?;
        }

        Ok(())
    }
}
