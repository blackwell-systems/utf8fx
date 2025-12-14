//! Unified Registry for mdfx
//!
//! This module provides a single source of truth for all renderables, styles,
//! and configuration. It consolidates the previously fragmented data files
//! (styles.json, frames.json, separators.json, components.json, palette.json,
//! badges.json, shields.json) into one unified registry.
//!
//! The registry enables:
//! - IntelliSense/editor tooling support via single JSON schema
//! - Context-aware validation (inline vs block vs frame_chrome)
//! - Unified resolution pipeline for all renderables
//! - Single source of truth for palette colors

use crate::error::Result;
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

/// A frame definition with prefix/suffix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub prefix: String,
    pub suffix: String,
    #[serde(default)]
    pub description: Option<String>,
    pub contexts: Vec<EvalContext>,
    #[serde(default)]
    pub aliases: Vec<String>,
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

/// A badge type (number/letter Unicode badges)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub contexts: Vec<EvalContext>,
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
    pub components: HashMap<String, Component>,
    pub frames: HashMap<String, Frame>,
    pub styles: HashMap<String, Style>,
    pub badges: HashMap<String, Badge>,
}

/// Registry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMetadata {
    pub total_components: usize,
    pub total_frames: usize,
    pub total_styles: usize,
    pub total_badges: usize,
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
    badge_aliases: HashMap<String, String>,
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

        let mut badge_aliases = HashMap::new();
        for (id, badge) in &data.renderables.badges {
            for alias in &badge.aliases {
                badge_aliases.insert(alias.clone(), id.clone());
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
            badge_aliases,
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
    // Badge Operations
    // =========================================================================

    /// Get a badge by name or alias
    pub fn badge(&self, name: &str) -> Option<&Badge> {
        // Try direct lookup first
        if let Some(badge) = self.data.renderables.badges.get(name) {
            return Some(badge);
        }
        // Try alias lookup
        if let Some(id) = self.badge_aliases.get(name) {
            return self.data.renderables.badges.get(id);
        }
        None
    }

    /// Get all badges
    pub fn badges(&self) -> &HashMap<String, Badge> {
        &self.data.renderables.badges
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
    fn test_component_contexts() {
        let registry = Registry::new().unwrap();

        // Divider is block-only
        let component = registry.component("divider").unwrap();
        assert!(component.contexts.contains(&EvalContext::Block));
        assert!(!component.contexts.contains(&EvalContext::Inline));

        // Swatch works in both inline and block
        let component = registry.component("swatch").unwrap();
        assert!(component.contexts.contains(&EvalContext::Block));
        assert!(component.contexts.contains(&EvalContext::Inline));
    }

    #[test]
    fn test_unified_resolution() {
        let registry = Registry::new().unwrap();

        // Component resolution
        match registry.resolve("divider", EvalContext::Block) {
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
    fn test_badge_lookup() {
        let registry = Registry::new().unwrap();

        let badge = registry.badge("circle").unwrap();
        assert_eq!(badge.mappings.get("1"), Some(&"①".to_string()));

        // Alias lookup
        let badge = registry.badge("circled").unwrap();
        assert_eq!(badge.id, "circle");
    }

    #[test]
    fn test_metadata() {
        let registry = Registry::new().unwrap();
        let meta = registry.metadata();

        assert!(meta.total_styles > 0);
        assert!(meta.total_frames > 0);
        assert!(meta.total_components > 0);
    }
}
