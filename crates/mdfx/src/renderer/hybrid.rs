/// Hybrid backend that auto-selects between shields.io and SVG
///
/// Uses shields.io for simple swatches (works everywhere, no files needed)
/// Uses SVG only when advanced features are needed (gradients, shadows, corners)
use crate::error::Result;
use crate::primitive::Primitive;
use crate::renderer::shields::ShieldsBackend;
use crate::renderer::svg::SvgBackend;
use crate::renderer::{RenderedAsset, Renderer};

pub struct HybridBackend {
    shields: ShieldsBackend,
    svg: SvgBackend,
}

impl HybridBackend {
    pub fn new(assets_dir: impl Into<String>) -> Result<Self> {
        Ok(Self {
            shields: ShieldsBackend::new()?,
            svg: SvgBackend::new(assets_dir),
        })
    }

    /// Check if a primitive requires SVG-only features
    fn needs_svg(primitive: &Primitive) -> bool {
        match primitive {
            Primitive::Swatch {
                gradient,
                shadow,
                rx,
                ry,
                stroke_dash,
                border_top,
                border_right,
                border_bottom,
                border_left,
                ..
            } => {
                // Use SVG if any advanced feature is present
                gradient.is_some()
                    || shadow.is_some()
                    || rx.is_some()
                    || ry.is_some()
                    || stroke_dash.is_some()
                    || border_top.is_some()
                    || border_right.is_some()
                    || border_bottom.is_some()
                    || border_left.is_some()
            }
            // Tech badges use shields.io
            Primitive::Tech { .. } => false,
            // Progress bars always use SVG for proper rendering
            Primitive::Progress { .. } => true,
            // Donut charts always use SVG for proper rendering
            Primitive::Donut { .. } => true,
            // Gauge (half-donut) always uses SVG for proper rendering
            Primitive::Gauge { .. } => true,
            // Sparklines always use SVG for proper rendering
            Primitive::Sparkline { .. } => true,
        }
    }
}

impl Renderer for HybridBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        if Self::needs_svg(primitive) {
            self.svg.render(primitive)
        } else {
            self.shields.render(primitive)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_swatch_uses_shields() {
        let backend = HybridBackend::new("assets").unwrap();
        let primitive = Primitive::simple_swatch("FF0000", "flat-square");

        let result = backend.render(&primitive).unwrap();

        // Should be inline (shields.io URL)
        assert!(!result.is_file_based());
        assert!(result.to_markdown().contains("shields.io"));
    }

    #[test]
    fn test_gradient_swatch_uses_svg() {
        let backend = HybridBackend::new("assets").unwrap();
        let primitive = Primitive::Swatch {
            color: "000000".to_string(),
            style: "flat-square".to_string(),
            opacity: None,
            width: Some(100),
            height: Some(50),
            border_color: None,
            border_width: None,
            label: None,
            label_color: None,
            icon: None,
            icon_color: None,
            rx: None,
            ry: None,
            shadow: None,
            gradient: Some("horizontal/FF0000/0000FF".to_string()),
            stroke_dash: None,
            logo_size: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
        };

        let result = backend.render(&primitive).unwrap();

        // Should be file-based (SVG)
        assert!(result.is_file_based());
    }

    #[test]
    fn test_shadow_swatch_uses_svg() {
        let backend = HybridBackend::new("assets").unwrap();
        let primitive = Primitive::Swatch {
            color: "FF0000".to_string(),
            style: "flat-square".to_string(),
            opacity: None,
            width: Some(100),
            height: Some(50),
            border_color: None,
            border_width: None,
            label: None,
            label_color: None,
            icon: None,
            icon_color: None,
            rx: Some(10),
            ry: None,
            shadow: Some("000000/5/2/2".to_string()),
            gradient: None,
            stroke_dash: None,
            logo_size: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
        };

        let result = backend.render(&primitive).unwrap();

        // Should be file-based (SVG)
        assert!(result.is_file_based());
    }

    #[test]
    fn test_per_side_border_uses_svg() {
        let backend = HybridBackend::new("assets").unwrap();
        let primitive = Primitive::Swatch {
            color: "FF0000".to_string(),
            style: "flat-square".to_string(),
            opacity: None,
            width: Some(100),
            height: Some(50),
            border_color: None,
            border_width: None,
            label: None,
            label_color: None,
            icon: None,
            icon_color: None,
            rx: None,
            ry: None,
            shadow: None,
            gradient: None,
            stroke_dash: None,
            logo_size: None,
            border_top: Some("0000FF/3".to_string()),
            border_right: None,
            border_bottom: Some("00FF00/3".to_string()),
            border_left: None,
        };

        let result = backend.render(&primitive).unwrap();

        // Should be file-based (SVG) because per-side borders need SVG
        assert!(result.is_file_based());
    }
}
