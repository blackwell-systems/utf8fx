/// Plain text backend for maximum compatibility (PyPI, ASCII-only contexts).
///
/// This backend renders primitives as plain text representations without
/// external dependencies or Unicode decorations. Useful for PyPI package
/// descriptions and other environments with limited rendering support.
use crate::error::Result;
use crate::primitive::Primitive;
use crate::renderer::{RenderedAsset, Renderer};

/// Plain text rendering backend.
///
/// Renders primitives as ASCII-compatible text representations:
/// - Swatches: `[#RRGGBB]` color codes
/// - Dividers: ASCII line separators
/// - Tech badges: `[Technology]` text labels
/// - Status: `[OK]`, `[WARN]`, `[ERR]` indicators
#[derive(Debug, Clone, Default)]
pub struct PlainTextBackend;

impl PlainTextBackend {
    /// Create a new plain text backend
    pub fn new() -> Self {
        PlainTextBackend
    }
}

impl Renderer for PlainTextBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        let text = match primitive {
            Primitive::Swatch {
                color, label, icon, ..
            } => {
                // Icon takes precedence over label
                if let Some(icon_name) = icon {
                    format!("[#{} {}]", color, icon_name)
                } else if let Some(lbl) = label {
                    format!("[#{} {}]", color, lbl)
                } else {
                    format!("[#{}]", color)
                }
            }

            Primitive::Divider { colors, .. } => {
                if colors.is_empty() {
                    "---".to_string()
                } else {
                    // Show colors in divider
                    let color_str = colors
                        .iter()
                        .map(|c| format!("#{}", c))
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("--- {} ---", color_str)
                }
            }

            Primitive::Tech { name, .. } => {
                format!("[{}]", name)
            }

            Primitive::Status { level, .. } => {
                let indicator = match level.to_lowercase().as_str() {
                    // Semantic names
                    "success" | "ok" | "pass" | "green" => "[OK]",
                    "warning" | "warn" | "yellow" => "[WARN]",
                    "error" | "err" | "fail" | "red" => "[ERR]",
                    "info" | "blue" => "[INFO]",
                    // Hex colors from palette resolution
                    "22c55e" => "[OK]",   // success green
                    "eab308" => "[WARN]", // warning yellow
                    "ef4444" => "[ERR]",  // error red
                    "3b82f6" => "[INFO]", // info blue
                    _ => "[?]",
                };
                indicator.to_string()
            }
        };

        Ok(RenderedAsset::InlineMarkdown(text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plaintext_swatch() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::simple_swatch("F41C80", "flat-square");
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[#F41C80]");
    }

    #[test]
    fn test_plaintext_swatch_with_label() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Swatch {
            color: "FF6B35".to_string(),
            style: "flat-square".to_string(),
            opacity: None,
            width: None,
            height: None,
            border_color: None,
            border_width: None,
            label: Some("v1.0".to_string()),
            label_color: None,
            icon: None,
            icon_color: None,
            rx: None,
            ry: None,
            shadow: None,
            gradient: None,
            stroke_dash: None,
            logo_size: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[#FF6B35 v1.0]");
    }

    #[test]
    fn test_plaintext_swatch_with_icon() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat-square".to_string(),
            opacity: None,
            width: None,
            height: None,
            border_color: None,
            border_width: None,
            label: None,
            label_color: None,
            icon: Some("rust".to_string()),
            icon_color: None,
            rx: None,
            ry: None,
            shadow: None,
            gradient: None,
            stroke_dash: None,
            logo_size: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[#F41C80 rust]");
    }

    #[test]
    fn test_plaintext_divider() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Divider {
            colors: vec!["F41C80".to_string(), "2B6CB0".to_string()],
            style: "flat-square".to_string(),
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "--- #F41C80 #2B6CB0 ---");
    }

    #[test]
    fn test_plaintext_divider_empty() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Divider {
            colors: vec![],
            style: "flat-square".to_string(),
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "---");
    }

    #[test]
    fn test_plaintext_tech() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "DEA584".to_string(),
            logo_color: "000000".to_string(),
            style: "flat-square".to_string(),
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[rust]");
    }

    #[test]
    fn test_plaintext_status_success() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Status {
            level: "success".to_string(),
            style: "flat-square".to_string(),
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[OK]");
    }

    #[test]
    fn test_plaintext_status_warning() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Status {
            level: "warning".to_string(),
            style: "flat-square".to_string(),
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[WARN]");
    }

    #[test]
    fn test_plaintext_status_error() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Status {
            level: "error".to_string(),
            style: "flat-square".to_string(),
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[ERR]");
    }

    #[test]
    fn test_plaintext_status_hex_color() {
        let backend = PlainTextBackend::new();
        // Components resolve "success" to hex color "22C55E"
        let primitive = Primitive::Status {
            level: "22C55E".to_string(),
            style: "flat-square".to_string(),
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[OK]");
    }
}
