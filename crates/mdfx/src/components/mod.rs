//! Components renderer for high-level UI elements
//!
//! ComponentsRenderer provides a semantic layer on top of primitives (shields, frames, badges).
//! Components are defined in `registry.json` and expand to primitive templates at parse time.
//!
//! This allows users to write concise, semantic markup like `{{ui:swatch:cobalt/}}` instead of
//! verbose primitive calls like `{{shields:block:color=...}}`.

mod handlers;
pub mod params;

#[cfg(feature = "fetch")]
pub use handlers::FetchContext;

use crate::error::{Error, Result};
use crate::primitive::Primitive;
use serde::Deserialize;
use std::collections::HashMap;

/// Components renderer for high-level UI elements
pub struct ComponentsRenderer {
    palette: HashMap<String, String>,
    components: HashMap<String, ComponentDef>,
    #[cfg(feature = "fetch")]
    fetch_ctx: Option<handlers::FetchContext>,
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
    /// Row layout with HTML wrapper (applied AFTER recursive parsing)
    /// Converts markdown images to HTML img tags and wraps in `<p align="...">`
    #[serde(skip)]
    Row { align: String },
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
#[allow(clippy::large_enum_variant)]
pub enum ComponentOutput {
    /// Direct primitive rendering (for image-based components)
    Primitive(Primitive),
    /// Template string for recursive parsing (for components using frames/styles)
    Template(String),
    /// Template with post-processing applied AFTER recursive parsing
    /// Used for components like `row` that need to transform rendered output
    TemplateDelayed {
        template: String,
        post_process: PostProcess,
    },
}

impl ComponentsRenderer {
    /// Create a new components renderer by loading from registry.json
    pub fn new() -> Result<Self> {
        let data = include_str!("../../data/registry.json");
        let registry: RegistryComponentsExtract = serde_json::from_str(data).map_err(|e| {
            Error::ParseError(format!(
                "Failed to parse registry.json for components: {}",
                e
            ))
        })?;

        Ok(ComponentsRenderer {
            palette: registry.palette,
            components: registry.renderables.components,
            #[cfg(feature = "fetch")]
            fetch_ctx: None,
        })
    }

    /// Set the fetch context for dynamic badges
    #[cfg(feature = "fetch")]
    pub fn set_fetch_context(&mut self, ctx: handlers::FetchContext) {
        self.fetch_ctx = Some(ctx);
    }

