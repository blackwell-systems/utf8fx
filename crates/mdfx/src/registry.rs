//! Unified Registry for mdfx
//!
//! This module provides a single source of truth for all renderables, styles,
//! and configuration. It consolidates the previously fragmented data files
//! (styles.json, frames.json, components.json, palette.json, shields.json)
//! into one unified registry.
//!
//! The registry enables:
//! - IntelliSense/editor tooling support via single JSON schema
//! - Context-aware validation (inline vs block vs frame_chrome)
//! - Unified resolution pipeline for all renderables
//! - Single source of truth for palette colors

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Evaluation context for renderables
///
/// Every renderable has a set of allowed contexts, and every expansion site
/// has a required context. The compiler validates compatibility at expansion time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalContext {
    /// Inline: Between characters in styled text
    /// Constraints: Compact, no newlines, max graphemes
    Inline,

    /// Block: Section-level, multiline allowed
    /// Constraints: None
    Block,

    /// FrameChrome: Frame prefix/suffix decorations
    /// Constraints: Single line, limited length
    FrameChrome,
}

impl EvalContext {
    /// Check if this context can be promoted to another context
    ///
    /// Promotion rules:
    /// - Inline → Block: Always allowed (inline is more restrictive)
    /// - Inline → FrameChrome: Allowed if meets length constraints
    /// - FrameChrome → Inline: Always allowed (chrome is compact)
    /// - FrameChrome → Block: Always allowed
    /// - Block → Inline: Never allowed
    /// - Block → FrameChrome: Never allowed
    pub fn can_promote_to(&self, target: EvalContext) -> bool {
        match (self, target) {
            // Same context is always valid
            (a, b) if *a == b => true,
            // Inline can promote to anything (it's the most restrictive)
            (EvalContext::Inline, _) => true,
            // FrameChrome can promote to anything (it's compact)
            (EvalContext::FrameChrome, _) => true,
            // Block cannot promote to more restrictive contexts
            (EvalContext::Block, EvalContext::Inline) => false,
            (EvalContext::Block, EvalContext::FrameChrome) => false,
            _ => false,
        }
    }
}

/// Optional parameter definition for components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalParam {
    #[serde(rename = "type")]
    pub param_type: String,
    pub default: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// A component definition (native or expand type)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    #[serde(rename = "type")]
    pub component_type: String,
    pub self_closing: bool,
    #[serde(default)]
    pub description: Option<String>,
    pub contexts: Vec<EvalContext>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub optional_params: Option<HashMap<String, OptionalParam>>,
    #[serde(default)]
    pub post_process: Option<String>,
}

/// Suffix generation mode for frames
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SuffixMode {
    /// Suffix is the pattern reversed (▓▒░ → ░▒▓)
    Mirror,
    /// Suffix is the same as pattern (▓▒░ → ▓▒░)
    Repeat,
    /// Prefix only, no suffix
    PrefixOnly,
    /// Suffix only, no prefix
    SuffixOnly,
    /// Suffix is the pattern rotated (▓▒░ → ▒░▓) - creates wave effect
    Alternate,
}

/// Raw frame definition from JSON (supports both formats)
#[derive(Debug, Clone, Deserialize)]
struct FrameRaw {
    // New pattern+mode format
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default)]
    mode: Option<SuffixMode>,
    // Legacy explicit format
    #[serde(default)]
    prefix: Option<String>,
    #[serde(default)]
    suffix: Option<String>,
    // Common fields
    #[serde(default)]
    description: Option<String>,
    contexts: Vec<EvalContext>,
    #[serde(default)]
    aliases: Vec<String>,
}

/// A frame definition with prefix/suffix
#[derive(Debug, Clone, Serialize)]
pub struct Frame {
    pub prefix: String,
    pub suffix: String,
    #[serde(default)]
    pub description: Option<String>,
    pub contexts: Vec<EvalContext>,
    #[serde(default)]
    pub aliases: Vec<String>,
}

