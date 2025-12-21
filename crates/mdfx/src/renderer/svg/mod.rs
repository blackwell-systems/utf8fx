//! SVG backend for rendering primitives as local SVG files
//!
//! This backend generates SVG files and stores them in a specified directory.
//! Filenames are content-addressed using SHA-256 hashing for:
//! - Stable filenames across Rust versions
//! - True deduplication (same content = same file)
//! - Reproducible builds

mod donut;
mod gauge;
mod progress;
mod rating;
mod sparkline;
pub mod swatch;
pub mod tech;
mod utils;
mod waveform;

use crate::error::Result;
use crate::manifest::content_addressed_filename;
use crate::primitive::Primitive;
use crate::renderer::{RenderedAsset, Renderer};

/// SVG rendering backend (file-based or inline)
pub struct SvgBackend {
    /// Output directory for generated SVG files (e.g., "assets/mdfx")
    out_dir: String,
    /// Prefix for asset paths in markdown references (e.g., "assets/" instead of "examples/assets/")
    /// When None, uses out_dir as the prefix
    assets_prefix: Option<String>,
    /// When true, embed SVGs as data URIs instead of writing files
    inline: bool,
}

impl SvgBackend {
    /// Create a new SVG backend with specified output directory
    pub fn new(out_dir: impl Into<String>) -> Self {
        Self {
            out_dir: out_dir.into(),
            assets_prefix: None,
            inline: false,
        }
    }

    /// Create a new SVG backend with separate output dir and markdown prefix
    ///
    /// Use this when you write files to one location (e.g., `examples/assets/`)
    /// but need markdown to reference a different path (e.g., `assets/`).
    pub fn with_prefix(out_dir: impl Into<String>, assets_prefix: impl Into<String>) -> Self {
        Self {
            out_dir: out_dir.into(),
            assets_prefix: Some(assets_prefix.into()),
            inline: false,
        }
    }

    /// Create a new inline SVG backend (embeds as data URIs)
    pub fn new_inline() -> Self {
        Self {
            out_dir: String::new(),
            assets_prefix: None,
            inline: true,
        }
    }

    /// Check if this backend uses inline mode
    pub fn is_inline(&self) -> bool {
        self.inline
    }

    /// Get the type prefix for a primitive (used in filenames)
    fn type_prefix(primitive: &Primitive) -> &'static str {
        match primitive {
            Primitive::Swatch { .. } => "swatch",
            Primitive::Tech(_) => "tech",
            Primitive::Version(_) => "version",
            Primitive::License(_) => "license",
            Primitive::Progress { .. } => "progress",
            Primitive::Donut { .. } => "donut",
            Primitive::Gauge { .. } => "gauge",
            Primitive::Sparkline { .. } => "sparkline",
            Primitive::Rating { .. } => "rating",
            Primitive::Waveform { .. } => "waveform",
        }
    }
}

