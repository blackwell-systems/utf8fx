//! SVG rendering for technology badges - extracted from mdfx tech.rs

use std::io;
use std::path::Path;

use crate::badge::TechBadge;
use crate::shapes::{chevron_path_with_overlap, rounded_rect_path};
use crate::style::{BadgeStyle, SvgMetrics};

/// Render a badge to SVG string
pub fn render(badge: &TechBadge) -> String {
    let label = badge.display_label();
    let icon_size = badge.logo_size.to_pixels();
    let font_size = calculate_font_size(badge.style);

    // Check if we have an icon (custom_icon takes priority)
    let icon_path: Option<&str> = badge
        .custom_icon
        .as_deref()
        .or_else(|| mdfx_icons::icon_path(&badge.name));

    let metrics = SvgMetrics::calculate(
        &label,
        icon_size as f32,
        font_size as f32,
        badge.style,
        icon_path.is_some(),
    );

    // Handle outline mode separately for proper rendering
    if badge.outline {
        return match icon_path {
            Some(path) => render_outline_with_icon(badge, &metrics, &label, path),
            None => render_outline_text_only(badge, &metrics, &label),
        };
    }

    match icon_path {
        // Icon badge (with or without label)
        Some(path) => render_with_icon(badge, &metrics, &label, path),
        // Text-only badge
        None => render_text_only(badge, &metrics, &label),
    }
}

/// Calculate appropriate font size based on badge style
fn calculate_font_size(style: BadgeStyle) -> u32 {
    match style {
        BadgeStyle::ForTheBadge => 11,
        BadgeStyle::Social => 11,
        _ => 10,
    }
}

