//! # mdfx
//!
//! Markdown effects: Unicode text styling and UI components through template syntax.
//!
//! mdfx is a library and CLI tool for transforming markdown with Unicode text effects,
//! UI components (dividers, badges, status indicators), and visual frames. Process
//! markdown files with intuitive template syntax for consistent, maintainable styling.
//!
//! ## Quick Start
//!
//! ```
//! use mdfx::Converter;
//!
//! let converter = Converter::new().unwrap();
//! let result = converter.convert("HELLO", "mathbold").unwrap();
//! assert_eq!(result, "𝐇𝐄𝐋𝐋𝐎");
//! ```
//!
//! ## Available Styles
//!
//! - `mathbold` - Mathematical Bold (𝐀𝐁𝐂)
//! - `fullwidth` - Full-Width (ＡＢＣ)
//! - `negative-squared` - Negative Squared (🅰🅱🅲)
//! - `negative-circled` - Negative Circled (🅐🅑🅒)
//! - `squared-latin` - Squared Latin (🄰🄱🄲)
//! - `small-caps` - Small Caps (ᴀʙᴄ)
//! - `monospace` - Monospace (𝙰𝙱𝙲)
//! - `double-struck` - Double-Struck (𝔸𝔹ℂ)
//! - `sans-serif-bold` - Sans-Serif Bold (𝗔𝗕𝗖)
//! - `italic` - Italic (𝐴𝐵𝐶)
//! - `bold-italic` - Bold Italic (𝑨𝑩𝑪)
//!
//! ## Features
//!
//! - Convert text to 11 different Unicode styles
//! - Style aliases for shorter names (e.g., `mb` for `mathbold`)
//! - Preserves whitespace, punctuation, and unsupported characters
//! - Zero-copy operations for maximum performance
//! - Comprehensive error handling

pub mod badges;
pub mod components;
pub mod converter;
pub mod error;
pub mod frames;
pub mod manifest;
pub mod parser;
pub mod primitive;
pub mod registry;
pub mod renderer;
pub mod shields;
pub mod styles;
pub mod targets;

// Re-export main types for convenience
pub use badges::{BadgeRenderer, BadgeType};
pub use components::{ComponentDef, ComponentOutput, ComponentsRenderer, PostProcess};
pub use converter::Converter;
pub use error::{Error, Result};
pub use frames::{FrameRenderer, FrameStyle};
pub use manifest::{AssetEntry, AssetManifest, PrimitiveInfo, VerificationResult};
pub use parser::{ProcessedMarkdown, TemplateParser};
pub use primitive::Primitive;
pub use registry::{EvalContext, Registry, ResolvedRenderable};
pub use renderer::plaintext::PlainTextBackend;
pub use renderer::{RenderedAsset, Renderer};
pub use shields::ShieldStyle as ShieldsShieldStyle;
pub use shields::ShieldsRenderer;
pub use styles::{Style, StyleCategory, StyleSupport, StylesData};
pub use targets::{
    available_targets, default_target, detect_target_from_path, get_target, BackendType,
    GitHubTarget, LocalDocsTarget, NpmTarget, Target,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("Test", "mathbold").unwrap();
        assert_eq!(result, "𝐓𝐞𝐬𝐭");
    }
}
