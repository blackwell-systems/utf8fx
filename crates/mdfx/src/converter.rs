use crate::error::{Error, Result};
use crate::styles::{Style, StylesData};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref STYLES: StylesData = StylesData::load().expect("Failed to load styles.json");
}

/// Main converter for Unicode text styling
pub struct Converter {
    styles: HashMap<String, Style>,
    /// Map from alias -> style ID for O(1) lookup
    alias_map: HashMap<String, String>,
}

impl Converter {
    /// Create a new converter with all available styles loaded
    pub fn new() -> Result<Self> {
        let styles_data = &*STYLES;

        // Build alias map for O(1) alias lookup
        let alias_map: HashMap<String, String> = styles_data
            .styles
            .iter()
            .flat_map(|(id, style)| {
                style
                    .aliases
                    .iter()
                    .map(move |alias| (alias.clone(), id.clone()))
            })
            .collect();

        Ok(Self {
            styles: styles_data.styles.clone(),
            alias_map,
        })
    }

    /// Internal unified method for converting text with optional character separation
    ///
    /// This method handles all conversion cases: no spacing, space-based spacing,
    /// and custom separator characters. All public conversion methods delegate to this.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to convert
    /// * `style` - The style ID or alias
    /// * `separator` - The separator string to insert between characters
    /// * `count` - Number of times to repeat the separator (0 = no separation)
    fn convert_with_char_between(
        &self,
        text: &str,
        style: &str,
        separator: &str,
        count: usize,
    ) -> Result<String> {
        let style_obj = self.get_style(style)?;

        // Fast path: no separation needed
        if count == 0 || separator.is_empty() {
            let result: String = text
                .chars()
                .map(|c| style_obj.convert_char_to_string(c))
                .collect();
            return Ok(result);
        }

        // With separation: convert each char and add separator between
        let mut result = String::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            result.push_str(&style_obj.convert_char_to_string(c));

            // Add separator after each character except the last
            if chars.peek().is_some() {
                for _ in 0..count {
                    result.push_str(separator);
                }
            }
        }

        Ok(result)
    }

    /// Convert text to a specified Unicode style
    ///
    /// # Arguments
    ///
    /// * `text` - The text to convert
    /// * `style` - The style ID or alias (e.g., "mathbold" or "mb")
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::Converter;
    ///
    /// let converter = Converter::new().unwrap();
    /// let result = converter.convert("HELLO", "mathbold").unwrap();
    /// assert_eq!(result, "𝐇𝐄𝐋𝐋𝐎");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::UnknownStyle` if the style doesn't exist.
    pub fn convert(&self, text: &str, style: &str) -> Result<String> {
        self.convert_with_char_between(text, style, "", 0)
    }

    /// Convert text to a specified Unicode style with character spacing
    ///
    /// # Arguments
    ///
    /// * `text` - The text to convert
    /// * `style` - The style ID or alias (e.g., "mathbold" or "mb")
    /// * `spacing` - Number of spaces to insert between each character (0 = no spacing)
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::Converter;
    ///
    /// let converter = Converter::new().unwrap();
    /// let result = converter.convert_with_spacing("HELLO", "mathbold", 1).unwrap();
    /// assert_eq!(result, "𝐇 𝐄 𝐋 𝐋 𝐎");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::UnknownStyle` if the style doesn't exist.
    pub fn convert_with_spacing(&self, text: &str, style: &str, spacing: usize) -> Result<String> {
        self.convert_with_char_between(text, style, " ", spacing)
    }

    /// Convert text to a style with custom separator between characters
    ///
    /// # Arguments
    ///
    /// * `text` - The text to convert
    /// * `style` - The style ID or alias (e.g., "mathbold" or "mb")
    /// * `separator` - The separator character(s) to insert between each character
    /// * `count` - Number of times to repeat the separator (1 = single separator)
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::Converter;
    ///
    /// let converter = Converter::new().unwrap();
    /// let result = converter.convert_with_separator("HELLO", "mathbold", "·", 1).unwrap();
    /// assert_eq!(result, "𝐇·𝐄·𝐋·𝐋·𝐎");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::UnknownStyle` if the style doesn't exist.
    pub fn convert_with_separator(
        &self,
        text: &str,
        style: &str,
        separator: &str,
        count: usize,
    ) -> Result<String> {
        self.convert_with_char_between(text, style, separator, count)
    }

    /// Get a style by ID or alias (O(1) lookup)
    pub fn get_style(&self, name: &str) -> Result<&Style> {
        // First try direct lookup by ID
        if let Some(style) = self.styles.get(name) {
            return Ok(style);
        }

        // Then try alias lookup via pre-built map (O(1) instead of O(n))
        if let Some(id) = self.alias_map.get(name) {
            if let Some(style) = self.styles.get(id) {
                return Ok(style);
            }
        }

        Err(Error::UnknownStyle(name.to_string()))
    }

    /// List all available styles
    pub fn list_styles(&self) -> Vec<&Style> {
        let mut styles: Vec<_> = self.styles.values().collect();
        styles.sort_by(|a, b| a.id.cmp(&b.id));
        styles
    }

    /// List style IDs
    pub fn list_ids(&self) -> Vec<String> {
        let mut ids: Vec<_> = self.styles.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// Check if a style exists
    pub fn has_style(&self, name: &str) -> bool {
        self.get_style(name).is_ok()
    }
}

