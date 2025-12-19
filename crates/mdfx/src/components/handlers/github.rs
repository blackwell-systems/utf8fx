//! GitHub badge component handler
//!
//! Renders badges with live GitHub repository data.
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
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
    fetch_ctx: &FetchContext,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "github component requires owner/repo argument".to_string(),
        ));
    }

    let repo = &args[0];

    // Metric can be second arg or param
    let metric = args
        .get(1)
        .or_else(|| params.get("metric"))
        .map(|s| s.as_str())
        .unwrap_or("stars");

    // Fetch the data
    let value = fetch_ctx
        .fetcher
        .fetch("github", repo, metric)
        .map_err(|e| Error::ParseError(format!("Failed to fetch GitHub data: {}", e)))?;

    // Get metric info
    let label = fetch_ctx
        .fetcher
        .metric_info("github", metric)
        .map(|info| info.label)
        .unwrap_or_else(|| metric.to_string());

    // Determine color
    let bg_color = params.get("bg").map(|c| resolve_color(c)).unwrap_or_else(|| {
        fetch_ctx
            .fetcher
            .metric_color("github", metric, &value)
            .unwrap_or_else(|| "3B82F6".to_string())
    });

    // Text color (default white for most colors, black for yellow)
    let text_color = params
        .get("text")
        .or_else(|| params.get("text_color"))
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| {
            // Use black text for yellow-ish backgrounds
            if bg_color == "EAB308" || bg_color == "FFD700" {
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
    let icon = params.get("icon").cloned().or_else(|| {
        // Default icons for some metrics
        match metric {
            "stars" => Some("star".to_string()),
            "forks" => Some("git-branch".to_string()),
            "issues" => Some("issue-opened".to_string()),
            _ => None,
        }
    });

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

/// Placeholder handler when fetch feature is not enabled
#[cfg(not(feature = "fetch"))]
pub fn handle(
    _args: &[String],
    _params: &HashMap<String, String>,
    _style: &str,
    _resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    Err(Error::ParseError(
        "GitHub badges require the 'fetch' feature. Enable with: mdfx --features fetch".to_string(),
    ))
}

#[cfg(all(test, feature = "fetch"))]
mod tests {
    use super::*;
    use std::path::PathBuf;
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
    fn test_handle_offline_no_cache() {
        let (ctx, _dir) = temp_fetch_ctx(true);
        let params = HashMap::new();
        let result = handle(
            &["rust-lang/rust".to_string()],
            &params,
            "flat",
            |c| c.to_string(),
            &ctx,
        );

        // Should fail in offline mode with no cache
        assert!(result.is_err());
    }
}
