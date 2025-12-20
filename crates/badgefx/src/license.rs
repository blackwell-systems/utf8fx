//! License badge generation
//!
//! Renders license badges with category-aware coloring.
//! Supports common SPDX license identifiers.
//!
//! # Examples
//!
//! ```rust
//! use badgefx::license;
//!
//! // Auto-detects category from license name
//! let svg = license("MIT").render();          // green (permissive)
//! let svg = license("GPL-3.0").render();      // yellow (copyleft)
//! let svg = license("LGPL-3.0").render();     // blue (weak copyleft)
//! let svg = license("CC0").render();          // cyan (public domain)
//!
//! // Override auto-detection
//! use badgefx::license::Category;
//! let svg = license("Custom")
//!     .category(Category::Proprietary)
//!     .render();
//! ```

use crate::style::{BadgeStyle, SvgMetrics};

/// License categories for coloring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// Green - MIT, Apache, BSD, ISC
    Permissive,
    /// Blue - LGPL, MPL, EPL
    WeakCopyleft,
    /// Yellow - GPL, AGPL
    Copyleft,
    /// Gray - closed source
    Proprietary,
    /// Cyan - CC0, Unlicense
    PublicDomain,
    /// Slate - unrecognized
    Unknown,
}

impl Category {
    /// Get the background color for this category (hex without #)
    pub fn bg_color(&self) -> &'static str {
        match self {
            Category::Permissive => "22C55E",   // success green
            Category::WeakCopyleft => "3B82F6", // info blue
            Category::Copyleft => "EAB308",     // warning yellow
            Category::Proprietary => "6B7280",  // gray
            Category::PublicDomain => "06B6D4", // cyan
            Category::Unknown => "475569",      // slate
        }
    }

    /// Get the text color for this category (hex without #)
    pub fn text_color(&self) -> &'static str {
        match self {
            Category::Permissive => "FFFFFF",
            Category::WeakCopyleft => "FFFFFF",
            Category::Copyleft => "000000",
            Category::Proprietary => "FFFFFF",
            Category::PublicDomain => "000000",
            Category::Unknown => "FFFFFF",
        }
    }
}

/// Categorize a license by its SPDX identifier or common name
pub fn categorize(license: &str) -> Category {
    let upper = license.to_uppercase();
    let normalized = upper.replace(['-', ' '], "");

    // Permissive licenses
    if matches!(
        normalized.as_str(),
        "MIT"
            | "APACHE"
            | "APACHE2"
            | "APACHE20"
            | "APACHE2.0"
            | "BSD"
            | "BSD2"
            | "BSD2CLAUSE"
            | "BSD3"
            | "BSD3CLAUSE"
            | "ISC"
            | "ZLIB"
            | "WTFPL"
            | "BOOST"
            | "BSL"
            | "BSL1"
            | "BSL10"
            | "BSL1.0"
    ) {
        return Category::Permissive;
    }

    // Weak copyleft licenses
    if normalized.starts_with("LGPL")
        || normalized.starts_with("MPL")
        || normalized.starts_with("EPL")
        || normalized.starts_with("CDDL")
    {
        return Category::WeakCopyleft;
    }

    // Strong copyleft licenses
    if normalized.starts_with("GPL")
        || normalized.starts_with("AGPL")
        || normalized.starts_with("SSPL")
    {
        return Category::Copyleft;
    }

    // Public domain
    if matches!(
        normalized.as_str(),
        "CC0" | "CC01" | "CC010" | "CC0UNIVERSAL" | "UNLICENSE" | "PUBLICDOMAIN" | "PD"
    ) {
        return Category::PublicDomain;
    }

    // Proprietary/closed source
    if matches!(
        normalized.as_str(),
        "PROPRIETARY" | "COMMERCIAL" | "CLOSED" | "ALLRIGHTSRESERVED" | "ARR"
    ) {
        return Category::Proprietary;
    }

    Category::Unknown
}

/// Format license name for display
pub fn format_name(license: &str) -> String {
    let upper = license.to_uppercase();

    match upper.as_str() {
        "MIT" => "MIT".to_string(),
        "APACHE-2.0" | "APACHE2.0" | "APACHE2" | "APACHE-2" => "Apache 2.0".to_string(),
        "GPL-3.0" | "GPL3.0" | "GPL3" | "GPL-3" => "GPL 3.0".to_string(),
        "GPL-2.0" | "GPL2.0" | "GPL2" | "GPL-2" => "GPL 2.0".to_string(),
        "LGPL-3.0" | "LGPL3.0" | "LGPL3" | "LGPL-3" => "LGPL 3.0".to_string(),
        "LGPL-2.1" | "LGPL2.1" | "LGPL21" | "LGPL-21" => "LGPL 2.1".to_string(),
        "AGPL-3.0" | "AGPL3.0" | "AGPL3" | "AGPL-3" => "AGPL 3.0".to_string(),
        "BSD-3-CLAUSE" | "BSD3CLAUSE" | "BSD3" | "BSD-3" => "BSD 3-Clause".to_string(),
        "BSD-2-CLAUSE" | "BSD2CLAUSE" | "BSD2" | "BSD-2" => "BSD 2-Clause".to_string(),
        "MPL-2.0" | "MPL2.0" | "MPL2" | "MPL-2" => "MPL 2.0".to_string(),
        "CC0-1.0" | "CC010" | "CC0" => "CC0".to_string(),
        "UNLICENSE" => "Unlicense".to_string(),
        "ISC" => "ISC".to_string(),
        "WTFPL" => "WTFPL".to_string(),
        _ => license.to_string(), // Keep original casing
    }
}