impl<'de> Deserialize<'de> for Frame {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = FrameRaw::deserialize(deserializer)?;

        let (prefix, suffix) = if let Some(pattern) = raw.pattern {
            // New format: generate prefix/suffix from pattern + mode
            let mode = raw.mode.unwrap_or(SuffixMode::Mirror);
            let reversed: String = pattern.chars().rev().collect();
            // Rotate pattern by 1 position for alternate mode
            let chars: Vec<char> = pattern.chars().collect();
            let rotated: String = if chars.len() > 1 {
                chars[1..].iter().chain(chars[..1].iter()).collect()
            } else {
                pattern.clone()
            };

            match mode {
                SuffixMode::Mirror => (format!("{} ", pattern), format!(" {}", reversed)),
                SuffixMode::Repeat => (format!("{} ", pattern), format!(" {}", pattern)),
                SuffixMode::PrefixOnly => (pattern, String::new()),
                SuffixMode::SuffixOnly => (String::new(), pattern),
                SuffixMode::Alternate => (format!("{} ", pattern), format!(" {}", rotated)),
            }
        } else {
            // Legacy format: use explicit prefix/suffix
            (
                raw.prefix.unwrap_or_default(),
                raw.suffix.unwrap_or_default(),
            )
        };

        Ok(Frame {
            prefix,
            suffix,
            description: raw.description,
            contexts: raw.contexts,
            aliases: raw.aliases,
        })
    }
}

/// Character support information for styles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleSupports {
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

/// A Unicode text transformation style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub unicode_block: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub contexts: Vec<EvalContext>,
    pub supports: StyleSupports,
    pub mappings: HashMap<String, String>,
}

/// Shield style definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldStyle {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub default: bool,
}

/// All renderables in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Renderables {
    pub glyphs: HashMap<String, String>,
    pub components: HashMap<String, Component>,
    pub frames: HashMap<String, Frame>,
    pub styles: HashMap<String, Style>,
}

/// Registry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMetadata {
    pub total_glyphs: usize,
    pub total_components: usize,
    pub total_frames: usize,
    pub total_styles: usize,
    pub total_palette_colors: usize,
    pub total_shield_styles: usize,
    pub last_updated: String,
}

/// The unified registry containing all mdfx data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryData {
    pub version: String,
    pub schema_version: String,
    #[serde(default)]
    pub description: Option<String>,
    pub palette: HashMap<String, String>,
    pub shield_styles: HashMap<String, ShieldStyle>,
    pub renderables: Renderables,
    pub metadata: RegistryMetadata,
}

/// The Registry provides access to all mdfx renderables and configuration
pub struct Registry {
    data: RegistryData,
    // Lookup caches for aliases
    style_aliases: HashMap<String, String>,
    frame_aliases: HashMap<String, String>,
    shield_style_aliases: HashMap<String, String>,
}

impl Registry {
    /// Load the registry from the embedded JSON data
    pub fn new() -> Result<Self> {
        let json_data = include_str!("../data/registry.json");
        Self::from_json(json_data)
    }

    /// Load the registry from a JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        let data: RegistryData = serde_json::from_str(json)?;

        // Build alias lookup tables
        let mut style_aliases = HashMap::new();
        for (id, style) in &data.renderables.styles {
            for alias in &style.aliases {
                style_aliases.insert(alias.clone(), id.clone());
            }
        }

        let mut frame_aliases = HashMap::new();
        for (id, frame) in &data.renderables.frames {
            for alias in &frame.aliases {
                frame_aliases.insert(alias.clone(), id.clone());
            }
        }

        let mut shield_style_aliases = HashMap::new();
        for (id, style) in &data.shield_styles {
            for alias in &style.aliases {
                shield_style_aliases.insert(alias.clone(), id.clone());
            }
        }

