use crate::error::{Error, Result};
use crate::primitive::Primitive;
use serde::Deserialize;
use std::collections::HashMap;

/// Components renderer for high-level UI elements
///
/// ComponentsRenderer provides a semantic layer on top of primitives (shields, frames, badges).
/// Components are defined in `components.json` and expand to primitive templates at parse time.
///
/// This allows users to write concise, semantic markup like `{{ui:divider/}}` instead of
/// verbose primitive calls like `{{shields:bar:colors=...}}`.
pub struct ComponentsRenderer {
    palette: HashMap<String, String>,
    components: HashMap<String, ComponentDef>,
}

/// Post-processing operations applied after template expansion
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum PostProcess {
    /// No post-processing (default)
    #[default]
    None,
    /// Prefix every line with "> " for Markdown blockquotes
    Blockquote,
}

/// A component definition from registry.json
#[derive(Debug, Clone, Deserialize)]
pub struct ComponentDef {
    #[serde(rename = "type")]
    pub component_type: String, // "expand" or "native"
    pub self_closing: bool,
    pub description: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub post_process: PostProcess,
}

/// Intermediate structure to parse registry.json for components
#[derive(Debug, Deserialize)]
struct RegistryComponentsExtract {
    palette: HashMap<String, String>,
    renderables: RenderablesExtract,
}

#[derive(Debug, Deserialize)]
struct RenderablesExtract {
    components: HashMap<String, ComponentDef>,
}

/// Output from expanding a component
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentOutput {
    /// Direct primitive rendering (for image-based components)
    Primitive(Primitive),
    /// Template string for recursive parsing (for components using frames/styles)
    Template(String),
}

impl ComponentsRenderer {
    /// Create a new components renderer by loading from registry.json
    pub fn new() -> Result<Self> {
        let data = include_str!("../data/registry.json");
        let registry: RegistryComponentsExtract = serde_json::from_str(data).map_err(|e| {
            Error::ParseError(format!(
                "Failed to parse registry.json for components: {}",
                e
            ))
        })?;

        Ok(ComponentsRenderer {
            palette: registry.palette,
            components: registry.renderables.components,
        })
    }

    /// Extend the palette with custom color definitions
    /// Custom colors override built-in palette colors with the same name
    pub fn extend_palette(&mut self, custom_palette: HashMap<String, String>) {
        for (name, color) in custom_palette {
            self.palette.insert(name, color);
        }
    }

    /// Expand a component into either a Primitive or Template
    ///
    /// # Arguments
    ///
    /// * `component` - Component name (e.g., "divider", "tech", "header")
    /// * `args` - Positional arguments (e.g., ["rust"] for tech:rust)
    /// * `content` - Optional content between tags (for non-self-closing components)
    ///
    /// # Returns
    ///
    /// Either:
    /// - `ComponentOutput::Primitive` for image-based components (divider, swatch, tech, status)
    /// - `ComponentOutput::Template` for components using frames/styles (header, callout)
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::{ComponentsRenderer, ComponentOutput};
    ///
    /// let renderer = ComponentsRenderer::new().unwrap();
    ///
    /// // Swatch returns a Primitive
    /// let result = renderer.expand("swatch", &["accent".to_string()], None).unwrap();
    /// assert!(matches!(result, ComponentOutput::Primitive(_)));
    ///
    /// // Header returns a Template
    /// let result = renderer.expand("header", &[], Some("TITLE")).unwrap();
    /// assert!(matches!(result, ComponentOutput::Template(_)));
    /// ```
    pub fn expand(
        &self,
        component: &str,
        args: &[String],
        content: Option<&str>,
    ) -> Result<ComponentOutput> {
        // Get component definition (single source of truth)
        let comp = self.components.get(component).ok_or_else(|| {
            Error::ParseError(format!(
                "Unknown component '{}'. Run `mdfx components list` to see available components.",
                component
            ))
        })?;

        // Dispatch based on component type from JSON
        match comp.component_type.as_str() {
            "native" => {
                // Native components return Primitives
                self.expand_native(component, args, content)
            }
            "expand" => {
                // Expand components return Templates
                let template = self.expand_template(component, args, content)?;
                Ok(ComponentOutput::Template(template))
            }
            unknown => Err(Error::ParseError(format!(
                "Unknown component type '{}' for component '{}'",
                unknown, component
            ))),
        }
    }

    /// Extract style= parameter from args, returning (remaining_args, style)
    fn split_style_arg(args: &[String]) -> (Vec<String>, String) {
        let mut style: Option<String> = None;
        let mut kept = Vec::new();

        for arg in args {
            if let Some(rest) = arg.strip_prefix("style=") {
                // Last one wins if repeated
                style = Some(rest.to_string());
            } else {
                kept.push(arg.clone());
            }
        }

        (
            kept,
            style.unwrap_or_else(|| Primitive::default_style().to_string()),
        )
    }

