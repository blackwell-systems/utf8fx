//! Data sources for fetching metrics from external APIs

mod actions;
mod codecov;
mod crates;
mod github;
mod npm;
mod pypi;

pub use actions::ActionsSource;
pub use codecov::CodecovSource;
pub use crates::CratesSource;
pub use github::GitHubSource;
pub use npm::NpmSource;
pub use pypi::PyPISource;

use crate::error::Result;
use crate::value::DataValue;

/// Trait for data sources that can fetch metrics
pub trait DataSource: Send + Sync {
    /// Unique identifier for this source (e.g., "github", "npm")
    fn id(&self) -> &'static str;

    /// Human-readable name for this source
    fn name(&self) -> &'static str;

    /// Fetch a metric value for the given query
    ///
    /// # Arguments
    /// * `query` - Source-specific query (e.g., "owner/repo" for GitHub)
    /// * `metric` - Metric to fetch (e.g., "stars", "version")
    ///
    /// # Returns
    /// The fetched value or an error
    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue>;

    /// List of available metrics for this source
    fn available_metrics(&self) -> &'static [&'static str];

    /// Default TTL for caching this source's data (in seconds)
    fn default_ttl(&self) -> u64 {
        3600 // 1 hour default
    }

    /// Whether this source requires authentication
    fn requires_auth(&self) -> bool {
        false
    }

    /// Get the label to display for a metric
    fn metric_label(&self, metric: &str) -> &'static str;

    /// Get the color to use for a metric value
    fn metric_color(&self, _metric: &str, _value: &DataValue) -> Option<&str> {
        None // Use default color
    }
}

/// Registry of all available data sources
pub struct SourceRegistry {
    sources: Vec<Box<dyn DataSource>>,
}

impl Default for SourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceRegistry {
    /// Create a new registry with all built-in sources
    pub fn new() -> Self {
        SourceRegistry {
            sources: vec![
                Box::new(GitHubSource::new()),
                Box::new(NpmSource::new()),
                Box::new(CratesSource::new()),
                Box::new(PyPISource::new()),
                Box::new(CodecovSource::new()),
                Box::new(ActionsSource::new()),
            ],
        }
    }

    /// Get a source by ID
    pub fn get(&self, id: &str) -> Option<&dyn DataSource> {
        self.sources
            .iter()
            .find(|s| s.id() == id)
            .map(|s| s.as_ref())
    }

    /// List all available source IDs
    pub fn list(&self) -> Vec<&'static str> {
        self.sources.iter().map(|s| s.id()).collect()
    }

    /// List all sources with their available metrics
    pub fn list_with_metrics(&self) -> Vec<(&'static str, &'static [&'static str])> {
        self.sources
            .iter()
            .map(|s| (s.id(), s.available_metrics()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Registry Source Availability (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("github")]
    #[case("npm")]
    #[case("crates")]
    #[case("pypi")]
    #[case("codecov")]
    #[case("actions")]
    fn test_registry_has_source(#[case] source_id: &str) {
        let registry = SourceRegistry::new();
        assert!(
            registry.get(source_id).is_some(),
            "Registry should have source: {}",
            source_id
        );
    }

    #[test]
    fn test_registry_list() {
        let registry = SourceRegistry::new();
        let sources = registry.list();
        assert!(sources.contains(&"github"));
        assert!(sources.contains(&"npm"));
        assert!(sources.contains(&"crates"));
        assert!(sources.contains(&"pypi"));
        assert!(sources.contains(&"codecov"));
        assert!(sources.contains(&"actions"));
    }
}
