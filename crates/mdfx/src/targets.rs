//! Target Abstraction for Multi-Surface Rendering
//!
//! Targets define rendering destinations with specific capabilities and constraints.
//! Same source markdown compiles to different targets with appropriate optimizations.
//!
//! Shipped targets:
//! - `GitHubTarget`: shields.io badges, no HTML
//! - `LocalDocsTarget`: SVG files, offline-first
//! - `NpmTarget`: Similar to GitHub
//! - `GitLabTarget`: More HTML support, Mermaid diagrams
//! - `PyPITarget`: Plain text fallbacks, ASCII-safe

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Backend types for rendering primitives
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendType {
    /// shields.io URL-based badges (GitHub, npm)
    #[default]
    Shields,
    /// Local SVG file generation (local docs)
    Svg,
    /// Plain text fallback (PyPI, ASCII-only contexts)
    PlainText,
}

/// Target trait defines a rendering destination with specific capabilities
pub trait Target: Send + Sync {
    /// Target identifier (e.g., "github", "gitlab", "pypi")
    fn name(&self) -> &str;

    /// Does this target support raw HTML in markdown?
    fn supports_html(&self) -> bool;

    /// Does this target support embedded SVG?
    fn supports_svg_embed(&self) -> bool;

    /// Does this target support external image URLs?
    fn supports_external_images(&self) -> bool;

    /// Max line length (None = unlimited)
    fn max_line_length(&self) -> Option<usize>;

    /// Preferred backend for this target
    fn preferred_backend(&self) -> BackendType;

    /// Does this target support Unicode styling?
    fn supports_unicode_styling(&self) -> bool {
        true // Most targets support Unicode
    }

    /// Target-specific post-processing
    fn post_process(&self, markdown: &str) -> Result<String> {
        Ok(markdown.to_string()) // Default: no-op
    }

    /// Get a description of the target
    fn description(&self) -> &str {
        "No description available"
    }
}

// =============================================================================
// GitHub Target
// =============================================================================

/// GitHub rendering target
///
/// Characteristics:
/// - Markdown flavor: GitHub-Flavored Markdown (GFM)
/// - HTML support: Very limited (no `<style>`, no `<script>`)
/// - Image support: External URLs, embedded SVGs
/// - Unicode: Full support
/// - Blockquotes: Special syntax for callouts (`> [!NOTE]`)
#[derive(Debug, Clone, Copy, Default)]
pub struct GitHubTarget;

impl Target for GitHubTarget {
    fn name(&self) -> &str {
        "github"
    }

    fn supports_html(&self) -> bool {
        false // GitHub strips most HTML
    }

    fn supports_svg_embed(&self) -> bool {
        true // Can embed SVG files
    }

    fn supports_external_images(&self) -> bool {
        true // shields.io URLs work
    }

    fn max_line_length(&self) -> Option<usize> {
        None // No hard limit
    }

    fn preferred_backend(&self) -> BackendType {
        BackendType::Shields // Use shields.io by default
    }

    fn description(&self) -> &str {
        "GitHub-Flavored Markdown with shields.io badges"
    }

    fn post_process(&self, markdown: &str) -> Result<String> {
        // GitHub-specific: Convert callouts to GitHub alert syntax if needed
        let output = convert_callouts_to_github_alerts(markdown);
        Ok(output)
    }
}

/// Convert mdfx callouts to GitHub's native alert syntax
fn convert_callouts_to_github_alerts(markdown: &str) -> String {
    markdown
        .replace("> 🟢 **Note**", "> [!NOTE]")
        .replace("> ⚠️ **Warning**", "> [!WARNING]")
        .replace("> 🔴 **Error**", "> [!CAUTION]")
        .replace("> 🟡 **Tip**", "> [!TIP]")
        .replace("> ℹ️ **Info**", "> [!NOTE]")
}

// =============================================================================
// Local Docs Target
// =============================================================================

