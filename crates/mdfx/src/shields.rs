use crate::error::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;

/// Shields renderer for generating shields.io badge Markdown
///
/// ShieldsRenderer creates shields.io badge URLs as Markdown image links.
/// These badges serve as design primitives: dividers, progress bars, headers, icons.
///
/// Unlike styles (character transformation) or frames (prefix/suffix),
/// shields generate external image links that render as visual elements in Markdown.
pub struct ShieldsRenderer {
    palette: HashMap<String, String>,
    styles: HashMap<String, ShieldStyle>,
}

/// A shield style definition
#[derive(Debug, Clone, Deserialize)]
pub struct ShieldStyle {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// Intermediate structure to parse registry.json for shields
#[derive(Debug, Deserialize)]
struct RegistryShieldsExtract {
    palette: HashMap<String, String>,
    shield_styles: HashMap<String, ShieldStyle>,
}

impl ShieldsRenderer {
    /// Create a new shields renderer by loading from registry.json
    pub fn new() -> Result<Self> {
        let data = include_str!("../data/registry.json");
        let registry: RegistryShieldsExtract = serde_json::from_str(data).map_err(|e| {
            Error::ParseError(format!("Failed to parse registry.json for shields: {}", e))
        })?;

        Ok(ShieldsRenderer {
            palette: registry.palette,
            styles: registry.shield_styles,
        })
    }

    /// Render a single color block
    ///
    /// # Arguments
    ///
    /// * `color` - Palette name or hex code (e.g., "cobalt" or "2B6CB0")
    /// * `style` - Shield style ("flat-square", "for-the-badge", etc.)
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let result = renderer.render_block("2B6CB0", "flat-square").unwrap();
    /// assert!(result.contains("https://img.shields.io/badge/"));
    /// ```
    pub fn render_block(&self, color: &str, style: &str) -> Result<String> {
        let resolved_color = self.resolve_color(color)?;
        let resolved_style = self.resolve_style(style)?;

        let url = format!(
            "https://img.shields.io/badge/-%20-{}?style={}",
            resolved_color, resolved_style
        );

        Ok(format!("![]({})", url))
    }

    /// Render a two-tone block (left/right colors)
    ///
    /// # Arguments
    ///
    /// * `left_color` - Left side color (palette name or hex)
    /// * `right_color` - Right side color (palette name or hex)
    /// * `style` - Shield style
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let result = renderer.render_twotone("111111", "2B6CB0", "flat-square").unwrap();
    /// assert!(result.contains("labelColor=111111"));
    /// assert!(result.contains("-2B6CB0?"));  // Right color appears in badge path
    /// ```
    pub fn render_twotone(
        &self,
        left_color: &str,
        right_color: &str,
        style: &str,
    ) -> Result<String> {
        let left = self.resolve_color(left_color)?;
        let right = self.resolve_color(right_color)?;
        let resolved_style = self.resolve_style(style)?;

        let url = format!(
            "https://img.shields.io/badge/-%20-{}?style={}&label=&labelColor={}",
            right, resolved_style, left
        );

        Ok(format!("![]({})", url))
    }

    /// Render a bar of multiple colored blocks
    ///
    /// # Arguments
    ///
    /// * `colors` - Slice of colors (palette names or hex codes)
    /// * `style` - Shield style
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let colors = vec!["22C55E".to_string(), "F59E0B".to_string()];
    /// let result = renderer.render_bar(&colors, "flat-square").unwrap();
    /// assert!(result.contains("22C55E"));
    /// assert!(result.contains("F59E0B"));
    /// ```
    pub fn render_bar(&self, colors: &[String], style: &str) -> Result<String> {
        let resolved_style = self.resolve_style(style)?;
        let mut blocks = Vec::new();

        for color in colors {
            let resolved_color = self.resolve_color(color)?;
            let url = format!(
                "https://img.shields.io/badge/-%20-{}?style={}",
                resolved_color, resolved_style
            );
            blocks.push(format!("![]({})", url));
        }

        Ok(blocks.join(""))
    }

    /// Render an icon chip (logo-only badge)
    ///
    /// # Arguments
    ///
    /// * `logo` - Simple Icons slug (e.g., "rust", "python", "amazonaws")
    /// * `bg_color` - Background color (palette name or hex)
    /// * `logo_color` - Logo color (palette name or hex)
    /// * `style` - Shield style
    ///
    /// # Examples
    ///
    /// ```
    /// use mdfx::ShieldsRenderer;
    ///
    /// let renderer = ShieldsRenderer::new().unwrap();
    /// let result = renderer.render_icon("rust", "000000", "white", "flat-square").unwrap();
    /// assert!(result.contains("logo=rust"));
    /// assert!(result.contains("logoColor="));
    /// ```
    pub fn render_icon(
        &self,
        logo: &str,
        bg_color: &str,
        logo_color: &str,
        style: &str,
    ) -> Result<String> {
        let bg = self.resolve_color(bg_color)?;
        let logo_col = self.resolve_color(logo_color)?;
        let resolved_style = self.resolve_style(style)?;

        let url = format!(
            "https://img.shields.io/badge/-%20-{}?style={}&logo={}&logoColor={}&label=&labelColor={}",
            bg, resolved_style, logo, logo_col, bg
        );

        Ok(format!("![]({})", url))
    }

