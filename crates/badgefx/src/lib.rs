//! # Badgery - Technology Badge Generation
//!
//! A powerful library for generating SVG technology badges with custom styling,
//! icons from Simple Icons, and flexible layouts.
//!
//! ## Quick Start
//!
//! ```rust
//! use badgefx::{badge, BadgeStyle};
//!
//! // Simple badge with defaults
//! let svg = badge("rust").render();
//!
//! // Customized badge  
//! let svg = badge("typescript")
//!     .label("TypeScript v5.0")
//!     .style(BadgeStyle::FlatSquare)
//!     .bg_color("#3178C6")
//!     .logo_size_lg()
//!     .render();
//! ```
//!
//! ## Features
//!
//! - **20+ Built-in Icons**: Popular tech icons from Simple Icons
//! - **Multiple Styles**: Flat, plastic, rounded, and more
//! - **Custom Colors**: Override brand colors or use custom palettes
//! - **Flexible Sizing**: From extra-small to extra-large logos
//! - **Chevron Shapes**: Directional arrows and custom badge shapes
//! - **Typography**: Custom fonts and text styling
//! - **Glyphs**: 500+ Unicode decorative characters (with "glyphs" feature)

pub mod badge;
pub mod group;
pub mod render;
pub mod shapes;
pub mod style;

#[cfg(feature = "glyphs")]
pub mod glyphs;

// Re-export main public API
pub use badge::{BadgeBuilder, LogoSize, TechBadge};
pub use render::{render, render_to_file};
pub use style::{BadgeStyle, Border, Chevron, Corners, SvgMetrics};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_creation() {
        let svg = badge("rust").render();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Rust")); // display_label capitalizes
    }

    #[test]
    fn test_simple_badge() {
        let svg = simple_badge("typescript");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("TypeScript"));
    }

    #[test]
    fn test_badge_customization() {
        let svg = badge("python")
            .label("Python 3.11")
            .style(BadgeStyle::FlatSquare)
            .bg_color("#3776AB")
            .logo_size_lg()
            .render();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("Python 3.11"));
    }
}