impl Default for Converter {
    fn default() -> Self {
        Self::new().expect("Failed to create default converter")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Basic Converter Tests
    // ========================================================================

    #[test]
    fn test_converter_new() {
        let converter = Converter::new();
        assert!(converter.is_ok());
    }

    #[test]
    fn test_list_styles() {
        let converter = Converter::new().unwrap();
        let styles = converter.list_styles();
        assert_eq!(styles.len(), 24);
    }

    #[test]
    fn test_convert_unknown_style() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("TEST", "fakestyle");
        assert!(result.is_err());
        let Err(Error::UnknownStyle(name)) = result else {
            unreachable!("Expected UnknownStyle error, got {:?}", result);
        };
        assert_eq!(name, "fakestyle");
    }

    // ========================================================================
    // Parameterized Style Conversion Tests
    // ========================================================================

    #[rstest]
    #[case("ABC", "mathbold", "𝐀𝐁𝐂")]
    #[case("abc", "mathbold", "𝐚𝐛𝐜")]
    #[case("123", "mathbold", "𝟏𝟐𝟑")]
    #[case("Hello World 123!", "mathbold", "𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝 𝟏𝟐𝟑!")]
    #[case("ABC", "fullwidth", "ＡＢＣ")]
    #[case("WARNING", "negative-squared", "🆆🅰🆁🅽🅸🅽🅶")]
    #[case("hello", "small-caps", "ʜᴇʟʟᴏ")]
    #[case("TEST", "mb", "𝐓𝐄𝐒𝐓")] // alias test
    #[case("A B  C", "mathbold", "𝐀 𝐁  𝐂")] // preserves whitespace
    #[case("Hello, World!", "mathbold", "𝐇𝐞𝐥𝐥𝐨, 𝐖𝐨𝐫𝐥𝐝!")] // preserves punctuation
    fn test_convert(#[case] input: &str, #[case] style: &str, #[case] expected: &str) {
        let converter = Converter::new().unwrap();
        let result = converter.convert(input, style).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "Hello",
        "strikethrough",
        "H\u{0336}e\u{0336}l\u{0336}l\u{0336}o\u{0336}"
    )]
    #[case(
        "Hi there",
        "strikethrough",
        "H\u{0336}i\u{0336} t\u{0336}h\u{0336}e\u{0336}r\u{0336}e\u{0336}"
    )]
    fn test_strikethrough(#[case] input: &str, #[case] style: &str, #[case] expected: &str) {
        let converter = Converter::new().unwrap();
        let result = converter.convert(input, style).unwrap();
        assert_eq!(result, expected);
    }

    // ========================================================================
    // Style Existence Tests
    // ========================================================================

    #[rstest]
    #[case("mathbold", true)]
    #[case("mb", true)]
    #[case("fullwidth", true)]
    #[case("negative-squared", true)]
    #[case("negative-circled", true)]
    #[case("squared-latin", true)]
    #[case("small-caps", true)]
    #[case("monospace", true)]
    #[case("double-struck", true)]
    #[case("sans-serif-bold", true)]
    #[case("italic", true)]
    #[case("bold-italic", true)]
    #[case("strike", true)]
    #[case("st", true)]
    #[case("crossed", true)]
    #[case("fakestyle", false)]
    fn test_has_style(#[case] style: &str, #[case] expected: bool) {
        let converter = Converter::new().unwrap();
        assert_eq!(converter.has_style(style), expected);
    }

    // ========================================================================
    // Spacing Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("HELLO", "mathbold", 0, "𝐇𝐄𝐋𝐋𝐎")]
    #[case("HELLO", "mathbold", 1, "𝐇 𝐄 𝐋 𝐋 𝐎")]
    #[case("ABC", "script", 2, "𝒜  ℬ  𝒞")]
    #[case("GO", "fraktur", 3, "𝔊   𝔒")]
    #[case("hello", "mathbold", 1, "𝐡 𝐞 𝐥 𝐥 𝐨")]
    #[case("A", "mathbold", 2, "𝐀")] // single char - no spacing added
    fn test_spacing(
        #[case] input: &str,
        #[case] style: &str,
        #[case] spacing: usize,
        #[case] expected: &str,
    ) {
        let converter = Converter::new().unwrap();
        let result = converter
            .convert_with_spacing(input, style, spacing)
            .unwrap();
        assert_eq!(result, expected);
    }

    // ========================================================================
    // Separator Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("HELLO", "mathbold", "·", 1, "𝐇·𝐄·𝐋·𝐋·𝐎")]
    #[case("ABC", "mathbold", "─", 1, "𝐀─𝐁─𝐂")]
    #[case("HI", "mathbold", "━", 1, "𝐇━𝐈")]
    #[case("ABC", "mathbold", "→", 1, "𝐀→𝐁→𝐂")]
    #[case("AB", "mathbold", "·", 3, "𝐀···𝐁")] // multiple count
    #[case("X", "mathbold", "·", 1, "𝐗")] // single char - no separator
    fn test_separator(
        #[case] input: &str,
        #[case] style: &str,
        #[case] separator: &str,
        #[case] count: usize,
        #[case] expected: &str,
    ) {
        let converter = Converter::new().unwrap();
        let result = converter
            .convert_with_separator(input, style, separator, count)
            .unwrap();
        assert_eq!(result, expected);
    }
}