/// Local documentation target (MkDocs, Docusaurus, etc.)
///
/// Characteristics:
/// - Markdown flavor: Any (depends on generator)
/// - HTML support: Full (can customize CSS)
/// - Image support: Local files preferred (offline-first)
/// - Unicode: Full support
///
/// Optimizations:
/// - Use SVG backend (no external dependencies)
/// - Deterministic asset names (version control friendly)
/// - Manifest for asset management
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalDocsTarget;

impl Target for LocalDocsTarget {
    fn name(&self) -> &str {
        "local"
    }

    fn supports_html(&self) -> bool {
        true // Local docs can use full HTML
    }

    fn supports_svg_embed(&self) -> bool {
        true
    }

    fn supports_external_images(&self) -> bool {
        false // Prefer local assets (offline-first)
    }

    fn max_line_length(&self) -> Option<usize> {
        None
    }

    fn preferred_backend(&self) -> BackendType {
        BackendType::Svg // Generate local SVG files
    }

    fn description(&self) -> &str {
        "Local documentation with SVG assets (offline-first)"
    }
}

// =============================================================================
// npm Target
// =============================================================================

/// npm package README target
///
/// Very similar to GitHub - npm READMEs are displayed similarly.
#[derive(Debug, Clone, Copy, Default)]
pub struct NpmTarget;

impl Target for NpmTarget {
    fn name(&self) -> &str {
        "npm"
    }

    fn supports_html(&self) -> bool {
        false // npm README doesn't support HTML
    }

    fn supports_svg_embed(&self) -> bool {
        true // Can embed SVGs
    }

    fn supports_external_images(&self) -> bool {
        true // External URLs work
    }

    fn max_line_length(&self) -> Option<usize> {
        None
    }

    fn preferred_backend(&self) -> BackendType {
        BackendType::Shields // Same as GitHub
    }

    fn description(&self) -> &str {
        "npm package README (similar to GitHub)"
    }
}

// =============================================================================
// GitLab Target
// =============================================================================

/// GitLab rendering target
///
/// Characteristics:
/// - Markdown flavor: GitLab-Flavored Markdown
/// - HTML support: More permissive than GitHub
/// - Image support: External URLs, embedded SVGs
/// - Special features: Mermaid diagrams, issue/MR references
/// - Unicode: Full support
#[derive(Debug, Clone, Copy, Default)]
pub struct GitLabTarget;

impl Target for GitLabTarget {
    fn name(&self) -> &str {
        "gitlab"
    }

    fn supports_html(&self) -> bool {
        true // GitLab allows more HTML than GitHub
    }

    fn supports_svg_embed(&self) -> bool {
        true
    }

    fn supports_external_images(&self) -> bool {
        true
    }

    fn max_line_length(&self) -> Option<usize> {
        None
    }

    fn preferred_backend(&self) -> BackendType {
        BackendType::Shields
    }

    fn description(&self) -> &str {
        "GitLab-Flavored Markdown with extended HTML support"
    }

    fn post_process(&self, markdown: &str) -> Result<String> {
        // GitLab-specific transformations
        let output = convert_callouts_to_gitlab_alerts(markdown);
        Ok(output)
    }
}

/// Convert mdfx callouts to GitLab's alert syntax
fn convert_callouts_to_gitlab_alerts(markdown: &str) -> String {
    // GitLab uses similar blockquote alerts but with different syntax
    markdown
        .replace("> 🟢 **Note**", "> **Note**")
        .replace("> ⚠️ **Warning**", "> **Warning**")
        .replace("> 🔴 **Error**", "> **Danger**")
        .replace("> 🟡 **Tip**", "> **Tip**")
        .replace("> ℹ️ **Info**", "> **Info**")
}

// =============================================================================
// PyPI Target
// =============================================================================

/// PyPI package description target
///
/// Characteristics:
/// - Markdown flavor: Limited Markdown or reStructuredText
/// - HTML support: None (stripped)
/// - Image support: External URLs only (no embed)
/// - Unicode: Limited (some clients are ASCII-only)
///
/// Optimizations:
/// - Plain text fallbacks for styled text
/// - External shields.io URLs (if images allowed)
/// - ASCII alternatives for Unicode decorations
#[derive(Debug, Clone, Copy, Default)]
pub struct PyPITarget;

