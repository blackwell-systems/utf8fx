//! Donut/ring chart SVG renderer

use super::utils::{arc_dash_lengths, build_stroke_attr, point_on_circle, thumb_padding};
use crate::primitive::ThumbConfig;

/// Render a donut/ring chart using stroke-dasharray trick
#[allow(clippy::too_many_arguments)]
pub fn render(
    percent: u8,
    size: u32,
    thickness: u32,
    track_color: &str,
    fill_color: &str,
    show_label: bool,
    label_color: Option<&str>,
    thumb: Option<&ThumbConfig>,
) -> String {
    // Use radius ~15.9 so circumference ≈ 100 (makes percentage math easy)
    // Scale radius based on size: r = (size/2 - thickness/2)
    let center = size as f32 / 2.0;
    let radius = center - (thickness as f32 / 2.0);
    let circumference = 2.0 * std::f32::consts::PI * radius;

    // Calculate dash lengths for percentage
    let (fill_length, gap_length) = arc_dash_lengths(circumference, percent);

    // Calculate viewbox padding for thumb (thumb might extend beyond circle)
    let padding = thumb_padding(thumb, thickness);
    let svg_size = size + padding * 2;
    let adjusted_center = center + padding as f32;

    // Build label element if requested (and size is large enough)
    let label_elem = if show_label && size >= 30 {
        let label_col = label_color.unwrap_or("FFFFFF");
        // Font size scales with donut size
        let font_size = (size / 4).clamp(10, 16);
        format!(
            "\n  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"central\" fill=\"#{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\">{}%</text>",
            adjusted_center, adjusted_center, label_col, font_size, percent
        )
    } else {
        String::new()
    };

    // Build thumb element if requested
    let thumb_elem = if let Some(thumb_cfg) = thumb {
        let t_color = thumb_cfg.color.as_deref().unwrap_or(fill_color);
        // Calculate thumb position on the circle
        // Angle: starts at top (-90°), progresses clockwise
        let angle_deg = -90.0 + (percent as f32 * 360.0 / 100.0);
        let (thumb_x, thumb_y) = point_on_circle(adjusted_center, adjusted_center, radius, angle_deg);
        let thumb_r = thumb_cfg.size as f32 / 2.0;
        // Build thumb border attributes if specified
        let border_attr = build_stroke_attr(thumb_cfg.border.as_deref(), thumb_cfg.border_width);
        format!(
            "\n  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#{}\"{border_attr}/>",
            thumb_x, thumb_y, thumb_r, t_color
        )
    } else {
        String::new()
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\"/>\n\
  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" stroke-dasharray=\"{:.2} {:.2}\" transform=\"rotate(-90 {:.1} {:.1})\"/>{}{}\n\
</svg>",
        svg_size,
        svg_size,
        svg_size,
        svg_size,
        adjusted_center,
        adjusted_center,
        radius,
        track_color,
        thickness,
        adjusted_center,
        adjusted_center,
        radius,
        fill_color,
        thickness,
        fill_length,
        gap_length,
        adjusted_center,
        adjusted_center,
        thumb_elem,
        label_elem
    )
}
