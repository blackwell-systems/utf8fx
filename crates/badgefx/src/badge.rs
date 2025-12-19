//! Core badge structures and builder pattern

use crate::style::{BadgeStyle, Border, Chevron, Corners};

/// Complete specification for a technology badge
#[derive(Debug, Clone)]
pub struct TechBadge {
    /// Technology name (used for icon lookup)
    pub name: String,
    /// Custom label text (defaults to capitalized name)
    pub label: Option<String>,
    /// Visual style of the badge
    pub style: BadgeStyle,
    /// Custom background color (overrides brand color)
    pub bg_color: Option<String>,
    /// Left segment (icon) background color
    pub bg_left: Option<String>,
    /// Right segment (label) background color
    pub bg_right: Option<String>,
    /// Custom logo/icon color
    pub logo_color: Option<String>,
    /// Custom text color
    pub text_color: Option<String>,
    /// Border styling
    pub border: Option<Border>,
    /// Apply border to full badge (both segments) instead of just icon segment
    pub border_full: bool,
    /// Custom corner radii
    pub corners: Option<Corners>,
    /// Chevron/arrow configuration
    pub chevron: Option<Chevron>,
    /// Raised icon effect (pixels above/below label)
    pub raised: Option<u32>,
    /// Logo/icon size in pixels (default: 14)
    pub logo_size: Option<u32>,
    /// Outline mode
    pub outline: bool,
    /// Custom font family
    pub font: Option<String>,
    /// Custom SVG icon path (overrides built-in icons)
    pub custom_icon: Option<String>,
}

