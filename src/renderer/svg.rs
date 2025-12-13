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
    /// Output directory for generated SVG files (e.g., "assets/utf8fx")
    out_dir: String,
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
    fn render_swatch_svg(color: &str) -> String {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\" viewBox=\"0 0 20 20\">\n\
  <rect width=\"20\" height=\"20\" fill=\"#{}\" rx=\"2\"/>\n\
</svg>",
            color
        )
    }

    /// Render a divider (multiple colored rectangles inline)
    fn render_divider_svg(colors: &[String]) -> String {
        let width_per_block = 20;
        let total_width = colors.len() * width_per_block;
        let height = 20;

        let mut rects = String::new();
        for (i, color) in colors.iter().enumerate() {
            let x = i * width_per_block;
            rects.push_str(&format!(
                "  <rect x=\"{}\" width=\"{}\" height=\"{}\" fill=\"#{}\" rx=\"2\"/>\n",
                x, width_per_block, height, color
            ));
        }

        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n{}</svg>",
            total_width, height, total_width, height, rects
        )
    }

    /// Render a tech badge (with placeholder for logo)
    fn render_tech_svg(name: &str, bg_color: &str, _logo_color: &str) -> String {
        // MVP: render with text instead of logo
        // Full implementation would require bundling Simple Icons SVGs
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"80\" height=\"20\" viewBox=\"0 0 80 20\">\n\
  <rect width=\"80\" height=\"20\" fill=\"#{}\" rx=\"3\"/>\n\
  <text x=\"40\" y=\"14\" text-anchor=\"middle\" fill=\"white\" font-family=\"Arial, sans-serif\" font-size=\"12\">{}</text>\n\
</svg>",
            bg_color,
            name.to_uppercase()
        )
    }
}

impl Renderer for SvgBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        let filename = Self::filename_for(primitive);
        let relative_path = format!("{}/{}", self.out_dir, filename);

        let svg = match primitive {
            Primitive::Swatch { color, .. } => Self::render_swatch_svg(color),

            Primitive::Divider { colors, .. } => Self::render_divider_svg(colors),

            Primitive::Status { level, .. } => Self::render_swatch_svg(level),

            Primitive::Tech {
                name,
                bg_color,
                logo_color,
                ..
            } => Self::render_tech_svg(name, bg_color, logo_color),
        };

        let markdown_ref = format!("![]({})", relative_path);

        Ok(RenderedAsset::File {
            relative_path,
            bytes: svg.into_bytes(),
            markdown_ref,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_backend_creation() {
        let backend = SvgBackend::new("assets/utf8fx");
        assert_eq!(backend.out_dir, "assets/utf8fx");
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
}
