//! SVG rendering for technology badges - extracted from mdfx tech.rs
//! This implementation produces pixel-perfect identical output to the original mdfx renderer

use std::io;
use std::path::Path;

use crate::badge::{LogoSize, TechBadge};
use crate::shapes::rounded_rect_path;
use crate::style::SvgMetrics;

/// Render a badge to SVG string (pixel-perfect match to original mdfx)
pub fn render(badge: &TechBadge) -> String {
    // Get the label - keep lowercase if no explicit label set (matching original behavior)
    let label = badge.label.as_deref().unwrap_or(&badge.name);

    // Check if we have an icon (custom_icon takes priority)
    let icon_path: Option<&str> = badge
        .custom_icon
        .as_deref()
        .or_else(|| mdfx_icons::icon_path(&badge.name));

    // Handle outline mode separately for proper rendering
    if badge.outline {
        let brand_color = badge
            .effective_bg_color()
            .unwrap_or_else(|| "#555".to_string());
        let brand_color = brand_color.trim_start_matches('#');

        return match (icon_path, !label.is_empty()) {
            (Some(path), true) => render_outline_two_segment(badge, path, label, brand_color),
            (Some(path), false) => render_outline_icon_only(badge, path, brand_color),
            (None, _) => render_outline_text_only(badge, label, brand_color),
        };
    }

    // Get colors
    let bg_color = badge
        .effective_bg_color()
        .unwrap_or_else(|| "#555".to_string());
    let bg_color = bg_color.trim_start_matches('#');
    let logo_color = badge
        .logo_color
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .or_else(|| mdfx_icons::brand_contrast_color(&badge.name))
        .unwrap_or("FFFFFF");

    match (icon_path, !label.is_empty()) {
        // Icon + Label: Two-segment badge
        (Some(path), true) => render_two_segment(badge, path, label, bg_color, logo_color),
        // Icon only: Single segment with centered icon
        (Some(path), false) => render_icon_only(badge, path, bg_color, logo_color),
        // No icon found: Fallback to text
        (None, _) => render_text_only(badge, label, bg_color),
    }
}

/// Get effective border attribute for inline stroke
fn get_border_attr(badge: &TechBadge) -> String {
    if let Some(border) = &badge.border {
        let color = border.color.trim_start_matches('#');
        format!(" stroke=\"#{}\" stroke-width=\"{}\"", color, border.width)
    } else {
        String::new()
    }
}

