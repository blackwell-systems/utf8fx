//! Dynamic badge component handlers
//!
//! Renders badges with live data from external APIs (GitHub, npm, crates.io, PyPI).
//! Requires the `fetch` feature to be enabled.

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

#[cfg(feature = "fetch")]
use mdfx_fetch::{FetchConfig, Fetcher};

/// Fetch context for dynamic badges
#[cfg(feature = "fetch")]
pub struct FetchContext {
    fetcher: Fetcher,
}

#[cfg(feature = "fetch")]
impl FetchContext {
    /// Create a new fetch context
    pub fn new(config: FetchConfig) -> Result<Self> {
        let fetcher = Fetcher::new(config).map_err(|e| Error::ParseError(e.to_string()))?;
        Ok(FetchContext { fetcher })
    }

    /// Get the underlying fetcher
    pub fn fetcher(&self) -> &Fetcher {
        &self.fetcher
    }
}

/// Generic handler for any data source
///
/// This is the core function that handles fetching data from any source
/// and rendering it as a badge.
#[cfg(feature = "fetch")]
pub fn handle_source(
    source_id: &str,
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
    fetch_ctx: &FetchContext,
    default_metric: &str,
    default_color: &str,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(format!(
            "{} component requires a query argument",
            source_id
        )));
    }

    let query = &args[0];

    // Metric can be second arg or param
    let metric = args
        .get(1)
        .or_else(|| params.get("metric"))
        .map(|s| s.as_str())
        .unwrap_or(default_metric);

    // Fetch the data
    let value = fetch_ctx
        .fetcher
        .fetch(source_id, query, metric)
        .map_err(|e| Error::ParseError(format!("Failed to fetch {} data: {}", source_id, e)))?;

    // Get metric info
    let label = fetch_ctx
        .fetcher
        .metric_info(source_id, metric)
        .map(|info| info.label)
        .unwrap_or_else(|| metric.to_string());

    // Determine color
    let bg_color = params.get("bg").map(|c| resolve_color(c)).unwrap_or_else(|| {
        fetch_ctx
            .fetcher
            .metric_color(source_id, metric, &value)
            .unwrap_or_else(|| default_color.to_string())
    });

    // Text color (default white for most colors, black for yellow/bright)
    let text_color = params
        .get("text")
        .or_else(|| params.get("text_color"))
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| {
            // Use black text for yellow-ish backgrounds
            if bg_color == "EAB308" || bg_color == "FFD700" || bg_color == "FFD43B" {
                "000000".to_string()
            } else {
                "FFFFFF".to_string()
            }
        });

    // Format label with value
    let display_label = format!("{}: {}", label, value.format());

    // Calculate width (approx 7px per char + 16px padding)
    let estimated_width = params
        .get("width")
        .and_then(|w| w.parse().ok())
        .unwrap_or_else(|| (display_label.len() as u32 * 7 + 16).max(50));

    // Border parameters
    let border_color = params.get("border").map(|c| resolve_color(c));
    let border_width = params.get("border_width").and_then(|v| v.parse().ok());

    // Corner radius - default to 3
    let rx = params.get("rx").and_then(|v| v.parse().ok()).or(Some(3));

    // Optional icon
    let icon = params.get("icon").cloned();

    Ok(ComponentOutput::Primitive(Primitive::Swatch {
        color: bg_color,
        style: style.to_string(),
        opacity: None,
        width: Some(estimated_width),
        height: None,
        border_color,
        border_width,
        label: Some(display_label),
        label_color: Some(text_color),
        icon,
        icon_color: None,
        rx,
        ry: None,
        shadow: None,
        gradient: None,
        stroke_dash: None,
        logo_size: None,
        border_top: None,
        border_right: None,
        border_bottom: None,
        border_left: None,
    }))
}

/// Handle github component expansion
///
/// Syntax: {{ui:github:owner/repo:metric/}}
///
/// Metrics:
/// - stars - Repository star count
/// - forks - Fork count
/// - issues - Open issue count
/// - license - SPDX license identifier
/// - language - Primary programming language
///
/// Examples:
/// - {{ui:github:rust-lang/rust:stars/}}
/// - {{ui:github:facebook/react:license/}}
/// - {{ui:github:torvalds/linux:forks:bg=dark1/}}
#[cfg(feature = "fetch")]
pub fn handle_github(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
    fetch_ctx: &FetchContext,
) -> Result<ComponentOutput> {
    handle_source("github", args, params, style, resolve_color, fetch_ctx, "stars", "3B82F6")
}

