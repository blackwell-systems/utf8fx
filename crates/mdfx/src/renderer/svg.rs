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

/// SVG rendering backend (file-based)
pub struct SvgBackend {
    /// Output directory for generated SVG files (e.g., "assets/mdfx")
    out_dir: String,
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
    icon: Option<&'a str>,
    icon_color: Option<&'a str>,
}

impl SvgBackend {
    /// Create a new SVG backend with specified output directory
    pub fn new(out_dir: impl Into<String>) -> Self {
        Self {
            out_dir: out_dir.into(),
        }
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
            }
            Primitive::Divider { colors, style } => {
                "divider".hash(&mut hasher);
                for color in colors {
                    color.hash(&mut hasher);
                }
                style.hash(&mut hasher);
            }
            Primitive::Status { level, style } => {
                "status".hash(&mut hasher);
                level.hash(&mut hasher);
                style.hash(&mut hasher);
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
        }

        let hash = hasher.finish();
        let type_name = match primitive {
            Primitive::Swatch { .. } => "swatch",
            Primitive::Divider { .. } => "divider",
            Primitive::Status { .. } => "status",
            Primitive::Tech { .. } => "tech",
        };

        format!("{}_{:x}.svg", type_name, hash)
    }

    /// Render a swatch (single colored rectangle with optional enhancements)
    fn render_swatch_svg(opts: SwatchOptions) -> String {
        let metrics = SvgMetrics::from_style(opts.style);

        // Use custom dimensions or defaults
        let width = opts.width.unwrap_or(20);
        let height = opts.height.unwrap_or(metrics.height);

        // Build opacity attribute
        let opacity_attr = match opts.opacity {
            Some(o) if o < 1.0 => format!(" fill-opacity=\"{}\"", o),
            _ => String::new(),
        };

        // Build border attributes
        let (border_attrs, border_offset) = match (opts.border_color, opts.border_width) {
            (Some(bc), Some(bw)) if bw > 0 => {
                (format!(" stroke=\"#{}\" stroke-width=\"{}\"", bc, bw), bw)
            }
            (Some(bc), None) => (format!(" stroke=\"#{}\" stroke-width=\"1\"", bc), 1),
            _ => (String::new(), 0),
        };

        // Adjust viewBox for border
        let vb_width = width + border_offset * 2;
        let vb_height = height + border_offset * 2;

        // Build text content element (icon takes precedence over label)
        let label_elem = if let Some(icon_name) = opts.icon {
            // Render icon as text (fallback - actual icons would require bundling SVG paths)
            let font_size = if height > 24 { 12 } else { 8 };
            let y_pos = height / 2 + font_size / 3 + border_offset;
            let x_pos = width / 2 + border_offset;
            let fill_color = opts.icon_color.unwrap_or("white");
            let fill = if fill_color.chars().all(|c| c.is_ascii_hexdigit()) && fill_color.len() == 6
            {
                format!("#{}", fill_color)
            } else {
                fill_color.to_string()
            };
            // Use uppercase abbreviation for icon name
            let abbrev = if icon_name.len() <= 3 {
                icon_name.to_uppercase()
            } else {
                icon_name[..3].to_uppercase()
            };
            format!(
                "\n  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\">{}</text>",
                x_pos, y_pos, fill, font_size, abbrev
            )
        } else if let Some(text) = opts.label {
            let font_size = if height > 24 { 14 } else { 10 };
            let y_pos = height / 2 + font_size / 3 + border_offset;
            let x_pos = width / 2 + border_offset;
            let fill_color = opts.label_color.unwrap_or("white");
            // If it's a hex color without #, add it
            let fill = if fill_color.chars().all(|c| c.is_ascii_hexdigit()) && fill_color.len() == 6
            {
                format!("#{}", fill_color)
            } else {
                fill_color.to_string()
            };
            format!(
                "\n  <text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" font-weight=\"bold\">{}</text>",
                x_pos, y_pos, fill, font_size, text
            )
        } else {
            String::new()
        };

        // Build defs for plastic effect
        let defs = if metrics.plastic {
            "  <defs>\n\
    <linearGradient id=\"shine\" x1=\"0%\" y1=\"0%\" x2=\"0%\" y2=\"100%\">\n\
      <stop offset=\"0%\" style=\"stop-color:#ffffff;stop-opacity:0.2\" />\n\
      <stop offset=\"100%\" style=\"stop-color:#000000;stop-opacity:0.1\" />\n\
    </linearGradient>\n\
  </defs>\n"
                .to_string()
        } else {
            String::new()
        };

        // Build shine overlay for plastic
        let shine_overlay = if metrics.plastic {
            format!(
                "\n  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"url(#shine)\" rx=\"{}\"/>",
                border_offset, border_offset, width, height, metrics.rx
            )
        } else {
            String::new()
        };

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n\
{}  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"{}{}/>{}{}
</svg>",
            vb_width,
            vb_height,
            vb_width,
            vb_height,
            defs,
            border_offset,
            border_offset,
            width,
            height,
            opts.color,
            metrics.rx,
            opacity_attr,
            border_attrs,
            shine_overlay,
            label_elem
        )
    }

    /// Render a divider (multiple colored rectangles inline)
    fn render_divider_svg(colors: &[String], style: &str) -> String {
        let metrics = SvgMetrics::from_style(style);
        let width_per_block = 20;
        let total_width = colors.len() * width_per_block;

        let mut rects = String::new();
        for (i, color) in colors.iter().enumerate() {
            let x = i * width_per_block;
            rects.push_str(&format!(
                "  <rect x=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>\n",
                x, width_per_block, metrics.height, color, metrics.rx
            ));
        }

        // Add plastic shine overlay if needed
        if metrics.plastic {
            rects.push_str(&format!(
                "  <defs>\n\
    <linearGradient id=\"shine\" x1=\"0%\" y1=\"0%\" x2=\"0%\" y2=\"100%\">\n\
      <stop offset=\"0%\" style=\"stop-color:#ffffff;stop-opacity:0.2\" />\n\
      <stop offset=\"100%\" style=\"stop-color:#000000;stop-opacity:0.1\" />\n\
    </linearGradient>\n\
  </defs>\n\
  <rect width=\"{}\" height=\"{}\" fill=\"url(#shine)\" rx=\"{}\"/>\n",
                total_width, metrics.height, metrics.rx
            ));
        }

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n{}</svg>",
            total_width, metrics.height, total_width, metrics.height, rects
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
                icon,
                icon_color,
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
                icon: icon.as_deref(),
                icon_color: icon_color.as_deref(),
            }),

            Primitive::Divider { colors, style } => Self::render_divider_svg(colors, style),

            Primitive::Status { level, style } => {
                // Status uses simplified swatch (no extra options)
                Self::render_swatch_svg(SwatchOptions {
                    color: level,
                    style,
                    opacity: None,
                    width: None,
                    height: None,
                    border_color: None,
                    border_width: None,
                    label: None,
                    label_color: None,
                    icon: None,
                    icon_color: None,
                })
            }

            Primitive::Tech {
                name,
                bg_color,
                logo_color,
                style,
            } => Self::render_tech_svg(name, bg_color, logo_color, style),
        };

        let markdown_ref = format!("![]({})", relative_path);

        Ok(RenderedAsset::File {
            relative_path,
            bytes: svg.into_bytes(),
            markdown_ref,
            primitive: primitive.clone(),
        })
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
    fn test_render_divider_primitive() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::Divider {
            colors: vec![
                "FF0000".to_string(),
                "00FF00".to_string(),
                "0000FF".to_string(),
            ],
            style: "flat-square".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should have 3 rectangles
        assert_eq!(svg.matches("<rect").count(), 3);
        assert!(svg.contains("FF0000"));
        assert!(svg.contains("00FF00"));
        assert!(svg.contains("0000FF"));
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
    fn test_svg_divider_with_style() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::Divider {
            colors: vec!["FF0000".to_string(), "00FF00".to_string()],
            style: "for-the-badge".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should apply tall height to all blocks
        assert!(svg.contains("height=\"28\""));
        // Should have multiple blocks
        assert!(svg.contains("#FF0000"));
        assert!(svg.contains("#00FF00"));
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
