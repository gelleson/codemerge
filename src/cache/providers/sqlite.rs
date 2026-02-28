use crate::cache::trait_def::{Cache, CacheConfig, Info};
use crate::core::file::FileData;
use crate::error::{Error, Result};
use rusqlite::{params, Connection, OpenFlags};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// SQLite-based cache implementation
pub struct SQLiteCache {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl SQLiteCache {
    /// Convert SystemTime to seconds since UNIX epoch
    fn system_time_to_timestamp(time: SystemTime) -> i64 {
        time.duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }

    /// Convert seconds since UNIX epoch to SystemTime
    #[allow(dead_code)]
    fn timestamp_to_system_time(timestamp: i64) -> SystemTime {
        UNIX_EPOCH + std::time::Duration::from_secs(timestamp as u64)
    }
}

impl Cache for SQLiteCache {
    fn new(config: CacheConfig) -> Result<Self> {
        // Cache directory is already created by the factory function

        let db_path = config.cache_dir.join("cache.db");
        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .map_err(|e| {
            Error::Config(format!(
                "Failed to open SQLite database {}: {}",
                db_path.display(),
                e
            ))
        })?;

        // Optimization PRAGMAs
        let _: String = conn.query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))
            .map_err(|e| Error::Config(format!("Failed to set WAL mode: {}", e)))?;
        conn.execute("PRAGMA synchronous = NORMAL", [])
            .map_err(|e| Error::Config(format!("Failed to set synchronous mode: {}", e)))?;
        conn.execute("PRAGMA cache_size = -10000", []) // 10MB cache
            .map_err(|e| Error::Config(format!("Failed to set cache size: {}", e)))?;

        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS file_cache (
                path TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                tokens INTEGER NOT NULL,
                mtime INTEGER NOT NULL,
                error TEXT
            )",
            [],
        )
        .map_err(|e| Error::Config(format!("Failed to create cache table: {}", e)))?;

        // Create index on path
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_file_cache_path ON file_cache(path)",
            [],
        )
        .map_err(|e| Error::Config(format!("Failed to create cache index: {}", e)))?;

        Ok(SQLiteCache {
            conn: Arc::new(Mutex::new(conn)),
            db_path,
        })
    }

    fn get_file_data(&self, path: &str, mtime: SystemTime) -> Option<FileData> {
        self.get_file_data_batch(&[(path, mtime)])
            .into_iter()
            .next()
            .flatten()
    }

    fn get_file_data_batch(&self, paths: &[(&str, SystemTime)]) -> Vec<Option<FileData>> {
        let conn = match self.conn.lock() {
            Ok(conn) => conn,
            Err(_) => return vec![None; paths.len()], // Lock poisoned
        };

        let mut results = vec![None; paths.len()];
        let path_to_idx: std::collections::HashMap<&str, usize> = paths
            .iter()
            .enumerate()
            .map(|(i, (p, _))| (*p, i))
            .collect();

        for chunk in paths.chunks(900) {
            let placeholders = vec!["?"; chunk.len()].join(",");
            let sql = format!(
                "SELECT path, content, tokens, mtime, error FROM file_cache WHERE path IN ({})",
                placeholders
            );

            let mut stmt = match conn.prepare_cached(&sql) {
                Ok(stmt) => stmt,
                Err(_) => continue,
            };

            let params: Vec<&dyn rusqlite::ToSql> =
                chunk.iter().map(|(p, _)| p as &dyn rusqlite::ToSql).collect();

            let mut rows = match stmt.query(params.as_slice()) {
                Ok(rows) => rows,
                Err(_) => continue,
            };

            while let Ok(Some(row)) = rows.next() {
                let path: String = row.get(0).unwrap_or_default();
                if let Some(&idx) = path_to_idx.get(path.as_str()) {
                    let cache_mtime: i64 = row.get(3).unwrap_or(0);
                    let required_mtime = Self::system_time_to_timestamp(paths[idx].1);
                    
                    if cache_mtime >= required_mtime {
                        let content: String = row.get(1).unwrap_or_default();
                        let tokens: usize = row.get(2).unwrap_or(0);
                        let error: Option<String> = row.get(4).unwrap_or(None);

                        results[idx] = Some(FileData {
                            path,
                            content,
                            tokens,
                            error,
                        });
                    }
                }
            }
        }

        results
    }

    fn store_file_data(&self, file_data: &FileData, mtime: SystemTime) -> Result<()> {
        self.store_file_data_batch(&[(file_data.clone(), mtime)])
    }

    fn store_file_data_batch(&self, batch: &[(FileData, SystemTime)]) -> Result<()> {
        let mut conn = self.conn.lock().map_err(|_| {
            Error::Config("Failed to acquire lock on SQLite connection".to_string())
        })?;

        let tx = conn
            .transaction()
            .map_err(|e| Error::Config(format!("Failed to start transaction: {}", e)))?;

        {
            let mut stmt = tx
                .prepare_cached(
                    "INSERT OR REPLACE INTO file_cache (path, content, tokens, mtime, error) VALUES (?, ?, ?, ?, ?)",
                )
                .map_err(|e| Error::Config(format!("Failed to prepare statement: {}", e)))?;

            for (file_data, mtime) in batch {
                let mtime_ts = Self::system_time_to_timestamp(*mtime);
                stmt.execute(params![
                    file_data.path,
                    file_data.content,
                    file_data.tokens,
                    mtime_ts,
                    file_data.error,
                ])
                .map_err(|e| Error::Config(format!("Failed to store file data in cache: {}", e)))?;
            }
        }

        tx.commit()
            .map_err(|e| Error::Config(format!("Failed to commit transaction: {}", e)))?;

        Ok(())
    }

    fn clear(&self) -> Result<()> {
        let conn = self.conn.lock().map_err(|_| {
            Error::Config("Failed to acquire lock on SQLite connection".to_string())
        })?;

        conn.execute("DELETE FROM file_cache", [])
            .map_err(|e| Error::Config(format!("Failed to clear cache: {}", e)))?;

        Ok(())
    }

    fn info(&self) -> Result<Info> {
        let conn = self.conn.lock().map_err(|_| {
            Error::Config("Failed to acquire lock on SQLite connection".to_string())
        })?;

        let mut info = Info::default();

        // Get the number of Records in the table.
        let result = conn.query_row(
            "SELECT COUNT(*) FROM file_cache", //Efficient way to get row count
            params![],
            |row| {
                let total: i64 = row.get(0)?;

                Ok(total)
            },
        );

        info.records =
            result.map_err(|e| Error::Config(format!("Failed to request cache info: {}", e)))?;

        info.allocated = get_db_size(self.db_path.clone())
            .map_err(|e| Error::Config(format!("Failed to request cache size: {}", e)))?
            as i64;

        Ok(info)
    }
}

pub fn get_db_size(path: PathBuf) -> Result<u64> {
    fs::metadata(&path)
        .map(|metadata| metadata.len())
        .map_err(|e| Error::Config(format!("Failed to get database size: {}", e)))
}
