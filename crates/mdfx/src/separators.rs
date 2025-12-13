use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

/// A separator character with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Separator {
    pub id: String,
    pub name: String,
    pub char: String,
    pub unicode: String,
    pub description: String,
    pub example: String,
}

/// Root structure for separators.json
#[derive(Debug, Serialize, Deserialize)]
pub struct SeparatorsData {
    pub version: String,
    pub separators: Vec<Separator>,
}

impl SeparatorsData {
    /// Load separators from embedded JSON
    pub fn load() -> crate::Result<Self> {
        const SEPARATORS_JSON: &str = include_str!("../data/separators.json");
        let data: SeparatorsData = serde_json::from_str(SEPARATORS_JSON)?;
        Ok(data)
    }

    /// Find a separator by ID
    pub fn find_separator(&self, id: &str) -> Option<&Separator> {
        self.separators.iter().find(|s| s.id == id)
    }

    /// Get separator character by ID, or return the input if it's a single grapheme
    ///
    /// # Validation
    /// - Trims whitespace
    /// - Rejects template delimiters (`:`, `/`, `}`)
    /// - Requires exactly 1 grapheme cluster (supports emoji with variation selectors)
    pub fn resolve(&self, input: &str) -> Result<String, String> {
        // Normalize: trim whitespace
        let normalized = input.trim();

        if normalized.is_empty() {
            return Err("Separator cannot be empty".to_string());
        }

        // First try to find by ID
        if let Some(sep) = self.find_separator(normalized) {
            return Ok(sep.char.clone());
        }

        // If input is a single grapheme cluster, validate and use it directly
        let graphemes: Vec<&str> = normalized.graphemes(true).collect();
        if graphemes.len() == 1 {
            let grapheme = graphemes[0];

            // Reject template delimiters (single-char check is sufficient)
            if grapheme == ":" || grapheme == "/" || grapheme == "}" {
                return Err(format!(
                    "Character '{}' cannot be used as separator (reserved for template syntax)",
                    grapheme
                ));
            }

            return Ok(grapheme.to_string());
        }

        // Not found and not a single grapheme - provide helpful error
        Err(self.suggest_separator(normalized))
    }

    /// Generate a helpful error message with "did you mean" suggestions
    fn suggest_separator(&self, input: &str) -> String {
        let mut msg = format!("Unknown separator '{}'.", input);

        // Try to find similar named separators (simple edit distance)
        let similar: Vec<&str> = self.separators
            .iter()
            .filter(|s| {
                // Simple similarity check: common prefix or contains substring
                s.id.starts_with(input) ||
                s.id.contains(input) ||
                input.contains(&s.id)
            })
            .map(|s| s.id.as_str())
            .take(3)
            .collect();

        if !similar.is_empty() {
            msg.push_str(&format!("\n  Did you mean: {}?", similar.join(", ")));
        }

        msg.push_str("\n  Available named separators: ");
        msg.push_str(&self.list_ids().join(", "));
        msg.push_str("\n  Or use any single Unicode character (e.g., separator=⚡)");

        msg
    }

    /// List all separator IDs
    pub fn list_ids(&self) -> Vec<String> {
        self.separators.iter().map(|s| s.id.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_separators() {
        let data = SeparatorsData::load().unwrap();
        assert!(data.separators.len() >= 5);
        assert_eq!(data.version, "1.0.0");
    }

    #[test]
    fn test_find_separator_by_id() {
        let data = SeparatorsData::load().unwrap();
        let sep = data.find_separator("dot");
        assert!(sep.is_some());
        assert_eq!(sep.unwrap().char, "·");
    }

    #[test]
    fn test_resolve_named_separator() {
        let data = SeparatorsData::load().unwrap();
        let result = data.resolve("dot");
        assert_eq!(result, Ok("·".to_string()));
    }

    #[test]
    fn test_resolve_direct_character() {
        let data = SeparatorsData::load().unwrap();
        let result = data.resolve("⚡");
        assert_eq!(result, Ok("⚡".to_string()));
    }

    #[test]
    fn test_resolve_invalid() {
        let data = SeparatorsData::load().unwrap();
        let result = data.resolve("notaseparator");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unknown separator"));
        assert!(err.contains("Available named separators"));
    }

    #[test]
    fn test_resolve_with_whitespace() {
        let data = SeparatorsData::load().unwrap();
        let result = data.resolve("  dot  ");
        assert_eq!(result, Ok("·".to_string()));
    }

    #[test]
    fn test_resolve_empty() {
        let data = SeparatorsData::load().unwrap();
        let result = data.resolve("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_resolve_template_delimiters() {
        let data = SeparatorsData::load().unwrap();

        // Reject :
        let result = data.resolve(":");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("reserved for template syntax"));

        // Reject /
        let result = data.resolve("/");
        assert!(result.is_err());

        // Reject }
        let result = data.resolve("}");
        assert!(result.is_err());
    }

    #[test]
    fn test_suggest_separator() {
        let data = SeparatorsData::load().unwrap();

        // Test "did you mean" for partial match
        let result = data.resolve("arr");  // partial: arrow
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Did you mean"));
        assert!(err.contains("arrow"));
    }

    #[test]
    fn test_grapheme_clusters() {
        let data = SeparatorsData::load().unwrap();

        // Single emoji (1 grapheme, 1 char)
        let result = data.resolve("⭐");
        assert_eq!(result, Ok("⭐".to_string()));

        // Emoji with variation selector (1 grapheme, 2+ chars)
        let result = data.resolve("👨‍💻");  // Man technologist
        assert_eq!(result, Ok("👨‍💻".to_string()));

        // Flag emoji (1 grapheme, multiple scalars)
        let result = data.resolve("🇺🇸");
        assert_eq!(result, Ok("🇺🇸".to_string()));
    }

    #[test]
    fn test_list_ids() {
        let data = SeparatorsData::load().unwrap();
        let ids = data.list_ids();
        assert!(ids.contains(&"dot".to_string()));
        assert!(ids.contains(&"bullet".to_string()));
        assert!(ids.contains(&"arrow".to_string()));
    }

    #[test]
    fn test_all_predefined_separators() {
        let data = SeparatorsData::load().unwrap();

        // Test that all original separators still exist
        assert!(data.find_separator("dot").is_some());
        assert!(data.find_separator("bullet").is_some());
        assert!(data.find_separator("dash").is_some());
        assert!(data.find_separator("bolddash").is_some());
        assert!(data.find_separator("arrow").is_some());
    }
}