    /// Check if fetch context is available
    #[cfg(feature = "fetch")]
    pub fn has_fetch_context(&self) -> bool {
        self.fetch_ctx.is_some()
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
    /// * `component` - Component name (e.g., "swatch", "tech", "status")
    /// * `args` - Positional arguments (e.g., ["rust"] for tech:rust)
    /// * `content` - Optional content between tags (for non-self-closing components)
    ///
    /// # Returns
    ///
    /// Returns `ComponentOutput::Primitive` for image-based components (swatch, tech).
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::{ComponentsRenderer, ComponentOutput};
    ///
    /// let renderer = ComponentsRenderer::new().unwrap();
    ///
    /// // Swatch returns a Primitive (shields.io badge)
    /// let result = renderer.expand("swatch", &["cobalt".to_string()], None).unwrap();
    /// assert!(matches!(result, ComponentOutput::Primitive(_)));
    ///
    /// // Tech badge also returns a Primitive
    /// let result = renderer.expand("tech", &["rust".to_string()], None).unwrap();
    /// assert!(matches!(result, ComponentOutput::Primitive(_)));
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
            #[cfg(feature = "fetch")]
            "dynamic" => {
                // Dynamic components require fetch context
                self.expand_dynamic(component, args, content)
            }
            #[cfg(not(feature = "fetch"))]
            "dynamic" => Err(Error::ParseError(format!(
                "Dynamic component '{}' requires the 'fetch' feature. Rebuild with: --features fetch",
                component
            ))),
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

    /// Expand a native component to a Primitive (or TemplateDelayed for row)
    fn expand_native(
        &self,
        component: &str,
        args: &[String],
        content: Option<&str>,
    ) -> Result<ComponentOutput> {
        let (args, style) = Self::split_style_arg(args);
        let (positional, params) = Self::extract_params(&args);

        // Create a closure for color resolution
        let resolve = |color: &str| self.resolve_color(color);

        match component {
            "swatch" => handlers::swatch::handle(&positional, &params, &style, resolve),
            "tech" => handlers::tech::handle(&positional, &params, &style, resolve),
            "tech-group" => handlers::tech_group::handle(&params, content),
            "progress" => handlers::progress::handle(&positional, &params, resolve),
            "donut" => handlers::donut::handle(&positional, &params, resolve),
            "gauge" => handlers::gauge::handle(&positional, &params, resolve),
            "sparkline" => handlers::sparkline::handle(&positional, &params, resolve),
            "rating" => handlers::rating::handle(&positional, &params, resolve),
            "waveform" => handlers::waveform::handle(&positional, &params, resolve),
            "row" => handlers::row::handle(&params, content),
            "version" => handlers::version::handle(&positional, &params, &style, resolve),
            "license" => handlers::license::handle(&positional, &params, &style, resolve),
            _ => Err(Error::ParseError(format!(
                "Native component '{}' has no implementation",
                component
            ))),
        }
    }

    /// Expand a dynamic component that fetches data from external APIs
    #[cfg(feature = "fetch")]
    fn expand_dynamic(
        &self,
        component: &str,
        args: &[String],
        _content: Option<&str>,
    ) -> Result<ComponentOutput> {
        let fetch_ctx = self.fetch_ctx.as_ref().ok_or_else(|| {
            Error::ParseError(
                "Dynamic badges require fetch context. Use --offline=false or configure fetch."
                    .to_string(),
            )
        })?;

        let (args, style) = Self::split_style_arg(args);
        let (positional, params) = Self::extract_params(&args);

        // Create a closure for color resolution
        let resolve = |color: &str| self.resolve_color(color);

        match component {
            "live" => {
                // live:source:query:metric - source is first arg
                if positional.is_empty() {
                    return Err(Error::ParseError(
                        "live component requires source (github, npm, crates, pypi)".to_string(),
                    ));
                }
                let source = positional[0].as_str();
                let remaining_args: Vec<String> = positional[1..].to_vec();

                match source {
                    "github" => handlers::handle_github(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    "npm" => {
                        handlers::handle_npm(&remaining_args, &params, &style, resolve, fetch_ctx)
                    }
                    "crates" => handlers::handle_crates(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    "pypi" => {
                        handlers::handle_pypi(&remaining_args, &params, &style, resolve, fetch_ctx)
                    }
                    "codecov" => handlers::handle_codecov(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    "actions" => handlers::handle_actions(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    "docker" => handlers::handle_docker(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    "packagist" => handlers::handle_packagist(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    "rubygems" => handlers::handle_rubygems(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    "nuget" => handlers::handle_nuget(
                        &remaining_args,
                        &params,
                        &style,
                        resolve,
                        fetch_ctx,
                    ),
                    _ => Err(Error::ParseError(format!(
                        "Unknown live source '{}'. Available: github, npm, crates, pypi, codecov, actions, docker, packagist, rubygems, nuget",
                        source
                    ))),
                }
            }
            _ => Err(Error::ParseError(format!(
                "Dynamic component '{}' has no implementation",
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
        // Note: Row is handled as delayed post-processing in the parser,
        // so it shouldn't appear here. Include for exhaustiveness.
        let processed = match &comp.post_process {
            PostProcess::None => expanded,
            PostProcess::Blockquote => self.apply_blockquote(&expanded),
            PostProcess::Row { .. } => expanded, // Delayed; handled in parser
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

    /// Apply row formatting (wrap in HTML with alignment)
    ///
    /// Delegates to the row handler module.
    pub fn apply_row(content: &str, align: &str) -> String {
        handlers::row::apply_row(content, align)
    }

    /// Resolve a color from palette or pass through
    ///
    /// # Arguments
    ///
    /// * `color` - Color name (e.g., "cobalt", "success") or hex code
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
    use rstest::rstest;

    #[test]
    fn test_components_renderer_new() {
        let renderer = ComponentsRenderer::new();
        assert!(renderer.is_ok());
    }

    // ========================================================================
    // Swatch Color Resolution (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("pink", "F41C80")]
    #[case("cobalt", "2B6CB0")]
    #[case("dark1", "292A2D")]
    #[case("abc123", "abc123")] // hex passthrough
    #[case("FF0000", "FF0000")] // hex passthrough
    fn test_expand_swatch_colors(#[case] input: &str, #[case] expected: &str) {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("swatch", &[input.to_string()], None)
            .unwrap();

        let ComponentOutput::Primitive(Primitive::Swatch { color, .. }) = result else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(color, expected);
    }

    #[test]
    fn test_expand_swatch_with_opacity() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand(
                "swatch",
                &["pink".to_string(), "opacity=0.5".to_string()],
                None,
            )
            .unwrap();

        let ComponentOutput::Primitive(Primitive::Swatch { color, opacity, .. }) = result else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(color, "F41C80");
        assert_eq!(opacity, Some(0.5));
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

        let ComponentOutput::Primitive(Primitive::Swatch {
            color,
            width,
            height,
            ..
        }) = result
        else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(color, "2B6CB0"); // cobalt resolved
        assert_eq!(width, Some(40));
        assert_eq!(height, Some(30));
    }

    #[test]
    fn test_expand_swatch_with_border() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand(
                "swatch",
                &[
                    "pink".to_string(),
                    "border=white".to_string(),
                    "border_width=2".to_string(),
                ],
                None,
            )
            .unwrap();

        let ComponentOutput::Primitive(Primitive::Swatch {
            border_color,
            border_width,
            ..
        }) = result
        else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(border_color, Some("FFFFFF".to_string()));
        assert_eq!(border_width, Some(2));
    }

    #[test]
    fn test_expand_swatch_with_label() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand(
                "swatch",
                &["pink".to_string(), "label=v1".to_string()],
                None,
            )
            .unwrap();

        let ComponentOutput::Primitive(Primitive::Swatch { label, .. }) = result else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(label, Some("v1".to_string()));
    }

    #[test]
    fn test_expand_swatch_opacity_clamped() {
        let renderer = ComponentsRenderer::new().unwrap();
        // Test opacity > 1.0 gets clamped
        let result = renderer
            .expand(
                "swatch",
                &["pink".to_string(), "opacity=1.5".to_string()],
                None,
            )
            .unwrap();

        let ComponentOutput::Primitive(Primitive::Swatch { opacity, .. }) = result else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(opacity, Some(1.0)); // clamped to 1.0
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

        let ComponentOutput::Primitive(Primitive::Swatch { color, .. }) = result else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(color, "FF5500");
    }

    #[test]
    fn test_extend_palette_override() {
        let mut renderer = ComponentsRenderer::new().unwrap();

        // Override built-in pink color
        let mut custom = HashMap::new();
        custom.insert("pink".to_string(), "00FF00".to_string());
        renderer.extend_palette(custom);

        let result = renderer
            .expand("swatch", &["pink".to_string()], None)
            .unwrap();

        let ComponentOutput::Primitive(Primitive::Swatch { color, .. }) = result else {
            unreachable!("Expected Primitive::Swatch");
        };
        assert_eq!(color, "00FF00"); // overridden, not F41C80
    }

    #[test]
    fn test_expand_tech() {
        let renderer = ComponentsRenderer::new().unwrap();
        let result = renderer
            .expand("tech", &["rust".to_string()], None)
            .unwrap();

        // Tech should return a Primitive::Tech with brand color
        let ComponentOutput::Primitive(Primitive::Tech(cfg)) = result else {
            unreachable!("Expected Primitive::Tech");
        };
        assert_eq!(cfg.name, "rust");
        assert_eq!(cfg.bg_color, "DEA584"); // Rust brand color
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

    // ========================================================================
    // Component Availability (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("swatch", true)]
    #[case("tech", true)]
    #[case("row", true)]
    #[case("tech-group", true)]
    #[case("nonexistent", false)]
    #[case("unknown-component", false)]
    fn test_has_component(#[case] name: &str, #[case] expected: bool) {
        let renderer = ComponentsRenderer::new().unwrap();
        assert_eq!(renderer.has(name), expected);
    }

    #[test]
    fn test_list_components() {
        let renderer = ComponentsRenderer::new().unwrap();
        let components = renderer.list();

        assert!(!components.is_empty());
        assert!(components.iter().any(|(name, _)| *name == "swatch"));
        assert!(components.iter().any(|(name, _)| *name == "tech"));
    }

    #[test]
    fn test_list_palette() {
        let renderer = ComponentsRenderer::new().unwrap();
        let colors = renderer.list_palette();

        assert!(!colors.is_empty());
        assert!(colors.iter().any(|(name, _)| *name == "pink"));
        assert!(colors.iter().any(|(name, _)| *name == "dark1"));
    }

    // ========================================================================
    // Color Resolution (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("pink", "F41C80")]
    #[case("cobalt", "2B6CB0")]
    #[case("dark1", "292A2D")]
    #[case("abc123", "abc123")] // hex passthrough
    #[case("FF0000", "FF0000")] // hex passthrough
    fn test_resolve_color(#[case] input: &str, #[case] expected: &str) {
        let renderer = ComponentsRenderer::new().unwrap();
        assert_eq!(renderer.resolve_color(input), expected);
    }

    // ========================================================================
    // Blockquote Post-Processor Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("Single line", "> Single line")]
    #[case("Line 1\nLine 2\nLine 3", "> Line 1\n> Line 2\n> Line 3")]
    #[case("Line 1\n\nLine 3", "> Line 1\n>\n> Line 3")] // empty lines
    #[case(
        "Normal\n  Indented\n    More indented",
        "> Normal\n>   Indented\n>     More indented"
    )]
    #[case(
        "**Bold text**\n- List item\n- Another item",
        "> **Bold text**\n> - List item\n> - Another item"
    )]
    #[case("", "")] // empty input
    #[case("Line 1\n   \nLine 3", "> Line 1\n>\n> Line 3")] // whitespace-only lines
    fn test_apply_blockquote(#[case] input: &str, #[case] expected: &str) {
        let renderer = ComponentsRenderer::new().unwrap();
        assert_eq!(renderer.apply_blockquote(input), expected);
    }
}

#[cfg(test)]
mod style_tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Style Argument Splitting (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(&["F41C80", "style=flat"], "flat")]
    #[case(&["F41C80"], "flat-square")] // default
    #[case(&["abc123", "style=plastic"], "plastic")]
    #[case(&["color", "width=10", "style=social"], "social")]
    fn test_split_style_arg(#[case] args: &[&str], #[case] expected_style: &str) {
        let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        let (_, style) = ComponentsRenderer::split_style_arg(&args);
        assert_eq!(style, expected_style);
    }

    // ========================================================================
    // Component Lookup (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("swatch", true)]
    #[case("tech", true)]
    #[case("row", true)]
    #[case("nonexistent_component", false)]
    #[case("unknown", false)]
    fn test_get_component(#[case] name: &str, #[case] should_exist: bool) {
        let renderer = ComponentsRenderer::new().unwrap();
        let component = renderer.get(name);
        assert_eq!(component.is_some(), should_exist);
    }

    // ========================================================================
    // Row Layout (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("  Hello   World  ", "center", "Hello World", "<p align=\"center\">")]
    #[case(
        "![alt text](image.png)",
        "left",
        "<img alt=\"alt text\" src=\"image.png\">",
        "<p align=\"left\">"
    )]
    #[case(
        "![](image.png)",
        "right",
        "<img alt=\"\" src=\"image.png\">",
        "<p align=\"right\">"
    )]
    #[case(
        "Line1\n\n\nLine2    Line3",
        "center",
        "Line1 Line2 Line3",
        "<p align=\"center\">"
    )]
    fn test_apply_row(
        #[case] input: &str,
        #[case] align: &str,
        #[case] expected_content: &str,
        #[case] expected_wrapper: &str,
    ) {
        let result = ComponentsRenderer::apply_row(input, align);
        assert!(result.contains(expected_content));
        assert!(result.contains(expected_wrapper));
        assert!(result.contains("</p>"));
    }

    #[test]
    fn test_apply_row_multiple_images() {
        // Test multiple images (complex case)
        let result = ComponentsRenderer::apply_row("![a](1.png) ![b](2.png)", "center");
        assert!(result.contains("<img alt=\"a\" src=\"1.png\">"));
        assert!(result.contains("<img alt=\"b\" src=\"2.png\">"));
    }
}
