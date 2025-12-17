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
/// - Tech badges: `[Technology]` text labels
/// - Progress: `[=====>    ] 50%` ASCII bars
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

            Primitive::Tech { name, label, .. } => {
                if let Some(lbl) = label {
                    format!("[{} | {}]", name, lbl)
                } else {
                    format!("[{}]", name)
                }
            }

            Primitive::Progress { percent, .. } => {
                // Render as ASCII progress bar: [=====>    ] 50%
                let width = 10;
                let filled = (*percent as usize * width / 100).min(width);
                let empty = width - filled;
                let bar: String = "=".repeat(filled.saturating_sub(1))
                    + if filled > 0 { ">" } else { "" }
                    + &" ".repeat(empty);
                format!("[{}] {}%", bar, percent)
            }

            Primitive::Donut { percent, .. } => {
                // Render as ASCII donut: (75%)
                format!("({}%)", percent)
            }

            Primitive::Gauge { percent, .. } => {
                // Render as ASCII gauge: [75%]
                format!("[{}%]", percent)
            }

            Primitive::Sparkline { values, .. } => {
                // Render as ASCII sparkline using braille-like characters
                // ▁▂▃▄▅▆▇█
                if values.is_empty() {
                    return Ok(RenderedAsset::InlineMarkdown("▁".to_string()));
                }
                let min = values.iter().cloned().fold(f32::INFINITY, f32::min);
                let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                let range = if (max - min).abs() < 0.001 {
                    1.0
                } else {
                    max - min
                };
                let bars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let spark: String = values
                    .iter()
                    .map(|&v| {
                        let normalized = (v - min) / range;
                        let idx = ((normalized * 7.0).round() as usize).min(7);
                        bars[idx]
                    })
                    .collect();
                spark
            }

            Primitive::Rating {
                value, max, icon, ..
            } => {
                // Render as Unicode stars/hearts/circles
                let (filled_char, empty_char) = match icon.as_str() {
                    "heart" => ('♥', '♡'),
                    "circle" => ('●', '○'),
                    _ => ('★', '☆'), // star (default)
                };

                let filled = value.floor() as u32;
                let has_half = (value - value.floor()) >= 0.5;
                let empty = max
                    .saturating_sub(filled)
                    .saturating_sub(if has_half { 1 } else { 0 });

                let mut result = String::new();
                for _ in 0..filled.min(*max) {
                    result.push(filled_char);
                }
                if has_half && filled < *max {
                    // Use a half character or just show empty for simplicity
                    result.push(empty_char);
                }
                for _ in 0..empty {
                    result.push(empty_char);
                }
                result
            }

            Primitive::Waveform { values, .. } => {
                // Render as Unicode bar characters based on value magnitude
                // Use block characters: ▁▂▃▄▅▆▇█ for positive, ▔ for negative center
                if values.is_empty() {
                    return Ok(RenderedAsset::InlineMarkdown("▔".to_string()));
                }
                let max_abs = values
                    .iter()
                    .map(|v| v.abs())
                    .fold(0.0f32, f32::max)
                    .max(0.001);
                let bars_pos = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
                let wave: String = values
                    .iter()
                    .map(|&v| {
                        let normalized = (v / max_abs).abs();
                        let idx = ((normalized * 7.0).round() as usize).min(7);
                        if v >= 0.0 {
                            bars_pos[idx]
                        } else {
                            // For negative, use same bars but could differentiate
                            bars_pos[idx]
                        }
                    })
                    .collect();
                wave
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
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
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
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[#F41C80 rust]");
    }

    #[test]
    fn test_plaintext_tech() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "DEA584".to_string(),
            logo_color: "000000".to_string(),
            style: "flat-square".to_string(),
            label: None,
            border_color: None,
            border_width: None,
            rx: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[rust]");
    }

    #[test]
    fn test_plaintext_tech_with_label() {
        let backend = PlainTextBackend::new();
        let primitive = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "DEA584".to_string(),
            logo_color: "000000".to_string(),
            style: "flat-square".to_string(),
            label: Some("v1.80".to_string()),
            border_color: None,
            border_width: None,
            rx: None,
        };
        let asset = backend.render(&primitive).unwrap();
        assert_eq!(asset.to_markdown(), "[rust | v1.80]");
    }
}
