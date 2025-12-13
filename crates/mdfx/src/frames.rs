use crate::error::{Error, Result};
use crate::registry::EvalContext;
use serde::Deserialize;
use std::collections::HashMap;

/// Frame renderer for adding decorative elements around text
pub struct FrameRenderer {
    frames: HashMap<String, FrameStyle>,
}

/// A frame style definition
#[derive(Debug, Clone, Deserialize)]
pub struct FrameStyle {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prefix: String,
    pub suffix: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub contexts: Vec<EvalContext>,
}

/// Intermediate structure to parse registry.json for frames
#[derive(Debug, Deserialize)]
struct RegistryFramesExtract {
    renderables: RenderablesExtract,
}

#[derive(Debug, Deserialize)]
struct RenderablesExtract {
    frames: HashMap<String, FrameEntry>,
}

/// Frame entry as stored in registry.json (without redundant id/name)
#[derive(Debug, Deserialize)]
struct FrameEntry {
    prefix: String,
    suffix: String,
    description: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    contexts: Vec<EvalContext>,
}

impl FrameRenderer {
    /// Create a new frame renderer by loading from registry.json
    pub fn new() -> Result<Self> {
        let data = include_str!("../data/registry.json");
        let registry: RegistryFramesExtract = serde_json::from_str(data).map_err(|e| {
            Error::ParseError(format!("Failed to parse registry.json for frames: {}", e))
        })?;

        // Convert FrameEntry to FrameStyle, deriving id and name from the key
        let frames: HashMap<String, FrameStyle> = registry
            .renderables
            .frames
            .into_iter()
            .map(|(id, entry)| {
                let name = id
                    .split('-')
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                            None => String::new(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                let style = FrameStyle {
                    id: id.clone(),
                    name,
                    description: entry.description,
                    prefix: entry.prefix,
                    suffix: entry.suffix,
                    aliases: entry.aliases,
                    contexts: entry.contexts,
                };
                (id, style)
            })
            .collect();

        Ok(FrameRenderer { frames })
    }

    /// Apply a frame around text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to frame
    /// * `frame_style` - The frame style ID or alias (e.g., "gradient", "solid-left")
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::FrameRenderer;
    ///
    /// let renderer = FrameRenderer::new().unwrap();
    /// let result = renderer.apply_frame("Title", "gradient").unwrap();
    /// assert_eq!(result, "▓▒░ Title ░▒▓");
    /// ```
    pub fn apply_frame(&self, text: &str, frame_style: &str) -> Result<String> {
        let style = self.get_frame(frame_style)?;
        Ok(format!("{}{}{}", style.prefix, text, style.suffix))
    }

    /// Get a frame style by ID or alias
    pub fn get_frame(&self, name: &str) -> Result<&FrameStyle> {
        // First try direct lookup
        if let Some(frame) = self.frames.get(name) {
            return Ok(frame);
        }

        // Then try aliases
        self.frames
            .values()
            .find(|f| f.aliases.contains(&name.to_string()))
            .ok_or_else(|| Error::UnknownFrame(name.to_string()))
    }

    /// Check if a frame style exists
    pub fn has_frame(&self, name: &str) -> bool {
        self.get_frame(name).is_ok()
    }

    /// List all available frames
    pub fn list_frames(&self) -> Vec<&FrameStyle> {
        let mut frames: Vec<_> = self.frames.values().collect();
        frames.sort_by(|a, b| a.id.cmp(&b.id));
        frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_renderer_new() {
        let renderer = FrameRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_apply_gradient_frame() {
        let renderer = FrameRenderer::new().unwrap();
        let result = renderer.apply_frame("Test", "gradient");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "▓▒░ Test ░▒▓");
    }

    #[test]
    fn test_has_frame() {
        let renderer = FrameRenderer::new().unwrap();
        assert!(renderer.has_frame("gradient"));
        assert!(renderer.has_frame("solid-left"));
        assert!(!renderer.has_frame("nonexistent"));
    }

    #[test]
    fn test_list_frames() {
        let renderer = FrameRenderer::new().unwrap();
        let frames = renderer.list_frames();
        assert!(frames.len() >= 10); // At least 10 frames
    }

    #[test]
    fn test_frame_alias() {
        let renderer = FrameRenderer::new().unwrap();
        // Check that aliases work
        let frame = renderer.get_frame("gradient");
        assert!(frame.is_ok());
    }
}
