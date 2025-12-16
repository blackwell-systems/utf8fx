/// SVG backend for rendering primitives as local SVG files
///
/// This backend generates SVG files and stores them in a specified directory.
/// File names are deterministic based on primitive content (hash-based) to
/// enable caching and reproducible builds.
use crate::error::Result;
use crate::primitive::Primitive;
use crate::renderer::{RenderedAsset, Renderer};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// SVG rendering backend (file-based or inline)
pub struct SvgBackend {
    /// Output directory for generated SVG files (e.g., "assets/mdfx")
    out_dir: String,
    /// When true, embed SVGs as data URIs instead of writing files
    inline: bool,
}

/// SVG style metrics for different badge styles
struct SvgMetrics {
    height: u32,
    rx: u32,
    plastic: bool,
}

impl SvgMetrics {
    fn from_style(style: &str) -> Self {
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
struct SwatchOptions<'a> {
    color: &'a str,
    style: &'a str,
    opacity: Option<f32>,
    width: Option<u32>,
    height: Option<u32>,
    border_color: Option<&'a str>,
    border_width: Option<u32>,
    label: Option<&'a str>,
    label_color: Option<&'a str>,
    rx: Option<u32>,
    ry: Option<u32>,
    shadow: Option<&'a str>,
    gradient: Option<&'a str>,
    stroke_dash: Option<&'a str>,
    /// Per-side borders (format: "color/width" or just "color")
    border_top: Option<&'a str>,
    border_right: Option<&'a str>,
    border_bottom: Option<&'a str>,
    border_left: Option<&'a str>,
}

impl SvgBackend {
    /// Create a new SVG backend with specified output directory
    pub fn new(out_dir: impl Into<String>) -> Self {
        Self {
            out_dir: out_dir.into(),
            inline: false,
        }
    }

    /// Create a new inline SVG backend (embeds as data URIs)
    pub fn new_inline() -> Self {
        Self {
            out_dir: String::new(),
            inline: true,
        }
    }

    /// Check if this backend uses inline mode
    pub fn is_inline(&self) -> bool {
        self.inline
    }

    /// Generate deterministic filename for a primitive
    fn filename_for(primitive: &Primitive) -> String {
        let mut hasher = DefaultHasher::new();
        // Hash the primitive's discriminant and data
        match primitive {
            Primitive::Swatch {
                color,
                style,
                opacity,
                width,
                height,
                border_color,
                border_width,
                label,
                label_color,
                icon,
                icon_color,
                rx,
                ry,
                shadow,
                gradient,
                stroke_dash,
                logo_size,
                border_top,
                border_right,
                border_bottom,
                border_left,
            } => {
                "swatch".hash(&mut hasher);
                color.hash(&mut hasher);
                style.hash(&mut hasher);
                // Hash optional fields for unique filenames
                if let Some(o) = opacity {
                    o.to_bits().hash(&mut hasher);
                }
                width.hash(&mut hasher);
                height.hash(&mut hasher);
                border_color.hash(&mut hasher);
                border_width.hash(&mut hasher);
                label.hash(&mut hasher);
                label_color.hash(&mut hasher);
                icon.hash(&mut hasher);
                icon_color.hash(&mut hasher);
                rx.hash(&mut hasher);
                ry.hash(&mut hasher);
                shadow.hash(&mut hasher);
                gradient.hash(&mut hasher);
                stroke_dash.hash(&mut hasher);
                logo_size.hash(&mut hasher);
                border_top.hash(&mut hasher);
                border_right.hash(&mut hasher);
                border_bottom.hash(&mut hasher);
                border_left.hash(&mut hasher);
            }
            Primitive::Tech {
                name,
                bg_color,
                logo_color,
                style,
            } => {
                "tech".hash(&mut hasher);
                name.hash(&mut hasher);
                bg_color.hash(&mut hasher);
                logo_color.hash(&mut hasher);
                style.hash(&mut hasher);
            }
            Primitive::Progress {
                percent,
                width,
                height,
                track_color,
                fill_color,
                fill_height,
                rx,
                show_label,
                label_color,
                border_color,
                border_width,
                thumb_size,
                thumb_width,
                thumb_color,
                thumb_shape,
            } => {
                "progress".hash(&mut hasher);
                percent.hash(&mut hasher);
                width.hash(&mut hasher);
                height.hash(&mut hasher);
                track_color.hash(&mut hasher);
                fill_color.hash(&mut hasher);
                fill_height.hash(&mut hasher);
                rx.hash(&mut hasher);
                show_label.hash(&mut hasher);
                label_color.hash(&mut hasher);
                border_color.hash(&mut hasher);
                border_width.hash(&mut hasher);
                thumb_size.hash(&mut hasher);
                thumb_width.hash(&mut hasher);
                thumb_color.hash(&mut hasher);
                thumb_shape.hash(&mut hasher);
            }
            Primitive::Donut {
                percent,
                size,
                thickness,
                track_color,
                fill_color,
                show_label,
                label_color,
                thumb_size,
                thumb_color,
            } => {
                "donut".hash(&mut hasher);
                percent.hash(&mut hasher);
                size.hash(&mut hasher);
                thickness.hash(&mut hasher);
                track_color.hash(&mut hasher);
                fill_color.hash(&mut hasher);
                show_label.hash(&mut hasher);
                label_color.hash(&mut hasher);
                thumb_size.hash(&mut hasher);
                thumb_color.hash(&mut hasher);
            }
            Primitive::Gauge {
                percent,
                size,
                thickness,
                track_color,
                fill_color,
                show_label,
                label_color,
                thumb_size,
                thumb_color,
            } => {
                "gauge".hash(&mut hasher);
                percent.hash(&mut hasher);
                size.hash(&mut hasher);
                thickness.hash(&mut hasher);
                track_color.hash(&mut hasher);
                fill_color.hash(&mut hasher);
                show_label.hash(&mut hasher);
                label_color.hash(&mut hasher);
                thumb_size.hash(&mut hasher);
                thumb_color.hash(&mut hasher);
            }
        }

        let hash = hasher.finish();
        let type_name = match primitive {
            Primitive::Swatch { .. } => "swatch",
            Primitive::Tech { .. } => "tech",
            Primitive::Progress { .. } => "progress",
            Primitive::Donut { .. } => "donut",
            Primitive::Gauge { .. } => "gauge",
        };

        format!("{}_{:x}.svg", type_name, hash)
    }

