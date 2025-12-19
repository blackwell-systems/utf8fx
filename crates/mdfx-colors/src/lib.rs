//! Color utilities for badge generation
//!
//! This crate provides lightweight color manipulation functions focused on
//! badge generation use cases: luminance calculation for contrast detection,
//! color darkening, and hex color parsing.

/// Calculate the relative luminance of a hex color using ITU-R BT.709 coefficients
///
/// Returns a value between 0.0 (black) and 1.0 (white) indicating the perceived
/// brightness of the color. Used for determining appropriate contrast colors.
///
/// # Examples
///
/// ```
/// use mdfx_colors::luminance;
///
/// assert!(luminance("#FFFFFF") > 0.9); // White is very bright
/// assert!(luminance("#000000") < 0.1); // Black is very dark
/// assert!(luminance("#808080") > 0.45 && luminance("#808080") < 0.55); // Gray is medium
/// ```
pub fn luminance(hex: &str) -> f32 {
    let (r, g, b) = parse_hex(hex).unwrap_or((0, 0, 0));

    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    // ITU-R BT.709 coefficients for relative luminance
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Get the ideal contrast color (white or black) for text on the given background
///
/// Uses relative luminance calculation to determine whether white or black text
/// would provide better contrast against the background color.
///
/// # Examples
///
/// ```
/// use mdfx_colors::contrast_color;
///
/// assert_eq!(contrast_color("#FFFFFF"), "#000000"); // Black on white
/// assert_eq!(contrast_color("#000000"), "#FFFFFF"); // White on black  
/// assert_eq!(contrast_color("#3178C6"), "#FFFFFF"); // White on blue
/// ```
pub fn contrast_color(bg: &str) -> &'static str {
    let lum = luminance(bg);

    // Use black text on light backgrounds, white on dark
    if lum > 0.5 {
        "#000000"
    } else {
        "#FFFFFF"
    }
}

/// Darken a hex color by the specified amount
///
/// Reduces the brightness of each RGB component by the given percentage.
/// The amount should be between 0.0 (no change) and 1.0 (complete darkening).
///
/// # Examples
///
/// ```
/// use mdfx_colors::darken;
///
/// assert_eq!(darken("#FFFFFF", 0.5), "#808080"); // 50% gray (with rounding)
/// assert_eq!(darken("#FF0000", 0.2), "#CC0000"); // 20% darker red
/// ```
pub fn darken(hex: &str, amount: f32) -> String {
    let (r, g, b) = parse_hex(hex).unwrap_or((255, 255, 255));

    let factor = 1.0 - amount.clamp(0.0, 1.0);

    let new_r = ((r as f32) * factor).round() as u8;
    let new_g = ((g as f32) * factor).round() as u8;
    let new_b = ((b as f32) * factor).round() as u8;

    format!("#{:02X}{:02X}{:02X}", new_r, new_g, new_b)
}

/// Parse a hex color string into RGB components
///
/// Accepts hex colors with or without the # prefix, in 3-digit or 6-digit format.
/// Returns None for invalid input.
///
/// # Examples
///
/// ```
/// use mdfx_colors::parse_hex;
///
/// assert_eq!(parse_hex("#FF0000"), Some((255, 0, 0)));
/// assert_eq!(parse_hex("00FF00"), Some((0, 255, 0)));
/// assert_eq!(parse_hex("#F0A"), Some((255, 0, 170))); // 3-digit expanded
/// assert_eq!(parse_hex("invalid"), None);
/// ```
pub fn parse_hex(s: &str) -> Option<(u8, u8, u8)> {
    let hex = s.trim_start_matches('#');

    match hex.len() {
        // 3-digit hex: #F0A -> #FF00AA
        3 => {
            let chars: Vec<char> = hex.chars().collect();
            let r = u8::from_str_radix(&format!("{}{}", chars[0], chars[0]), 16).ok()?;
            let g = u8::from_str_radix(&format!("{}{}", chars[1], chars[1]), 16).ok()?;
            let b = u8::from_str_radix(&format!("{}{}", chars[2], chars[2]), 16).ok()?;
            Some((r, g, b))
        }
        // 6-digit hex: #FF00AA
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Luminance (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("#FFFFFF", 0.9, 1.1)] // White - very bright
    #[case("#000000", 0.0, 0.1)] // Black - very dark
    #[case("#808080", 0.45, 0.55)] // Gray - medium
    fn test_luminance(#[case] hex: &str, #[case] min: f32, #[case] max: f32) {
        let lum = luminance(hex);
        assert!(lum >= min && lum <= max, "luminance({}) = {} not in [{}, {}]", hex, lum, min, max);
    }

    // ========================================================================
    // Contrast Color (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("#FFFFFF", "#000000")] // Black on white
    #[case("#000000", "#FFFFFF")] // White on black
    #[case("#3178C6", "#FFFFFF")] // White on TypeScript blue
    #[case("#F7DF1E", "#000000")] // Black on JavaScript yellow
    fn test_contrast_color(#[case] bg: &str, #[case] expected: &str) {
        assert_eq!(contrast_color(bg), expected);
    }

    // ========================================================================
    // Darken (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("#FFFFFF", 0.0, "#FFFFFF")] // No change
    #[case("#FFFFFF", 1.0, "#000000")] // Complete darkening
    #[case("#FF0000", 0.5, "#800000")] // 50% red darkening
    fn test_darken(#[case] hex: &str, #[case] amount: f32, #[case] expected: &str) {
        assert_eq!(darken(hex, amount), expected);
    }

    // ========================================================================
    // Hex Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    // 6-digit hex
    #[case("#FF0000", Some((255, 0, 0)))]
    #[case("00FF00", Some((0, 255, 0)))]
    #[case("#0000FF", Some((0, 0, 255)))]
    // 3-digit hex expansion
    #[case("#F00", Some((255, 0, 0)))]
    #[case("0F0", Some((0, 255, 0)))]
    #[case("#00F", Some((0, 0, 255)))]
    // Case insensitive
    #[case("#ff0000", Some((255, 0, 0)))]
    #[case("#Ff0000", Some((255, 0, 0)))]
    // Invalid input
    #[case("invalid", None)]
    #[case("#GG0000", None)]
    #[case("#FF", None)]
    #[case("#FF00000", None)]
    fn test_parse_hex(#[case] input: &str, #[case] expected: Option<(u8, u8, u8)>) {
        assert_eq!(parse_hex(input), expected);
    }
}
