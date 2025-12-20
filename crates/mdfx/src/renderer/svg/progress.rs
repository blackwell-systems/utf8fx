//! Progress bar and slider SVG renderer

/// Render a progress bar with track and fill
#[allow(clippy::too_many_arguments)]
pub fn render(
    percent: u8,
    width: u32,
    height: u32,
    track_color: &str,
    fill_color: &str,
    fill_height: u32,
    rx: u32,
    show_label: bool,
    label_color: Option<&str>,
    border_color: Option<&str>,
    border_width: u32,
    thumb_size: Option<u32>,
    thumb_width: Option<u32>,
    thumb_color: Option<&str>,
    thumb_shape: &str,
    thumb_border: Option<&str>,
    thumb_border_width: u32,
) -> String {
    // Slider mode: track with thumb at position
    if let Some(thumb_sz) = thumb_size {
        return render_slider(
            percent,
            width,
            height,
            track_color,
            fill_color,
            thumb_sz,
            thumb_width,
            thumb_color,
            thumb_shape,
            border_color,
            border_width,
            thumb_border,
            thumb_border_width,
        );
    }

    // Standard progress bar mode
    // Calculate fill width based on percentage
    let fill_width = (width as f32 * percent as f32 / 100.0) as u32;

    // Center the fill vertically if it's shorter than track
    let fill_y = if fill_height < height {
        (height - fill_height) / 2
    } else {
        0
    };

    // Use a slightly smaller rx for the fill if height differs
    let fill_rx = if fill_height < height {
        rx.min(fill_height / 2)
    } else {
        rx
    };

    // Build label element if requested
    // Require minimum dimensions for readable labels (50px wide, 14px tall)
    let label_elem = if show_label && width >= 50 && height >= 14 {
        let label_col = label_color.unwrap_or("FFFFFF");
        let font_size = if height >= 20 { 12 } else { 10 };
        let text_y = height / 2 + font_size / 3;
        let text_x = width / 2;
        // Add letter-spacing for better readability
        format!(
            "\n  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\" letter-spacing=\"1\">{}%</text>",
            text_x, text_y, label_col, font_size, percent
        )
    } else {
        String::new()
    };

    // Build border attribute if specified
    let border_attr = if let Some(bc) = border_color {
        if border_width > 0 {
            format!(" stroke=\"#{}\" stroke-width=\"{}\"", bc, border_width)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <rect width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>\n\
  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>{}\n\
</svg>",
        width,
        height,
        width,
        height,
        width,
        height,
        track_color,
        rx,
        border_attr,
        fill_y,
        fill_width,
        fill_height,
        fill_color,
        fill_rx,
        label_elem
    )
}

/// Render a slider with track and thumb at position
#[allow(clippy::too_many_arguments)]
fn render_slider(
    percent: u8,
    width: u32,
    height: u32,
    track_color: &str,
    fill_color: &str,
    thumb_height: u32,
    thumb_width: Option<u32>,
    thumb_color: Option<&str>,
    thumb_shape: &str,
    border_color: Option<&str>,
    border_width: u32,
    thumb_border: Option<&str>,
    thumb_border_width: u32,
) -> String {
    // Thumb width defaults to thumb_height (square/circle) if not specified
    let thumb_w = thumb_width.unwrap_or(thumb_height);

    // SVG height must accommodate the thumb
    let svg_height = height.max(thumb_height);
    let center_y = svg_height / 2;

    // Track uses the specified height (can be thin or thick)
    let track_height = height;
    let track_y = center_y - track_height / 2;
    let track_rx = (track_height / 2).min(3);

    // Build border attribute for track if specified
    let border_attr = if let Some(bc) = border_color {
        if border_width > 0 {
            format!(" stroke=\"#{}\" stroke-width=\"{}\"", bc, border_width)
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Thumb position based on percentage
    // Ensure thumb stays within bounds (half thumb width from edges)
    let thumb_half_w = thumb_w / 2;
    let usable_width = width.saturating_sub(thumb_w);
    let thumb_x = thumb_half_w + (usable_width as f32 * percent as f32 / 100.0) as u32;

    // Fill element: colored portion from left edge to thumb position
    let fill_width = thumb_x;
    let fill_elem = if fill_width > 0 {
        format!(
            "\n  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>",
            track_y, fill_width, track_height, fill_color, track_rx
        )
    } else {
        String::new()
    };

    // Thumb color defaults to fill color
    let t_color = thumb_color.unwrap_or(fill_color);

    // Build thumb border attributes if specified
    let thumb_border_attr = if let Some(bc) = thumb_border {
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

    // Render thumb based on shape
    let thumb_elem = match thumb_shape {
        "square" => {
            let half_h = thumb_height / 2;
            let half_w = thumb_w / 2;
            // Use smaller dimension for rx to create pill shape when width != height
            let rx = half_h.min(half_w).min(4);
            format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{thumb_border_attr}/>",
                thumb_x - half_w,
                center_y - half_h,
                thumb_w,
                thumb_height,
                t_color,
                rx
            )
        }
        "diamond" => {
            let half_h = thumb_height / 2;
            let half_w = thumb_w / 2;
            format!(
                "<polygon points=\"{},{} {},{} {},{} {},{}\" fill=\"#{}\"{thumb_border_attr}/>",
                thumb_x,
                center_y - half_h, // top
                thumb_x + half_w,
                center_y, // right
                thumb_x,
                center_y + half_h, // bottom
                thumb_x - half_w,
                center_y, // left
                t_color
            )
        }
        _ => {
            // Default: ellipse (or circle if width == height)
            let rx = thumb_w as f32 / 2.0;
            let ry = thumb_height as f32 / 2.0;
            format!(
                "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"#{}\"{thumb_border_attr}/>",
                thumb_x, center_y, rx, ry, t_color
            )
        }
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>{}\n\
  {}\n\
</svg>",
        width,
        svg_height,
        width,
        svg_height,
        track_y,
        width,
        track_height,
        track_color,
        track_rx,
        border_attr,
        fill_elem,
        thumb_elem
    )
}
