use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when using utf8fx
#[derive(Error, Debug)]
pub enum Error {
    /// The requested style does not exist
    #[error("Unknown style '{0}'. Run `utf8fx list` to see available styles.")]
    UnknownStyle(String),

    /// Style doesn't support the requested character type
    #[error("Style '{0}' does not support {1}")]
    StyleNotSupported(String, String),

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

    /// Regex error
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    /// UTF-8 encoding error
    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

/// Result type for utf8fx operations
pub type Result<T> = std::result::Result<T, Error>;
