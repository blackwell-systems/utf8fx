//! Configuration file support for mdfx
//!
//! Loads project-specific configuration from `.mdfx.json` files,
//! including user-defined template partials.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A user-defined partial template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialDef {
    /// The template string (may contain $1, $2 placeholders)
    pub template: String,

    /// Optional description for documentation
    #[serde(default)]
    pub description: Option<String>,
}

/// mdfx configuration loaded from `.mdfx.json`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MdfxConfig {
    /// User-defined partial templates
    #[serde(default)]
    pub partials: HashMap<String, PartialDef>,

    /// Custom color palette overrides
    #[serde(default)]
    pub palette: HashMap<String, String>,
}

impl MdfxConfig {
    /// Create an empty configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from a JSON file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the `.mdfx.json` file
    ///
    /// # Example
    ///
    /// ```ignore
    /// use mdfx::config::MdfxConfig;
    ///
    /// let config = MdfxConfig::load(".mdfx.json")?;
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(Error::IoError)?;
        let config: MdfxConfig = serde_json::from_str(&content).map_err(|e| {
            Error::ParseError(format!(
                "Failed to parse config '{}': {}",
                path.display(),
                e
            ))
        })?;
        Ok(config)
    }

    /// Try to load configuration from default locations
    ///
    /// Searches for `.mdfx.json` in the current directory and parent directories.
    /// Returns None if no config file is found.
    pub fn discover() -> Option<Self> {
        let mut current = std::env::current_dir().ok()?;

        loop {
            let config_path = current.join(".mdfx.json");
            if config_path.exists() {
                return Self::load(&config_path).ok();
            }

            // Move to parent directory
            if !current.pop() {
                break;
            }
        }

        None
    }

    /// Get a partial template by name
    pub fn get_partial(&self, name: &str) -> Option<&PartialDef> {
        self.partials.get(name)
    }

    /// Check if a partial exists
    pub fn has_partial(&self, name: &str) -> bool {
        self.partials.contains_key(name)
    }

    /// Get all partial names
    pub fn partial_names(&self) -> impl Iterator<Item = &String> {
        self.partials.keys()
    }

    /// Merge another config into this one (other takes precedence)
    pub fn merge(&mut self, other: MdfxConfig) {
        self.partials.extend(other.partials);
        self.palette.extend(other.palette);
    }
}

/// Expand a partial template with content substitution
///
/// Replaces `$content` or `$1` with the provided content.
/// Future: support `$2`, `$3` for multi-argument partials.
pub fn expand_partial(template: &str, content: &str) -> String {
    template.replace("$content", content).replace("$1", content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_config_parse() {
        let json = r#"{
            "partials": {
                "hero": {
                    "template": "{{fr:gradient}}{{mathbold}}$1{{/}}{{/}}",
                    "description": "Hero header with gradient frame"
                },
                "techstack": {
                    "template": "{{ui:tech:rust/}} {{ui:tech:typescript/}}"
                }
            },
            "palette": {
                "brand": "FF5500"
            }
        }"#;

        let config: MdfxConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.partials.len(), 2);
        assert!(config.has_partial("hero"));
        assert!(config.has_partial("techstack"));
        assert_eq!(config.palette.get("brand"), Some(&"FF5500".to_string()));

        let hero = config.get_partial("hero").unwrap();
        assert_eq!(hero.template, "{{fr:gradient}}{{mathbold}}$1{{/}}{{/}}");
        assert_eq!(
            hero.description,
            Some("Hero header with gradient frame".to_string())
        );
    }

    #[test]
    fn test_expand_partial() {
        let template = "{{fr:gradient}}$1{{/}}";
        let result = expand_partial(template, "HELLO");
        assert_eq!(result, "{{fr:gradient}}HELLO{{/}}");

        let template2 = "PREFIX $content SUFFIX";
        let result2 = expand_partial(template2, "MIDDLE");
        assert_eq!(result2, "PREFIX MIDDLE SUFFIX");
    }

    #[test]
    fn test_config_defaults() {
        let json = "{}";
        let config: MdfxConfig = serde_json::from_str(json).unwrap();
        assert!(config.partials.is_empty());
        assert!(config.palette.is_empty());
    }

    #[test]
    fn test_config_merge() {
        let mut config1 = MdfxConfig::new();
        config1.partials.insert(
            "a".to_string(),
            PartialDef {
                template: "template_a".to_string(),
                description: None,
            },
        );

        let mut config2 = MdfxConfig::new();
        config2.partials.insert(
            "b".to_string(),
            PartialDef {
                template: "template_b".to_string(),
                description: None,
            },
        );

        config1.merge(config2);
        assert!(config1.has_partial("a"));
        assert!(config1.has_partial("b"));
    }

    #[test]
    fn test_config_new() {
        let config = MdfxConfig::new();
        assert!(config.partials.is_empty());
        assert!(config.palette.is_empty());
    }

    #[test]
    fn test_partial_names() {
        let mut config = MdfxConfig::new();
        config.partials.insert(
            "alpha".to_string(),
            PartialDef {
                template: "a".to_string(),
                description: None,
            },
        );
        config.partials.insert(
            "beta".to_string(),
            PartialDef {
                template: "b".to_string(),
                description: None,
            },
        );

        let names: Vec<&String> = config.partial_names().collect();
        assert_eq!(names.len(), 2);
        assert!(names.iter().any(|n| *n == "alpha"));
        assert!(names.iter().any(|n| *n == "beta"));
    }

    #[test]
    fn test_get_partial_not_found() {
        let config = MdfxConfig::new();
        assert!(config.get_partial("nonexistent").is_none());
        assert!(!config.has_partial("nonexistent"));
    }

    #[test]
    fn test_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".mdfx.json");

        let json = r#"{
            "partials": {
                "test": {
                    "template": "hello $1"
                }
            },
            "palette": {
                "custom": "AABBCC"
            }
        }"#;

        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();

        let config = MdfxConfig::load(&config_path).unwrap();
        assert!(config.has_partial("test"));
        assert_eq!(config.palette.get("custom"), Some(&"AABBCC".to_string()));
    }

    #[test]
    fn test_load_file_not_found() {
        let result = MdfxConfig::load("/nonexistent/path/.mdfx.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".mdfx.json");

        let mut file = std::fs::File::create(&config_path).unwrap();
        file.write_all(b"{ invalid json }").unwrap();

        let result = MdfxConfig::load(&config_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge_palette_override() {
        let mut config1 = MdfxConfig::new();
        config1
            .palette
            .insert("color".to_string(), "FF0000".to_string());

        let mut config2 = MdfxConfig::new();
        config2
            .palette
            .insert("color".to_string(), "00FF00".to_string());

        config1.merge(config2);
        assert_eq!(config1.palette.get("color"), Some(&"00FF00".to_string()));
    }

    #[test]
    fn test_expand_partial_both_placeholders() {
        let template = "$content and $1";
        let result = expand_partial(template, "VALUE");
        assert_eq!(result, "VALUE and VALUE");
    }

    #[test]
    fn test_expand_partial_no_placeholder() {
        let template = "no placeholders here";
        let result = expand_partial(template, "unused");
        assert_eq!(result, "no placeholders here");
    }
}