/// License badge specification
#[derive(Debug, Clone)]
pub struct LicenseBadge {
    /// License identifier (e.g., "MIT", "GPL-3.0")
    pub license: String,
    /// Category (auto-detected or overridden)
    pub category: Category,
    /// Visual style of the badge
    pub style: BadgeStyle,
    /// Custom label (overrides formatted name)
    pub label: Option<String>,
    /// Custom background color (overrides category color)
    pub bg_color: Option<String>,
    /// Custom text color
    pub text_color: Option<String>,
    /// Border color
    pub border_color: Option<String>,
    /// Border width in pixels
    pub border_width: Option<u32>,
    /// Corner radius
    pub rx: Option<u32>,
}

impl LicenseBadge {
    /// Create a new license badge with auto-detected category
    pub fn new(license: impl Into<String>) -> Self {
        let license = license.into();
        let category = categorize(&license);
        Self {
            license,
            category,
            style: BadgeStyle::default(),
            label: None,
            bg_color: None,
            text_color: None,
            border_color: None,
            border_width: None,
            rx: None,
        }
    }

    /// Get the display label
    pub fn display_label(&self) -> String {
        self.label
            .clone()
            .unwrap_or_else(|| format_name(&self.license))
    }

    /// Get effective background color
    pub fn effective_bg_color(&self) -> &str {
        self.bg_color
            .as_deref()
            .map(|c| c.trim_start_matches('#'))
            .unwrap_or_else(|| self.category.bg_color())
    }

    /// Get effective text color
    pub fn effective_text_color(&self) -> &str {
        self.text_color
            .as_deref()
            .map(|c| c.trim_start_matches('#'))
            .unwrap_or_else(|| self.category.text_color())
    }
}

/// Builder for creating license badges
#[derive(Debug)]
pub struct LicenseBuilder {
    badge: LicenseBadge,
}

impl LicenseBuilder {
    /// Create a new license badge builder
    pub fn new(license: impl Into<String>) -> Self {
        Self {
            badge: LicenseBadge::new(license),
        }
    }

    /// Override auto-detected category
    pub fn category(mut self, category: Category) -> Self {
        self.badge.category = category;
        self
    }

    /// Set badge style
    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.badge.style = style;
        self
    }

    /// Set custom label (overrides formatted name)
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.badge.label = Some(label.into());
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
    pub fn build(self) -> LicenseBadge {
        self.badge
    }

    /// Render the badge to SVG string
    pub fn render(self) -> String {
        render(&self.build())
    }
}

/// Render a license badge to SVG
pub fn render(badge: &LicenseBadge) -> String {
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
    // License Categorization (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("MIT", Category::Permissive)]
    #[case("Apache-2.0", Category::Permissive)]
    #[case("BSD-3-Clause", Category::Permissive)]
    #[case("ISC", Category::Permissive)]
    #[case("LGPL-3.0", Category::WeakCopyleft)]
    #[case("MPL-2.0", Category::WeakCopyleft)]
    #[case("EPL-2.0", Category::WeakCopyleft)]
    #[case("GPL-3.0", Category::Copyleft)]
    #[case("AGPL-3.0", Category::Copyleft)]
    #[case("GPL-2.0", Category::Copyleft)]
    #[case("CC0", Category::PublicDomain)]
    #[case("Unlicense", Category::PublicDomain)]
    #[case("Proprietary", Category::Proprietary)]
    #[case("Commercial", Category::Proprietary)]
    #[case("SomeUnknown", Category::Unknown)]
    fn test_categorize(#[case] license: &str, #[case] expected: Category) {
        assert_eq!(categorize(license), expected);
    }

    // ========================================================================
    // Name Formatting (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("MIT", "MIT")]
    #[case("apache-2.0", "Apache 2.0")]
    #[case("GPL-3.0", "GPL 3.0")]
    #[case("BSD-3-Clause", "BSD 3-Clause")]
    #[case("CC0", "CC0")]
    #[case("CustomLicense", "CustomLicense")]
    fn test_format_name(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(format_name(input), expected);
    }

    // ========================================================================
    // Builder Pattern
    // ========================================================================

    #[test]
    fn test_builder() {
        let badge = LicenseBuilder::new("MIT")
            .category(Category::Proprietary)
            .label("Custom Label")
            .bg_color("#FF0000")
            .text_color("#FFFFFF")
            .border("#000000", 2)
            .rx(4)
            .build();

        assert_eq!(badge.license, "MIT");
        assert_eq!(badge.category, Category::Proprietary);
        assert_eq!(badge.label, Some("Custom Label".to_string()));
        assert_eq!(badge.bg_color, Some("#FF0000".to_string()));
        assert_eq!(badge.text_color, Some("#FFFFFF".to_string()));
        assert_eq!(badge.border_color, Some("#000000".to_string()));
        assert_eq!(badge.border_width, Some(2));
        assert_eq!(badge.rx, Some(4));
    }

    // ========================================================================
    // Rendering
    // ========================================================================

    #[test]
    fn test_render_mit() {
        let svg = LicenseBuilder::new("MIT").render();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("MIT"));
        assert!(svg.contains("22C55E")); // permissive green
    }

    #[test]
    fn test_render_gpl() {
        let svg = LicenseBuilder::new("GPL-3.0").render();
        assert!(svg.contains("GPL 3.0")); // formatted
        assert!(svg.contains("EAB308")); // copyleft yellow
    }

    #[test]
    fn test_render_with_custom_colors() {
        let svg = LicenseBuilder::new("MIT")
            .bg_color("#FF5500")
            .text_color("#000000")
            .render();
        assert!(svg.contains("FF5500"));
        assert!(svg.contains("000000"));
    }

    #[test]
    fn test_render_with_custom_label() {
        let svg = LicenseBuilder::new("MIT").label("Open Source").render();
        assert!(svg.contains("Open Source"));
        assert!(!svg.contains(">MIT<"));
    }
}
