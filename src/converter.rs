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
}

impl Converter {
    /// Create a new converter with all available styles loaded
    pub fn new() -> Result<Self> {
        let styles_data = &*STYLES;
        Ok(Self {
            styles: styles_data.styles.clone(),
        })
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
    /// use utf8fx::Converter;
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
        let style_obj = self.get_style(style)?;

        let result: String = text.chars().map(|c| style_obj.convert_char(c)).collect();

        Ok(result)
    }

    /// Get a style by ID or alias
    pub fn get_style(&self, name: &str) -> Result<&Style> {
        // First try direct lookup
        if let Some(style) = self.styles.get(name) {
            return Ok(style);
        }

        // Then try aliases
        self.styles
            .values()
            .find(|s| s.matches(name))
            .ok_or_else(|| Error::UnknownStyle(name.to_string()))
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

    #[test]
    fn test_converter_new() {
        let converter = Converter::new();
        assert!(converter.is_ok());
    }

    #[test]
    fn test_convert_mathbold() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("ABC", "mathbold").unwrap();
        assert_eq!(result, "𝐀𝐁𝐂");
    }

    #[test]
    fn test_convert_mathbold_lowercase() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("abc", "mathbold").unwrap();
        assert_eq!(result, "𝐚𝐛𝐜");
    }

    #[test]
    fn test_convert_mathbold_numbers() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("123", "mathbold").unwrap();
        assert_eq!(result, "𝟏𝟐𝟑");
    }

    #[test]
    fn test_convert_mathbold_mixed() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("Hello World 123!", "mathbold").unwrap();
        assert_eq!(result, "𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝 𝟏𝟐𝟑!");
    }

    #[test]
    fn test_convert_fullwidth() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("ABC", "fullwidth").unwrap();
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_convert_negative_squared() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("WARNING", "negative-squared").unwrap();
        assert_eq!(result, "🆆🅰🆁🅽🅸🅽🅶");
    }

    #[test]
    fn test_convert_small_caps() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("hello", "small-caps").unwrap();
        assert_eq!(result, "ʜᴇʟʟᴏ");
    }

    #[test]
    fn test_convert_with_alias() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("TEST", "mb").unwrap();
        assert_eq!(result, "𝐓𝐄𝐒𝐓");
    }

    #[test]
    fn test_convert_unknown_style() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("TEST", "fakestyle");
        assert!(result.is_err());
        match result {
            Err(Error::UnknownStyle(name)) => assert_eq!(name, "fakestyle"),
            _ => panic!("Expected UnknownStyle error"),
        }
    }

    #[test]
    fn test_list_styles() {
        let converter = Converter::new().unwrap();
        let styles = converter.list_styles();
        assert_eq!(styles.len(), 19);
    }

    #[test]
    fn test_has_style() {
        let converter = Converter::new().unwrap();
        assert!(converter.has_style("mathbold"));
        assert!(converter.has_style("mb"));
        assert!(!converter.has_style("fakestyle"));
    }

    #[test]
    fn test_preserves_whitespace() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("A B  C", "mathbold").unwrap();
        assert_eq!(result, "𝐀 𝐁  𝐂");
    }

    #[test]
    fn test_preserves_punctuation() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("Hello, World!", "mathbold").unwrap();
        assert_eq!(result, "𝐇𝐞𝐥𝐥𝐨, 𝐖𝐨𝐫𝐥𝐝!");
    }

    #[test]
    fn test_all_styles_loadable() {
        let converter = Converter::new().unwrap();
        let style_ids = vec![
            "mathbold",
            "fullwidth",
            "negative-squared",
            "negative-circled",
            "squared-latin",
            "small-caps",
            "monospace",
            "double-struck",
            "sans-serif-bold",
            "italic",
            "bold-italic",
        ];

        for id in style_ids {
            assert!(
                converter.has_style(id),
                "Style '{}' should be available",
                id
            );
            let result = converter.convert("TEST", id);
            assert!(result.is_ok(), "Style '{}' should convert successfully", id);
        }
    }
}