        Ok(Registry {
            data,
            style_aliases,
            frame_aliases,
            shield_style_aliases,
        })
    }

    /// Get the registry version
    pub fn version(&self) -> &str {
        &self.data.version
    }

    /// Get the schema version
    pub fn schema_version(&self) -> &str {
        &self.data.schema_version
    }

    // =========================================================================
    // Palette Operations
    // =========================================================================

    /// Resolve a color name to its hex value
    pub fn resolve_color(&self, name: &str) -> Option<&str> {
        self.data.palette.get(name).map(|s| s.as_str())
    }

    /// Get all palette colors
    pub fn palette(&self) -> &HashMap<String, String> {
        &self.data.palette
    }

    // =========================================================================
    // Glyph Operations (includes separators like dot, arrow, etc.)
    // =========================================================================

    /// Get a glyph by name (e.g., "dot", "block.lower.4", "shade.medium", "quad.1-4")
    pub fn glyph(&self, name: &str) -> Option<&str> {
        self.data.renderables.glyphs.get(name).map(|s| s.as_str())
    }

    /// Get all glyphs
    pub fn glyphs(&self) -> &HashMap<String, String> {
        &self.data.renderables.glyphs
    }

    /// Get a separator by name (alias for glyph lookup, for backward compatibility)
    pub fn separator(&self, name: &str) -> Option<&str> {
        self.glyph(name)
    }

    // =========================================================================
    // Component Operations
    // =========================================================================

    /// Get a component by name
    pub fn component(&self, name: &str) -> Option<&Component> {
        self.data.renderables.components.get(name)
    }

    /// Get all components
    pub fn components(&self) -> &HashMap<String, Component> {
        &self.data.renderables.components
    }

    // =========================================================================
    // Frame Operations
    // =========================================================================

    /// Get a frame by name or alias
    pub fn frame(&self, name: &str) -> Option<&Frame> {
        // Try direct lookup first
        if let Some(frame) = self.data.renderables.frames.get(name) {
            return Some(frame);
        }
        // Try alias lookup
        if let Some(id) = self.frame_aliases.get(name) {
            return self.data.renderables.frames.get(id);
        }
        None
    }

    /// Get all frames
    pub fn frames(&self) -> &HashMap<String, Frame> {
        &self.data.renderables.frames
    }

    /// Apply a frame around text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to frame
    /// * `frame_name` - The frame ID or alias (e.g., "gradient", "solid-left")
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::Registry;
    ///
    /// let registry = Registry::new().unwrap();
    /// let result = registry.apply_frame("Title", "gradient").unwrap();
    /// assert_eq!(result, "▓▒░ Title ░▒▓");
    /// ```
    pub fn apply_frame(&self, text: &str, frame_name: &str) -> Result<String> {
        let frame = self
            .frame(frame_name)
            .ok_or_else(|| Error::UnknownFrame(frame_name.to_string()))?;
        Ok(format!("{}{}{}", frame.prefix, text, frame.suffix))
    }

    // =========================================================================
    // Style Operations
    // =========================================================================

    /// Get a style by name or alias
    pub fn style(&self, name: &str) -> Option<&Style> {
        // Try direct lookup first
        if let Some(style) = self.data.renderables.styles.get(name) {
            return Some(style);
        }
        // Try alias lookup
        if let Some(id) = self.style_aliases.get(name) {
            return self.data.renderables.styles.get(id);
        }
        None
    }

    /// Get all styles
    pub fn styles(&self) -> &HashMap<String, Style> {
        &self.data.renderables.styles
    }

    // =========================================================================
    // Shield Style Operations
    // =========================================================================

    /// Get a shield style by name or alias
    pub fn shield_style(&self, name: &str) -> Option<&ShieldStyle> {
        // Try direct lookup first
        if let Some(style) = self.data.shield_styles.get(name) {
            return Some(style);
        }
        // Try alias lookup
        if let Some(id) = self.shield_style_aliases.get(name) {
            return self.data.shield_styles.get(id);
        }
        None
    }

    /// Get the default shield style
    pub fn default_shield_style(&self) -> &str {
        for (id, style) in &self.data.shield_styles {
            if style.default {
                return id;
            }
        }
        "flat-square" // Fallback default
    }

    /// Get all shield styles
    pub fn shield_styles(&self) -> &HashMap<String, ShieldStyle> {
        &self.data.shield_styles
    }

    // =========================================================================
    // Unified Resolution
    // =========================================================================

    /// Resolve a renderable name using the unified resolution order:
    /// components → literal
    ///
    /// Returns the type of renderable found and its data.
    pub fn resolve(&self, name: &str, context: EvalContext) -> ResolvedRenderable {
        // 1. Check components
        if let Some(component) = self.component(name) {
            if component.contexts.contains(&context)
                || component.contexts.iter().any(|c| c.can_promote_to(context))
            {
                return ResolvedRenderable::Component(component.clone());
            }
        }

        // 2. Treat as literal grapheme cluster
        ResolvedRenderable::Literal(name.to_string())
    }

    /// Get registry metadata
    pub fn metadata(&self) -> &RegistryMetadata {
        &self.data.metadata
    }
}

