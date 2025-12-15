use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when using mdfx
#[derive(Error, Debug)]
pub enum Error {
    /// The requested style does not exist
    #[error("Unknown style '{0}'. Run `mdfx list` to see available styles.")]
    UnknownStyle(String),

    /// The requested frame does not exist
    #[error("Unknown frame '{0}'. Run `mdfx frames list` to see available frames.")]
    UnknownFrame(String),

    /// The requested badge does not exist
    #[error("Unknown badge '{0}'. Run `mdfx badges list` to see available badges.")]
    UnknownBadge(String),

    /// The requested glyph does not exist
    #[error("Unknown glyph '{0}'. Available glyphs: block.*, shade.*, quad.*")]
    UnknownGlyph(String),

    /// Badge doesn't support the requested character
    #[error("Badge '{0}' does not support '{1}'. Check badge charset limits.")]
    UnsupportedChar(String, String),

    /// The requested shield style does not exist
    #[error("Unknown shield style '{0}'. Run `mdfx shields list` to see available styles.")]
    UnknownShieldStyle(String),

    /// Invalid color specification
    #[error("Invalid color '{0}'. Use 6-digit hex codes (e.g., 2B6CB0) or palette names (e.g., cobalt).")]
    InvalidColor(String),

    /// Unknown shield type in template
    #[error("Unknown shield type '{0}'. Available types: block, twotone, bar, icon")]
    UnknownShieldType(String),

    /// Missing required parameter for shield
    #[error("Missing required parameter '{0}' for shield type '{1}'")]
    MissingShieldParam(String, String),

    /// Generic parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Template syntax error - unclosed tag
    #[error("Unclosed tag: {{{{{0}}}}} (expected {{{{{0}}}}})")]
    UnclosedTag(String),

    /// Template syntax error - mismatched tags
    #[error("Mismatched tags: expected {{{{{0}}}}}, found {{{{{1}}}}}")]
    MismatchedTags(String, String),

    /// Invalid style name in template
    #[error("Invalid style name '{0}': must contain only alphanumeric characters and hyphens")]
    InvalidStyleName(String),

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error
    #[error("Failed to parse styles.json: {0}")]
    InvalidJson(#[from] serde_json::Error),

    /// UTF-8 encoding error
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

/// Result type for mdfx operations
pub type Result<T> = std::result::Result<T, Error>;
