//! Disk-based cache for fetched data

use crate::error::Result;
use crate::value::DataValue;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Directory to store cache files
    pub dir: PathBuf,
    /// Default TTL for cache entries (in seconds)
    pub default_ttl: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            dir: PathBuf::from(".mdfx-cache"),
            default_ttl: 3600, // 1 hour
        }
    }
}

/// A cached data entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// The cached value
    pub value: DataValue,
    /// Unix timestamp when the entry was created
    pub created_at: u64,
    /// TTL in seconds
    pub ttl: u64,
}

impl CacheEntry {
    /// Check if this entry has expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Use >= so that TTL=0 means immediately expired
        now >= self.created_at + self.ttl
    }

    /// Get time remaining until expiration (0 if expired)
    pub fn time_remaining(&self) -> Duration {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expires_at = self.created_at + self.ttl;
        if now >= expires_at {
            Duration::ZERO
        } else {
            Duration::from_secs(expires_at - now)
        }
    }
}

/// Disk-based cache for fetched data
pub struct Cache {
    config: CacheConfig,
}

impl Cache {
    /// Create a new cache with the given configuration
    pub fn new(config: CacheConfig) -> Result<Self> {
        // Create cache directory if it doesn't exist
        if !config.dir.exists() {
            fs::create_dir_all(&config.dir)?;
        }
        Ok(Cache { config })
    }

    /// Generate cache key from source, query, and metric
    fn cache_key(&self, source: &str, query: &str, metric: &str) -> String {
        // Sanitize query for filename safety
        let safe_query = query.replace(['/', '\\', ':', '@'], "_");
        format!("{}_{}_{}", source, safe_query, metric)
    }

    /// Get the file path for a cache key
    fn cache_path(&self, key: &str) -> PathBuf {
        self.config.dir.join(format!("{}.json", key))
    }

    /// Get a cached entry if it exists and is not expired
    pub fn get(&self, source: &str, query: &str, metric: &str) -> Option<CacheEntry> {
        let key = self.cache_key(source, query, metric);
        let path = self.cache_path(&key);

        if !path.exists() {
            return None;
        }

        let content = fs::read_to_string(&path).ok()?;
        let entry: CacheEntry = serde_json::from_str(&content).ok()?;

        Some(entry)
    }

    /// Get a cached entry only if it's not expired
    pub fn get_fresh(&self, source: &str, query: &str, metric: &str) -> Option<CacheEntry> {
        self.get(source, query, metric).filter(|e| !e.is_expired())
    }

    /// Get a cached entry even if expired (for fallback)
    pub fn get_stale(&self, source: &str, query: &str, metric: &str) -> Option<CacheEntry> {
        self.get(source, query, metric)
    }

    /// Set a cache entry
    pub fn set(
        &self,
        source: &str,
        query: &str,
        metric: &str,
        value: DataValue,
        ttl: Option<u64>,
    ) -> Result<()> {
        let key = self.cache_key(source, query, metric);
        let path = self.cache_path(&key);

        let entry = CacheEntry {
            value,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl: ttl.unwrap_or(self.config.default_ttl),
        };

        let content = serde_json::to_string_pretty(&entry)?;
        fs::write(&path, content)?;

        Ok(())
    }

    /// Remove a cache entry
    pub fn remove(&self, source: &str, query: &str, metric: &str) -> Result<()> {
        let key = self.cache_key(source, query, metric);
        let path = self.cache_path(&key);

        if path.exists() {
            fs::remove_file(&path)?;
        }

        Ok(())
    }

    /// Clear all cache entries
    pub fn clear(&self) -> Result<()> {
        if self.config.dir.exists() {
            for entry in fs::read_dir(&self.config.dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }

    /// Clear expired cache entries
    pub fn clear_expired(&self) -> Result<usize> {
        let mut removed = 0;

        if self.config.dir.exists() {
            for entry in fs::read_dir(&self.config.dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                        if cache_entry.is_expired() {
                            fs::remove_file(&path)?;
                            removed += 1;
                        }
                    }
                }
            }
        }

        Ok(removed)
    }

