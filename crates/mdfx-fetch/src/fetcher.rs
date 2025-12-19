//! Main fetcher facade that combines cache and sources

use crate::cache::{Cache, CacheConfig};
use crate::error::{FetchError, Result};
use crate::sources::SourceRegistry;
use crate::value::DataValue;
use std::path::PathBuf;

/// Configuration for the fetcher
#[derive(Debug, Clone)]
pub struct FetchConfig {
    /// Cache directory
    pub cache_dir: PathBuf,
    /// Default TTL for cache entries (in seconds)
    pub default_ttl: u64,
    /// Run in offline mode (cache only, no network)
    pub offline: bool,
    /// Force refresh (ignore cache, always fetch)
    pub refresh: bool,
}

impl Default for FetchConfig {
    fn default() -> Self {
        FetchConfig {
            cache_dir: PathBuf::from(".mdfx-cache"),
            default_ttl: 3600,
            offline: false,
            refresh: false,
        }
    }
}

/// Main fetcher that combines caching with data sources
pub struct Fetcher {
    cache: Cache,
    sources: SourceRegistry,
    config: FetchConfig,
}

impl Fetcher {
    /// Create a new fetcher with the given configuration
    pub fn new(config: FetchConfig) -> Result<Self> {
        let cache_config = CacheConfig {
            dir: config.cache_dir.clone(),
            default_ttl: config.default_ttl,
        };

        Ok(Fetcher {
            cache: Cache::new(cache_config)?,
            sources: SourceRegistry::new(),
            config,
        })
    }

    /// Fetch a metric from a data source
    ///
    /// # Arguments
    /// * `source_id` - ID of the data source (e.g., "github", "npm")
    /// * `query` - Source-specific query (e.g., "owner/repo" for GitHub)
    /// * `metric` - Metric to fetch (e.g., "stars", "version")
    ///
    /// # Returns
    /// The fetched value, potentially from cache
    pub fn fetch(&self, source_id: &str, query: &str, metric: &str) -> Result<DataValue> {
        // Get the data source
        let source = self
            .sources
            .get(source_id)
            .ok_or_else(|| FetchError::UnknownSource(source_id.to_string()))?;

        // Check if we should use cache
        if !self.config.refresh {
            // Try to get fresh cache entry
            if let Some(entry) = self.cache.get_fresh(source_id, query, metric) {
                return Ok(entry.value);
            }
        }

        // If offline, only use cache (even stale)
        if self.config.offline {
            if let Some(entry) = self.cache.get_stale(source_id, query, metric) {
                return Ok(entry.value);
            }
            return Err(FetchError::OfflineNoCache(format!(
                "{}:{}:{}",
                source_id, query, metric
            )));
        }

        // Fetch from network
        match source.fetch(query, metric) {
            Ok(value) => {
                // Cache the result
                let _ = self.cache.set(
                    source_id,
                    query,
                    metric,
                    value.clone(),
                    Some(source.default_ttl()),
                );
                Ok(value)
            }
            Err(e) if e.is_recoverable() => {
                // On recoverable errors, try stale cache as fallback
                if let Some(entry) = self.cache.get_stale(source_id, query, metric) {
                    return Ok(entry.value);
                }
                Err(e)
            }
            Err(e) => Err(e),
        }
    }

    /// Get metadata for a metric from a source
    pub fn metric_info(&self, source_id: &str, metric: &str) -> Option<MetricInfo> {
        let source = self.sources.get(source_id)?;

        Some(MetricInfo {
            label: source.metric_label(metric).to_string(),
            source_name: source.name().to_string(),
        })
    }

    /// Get the color for a metric value
    pub fn metric_color(&self, source_id: &str, metric: &str, value: &DataValue) -> Option<String> {
        let source = self.sources.get(source_id)?;
        source.metric_color(metric, value).map(String::from)
    }

    /// List all available sources
    pub fn list_sources(&self) -> Vec<&'static str> {
        self.sources.list()
    }

    /// Get available metrics for a source
    pub fn available_metrics(&self, source_id: &str) -> Option<&'static [&'static str]> {
        self.sources.get(source_id).map(|s| s.available_metrics())
    }

    /// Clear the cache
    pub fn clear_cache(&self) -> Result<()> {
        self.cache.clear()
    }

    /// Clear expired cache entries
    pub fn clear_expired_cache(&self) -> Result<usize> {
        self.cache.clear_expired()
    }

    /// Check if a source exists
    pub fn has_source(&self, source_id: &str) -> bool {
        self.sources.get(source_id).is_some()
    }
}

/// Metadata about a metric
#[derive(Debug, Clone)]
pub struct MetricInfo {
    /// Display label for the metric
    pub label: String,
    /// Name of the source
    pub source_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn temp_fetcher(offline: bool, refresh: bool) -> (Fetcher, TempDir) {
        let dir = TempDir::new().unwrap();
        let config = FetchConfig {
            cache_dir: dir.path().to_path_buf(),
            default_ttl: 3600,
            offline,
            refresh,
        };
        let fetcher = Fetcher::new(config).unwrap();
        (fetcher, dir)
    }

    #[test]
    fn test_fetcher_list_sources() {
        let (fetcher, _dir) = temp_fetcher(false, false);
        let sources = fetcher.list_sources();
        assert!(sources.contains(&"github"));
    }

    #[test]
    fn test_fetcher_has_source() {
        let (fetcher, _dir) = temp_fetcher(false, false);
        assert!(fetcher.has_source("github"));
        assert!(!fetcher.has_source("nonexistent"));
    }

    #[test]
    fn test_fetcher_available_metrics() {
        let (fetcher, _dir) = temp_fetcher(false, false);
        let metrics = fetcher.available_metrics("github").unwrap();
        assert!(metrics.contains(&"stars"));
    }

    #[test]
    fn test_fetcher_metric_info() {
        let (fetcher, _dir) = temp_fetcher(false, false);
        let info = fetcher.metric_info("github", "stars").unwrap();
        assert_eq!(info.label, "Stars");
        assert_eq!(info.source_name, "GitHub");
    }

    #[test]
    fn test_fetcher_offline_no_cache() {
        let (fetcher, _dir) = temp_fetcher(true, false);
        let result = fetcher.fetch("github", "rust-lang/rust", "stars");
        assert!(matches!(result, Err(FetchError::OfflineNoCache(_))));
    }

    #[test]
    fn test_fetcher_unknown_source() {
        let (fetcher, _dir) = temp_fetcher(false, false);
        let result = fetcher.fetch("nonexistent", "query", "metric");
        assert!(matches!(result, Err(FetchError::UnknownSource(_))));
    }
}
