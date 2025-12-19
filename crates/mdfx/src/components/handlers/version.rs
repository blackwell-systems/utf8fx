//! Version badge component handler
//!
//! Renders semantic version badges with status-aware coloring.
//! Automatically detects prerelease versions from the version string.

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Status categories for version coloring
#[derive(Debug, Clone, Copy, PartialEq)]
enum VersionStatus {
    Stable,      // Green - production ready
    Beta,        // Yellow - testing/preview
    Alpha,       // Orange - early development
    Deprecated,  // Red - no longer supported
    Dev,         // Purple - development only
}

impl VersionStatus {
    /// Get the background color for this status
    fn bg_color(&self) -> &'static str {
        match self {
            VersionStatus::Stable => "22C55E",     // success green
            VersionStatus::Beta => "EAB308",       // warning yellow
            VersionStatus::Alpha => "F97316",      // orange
            VersionStatus::Deprecated => "EF4444", // error red
            VersionStatus::Dev => "8B5CF6",        // purple
        }
    }

    /// Get the text color for this status (for contrast)
    fn text_color(&self) -> &'static str {
        match self {
            VersionStatus::Stable => "FFFFFF",
            VersionStatus::Beta => "000000",
            VersionStatus::Alpha => "FFFFFF",
            VersionStatus::Deprecated => "FFFFFF",
            VersionStatus::Dev => "FFFFFF",
        }
    }
}

/// Parse version string to detect status
fn detect_status(version: &str) -> VersionStatus {
    let lower = version.to_lowercase();

    // Check for explicit prerelease suffixes
    if lower.contains("-deprecated") || lower.contains("-eol") {
        return VersionStatus::Deprecated;
    }
    if lower.contains("-alpha") || lower.contains("-a.") {
        return VersionStatus::Alpha;
    }
    if lower.contains("-beta") || lower.contains("-b.") || lower.contains("-rc") || lower.contains("-preview") {
        return VersionStatus::Beta;
    }
    if lower.contains("-dev") || lower.contains("-snapshot") || lower.contains("-nightly") {
        return VersionStatus::Dev;
    }

    // Check for 0.x.x versions (typically considered unstable)
    if version.starts_with("0.") {
        return VersionStatus::Beta;
    }

    VersionStatus::Stable
}

/// Parse status override from string
fn parse_status(status: &str) -> Option<VersionStatus> {
    match status.to_lowercase().as_str() {
        "stable" | "release" | "production" => Some(VersionStatus::Stable),
        "beta" | "rc" | "preview" | "testing" => Some(VersionStatus::Beta),
        "alpha" | "early" | "experimental" => Some(VersionStatus::Alpha),
        "deprecated" | "eol" | "legacy" | "unsupported" => Some(VersionStatus::Deprecated),
        "dev" | "development" | "snapshot" | "nightly" => Some(VersionStatus::Dev),
        _ => None,
    }
}

/// Handle version component expansion
///
/// Syntax: {{ui:version:VERSION/}} or {{ui:version:VERSION:status=STATUS/}}
///
/// Examples:
/// - {{ui:version:1.0.0/}} -> green (stable)
/// - {{ui:version:2.0.0-beta.1/}} -> yellow (auto-detected beta)
/// - {{ui:version:0.1.0/}} -> yellow (0.x is beta)
/// - {{ui:version:1.5.0:status=deprecated/}} -> red (override)
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "version component requires a version string argument".to_string(),
        ));
    }

    let version = &args[0];

    // Determine status: explicit override or auto-detect
    let status = params
        .get("status")
        .and_then(|s| parse_status(s))
        .unwrap_or_else(|| detect_status(version));

    // Allow color overrides
    let bg_color = params
        .get("bg")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| status.bg_color().to_string());

    let text_color = params
        .get("text")
        .or_else(|| params.get("text_color"))
        .or_else(|| params.get("color"))
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| status.text_color().to_string());

    // Build label: "v" prefix unless already present or disabled
    let prefix = params.get("prefix").map(|s| s.as_str()).unwrap_or("v");
    let label = if version.starts_with('v') || version.starts_with('V') || prefix.is_empty() {
        version.clone()
    } else {
        format!("{}{}", prefix, version)
    };

    // Optional icon (e.g., "tag", "package")
    let icon = params.get("icon").cloned();

    // Border parameters
    let border_color = params.get("border").map(|c| resolve_color(c));
    let border_width = params.get("border_width").and_then(|v| v.parse().ok());

    // Corner radius
    let rx = params.get("rx").and_then(|v| v.parse().ok());

    // URL for clickable links
    let _url = params.get("url").cloned();

    Ok(ComponentOutput::Primitive(Primitive::Swatch {
        color: bg_color,
        style: style.to_string(),
        opacity: None,
        width: None,
        height: None,
        border_color,
        border_width,
        label: Some(label),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_stable() {
        assert_eq!(detect_status("1.0.0"), VersionStatus::Stable);
        assert_eq!(detect_status("2.5.3"), VersionStatus::Stable);
        assert_eq!(detect_status("10.0.0"), VersionStatus::Stable);
    }

    #[test]
    fn test_detect_beta() {
        assert_eq!(detect_status("1.0.0-beta"), VersionStatus::Beta);
        assert_eq!(detect_status("2.0.0-beta.1"), VersionStatus::Beta);
        assert_eq!(detect_status("1.0.0-rc.1"), VersionStatus::Beta);
        assert_eq!(detect_status("1.0.0-preview"), VersionStatus::Beta);
        assert_eq!(detect_status("0.5.0"), VersionStatus::Beta); // 0.x
    }

    #[test]
    fn test_detect_alpha() {
        assert_eq!(detect_status("1.0.0-alpha"), VersionStatus::Alpha);
        assert_eq!(detect_status("1.0.0-alpha.2"), VersionStatus::Alpha);
    }

    #[test]
    fn test_detect_deprecated() {
        assert_eq!(detect_status("1.0.0-deprecated"), VersionStatus::Deprecated);
        assert_eq!(detect_status("1.0.0-eol"), VersionStatus::Deprecated);
    }

    #[test]
    fn test_detect_dev() {
        assert_eq!(detect_status("1.0.0-dev"), VersionStatus::Dev);
        assert_eq!(detect_status("1.0.0-snapshot"), VersionStatus::Dev);
        assert_eq!(detect_status("1.0.0-nightly"), VersionStatus::Dev);
    }

    #[test]
    fn test_parse_status() {
        assert_eq!(parse_status("stable"), Some(VersionStatus::Stable));
        assert_eq!(parse_status("beta"), Some(VersionStatus::Beta));
        assert_eq!(parse_status("alpha"), Some(VersionStatus::Alpha));
        assert_eq!(parse_status("deprecated"), Some(VersionStatus::Deprecated));
        assert_eq!(parse_status("dev"), Some(VersionStatus::Dev));
        assert_eq!(parse_status("unknown"), None);
    }
}
