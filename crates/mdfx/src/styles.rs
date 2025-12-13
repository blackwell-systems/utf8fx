use crate::registry::EvalContext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Category of Unicode style
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleCategory {
    Bold,
    Boxed,
    Technical,
    Elegant,
}

/// What character types a style supports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleSupport {
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

/// A Unicode text style with character mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: StyleCategory,
    pub unicode_block: String,
    pub aliases: Vec<String>,
    pub supports: StyleSupport,
    pub mappings: HashMap<char, char>,
    #[serde(default)]
    pub contexts: Vec<EvalContext>,
}

impl Style {
    /// Check if this style supports a given character
    pub fn supports_char(&self, c: char) -> bool {
        self.mappings.contains_key(&c)
    }

    /// Convert a character using this style's mappings
    /// Returns the original character if no mapping exists
    pub fn convert_char(&self, c: char) -> char {
        *self.mappings.get(&c).unwrap_or(&c)
    }

    /// Check if this style matches a given ID or alias
    pub fn matches(&self, name: &str) -> bool {
        self.id == name || self.aliases.contains(&name.to_string())
    }
}

/// Intermediate structure to parse registry.json for styles
#[derive(Debug, Deserialize)]
struct RegistryStylesExtract {
    version: String,
    renderables: RenderablesExtract,
}

#[derive(Debug, Deserialize)]
struct RenderablesExtract {
    styles: HashMap<String, Style>,
}

/// Root structure for styles data (compatible with old StylesData API)
#[derive(Debug, Serialize, Deserialize)]
pub struct StylesData {
    pub version: String,
    #[serde(default)]
    pub last_updated: String,
    #[serde(default)]
    pub total_styles: usize,
    pub styles: HashMap<String, Style>,
}

impl StylesData {
    /// Load styles from unified registry.json
    pub fn load() -> crate::Result<Self> {
        const REGISTRY_JSON: &str = include_str!("../data/registry.json");
        let registry: RegistryStylesExtract = serde_json::from_str(REGISTRY_JSON)?;

        let styles_count = registry.renderables.styles.len();
        Ok(Self {
            version: registry.version,
            last_updated: String::new(),
            total_styles: styles_count,
            styles: registry.renderables.styles,
        })
    }

    /// Find a style by ID or alias
    pub fn find_style(&self, name: &str) -> Option<&Style> {
        // First try direct lookup
        if let Some(style) = self.styles.get(name) {
            return Some(style);
        }

        // Then try aliases
        self.styles.values().find(|s| s.matches(name))
    }

    /// Get all styles in a category
    pub fn by_category(&self, category: &StyleCategory) -> Vec<&Style> {
        self.styles
            .values()
            .filter(|s| &s.category == category)
            .collect()
    }

    /// List all style IDs
    pub fn list_ids(&self) -> Vec<String> {
        let mut ids: Vec<_> = self.styles.keys().cloned().collect();
        ids.sort();
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_styles() {
        let data = StylesData::load().unwrap();
        assert_eq!(data.total_styles, 19);
        assert_eq!(data.styles.len(), 19);
    }

    #[test]
    fn test_find_style_by_id() {
        let data = StylesData::load().unwrap();
        let style = data.find_style("mathbold");
        assert!(style.is_some());
        assert_eq!(style.unwrap().name, "Mathematical Bold");
    }

    #[test]
    fn test_find_style_by_alias() {
        let data = StylesData::load().unwrap();
        let style = data.find_style("mb");
        assert!(style.is_some());
        assert_eq!(style.unwrap().id, "mathbold");
    }

    #[test]
    fn test_style_supports_char() {
        let data = StylesData::load().unwrap();
        let mathbold = data.find_style("mathbold").unwrap();

        assert!(mathbold.supports_char('A'));
        assert!(mathbold.supports_char('a'));
        assert!(mathbold.supports_char('0'));
        assert!(!mathbold.supports_char('!'));
    }

    #[test]
    fn test_convert_char() {
        let data = StylesData::load().unwrap();
        let mathbold = data.find_style("mathbold").unwrap();

        assert_eq!(mathbold.convert_char('A'), '𝐀');
        assert_eq!(mathbold.convert_char('B'), '𝐁');
        assert_eq!(mathbold.convert_char('!'), '!'); // unchanged
    }

    #[test]
    fn test_by_category() {
        let data = StylesData::load().unwrap();
        let bold_styles = data.by_category(&StyleCategory::Bold);
        assert!(bold_styles.len() >= 3);
    }
}
