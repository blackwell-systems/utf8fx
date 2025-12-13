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
            Primitive::Swatch { color, style } => {
                "swatch".hash(&mut hasher);
                color.hash(&mut hasher);
                style.hash(&mut hasher);
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

    /// Render a swatch (single colored rectangle)
    fn render_swatch_svg(color: &str, style: &str) -> String {
        let metrics = SvgMetrics::from_style(style);
        let svg = if metrics.plastic {
            // Plastic style: add vertical gradient for shine effect
            format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"{}\" viewBox=\"0 0 20 {}\">\n\
  <defs>\n\
    <linearGradient id=\"shine\" x1=\"0%\" y1=\"0%\" x2=\"0%\" y2=\"100%\">\n\
      <stop offset=\"0%\" style=\"stop-color:#ffffff;stop-opacity:0.2\" />\n\
      <stop offset=\"100%\" style=\"stop-color:#000000;stop-opacity:0.1\" />\n\
    </linearGradient>\n\
  </defs>\n\
  <rect width=\"20\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>\n\
  <rect width=\"20\" height=\"{}\" fill=\"url(#shine)\" rx=\"{}\"/>\n\
</svg>",
                metrics.height, metrics.height, metrics.height, color, metrics.rx, metrics.height, metrics.rx
            )
        } else {
            format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"{}\" viewBox=\"0 0 20 {}\">\n\
  <rect width=\"20\" height=\"{}\" fill=\"#{}\" rx=\"{}\"/>\n\
</svg>",
                metrics.height, metrics.height, metrics.height, color, metrics.rx
            )
        };
        svg
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
            Primitive::Swatch { color, style } => Self::render_swatch_svg(color, style),

            Primitive::Divider { colors, style } => Self::render_divider_svg(colors, style),

            Primitive::Status { level, style } => Self::render_swatch_svg(level, style),

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
        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
        };

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
        let primitive1 = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
        };
        let primitive2 = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
        };

        let result1 = backend.render(&primitive1).unwrap();
        let result2 = backend.render(&primitive2).unwrap();

        // Same primitive → same filename
        assert_eq!(result1.file_path(), result2.file_path());
    }

    #[test]
    fn test_different_primitives_different_filenames() {
        let backend = SvgBackend::new("assets");
        let primitive1 = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
        };
        let primitive2 = Primitive::Swatch {
            color: "00FF00".to_string(),
            style: "flat-square".to_string(),
        };

        let result1 = backend.render(&primitive1).unwrap();
        let result2 = backend.render(&primitive2).unwrap();

        // Different color → different filename
        assert_ne!(result1.file_path(), result2.file_path());
    }

    #[test]
    fn test_svg_flat_style_rounded_corners() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat".to_string(),
        };

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
        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should have sharp corners (rx="0")
        assert!(svg.contains("rx=\"0\""));
    }

    #[test]
    fn test_svg_for_the_badge_tall_height() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "for-the-badge".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should be taller (height="28")
        assert!(svg.contains("height=\"28\""));
        assert!(svg.contains("viewBox=\"0 0 20 28\""));
    }

    #[test]
    fn test_svg_plastic_style_has_gradient() {
        let backend = SvgBackend::new("assets");
        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "plastic".to_string(),
        };

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
        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "social".to_string(),
        };

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
        let swatch_flat = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat".to_string(),
        };
        let swatch_square = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
        };

        let filename_flat = SvgBackend::filename_for(&swatch_flat);
        let filename_square = SvgBackend::filename_for(&swatch_square);

        // Different styles should produce different filenames
        assert_ne!(filename_flat, filename_square);
        // Both should start with "swatch_"
        assert!(filename_flat.starts_with("swatch_"));
        assert!(filename_square.starts_with("swatch_"));
    }
}