/// Result of unified resolution
#[derive(Debug, Clone)]
pub enum ResolvedRenderable {
    Component(Component),
    Literal(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_loads() {
        let registry = Registry::new().unwrap();
        assert_eq!(registry.version(), "2.0.0");
    }

    #[test]
    fn test_palette_resolution() {
        let registry = Registry::new().unwrap();
        assert_eq!(registry.resolve_color("accent"), Some("F41C80"));
        assert_eq!(registry.resolve_color("success"), Some("22C55E"));
        assert_eq!(registry.resolve_color("nonexistent"), None);
    }

    #[test]
    fn test_separator_lookup() {
        let registry = Registry::new().unwrap();

        // Valid separator
        let result = registry.separator("dot");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "·");

        // Unknown separator
        let result = registry.separator("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_style_lookup() {
        let registry = Registry::new().unwrap();

        // Direct lookup
        let style = registry.style("mathbold");
        assert!(style.is_some());
        assert_eq!(style.unwrap().name, "Mathematical Bold");

        // Alias lookup
        let style = registry.style("mb");
        assert!(style.is_some());
        assert_eq!(style.unwrap().id, "mathbold");
    }

    #[test]
    fn test_frame_lookup() {
        let registry = Registry::new().unwrap();

        // Direct lookup
        let frame = registry.frame("gradient");
        assert!(frame.is_some());
        assert_eq!(frame.unwrap().prefix, "▓▒░ ");

        // Alias lookup
        let frame = registry.frame("grad");
        assert!(frame.is_some());
        assert_eq!(frame.unwrap().prefix, "▓▒░ ");
    }

    #[test]
    fn test_frame_alternate_mode() {
        let registry = Registry::new().unwrap();

        // Alternate mode: suffix is pattern rotated by 1 (▓▒░ → ▒░▓)
        let frame = registry.frame("gradient-wave").unwrap();
        assert_eq!(frame.prefix, "▓▒░ ");
        assert_eq!(frame.suffix, " ▒░▓");
    }

    #[test]
    fn test_component_contexts() {
        let registry = Registry::new().unwrap();

        // Swatch works in both inline and block
        let component = registry.component("swatch").unwrap();
        assert!(component.contexts.contains(&EvalContext::Block));
        assert!(component.contexts.contains(&EvalContext::Inline));

        // Tech works in both inline and block
        let component = registry.component("tech").unwrap();
        assert!(component.contexts.contains(&EvalContext::Block));
        assert!(component.contexts.contains(&EvalContext::Inline));
    }

    #[test]
    fn test_unified_resolution() {
        let registry = Registry::new().unwrap();

        // Component resolution
        match registry.resolve("swatch", EvalContext::Block) {
            ResolvedRenderable::Component(c) => assert_eq!(c.component_type, "native"),
            _ => panic!("Expected component"),
        }

        // Literal fallback
        match registry.resolve("→", EvalContext::Inline) {
            ResolvedRenderable::Literal(s) => assert_eq!(s, "→"),
            _ => panic!("Expected literal"),
        }
    }

    #[test]
    fn test_context_promotion() {
        // Inline can promote to anything
        assert!(EvalContext::Inline.can_promote_to(EvalContext::Block));
        assert!(EvalContext::Inline.can_promote_to(EvalContext::FrameChrome));

        // FrameChrome can promote to anything
        assert!(EvalContext::FrameChrome.can_promote_to(EvalContext::Inline));
        assert!(EvalContext::FrameChrome.can_promote_to(EvalContext::Block));

        // Block cannot promote to more restrictive
        assert!(!EvalContext::Block.can_promote_to(EvalContext::Inline));
        assert!(!EvalContext::Block.can_promote_to(EvalContext::FrameChrome));
    }

    #[test]
    fn test_shield_styles() {
        let registry = Registry::new().unwrap();

        let style = registry.shield_style("flat-square").unwrap();
        assert!(style.default);

        let style = registry.shield_style("square").unwrap(); // alias
        assert_eq!(style.id, "flat-square");

        assert_eq!(registry.default_shield_style(), "flat-square");
    }

    #[test]
    fn test_metadata() {
        let registry = Registry::new().unwrap();
        let meta = registry.metadata();

        assert!(meta.total_glyphs > 0);
        assert!(meta.total_styles > 0);
        assert!(meta.total_frames > 0);
    }

    #[test]
    fn test_schema_version() {
        let registry = Registry::new().unwrap();
        let schema = registry.schema_version();
        // Should have a valid schema version string
        assert!(!schema.is_empty());
    }

    #[test]
    fn test_palette() {
        let registry = Registry::new().unwrap();
        let palette = registry.palette();

        // Should have palette colors
        assert!(!palette.is_empty());
        // Should contain known colors
        assert!(palette.contains_key("accent"));
    }

    #[test]
    fn test_glyphs() {
        let registry = Registry::new().unwrap();
        let glyphs = registry.glyphs();

        // Should have glyphs
        assert!(!glyphs.is_empty());
    }

    #[test]
    fn test_glyph() {
        let registry = Registry::new().unwrap();

        // Test getting a known glyph
        let dot = registry.glyph("dot");
        assert!(dot.is_some());

        // Test getting unknown glyph returns None
        let unknown = registry.glyph("nonexistent_glyph_xyz");
        assert!(unknown.is_none());
    }

    #[test]
    fn test_components() {
        let registry = Registry::new().unwrap();
        let components = registry.components();

        // Should have components
        assert!(!components.is_empty());
    }

    #[test]
    fn test_frames() {
        let registry = Registry::new().unwrap();
        let frames = registry.frames();

        // Should have frames
        assert!(!frames.is_empty());
    }

    #[test]
    fn test_styles() {
        let registry = Registry::new().unwrap();
        let styles = registry.styles();

        // Should have styles
        assert!(!styles.is_empty());
    }

    #[test]
    fn test_shield_styles_collection() {
        let registry = Registry::new().unwrap();
        let shield_styles = registry.shield_styles();

        // Should have shield styles
        assert!(!shield_styles.is_empty());
        // Should contain flat-square (default)
        assert!(shield_styles.contains_key("flat-square"));
    }

    #[test]
    fn test_from_json_valid() {
        // Get the embedded JSON and verify it loads
        let json_data = include_str!("../data/registry.json");
        let registry = Registry::from_json(json_data);
        assert!(registry.is_ok());
    }

    #[test]
    fn test_from_json_invalid() {
        let result = Registry::from_json("{ invalid json }");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_json_empty() {
        let result = Registry::from_json("");
        assert!(result.is_err());
    }
}
