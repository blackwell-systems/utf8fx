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
        /// Text label inside swatch. SVG-only.
        label: Option<String>,
        /// Label text color (hex or palette name). Default: white. SVG-only.
        label_color: Option<String>,
        /// Simple Icons logo name (e.g., "rust", "python", "docker").
        icon: Option<String>,
        /// Icon color (hex or palette name). Default: white.
        icon_color: Option<String>,
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_swatch() {
        let swatch = Primitive::Swatch {
            color: "ff6b35".to_string(),
            style: "flat-square".to_string(),
            opacity: None,
            width: None,
            height: None,
            border_color: None,
            border_width: None,
            label: None,
            label_color: None,
            icon: None,
            icon_color: None,
        };
        assert_eq!(
            swatch,
            Primitive::Swatch {
                color: "ff6b35".to_string(),
                style: "flat-square".to_string(),
                opacity: None,
                width: None,
                height: None,
                border_color: None,
                border_width: None,
                label: None,
                label_color: None,
                icon: None,
                icon_color: None,
            }
        );
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
        };
        if let Primitive::Swatch {
            opacity,
            width,
            label,
            label_color,
            icon,
            icon_color,
            ..
        } = swatch
        {
            assert_eq!(opacity, Some(0.5));
            assert_eq!(width, Some(40));
            assert_eq!(label, Some("v1".to_string()));
            assert_eq!(label_color, Some("000000".to_string()));
            assert_eq!(icon, Some("rust".to_string()));
            assert_eq!(icon_color, Some("FFFFFF".to_string()));
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