    /// Parse shadow config: "color/blur/offset_x/offset_y" (e.g., "000000/4/2/2")
    /// Uses / as delimiter to avoid conflict with template's : separator
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
    /// Uses / as delimiter to avoid conflict with template's : separator
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
    fn render_swatch_svg(opts: SwatchOptions) -> String {
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
        let shadow_config = opts.shadow.and_then(Self::parse_shadow);

        // Parse gradient if provided
        let gradient_config = opts.gradient.and_then(Self::parse_gradient);

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

        // Build per-side border lines (drawn on top of rect)
        let has_per_side_borders = opts.border_top.is_some()
            || opts.border_right.is_some()
            || opts.border_bottom.is_some()
            || opts.border_left.is_some();

        let per_side_borders = if has_per_side_borders {
            let mut lines = String::new();

            // Top border: horizontal line at top
            if let Some(spec) = opts.border_top {
                let (color, bw) = Self::parse_border_spec(spec);
                lines.push_str(&format!(
                    "\n  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"square\"/>",
                    rect_x, rect_y, rect_x + width, rect_y, color, bw
                ));
            }

            // Right border: vertical line at right
            if let Some(spec) = opts.border_right {
                let (color, bw) = Self::parse_border_spec(spec);
                lines.push_str(&format!(
                    "\n  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"square\"/>",
                    rect_x + width, rect_y, rect_x + width, rect_y + height, color, bw
                ));
            }

            // Bottom border: horizontal line at bottom
            if let Some(spec) = opts.border_bottom {
                let (color, bw) = Self::parse_border_spec(spec);
                lines.push_str(&format!(
                    "\n  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#{}\" stroke-width=\"{}\" stroke-linecap=\"square\"/>",
                    rect_x + width, rect_y + height, rect_x, rect_y + height, color, bw
                ));
            }

            // Left border: vertical line at left
            if let Some(spec) = opts.border_left {
                let (color, bw) = Self::parse_border_spec(spec);
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
{}  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"{}\" ry=\"{}\"{}{}{}/>{}{}{}
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
            label_elem
        )
    }

    /// Render a tech badge (with placeholder for logo)
    fn render_tech_svg(name: &str, bg_color: &str, _logo_color: &str, style: &str) -> String {
        let metrics = SvgMetrics::from_style(style);
        // MVP: render with text instead of logo
        // Full implementation would require bundling Simple Icons SVGs
        let font_size = if metrics.height > 24 { 16 } else { 12 };
        let y_pos = metrics.height / 2 + font_size / 3;

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"80\" height=\"{}\" viewBox=\"0 0 80 {}\">\n\
  <rect width=\"80\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>\n\
  <text x=\"40\" y=\"{}\" text-anchor=\"middle\" fill=\"white\" font-family=\"Arial, sans-serif\" font-size=\"{}\">{}</text>\n\
</svg>",
            metrics.height, metrics.height, metrics.height, bg_color, metrics.rx, y_pos, font_size,
            name.to_uppercase()
        )
    }

