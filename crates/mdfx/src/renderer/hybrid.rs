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
            Primitive::Tech(_) => false,
            // Version badges use local SVG (via badgefx)
            Primitive::Version(_) => true,
            // License badges use local SVG (via badgefx)
            Primitive::License(_) => true,
            // Progress bars always use SVG for proper rendering
            Primitive::Progress { .. } => true,
            // Donut charts always use SVG for proper rendering
            Primitive::Donut { .. } => true,
            // Gauge (half-donut) always uses SVG for proper rendering
            Primitive::Gauge { .. } => true,
            // Sparklines always use SVG for proper rendering
            Primitive::Sparkline { .. } => true,
            // Ratings always use SVG for proper rendering
            Primitive::Rating { .. } => true,
            // Waveforms always use SVG for proper rendering
            Primitive::Waveform { .. } => true,
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
    use rstest::rstest;

    // Helper to create a swatch with optional advanced features
    fn swatch_with_features(
        gradient: Option<&str>,
        shadow: Option<&str>,
        rx: Option<u32>,
        stroke_dash: Option<&str>,
        border_top: Option<&str>,
    ) -> Primitive {
        Primitive::Swatch {
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
            rx,
            ry: None,
            shadow: shadow.map(String::from),
            gradient: gradient.map(String::from),
            stroke_dash: stroke_dash.map(String::from),
            logo_size: None,
            border_top: border_top.map(String::from),
            border_right: None,
            border_bottom: None,
            border_left: None,
        }
    }

    #[test]
    fn test_simple_swatch_uses_shields() {
        let backend = HybridBackend::new("assets").unwrap();
        let primitive = Primitive::simple_swatch("FF0000", "flat-square");

        let result = backend.render(&primitive).unwrap();

        // Should be inline (shields.io URL)
        assert!(!result.is_file_based());
        assert!(result.to_markdown().contains("shields.io"));
    }

    // ========================================================================
    // SVG Feature Detection (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(Some("horizontal/FF0000/0000FF"), None, None, None, None, true)] // gradient
    #[case(None, Some("000000/5/2/2"), Some(10), None, None, true)] // shadow + rx
    #[case(None, None, Some(10), None, None, true)] // rx alone
    #[case(None, None, None, Some("5,5"), None, true)] // stroke dash
    #[case(None, None, None, None, Some("0000FF/3"), true)] // border_top
    #[case(None, None, None, None, None, false)] // none (shields)
    fn test_svg_feature_detection(
        #[case] gradient: Option<&str>,
        #[case] shadow: Option<&str>,
        #[case] rx: Option<u32>,
        #[case] stroke_dash: Option<&str>,
        #[case] border_top: Option<&str>,
        #[case] expects_svg: bool,
    ) {
        let backend = HybridBackend::new("assets").unwrap();
        let primitive = swatch_with_features(gradient, shadow, rx, stroke_dash, border_top);

        let result = backend.render(&primitive).unwrap();

        assert_eq!(result.is_file_based(), expects_svg);
    }
}
