use crate::error::{Error, Result};
use crate::registry::EvalContext;
use serde::Deserialize;
use std::collections::HashMap;

/// Badge renderer for enclosed alphanumeric characters
///
/// Badges are pre-composed Unicode characters that enclose numbers or letters,
/// such as ① (circled one), ⒜ (parenthesized a), or ❶ (negative circled one).
///
/// Unlike styles (which map every character) or frames (which add prefix/suffix),
/// badges have limited charset support - only specific numbers (0-20) and letters (a-z).
pub struct BadgeRenderer {
    badges: HashMap<String, BadgeType>,
}

/// A badge type definition with character mappings
#[derive(Debug, Clone, Deserialize)]
pub struct BadgeType {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub mappings: HashMap<String, String>,
    #[serde(default)]
    pub contexts: Vec<EvalContext>,
}

/// Intermediate structure to parse registry.json for badges
#[derive(Debug, Deserialize)]
struct RegistryBadgesExtract {
    renderables: RenderablesExtract,
}

#[derive(Debug, Deserialize)]
struct RenderablesExtract {
    badges: HashMap<String, BadgeType>,
}

impl BadgeRenderer {
    /// Create a new badge renderer by loading from registry.json
    pub fn new() -> Result<Self> {
        let data = include_str!("../data/registry.json");
        let registry: RegistryBadgesExtract = serde_json::from_str(data).map_err(|e| {
            Error::ParseError(format!("Failed to parse registry.json for badges: {}", e))
        })?;

        Ok(BadgeRenderer {
            badges: registry.renderables.badges,
        })
    }

    /// Apply a badge to text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to enclose (must be in the badge's supported charset)
    /// * `badge_type` - The badge type ID or alias (e.g., "circle", "paren")
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::BadgeRenderer;
    ///
    /// let renderer = BadgeRenderer::new().unwrap();
    /// let result = renderer.apply_badge("1", "circle").unwrap();
    /// assert_eq!(result, "①");
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::UnknownBadge` if the badge type doesn't exist.
    /// Returns `Error::UnsupportedChar` if the text isn't in the badge's charset.
    pub fn apply_badge(&self, text: &str, badge_type: &str) -> Result<String> {
        let badge = self.get_badge(badge_type)?;

        // Try to map the text directly
        if let Some(mapped) = badge.mappings.get(text) {
            return Ok(mapped.clone());
        }

        // Text not supported by this badge type
        Err(Error::UnsupportedChar(
            badge_type.to_string(),
            text.to_string(),
        ))
    }

    /// Get a badge type by ID or alias
    pub fn get_badge(&self, name: &str) -> Result<&BadgeType> {
        // First try direct lookup
        if let Some(badge) = self.badges.get(name) {
            return Ok(badge);
        }

        // Then try aliases
        self.badges
            .values()
            .find(|b| b.aliases.contains(&name.to_string()))
            .ok_or_else(|| Error::UnknownBadge(name.to_string()))
    }

    /// Check if a badge type exists
    pub fn has_badge(&self, name: &str) -> bool {
        self.get_badge(name).is_ok()
    }

    /// List all available badge types
    pub fn list_badges(&self) -> Vec<&BadgeType> {
        let mut badges: Vec<_> = self.badges.values().collect();
        badges.sort_by(|a, b| a.id.cmp(&b.id));
        badges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_renderer_new() {
        let renderer = BadgeRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_apply_circle_badge() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("1", "circle").unwrap();
        assert_eq!(result, "①");
    }

    #[test]
    fn test_apply_circle_badge_multi_digit() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("10", "circle").unwrap();
        assert_eq!(result, "⑩");
    }

    #[test]
    fn test_apply_paren_badge() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("5", "paren").unwrap();
        assert_eq!(result, "⑸");
    }

    #[test]
    fn test_apply_paren_letter_badge() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("a", "paren-letter").unwrap();
        assert_eq!(result, "⒜");
    }

    #[test]
    fn test_apply_negative_circle_badge() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("3", "negative-circle").unwrap();
        assert_eq!(result, "❸");
    }

    #[test]
    fn test_badge_alias() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("2", "circled").unwrap();
        assert_eq!(result, "②");
    }

    #[test]
    fn test_unknown_badge() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("1", "invalid-badge");
        assert!(result.is_err());
    }

    #[test]
    fn test_unsupported_char() {
        let renderer = BadgeRenderer::new().unwrap();
        let result = renderer.apply_badge("99", "circle");
        assert!(result.is_err());
    }

    #[test]
    fn test_has_badge() {
        let renderer = BadgeRenderer::new().unwrap();
        assert!(renderer.has_badge("circle"));
        assert!(renderer.has_badge("circled"));
        assert!(!renderer.has_badge("nonexistent"));
    }

    #[test]
    fn test_list_badges() {
        let renderer = BadgeRenderer::new().unwrap();
        let badges = renderer.list_badges();
        assert!(!badges.is_empty());
    }
}