    /// Render a progress bar with track and fill
    #[allow(clippy::too_many_arguments)]
    fn render_progress_svg(
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
    ) -> String {
        // Slider mode: track with thumb at position
        if let Some(thumb_sz) = thumb_size {
            return Self::render_slider_svg(
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
            width, height, width, height,
            width, height, track_color, rx, border_attr,
            fill_y, fill_width, fill_height, fill_color, fill_rx,
            label_elem
        )
    }

    /// Render a slider with track and thumb at position
    #[allow(clippy::too_many_arguments)]
    fn render_slider_svg(
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

        // Render thumb based on shape
        let thumb_elem = match thumb_shape {
            "square" => {
                let half_h = thumb_height / 2;
                let half_w = thumb_w / 2;
                // Use smaller dimension for rx to create pill shape when width != height
                let rx = half_h.min(half_w).min(4);
                format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>",
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
                    "<polygon points=\"{},{} {},{} {},{} {},{}\" fill=\"#{}\"/>",
                    thumb_x, center_y - half_h,        // top
                    thumb_x + half_w, center_y,        // right
                    thumb_x, center_y + half_h,        // bottom
                    thumb_x - half_w, center_y,        // left
                    t_color
                )
            }
            _ => {
                // Default: ellipse (or circle if width == height)
                let rx = thumb_w as f32 / 2.0;
                let ry = thumb_height as f32 / 2.0;
                format!(
                    "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"#{}\"/>",
                    thumb_x, center_y, rx, ry, t_color
                )
            }
        };

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
  <rect x=\"0\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}/>{}\n\
  {}\n\
</svg>",
            width, svg_height, width, svg_height,
            track_y, width, track_height, track_color, track_rx, border_attr,
            fill_elem,
            thumb_elem
        )
    }

