use serde::{Deserialize, Serialize};

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

    /// Get separator character by ID, or return the input if it's a single char
    pub fn resolve(&self, input: &str) -> Option<String> {
        // First try to find by ID
        if let Some(sep) = self.find_separator(input) {
            return Some(sep.char.clone());
        }

        // If input is a single character, use it directly
        if input.chars().count() == 1 {
            return Some(input.to_string());
        }

        // Not found and not a single char
        None
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
        assert_eq!(result, Some("·".to_string()));
    }

    #[test]
    fn test_resolve_direct_character() {
        let data = SeparatorsData::load().unwrap();
        let result = data.resolve("⚡");
        assert_eq!(result, Some("⚡".to_string()));
    }

    #[test]
    fn test_resolve_invalid() {
        let data = SeparatorsData::load().unwrap();
        let result = data.resolve("notaseparator");
        assert_eq!(result, None);
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
