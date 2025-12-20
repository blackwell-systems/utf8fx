/// Shields.io backend for rendering primitives as badge URLs
///
/// This backend generates shields.io badge URLs wrapped in Markdown image syntax.
/// It's the default rendering backend for mdfx.
use crate::error::Result;
use crate::primitive::Primitive;
use crate::renderer::{RenderedAsset, Renderer};
use crate::shields::ShieldsRenderer;

/// Shields.io rendering backend (default)
pub struct ShieldsBackend {
    shields: ShieldsRenderer,
}

impl ShieldsBackend {
    /// Create a new shields backend
    pub fn new() -> Result<Self> {
        Ok(ShieldsBackend {
            shields: ShieldsRenderer::new()?,
        })
    }
}

impl Renderer for ShieldsBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        let markdown = match primitive {
            Primitive::Swatch {
                color,
                style,
                icon,
                icon_color,
                label,
                logo_size,
                ..
            } => {
                // Handle different combinations of icon and label
                match (icon, label) {
                    // Both icon and label - render badge with icon and text
                    (Some(icon_name), Some(label_text)) => {
                        let logo_color = icon_color.as_deref().unwrap_or("FFFFFF");
                        self.shields.render_icon_with_label(
                            icon_name, label_text, color, logo_color, style,
                        )?
                    }
                    // Icon only - render icon chip
                    (Some(icon_name), None) => {
                        let logo_color = icon_color.as_deref().unwrap_or("FFFFFF");
                        self.shields.render_icon_with_size(
                            icon_name,
                            color,
                            logo_color,
                            style,
                            logo_size.as_deref(),
                        )?
                    }
                    // Label only - render labeled block
                    (None, Some(label_text)) => self
                        .shields
                        .render_labeled_block(color, label_text, style)?,
                    // Neither - render plain color block
                    (None, None) => self.shields.render_block(color, style)?,
                }
            }

            Primitive::Tech(cfg) => {
                self.shields
                    .render_icon(&cfg.name, &cfg.bg_color, &cfg.logo_color, &cfg.style)?
            }

            // Version badges - render as simple version label
            // Uses badgefx status detection for color if not overridden
            Primitive::Version(cfg) => {
                let status = badgefx::version::detect_status(&cfg.version);
                let bg_color = cfg.bg_color.as_deref().unwrap_or_else(|| status.bg_color());
                let prefix = cfg.prefix.as_deref().unwrap_or("v");
                let label = if cfg.version.starts_with('v')
                    || cfg.version.starts_with('V')
                    || prefix.is_empty()
                {
                    cfg.version.clone()
                } else {
                    format!("{}{}", prefix, cfg.version)
                };
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style={})",
                    label.replace('-', "--"),
                    label.replace('-', "--"),
                    bg_color,
                    cfg.style
                )
            }

            // License badges - render as license label
            // Uses badgefx category detection for color if not overridden
            Primitive::License(cfg) => {
                let category = badgefx::license::categorize(&cfg.license);
                let bg_color = cfg
                    .bg_color
                    .as_deref()
                    .unwrap_or_else(|| category.bg_color());
                let label = cfg
                    .label
                    .clone()
                    .unwrap_or_else(|| badgefx::license::format_name(&cfg.license));
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style={})",
                    label.replace('-', "--").replace(' ', "%20"),
                    label.replace('-', "--").replace(' ', "%20"),
                    bg_color,
                    cfg.style
                )
            }

            // Progress bars use a simple percentage badge as shields.io fallback
            // Full progress bar rendering requires SVG backend
            Primitive::Progress {
                percent,
                fill_color,
                ..
            } => {
                let label = format!("{}%25", percent); // URL-encoded %
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style=flat-square)",
                    label, label, fill_color
                )
            }

            // Donut charts use a circular percentage badge as shields.io fallback
            // Full donut rendering requires SVG backend
            Primitive::Donut {
                percent,
                fill_color,
                ..
            } => {
                let label = format!("{}%25", percent); // URL-encoded %
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style=flat-square)",
                    label, label, fill_color
                )
            }

            // Gauge uses a percentage badge as shields.io fallback
            // Full gauge (semi-circle) rendering requires SVG backend
            Primitive::Gauge {
                percent,
                fill_color,
                ..
            } => {
                let label = format!("{}%25", percent); // URL-encoded %
                format!(
                    "![](https://img.shields.io/badge/{}-{}-{}?style=flat-square)",
                    label, label, fill_color
                )
            }

            // Sparkline uses a chart indicator as shields.io fallback
            // Full sparkline rendering requires SVG backend
            Primitive::Sparkline {
                values, fill_color, ..
            } => {
                // Create a simple text representation of the data range
                let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
                let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                let label = if values.is_empty() {
                    "chart".to_string()
                } else {
                    format!("{:.0}..{:.0}", min, max)
                };
                format!(
                    "![](https://img.shields.io/badge/📈-{}-{}?style=flat-square)",
                    label.replace(' ', "%20"),
                    fill_color
                )
            }

            // Rating uses a star badge as shields.io fallback
            // Full rating rendering requires SVG backend
            Primitive::Rating {
                value,
                max,
                fill_color,
                ..
            } => {
                let label = format!("{:.1}/{}", value, max);
                format!(
                    "![](https://img.shields.io/badge/⭐-{}-{}?style=flat-square)",
                    label.replace('.', "%2E"),
                    fill_color
                )
            }

            // Waveform uses audio emoji badge as shields.io fallback
            // Full waveform rendering requires SVG backend
            Primitive::Waveform {
                values,
                positive_color,
                ..
            } => {
                let label = format!("{}pts", values.len());
                format!(
                    "![](https://img.shields.io/badge/🎵-{}-{}?style=flat-square)",
                    label, positive_color
                )
            }
        };

        Ok(RenderedAsset::InlineMarkdown(markdown))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shields_backend_creation() {
        let backend = ShieldsBackend::new();
        assert!(backend.is_ok());
    }

    #[test]
    fn test_render_swatch_primitive() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::simple_swatch("2B6CB0", "flat-square");

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("https://img.shields.io/badge/"));
        assert!(markdown.contains("2B6CB0"));
        assert!(markdown.contains("style=flat-square"));
        assert!(!result.is_file_based());
    }

    #[test]
    fn test_render_tech_primitive() {
        use crate::primitive::TechConfig;
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Tech(TechConfig {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "FFFFFF".to_string(),
            ..Default::default()
        });

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("logo=rust"));
        assert!(markdown.contains("logoColor=FFFFFF"));
        assert!(markdown.contains("000000"));
    }

    #[test]
    fn test_render_with_palette_colors() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::simple_swatch("cobalt", "flat-square");

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        // cobalt should resolve to 2B6CB0
        assert!(markdown.contains("2B6CB0"));
    }
}
