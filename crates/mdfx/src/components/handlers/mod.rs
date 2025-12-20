//! Component handlers for native component expansion
//!
//! Each handler module implements the logic for expanding a specific
//! native component type into a Primitive or ComponentOutput.

use std::collections::HashMap;
use std::str::FromStr;

/// Parse a parameter with a default value.
///
/// # Example
/// ```ignore
/// let width: u32 = parse_param(&params, "width", 100);
/// ```
#[inline]
pub fn parse_param<T: FromStr>(params: &HashMap<String, String>, key: &str, default: T) -> T {
    params
        .get(key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/// Parse an optional parameter.
///
/// # Example
/// ```ignore
/// let thumb_size: Option<u32> = parse_param_opt(&params, "thumb");
/// ```
#[inline]
pub fn parse_param_opt<T: FromStr>(params: &HashMap<String, String>, key: &str) -> Option<T> {
    params.get(key).and_then(|v| v.parse().ok())
}

/// Parse a boolean parameter (accepts "true" or "1").
///
/// # Example
/// ```ignore
/// let show_label = parse_bool(&params, "label", false);
/// ```
#[inline]
pub fn parse_bool(params: &HashMap<String, String>, key: &str, default: bool) -> bool {
    params
        .get(key)
        .map(|v| v == "true" || v == "1")
        .unwrap_or(default)
}

/// Resolve a color parameter with a default.
///
/// # Example
/// ```ignore
/// let fill = resolve_color_with_default(&params, "fill", "pink", &resolve_color);
/// ```
#[inline]
pub fn resolve_color_with_default(
    params: &HashMap<String, String>,
    key: &str,
    default: &str,
    resolve: impl Fn(&str) -> String,
) -> String {
    params
        .get(key)
        .map(|c| resolve(c))
        .unwrap_or_else(|| resolve(default))
}

/// Resolve a color parameter with fallback keys and a default.
///
/// Tries keys in order, uses default if none found.
///
/// # Example
/// ```ignore
/// let track = resolve_color_with_fallback(&params, &["track", "color"], "gray", &resolve_color);
/// ```
#[inline]
pub fn resolve_color_with_fallback(
    params: &HashMap<String, String>,
    keys: &[&str],
    default: &str,
    resolve: impl Fn(&str) -> String,
) -> String {
    keys.iter()
        .find_map(|k| params.get(*k))
        .map(|c| resolve(c))
        .unwrap_or_else(|| resolve(default))
}

/// Resolve an optional color parameter.
///
/// # Example
/// ```ignore
/// let border_color: Option<String> = resolve_color_opt(&params, "border", &resolve_color);
/// ```
#[inline]
pub fn resolve_color_opt(
    params: &HashMap<String, String>,
    key: &str,
    resolve: impl Fn(&str) -> String,
) -> Option<String> {
    params.get(key).map(|c| resolve(c))
}

/// Get an optional string parameter.
///
/// # Example
/// ```ignore
/// let icon: Option<String> = get_string_opt(&params, "icon");
/// ```
#[inline]
#[allow(dead_code)] // API symmetry with get_string
pub fn get_string_opt(params: &HashMap<String, String>, key: &str) -> Option<String> {
    params.get(key).cloned()
}

/// Get a string parameter with a default.
///
/// # Example
/// ```ignore
/// let icon = get_string(&params, "icon", "star");
/// ```
#[inline]
pub fn get_string(params: &HashMap<String, String>, key: &str, default: &str) -> String {
    params
        .get(key)
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

pub mod donut;
pub mod gauge;
#[cfg(feature = "fetch")]
pub mod github;
pub mod license;
pub mod progress;
pub mod rating;
pub mod row;
pub mod sparkline;
pub mod swatch;
pub mod tech;
pub mod tech_group;
pub mod version;
pub mod waveform;

#[cfg(feature = "fetch")]
pub use github::{
    handle_codecov, handle_crates, handle_github, handle_npm, handle_pypi, FetchContext,
};
