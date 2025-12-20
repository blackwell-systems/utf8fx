//! Color picker support for the mdfx LSP
//!
//! Provides color information and presentation for hex colors in templates.

use tower_lsp::lsp_types::*;

/// Parse a 6-character hex string to a Color
pub fn parse_hex_color(hex: &str) -> Option<Color> {
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color {
        red: r as f32 / 255.0,
        green: g as f32 / 255.0,
        blue: b as f32 / 255.0,
        alpha: 1.0,
    })
}

/// Find all color information in document text
pub fn find_document_colors(text: &str) -> Vec<ColorInformation> {
    let mut colors = Vec::new();

    for (line_num, line) in text.lines().enumerate() {
        // Find hex color patterns: bg=RRGGBB, text=RRGGBB, border=RRGGBB, etc.
        // Also standalone hex colors in color contexts
        let mut pos = 0;
        while pos < line.len() {
            // Look for =RRGGBB or =RGB patterns (6 or 3 hex chars)
            if let Some(eq_pos) = line[pos..].find('=') {
                let start = pos + eq_pos + 1;
                if start < line.len() {
                    // Try to parse as hex color (6 chars)
                    if start + 6 <= line.len() {
                        let hex = &line[start..start + 6];
                        if hex.chars().all(|c| c.is_ascii_hexdigit()) {
                            // Check it's followed by : or / or } or end
                            let after = line.chars().nth(start + 6);
                            if after.is_none()
                                || matches!(after, Some(':') | Some('/') | Some('}'))
                            {
                                if let Some(color) = parse_hex_color(hex) {
                                    colors.push(ColorInformation {
                                        range: Range {
                                            start: Position {
                                                line: line_num as u32,
                                                character: start as u32,
                                            },
                                            end: Position {
                                                line: line_num as u32,
                                                character: (start + 6) as u32,
                                            },
                                        },
                                        color,
                                    });
                                }
                            }
                        }
                    }
                    // Try 3-char hex (RGB shorthand)
                    else if start + 3 <= line.len() {
                        let hex = &line[start..start + 3];
                        if hex.chars().all(|c| c.is_ascii_hexdigit()) {
                            let after = line.chars().nth(start + 3);
                            if after.is_none()
                                || matches!(after, Some(':') | Some('/') | Some('}'))
                            {
                                // Expand 3-char to 6-char: RGB -> RRGGBB
                                let expanded: String = hex.chars().flat_map(|c| [c, c]).collect();
                                if let Some(color) = parse_hex_color(&expanded) {
                                    colors.push(ColorInformation {
                                        range: Range {
                                            start: Position {
                                                line: line_num as u32,
                                                character: start as u32,
                                            },
                                            end: Position {
                                                line: line_num as u32,
                                                character: (start + 3) as u32,
                                            },
                                        },
                                        color,
                                    });
                                }
                            }
                        }
                    }
                }
                pos = pos + eq_pos + 1;
            } else {
                break;
            }
        }
    }

    colors
}

/// Convert a color to hex representation for presentation
pub fn color_to_hex(color: &Color) -> String {
    let r = (color.red * 255.0_f32) as u8;
    let g = (color.green * 255.0_f32) as u8;
    let b = (color.blue * 255.0_f32) as u8;

    format!("{:02X}{:02X}{:02X}", r, g, b)
}

/// Create color presentation for the color picker
pub fn create_color_presentation(color: &Color, range: Range) -> Vec<ColorPresentation> {
    let hex = color_to_hex(color);

    vec![ColorPresentation {
        label: hex.clone(),
        text_edit: Some(TextEdit {
            range,
            new_text: hex,
        }),
        additional_text_edits: None,
    }]
}
