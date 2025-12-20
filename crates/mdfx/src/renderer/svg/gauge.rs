//! Gauge (semi-circular meter) SVG renderer

/// Render a gauge (semi-circular meter) using SVG arc paths
#[allow(clippy::too_many_arguments)]
pub fn render(
    percent: u8,
    size: u32,
    thickness: u32,
    track_color: &str,
    fill_color: &str,
    show_label: bool,
    label_color: Option<&str>,
    thumb_size: Option<u32>,
    thumb_color: Option<&str>,
    thumb_border: Option<&str>,
    thumb_border_width: u32,
) -> String {
    // Gauge is a half-circle (180 degrees) arc
    // Size is the width, height is approximately size/2 + space for label
    let center_x = size as f32 / 2.0;
    let radius = (size as f32 / 2.0) - (thickness as f32 / 2.0);

    // Calculate arc endpoints
    // Arc goes from left (180°) to right (0°), through top
    let start_x = center_x - radius;
    let end_x = center_x + radius;
    let arc_y = radius + (thickness as f32 / 2.0);

    // Calculate padding for thumb (thumb might extend beyond arc)
    let thumb_padding = thumb_size
        .map(|t| (t / 2).saturating_sub(thickness / 2))
        .unwrap_or(0);

    // SVG height: half circle height + space for label if shown + thumb padding
    let svg_height = if show_label {
        (size / 2) + thickness + 20 + thumb_padding
    } else {
        (size / 2) + thickness + thumb_padding
    };

    // Semi-circle circumference = π × radius
    let semi_circumference = std::f32::consts::PI * radius;

    // Fill length based on percentage
    let fill_length = semi_circumference * (percent as f32 / 100.0);
    let gap_length = semi_circumference - fill_length;

    // Build track arc path (full semi-circle from left to right)
    // M = move to start, A = arc (rx ry x-rotation large-arc-flag sweep-flag x y)
    let track_path = format!(
        "M {:.1} {:.1} A {:.1} {:.1} 0 0 1 {:.1} {:.1}",
        start_x, arc_y, radius, radius, end_x, arc_y
    );

    // Build label element if requested
    let label_elem = if show_label {
        let label_col = label_color.unwrap_or("FFFFFF");
        let font_size = (size / 5).clamp(12, 18);
        let text_y = arc_y + font_size as f32 + 4.0;
        format!(
            "\n  <text x=\"{}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\">{}%</text>",
            center_x, text_y, label_col, font_size, percent
        )
    } else {
        String::new()
    };

    // Build thumb element if requested
    let thumb_elem = if let Some(thumb_sz) = thumb_size {
        let t_color = thumb_color.unwrap_or(fill_color);
        // Calculate thumb position on the semi-circle arc
        // Angle: starts at left (180°), progresses to right (0°)
        let angle_deg = 180.0 - (percent as f32 * 180.0 / 100.0);
        let angle_rad = angle_deg * std::f32::consts::PI / 180.0;
        let thumb_x = center_x + radius * angle_rad.cos();
        let thumb_y = arc_y - radius * angle_rad.sin();
        let thumb_r = thumb_sz as f32 / 2.0;
        // Build thumb border attributes if specified
        let border_attr = if let Some(bc) = thumb_border {
            if thumb_border_width > 0 {
                format!(
                    " stroke=\"#{}\" stroke-width=\"{}\"",
                    bc, thumb_border_width
                )
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        format!(
            "\n  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#{}\"{border_attr}/>",
            thumb_x, thumb_y, thumb_r, t_color
        )
    } else {
        String::new()
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <path d=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"round\"/>\n\
  <path d=\"{}\" fill=\"none\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-dasharray=\"{:.2} {:.2}\"/>{}{}\n\
</svg>",
        size,
        svg_height,
        size,
        svg_height,
        track_path,
        track_color,
        thickness,
        track_path,
        fill_color,
        thickness,
        fill_length,
        gap_length,
        thumb_elem,
        label_elem
    )
}