impl Renderer for SvgBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        // First, render the SVG content
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
                icon: icon.as_deref(),
                icon_color: icon_color.as_deref(),
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

            Primitive::Tech(cfg) => {
                // If source=shields, use shields.io URL instead of SVG
                if cfg.source.as_deref() == Some("shields") {
                    let label_text = cfg.label.as_deref().unwrap_or(&cfg.name);
                    // Simple URL encoding for common characters
                    let encode = |s: &str| {
                        s.replace(' ', "%20")
                            .replace('#', "%23")
                            .replace('+', "%2B")
                            .replace('&', "%26")
                    };
                    let shields_url = format!(
                        "https://img.shields.io/badge/{}-{}-{}?style={}&logo={}&logoColor={}",
                        encode(label_text),
                        encode(label_text),
                        cfg.bg_color,
                        cfg.style,
                        encode(&cfg.name),
                        cfg.logo_color
                    );
                    let markdown = if let Some(link_url) = &cfg.url {
                        format!("[![]({})]({})", shields_url, link_url)
                    } else {
                        format!("![]({})", shields_url)
                    };
                    return Ok(RenderedAsset::InlineMarkdown(markdown));
                }
                // Otherwise render as SVG
                tech::render_with_options(
                    &cfg.name,
                    cfg.label.as_deref(),
                    &cfg.bg_color,
                    &cfg.logo_color,
                    &cfg.style,
                    cfg.border_color.as_deref(),
                    cfg.border_width,
                    cfg.border_full,
                    cfg.divider,
                    cfg.rx,
                    cfg.corners,
                    cfg.text_color.as_deref(),
                    cfg.font.as_deref(),
                    cfg.chevron.as_deref(),
                    cfg.bg_left.as_deref(),
                    cfg.bg_right.as_deref(),
                    cfg.raised,
                    cfg.logo_size,
                    cfg.icon.as_deref(),
                )
            }

            Primitive::Version(cfg) => {
                // Use badgefx to render version badge
                let mut builder = badgefx::version(&cfg.version);

                // Apply status override if specified
                if let Some(status_str) = &cfg.status {
                    if let Some(status) = badgefx::version::parse_status(status_str) {
                        builder = builder.status(status);
                    }
                }

                // Apply style
                builder = builder.style(badgefx::BadgeStyle::parse(&cfg.style));

                // Apply optional overrides
                if let Some(bg) = &cfg.bg_color {
                    builder = builder.bg_color(bg);
                }
                if let Some(text) = &cfg.text_color {
                    builder = builder.text_color(text);
                }
                if let Some(prefix) = &cfg.prefix {
                    if prefix.is_empty() {
                        builder = builder.no_prefix();
                    } else {
                        builder = builder.prefix(prefix);
                    }
                }
                if let (Some(bc), Some(bw)) = (&cfg.border_color, cfg.border_width) {
                    builder = builder.border(bc, bw);
                }
                if let Some(rx) = cfg.rx {
                    builder = builder.rx(rx);
                }

                builder.render()
            }

            Primitive::License(cfg) => {
                // Use badgefx to render license badge
                let mut builder = badgefx::license(&cfg.license);

                // Apply style
                builder = builder.style(badgefx::BadgeStyle::parse(&cfg.style));

                // Apply optional overrides
                if let Some(label) = &cfg.label {
                    builder = builder.label(label);
                }
                if let Some(bg) = &cfg.bg_color {
                    builder = builder.bg_color(bg);
                }
                if let Some(text) = &cfg.text_color {
                    builder = builder.text_color(text);
                }
                if let (Some(bc), Some(bw)) = (&cfg.border_color, cfg.border_width) {
                    builder = builder.border(bc, bw);
                }
                if let Some(rx) = cfg.rx {
                    builder = builder.rx(rx);
                }

                builder.render()
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
                thumb,
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
                thumb.as_ref(),
            ),

            Primitive::Donut {
                percent,
                size,
                thickness,
                track_color,
                fill_color,
                show_label,
                label_color,
                thumb,
            } => donut::render(
                *percent,
                *size,
                *thickness,
                track_color,
                fill_color,
                *show_label,
                label_color.as_deref(),
                thumb.as_ref(),
            ),

            Primitive::Gauge {
                percent,
                size,
                thickness,
                track_color,
                fill_color,
                show_label,
                label_color,
                thumb,
            } => gauge::render(
                *percent,
                *size,
                *thickness,
                track_color,
                fill_color,
                *show_label,
                label_color.as_deref(),
                thumb.as_ref(),
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
            // Generate content-addressed filename from rendered SVG bytes
            // This ensures:
            // 1. Stable filenames across Rust versions (SHA-256 based)
            // 2. True deduplication (same content = same filename)
            // 3. Reproducible builds
            let svg_bytes = svg.as_bytes();
            let type_prefix = Self::type_prefix(primitive);
            let filename = content_addressed_filename(svg_bytes, type_prefix);

            // File path uses out_dir (where files are written)
            let out_dir = self.out_dir.trim_end_matches('/');
            let relative_path = format!("{}/{}", out_dir, filename);

            // Markdown reference uses assets_prefix if set, otherwise out_dir
            let md_prefix = self
                .assets_prefix
                .as_ref()
                .map(|p| p.trim_end_matches('/'))
                .unwrap_or(out_dir);
            let md_path = format!("{}/{}", md_prefix, filename);

            // Generate markdown image reference, optionally wrapped in a link
            let markdown_ref = if let Primitive::Tech(cfg) = primitive {
                if let Some(url) = &cfg.url {
                    format!("[![]({})]({})", md_path, url)
                } else {
                    format!("![]({})", md_path)
                }
            } else {
                format!("![]({})", md_path)
            };
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
    use crate::primitive::TechConfig;
    use insta::assert_snapshot;
    use rstest::rstest;

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

        match result {
            RenderedAsset::File {
                relative_path,
                bytes,
                markdown_ref,
                ..
            } => {
                // Content-addressed filename format: swatch_{16-char-hash}.svg
                assert!(relative_path.starts_with("assets/swatch_"));
                assert!(relative_path.ends_with(".svg"));
                assert_eq!(
                    relative_path.len(),
                    "assets/".len() + "swatch_".len() + 16 + ".svg".len()
                );
                assert!(!bytes.is_empty());
                assert!(markdown_ref.contains(&relative_path));
            }
            _ => unreachable!("Expected File asset"),
        }
    }

    // ========================================================================
    // Content-Addressed Filename Determinism (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("F41C80", "F41C80", true)] // same color -> same filename
    #[case("F41C80", "2B6CB0", false)] // different colors -> different filenames
    #[case("FFFFFF", "FFFFFF", true)] // same color -> same filename
    #[case("000000", "FFFFFF", false)] // black vs white -> different filenames
    fn test_content_addressed_filenames(
        #[case] color1: &str,
        #[case] color2: &str,
        #[case] should_match: bool,
    ) {
        let backend = SvgBackend::new("assets");
        let prim1 = Primitive::simple_swatch(color1, "flat-square");
        let prim2 = Primitive::simple_swatch(color2, "flat-square");

        let result1 = backend.render(&prim1).unwrap();
        let result2 = backend.render(&prim2).unwrap();

        assert_eq!(result1.file_path() == result2.file_path(), should_match);
    }

    #[test]
    fn test_inline_mode() {
        let backend = SvgBackend::new_inline();
        assert!(backend.is_inline());

        let primitive = Primitive::simple_swatch("F41C80", "flat-square");
        let result = backend.render(&primitive).unwrap();

        let RenderedAsset::InlineMarkdown(svg) = result else {
            unreachable!("Expected InlineMarkdown asset");
        };
        assert!(svg.starts_with("<svg"));
    }

    // ========================================================================
    // Type Prefix Detection (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(Primitive::simple_swatch("F41C80", "flat"), "swatch")]
    #[case(Primitive::Tech(TechConfig::new("rust")), "tech")]
    #[case(Primitive::simple_progress(50, "E0E0E0", "4CAF50"), "progress")]
    #[case(Primitive::simple_donut(75, "E0E0E0", "4CAF50"), "donut")]
    fn test_type_prefix(#[case] primitive: Primitive, #[case] expected: &str) {
        assert_eq!(SvgBackend::type_prefix(&primitive), expected);
    }

    // ========================================================================
    // Snapshot Tests for SVG Output Stability
    // ========================================================================
    //
    // These tests capture the exact SVG output for each primitive type.
    // Run `cargo insta review` to accept changes when intentionally modifying output.

    /// Helper to extract SVG content from inline render
    fn render_inline_svg(primitive: &Primitive) -> String {
        let backend = SvgBackend::new_inline();
        let RenderedAsset::InlineMarkdown(svg) = backend.render(primitive).unwrap() else {
            unreachable!("Expected InlineMarkdown");
        };
        svg
    }

    #[test]
    fn snapshot_swatch_flat_square() {
        let primitive = Primitive::simple_swatch("F41C80", "flat-square");
        assert_snapshot!("swatch_flat_square", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_swatch_pill() {
        let primitive = Primitive::simple_swatch("2B6CB0", "pill");
        assert_snapshot!("swatch_pill", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_swatch_circle() {
        let primitive = Primitive::simple_swatch("4CAF50", "circle");
        assert_snapshot!("swatch_circle", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_swatch_with_label() {
        let primitive = Primitive::Swatch {
            color: "FF5722".to_string(),
            style: "flat-square".to_string(),
            opacity: Some(1.0),
            width: Some(80),
            height: Some(20),
            border_color: None,
            border_width: Some(0),
            label: Some("Orange".to_string()),
            label_color: Some("FFFFFF".to_string()),
            icon: None,
            icon_color: None,
            rx: Some(3),
            ry: Some(3),
            shadow: None,
            gradient: None,
            stroke_dash: None,
            logo_size: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
        };
        assert_snapshot!("swatch_with_label", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_progress_bar() {
        let primitive = Primitive::simple_progress(65u8, "E0E0E0", "4CAF50");
        assert_snapshot!("progress_65_percent", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_donut() {
        let primitive = Primitive::simple_donut(80u8, "E0E0E0", "2196F3");
        assert_snapshot!("donut_80_percent", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_gauge() {
        let primitive = Primitive::simple_gauge(45u8, "E0E0E0", "FF9800");
        assert_snapshot!("gauge_45_percent", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_sparkline() {
        let primitive =
            Primitive::simple_sparkline(vec![10.0, 25.0, 45.0, 30.0, 55.0, 40.0, 60.0], "4CAF50");
        assert_snapshot!("sparkline_line", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_rating() {
        let primitive = Primitive::simple_rating(3.5, "FFD700");
        assert_snapshot!("rating_3_5_stars", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_waveform() {
        let primitive = Primitive::simple_waveform(
            vec![0.5, -0.3, 0.8, -0.6, 0.2, -0.4, 0.7],
            "4CAF50",
            "FF5722",
        );
        assert_snapshot!("waveform_audio", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_tech_badge() {
        let primitive = Primitive::Tech(TechConfig::new("rust"));
        assert_snapshot!("tech_rust", render_inline_svg(&primitive));
    }

    #[rstest]
    #[case(0u8, "progress_0")]
    #[case(50u8, "progress_50")]
    #[case(100u8, "progress_100")]
    fn snapshot_progress_extremes(#[case] percent: u8, #[case] name: &str) {
        let primitive = Primitive::simple_progress(percent, "E0E0E0", "4CAF50");
        assert_snapshot!(format!("{}_percent", name), render_inline_svg(&primitive));
    }

    // ========================================================================
    // Progress Bar - Extended Snapshot Tests
    // ========================================================================

    #[test]
    fn snapshot_progress_with_label() {
        let primitive = Primitive::Progress {
            percent: 75,
            width: 120,
            height: 20,
            track_color: "E0E0E0".to_string(),
            fill_color: "4CAF50".to_string(),
            fill_height: 20,
            rx: 5,
            show_label: true,
            label_color: Some("FFFFFF".to_string()),
            border_color: None,
            border_width: 0,
            thumb: None,
        };
        assert_snapshot!("progress_with_label", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_progress_label_too_small() {
        // Label should NOT render when dimensions are too small (< 50px wide or < 14px tall)
        let primitive = Primitive::Progress {
            percent: 50,
            width: 40,
            height: 10,
            track_color: "E0E0E0".to_string(),
            fill_color: "4CAF50".to_string(),
            fill_height: 10,
            rx: 3,
            show_label: true,
            label_color: Some("FFFFFF".to_string()),
            border_color: None,
            border_width: 0,
            thumb: None,
        };
        assert_snapshot!("progress_label_too_small", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_progress_centered_fill() {
        // Fill height < track height creates a centered "floating" fill
        let primitive = Primitive::Progress {
            percent: 60,
            width: 100,
            height: 16,
            track_color: "E0E0E0".to_string(),
            fill_color: "2196F3".to_string(),
            fill_height: 8,
            rx: 4,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb: None,
        };
        assert_snapshot!("progress_centered_fill", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_progress_with_border() {
        let primitive = Primitive::Progress {
            percent: 50,
            width: 100,
            height: 12,
            track_color: "F5F5F5".to_string(),
            fill_color: "FF5722".to_string(),
            fill_height: 12,
            rx: 6,
            show_label: false,
            label_color: None,
            border_color: Some("CCCCCC".to_string()),
            border_width: 2,
            thumb: None,
        };
        assert_snapshot!("progress_with_border", render_inline_svg(&primitive));
    }

    // ========================================================================
    // Slider Mode - Snapshot Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("circle", 50u8, 16, "4CAF50", "slider_circle")]
    #[case("square", 75u8, 14, "2196F3", "slider_square")]
    #[case("diamond", 25u8, 14, "FF9800", "slider_diamond")]
    fn snapshot_slider_thumb_shapes(
        #[case] shape: &str,
        #[case] percent: u8,
        #[case] size: u32,
        #[case] fill_color: &str,
        #[case] name: &str,
    ) {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Progress {
            percent,
            width: 120,
            height: 6,
            track_color: "E0E0E0".to_string(),
            fill_color: fill_color.to_string(),
            fill_height: 6,
            rx: 3,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb: Some(ThumbConfig {
                size,
                width: None,
                color: None,
                shape: shape.to_string(),
                border: None,
                border_width: 0,
            }),
        };
        assert_snapshot!(format!("{}_thumb", name), render_inline_svg(&primitive));
    }

    #[rstest]
    #[case(0u8, "slider_at_zero")]
    #[case(100u8, "slider_at_hundred")]
    fn snapshot_slider_extremes(#[case] percent: u8, #[case] name: &str) {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Progress {
            percent,
            width: 120,
            height: 6,
            track_color: "E0E0E0".to_string(),
            fill_color: "4CAF50".to_string(),
            fill_height: 6,
            rx: 3,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb: Some(ThumbConfig {
                size: 16,
                width: None,
                color: None,
                shape: "circle".to_string(),
                border: None,
                border_width: 0,
            }),
        };
        assert_snapshot!(name, render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_slider_with_thumb_border() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Progress {
            percent: 60,
            width: 120,
            height: 8,
            track_color: "E0E0E0".to_string(),
            fill_color: "9C27B0".to_string(),
            fill_height: 8,
            rx: 4,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb: Some(ThumbConfig {
                size: 18,
                width: None,
                color: Some("FFFFFF".to_string()),
                shape: "circle".to_string(),
                border: Some("9C27B0".to_string()),
                border_width: 2,
            }),
        };
        assert_snapshot!("slider_with_thumb_border", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_slider_custom_thumb_width() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Progress {
            percent: 40,
            width: 120,
            height: 6,
            track_color: "E0E0E0".to_string(),
            fill_color: "F44336".to_string(),
            fill_height: 6,
            rx: 3,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb: Some(ThumbConfig {
                size: 12,
                width: Some(20),
                color: None,
                shape: "circle".to_string(),
                border: None,
                border_width: 0,
            }),
        };
        assert_snapshot!("slider_custom_thumb_width", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_slider_with_track_border() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Progress {
            percent: 50,
            width: 120,
            height: 8,
            track_color: "F5F5F5".to_string(),
            fill_color: "3F51B5".to_string(),
            fill_height: 8,
            rx: 4,
            show_label: false,
            label_color: None,
            border_color: Some("BDBDBD".to_string()),
            border_width: 1,
            thumb: Some(ThumbConfig {
                size: 16,
                width: None,
                color: None,
                shape: "circle".to_string(),
                border: None,
                border_width: 0,
            }),
        };
        assert_snapshot!("slider_with_track_border", render_inline_svg(&primitive));
    }

    // ========================================================================
    // Sparkline - Extended Snapshot Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("line", "sparkline_line_chart")]
    #[case("bar", "sparkline_bar_chart")]
    #[case("area", "sparkline_area_chart")]
    fn snapshot_sparkline_chart_types(#[case] chart_type: &str, #[case] name: &str) {
        let primitive = Primitive::Sparkline {
            values: vec![10.0, 25.0, 45.0, 30.0, 55.0, 40.0, 60.0],
            width: 120,
            height: 30,
            chart_type: chart_type.to_string(),
            fill_color: "4CAF50".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        };
        assert_snapshot!(name, render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_sparkline_with_track() {
        let primitive = Primitive::Sparkline {
            values: vec![10.0, 25.0, 45.0, 30.0, 55.0],
            width: 100,
            height: 25,
            chart_type: "line".to_string(),
            fill_color: "2196F3".to_string(),
            stroke_color: Some("1976D2".to_string()),
            stroke_width: 2,
            track_color: Some("F5F5F5".to_string()),
            show_dots: false,
            dot_radius: 2,
        };
        assert_snapshot!("sparkline_with_track", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_sparkline_with_dots() {
        let primitive = Primitive::Sparkline {
            values: vec![15.0, 35.0, 25.0, 50.0, 40.0],
            width: 100,
            height: 30,
            chart_type: "line".to_string(),
            fill_color: "FF5722".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: true,
            dot_radius: 3,
        };
        assert_snapshot!("sparkline_with_dots", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_sparkline_area_with_dots() {
        let primitive = Primitive::Sparkline {
            values: vec![20.0, 40.0, 35.0, 55.0, 45.0, 60.0],
            width: 120,
            height: 35,
            chart_type: "area".to_string(),
            fill_color: "9C27B0".to_string(),
            stroke_color: Some("7B1FA2".to_string()),
            stroke_width: 2,
            track_color: None,
            show_dots: true,
            dot_radius: 2,
        };
        assert_snapshot!("sparkline_area_with_dots", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_sparkline_empty() {
        let primitive = Primitive::Sparkline {
            values: vec![],
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: "4CAF50".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        };
        assert_snapshot!("sparkline_empty", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_sparkline_single_value() {
        let primitive = Primitive::Sparkline {
            values: vec![42.0],
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: "4CAF50".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: true,
            dot_radius: 3,
        };
        assert_snapshot!("sparkline_single_value", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_sparkline_flat_data() {
        // All same values: tests the flat data normalization path
        let primitive = Primitive::Sparkline {
            values: vec![50.0, 50.0, 50.0, 50.0, 50.0],
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: "607D8B".to_string(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        };
        assert_snapshot!("sparkline_flat_data", render_inline_svg(&primitive));
    }

    // ========================================================================
    // Donut - Extended Snapshot Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0u8, "donut_0")]
    #[case(25u8, "donut_25")]
    #[case(50u8, "donut_50")]
    #[case(75u8, "donut_75")]
    #[case(100u8, "donut_100")]
    fn snapshot_donut_percentages(#[case] percent: u8, #[case] name: &str) {
        let primitive = Primitive::Donut {
            percent,
            size: 60,
            thickness: 6,
            track_color: "E0E0E0".to_string(),
            fill_color: "4CAF50".to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };
        assert_snapshot!(format!("{}_percent", name), render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_donut_with_label() {
        let primitive = Primitive::Donut {
            percent: 65,
            size: 80,
            thickness: 8,
            track_color: "E0E0E0".to_string(),
            fill_color: "2196F3".to_string(),
            show_label: true,
            label_color: Some("333333".to_string()),
            thumb: None,
        };
        assert_snapshot!("donut_with_label", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_donut_label_too_small() {
        // Label should NOT render when size < 30
        let primitive = Primitive::Donut {
            percent: 50,
            size: 25,
            thickness: 3,
            track_color: "E0E0E0".to_string(),
            fill_color: "4CAF50".to_string(),
            show_label: true,
            label_color: Some("333333".to_string()),
            thumb: None,
        };
        assert_snapshot!("donut_label_too_small", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_donut_with_thumb() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Donut {
            percent: 45,
            size: 70,
            thickness: 6,
            track_color: "E0E0E0".to_string(),
            fill_color: "FF5722".to_string(),
            show_label: false,
            label_color: None,
            thumb: Some(ThumbConfig {
                size: 12,
                width: None,
                color: None,
                shape: "circle".to_string(),
                border: None,
                border_width: 0,
            }),
        };
        assert_snapshot!("donut_with_thumb", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_donut_thumb_with_border() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Donut {
            percent: 70,
            size: 80,
            thickness: 8,
            track_color: "E0E0E0".to_string(),
            fill_color: "9C27B0".to_string(),
            show_label: false,
            label_color: None,
            thumb: Some(ThumbConfig {
                size: 14,
                width: None,
                color: Some("FFFFFF".to_string()),
                shape: "circle".to_string(),
                border: Some("9C27B0".to_string()),
                border_width: 2,
            }),
        };
        assert_snapshot!("donut_thumb_with_border", render_inline_svg(&primitive));
    }

    #[rstest]
    #[case(40, 4, "donut_small")]
    #[case(80, 8, "donut_medium")]
    #[case(120, 12, "donut_large")]
    fn snapshot_donut_sizes(#[case] size: u32, #[case] thickness: u32, #[case] name: &str) {
        let primitive = Primitive::Donut {
            percent: 60,
            size,
            thickness,
            track_color: "E0E0E0".to_string(),
            fill_color: "3F51B5".to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };
        assert_snapshot!(name, render_inline_svg(&primitive));
    }

    // ========================================================================
    // Gauge - Extended Snapshot Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0u8, "gauge_0")]
    #[case(25u8, "gauge_25")]
    #[case(50u8, "gauge_50")]
    #[case(75u8, "gauge_75")]
    #[case(100u8, "gauge_100")]
    fn snapshot_gauge_percentages(#[case] percent: u8, #[case] name: &str) {
        let primitive = Primitive::Gauge {
            percent,
            size: 100,
            thickness: 10,
            track_color: "E0E0E0".to_string(),
            fill_color: "4CAF50".to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };
        assert_snapshot!(format!("{}_percent", name), render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_gauge_with_label() {
        let primitive = Primitive::Gauge {
            percent: 55,
            size: 120,
            thickness: 12,
            track_color: "E0E0E0".to_string(),
            fill_color: "2196F3".to_string(),
            show_label: true,
            label_color: Some("333333".to_string()),
            thumb: None,
        };
        assert_snapshot!("gauge_with_label", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_gauge_with_thumb() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Gauge {
            percent: 40,
            size: 100,
            thickness: 10,
            track_color: "E0E0E0".to_string(),
            fill_color: "FF5722".to_string(),
            show_label: false,
            label_color: None,
            thumb: Some(ThumbConfig {
                size: 14,
                width: None,
                color: None,
                shape: "circle".to_string(),
                border: None,
                border_width: 0,
            }),
        };
        assert_snapshot!("gauge_with_thumb", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_gauge_thumb_with_border() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Gauge {
            percent: 65,
            size: 100,
            thickness: 10,
            track_color: "E0E0E0".to_string(),
            fill_color: "9C27B0".to_string(),
            show_label: false,
            label_color: None,
            thumb: Some(ThumbConfig {
                size: 16,
                width: None,
                color: Some("FFFFFF".to_string()),
                shape: "circle".to_string(),
                border: Some("9C27B0".to_string()),
                border_width: 2,
            }),
        };
        assert_snapshot!("gauge_thumb_with_border", render_inline_svg(&primitive));
    }

    #[test]
    fn snapshot_gauge_label_and_thumb() {
        use crate::primitive::ThumbConfig;
        let primitive = Primitive::Gauge {
            percent: 70,
            size: 120,
            thickness: 12,
            track_color: "E0E0E0".to_string(),
            fill_color: "3F51B5".to_string(),
            show_label: true,
            label_color: Some("333333".to_string()),
            thumb: Some(ThumbConfig {
                size: 14,
                width: None,
                color: None,
                shape: "circle".to_string(),
                border: None,
                border_width: 0,
            }),
        };
        assert_snapshot!("gauge_label_and_thumb", render_inline_svg(&primitive));
    }

    #[rstest]
    #[case(60, 6, "gauge_small")]
    #[case(100, 10, "gauge_medium")]
    #[case(140, 14, "gauge_large")]
    fn snapshot_gauge_sizes(#[case] size: u32, #[case] thickness: u32, #[case] name: &str) {
        let primitive = Primitive::Gauge {
            percent: 60,
            size,
            thickness,
            track_color: "E0E0E0".to_string(),
            fill_color: "F44336".to_string(),
            show_label: false,
            label_color: None,
            thumb: None,
        };
        assert_snapshot!(name, render_inline_svg(&primitive));
    }
}
