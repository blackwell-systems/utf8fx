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

/// Parse a parameter with clamping to min/max bounds.
///
/// Returns value clamped to [min, max] range.
///
/// # Example
/// ```ignore
/// let width: u32 = parse_param_clamped(&params, "width", 100, 1, 2000);
/// ```
#[inline]
pub fn parse_param_clamped<T: FromStr + Ord + Copy>(
    params: &HashMap<String, String>,
    key: &str,
    default: T,
    min: T,
    max: T,
) -> T {
    let value = parse_param(params, key, default);
    value.clamp(min, max)
}

/// Parse an optional parameter with clamping to min/max bounds.
///
/// Returns Some(value) clamped to [min, max] range, or None if not present.
///
/// # Example
/// ```ignore
/// let thumb_size: Option<u32> = parse_param_opt_clamped(&params, "thumb", 1, 100);
/// ```
#[inline]
pub fn parse_param_opt_clamped<T: FromStr + Ord + Copy>(
    params: &HashMap<String, String>,
    key: &str,
    min: T,
    max: T,
) -> Option<T> {
    parse_param_opt(params, key).map(|v: T| v.clamp(min, max))
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

use crate::primitive::ThumbConfig;

/// Parse thumb configuration from parameters.
///
/// Returns Some(ThumbConfig) if thumb size is specified, None otherwise.
///
/// # Example
/// ```ignore
/// let thumb = parse_thumb_config(&params, &resolve_color);
/// ```
pub fn parse_thumb_config(
    params: &HashMap<String, String>,
    resolve: impl Fn(&str) -> String,
) -> Option<ThumbConfig> {
    // Thumb is only enabled if thumb size is specified
    // Thumb size: 1-100px
    let size: u32 = parse_param_opt_clamped(params, "thumb", 1, 100)?;

    Some(ThumbConfig {
        size,
        // Thumb width: 1-100px (optional)
        width: parse_param_opt_clamped(params, "thumb_width", 1, 100),
        color: resolve_color_opt(params, "thumb_color", &resolve),
        shape: get_string(params, "thumb_shape", "circle"),
        border: resolve_color_opt(params, "thumb_border", &resolve),
        // Thumb border width: 0-10px
        border_width: parse_param_clamped(params, "thumb_border_width", 0, 0, 10),
    })
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
    handle_actions, handle_codecov, handle_crates, handle_docker, handle_github, handle_npm,
    handle_nuget, handle_packagist, handle_pypi, handle_rubygems, FetchContext,
};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn identity_color(c: &str) -> String {
        c.to_string()
    }

    fn uppercase_color(c: &str) -> String {
        c.to_uppercase()
    }

    // ========================================================================
    // parse_param Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("width", "100", 100u32)]
    #[case("width", "200", 200u32)]
    #[case("nonexistent", "100", 50u32)] // uses default
    fn test_parse_param_u32(#[case] key: &str, #[case] value: &str, #[case] expected: u32) {
        let mut params = HashMap::new();
        params.insert("width".to_string(), value.to_string());

        let result = parse_param(&params, key, 50);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("height", "3.5", 3.5f32)]
    #[case("height", "1.0", 1.0f32)]
    fn test_parse_param_f32(#[case] key: &str, #[case] value: &str, #[case] expected: f32) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result: f32 = parse_param(&params, key, 0.0);
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_parse_param_invalid_value() {
        let mut params = HashMap::new();
        params.insert("width".to_string(), "not_a_number".to_string());

        let result: u32 = parse_param(&params, "width", 50);
        assert_eq!(result, 50); // uses default on parse error
    }

    // ========================================================================
    // parse_param_opt Tests
    // ========================================================================

    #[rstest]
    #[case("thumb", "10", Some(10u32))]
    #[case("nonexistent", "10", None)]
    fn test_parse_param_opt(#[case] key: &str, #[case] value: &str, #[case] expected: Option<u32>) {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), value.to_string());

        let result: Option<u32> = parse_param_opt(&params, key);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_param_opt_invalid() {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), "abc".to_string());

        let result: Option<u32> = parse_param_opt(&params, "thumb");
        assert!(result.is_none());
    }

    // ========================================================================
    // parse_bool Tests (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("label", "true", true)]
    #[case("label", "1", true)]
    #[case("label", "false", false)]
    #[case("label", "0", false)]
    #[case("label", "anything", false)] // not "true" or "1"
    #[case("nonexistent", "true", false)] // uses default
    fn test_parse_bool(#[case] key: &str, #[case] value: &str, #[case] expected: bool) {
        let mut params = HashMap::new();
        params.insert("label".to_string(), value.to_string());

        let result = parse_bool(&params, key, false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bool_default_true() {
        let params = HashMap::new();
        let result = parse_bool(&params, "nonexistent", true);
        assert!(result);
    }

    // ========================================================================
    // resolve_color_with_default Tests
    // ========================================================================

    #[test]
    fn test_resolve_color_with_default_found() {
        let mut params = HashMap::new();
        params.insert("fill".to_string(), "FF0000".to_string());

        let result = resolve_color_with_default(&params, "fill", "default", identity_color);
        assert_eq!(result, "FF0000");
    }

    #[test]
    fn test_resolve_color_with_default_not_found() {
        let params = HashMap::new();
        let result = resolve_color_with_default(&params, "fill", "default", identity_color);
        assert_eq!(result, "default");
    }

    #[test]
    fn test_resolve_color_with_default_uses_resolver() {
        let mut params = HashMap::new();
        params.insert("fill".to_string(), "red".to_string());

        let result = resolve_color_with_default(&params, "fill", "default", uppercase_color);
        assert_eq!(result, "RED");
    }

    // ========================================================================
    // resolve_color_with_fallback Tests
    // ========================================================================

    #[test]
    fn test_resolve_color_with_fallback_first_key() {
        let mut params = HashMap::new();
        params.insert("track".to_string(), "AAA".to_string());

        let result =
            resolve_color_with_fallback(&params, &["track", "color"], "default", identity_color);
        assert_eq!(result, "AAA");
    }

    #[test]
    fn test_resolve_color_with_fallback_second_key() {
        let mut params = HashMap::new();
        params.insert("color".to_string(), "BBB".to_string());

        let result =
            resolve_color_with_fallback(&params, &["track", "color"], "default", identity_color);
        assert_eq!(result, "BBB");
    }

    #[test]
    fn test_resolve_color_with_fallback_uses_default() {
        let params = HashMap::new();
        let result =
            resolve_color_with_fallback(&params, &["track", "color"], "default", identity_color);
        assert_eq!(result, "default");
    }

    // ========================================================================
    // resolve_color_opt Tests
    // ========================================================================

    #[rstest]
    #[case("border", Some("000000".to_string()))]
    #[case("nonexistent", None)]
    fn test_resolve_color_opt(#[case] key: &str, #[case] expected: Option<String>) {
        let mut params = HashMap::new();
        params.insert("border".to_string(), "000000".to_string());

        let result = resolve_color_opt(&params, key, identity_color);
        assert_eq!(result, expected);
    }

    // ========================================================================
    // get_string / get_string_opt Tests
    // ========================================================================

    #[rstest]
    #[case("icon", "heart", "heart")]
    #[case("nonexistent", "heart", "star")] // uses default
    fn test_get_string(#[case] key: &str, #[case] value: &str, #[case] expected: &str) {
        let mut params = HashMap::new();
        params.insert("icon".to_string(), value.to_string());

        let result = get_string(&params, key, "star");
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("icon", Some("heart".to_string()))]
    #[case("nonexistent", None)]
    fn test_get_string_opt(#[case] key: &str, #[case] expected: Option<String>) {
        let mut params = HashMap::new();
        params.insert("icon".to_string(), "heart".to_string());

        let result = get_string_opt(&params, key);
        assert_eq!(result, expected);
    }

    // ========================================================================
    // parse_thumb_config Tests
    // ========================================================================

    #[test]
    fn test_parse_thumb_config_none() {
        let params = HashMap::new();
        let result = parse_thumb_config(&params, identity_color);
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_thumb_config_basic() {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), "10".to_string());

        let result = parse_thumb_config(&params, identity_color);
        assert!(result.is_some());
        let thumb = result.unwrap();
        assert_eq!(thumb.size, 10);
        assert!(thumb.width.is_none());
        assert!(thumb.color.is_none());
        assert_eq!(thumb.shape, "circle"); // default
        assert!(thumb.border.is_none());
        assert_eq!(thumb.border_width, 0);
    }

    #[test]
    fn test_parse_thumb_config_full() {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), "12".to_string());
        params.insert("thumb_width".to_string(), "20".to_string());
        params.insert("thumb_color".to_string(), "FFFFFF".to_string());
        params.insert("thumb_shape".to_string(), "square".to_string());
        params.insert("thumb_border".to_string(), "000000".to_string());
        params.insert("thumb_border_width".to_string(), "2".to_string());

        let result = parse_thumb_config(&params, identity_color);
        assert!(result.is_some());
        let thumb = result.unwrap();
        assert_eq!(thumb.size, 12);
        assert_eq!(thumb.width, Some(20));
        assert_eq!(thumb.color, Some("FFFFFF".to_string()));
        assert_eq!(thumb.shape, "square");
        assert_eq!(thumb.border, Some("000000".to_string()));
        assert_eq!(thumb.border_width, 2);
    }

    // ========================================================================
    // parse_param_clamped Tests
    // ========================================================================

    #[rstest]
    #[case("100", 100, 1, 200, 100)] // within bounds
    #[case("0", 50, 1, 200, 1)] // below min -> clamped to min
    #[case("300", 50, 1, 200, 200)] // above max -> clamped to max
    #[case("1", 50, 1, 200, 1)] // at min
    #[case("200", 50, 1, 200, 200)] // at max
    fn test_parse_param_clamped(
        #[case] value: &str,
        #[case] default: u32,
        #[case] min: u32,
        #[case] max: u32,
        #[case] expected: u32,
    ) {
        let mut params = HashMap::new();
        params.insert("width".to_string(), value.to_string());

        let result = parse_param_clamped(&params, "width", default, min, max);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_param_clamped_missing_uses_default() {
        let params = HashMap::new();
        // default of 50 is within bounds [1, 200]
        let result: u32 = parse_param_clamped(&params, "width", 50, 1, 200);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_parse_param_clamped_default_clamped() {
        let params = HashMap::new();
        // default of 300 is above max of 200, should be clamped
        let result: u32 = parse_param_clamped(&params, "width", 300, 1, 200);
        assert_eq!(result, 200);
    }

    // ========================================================================
    // parse_param_opt_clamped Tests
    // ========================================================================

    #[rstest]
    #[case("50", 1, 100, Some(50))] // within bounds
    #[case("0", 1, 100, Some(1))] // below min -> clamped
    #[case("200", 1, 100, Some(100))] // above max -> clamped
    fn test_parse_param_opt_clamped(
        #[case] value: &str,
        #[case] min: u32,
        #[case] max: u32,
        #[case] expected: Option<u32>,
    ) {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), value.to_string());

        let result: Option<u32> = parse_param_opt_clamped(&params, "thumb", min, max);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_param_opt_clamped_missing() {
        let params = HashMap::new();
        let result: Option<u32> = parse_param_opt_clamped(&params, "thumb", 1, 100);
        assert!(result.is_none());
    }

    // ========================================================================
    // parse_thumb_config Clamping Tests
    // ========================================================================

    #[test]
    fn test_parse_thumb_config_clamped_size() {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), "200".to_string()); // above max of 100

        let result = parse_thumb_config(&params, identity_color);
        assert!(result.is_some());
        let thumb = result.unwrap();
        assert_eq!(thumb.size, 100); // clamped to max
    }

    #[test]
    fn test_parse_thumb_config_clamped_border_width() {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), "10".to_string());
        params.insert("thumb_border_width".to_string(), "50".to_string()); // above max of 10

        let result = parse_thumb_config(&params, identity_color);
        assert!(result.is_some());
        let thumb = result.unwrap();
        assert_eq!(thumb.border_width, 10); // clamped to max
    }
}
