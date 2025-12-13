/// Multi-backend rendering system for visual primitives.
///
/// This module defines the trait-based architecture for rendering primitives
/// to different output formats (shields.io URLs, local SVG files, etc.).
pub mod shields;
pub mod svg;

use crate::error::Result;
use crate::primitive::Primitive;

/// Represents the output of rendering a primitive.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderedAsset {
    /// Inline Markdown (e.g., shields.io URL wrapped in ![](url))
    InlineMarkdown(String),

    /// File-based asset (e.g., generated SVG file)
    File {
        /// Relative path to the generated file (e.g., "assets/utf8fx/divider_a3f8e2.svg")
        relative_path: String,
        /// File contents as bytes
        bytes: Vec<u8>,
        /// Markdown reference to embed (e.g., "![](assets/utf8fx/divider_a3f8e2.svg)")
        markdown_ref: String,
    },
}

impl RenderedAsset {
    /// Get the Markdown representation of this asset
    pub fn to_markdown(&self) -> &str {
        match self {
            RenderedAsset::InlineMarkdown(md) => md,
            RenderedAsset::File { markdown_ref, .. } => markdown_ref,
        }
    }

    /// Check if this asset requires a file to be written
    pub fn is_file_based(&self) -> bool {
        matches!(self, RenderedAsset::File { .. })
    }

    /// Get file bytes if this is a file-based asset
    pub fn file_bytes(&self) -> Option<&[u8]> {
        match self {
            RenderedAsset::File { bytes, .. } => Some(bytes),
            _ => None,
        }
    }

    /// Get relative path if this is a file-based asset
    pub fn file_path(&self) -> Option<&str> {
        match self {
            RenderedAsset::File { relative_path, .. } => Some(relative_path),
            _ => None,
        }
    }
}

/// Trait for rendering primitives to output formats.
///
/// Implementations handle backend-specific logic:
/// - ShieldsBackend: Generates shields.io URLs
/// - SvgBackend (future): Generates local SVG files
pub trait Renderer {
    /// Render a primitive to an asset (inline or file-based)
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendered_asset_inline() {
        let asset = RenderedAsset::InlineMarkdown("![](https://example.com)".to_string());
        assert_eq!(asset.to_markdown(), "![](https://example.com)");
        assert!(!asset.is_file_based());
    }

    #[test]
    fn test_rendered_asset_file() {
        let asset = RenderedAsset::File {
            relative_path: "assets/badge.svg".to_string(),
            bytes: b"<svg></svg>".to_vec(),
            markdown_ref: "![](assets/badge.svg)".to_string(),
        };
        assert_eq!(asset.to_markdown(), "![](assets/badge.svg)");
        assert!(asset.is_file_based());
        assert_eq!(asset.file_path(), Some("assets/badge.svg"));
        assert_eq!(asset.file_bytes(), Some(b"<svg></svg>".as_slice()));
    }
}
