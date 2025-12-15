/// Rendering-neutral primitives for image-based visual elements.
///
/// These primitives represent the SEMANTIC intent of UI components
/// without committing to a specific rendering backend (shields.io, SVG, etc.).
///
/// Each primitive corresponds to a high-level UI concept:
/// - Swatch: Single colored block
/// - Divider: Multi-color separator bar
/// - Tech: Technology logo badge
/// - Status: Colored status indicator
///
/// Text-based transformations (frames, styles, badges) remain as direct
/// Unicode rendering and don't use this abstraction.

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// Single colored swatch block with optional enhancements
    Swatch {
        color: String,
        style: String,
        /// Opacity (0.0 = transparent, 1.0 = opaque). SVG-only.
        opacity: Option<f32>,
        /// Custom width in pixels (default: 20)
        width: Option<u32>,
        /// Custom height in pixels (default: style-dependent, usually 20)
        height: Option<u32>,
        /// Border color (hex or palette name). SVG-only.
        border_color: Option<String>,
        /// Border width in pixels (default: 0 = no border). SVG-only.
        border_width: Option<u32>,
        /// Text label inside swatch.
        label: Option<String>,
        /// Label text color (hex or palette name). Default: white.
        label_color: Option<String>,
        /// Simple Icons logo name (e.g., "rust", "python", "docker").
        icon: Option<String>,
        /// Icon color (hex or palette name). Default: white.
        icon_color: Option<String>,
        /// Horizontal corner radius in pixels. SVG-only.
        rx: Option<u32>,
        /// Vertical corner radius in pixels (defaults to rx if not set). SVG-only.
        ry: Option<u32>,
        /// Drop shadow: "color:blur:offset_x:offset_y" (e.g., "000000:4:2:2"). SVG-only.
        shadow: Option<String>,
        /// Gradient fill: "direction:color1:color2" (e.g., "horizontal:FF0000:0000FF"). SVG-only.
        gradient: Option<String>,
        /// Border dash pattern: "dash:gap" (e.g., "4:2" for dashed). SVG-only.
        stroke_dash: Option<String>,
        /// Logo size for shields.io ("auto" for adaptive). Shields-only.
        logo_size: Option<String>,
        /// Top border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_top: Option<String>,
        /// Right border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_right: Option<String>,
        /// Bottom border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_bottom: Option<String>,
        /// Left border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_left: Option<String>,
    },

    /// Multi-color divider bar for section separation
    Divider { colors: Vec<String>, style: String },

    /// Technology badge with logo (uses Simple Icons)
    Tech {
        name: String,
        bg_color: String,
        logo_color: String,
        style: String,
    },

    /// Status indicator (success, warning, error, info)
    Status { level: String, style: String },
}

impl Primitive {
    /// Get the default shield style
    pub fn default_style() -> &'static str {
        "flat-square"
    }

    /// Create a simple swatch with just color and style (all other options None)
    pub fn simple_swatch(color: impl Into<String>, style: impl Into<String>) -> Self {
        Primitive::Swatch {
            color: color.into(),
            style: style.into(),
            opacity: None,
            width: None,
            height: None,
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
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_swatch() {
        let swatch = Primitive::simple_swatch("ff6b35", "flat-square");
        if let Primitive::Swatch { color, style, .. } = swatch {
            assert_eq!(color, "ff6b35");
            assert_eq!(style, "flat-square");
        } else {
            panic!("Expected Swatch primitive");
        }
    }

    #[test]
    fn test_primitive_swatch_with_options() {
        let swatch = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat".to_string(),
            opacity: Some(0.5),
            width: Some(40),
            height: Some(30),
            border_color: Some("FFFFFF".to_string()),
            border_width: Some(2),
            label: Some("v1".to_string()),
            label_color: Some("000000".to_string()),
            icon: Some("rust".to_string()),
            icon_color: Some("FFFFFF".to_string()),
            rx: Some(5),
            ry: Some(10),
            shadow: Some("000000:4:2:2".to_string()),
            gradient: Some("horizontal:FF0000:0000FF".to_string()),
            stroke_dash: Some("4:2".to_string()),
            logo_size: Some("auto".to_string()),
            border_top: Some("FF0000/2".to_string()),
            border_right: None,
            border_bottom: Some("0000FF/3".to_string()),
            border_left: None,
        };
        if let Primitive::Swatch {
            opacity,
            width,
            label,
            rx,
            shadow,
            gradient,
            stroke_dash,
            logo_size,
            ..
        } = swatch
        {
            assert_eq!(opacity, Some(0.5));
            assert_eq!(width, Some(40));
            assert_eq!(label, Some("v1".to_string()));
            assert_eq!(rx, Some(5));
            assert_eq!(shadow, Some("000000:4:2:2".to_string()));
            assert_eq!(gradient, Some("horizontal:FF0000:0000FF".to_string()));
            assert_eq!(stroke_dash, Some("4:2".to_string()));
            assert_eq!(logo_size, Some("auto".to_string()));
        } else {
            panic!("Expected Swatch primitive");
        }
    }

    #[test]
    fn test_primitive_divider() {
        let divider = Primitive::Divider {
            colors: vec![
                "ff0000".to_string(),
                "00ff00".to_string(),
                "0000ff".to_string(),
            ],
            style: "flat".to_string(),
        };

        if let Primitive::Divider { colors, .. } = divider {
            assert_eq!(colors.len(), 3);
        } else {
            panic!("Expected Divider primitive");
        }
    }

    #[test]
    fn test_primitive_tech() {
        let tech = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "ffffff".to_string(),
            style: "flat-square".to_string(),
        };

        if let Primitive::Tech { name, .. } = tech {
            assert_eq!(name, "rust");
        } else {
            panic!("Expected Tech primitive");
        }
    }

    #[test]
    fn test_primitive_status() {
        let status = Primitive::Status {
            level: "success".to_string(),
            style: "flat-square".to_string(),
        };

        if let Primitive::Status { level, .. } = status {
            assert_eq!(level, "success");
        } else {
            panic!("Expected Status primitive");
        }
    }
}
