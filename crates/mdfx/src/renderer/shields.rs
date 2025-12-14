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
                ..
            } => {
                // If icon is specified, render as icon badge with custom background
                if let Some(icon_name) = icon {
                    let logo_color = icon_color.as_deref().unwrap_or("FFFFFF");
                    self.shields.render_icon(icon_name, color, logo_color, style)?
                } else if let Some(label_text) = label {
                    // Render block with label text
                    self.shields.render_labeled_block(color, label_text, style)?
                } else {
                    self.shields.render_block(color, style)?
                }
            }

            Primitive::Divider { colors, style } => self.shields.render_bar(colors, style)?,

            Primitive::Tech {
                name,
                bg_color,
                logo_color,
                style,
            } => self
                .shields
                .render_icon(name, bg_color, logo_color, style)?,

            Primitive::Status { level, style } => {
                // Status uses the level as the color (e.g., "success" → green)
                self.shields.render_block(level, style)?
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
    fn test_render_divider_primitive() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Divider {
            colors: vec![
                "22C55E".to_string(),
                "F59E0B".to_string(),
                "334155".to_string(),
            ],
            style: "flat-square".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("22C55E"));
        assert!(markdown.contains("F59E0B"));
        assert!(markdown.contains("334155"));
        assert_eq!(markdown.matches("![](").count(), 3);
    }

    #[test]
    fn test_render_tech_primitive() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "FFFFFF".to_string(),
            style: "flat-square".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("logo=rust"));
        assert!(markdown.contains("logoColor=FFFFFF"));
        assert!(markdown.contains("000000"));
    }

    #[test]
    fn test_render_status_primitive() {
        let backend = ShieldsBackend::new().unwrap();
        let primitive = Primitive::Status {
            level: "success".to_string(),
            style: "flat-square".to_string(),
        };

        let result = backend.render(&primitive).unwrap();
        let markdown = result.to_markdown();

        assert!(markdown.contains("https://img.shields.io/badge/"));
        // success should resolve to green color (22C55E from palette)
        assert!(markdown.contains("22C55E"));
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