impl Target for PyPITarget {
    fn name(&self) -> &str {
        "pypi"
    }

    fn supports_html(&self) -> bool {
        false // PyPI strips HTML in long_description
    }

    fn supports_svg_embed(&self) -> bool {
        false // No SVG embed support
    }

    fn supports_external_images(&self) -> bool {
        true // Can link to external images
    }

    fn max_line_length(&self) -> Option<usize> {
        Some(80) // PyPI recommends 80-char lines
    }

    fn preferred_backend(&self) -> BackendType {
        BackendType::PlainText // Fallback to text for maximum compatibility
    }

    fn supports_unicode_styling(&self) -> bool {
        false // PyPI often displays in ASCII-only contexts
    }

    fn description(&self) -> &str {
        "PyPI package description (plain text fallbacks)"
    }

    fn post_process(&self, markdown: &str) -> Result<String> {
        // PyPI-specific: Convert fancy Unicode to ASCII equivalents
        let output = convert_unicode_to_ascii(markdown);
        Ok(output)
    }
}

/// Convert Unicode decorations to ASCII equivalents for PyPI
fn convert_unicode_to_ascii(markdown: &str) -> String {
    markdown
        // Convert common Unicode arrows and bullets
        .replace("→", "->")
        .replace("←", "<-")
        .replace("↔", "<->")
        .replace("•", "*")
        .replace("·", ".")
        // Convert box drawing to ASCII
        .replace("─", "-")
        .replace("│", "|")
        .replace("┌", "+")
        .replace("┐", "+")
        .replace("└", "+")
        .replace("┘", "+")
        .replace("├", "+")
        .replace("┤", "+")
        .replace("┬", "+")
        .replace("┴", "+")
        .replace("┼", "+")
        // Convert gradient frame chars
        .replace("▓", "#")
        .replace("▒", "=")
        .replace("░", "-")
        // Convert block chars
        .replace("█", "#")
        .replace("▌", "|")
        // Keep emoji indicators but add text
        .replace("🟢", "[OK]")
        .replace("🟡", "[WARN]")
        .replace("🔴", "[ERR]")
        .replace("⚠️", "[!]")
        .replace("ℹ️", "[i]")
}

// =============================================================================
// Target Registry
// =============================================================================

/// Get a target by name
pub fn get_target(name: &str) -> Option<Box<dyn Target>> {
    match name.to_lowercase().as_str() {
        "github" => Some(Box::new(GitHubTarget)),
        "local" => Some(Box::new(LocalDocsTarget)),
        "npm" => Some(Box::new(NpmTarget)),
        "gitlab" => Some(Box::new(GitLabTarget)),
        "pypi" => Some(Box::new(PyPITarget)),
        _ => None,
    }
}

/// Get the default target (GitHub)
pub fn default_target() -> Box<dyn Target> {
    Box::new(GitHubTarget)
}

/// List all available target names
pub fn available_targets() -> Vec<&'static str> {
    vec!["github", "local", "npm", "gitlab", "pypi"]
}