    /// Render a donut/ring chart using stroke-dasharray trick
    #[allow(clippy::too_many_arguments)]
    fn render_donut_svg(
        percent: u8,
        size: u32,
        thickness: u32,
        track_color: &str,
        fill_color: &str,
        show_label: bool,
        label_color: Option<&str>,
        thumb_size: Option<u32>,
        thumb_color: Option<&str>,
    ) -> String {
        // Use radius ~15.9 so circumference ≈ 100 (makes percentage math easy)
        // Scale radius based on size: r = (size/2 - thickness/2)
        let center = size as f32 / 2.0;
        let radius = center - (thickness as f32 / 2.0);
        let circumference = 2.0 * std::f32::consts::PI * radius;

        // Calculate dash lengths for percentage
        let fill_length = circumference * (percent as f32 / 100.0);
        let gap_length = circumference - fill_length;

        // Calculate viewbox padding for thumb (thumb might extend beyond circle)
        let thumb_padding = thumb_size.map(|t| (t / 2).saturating_sub(thickness / 2)).unwrap_or(0);
        let svg_size = size + thumb_padding * 2;
        let adjusted_center = center + thumb_padding as f32;

        // Build label element if requested (and size is large enough)
        let label_elem = if show_label && size >= 30 {
            let label_col = label_color.unwrap_or("FFFFFF");
            // Font size scales with donut size
            let font_size = (size / 4).max(10).min(16);
            format!(
                "\n  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" dominant-baseline=\"central\" fill=\"#{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\">{}%</text>",
                adjusted_center, adjusted_center, label_col, font_size, percent
            )
        } else {
            String::new()
        };

        // Build thumb element if requested
        let thumb_elem = if let Some(thumb_sz) = thumb_size {
            let t_color = thumb_color.unwrap_or(fill_color);
            // Calculate thumb position on the circle
            // Angle: starts at top (-90°), progresses clockwise
            let angle_deg = -90.0 + (percent as f32 * 360.0 / 100.0);
            let angle_rad = angle_deg * std::f32::consts::PI / 180.0;
            let thumb_x = adjusted_center + radius * angle_rad.cos();
            let thumb_y = adjusted_center + radius * angle_rad.sin();
            let thumb_r = thumb_sz as f32 / 2.0;
            format!(
                "\n  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#{}\"/>",
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
            svg_size, svg_size, svg_size, svg_size,
            adjusted_center, adjusted_center, radius, track_color, thickness,
            adjusted_center, adjusted_center, radius, fill_color, thickness, fill_length, gap_length, adjusted_center, adjusted_center,
            thumb_elem, label_elem
        )
    }

    /// Render a gauge (semi-circular meter) using SVG arc paths
    #[allow(clippy::too_many_arguments)]
    fn render_gauge_svg(
        percent: u8,
        size: u32,
        thickness: u32,
        track_color: &str,
        fill_color: &str,
        show_label: bool,
        label_color: Option<&str>,
        thumb_size: Option<u32>,
        thumb_color: Option<&str>,
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
        let thumb_padding = thumb_size.map(|t| (t / 2).saturating_sub(thickness / 2)).unwrap_or(0);

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
            let font_size = (size / 5).max(12).min(18);
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
            format!(
                "\n  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#{}\"/>",
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
            size, svg_height, size, svg_height,
            track_path, track_color, thickness,
            track_path, fill_color, thickness, fill_length, gap_length,
            thumb_elem, label_elem
        )
    }
}

impl Renderer for SvgBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        let filename = Self::filename_for(primitive);
        let relative_path = format!("{}/{}", self.out_dir, filename);

        let svg = match primitive {
            Primitive::Swatch {
                color,
                style,
                opacity,
                width,
                height,
                border_color,
                border_width,
                label,
                label_color,
                icon: _,
                icon_color: _,
                rx,
                ry,
                shadow,
                gradient,
                stroke_dash,
                logo_size: _,
                border_top,
                border_right,
                border_bottom,
                border_left,
            } => Self::render_swatch_svg(SwatchOptions {
                color,
                style,
                opacity: *opacity,
                width: *width,
                height: *height,
                border_color: border_color.as_deref(),
                border_width: *border_width,
                label: label.as_deref(),
                label_color: label_color.as_deref(),
                rx: *rx,
                ry: *ry,
                shadow: shadow.as_deref(),
                gradient: gradient.as_deref(),
                stroke_dash: stroke_dash.as_deref(),
                border_top: border_top.as_deref(),
                border_right: border_right.as_deref(),
                border_bottom: border_bottom.as_deref(),
                border_left: border_left.as_deref(),
            }),

            Primitive::Tech {
                name,
                bg_color,
                logo_color,
                style,
            } => Self::render_tech_svg(name, bg_color, logo_color, style),

            Primitive::Progress {
                percent,
                width,
                height,
                track_color,
                fill_color,
                fill_height,
                rx,
                show_label,
                label_color,
                border_color,
                border_width,
                thumb_size,
                thumb_width,
                thumb_color,
                thumb_shape,
            } => Self::render_progress_svg(
                *percent,
                *width,
                *height,
                track_color,
                fill_color,
                *fill_height,
                *rx,
                *show_label,
                label_color.as_deref(),
                border_color.as_deref(),
                *border_width,
                *thumb_size,
                *thumb_width,
                thumb_color.as_deref(),
                thumb_shape,
            ),

            Primitive::Donut {
                percent,
                size,
                thickness,
                track_color,
                fill_color,
                show_label,
                label_color,
                thumb_size,
                thumb_color,
            } => Self::render_donut_svg(
                *percent,
                *size,
                *thickness,
                track_color,
                fill_color,
                *show_label,
                label_color.as_deref(),
                *thumb_size,
                thumb_color.as_deref(),
            ),

            Primitive::Gauge {
                percent,
                size,
                thickness,
                track_color,
                fill_color,
                show_label,
                label_color,
                thumb_size,
                thumb_color,
            } => Self::render_gauge_svg(
                *percent,
                *size,
                *thickness,
                track_color,
                fill_color,
                *show_label,
                label_color.as_deref(),
                *thumb_size,
                thumb_color.as_deref(),
            ),
        };