/// Render two-segment badge: icon left, label right
fn render_two_segment(
    badge: &TechBadge,
    icon_path: &str,
    label: &str,
    bg_color: &str,
    logo_color: &str,
) -> String {
    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    // Use hardcoded values for default logo size (matching original mdfx)
    let (icon_width, icon_size): (u32, u32) = if badge.logo_size == LogoSize::Md {
        (36, 14)
    } else {
        let size = badge.logo_size.to_pixels();
        ((size as f32 * 2.5).ceil() as u32 + 1, size)
    };

    let label_width = estimate_text_width(label) + 16;
    let total_width = icon_width + label_width;
    let icon_x = (icon_width as f32 - icon_size as f32) / 2.0;
    let icon_y = (height as f32 - icon_size as f32) / 2.0;
    let font_size = if height > 24 { 11 } else { 10 };
    let text_x = icon_width + label_width / 2;
    let text_y = height / 2 + font_size / 3;
    let scale = icon_size as f32 / 24.0;

    // Segment colors
    let left_bg = badge
        .bg_left
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .unwrap_or(bg_color);
    let default_right_bg = darken_color(bg_color, 0.15);
    let right_bg = badge
        .bg_right
        .as_deref()
        .map(|c| c.trim_start_matches('#').to_string())
        .unwrap_or(default_right_bg);

    // Text color
    let text_color = badge
        .text_color
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .unwrap_or_else(|| get_logo_color_for_bg(&right_bg));

    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");
    let border_attr = get_border_attr(badge);

    // Handle chevron shapes
    if let Some(chevron) = &badge.chevron {
        let arrow = chevron.depth;
        let h = height as f32;
        let center_y = h / 2.0;

        // Determine arrow directions
        let has_left_arr = chevron.direction.has_left();
        let has_right_arr = chevron.direction.has_right();

        // Calculate viewBox dimensions
        let vb_width = total_width as f32
            + if has_left_arr { arrow } else { 0.0 }
            + if has_right_arr { arrow } else { 0.0 };
        let svg_width = vb_width as u32;

        let content_offset = if has_left_arr { arrow } else { 0.0 };
        let left_x = content_offset;
        let right_x = content_offset + icon_width as f32;

        // Left segment path
        let left_path = if has_left_arr {
            format!(
                "M{tip} {cy}L{x} 0H{right}V{h}H{x}L{tip} {cy}Z",
                tip = 0.0,
                cy = center_y,
                x = left_x,
                right = left_x + icon_width as f32,
                h = h
            )
        } else {
            format!(
                "M{x} 0H{right}V{h}H{x}Z",
                x = left_x,
                right = left_x + icon_width as f32,
                h = h
            )
        };

        // Right segment path
        let right_path = if has_right_arr {
            format!(
                "M{x} 0H{base}L{tip} {cy}L{base} {h}H{x}Z",
                x = right_x,
                base = right_x + label_width as f32,
                tip = right_x + label_width as f32 + arrow,
                cy = center_y,
                h = h
            )
        } else {
            format!(
                "M{x} 0H{right}V{h}H{x}Z",
                x = right_x,
                right = right_x + label_width as f32,
                h = h
            )
        };

        return format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <path d=\"{}\" fill=\"#{}\"{}/>  \n\
  <path d=\"{}\" fill=\"#{}\"/>  \n\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
            svg_width, height, vb_width, height,
            left_path, left_bg, border_attr,
            right_path, right_bg,
            icon_x + content_offset, icon_y, scale,
            logo_color, icon_path,
            text_x as f32 + content_offset, text_y, text_color, font_family, font_size, label
        );
    }

    // Generate segments based on corners
    let (left_segment, right_segment) = if let Some(corners) = &badge.corners {
        // Per-corner radii: split into left and right segments
        let left_corners = [corners.top_left, 0, 0, corners.bottom_left];
        let right_corners = [0, corners.top_right, corners.bottom_right, 0];
        let left_path = rounded_rect_path(0.0, 0.0, icon_width as f32, height as f32, left_corners);
        let right_path = rounded_rect_path(
            icon_width as f32,
            0.0,
            label_width as f32,
            height as f32,
            right_corners,
        );
        (
            format!(
                "<path d=\"{}\" fill=\"#{}\"{}/>",
                left_path, left_bg, border_attr
            ),
            format!("<path d=\"{}\" fill=\"#{}\"/>", right_path, right_bg),
        )
    } else {
        // Uniform radius: use original 3-rect approach
        (
            format!(
                "<rect width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>\n\
  <rect x=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"0\"/>",
                total_width,
                height,
                left_bg,
                rx,
                border_attr,
                icon_width,
                label_width,
                height,
                right_bg
            ),
            format!(
                "<rect x=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>",
                total_width - rx,
                rx,
                height,
                right_bg,
                rx
            ),
        )
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\n\
  {}\n\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
        total_width, height, total_width, height,
        left_segment,
        right_segment,
        icon_x, icon_y, scale,
        logo_color, icon_path,
        text_x, text_y, text_color, font_family, font_size, label
    )
}

/// Render icon-only badge
fn render_icon_only(badge: &TechBadge, icon_path: &str, bg_color: &str, logo_color: &str) -> String {
    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let width: u32 = 40;
    let icon_size: u32 = 16;
    let icon_x = (width as f32 - icon_size as f32) / 2.0;
    let icon_y = (height as f32 - icon_size as f32) / 2.0;
    let scale = icon_size as f32 / 24.0;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    let border_attr = get_border_attr(badge);

    // Generate background
    let bg = if let Some(corners) = &badge.corners {
        let c = [
            corners.top_left,
            corners.top_right,
            corners.bottom_right,
            corners.bottom_left,
        ];
        format!(
            "<path d=\"{}\" fill=\"#{}\"{}/>",
            rounded_rect_path(0.0, 0.0, width as f32, height as f32, c),
            bg_color,
            border_attr
        )
    } else {
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>\n\
  <rect x=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"0\"/>",
            width, height, bg_color, rx, border_attr, 0, width, height, bg_color
        )
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\n\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
</svg>",
        width, height, width, height, bg, icon_x, icon_y, scale, logo_color, icon_path
    )
}