/// Detect target from output path
pub fn detect_target_from_path(path: &std::path::Path) -> Option<&'static str> {
    // Check filename
    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        match filename {
            "README.md" => return Some("github"),
            "PKG-INFO" | "PKG-INFO.md" => return Some("pypi"),
            "package.json" => return Some("npm"),
            _ => {}
        }
    }

    // Check parent directory names
    for ancestor in path.ancestors() {
        if let Some(name) = ancestor.file_name().and_then(|f| f.to_str()) {
            if name == "docs" || name == "documentation" {
                return Some("local");
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_target() {
        let target = GitHubTarget;
        assert_eq!(target.name(), "github");
        assert!(!target.supports_html());
        assert!(target.supports_svg_embed());
        assert!(target.supports_external_images());
        assert!(target.supports_unicode_styling());
        assert_eq!(target.preferred_backend(), BackendType::Shields);
    }

    #[test]
    fn test_local_target() {
        let target = LocalDocsTarget;
        assert_eq!(target.name(), "local");
        assert!(target.supports_html());
        assert!(target.supports_svg_embed());
        assert!(!target.supports_external_images()); // Offline-first
        assert_eq!(target.preferred_backend(), BackendType::Svg);
    }

    #[test]
    fn test_npm_target() {
        let target = NpmTarget;
        assert_eq!(target.name(), "npm");
        assert!(!target.supports_html());
        assert!(target.supports_external_images());
        assert_eq!(target.preferred_backend(), BackendType::Shields);
    }

    #[test]
    fn test_gitlab_target() {
        let target = GitLabTarget;
        assert_eq!(target.name(), "gitlab");
        assert!(target.supports_html()); // GitLab allows more HTML
        assert!(target.supports_svg_embed());
        assert!(target.supports_external_images());
        assert!(target.supports_unicode_styling());
        assert_eq!(target.preferred_backend(), BackendType::Shields);
    }

    #[test]
    fn test_pypi_target() {
        let target = PyPITarget;
        assert_eq!(target.name(), "pypi");
        assert!(!target.supports_html());
        assert!(!target.supports_svg_embed()); // No SVG embed
        assert!(target.supports_external_images());
        assert!(!target.supports_unicode_styling()); // ASCII-safe
        assert_eq!(target.preferred_backend(), BackendType::PlainText);
        assert_eq!(target.max_line_length(), Some(80));
    }

    #[test]
    fn test_get_target() {
        assert!(get_target("github").is_some());
        assert!(get_target("GitHub").is_some()); // Case insensitive
        assert!(get_target("local").is_some());
        assert!(get_target("npm").is_some());
        assert!(get_target("gitlab").is_some());
        assert!(get_target("pypi").is_some());
        assert!(get_target("unknown").is_none());
    }

    #[test]
    fn test_available_targets() {
        let targets = available_targets();
        assert!(targets.contains(&"github"));
        assert!(targets.contains(&"local"));
        assert!(targets.contains(&"npm"));
        assert!(targets.contains(&"gitlab"));
        assert!(targets.contains(&"pypi"));
    }

    #[test]
    fn test_detect_target_from_path() {
        use std::path::Path;

        assert_eq!(
            detect_target_from_path(Path::new("README.md")),
            Some("github")
        );
        assert_eq!(
            detect_target_from_path(Path::new("/project/README.md")),
            Some("github")
        );
        assert_eq!(
            detect_target_from_path(Path::new("/project/docs/index.md")),
            Some("local")
        );
        assert_eq!(detect_target_from_path(Path::new("PKG-INFO")), Some("pypi"));
        assert_eq!(detect_target_from_path(Path::new("random.md")), None);
    }

    #[test]
    fn test_github_post_process() {
        let target = GitHubTarget;
        let input = "> 🟢 **Note**\n> This is a note";
        let output = target.post_process(input).unwrap();
        assert!(output.contains("[!NOTE]"));
    }

    #[test]
    fn test_gitlab_post_process() {
        let target = GitLabTarget;
        let input = "> 🔴 **Error**\n> This is an error";
        let output = target.post_process(input).unwrap();
        assert!(output.contains("**Danger**"));
    }

    #[test]
    fn test_pypi_post_process() {
        let target = PyPITarget;
        let input = "▓▒░ Title ░▒▓ → Next";
        let output = target.post_process(input).unwrap();
        assert!(output.contains("#=-"));
        assert!(output.contains("->"));
        assert!(!output.contains("→"));
    }

    #[test]
    fn test_pypi_emoji_conversion() {
        let target = PyPITarget;
        let input = "🟢 Success 🟡 Warning 🔴 Error";
        let output = target.post_process(input).unwrap();
        assert!(output.contains("[OK]"));
        assert!(output.contains("[WARN]"));
        assert!(output.contains("[ERR]"));
    }

    #[test]
    fn test_default_target() {
        let target = default_target();
        assert_eq!(target.name(), "github");
    }
}