    /// Get cache statistics
    pub fn stats(&self) -> Result<CacheStats> {
        let mut total = 0;
        let mut expired = 0;
        let mut size_bytes = 0;

        if self.config.dir.exists() {
            for entry in fs::read_dir(&self.config.dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }

                total += 1;
                size_bytes += entry.metadata()?.len();

                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                        if cache_entry.is_expired() {
                            expired += 1;
                        }
                    }
                }
            }
        }

        Ok(CacheStats {
            total_entries: total,
            expired_entries: expired,
            size_bytes,
        })
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub size_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tempfile::TempDir;

    fn temp_cache() -> (Cache, TempDir) {
        let dir = TempDir::new().unwrap();
        let config = CacheConfig {
            dir: dir.path().to_path_buf(),
            default_ttl: 3600,
        };
        let cache = Cache::new(config).unwrap();
        (cache, dir)
    }

    #[test]
    fn test_cache_set_get() {
        let (cache, _dir) = temp_cache();

        cache
            .set(
                "github",
                "rust-lang/rust",
                "stars",
                DataValue::Number(100_000),
                None,
            )
            .unwrap();

        let entry = cache.get("github", "rust-lang/rust", "stars").unwrap();
        assert_eq!(entry.value, DataValue::Number(100_000));
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_expiry() {
        let (cache, _dir) = temp_cache();

        // Set with 0 TTL (immediately expired)
        cache
            .set(
                "github",
                "test/repo",
                "stars",
                DataValue::Number(42),
                Some(0),
            )
            .unwrap();

        // Should be expired immediately
        let entry = cache.get("github", "test/repo", "stars").unwrap();
        assert!(entry.is_expired());

        // get_fresh should return None for expired
        assert!(cache.get_fresh("github", "test/repo", "stars").is_none());

        // get_stale should still return it
        assert!(cache.get_stale("github", "test/repo", "stars").is_some());
    }

    #[test]
    fn test_cache_remove() {
        let (cache, _dir) = temp_cache();

        cache
            .set(
                "npm",
                "lodash",
                "version",
                DataValue::String("4.17.21".to_string()),
                None,
            )
            .unwrap();
        assert!(cache.get("npm", "lodash", "version").is_some());

        cache.remove("npm", "lodash", "version").unwrap();
        assert!(cache.get("npm", "lodash", "version").is_none());
    }

    #[test]
    fn test_cache_clear() {
        let (cache, _dir) = temp_cache();

        cache
            .set("github", "a/b", "stars", DataValue::Number(1), None)
            .unwrap();
        cache
            .set(
                "npm",
                "c",
                "version",
                DataValue::String("1.0".to_string()),
                None,
            )
            .unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 2);

        cache.clear().unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 0);
    }

    // ========================================================================
    // Cache Key Sanitization (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("npm", "@scope/package", "version", "1.0")] // scoped package
    #[case("github", "user/repo", "stars", "100")] // slash in query
    #[case("npm", "pkg:alias", "version", "2.0")] // colon in query
    #[case("github", "user\\repo", "forks", "50")] // backslash in query
    fn test_cache_key_sanitization(
        #[case] source: &str,
        #[case] query: &str,
        #[case] metric: &str,
        #[case] value: &str,
    ) {
        let (cache, _dir) = temp_cache();

        cache
            .set(
                source,
                query,
                metric,
                DataValue::String(value.to_string()),
                None,
            )
            .unwrap();

        let entry = cache.get(source, query, metric).unwrap();
        assert_eq!(entry.value, DataValue::String(value.to_string()));
    }

    // ========================================================================
    // Additional Coverage Tests
    // ========================================================================

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        assert_eq!(config.dir, PathBuf::from(".mdfx-cache"));
        assert_eq!(config.default_ttl, 3600);
    }

    #[test]
    fn test_cache_entry_time_remaining() {
        // Entry with long TTL should have time remaining
        let entry = CacheEntry {
            value: DataValue::Number(42),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl: 3600,
        };
        assert!(entry.time_remaining().as_secs() > 0);

        // Entry with 0 TTL should have zero time remaining
        let expired_entry = CacheEntry {
            value: DataValue::Number(42),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl: 0,
        };
        assert_eq!(expired_entry.time_remaining(), std::time::Duration::ZERO);
    }

    #[test]
    fn test_cache_remove_nonexistent() {
        let (cache, _dir) = temp_cache();
        // Should not error when removing non-existent entry
        assert!(cache.remove("github", "nonexistent/repo", "stars").is_ok());
    }

    #[test]
    fn test_cache_clear_expired() {
        let (cache, _dir) = temp_cache();

        // Add expired entry
        cache
            .set("github", "a/b", "stars", DataValue::Number(1), Some(0))
            .unwrap();

        // Add fresh entry
        cache
            .set("github", "c/d", "forks", DataValue::Number(2), Some(3600))
            .unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.expired_entries, 1);

        // Clear only expired entries
        let removed = cache.clear_expired().unwrap();
        assert_eq!(removed, 1);

        // Only fresh entry should remain
        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.expired_entries, 0);
    }

    #[test]
    fn test_cache_stats_with_size() {
        let (cache, _dir) = temp_cache();

        cache
            .set(
                "github",
                "test/repo",
                "stars",
                DataValue::Number(12345),
                None,
            )
            .unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 1);
        assert!(stats.size_bytes > 0);
    }
}
