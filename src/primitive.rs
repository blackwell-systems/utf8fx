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
    /// Single colored swatch block
    Swatch { color: String, style: String },

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_swatch() {
        let swatch = Primitive::Swatch {
            color: "ff6b35".to_string(),
            style: "flat-square".to_string(),
        };
        assert_eq!(
            swatch,
            Primitive::Swatch {
                color: "ff6b35".to_string(),
                style: "flat-square".to_string(),
            }
        );
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
