//! Version badge generation
//!
//! Renders semantic version badges with status-aware coloring.
//! Automatically detects version status from the version string.
//!
//! # Examples
//!
//! ```rust
//! use badgefx::version;
//!
//! // Auto-detects status from version string
//! let svg = version("1.0.0").render();        // green (stable)
//! let svg = version("2.0.0-beta").render();   // yellow (beta)
//! let svg = version("0.5.0").render();        // yellow (0.x = beta)
//!
//! // Override auto-detection
//! use badgefx::version::Status;
//! let svg = version("1.0.0")
//!     .status(Status::Deprecated)
//!     .render();
//! ```

use crate::style::{BadgeStyle, SvgMetrics};

/// Version status categories for coloring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Green - production ready (1.x.x+)
    Stable,
    /// Yellow - testing/preview (-beta, -rc, 0.x.x)
    Beta,
    /// Orange - early development (-alpha)
    Alpha,
    /// Red - no longer supported (-deprecated, -eol)
    Deprecated,
    /// Purple - development only (-dev, -snapshot, -nightly)
    Dev,
}

impl Status {
    /// Get the background color for this status (hex without #)
    pub fn bg_color(&self) -> &'static str {
        match self {
            Status::Stable => "22C55E",     // success green
            Status::Beta => "EAB308",       // warning yellow
            Status::Alpha => "F97316",      // orange
            Status::Deprecated => "EF4444", // error red
            Status::Dev => "8B5CF6",        // purple
        }
    }

    /// Get the text color for this status (hex without #)
    pub fn text_color(&self) -> &'static str {
        match self {
            Status::Stable => "FFFFFF",
            Status::Beta => "000000",
            Status::Alpha => "FFFFFF",
            Status::Deprecated => "FFFFFF",
            Status::Dev => "FFFFFF",
        }
    }
}

/// Detect version status from version string
pub fn detect_status(version: &str) -> Status {
    let lower = version.to_lowercase();

    // Check for explicit prerelease suffixes
    if lower.contains("-deprecated") || lower.contains("-eol") {
        return Status::Deprecated;
    }
    if lower.contains("-alpha") || lower.contains("-a.") {
        return Status::Alpha;
    }
    if lower.contains("-beta")
        || lower.contains("-b.")
        || lower.contains("-rc")
        || lower.contains("-preview")
    {
        return Status::Beta;
    }
    if lower.contains("-dev") || lower.contains("-snapshot") || lower.contains("-nightly") {
        return Status::Dev;
    }

    // Check for 0.x.x versions (typically considered unstable)
    if version.starts_with("0.") {
        return Status::Beta;
    }

    Status::Stable
}

/// Parse status from string (for parameter overrides)
pub fn parse_status(status: &str) -> Option<Status> {
    match status.to_lowercase().as_str() {
        "stable" | "release" | "production" => Some(Status::Stable),
        "beta" | "rc" | "preview" | "testing" => Some(Status::Beta),
        "alpha" | "early" | "experimental" => Some(Status::Alpha),
        "deprecated" | "eol" | "legacy" | "unsupported" => Some(Status::Deprecated),
        "dev" | "development" | "snapshot" | "nightly" => Some(Status::Dev),
        _ => None,
    }
}

/// Version badge specification
#[derive(Debug, Clone)]
pub struct VersionBadge {
    /// Version string (e.g., "1.0.0", "2.0.0-beta.1")
    pub version: String,
    /// Status (auto-detected or overridden)
    pub status: Status,
    /// Visual style of the badge
    pub style: BadgeStyle,
    /// Custom background color (overrides status color)
    pub bg_color: Option<String>,
    /// Custom text color
    pub text_color: Option<String>,
    /// Version prefix ("v" by default, empty to disable)
    pub prefix: String,
    /// Border color
    pub border_color: Option<String>,
    /// Border width in pixels
    pub border_width: Option<u32>,
    /// Corner radius
    pub rx: Option<u32>,
}

