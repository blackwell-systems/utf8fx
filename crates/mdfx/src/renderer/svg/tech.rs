//! Tech badge SVG renderer - delegates to badgefx for rendering
//!
//! This module provides the tech badge rendering API for mdfx, delegating
//! the actual SVG generation to the badgefx crate.

use badgefx::{BadgeBuilder, BadgeStyle, Chevron, Corners};

/// Get brand color for a given technology name
/// Colors sourced from https://simpleicons.org/
///
/// This is a thin wrapper around mdfx-icons for API compatibility.
pub fn get_brand_color(name: &str) -> Option<&'static str> {
    mdfx_icons::brand_color(name)
}

/// Get the ideal logo color (white or black) for contrast against background
/// Uses relative luminance calculation for accessibility
pub fn get_logo_color_for_bg(bg_hex: &str) -> &'static str {
    let hex = bg_hex.trim_start_matches('#');
    if hex.len() < 6 {
        return "FFFFFF";
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;

    // Calculate relative luminance (ITU-R BT.709)
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

    // Use black text on light backgrounds, white on dark
    if luminance > 0.5 {
        "000000"
    } else {
        "FFFFFF"
    }
}

/// Render a tech badge with full options
///
/// Supports:
/// - Icon only or Icon + label layouts
/// - Custom border color and width
/// - Custom corner radius (uniform or per-corner)
/// - Custom text color and font
/// - Chevron/arrow shapes for tab-style badges
/// - Independent segment background colors
/// - Outline/ghost style (transparent fill with border)
#[allow(clippy::too_many_arguments)]
pub fn render_with_options(
    name: &str,
    label: Option<&str>,
    bg_color: &str,
    logo_color: &str,
    style: &str,
    border_color: Option<&str>,
    border_width: Option<u32>,
    rx: Option<u32>,
    corners: Option<[u32; 4]>,
    text_color: Option<&str>,
    font: Option<&str>,
    chevron: Option<&str>,
    bg_left: Option<&str>,
    bg_right: Option<&str>,
    raised: Option<u32>,
) -> String {
    // Build badge using badgefx
    let mut builder = BadgeBuilder::new(name);

    // Set label - None means icon-only mode (empty label)
    // Some("text") means use that label
    match label {
        Some(l) => builder = builder.label(l),
        None => builder = builder.label(""), // Empty label for icon-only
    }

    // Parse and set style
    let badge_style = BadgeStyle::parse(style);
    builder = builder.style(badge_style);

    // Set background color - ensure it has # prefix for badgefx
    let bg = ensure_hash_prefix(bg_color);
    builder = builder.bg_color(&bg);

    // Set logo color - ensure it has # prefix
    let logo = ensure_hash_prefix(logo_color);
    builder = builder.logo_color(&logo);

    // Set segment colors if provided
    if let Some(left) = bg_left {
        builder = builder.bg_left(ensure_hash_prefix(left));
    }
    if let Some(right) = bg_right {
        builder = builder.bg_right(ensure_hash_prefix(right));
    }

    // Set border if specified
    // Note: outline/ghost style defaults to border_width=2, others default to 1
    let is_outline = matches!(style.to_lowercase().as_str(), "outline" | "ghost");
    let default_border_width = if is_outline { 2 } else { 1 };

    if let Some(color) = border_color {
        let width = border_width.unwrap_or(default_border_width);
        builder = builder.border(ensure_hash_prefix(color), width);
    } else if let Some(width) = border_width {
        // Border width without color - use default border color
        builder = builder.border("#FFFFFF", width);
    }

    // Set corners if specified
    if let Some([tl, tr, br, bl]) = corners {
        builder = builder.corners(Corners::custom(tl, tr, br, bl));
    } else if let Some(radius) = rx {
        builder = builder.corners(Corners::uniform(radius));
    }

    // Set text color if specified
    if let Some(color) = text_color {
        builder = builder.text_color(ensure_hash_prefix(color));
    }

    // Set font if specified
    if let Some(f) = font {
        builder = builder.font(f);
    }

    // Set chevron if specified
    if let Some(chevron_type) = chevron {
        let depth = 10.0; // Match original CHEVRON_ARROW_DEPTH constant
        let chev = match chevron_type {
            "left" => Chevron::left(depth),
            "right" => Chevron::right(depth),
            "both" => Chevron::both(depth),
            _ => Chevron::right(depth),
        };
        builder = builder.chevron(chev);
    }

    // Set raised if specified (icon section taller than label)
    if let Some(px) = raised {
        builder = builder.raised(px);
    }

    // Handle outline/ghost style
    if matches!(style.to_lowercase().as_str(), "outline" | "ghost") {
        builder = builder.outline();
    }

    // Render and return
    builder.render()
}

/// Ensure a hex color string has a # prefix
fn ensure_hash_prefix(color: &str) -> String {
    if color.starts_with('#') {
        color.to_string()
    } else {
        format!("#{}", color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_brand_color() {
        assert_eq!(get_brand_color("rust"), Some("DEA584"));
        assert_eq!(get_brand_color("typescript"), Some("3178C6"));
        assert_eq!(get_brand_color("unknown"), None);
    }

    #[test]
    fn test_get_logo_color_for_bg() {
        // Dark backgrounds -> white logo
        assert_eq!(get_logo_color_for_bg("000000"), "FFFFFF");
        assert_eq!(get_logo_color_for_bg("#000000"), "FFFFFF");
        assert_eq!(get_logo_color_for_bg("3178C6"), "FFFFFF");

        // Light backgrounds -> black logo
        assert_eq!(get_logo_color_for_bg("FFFFFF"), "000000");
        assert_eq!(get_logo_color_for_bg("#FFFFFF"), "000000");
        assert_eq!(get_logo_color_for_bg("F7DF1E"), "000000");
    }

    #[test]
    fn test_ensure_hash_prefix() {
        assert_eq!(ensure_hash_prefix("FF0000"), "#FF0000");
        assert_eq!(ensure_hash_prefix("#FF0000"), "#FF0000");
    }

    #[test]
    fn test_render_basic_badge() {
        let svg = render_with_options(
            "rust",
            Some("rust"),
            "DEA584",
            "FFFFFF",
            "flat-square",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_render_with_border() {
        let svg = render_with_options(
            "rust",
            Some("rust"),
            "DEA584",
            "FFFFFF",
            "flat-square",
            Some("FF0000"),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(svg.contains("<svg"));
        assert!(svg.contains("stroke"));
    }

    #[test]
    fn test_render_outline_style() {
        let svg = render_with_options(
            "rust",
            Some("rust"),
            "DEA584",
            "FFFFFF",
            "outline",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        assert!(svg.contains("<svg"));
        // Outline style should have stroke but no fill on the background
        assert!(svg.contains("stroke"));
    }

    #[test]
    fn test_render_with_chevron() {
        let svg = render_with_options(
            "rust",
            Some("rust"),
            "DEA584",
            "FFFFFF",
            "flat-square",
            None,
            None,
            None,
            None,
            None,
            None,
            Some("right"),
            None,
            None,
            None,
        );

        assert!(svg.contains("<svg"));
        // Chevron badges use path elements
        assert!(svg.contains("<path"));
    }

    #[test]
    fn test_render_raised_badge() {
        let svg = render_with_options(
            "rust",
            Some("rust"),
            "DEA584",
            "000000",
            "flat",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(4), // 4px raised
        );

        assert!(svg.contains("<svg"));
        // Raised badge should have increased height (20 + 4*2 = 28)
        assert!(svg.contains("height=\"28\""));
    }
}