/// Render badge with icon and optional text
fn render_with_icon(
    badge: &TechBadge,
    metrics: &SvgMetrics,
    label: &str,
    icon_path: &str,
) -> String {
    let height = metrics.height;
    let icon_size = badge.logo_size.to_pixels() as f32;

    // Calculate layout
    let icon_width = (icon_size * 2.5).ceil() as u32 + 1;
    let label_width = estimate_text_width(label) + 16;
    let has_label = !label.is_empty();
    let total_width = icon_width + if has_label { label_width } else { 0 };

    let font_size = calculate_font_size(badge.style);
    let text_x = icon_width + label_width / 2;

    // Colors - support separate left/right backgrounds
    let bg_color = badge
        .effective_bg_color()
        .unwrap_or_else(|| "#555".to_string());
    let left_bg = badge.bg_left.as_deref().unwrap_or(&bg_color);
    let default_right_bg = darken_color(&bg_color, 0.15);
    let right_bg = badge
        .bg_right
        .as_deref()
        .map(|s| s.to_string())
        .unwrap_or(default_right_bg);

    let logo_color = badge
        .logo_color
        .as_deref()
        .or_else(|| mdfx_icons::brand_contrast_color(&badge.name))
        .unwrap_or("#FFFFFF");
    let text_color = badge
        .text_color
        .as_deref()
        .unwrap_or_else(|| mdfx_colors::contrast_color(&right_bg));

    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");

    // Handle raised effect - icon section extends above/below label
    let (svg_height, icon_y, text_y, _label_y_offset) = if let Some(raised) = badge.raised {
        let raised = raised as f32;
        let total_height = height + 2.0 * raised;
        let icon_y = (total_height - icon_size) / 2.0;
        let text_y = raised + height / 2.0 + font_size as f32 / 3.0;
        (total_height, icon_y, text_y as u32, raised)
    } else {
        let icon_y = (height - icon_size) / 2.0;
        let text_y = height as u32 / 2 + font_size / 3;
        (height, icon_y, text_y, 0.0)
    };

    let icon_x = (icon_width as f32 - icon_size) / 2.0;

    // Start building SVG
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        total_width, svg_height as u32, total_width, svg_height as u32
    );

    // Add gradients for plastic style
    if badge.style.has_gradients() {
        svg.push_str(&create_gradient_defs());
    }

    // Background shapes with raised effect support
    if let Some(raised) = badge.raised {
        let raised = raised as f32;
        // Icon section: full height
        let icon_corners = badge
            .corners
            .as_ref()
            .map(|c| [c.top_left, 0, 0, c.bottom_left])
            .unwrap_or([metrics.radius as u32, 0, 0, metrics.radius as u32]);
        svg.push_str(&format!(
            r#"<path d="{}" fill="{}"/>"#,
            rounded_rect_path(0.0, 0.0, icon_width as f32, svg_height, icon_corners),
            left_bg
        ));
        // Label section: offset by raised amount, original height
        if has_label {
            let label_corners = badge
                .corners
                .as_ref()
                .map(|c| [0, c.top_right, c.bottom_right, 0])
                .unwrap_or([0, metrics.radius as u32, metrics.radius as u32, 0]);
            svg.push_str(&format!(
                r#"<path d="{}" fill="{}"/>"#,
                rounded_rect_path(
                    icon_width as f32,
                    raised,
                    label_width as f32,
                    height,
                    label_corners
                ),
                right_bg
            ));
        }
    } else if let Some(chevron) = &badge.chevron {
        // Chevron style background
        let (left_path, _, _) = chevron_path_with_overlap(
            0.0,
            0.0,
            icon_width as f32,
            height,
            chevron.direction,
            chevron.depth,
        );
        svg.push_str(&format!(r#"<path d="{}" fill="{}"/>"#, left_path, left_bg));

        if has_label {
            let (right_path, _, _) = chevron_path_with_overlap(
                icon_width as f32,
                0.0,
                label_width as f32,
                height,
                chevron.direction,
                chevron.depth,
            );
            svg.push_str(&format!(
                r#"<path d="{}" fill="{}"/>"#,
                right_path, right_bg
            ));
        }
    } else {
        // Regular rounded rectangles with per-corner split for two-segment badges
        let (left_corners, right_corners) = if let Some(c) = &badge.corners {
            // Split corners: left segment gets left corners, right gets right corners
            (
                [c.top_left, 0, 0, c.bottom_left],
                [0, c.top_right, c.bottom_right, 0],
            )
        } else {
            let r = metrics.radius as u32;
            ([r, 0, 0, r], [0, r, r, 0])
        };

        // Left segment (icon)
        svg.push_str(&format!(
            r#"<path d="{}" fill="{}"/>"#,
            rounded_rect_path(0.0, 0.0, icon_width as f32, height, left_corners),
            left_bg
        ));

        // Right segment (text) if label exists
        if has_label {
            svg.push_str(&format!(
                r#"<path d="{}" fill="{}"/>"#,
                rounded_rect_path(
                    icon_width as f32,
                    0.0,
                    label_width as f32,
                    height,
                    right_corners
                ),
                right_bg
            ));
        }
    }

    // Icon
    let scale = icon_size / 24.0;
    svg.push_str(&format!(
        r#"<g transform="translate({}, {}) scale({})">"#,
        icon_x, icon_y, scale
    ));
    svg.push_str(&format!(
        r#"<path d="{}" fill="{}"/>"#,
        icon_path, logo_color
    ));
    svg.push_str("</g>");

    // Text label (if present)
    if has_label {
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{}" font-weight="600" text-anchor="middle">{}</text>"#,
            text_x,
            text_y,
            font_family,
            font_size,
            text_color,
            escape_xml(label)
        ));
    }

    // Border if specified
    if let Some(border) = &badge.border {
        let border_path = if let Some(chevron) = &badge.chevron {
            // Chevron border
            let (path, _, _) = chevron_path_with_overlap(
                border.width as f32 / 2.0,
                border.width as f32 / 2.0,
                total_width as f32 - border.width as f32,
                svg_height - border.width as f32,
                chevron.direction,
                chevron.depth,
            );
            path
        } else {
            // Regular border
            let corners = badge
                .corners
                .as_ref()
                .map(|c| [c.top_left, c.top_right, c.bottom_right, c.bottom_left])
                .unwrap_or([metrics.radius as u32; 4]);
            rounded_rect_path(
                border.width as f32 / 2.0,
                border.width as f32 / 2.0,
                total_width as f32 - border.width as f32,
                svg_height - border.width as f32,
                corners,
            )
        };

        svg.push_str(&format!(
            r#"<path d="{}" fill="none" stroke="{}" stroke-width="{}"/>"#,
            border_path, border.color, border.width
        ));
    }

    // Apply plastic style effects
    if badge.style.has_gradients() {
        svg.push_str(&format!(
            r#"<path d="{}" fill="url(#bg-gradient)" opacity="0.1"/>"#,
            rounded_rect_path(
                0.0,
                0.0,
                total_width as f32,
                svg_height,
                [metrics.radius as u32; 4]
            )
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Render text-only badge (no icon available)
fn render_text_only(badge: &TechBadge, metrics: &SvgMetrics, label: &str) -> String {
    let height = metrics.height;
    let label_width = estimate_text_width(label) + 24; // Extra padding for text-only
    let font_size = calculate_font_size(badge.style);

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        label_width, height as u32, label_width, height as u32
    );

    // Background
    let bg_color = badge
        .effective_bg_color()
        .unwrap_or_else(|| "#555".to_string());

    if let Some(chevron) = &badge.chevron {
        let (path, _, _) = chevron_path_with_overlap(
            0.0,
            0.0,
            label_width as f32,
            height,
            chevron.direction,
            chevron.depth,
        );
        svg.push_str(&format!(r#"<path d="{}" fill="{}"/>"#, path, bg_color));
    } else {
        let corners = badge
            .corners
            .as_ref()
            .map(|c| [c.top_left, c.top_right, c.bottom_right, c.bottom_left])
            .unwrap_or([metrics.radius as u32; 4]);
        svg.push_str(&format!(
            r#"<path d="{}" fill="{}"/>"#,
            rounded_rect_path(0.0, 0.0, label_width as f32, height, corners),
            bg_color
        ));
    }

    // Text
    let text_color = badge
        .text_color
        .as_deref()
        .unwrap_or_else(|| mdfx_colors::contrast_color(&bg_color));
    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");

    svg.push_str(&format!(
        r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{}" text-anchor="middle" dominant-baseline="middle">{}</text>"#,
        label_width as f32 / 2.0,
        height / 2.0,
        font_family,
        font_size,
        text_color,
        escape_xml(label)
    ));

    svg.push_str("</svg>");
    svg
}

/// Render outline/ghost style badge with icon
fn render_outline_with_icon(
    badge: &TechBadge,
    metrics: &SvgMetrics,
    label: &str,
    icon_path: &str,
) -> String {
    let height = metrics.height;
    let icon_size = badge.logo_size.to_pixels() as f32;

    // Calculate layout
    let icon_width = (icon_size * 2.5).ceil() as u32 + 1;
    let label_width = estimate_text_width(label) + 16;
    let has_label = !label.is_empty();
    let total_width = icon_width + if has_label { label_width } else { 0 };

    let icon_x = (icon_width as f32 - icon_size) / 2.0;
    let icon_y = (height - icon_size) / 2.0;
    let font_size = calculate_font_size(badge.style);
    let text_x = icon_width + label_width / 2;
    let text_y = height as u32 / 2 + font_size / 3;

    // Colors for outline mode
    let stroke_color = badge
        .effective_bg_color()
        .unwrap_or_else(|| "#555".to_string());
    let stroke_width = badge.border.as_ref().map(|b| b.width).unwrap_or(2);
    let logo_color = badge.logo_color.as_deref().unwrap_or(stroke_color.as_str());
    let text_color = badge.text_color.as_deref().unwrap_or(stroke_color.as_str());

    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");
    let corners = badge
        .corners
        .as_ref()
        .map(|c| [c.top_left, c.top_right, c.bottom_right, c.bottom_left])
        .unwrap_or([metrics.radius as u32; 4]);

    // Start building SVG
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        total_width, height as u32, total_width, height as u32
    );

    // Outline border (transparent fill)
    svg.push_str(&format!(
        r#"<path d="{}" fill="none" stroke="{}" stroke-width="{}"/>"#,
        rounded_rect_path(
            stroke_width as f32 / 2.0,
            stroke_width as f32 / 2.0,
            total_width as f32 - stroke_width as f32,
            height - stroke_width as f32,
            corners
        ),
        stroke_color,
        stroke_width
    ));

    // Separator line between icon and label sections
    if has_label {
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}"/>"#,
            icon_width,
            stroke_width / 2,
            icon_width,
            height as u32 - stroke_width / 2,
            stroke_color,
            stroke_width
        ));
    }

    // Icon
    let scale = icon_size / 24.0;
    svg.push_str(&format!(
        r#"<g transform="translate({}, {}) scale({})">"#,
        icon_x, icon_y, scale
    ));
    svg.push_str(&format!(
        r#"<path d="{}" fill="{}"/>"#,
        icon_path, logo_color
    ));
    svg.push_str("</g>");

    // Text label (if present)
    if has_label {
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{}" font-weight="600" text-anchor="middle">{}</text>"#,
            text_x,
            text_y,
            font_family,
            font_size,
            text_color,
            escape_xml(label)
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Render outline/ghost style text-only badge
fn render_outline_text_only(badge: &TechBadge, metrics: &SvgMetrics, label: &str) -> String {
    let height = metrics.height;
    let label_width = estimate_text_width(label) + 24;
    let font_size = calculate_font_size(badge.style);

    // Colors for outline mode
    let stroke_color = badge
        .effective_bg_color()
        .unwrap_or_else(|| "#555".to_string());
    let stroke_width = badge.border.as_ref().map(|b| b.width).unwrap_or(2);
    let text_color = badge.text_color.as_deref().unwrap_or(stroke_color.as_str());

    let font_family = badge.font.as_deref().unwrap_or("Verdana,Arial,sans-serif");
    let corners = badge
        .corners
        .as_ref()
        .map(|c| [c.top_left, c.top_right, c.bottom_right, c.bottom_left])
        .unwrap_or([metrics.radius as u32; 4]);

    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}">"#,
        label_width, height as u32, label_width, height as u32
    );

    // Outline border (transparent fill)
    svg.push_str(&format!(
        r#"<path d="{}" fill="none" stroke="{}" stroke-width="{}"/>"#,
        rounded_rect_path(
            stroke_width as f32 / 2.0,
            stroke_width as f32 / 2.0,
            label_width as f32 - stroke_width as f32,
            height - stroke_width as f32,
            corners
        ),
        stroke_color,
        stroke_width
    ));

    // Text
    svg.push_str(&format!(
        r#"<text x="{}" y="{}" font-family="{}" font-size="{}" fill="{}" font-weight="600" text-anchor="middle">{}</text>"#,
        label_width as f32 / 2.0,
        height / 2.0 + font_size as f32 / 3.0,
        font_family,
        font_size,
        text_color,
        escape_xml(label)
    ));

    svg.push_str("</svg>");
    svg
}

