//! SVG rendering for technology badges - extracted from mdfx tech.rs
//! This implementation produces pixel-perfect identical output to the original mdfx renderer

use std::io;
use std::path::Path;

use crate::badge::TechBadge;
use crate::shapes::rounded_rect_path;
use crate::style::{BadgeStyle, SvgMetrics};

/// Render a badge to SVG string (pixel-perfect match to original mdfx)
pub fn render(badge: &TechBadge) -> String {
    // Get the label - keep lowercase if no explicit label set (matching original behavior)
    let label = badge.label.as_deref().unwrap_or(&badge.name);

    // Check if we have an icon (custom_icon takes priority)
    let icon_path: Option<&str> = badge
        .custom_icon
        .as_deref()
        .or_else(|| mdfx_icons::icon_path(&badge.name));

    // Get colors
    let bg_color = badge
        .effective_bg_color()
        .unwrap_or_else(|| "#555".to_string());
    let bg_color_clean = bg_color.trim_start_matches('#');

    // Handle raised mode - icon section taller than label section
    if let Some(raised_px) = badge.raised {
        if let Some(path) = icon_path {
            let logo_color = badge
                .logo_color
                .as_deref()
                .map(|c| c.trim_start_matches('#'))
                .unwrap_or_else(|| get_logo_color_for_bg(bg_color_clean));
            return render_raised_badge(badge, path, label, bg_color_clean, logo_color, raised_px);
        }
    }

    // Handle outline mode separately for proper rendering
    if badge.outline {
        let brand_color = bg_color_clean;

        return match (icon_path, !label.is_empty()) {
            (Some(path), true) => render_outline_two_segment(badge, path, label, brand_color),
            (Some(path), false) => render_outline_icon_only(badge, path, brand_color),
            (None, _) => render_outline_text_only(badge, label, brand_color),
        };
    }

    let bg_color = bg_color_clean;

    // Logo color: use explicit if provided, otherwise calculate from actual bg_color
    // This matches mdfx behavior where logo contrast is based on the actual background,
    // not the brand's default color
    let logo_color = badge
        .logo_color
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .unwrap_or_else(|| get_logo_color_for_bg(bg_color));

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

    // Hardcoded values matching original mdfx tech.rs
    let icon_width: u32 = 36;
    let icon_size: u32 = badge.logo_size.unwrap_or(14);

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
    // Only apply border to left segment if not border_full mode
    let border_attr = if badge.border_full {
        String::new()
    } else {
        get_border_attr(badge)
    };

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

    // Full border outline (drawn last, on top)
    let full_border_outline = if badge.border_full {
        if let Some(border) = &badge.border {
            let color = border.color.trim_start_matches('#');
            format!(
                "\n  <rect width=\"{}\" height=\"{}\" fill=\"none\" rx=\"{}\" stroke=\"#{}\" stroke-width=\"{}\"/>",
                total_width, height, rx, color, border.width
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\n\
  {}\n\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\">{}</text>{}\n\
</svg>",
        total_width, height, total_width, height,
        left_segment,
        right_segment,
        icon_x, icon_y, scale,
        logo_color, icon_path,
        text_x, text_y, text_color, font_family, font_size, label,
        full_border_outline
    )
}

/// Render raised badge: icon section taller than label section
///
/// The raised badge has the icon extending above and below the skinnier label area.
/// Uses a single background color for visual conformity.
///
/// ```text
/// ┌─────────┬────────────┐
/// │         │            │
/// │  ICON   │   label    │  <- label section is vertically centered
/// │         │            │
/// └─────────┴────────────┘
/// ```
fn render_raised_badge(
    badge: &TechBadge,
    icon_path: &str,
    label: &str,
    bg_color: &str,
    logo_color: &str,
    raised_px: u32,
) -> String {
    let metrics = SvgMetrics::from_style(badge.style);

    // Icon section is taller - full height plus raised pixels above and below
    let icon_height = metrics.height as u32 + (raised_px * 2);
    // Label section is the standard height
    let label_height = metrics.height as u32;
    // Total badge height is the icon section height
    let total_height = icon_height;

    // Widths
    let icon_width: u32 = 36;
    let label_width = estimate_text_width(label) + 16;
    let total_width = icon_width + label_width;

    // Icon sizing and positioning (default 16 for raised badges)
    let icon_size: u32 = badge.logo_size.unwrap_or(16);
    let icon_x = (icon_width as f32 - icon_size as f32) / 2.0;
    let icon_y = (icon_height as f32 - icon_size as f32) / 2.0;
    let scale = icon_size as f32 / 24.0;

    // Label positioning - vertically centered
    let label_y_offset = raised_px as f32; // Offset from top
    let font_size = if metrics.height > 24.0 { 11 } else { 10 };
    let text_x = icon_width + label_width / 2;
    let text_y = label_y_offset + label_height as f32 / 2.0 + font_size as f32 / 3.0;

    // Text color
    let text_color = badge
        .text_color
        .as_deref()
        .map(|c| c.trim_start_matches('#'))
        .unwrap_or_else(|| get_logo_color_for_bg(bg_color));

    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");

    // Corner radius
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    // Border handling
    let border_attr = get_border_attr(badge);

    // Generate SVG
    // Icon section: full height rectangle on left (extends 1px into label to prevent seam)
    // Label section: shorter rectangle on right, vertically centered
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>\n\
  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\"/>\n\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
        total_width, total_height, total_width, total_height,
        // Icon section background (full height, +1px overlap to prevent seam)
        icon_width + 1, total_height, bg_color, rx, border_attr,
        // Label section background (shorter, centered)
        icon_width, label_y_offset, label_width, label_height, bg_color,
        // Icon
        icon_x, icon_y, scale,
        logo_color, icon_path,
        // Text
        text_x, text_y as u32, text_color, font_family, font_size, label
    )
}

/// Render icon-only badge (matching original mdfx)
fn render_icon_only(
    badge: &TechBadge,
    icon_path: &str,
    bg_color: &str,
    logo_color: &str,
) -> String {
    let metrics = SvgMetrics::from_style(badge.style);
    let height = metrics.height as u32;
    let width: u32 = 40;
    let icon_size: u32 = badge.logo_size.unwrap_or(16);
    let icon_x = (width as f32 - icon_size as f32) / 2.0;
    let icon_y = (height as f32 - icon_size as f32) / 2.0;
    let scale = icon_size as f32 / 24.0;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    let border_attr = get_border_attr(badge);

    // Generate background using render_bg_element logic (single rect for uniform rx)
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
    } else if rx > 0 {
        // Single rect with uniform radius (matching original)
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>\n",
            width, height, bg_color, rx, border_attr
        )
    } else {
        // Square corners
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"#{}\"{}/>\n",
            width, height, bg_color, border_attr
        )
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\
  <g transform=\"translate({}, {}) scale({})\">\n\
    <path fill=\"#{}\" d=\"{}\"/>\n\
  </g>\n\
</svg>",
        width, height, width, height, bg, icon_x, icon_y, scale, logo_color, icon_path
    )
}

