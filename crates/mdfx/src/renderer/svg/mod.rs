//! SVG backend for rendering primitives as local SVG files
//!
//! This backend generates SVG files and stores them in a specified directory.
//! File names are deterministic based on primitive content (hash-based) to
//! enable caching and reproducible builds.

mod donut;
mod gauge;
mod progress;
mod rating;
mod sparkline;
pub mod swatch;
mod tech;
mod waveform;

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
                label,
            } => {
                "tech".hash(&mut hasher);
                name.hash(&mut hasher);
                bg_color.hash(&mut hasher);
                logo_color.hash(&mut hasher);
                style.hash(&mut hasher);
                label.hash(&mut hasher);
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
            Primitive::Sparkline {
                values,
                width,
                height,
                chart_type,
                fill_color,
                stroke_color,
                stroke_width,
                track_color,
                show_dots,
                dot_radius,
            } => {
                "sparkline".hash(&mut hasher);
                // Hash values by converting to bits
                for v in values {
                    v.to_bits().hash(&mut hasher);
                }
                width.hash(&mut hasher);
                height.hash(&mut hasher);
                chart_type.hash(&mut hasher);
                fill_color.hash(&mut hasher);
                stroke_color.hash(&mut hasher);
                stroke_width.hash(&mut hasher);
                track_color.hash(&mut hasher);
                show_dots.hash(&mut hasher);
                dot_radius.hash(&mut hasher);
            }
            Primitive::Rating {
                value,
                max,
                size,
                fill_color,
                empty_color,
                icon,
                spacing,
            } => {
                "rating".hash(&mut hasher);
                value.to_bits().hash(&mut hasher);
                max.hash(&mut hasher);
                size.hash(&mut hasher);
                fill_color.hash(&mut hasher);
                empty_color.hash(&mut hasher);
                icon.hash(&mut hasher);
                spacing.hash(&mut hasher);
            }

            Primitive::Waveform {
                values,
                width,
                height,
                positive_color,
                negative_color,
                bar_width,
                spacing,
                track_color,
                show_center_line,
                center_line_color,
            } => {
                "waveform".hash(&mut hasher);
                for v in values {
                    v.to_bits().hash(&mut hasher);
                }
                width.hash(&mut hasher);
                height.hash(&mut hasher);
                positive_color.hash(&mut hasher);
                negative_color.hash(&mut hasher);
                bar_width.hash(&mut hasher);
                spacing.hash(&mut hasher);
                track_color.hash(&mut hasher);
                show_center_line.hash(&mut hasher);
                center_line_color.hash(&mut hasher);
            }
        }

        let hash = hasher.finish();
        let type_name = match primitive {
            Primitive::Swatch { .. } => "swatch",
            Primitive::Tech { .. } => "tech",
            Primitive::Progress { .. } => "progress",
            Primitive::Donut { .. } => "donut",
            Primitive::Gauge { .. } => "gauge",
            Primitive::Sparkline { .. } => "sparkline",
            Primitive::Rating { .. } => "rating",
            Primitive::Waveform { .. } => "waveform",
        };

        format!("{}_{:x}.svg", type_name, hash)
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
            } => swatch::render(swatch::SwatchOptions {
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
                label,
            } => tech::render_with_label(name, label.as_deref(), bg_color, logo_color, style),

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
            } => progress::render(
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
            } => donut::render(
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
            } => gauge::render(
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

            Primitive::Sparkline {
                values,
                width,
                height,
                chart_type,
                fill_color,
                stroke_color,
                stroke_width,
                track_color,
                show_dots,
                dot_radius,
            } => sparkline::render(
                values,
                *width,
                *height,
                chart_type,
                fill_color,
                stroke_color.as_deref(),
                *stroke_width,
                track_color.as_deref(),
                *show_dots,
                *dot_radius,
            ),

            Primitive::Rating {
                value,
                max,
                size,
                fill_color,
                empty_color,
                icon,
                spacing,
            } => rating::render(*value, *max, *size, fill_color, empty_color, icon, *spacing),

            Primitive::Waveform {
                values,
                width,
                height,
                positive_color,
                negative_color,
                bar_width,
                spacing,
                track_color,
                show_center_line,
                center_line_color,
            } => waveform::render(
                values,
                *width,
                *height,
                positive_color,
                negative_color,
                *bar_width,
                *spacing,
                track_color.as_deref(),
                *show_center_line,
                center_line_color.as_deref(),
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
            label: None,
        };

        let result = backend.render(&primitive).unwrap();
        let svg = String::from_utf8(result.file_bytes().unwrap().to_vec()).unwrap();

        // Should have icon path and colors
        assert!(svg.contains("<path"));
        assert!(svg.contains("000000"));
        assert!(svg.contains("FFFFFF"));
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