/// Render text-only badge
fn render_text_only(badge: &TechBadge, label: &str, bg_color: &str) -> String {
    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let width = estimate_text_width(label) + 20;
    let font_size = if height > 24 { 12 } else { 11 };
    let text_y = height / 2 + font_size / 3;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    let text_color = badge
        .text_color
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .unwrap_or_else(|| get_logo_color_for_bg(bg_color));

    let border_attr = get_border_attr(badge);

    let bg = if let Some(corners) = &badge.corners {
        let c = [
            corners.top_left,
            corners.top_right,
            corners.bottom_right,
            corners.bottom_left,
        ];
        format!(
            "<path d=\"{}\" fill=\"#{}\"{}/>",
            rounded_rect_path(0.0, 0.0, width as f32, height as f32, c),
            bg_color,
            border_attr
        )
    } else {
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>\n\
  <rect x=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"0\"/>",
            width, height, bg_color, rx, border_attr, 0, width, height, bg_color
        )
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"Verdana,Arial,sans-serif\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
        width,
        height,
        width,
        height,
        bg,
        width / 2,
        text_y,
        text_color,
        font_size,
        label.to_uppercase()
    )
}

/// Render outline/ghost style two-segment badge
fn render_outline_two_segment(
    badge: &TechBadge,
    icon_path: &str,
    label: &str,
    brand_color: &str,
) -> String {
    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    let icon_width: u32 = 36;
    let label_width = estimate_text_width(label) + 16;
    let total_width = icon_width + label_width;
    let icon_size: u32 = 14;
    let icon_x = (icon_width as f32 - icon_size as f32) / 2.0;
    let icon_y = (height as f32 - icon_size as f32) / 2.0;
    let font_size = if height > 24 { 11 } else { 10 };
    let text_x = icon_width + label_width / 2;
    let text_y = height / 2 + font_size / 3;
    let scale = icon_size as f32 / 24.0;

    // For outline style, use brand color for icon and text
    let stroke_color = badge
        .border
        .as_ref()
        .map(|b| b.color.trim_start_matches('#'))
        .unwrap_or(brand_color);
    let stroke_width = badge.border.as_ref().map(|b| b.width).unwrap_or(2);
    let icon_color = brand_color;
    let text_color = badge
        .text_color
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .unwrap_or(brand_color);
    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");

    // Generate outline background
    let bg = if let Some(corners) = &badge.corners {
        let c = [
            corners.top_left,
            corners.top_right,
            corners.bottom_right,
            corners.bottom_left,
        ];
        format!(
            "<path d=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\"/>",
            rounded_rect_path(0.0, 0.0, total_width as f32, height as f32, c),
            stroke_color,
            stroke_width
        )
    } else {
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" rx=\"{}\"/>",
            total_width, height, stroke_color, stroke_width, rx
        )
    };

    // Add vertical separator line
    let separator = format!(
        "<line x1=\"{}\" y1=\"0\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\"/>",
        icon_width, icon_width, height, stroke_color, stroke_width
    );

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\n\
  {}\n\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
        total_width, height, total_width, height,
        bg,
        separator,
        icon_x, icon_y, scale,
        icon_color, icon_path,
        text_x, text_y, text_color, font_family, font_size, label
    )
}

/// Render outline-style icon-only badge
fn render_outline_icon_only(badge: &TechBadge, icon_path: &str, brand_color: &str) -> String {
    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let width: u32 = 40;
    let icon_size: u32 = 16;
    let icon_x = (width as f32 - icon_size as f32) / 2.0;
    let icon_y = (height as f32 - icon_size as f32) / 2.0;
    let scale = icon_size as f32 / 24.0;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    let stroke_color = badge
        .border
        .as_ref()
        .map(|b| b.color.trim_start_matches('#'))
        .unwrap_or(brand_color);
    let stroke_width = badge.border.as_ref().map(|b| b.width).unwrap_or(2);
    let icon_color = brand_color;

    let bg = if let Some(corners) = &badge.corners {
        let c = [
            corners.top_left,
            corners.top_right,
            corners.bottom_right,
            corners.bottom_left,
        ];
        format!(
            "<path d=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\"/>",
            rounded_rect_path(0.0, 0.0, width as f32, height as f32, c),
            stroke_color,
            stroke_width
        )
    } else {
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" rx=\"{}\"/>",
            width, height, stroke_color, stroke_width, rx
        )
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\n\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
</svg>",
        width, height, width, height, bg, icon_x, icon_y, scale, icon_color, icon_path
    )
}

