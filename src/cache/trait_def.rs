use serde::Serialize;

use crate::core::file::FileData;
use crate::error::Result;
use std::path::PathBuf;
use std::time::SystemTime;

/// Configuration for cache providers
pub struct CacheConfig {
    /// Directory where cache files will be stored
    pub cache_dir: PathBuf,
}

/// Descriptive information about the cache.
#[derive(Default, Serialize)]
pub struct Info {
    /// The number of records stored in the cache.
    pub records: i64,
    /// The total number of bytes allocated by the cache.
    pub allocated: i64,
}

/// Cache trait that defines the interface for all cache implementations
pub trait Cache: Send + Sync {
    /// Create a new cache instance with the given configuration
    fn new(config: CacheConfig) -> Result<Self>
    where
        Self: Sized;

    /// Get file data from the cache if it exists and is not modified since last cache
    ///
    /// # Arguments
    /// * `path` - The path to the file
    /// * `mtime` - The last modification time of the file
    ///
    /// # Returns
    /// * `Some(FileData)` if the file is in the cache and not modified
    /// * `None` if the file is not in the cache or has been modified
    fn get_file_data(&self, path: &str, mtime: SystemTime) -> Option<FileData>;

    /// Store file data in the cache
    ///
    /// # Arguments
    /// * `file_data` - The file data to store
    /// * `mtime` - The last modification time of the file
    fn store_file_data(&self, file_data: &FileData, mtime: SystemTime) -> Result<()>;

    /// Clear the cache
    fn clear(&self) -> Result<()>;

    /// Retrieve descriptive information about the cache
    ///
    /// Returns information such as its type, size,
    /// number of entries, and other relevant statistics.
    fn info(&self) -> Result<Info>;
}