impl VersionBadge {
    /// Create a new version badge with auto-detected status
    pub fn new(version: impl Into<String>) -> Self {
        let version = version.into();
        let status = detect_status(&version);
        Self {
            version,
            status,
            style: BadgeStyle::default(),
            bg_color: None,
            text_color: None,
            prefix: "v".to_string(),
            border_color: None,
            border_width: None,
            rx: None,
        }
    }

    /// Get the display label (prefix + version)
    pub fn display_label(&self) -> String {
        if self.version.starts_with('v') || self.version.starts_with('V') || self.prefix.is_empty()
        {
            self.version.clone()
        } else {
            format!("{}{}", self.prefix, self.version)
        }
    }

    /// Get effective background color
    pub fn effective_bg_color(&self) -> &str {
        self.bg_color
            .as_deref()
            .map(|c| c.trim_start_matches('#'))
            .unwrap_or_else(|| self.status.bg_color())
    }

    /// Get effective text color
    pub fn effective_text_color(&self) -> &str {
        self.text_color
            .as_deref()
            .map(|c| c.trim_start_matches('#'))
            .unwrap_or_else(|| self.status.text_color())
    }
}

/// Builder for creating version badges
#[derive(Debug)]
pub struct VersionBuilder {
    badge: VersionBadge,
}

impl VersionBuilder {
    /// Create a new version badge builder
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            badge: VersionBadge::new(version),
        }
    }

    /// Override auto-detected status
    pub fn status(mut self, status: Status) -> Self {
        self.badge.status = status;
        self
    }

    /// Set badge style
    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.badge.style = style;
        self
    }

    /// Set custom background color
    pub fn bg_color(mut self, color: impl Into<String>) -> Self {
        self.badge.bg_color = Some(color.into());
        self
    }

    /// Set custom text color
    pub fn text_color(mut self, color: impl Into<String>) -> Self {
        self.badge.text_color = Some(color.into());
        self
    }

    /// Set version prefix (default: "v")
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.badge.prefix = prefix.into();
        self
    }

    /// Disable version prefix
    pub fn no_prefix(mut self) -> Self {
        self.badge.prefix = String::new();
        self
    }

    /// Add border
    pub fn border(mut self, color: impl Into<String>, width: u32) -> Self {
        self.badge.border_color = Some(color.into());
        self.badge.border_width = Some(width);
        self
    }

    /// Set corner radius
    pub fn rx(mut self, radius: u32) -> Self {
        self.badge.rx = Some(radius);
        self
    }

    /// Build the badge specification
    pub fn build(self) -> VersionBadge {
        self.badge
    }

    /// Render the badge to SVG string
    pub fn render(self) -> String {
        render(&self.build())
    }
}

