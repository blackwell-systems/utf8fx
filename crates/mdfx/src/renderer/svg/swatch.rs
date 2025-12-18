//! Swatch SVG renderer - colored rectangles with optional enhancements

/// SVG style metrics for different badge styles
pub struct SvgMetrics {
    pub height: u32,
    pub rx: u32,
    pub plastic: bool,
}

impl SvgMetrics {
    pub fn from_style(style: &str) -> Self {
        match style {
            "flat-square" => SvgMetrics {
                height: 20,
                rx: 0,
                plastic: false,
            },
            "flat" => SvgMetrics {
                height: 20,
                rx: 3,
                plastic: false,
            },
            "for-the-badge" => SvgMetrics {
                height: 28,
                rx: 3,
                plastic: false,
            },
            "plastic" => SvgMetrics {
                height: 20,
                rx: 3,
                plastic: true,
            },
            "social" => SvgMetrics {
                height: 20,
                rx: 10,
                plastic: false,
            },
            _ => SvgMetrics {
                height: 20,
                rx: 3,
                plastic: false,
            },
        }
    }
}

/// Options for swatch rendering
pub struct SwatchOptions<'a> {
    pub color: &'a str,
    pub style: &'a str,
    pub opacity: Option<f32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub border_color: Option<&'a str>,
    pub border_width: Option<u32>,
    pub label: Option<&'a str>,
    pub label_color: Option<&'a str>,
    /// Simple Icons logo name (e.g., "rust", "python")
    pub icon: Option<&'a str>,
    /// Icon color (hex code without #)
    pub icon_color: Option<&'a str>,
    pub rx: Option<u32>,
    pub ry: Option<u32>,
    pub shadow: Option<&'a str>,
    pub gradient: Option<&'a str>,
    pub stroke_dash: Option<&'a str>,
    /// Per-side borders (format: "color/width" or just "color")
    pub border_top: Option<&'a str>,
    pub border_right: Option<&'a str>,
    pub border_bottom: Option<&'a str>,
    pub border_left: Option<&'a str>,
}

/// Parse shadow config: "color/blur/offset_x/offset_y" (e.g., "000000/4/2/2")
fn parse_shadow(shadow: &str) -> Option<(String, u32, i32, i32)> {
    let parts: Vec<&str> = shadow.split('/').collect();
    if parts.len() >= 4 {
        let color = parts[0].to_string();
        let blur = parts[1].parse().ok()?;
        let offset_x = parts[2].parse().ok()?;
        let offset_y = parts[3].parse().ok()?;
        Some((color, blur, offset_x, offset_y))
    } else {
        None
    }
}

/// Parse gradient config: "direction/color1/color2" (e.g., "horizontal/FF0000/0000FF")
fn parse_gradient(gradient: &str) -> Option<(String, String, String)> {
    let parts: Vec<&str> = gradient.split('/').collect();
    if parts.len() >= 3 {
        Some((
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        ))
    } else {
        None
    }
}

/// Parse a per-side border spec: "color/width" or just "color" (defaults to width 2)
fn parse_border_spec(spec: &str) -> (String, u32) {
    let parts: Vec<&str> = spec.split('/').collect();
    match parts.as_slice() {
        [color, width] => (color.to_string(), width.parse::<u32>().unwrap_or(2)),
        [color] => (color.to_string(), 2),
        _ => ("000000".to_string(), 2),
    }
}