/// Render outline-style text-only badge
fn render_outline_text_only(badge: &TechBadge, label: &str, brand_color: &str) -> String {
    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let width = estimate_text_width(label) + 20;
    let font_size = if height > 24 { 12 } else { 11 };
    let text_y = height / 2 + font_size / 3;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    let stroke_color = badge
        .border
        .as_ref()
        .map(|b| b.color.trim_start_matches('#'))
        .unwrap_or(brand_color);
    let stroke_width = badge.border.as_ref().map(|b| b.width).unwrap_or(2);
    let text_color = badge
        .text_color
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .unwrap_or(brand_color);

    let bg = if let Some(corners) = &badge.corners {
        let c = [
            corners.top_left,
            corners.top_right,
            corners.bottom_right,
            corners.bottom_left,
        ];
        format!(
            "<path d=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\"/>",
            rounded_rect_path(0.0, 0.0, width as f32, height as f32, c),
            stroke_color,
            stroke_width
        )
    } else {
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" rx=\"{}\"/>",
            width, height, stroke_color, stroke_width, rx
        )
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"Verdana,Arial,sans-serif\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
        width,
        height,
        width,
        height,
        bg,
        width / 2,
        text_y,
        text_color,
        font_size,
        label.to_uppercase()
    )
}

/// Estimate text width in pixels (rough approximation)
fn estimate_text_width(text: &str) -> u32 {
    (text.chars().count() as f32 * 6.5) as u32
}

/// Darken a hex color by the specified amount
fn darken_color(hex: &str, amount: f32) -> String {
    mdfx_colors::darken(hex, amount)
}

/// Get the ideal logo color (white or black) for contrast against background
fn get_logo_color_for_bg(bg_hex: &str) -> &'static str {
    let hex = bg_hex.trim_start_matches('#');
    if hex.len() < 6 {
        return "FFFFFF";
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;

    // Calculate relative luminance
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

    if luminance > 0.5 {
        "000000"
    } else {
        "FFFFFF"
    }
}

/// Render badge to file
pub fn render_to_file(badge: &TechBadge, path: impl AsRef<Path>) -> io::Result<()> {
    let svg = render(badge);
    std::fs::write(path, svg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::badge::BadgeBuilder;
    use crate::style::BadgeStyle;

    #[test]
    fn test_render_with_icon() {
        let badge = BadgeBuilder::new("rust").build();
        let svg = render(&badge);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        // Original behavior: lowercase label
        assert!(svg.contains("rust"));
    }

    #[test]
    fn test_render_text_only() {
        let badge = BadgeBuilder::new("unknown-tech").build();
        let svg = render(&badge);

        assert!(svg.contains("<svg"));
        // Text-only badges use uppercase
        assert!(svg.contains("UNKNOWN-TECH"));
    }

    #[test]
    fn test_custom_styling() {
        let badge = BadgeBuilder::new("typescript")
            .label("TypeScript v5.0")
            .style(BadgeStyle::Plastic)
            .bg_color("#3178C6")
            .text_color("#FFFFFF")
            .build();

        let svg = render(&badge);
        assert!(svg.contains("TypeScript v5.0"));
        assert!(svg.contains("3178C6"));
    }

    #[test]
    fn test_estimate_text_width() {
        assert_eq!(estimate_text_width("Rust"), 26); // 4 chars * 6.5
        assert_eq!(estimate_text_width("TypeScript"), 65); // 10 chars * 6.5
    }

    #[test]
    fn test_font_sizes() {
        assert_eq!(calculate_font_size(BadgeStyle::Flat), 10);
        assert_eq!(calculate_font_size(BadgeStyle::ForTheBadge), 11);
        assert_eq!(calculate_font_size(BadgeStyle::Social), 10); // Social has height 20
    }

    fn calculate_font_size(style: BadgeStyle) -> u32 {
        let metrics = SvgMetrics::from_style(style);
        if metrics.height as u32 > 24 {
            11
        } else {
            10
        }
    }

    #[test]
    fn test_render_with_border() {
        let badge = BadgeBuilder::new("python").border("#FF0000", 2).build();

        let svg = render(&badge);
        // Border is now inline on the first element
        assert!(svg.contains("stroke=\"#FF0000\""));
        assert!(svg.contains("stroke-width=\"2\""));
    }
}