/// Render a version badge to SVG
pub fn render(badge: &VersionBadge) -> String {
    let label = badge.display_label();
    let bg_color = badge.effective_bg_color();
    let text_color = badge.effective_text_color();

    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let rx = badge.rx.unwrap_or(metrics.radius as u32);

    // Calculate width: ~7px per char + padding
    let width = (label.len() as u32 * 7 + 16).max(40);
    let font_size = if height > 24 { 12 } else { 11 };
    let text_y = height / 2 + font_size / 3;

    // Border attributes
    let border_attr = if let (Some(color), Some(width)) = (&badge.border_color, badge.border_width)
    {
        let color = color.trim_start_matches('#');
        format!(" stroke=\"#{}\" stroke-width=\"{}\"", color, width)
    } else {
        String::new()
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <rect width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{} />\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"Verdana,Arial,sans-serif\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
        width,
        height,
        width,
        height,
        width,
        height,
        bg_color,
        rx,
        border_attr,
        width / 2,
        text_y,
        text_color,
        font_size,
        label
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Status Detection (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("1.0.0", Status::Stable)]
    #[case("2.5.3", Status::Stable)]
    #[case("10.0.0", Status::Stable)]
    #[case("1.0.0-beta", Status::Beta)]
    #[case("2.0.0-beta.1", Status::Beta)]
    #[case("1.0.0-rc.1", Status::Beta)]
    #[case("1.0.0-preview", Status::Beta)]
    #[case("0.5.0", Status::Beta)] // 0.x versions
    #[case("1.0.0-alpha", Status::Alpha)]
    #[case("1.0.0-alpha.2", Status::Alpha)]
    #[case("1.0.0-deprecated", Status::Deprecated)]
    #[case("1.0.0-eol", Status::Deprecated)]
    #[case("1.0.0-dev", Status::Dev)]
    #[case("1.0.0-snapshot", Status::Dev)]
    #[case("1.0.0-nightly", Status::Dev)]
    fn test_detect_status(#[case] version: &str, #[case] expected: Status) {
        assert_eq!(detect_status(version), expected);
    }

    // ========================================================================
    // Status Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("stable", Some(Status::Stable))]
    #[case("beta", Some(Status::Beta))]
    #[case("alpha", Some(Status::Alpha))]
    #[case("deprecated", Some(Status::Deprecated))]
    #[case("dev", Some(Status::Dev))]
    #[case("unknown", None)]
    fn test_parse_status(#[case] status: &str, #[case] expected: Option<Status>) {
        assert_eq!(parse_status(status), expected);
    }

    // ========================================================================
    // Display Label (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("1.0.0", "v", "v1.0.0")]
    #[case("v1.0.0", "v", "v1.0.0")] // already has prefix
    #[case("V2.0.0", "v", "V2.0.0")] // uppercase V
    #[case("1.0.0", "", "1.0.0")] // no prefix
    #[case("1.0.0", "ver", "ver1.0.0")]
    fn test_display_label(#[case] version: &str, #[case] prefix: &str, #[case] expected: &str) {
        let badge = VersionBadge {
            prefix: prefix.to_string(),
            ..VersionBadge::new(version)
        };
        assert_eq!(badge.display_label(), expected);
    }

    // ========================================================================
    // Builder Pattern
    // ========================================================================

    #[test]
    fn test_builder() {
        let badge = VersionBuilder::new("1.0.0")
            .status(Status::Deprecated)
            .bg_color("#FF0000")
            .text_color("#FFFFFF")
            .prefix("ver")
            .border("#000000", 2)
            .rx(4)
            .build();

        assert_eq!(badge.version, "1.0.0");
        assert_eq!(badge.status, Status::Deprecated);
        assert_eq!(badge.bg_color, Some("#FF0000".to_string()));
        assert_eq!(badge.text_color, Some("#FFFFFF".to_string()));
        assert_eq!(badge.prefix, "ver");
        assert_eq!(badge.border_color, Some("#000000".to_string()));
        assert_eq!(badge.border_width, Some(2));
        assert_eq!(badge.rx, Some(4));
    }

    #[test]
    fn test_no_prefix() {
        let badge = VersionBuilder::new("1.0.0").no_prefix().build();
        assert_eq!(badge.prefix, "");
        assert_eq!(badge.display_label(), "1.0.0");
    }

    // ========================================================================
    // Rendering
    // ========================================================================

    #[test]
    fn test_render_basic() {
        let svg = VersionBuilder::new("1.0.0").render();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("v1.0.0"));
        assert!(svg.contains("22C55E")); // stable green
    }

    #[test]
    fn test_render_beta() {
        let svg = VersionBuilder::new("2.0.0-beta").render();
        assert!(svg.contains("v2.0.0-beta"));
        assert!(svg.contains("EAB308")); // beta yellow
    }

    #[test]
    fn test_render_with_custom_colors() {
        let svg = VersionBuilder::new("1.0.0")
            .bg_color("#FF5500")
            .text_color("#000000")
            .render();
        assert!(svg.contains("FF5500"));
        assert!(svg.contains("000000"));
    }
}