    /// Extract key=value parameters from args, returning (positional_args, params_map)
    fn extract_params(args: &[String]) -> (Vec<String>, HashMap<String, String>) {
        let mut params = HashMap::new();
        let mut positional = Vec::new();

        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                params.insert(key.to_string(), value.to_string());
            } else {
                positional.push(arg.clone());
            }
        }

        (positional, params)
    }

    /// Expand a native component to a Primitive
    fn expand_native(
        &self,
        component: &str,
        args: &[String],
        _content: Option<&str>,
    ) -> Result<ComponentOutput> {
        let (args, style) = Self::split_style_arg(args);

        match component {
            "divider" => {
                let colors = vec![
                    self.resolve_color("ui.bg"),
                    self.resolve_color("ui.surface"),
                    self.resolve_color("accent"),
                    self.resolve_color("ui.panel"),
                ];
                Ok(ComponentOutput::Primitive(Primitive::Divider {
                    colors,
                    style,
                }))
            }

            "swatch" => {
                // Extract all key=value params from args
                let (positional, params) = Self::extract_params(&args);

                if positional.is_empty() {
                    return Err(Error::ParseError(
                        "swatch component requires a color argument".to_string(),
                    ));
                }

                let color = self.resolve_color(&positional[0]);

                // Parse optional SVG-only parameters
                let opacity = params
                    .get("opacity")
                    .and_then(|v| v.parse::<f32>().ok())
                    .map(|o| o.clamp(0.0, 1.0));
                let width = params.get("width").and_then(|v| v.parse::<u32>().ok());
                let height = params.get("height").and_then(|v| v.parse::<u32>().ok());
                let border_color = params.get("border").map(|v| self.resolve_color(v));
                let border_width = params
                    .get("border_width")
                    .and_then(|v| v.parse::<u32>().ok());
                let label = params.get("label").cloned();
                let label_color = params.get("label_color").map(|v| self.resolve_color(v));
                let icon = params.get("icon").cloned();
                let icon_color = params.get("icon_color").map(|v| self.resolve_color(v));

                // Style can come from params or use default
                let style = params
                    .get("style")
                    .cloned()
                    .unwrap_or_else(|| style.clone());

                Ok(ComponentOutput::Primitive(Primitive::Swatch {
                    color,
                    style,
                    opacity,
                    width,
                    height,
                    border_color,
                    border_width,
                    label,
                    label_color,
                    icon,
                    icon_color,
                }))
            }

            "tech" => {
                if args.is_empty() {
                    return Err(Error::ParseError(
                        "tech component requires a technology name argument".to_string(),
                    ));
                }
                let name = args[0].clone();
                let bg_color = self.resolve_color("ui.bg");
                let logo_color = self.resolve_color("white");
                Ok(ComponentOutput::Primitive(Primitive::Tech {
                    name,
                    bg_color,
                    logo_color,
                    style,
                }))
            }

            "status" => {
                if args.is_empty() {
                    return Err(Error::ParseError(
                        "status component requires a level argument".to_string(),
                    ));
                }
                let level = self.resolve_color(&args[0]);
                Ok(ComponentOutput::Primitive(Primitive::Status {
                    level,
                    style,
                }))
            }

            _ => Err(Error::ParseError(format!(
                "Native component '{}' has no implementation",
                component
            ))),
        }
    }

    /// Expand a component using template substitution (internal method)
    fn expand_template(
        &self,
        component: &str,
        args: &[String],
        content: Option<&str>,
    ) -> Result<String> {
        let comp = self.components.get(component).ok_or_else(|| {
            Error::ParseError(format!(
                "Unknown component '{}'. Run `mdfx components list` to see available components.",
                component
            ))
        })?;

        // Start with template
        let mut expanded = comp.template.clone();

        // Substitute positional args ($1, $2, ...)
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("${}", i + 1);
            let resolved_arg = self.resolve_color(arg);
            expanded = expanded.replace(&placeholder, &resolved_arg);
        }

        // Substitute content
        if let Some(content_str) = content {
            expanded = expanded.replace("$content", content_str);
        }

        // Resolve any remaining palette references in the template
        expanded = self.resolve_palette_refs(&expanded);

        // Apply post-processing based on component definition
        let processed = match &comp.post_process {
            PostProcess::None => expanded,
            PostProcess::Blockquote => self.apply_blockquote(&expanded),
        };

        Ok(processed)
    }

    /// Apply blockquote formatting (prefix every line with "> ")
    ///
    /// This is used for GitHub-compatible blockquote callouts.
    /// Every line, including empty lines, gets the "> " prefix.
    ///
    /// Each line is prefixed with "> ", and empty lines become ">".
    fn apply_blockquote(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    ">".to_string() // Empty blockquote line
                } else {
                    format!("> {}", line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Resolve a color from palette or pass through
    ///
    /// # Arguments
    ///
    /// * `color` - Color name (e.g., "accent", "ui.bg") or hex code
    ///
    /// # Returns
    ///
    /// Hex code if found in palette, otherwise the original string
    fn resolve_color(&self, color: &str) -> String {
        self.palette
            .get(color)
            .cloned()
            .unwrap_or_else(|| color.to_string())
    }

    /// Resolve all palette references in a template string
    ///
    /// Only replaces palette names in parameter contexts like:
    /// - color=NAME
    /// - colors=NAME,NAME
    /// - bg=NAME
    /// - logoColor=NAME
    /// - labelColor=NAME
    ///
    /// This prevents accidental replacement in content or other contexts.
    fn resolve_palette_refs(&self, template: &str) -> String {
        let mut result = template.to_string();

        // Parameter keys that can contain colors
        let color_params = [
            "color",
            "colors",
            "bg",
            "logoColor",
            "labelColor",
            "label_color",
            "icon_color",
        ];

        for param in &color_params {
            // Find all occurrences of param=value or param=value1,value2,...
            let mut search_pos = 0;
            while let Some(start) = result[search_pos..].find(&format!("{}=", param)) {
                let abs_start = search_pos + start + param.len() + 1; // After "param="

                // Find end of value (next : or / or })
                let remaining = &result[abs_start..];
                let end_chars = [':', '/', '}'];
                let end_pos = remaining
                    .find(|c| end_chars.contains(&c))
                    .unwrap_or(remaining.len());

                let value = &remaining[..end_pos];

                // Replace palette names in this value (comma-separated list)
                let resolved_value = value
                    .split(',')
                    .map(|part| {
                        self.palette
                            .get(part)
                            .cloned()
                            .unwrap_or_else(|| part.to_string())
                    })
                    .collect::<Vec<_>>()
                    .join(",");

                // Replace in result
                let before = &result[..abs_start];
                let after = &result[abs_start + value.len()..];
                result = format!("{}{}{}", before, resolved_value, after);

                // Move search position forward
                search_pos = abs_start + resolved_value.len();
            }
        }

        result
    }

    /// Check if a component exists
    pub fn has(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }

    /// List all available components
    pub fn list(&self) -> Vec<(&String, &ComponentDef)> {
        let mut components: Vec<_> = self.components.iter().collect();
        components.sort_by(|a, b| a.0.cmp(b.0));
        components
    }

    /// List all palette colors
    pub fn list_palette(&self) -> Vec<(&String, &String)> {
        let mut colors: Vec<_> = self.palette.iter().collect();
        colors.sort_by(|a, b| a.0.cmp(b.0));
        colors
    }

    /// Get a component definition
    pub fn get(&self, name: &str) -> Option<&ComponentDef> {
        self.components.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_components_renderer_new() {
        let renderer = ComponentsRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_expand_divider() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer.expand("divider", &[], None).unwrap();

        // Divider should return a Primitive::Divider
        match result {
            ComponentOutput::Primitive(Primitive::Divider { colors, .. }) => {
                assert_eq!(colors.len(), 4);
                assert_eq!(colors[0], "292A2D"); // ui.bg
                assert_eq!(colors[2], "F41C80"); // accent
            }
            _ => panic!("Expected Primitive::Divider"),
        }
    }

    #[test]
    fn test_expand_swatch_with_arg() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("swatch", &["accent".to_string()], None)
            .unwrap();

        // Swatch should return a Primitive::Swatch with resolved color
        match result {
            ComponentOutput::Primitive(Primitive::Swatch { color, .. }) => {
                assert_eq!(color, "F41C80"); // accent resolved
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_expand_swatch_with_hex() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("swatch", &["abc123".to_string()], None)
            .unwrap();

        // Swatch should pass through hex as-is
        match result {
            ComponentOutput::Primitive(Primitive::Swatch { color, .. }) => {
                assert_eq!(color, "abc123");
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_expand_swatch_with_opacity() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand(
                "swatch",
                &["accent".to_string(), "opacity=0.5".to_string()],
                None,
            )
            .unwrap();

        match result {
            ComponentOutput::Primitive(Primitive::Swatch { color, opacity, .. }) => {
                assert_eq!(color, "F41C80");
                assert_eq!(opacity, Some(0.5));
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_expand_swatch_with_dimensions() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand(
                "swatch",
                &[
                    "cobalt".to_string(),
                    "width=40".to_string(),
                    "height=30".to_string(),
                ],
                None,
            )
            .unwrap();

        match result {
            ComponentOutput::Primitive(Primitive::Swatch {
                color,
                width,
                height,
                ..
            }) => {
                assert_eq!(color, "2B6CB0"); // cobalt resolved
                assert_eq!(width, Some(40));
                assert_eq!(height, Some(30));
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_expand_swatch_with_border() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand(
                "swatch",
                &[
                    "accent".to_string(),
                    "border=white".to_string(),
                    "border_width=2".to_string(),
                ],
                None,
            )
            .unwrap();

        match result {
            ComponentOutput::Primitive(Primitive::Swatch {
                border_color,
                border_width,
                ..
            }) => {
                assert_eq!(border_color, Some("FFFFFF".to_string()));
                assert_eq!(border_width, Some(2));
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_expand_swatch_with_label() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand(
                "swatch",
                &["accent".to_string(), "label=v1".to_string()],
                None,
            )
            .unwrap();

        match result {
            ComponentOutput::Primitive(Primitive::Swatch { label, .. }) => {
                assert_eq!(label, Some("v1".to_string()));
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_expand_swatch_opacity_clamped() {
        let renderer = ComponentsRenderer::new().unwrap();
        // Test opacity > 1.0 gets clamped
        let result = renderer
            .expand(
                "swatch",
                &["accent".to_string(), "opacity=1.5".to_string()],
                None,
            )
            .unwrap();

        match result {
            ComponentOutput::Primitive(Primitive::Swatch { opacity, .. }) => {
                assert_eq!(opacity, Some(1.0)); // clamped to 1.0
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_extend_palette() {
        let mut renderer = ComponentsRenderer::new().unwrap();

        // Add custom color
        let mut custom = HashMap::new();
        custom.insert("brand".to_string(), "FF5500".to_string());
        renderer.extend_palette(custom);

        // Use custom color in swatch
        let result = renderer
            .expand("swatch", &["brand".to_string()], None)
            .unwrap();

        match result {
            ComponentOutput::Primitive(Primitive::Swatch { color, .. }) => {
                assert_eq!(color, "FF5500");
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_extend_palette_override() {
        let mut renderer = ComponentsRenderer::new().unwrap();

        // Override built-in accent color
        let mut custom = HashMap::new();
        custom.insert("accent".to_string(), "00FF00".to_string());
        renderer.extend_palette(custom);

        let result = renderer
            .expand("swatch", &["accent".to_string()], None)
            .unwrap();

        match result {
            ComponentOutput::Primitive(Primitive::Swatch { color, .. }) => {
                assert_eq!(color, "00FF00"); // overridden, not F41C80
            }
            _ => panic!("Expected Primitive::Swatch"),
        }
    }

    #[test]
    fn test_expand_tech() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("tech", &["rust".to_string()], None)
            .unwrap();

        // Tech should return a Primitive::Tech
        match result {
            ComponentOutput::Primitive(Primitive::Tech { name, bg_color, .. }) => {
                assert_eq!(name, "rust");
                assert_eq!(bg_color, "292A2D"); // ui.bg resolved
            }
            _ => panic!("Expected Primitive::Tech"),
        }
    }

    #[test]
    fn test_expand_status() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("status", &["success".to_string()], None)
            .unwrap();

        // Status should return a Primitive::Status with resolved color
        match result {
            ComponentOutput::Primitive(Primitive::Status { level, .. }) => {
                assert_eq!(level, "22C55E"); // success → green
            }
            _ => panic!("Expected Primitive::Status"),
        }
    }

    #[test]
    fn test_expand_header_with_content() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("header", &[], Some("INSTALLATION"))
            .unwrap();

        // Header should return a Template for recursive processing
        match result {
            ComponentOutput::Template(template) => {
                assert!(template.contains("INSTALLATION"));
                assert!(template.contains("{{frame:gradient}}"));
                assert!(template.contains("{{mathbold:separator=dot}}"));
            }
            _ => panic!("Expected ComponentOutput::Template"),
        }
    }

    #[test]
    fn test_expand_callout_with_content() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("callout", &["warning".to_string()], Some("Breaking change"))
            .unwrap();

        // Callout should return a Template with substitutions
        match result {
            ComponentOutput::Template(template) => {
                assert!(template.contains("Breaking change"));
                assert!(template.contains("EAB308")); // warning color resolved
            }
            _ => panic!("Expected ComponentOutput::Template"),
        }
    }

    #[test]
    fn test_expand_unknown_component() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer.expand("nonexistent", &[], None);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown component"));
    }

    #[test]
    fn test_has_component() {
        let renderer = ComponentsRenderer::new().unwrap();
        assert!(renderer.has("divider"));
        assert!(renderer.has("tech"));
        assert!(!renderer.has("nonexistent"));
    }

    #[test]
    fn test_list_components() {
        let renderer = ComponentsRenderer::new().unwrap();
        let components = renderer.list();

        assert!(!components.is_empty());
        assert!(components.iter().any(|(name, _)| *name == "divider"));
        assert!(components.iter().any(|(name, _)| *name == "tech"));
    }

    #[test]
    fn test_list_palette() {
        let renderer = ComponentsRenderer::new().unwrap();
        let colors = renderer.list_palette();

        assert!(!colors.is_empty());
        assert!(colors.iter().any(|(name, _)| *name == "accent"));
        assert!(colors.iter().any(|(name, _)| *name == "ui.bg"));
    }

    #[test]
    fn test_resolve_color_palette() {
        let renderer = ComponentsRenderer::new().unwrap();
        let resolved = renderer.resolve_color("accent");
        assert_eq!(resolved, "F41C80");
    }

    #[test]
    fn test_resolve_color_passthrough() {
        let renderer = ComponentsRenderer::new().unwrap();
        let resolved = renderer.resolve_color("abc123");
        assert_eq!(resolved, "abc123");
    }

    // ========================================
    // Blockquote Post-Processor Tests
    // ========================================

    #[test]
    fn test_apply_blockquote_single_line() {
        let renderer = ComponentsRenderer::new().unwrap();
        let input = "Single line";
        let result = renderer.apply_blockquote(input);
        assert_eq!(result, "> Single line");
    }

    #[test]
    fn test_apply_blockquote_multiple_lines() {
        let renderer = ComponentsRenderer::new().unwrap();
        let input = "Line 1\nLine 2\nLine 3";
        let result = renderer.apply_blockquote(input);
        assert_eq!(result, "> Line 1\n> Line 2\n> Line 3");
    }

    #[test]
    fn test_apply_blockquote_with_empty_lines() {
        let renderer = ComponentsRenderer::new().unwrap();
        let input = "Line 1\n\nLine 3";
        let result = renderer.apply_blockquote(input);
        // Empty lines should get ">" (no trailing space)
        assert_eq!(result, "> Line 1\n>\n> Line 3");
    }

    #[test]
    fn test_apply_blockquote_preserves_indentation() {
        let renderer = ComponentsRenderer::new().unwrap();
        let input = "Normal\n  Indented\n    More indented";
        let result = renderer.apply_blockquote(input);
        // Blockquote prefix goes before existing indentation
        assert_eq!(result, "> Normal\n>   Indented\n>     More indented");
    }

    #[test]
    fn test_apply_blockquote_with_markdown() {
        let renderer = ComponentsRenderer::new().unwrap();
        let input = "**Bold text**\n- List item\n- Another item";
        let result = renderer.apply_blockquote(input);
        // Markdown inside blockquote should be preserved
        assert_eq!(result, "> **Bold text**\n> - List item\n> - Another item");
    }

    #[test]
    fn test_apply_blockquote_empty_string() {
        let renderer = ComponentsRenderer::new().unwrap();
        let input = "";
        let result = renderer.apply_blockquote(input);
        // Empty input should return empty (lines() on empty string returns no lines)
        assert_eq!(result, "");
    }

    #[test]
    fn test_apply_blockquote_whitespace_only_lines() {
        let renderer = ComponentsRenderer::new().unwrap();
        let input = "Line 1\n   \nLine 3";
        let result = renderer.apply_blockquote(input);
        // Whitespace-only lines should be treated as empty (trim() is empty)
        assert_eq!(result, "> Line 1\n>\n> Line 3");
    }
}

#[cfg(test)]
mod style_tests {
    use super::*;

    #[test]
    fn test_split_style_arg_with_style() {
        let args = vec!["F41C80".to_string(), "style=flat".to_string()];
        let (remaining, style) = ComponentsRenderer::split_style_arg(&args);
        assert_eq!(remaining, vec!["F41C80"]);
        assert_eq!(style, "flat");
    }

    #[test]
    fn test_split_style_arg_no_style() {
        let args = vec!["F41C80".to_string()];
        let (remaining, style) = ComponentsRenderer::split_style_arg(&args);
        assert_eq!(remaining, vec!["F41C80"]);
        assert_eq!(style, "flat-square");
    }
}