impl TechBadge {
    /// Create a new badge with defaults
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: None,
            style: BadgeStyle::default(),
            bg_color: None,
            bg_left: None,
            bg_right: None,
            logo_color: None,
            text_color: None,
            border: None,
            border_full: false,
            corners: None,
            chevron: None,
            raised: None,
            logo_size: None,
            outline: false,
            font: None,
            custom_icon: None,
        }
    }

    /// Get the display label for this badge
    pub fn display_label(&self) -> String {
        match &self.label {
            Some(label) => label.clone(),
            None => {
                // Auto-capitalize common names
                match self.name.to_lowercase().as_str() {
                    "javascript" => "JavaScript".to_string(),
                    "typescript" => "TypeScript".to_string(),
                    "nodejs" | "node.js" => "Node.js".to_string(),
                    "postgresql" => "PostgreSQL".to_string(),
                    "mongodb" => "MongoDB".to_string(),
                    "vuejs" | "vue.js" => "Vue.js".to_string(),
                    name => {
                        // Simple title case for other names
                        let mut chars = name.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get effective background color (brand color or custom)
    pub fn effective_bg_color(&self) -> Option<String> {
        self.bg_color
            .clone()
            .or_else(|| mdfx_icons::brand_color(&self.name).map(|color| format!("#{}", color)))
    }
}

/// Builder for creating customized technology badges
#[derive(Debug)]
pub struct BadgeBuilder {
    badge: TechBadge,
}

impl BadgeBuilder {
    /// Create a new badge builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            badge: TechBadge::new(name),
        }
    }

    /// Set custom label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.badge.label = Some(label.into());
        self
    }

    /// Set badge style
    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.badge.style = style;
        self
    }

    /// Set custom background color
    pub fn bg_color(mut self, color: impl Into<String>) -> Self {
        self.badge.bg_color = Some(color.into());
        self
    }

    /// Set left segment (icon) background color
    pub fn bg_left(mut self, color: impl Into<String>) -> Self {
        self.badge.bg_left = Some(color.into());
        self
    }

    /// Set right segment (label) background color
    pub fn bg_right(mut self, color: impl Into<String>) -> Self {
        self.badge.bg_right = Some(color.into());
        self
    }

    /// Set custom logo/icon color
    pub fn logo_color(mut self, color: impl Into<String>) -> Self {
        self.badge.logo_color = Some(color.into());
        self
    }

    /// Set custom SVG icon path (overrides built-in icons)
    pub fn custom_icon(mut self, path: impl Into<String>) -> Self {
        self.badge.custom_icon = Some(path.into());
        self
    }

    /// Set custom text color
    pub fn text_color(mut self, color: impl Into<String>) -> Self {
        self.badge.text_color = Some(color.into());
        self
    }

    /// Add border styling
    pub fn border(mut self, color: impl Into<String>, width: u32) -> Self {
        self.badge.border = Some(Border::new(color, width));
        self
    }

    /// Apply border to full badge (both segments)
    pub fn border_full(mut self) -> Self {
        self.badge.border_full = true;
        self
    }

    /// Set custom corner radii
    pub fn corners(mut self, corners: Corners) -> Self {
        self.badge.corners = Some(corners);
        self
    }

    /// Add chevron/arrow styling
    pub fn chevron(mut self, chevron: Chevron) -> Self {
        self.badge.chevron = Some(chevron);
        self
    }

    /// Add raised icon effect
    pub fn raised(mut self, pixels: u32) -> Self {
        self.badge.raised = Some(pixels);
        self
    }

    /// Set logo/icon size in pixels
    pub fn logo_size(mut self, size: u32) -> Self {
        self.badge.logo_size = Some(size);
        self
    }

    /// Enable outline mode
    pub fn outline(mut self) -> Self {
        self.badge.outline = true;
        self
    }

    /// Set custom font family
    pub fn font(mut self, font: impl Into<String>) -> Self {
        self.badge.font = Some(font.into());
        self
    }

    /// Build the final badge configuration
    pub fn build(self) -> TechBadge {
        self.badge
    }

    /// Render the badge to SVG string
    pub fn render(self) -> String {
        crate::render::render(&self.build())
    }

    /// Render the badge to a file
    pub fn render_to_file(self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        crate::render::render_to_file(&self.build(), path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_creation() {
        let badge = TechBadge::new("rust");
        assert_eq!(badge.name, "rust");
        assert_eq!(badge.display_label(), "Rust");
    }

    #[test]
    fn test_display_labels() {
        assert_eq!(TechBadge::new("javascript").display_label(), "JavaScript");
        assert_eq!(TechBadge::new("typescript").display_label(), "TypeScript");
        assert_eq!(TechBadge::new("nodejs").display_label(), "Node.js");
        assert_eq!(TechBadge::new("postgresql").display_label(), "PostgreSQL");
        assert_eq!(TechBadge::new("python").display_label(), "Python");
    }

    #[test]
    fn test_custom_label() {
        let badge = TechBadge {
            name: "rust".to_string(),
            label: Some("Rust 1.70".to_string()),
            ..TechBadge::new("rust")
        };
        assert_eq!(badge.display_label(), "Rust 1.70");
    }

    #[test]
    fn test_builder_pattern() {
        let builder = BadgeBuilder::new("typescript")
            .label("TypeScript v5.0")
            .bg_color("#3178C6")
            .outline();

        let badge = builder.build();
        assert_eq!(badge.name, "typescript");
        assert_eq!(badge.label, Some("TypeScript v5.0".to_string()));
        assert_eq!(badge.bg_color, Some("#3178C6".to_string()));
        assert!(badge.outline);
    }

    #[test]
    fn test_effective_bg_color() {
        // Custom color overrides brand color
        let badge = TechBadge {
            bg_color: Some("#FF0000".to_string()),
            ..TechBadge::new("rust")
        };
        assert_eq!(badge.effective_bg_color(), Some("#FF0000".to_string()));

        // Falls back to brand color
        let badge = TechBadge::new("rust");
        assert_eq!(badge.effective_bg_color(), Some("#DEA584".to_string()));

        // No color for unknown tech
        let badge = TechBadge::new("unknown");
        assert_eq!(badge.effective_bg_color(), None);
    }
}