        // Handle inline mode (raw SVG) vs file mode
        if self.inline {
            // Output raw SVG directly (works in most markdown renderers that support HTML)
            Ok(RenderedAsset::InlineMarkdown(svg))
        } else {
            let markdown_ref = format!("![]({})", relative_path);
            Ok(RenderedAsset::File {
                relative_path,
                bytes: svg.into_bytes(),
                markdown_ref,
                primitive: Box::new(primitive.clone()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_backend_creation() {
        let backend = SvgBackend::new("assets/mdfx");
        assert_eq!(backend.out_dir, "assets/mdfx");
    }

    #[test]
    fn test_render_swatch_primitive() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::simple_swatch("F41C80", "flat-square");

        let result = backend.render(&primitive).unwrap();

        // Check it's a file-based asset
        assert!(result.is_file_based());

        // Check path format
        let path = result.file_path().unwrap();
        assert!(path.starts_with("assets/swatch_"));
        assert!(path.ends_with(".svg"));

        // Check SVG content
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();
        assert!(svg.contains("F41C80"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_render_tech_primitive() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "FFFFFF".to_string(),
            style: "flat-square".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should have text "RUST" (MVP mode)
        assert!(svg.contains("RUST"));
        assert!(svg.contains("000000"));
    }

    #[test]
    fn test_deterministic_filenames() {
        let backend = SvgBackend::new("assets");
        let primitive1 = Primitive::simple_swatch("F41C80", "flat-square");
        let primitive2 = Primitive::simple_swatch("F41C80", "flat-square");

        let result1 = backend.render(&primitive1).unwrap();
        let result2 = backend.render(&primitive2).unwrap();

        // Same primitive → same filename
        assert_eq!(result1.file_path(), result2.file_path());
    }

    #[test]
    fn test_different_primitives_different_filenames() {
        let backend = SvgBackend::new("assets");
        let primitive1 = Primitive::simple_swatch("F41C80", "flat-square");
        let primitive2 = Primitive::simple_swatch("00FF00", "flat-square");

        let result1 = backend.render(&primitive1).unwrap();
        let result2 = backend.render(&primitive2).unwrap();

        // Different color → different filename
        assert_ne!(result1.file_path(), result2.file_path());
    }

    #[test]
    fn test_svg_flat_style_rounded_corners() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::simple_swatch("F41C80", "flat");

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should have rounded corners (rx="3")
        assert!(svg.contains("rx=\"3\""));
        // Should be standard height
        assert!(svg.contains("height=\"20\""));
    }

    #[test]
    fn test_svg_flat_square_style_sharp_corners() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::simple_swatch("F41C80", "flat-square");

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should have sharp corners (rx="0")
        assert!(svg.contains("rx=\"0\""));
    }

    #[test]
    fn test_svg_for_the_badge_tall_height() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::simple_swatch("F41C80", "for-the-badge");

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should be taller (height="28")
        assert!(svg.contains("height=\"28\""));
        assert!(svg.contains("viewBox=\"0 0 20 28\""));
    }

    #[test]
    fn test_svg_plastic_style_has_gradient() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::simple_swatch("F41C80", "plastic");

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should have gradient definition
        assert!(svg.contains("<linearGradient"));
        assert!(svg.contains("id=\"shine\""));
        // Should have overlay rect using gradient
        assert!(svg.contains("fill=\"url(#shine)\""));
    }

    #[test]
    fn test_svg_social_style_very_rounded() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::simple_swatch("F41C80", "social");

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should be very rounded (rx="10")
        assert!(svg.contains("rx=\"10\""));
    }

    #[test]
    fn test_svg_style_affects_filename_hash() {
        let swatch_flat = Primitive::simple_swatch("F41C80", "flat");
        let swatch_square = Primitive::simple_swatch("F41C80", "flat-square");

        let filename_flat = SvgBackend::filename_for(&swatch_flat);
        let filename_square = SvgBackend::filename_for(&swatch_square);

        // Different styles should produce different filenames
        assert_ne!(filename_flat, filename_square);
        // Both should start with "swatch_"
        assert!(filename_flat.starts_with("swatch_"));
        assert!(filename_square.starts_with("swatch_"));
    }
}