    /// Resolve a color from palette name or pass through hex code
    ///
    /// # Arguments
    ///
    /// * `color` - Palette name (e.g., "cobalt") or hex code (e.g., "2B6CB0")
    ///
    /// # Returns
    ///
    /// Hex code (6 characters, no #)
    pub fn resolve_color(&self, color: &str) -> Result<String> {
        // Try palette lookup first
        if let Some(hex) = self.palette.get(color) {
            return Ok(hex.clone());
        }

        // Validate as hex code (must be 6 hex digits)
        if color.len() == 6 && color.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(color.to_uppercase());
        }

        Err(Error::InvalidColor(format!(
            "{} (use palette name or 6-digit hex)",
            color
        )))
    }

    /// Resolve a style by ID or alias
    fn resolve_style(&self, style: &str) -> Result<String> {
        // Try direct lookup
        if self.styles.contains_key(style) {
            return Ok(style.to_string());
        }

        // Try aliases
        for shield_style in self.styles.values() {
            if shield_style.aliases.contains(&style.to_string()) {
                return Ok(shield_style.id.clone());
            }
        }

        Err(Error::UnknownShieldStyle(style.to_string()))
    }

    /// Check if a shield style exists
    pub fn has_style(&self, name: &str) -> bool {
        self.resolve_style(name).is_ok()
    }

    /// List all available shield styles
    pub fn list_styles(&self) -> Vec<&ShieldStyle> {
        let mut styles: Vec<_> = self.styles.values().collect();
        styles.sort_by(|a, b| a.id.cmp(&b.id));
        styles
    }

    /// List all palette colors
    pub fn list_palette(&self) -> Vec<(&String, &String)> {
        let mut colors: Vec<_> = self.palette.iter().collect();
        colors.sort_by(|a, b| a.0.cmp(b.0));
        colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shields_renderer_new() {
        let renderer = ShieldsRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_render_block() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_block("2B6CB0", "flat-square").unwrap();
        assert!(result.contains("https://img.shields.io/badge/"));
        assert!(result.contains("2B6CB0"));
        assert!(result.contains("style=flat-square"));
    }

    #[test]
    fn test_render_block_with_palette_name() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.render_block("cobalt", "flat-square").unwrap();
        assert!(result.contains("2B6CB0"));
    }

    #[test]
    fn test_render_twotone() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_twotone("111111", "2B6CB0", "flat-square")
            .unwrap();
        assert!(result.contains("labelColor=111111"));
        assert!(result.contains("-2B6CB0?")); // Right color appears in badge path
    }

    #[test]
    fn test_render_bar() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = vec![
            "22C55E".to_string(),
            "F59E0B".to_string(),
            "334155".to_string(),
        ];
        let result = renderer.render_bar(&colors, "flat-square").unwrap();
        assert!(result.contains("22C55E"));
        assert!(result.contains("F59E0B"));
        assert!(result.contains("334155"));
        // Should contain 3 separate badges
        assert_eq!(result.matches("![](").count(), 3);
    }

    #[test]
    fn test_render_icon() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer
            .render_icon("rust", "000000", "FFFFFF", "flat-square")
            .unwrap();
        assert!(result.contains("logo=rust"));
        assert!(result.contains("logoColor=FFFFFF"));
        assert!(result.contains("000000"));
    }

    #[test]
    fn test_resolve_color_palette() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.resolve_color("cobalt").unwrap();
        assert_eq!(result, "2B6CB0");
    }

    #[test]
    fn test_resolve_color_hex() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.resolve_color("abc123").unwrap();
        assert_eq!(result, "ABC123");
    }

    #[test]
    fn test_resolve_color_invalid() {
        let renderer = ShieldsRenderer::new().unwrap();
        let result = renderer.resolve_color("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_style_alias() {
        let renderer = ShieldsRenderer::new().unwrap();
        // Test that "square" alias resolves to "flat-square"
        let result = renderer.render_block("cobalt", "square").unwrap();
        assert!(result.contains("style=flat-square"));

        // Test that "flat" is now its own style (not alias)
        let result_flat = renderer.render_block("cobalt", "flat").unwrap();
        assert!(result_flat.contains("style=flat"));
    }

    #[test]
    fn test_has_style() {
        let renderer = ShieldsRenderer::new().unwrap();
        assert!(renderer.has_style("flat-square"));
        assert!(renderer.has_style("flat"));
        assert!(!renderer.has_style("nonexistent"));
    }

    #[test]
    fn test_list_styles() {
        let renderer = ShieldsRenderer::new().unwrap();
        let styles = renderer.list_styles();
        assert!(!styles.is_empty());
    }

    #[test]
    fn test_list_palette() {
        let renderer = ShieldsRenderer::new().unwrap();
        let colors = renderer.list_palette();
        assert!(!colors.is_empty());
        assert!(colors.iter().any(|(name, _)| *name == "cobalt"));
    }
}