/// Render text-only badge (matching original mdfx)
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

    let border_attr = get_border_attr(badge);

    // Generate background using render_bg_element logic
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
    } else if rx > 0 {
        // Single rect with uniform radius (matching original)
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>\n",
            width, height, bg_color, rx, border_attr
        )
    } else {
        // Square corners
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"#{}\"{}/>\n",
            width, height, bg_color, border_attr
        )
    };

    // Original uses hardcoded white fill
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  {}\
  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"white\" font-family=\"Verdana,Arial,sans-serif\" font-size=\"{}\" font-weight=\"600\">{}</text>\n\
</svg>",
        width,
        height,
        width,
        height,
        bg,
        width / 2,
        text_y,
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
    // Outline style uses flat-square metrics (rx=0) to match original mdfx
    let metrics = SvgMetrics::from_style(BadgeStyle::FlatSquare);
    let height = metrics.height as u32;
    let rx = badge
        .corners
        .as_ref()
        .map(|c| c.top_left)
        .unwrap_or(metrics.radius as u32);

    let icon_width: u32 = 36;
    let label_width = estimate_text_width(label) + 16;
    let total_width = icon_width + label_width;
    let icon_size: u32 = badge.logo_size.unwrap_or(14);
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
    // Outline style uses flat-square metrics (rx=0) to match original mdfx
    let metrics = SvgMetrics::from_style(BadgeStyle::FlatSquare);
    let height = metrics.height as u32;
    let width: u32 = 40;
    let icon_size: u32 = badge.logo_size.unwrap_or(16);
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
    // Outline style uses flat-square metrics (rx=0) to match original mdfx
    let metrics = SvgMetrics::from_style(BadgeStyle::FlatSquare);
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

/// Estimate text width in pixels (matching original mdfx)
fn estimate_text_width(text: &str) -> u32 {
    // Approximate 7px per character for Verdana 11px
    (text.len() as u32 * 7).max(20)
}

/// Darken a hex color by the specified amount (returns without # prefix)
fn darken_color(hex: &str, amount: f32) -> String {
    // mdfx_colors::darken returns with # prefix, but we need without for consistency
    mdfx_colors::darken(hex, amount)
        .trim_start_matches('#')
        .to_string()
}

/// Get the ideal logo color (white or black) for contrast against background
/// Uses mdfx_colors for luminance calculation, strips # prefix for consistency
fn get_logo_color_for_bg(bg_hex: &str) -> &'static str {
    // mdfx_colors::contrast_color returns "#FFFFFF" or "#000000"
    // We need "FFFFFF" or "000000" to match original mdfx format
    mdfx_colors::contrast_color(bg_hex).trim_start_matches('#')
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
        assert_eq!(estimate_text_width("Rust"), 28); // 4 chars * 7
        assert_eq!(estimate_text_width("TypeScript"), 70); // 10 chars * 7
        assert_eq!(estimate_text_width("ab"), 20); // min 20
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