/// Handle npm component expansion
///
/// Syntax: {{ui:npm:package-name:metric/}}
///
/// Metrics:
/// - version - Latest stable version
/// - license - Package license
/// - next - Latest @next tag version
/// - beta - Latest @beta tag version
///
/// Examples:
/// - {{ui:npm:react:version/}}
/// - {{ui:npm:typescript:license/}}
#[cfg(feature = "fetch")]
pub fn handle_npm(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
    fetch_ctx: &FetchContext,
) -> Result<ComponentOutput> {
    handle_source("npm", args, params, style, resolve_color, fetch_ctx, "version", "CB3837")
}

/// Handle crates component expansion
///
/// Syntax: {{ui:crates:crate-name:metric/}}
///
/// Metrics:
/// - version - Latest version
/// - downloads - Total download count
/// - description - Crate description
///
/// Examples:
/// - {{ui:crates:serde:version/}}
/// - {{ui:crates:tokio:downloads/}}
#[cfg(feature = "fetch")]
pub fn handle_crates(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
    fetch_ctx: &FetchContext,
) -> Result<ComponentOutput> {
    handle_source("crates", args, params, style, resolve_color, fetch_ctx, "version", "E57300")
}

/// Handle pypi component expansion
///
/// Syntax: {{ui:pypi:package-name:metric/}}
///
/// Metrics:
/// - version - Latest version
/// - license - Package license
/// - author - Package author
/// - python - Required Python version
///
/// Examples:
/// - {{ui:pypi:requests:version/}}
/// - {{ui:pypi:numpy:python/}}
#[cfg(feature = "fetch")]
pub fn handle_pypi(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
    fetch_ctx: &FetchContext,
) -> Result<ComponentOutput> {
    handle_source("pypi", args, params, style, resolve_color, fetch_ctx, "version", "3776AB")
}

#[cfg(all(test, feature = "fetch"))]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn temp_fetch_ctx(offline: bool) -> (FetchContext, TempDir) {
        let dir = TempDir::new().unwrap();
        let config = FetchConfig {
            cache_dir: dir.path().to_path_buf(),
            default_ttl: 3600,
            offline,
            refresh: false,
        };
        let ctx = FetchContext::new(config).unwrap();
        (ctx, dir)
    }

    #[test]
    fn test_github_offline_no_cache() {
        let (ctx, _dir) = temp_fetch_ctx(true);
        let params = HashMap::new();
        let result = handle_github(
            &["rust-lang/rust".to_string()],
            &params,
            "flat",
            |c| c.to_string(),
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_npm_offline_no_cache() {
        let (ctx, _dir) = temp_fetch_ctx(true);
        let params = HashMap::new();
        let result = handle_npm(
            &["react".to_string()],
            &params,
            "flat",
            |c| c.to_string(),
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_crates_offline_no_cache() {
        let (ctx, _dir) = temp_fetch_ctx(true);
        let params = HashMap::new();
        let result = handle_crates(
            &["serde".to_string()],
            &params,
            "flat",
            |c| c.to_string(),
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_pypi_offline_no_cache() {
        let (ctx, _dir) = temp_fetch_ctx(true);
        let params = HashMap::new();
        let result = handle_pypi(
            &["requests".to_string()],
            &params,
            "flat",
            |c| c.to_string(),
            &ctx,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_query() {
        let (ctx, _dir) = temp_fetch_ctx(true);
        let params = HashMap::new();

        // All handlers should error with empty args
        assert!(handle_github(&[], &params, "flat", |c| c.to_string(), &ctx).is_err());
        assert!(handle_npm(&[], &params, "flat", |c| c.to_string(), &ctx).is_err());
        assert!(handle_crates(&[], &params, "flat", |c| c.to_string(), &ctx).is_err());
        assert!(handle_pypi(&[], &params, "flat", |c| c.to_string(), &ctx).is_err());
    }
}