/// Render a swatch (single colored rectangle with optional enhancements)
pub fn render(opts: SwatchOptions) -> String {
    let metrics = SvgMetrics::from_style(opts.style);

    // Use custom dimensions or defaults
    let width = opts.width.unwrap_or(20);
    let height = opts.height.unwrap_or(metrics.height);

    // Use custom rx/ry or fall back to style's rx
    let rx = opts.rx.unwrap_or(metrics.rx);
    let ry = opts.ry.unwrap_or(rx);

    // Build opacity attribute
    let opacity_attr = match opts.opacity {
        Some(o) if o < 1.0 => format!(" fill-opacity=\"{}\"", o),
        _ => String::new(),
    };

    // Build stroke dash attribute (format: "dash/gap", e.g., "4/2")
    let stroke_dash_attr = match opts.stroke_dash {
        Some(dash) => format!(" stroke-dasharray=\"{}\"", dash.replace('/', ",")),
        None => String::new(),
    };

    // Build border attributes
    let (border_attrs, border_offset) = match (opts.border_color, opts.border_width) {
        (Some(bc), Some(bw)) if bw > 0 => (
            format!(
                " stroke=\"#{}\" stroke-width=\"{}\"{}",
                bc, bw, stroke_dash_attr
            ),
            bw,
        ),
        (Some(bc), None) => (
            format!(" stroke=\"#{}\" stroke-width=\"1\"{}", bc, stroke_dash_attr),
            1,
        ),
        _ => (String::new(), 0),
    };

    // Parse shadow if provided
    let shadow_config = opts.shadow.and_then(parse_shadow);

    // Parse gradient if provided
    let gradient_config = opts.gradient.and_then(parse_gradient);

    // Calculate extra space needed for shadow
    let shadow_padding = if shadow_config.is_some() { 20 } else { 0 };

    // Adjust viewBox for border and shadow
    let vb_width = width + border_offset * 2 + shadow_padding;
    let vb_height = height + border_offset * 2 + shadow_padding;

    // Build label element
    let label_color = opts.label_color.unwrap_or("white");
    let label_elem = if let Some(text) = opts.label {
        let font_size = if height > 24 { 14 } else { 10 };
        let y_pos = height / 2 + font_size / 3 + border_offset + shadow_padding / 2;
        let x_pos = width / 2 + border_offset + shadow_padding / 2;
        format!(
            "\n  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\">{}</text>",
            x_pos, y_pos, label_color, font_size, text
        )
    } else {
        String::new()
    };

    // Build defs section (for gradients, shadows, plastic effect)
    let mut defs_content = String::new();

    // Add shadow filter if provided
    let filter_attr = if let Some((color, blur, offset_x, offset_y)) = &shadow_config {
        defs_content.push_str(&format!(
            "    <filter id=\"shadow\" x=\"-50%\" y=\"-50%\" width=\"200%\" height=\"200%\">\n\
  <feDropShadow dx=\"{}\" dy=\"{}\" stdDeviation=\"{}\" flood-color=\"#{}\" flood-opacity=\"0.8\"/>\n\
    </filter>\n",
            offset_x, offset_y, blur, color
        ));
        " filter=\"url(#shadow)\""
    } else {
        ""
    };

    // Add gradient if provided
    let fill_attr = if let Some((direction, color1, color2)) = &gradient_config {
        let (x1, y1, x2, y2) = match direction.as_str() {
            "vertical" => ("0%", "0%", "0%", "100%"),
            "diagonal" => ("0%", "0%", "100%", "100%"),
            _ => ("0%", "0%", "100%", "0%"), // horizontal default
        };
        defs_content.push_str(&format!(
            "    <linearGradient id=\"grad\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\">\n\
  <stop offset=\"0%\" style=\"stop-color:#{};stop-opacity:1\" />\n\
  <stop offset=\"100%\" style=\"stop-color:#{};stop-opacity:1\" />\n\
    </linearGradient>\n",
            x1, y1, x2, y2, color1, color2
        ));
        "url(#grad)".to_string()
    } else {
        format!("#{}", opts.color)
    };

    // Add plastic shine gradient if needed
    if metrics.plastic {
        defs_content.push_str(
            "    <linearGradient id=\"shine\" x1=\"0%\" y1=\"0%\" x2=\"0%\" y2=\"100%\">\n\
  <stop offset=\"0%\" style=\"stop-color:#ffffff;stop-opacity:0.2\" />\n\
  <stop offset=\"100%\" style=\"stop-color:#000000;stop-opacity:0.1\" />\n\
    </linearGradient>\n",
        );
    }

    let defs = if !defs_content.is_empty() {
        format!("  <defs>\n{}\n  </defs>\n", defs_content)
    } else {
        String::new()
    };

    // Build shine overlay for plastic
    let shine_overlay = if metrics.plastic {
        format!(
            "\n  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"url(#shine)\" rx=\"{}\" ry=\"{}\"/>",
            border_offset + shadow_padding / 2,
            border_offset + shadow_padding / 2,
            width,
            height,
            rx,
            ry
        )
    } else {
        String::new()
    };

    // Position rect with shadow padding
    let rect_x = border_offset + shadow_padding / 2;
    let rect_y = border_offset + shadow_padding / 2;

    // Build icon element if icon is specified
    let icon_elem = if let Some(icon_name) = opts.icon {
        if let Some(path_data) = mdfx_icons::icon_path(icon_name) {
            // Simple Icons use a 24x24 viewBox
            // Scale icon to fit within swatch (with padding)
            let icon_size = (height.min(width) as f32 * 0.6).max(10.0);
            let scale = icon_size / 24.0;

            // Center the icon in the swatch
            let icon_x = rect_x as f32 + (width as f32 - icon_size) / 2.0;
            let icon_y = rect_y as f32 + (height as f32 - icon_size) / 2.0;

            // Icon color defaults to white
            let color = opts.icon_color.unwrap_or("FFFFFF");

            format!(
                "\n  <g transform=\"translate({:.1}, {:.1}) scale({:.3})\"><path fill=\"#{}\" d=\"{}\"/></g>",
                icon_x, icon_y, scale, color, path_data
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // Build per-side border lines (drawn on top of rect)
    let has_per_side_borders = opts.border_top.is_some()
        || opts.border_right.is_some()
        || opts.border_bottom.is_some()
        || opts.border_left.is_some();

    let per_side_borders = if has_per_side_borders {
        let mut lines = String::new();

        // Top border: horizontal line at top
        if let Some(spec) = opts.border_top {
            let (color, bw) = parse_border_spec(spec);
            lines.push_str(&format!(
                "\n  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"square\"/>",
                rect_x, rect_y, rect_x + width, rect_y, color, bw
            ));
        }

        // Right border: vertical line at right
        if let Some(spec) = opts.border_right {
            let (color, bw) = parse_border_spec(spec);
            lines.push_str(&format!(
                "\n  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"square\"/>",
                rect_x + width, rect_y, rect_x + width, rect_y + height, color, bw
            ));
        }

        // Bottom border: horizontal line at bottom
        if let Some(spec) = opts.border_bottom {
            let (color, bw) = parse_border_spec(spec);
            lines.push_str(&format!(
                "\n  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"square\"/>",
                rect_x + width, rect_y + height, rect_x, rect_y + height, color, bw
            ));
        }

        // Left border: vertical line at left
        if let Some(spec) = opts.border_left {
            let (color, bw) = parse_border_spec(spec);
            lines.push_str(&format!(
                "\n  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"square\"/>",
                rect_x, rect_y + height, rect_x, rect_y, color, bw
            ));
        }

        lines
    } else {
        String::new()
    };

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
{}  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"{}\" ry=\"{}\"{}{}{}/>{}{}{}{}
</svg>",
        vb_width,
        vb_height,
        vb_width,
        vb_height,
        defs,
        rect_x,
        rect_y,
        width,
        height,
        fill_attr,
        rx,
        ry,
        opacity_attr,
        border_attrs,
        filter_attr,
        per_side_borders,
        shine_overlay,
        icon_elem,
        label_elem
    )
}
