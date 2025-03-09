use crate::cache::trait_def::{Cache, CacheConfig, Info};
use crate::core::file::FileData;
use crate::error::Result;
use std::time::SystemTime;

/// A no-op cache implementation that doesn't actually cache anything
pub struct NoneCache;

impl Cache for NoneCache {
    fn new(_config: CacheConfig) -> Result<Self> {
        Ok(NoneCache)
    }

    fn get_file_data(&self, _path: &str, _mtime: SystemTime) -> Option<FileData> {
        // Always return None to indicate cache miss
        None
    }

    fn store_file_data(&self, _file_data: &FileData, _mtime: SystemTime) -> Result<()> {
        // Do nothing
        Ok(())
    }

    fn clear(&self) -> Result<()> {
        // Nothing to clear
        Ok(())
    }

    fn info(&self) -> Result<Info> {
        Ok(Info::default())
    }
}
