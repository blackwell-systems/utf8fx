//! # badgefx - Badge Generation Library
//!
//! A powerful library for generating SVG badges: technology badges, version badges,
//! and license badges with custom styling.
//!
//! ## Quick Start
//!
//! ```rust
//! use badgefx::{badge, version, license, BadgeStyle};
//!
//! // Technology badge (with Simple Icons)
//! let svg = badge("rust").render();
//! let svg = badge("typescript")
//!     .label("TypeScript v5.0")
//!     .style(BadgeStyle::FlatSquare)
//!     .render();
//!
//! // Version badge (auto-detects status)
//! let svg = version("1.0.0").render();        // green (stable)
//! let svg = version("2.0.0-beta").render();   // yellow (beta)
//!
//! // License badge (auto-detects category)
//! let svg = license("MIT").render();          // green (permissive)
//! let svg = license("GPL-3.0").render();      // yellow (copyleft)
//! ```
//!
//! ## Badge Types
//!
//! - **Tech badges**: Technology/language badges with Simple Icons
//! - **Version badges**: Semantic version with status coloring
//! - **License badges**: SPDX license with category coloring
//!
//! ## Features
//!
//! - **500+ Tech Icons**: Popular tech icons from Simple Icons
//! - **Multiple Styles**: Flat, plastic, rounded, and more
//! - **Custom Colors**: Override brand colors or use custom palettes
//! - **Chevron Shapes**: Directional arrows and custom badge shapes
//! - **Typography**: Custom fonts and text styling
//! - **Glyphs**: 500+ Unicode decorative characters (with "glyphs" feature)

pub mod badge;
pub mod group;
pub mod license;
pub mod render;
pub mod shapes;
pub mod style;
pub mod version;

#[cfg(feature = "glyphs")]
pub mod glyphs;

// Re-export main public API
pub use badge::{BadgeBuilder, TechBadge};
pub use license::{LicenseBadge, LicenseBuilder};
pub use render::{render, render_to_file};
pub use style::{BadgeStyle, Border, Chevron, Corners, SvgMetrics};
pub use version::{VersionBadge, VersionBuilder};

/// Create a new badge builder for the given technology name
///
/// This is the main entry point for badge creation. The technology name
/// should match one of the supported icons from Simple Icons.
///
/// # Examples
///
/// ```
/// use badgefx::badge;
///
/// let svg = badge("rust")
///     .label("Rust 1.70")
///     .render();
/// ```
pub fn badge(name: &str) -> BadgeBuilder {
    BadgeBuilder::new(name)
}

/// Convenience function to render a simple badge with just the icon
///
/// # Examples
///
/// ```
/// use badgefx::simple_badge;
///
/// let rust_badge = simple_badge("rust");
/// let typescript_badge = simple_badge("typescript");
/// ```
pub fn simple_badge(name: &str) -> String {
    badge(name).render()
}

/// Create a new version badge builder
///
/// The version string is parsed to auto-detect the status:
/// - Stable (green): 1.x.x, 2.x.x, etc.
/// - Beta (yellow): 0.x.x, -beta, -rc, -preview
/// - Alpha (orange): -alpha
/// - Dev (purple): -dev, -snapshot, -nightly
/// - Deprecated (red): -deprecated, -eol
///
/// # Examples
///
/// ```
/// use badgefx::version;
///
/// let svg = version("1.0.0").render();        // green
/// let svg = version("2.0.0-beta").render();   // yellow
/// let svg = version("0.5.0").render();        // yellow (0.x = beta)
/// ```
pub fn version(ver: &str) -> VersionBuilder {
    VersionBuilder::new(ver)
}

/// Create a new license badge builder
///
/// The license name is parsed to auto-detect the category:
/// - Permissive (green): MIT, Apache, BSD, ISC
/// - Weak Copyleft (blue): LGPL, MPL, EPL
/// - Copyleft (yellow): GPL, AGPL
/// - Public Domain (cyan): CC0, Unlicense
/// - Proprietary (gray): Proprietary, Commercial
///
/// # Examples
///
/// ```
/// use badgefx::license;
///
/// let svg = license("MIT").render();          // green
/// let svg = license("GPL-3.0").render();      // yellow
/// let svg = license("CC0").render();          // cyan
/// ```
pub fn license(name: &str) -> LicenseBuilder {
    LicenseBuilder::new(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_creation() {
        let svg = badge("rust").render();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("rust")); // lowercase matches original mdfx behavior
    }

    #[test]
    fn test_simple_badge() {
        let svg = simple_badge("typescript");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("typescript")); // lowercase matches original mdfx behavior
    }

    #[test]
    fn test_badge_customization() {
        let svg = badge("python")
            .label("Python 3.11")
            .style(BadgeStyle::FlatSquare)
            .bg_color("#3776AB")
            .render();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("Python 3.11"));
    }

    #[test]
    fn test_version_badge() {
        let svg = version("1.0.0").render();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("v1.0.0"));
        assert!(svg.contains("22C55E")); // stable green
    }

    #[test]
    fn test_version_badge_beta() {
        let svg = version("2.0.0-beta").render();
        assert!(svg.contains("v2.0.0-beta"));
        assert!(svg.contains("EAB308")); // beta yellow
    }

    #[test]
    fn test_license_badge() {
        let svg = license("MIT").render();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("MIT"));
        assert!(svg.contains("22C55E")); // permissive green
    }

    #[test]
    fn test_license_badge_copyleft() {
        let svg = license("GPL-3.0").render();
        assert!(svg.contains("GPL 3.0")); // formatted
        assert!(svg.contains("EAB308")); // copyleft yellow
    }
}