/// Estimate text width in pixels (rough approximation)
fn estimate_text_width(text: &str) -> u32 {
    (text.chars().count() as f32 * 6.5) as u32
}

/// Darken a hex color by the specified amount
fn darken_color(hex: &str, amount: f32) -> String {
    mdfx_colors::darken(hex, amount)
}

/// Create gradient definitions for plastic style
fn create_gradient_defs() -> String {
    r#"<defs>
        <linearGradient id="bg-gradient" x2="0%" y2="100%">
            <stop offset="0%" style="stop-color:rgba(255,255,255,0.15);stop-opacity:1"/>
            <stop offset="100%" style="stop-color:rgba(0,0,0,0.15);stop-opacity:1"/>
        </linearGradient>
    </defs>"#
        .to_string()
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
        assert!(svg.contains("Rust"));
    }

    #[test]
    fn test_render_text_only() {
        let badge = BadgeBuilder::new("unknown-tech").build();
        let svg = render(&badge);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("Unknown-tech"));
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
        assert!(svg.contains("#3178C6"));
    }

    #[test]
    fn test_estimate_text_width() {
        assert_eq!(estimate_text_width("Rust"), 26); // 4 chars * 6.5
        assert_eq!(estimate_text_width("TypeScript"), 65); // 10 chars * 6.5
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("C++ & Rust"), "C++ &amp; Rust");
        assert_eq!(escape_xml("<script>"), "&lt;script&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_font_sizes() {
        assert_eq!(calculate_font_size(BadgeStyle::Flat), 10);
        assert_eq!(calculate_font_size(BadgeStyle::ForTheBadge), 11);
        assert_eq!(calculate_font_size(BadgeStyle::Social), 11);
    }

    #[test]
    fn test_render_with_chevron() {
        let badge = BadgeBuilder::new("rust")
            .chevron(crate::style::Chevron::right(8.0))
            .build();

        let svg = render(&badge);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Rust"));
    }

    #[test]
    fn test_render_with_border() {
        let badge = BadgeBuilder::new("python").border("#FF0000", 2).build();

        let svg = render(&badge);
        assert!(svg.contains("stroke=\"#FF0000\""));
        assert!(svg.contains("stroke-width=\"2\""));
    }
}
