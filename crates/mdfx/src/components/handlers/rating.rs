//! Rating component handler (stars, hearts, etc.)

use super::{get_string, parse_param_clamped, resolve_color_with_default};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle rating component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "rating component requires a value argument".to_string(),
        ));
    }

    // Parse value (first arg) - can be float like 3.5
    let value: f32 = args[0].parse().map_err(|_| {
        Error::ParseError(format!(
            "Invalid rating value '{}' - must be a number",
            args[0]
        ))
    })?;

    // Max: 1-20, Size: 1-100px, Spacing: 0-50px
    let max: u32 = parse_param_clamped(params, "max", 5, 1, 20);
    let size: u32 = parse_param_clamped(params, "size", 20, 1, 100);
    let spacing: u32 = parse_param_clamped(params, "spacing", 2, 0, 50);

    let fill_color = resolve_color_with_default(params, "fill", "warning", &resolve_color);
    let empty_color = resolve_color_with_default(params, "empty", "gray", &resolve_color);

    let icon = get_string(params, "icon", "star");

    Ok(ComponentOutput::Primitive(Primitive::Rating {
        value,
        max,
        size,
        fill_color,
        empty_color,
        icon,
        spacing,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn identity_color(c: &str) -> String {
        c.to_string()
    }

    // ========================================================================
    // Basic Rating Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("0", 0.0)]
    #[case("3", 3.0)]
    #[case("3.5", 3.5)]
    #[case("4.75", 4.75)]
    #[case("5", 5.0)]
    fn test_handle_value(#[case] input: &str, #[case] expected: f32) {
        let result = handle(&[input.to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Rating { value, .. })) = result {
            assert!((value - expected).abs() < 0.001);
        } else {
            panic!("Expected Rating primitive");
        }
    }

    #[test]
    fn test_handle_missing_args() {
        let result = handle(&[], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_invalid_value() {
        let result = handle(&["abc".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    // ========================================================================
    // Size Parameters (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("max", "10", 10, 20, 2)]
    #[case("size", "30", 5, 30, 2)]
    #[case("spacing", "4", 5, 20, 4)]
    fn test_handle_size_params(
        #[case] key: &str,
        #[case] value: &str,
        #[case] expected_max: u32,
        #[case] expected_size: u32,
        #[case] expected_spacing: u32,
    ) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["4".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Rating {
            max, size, spacing, ..
        })) = result
        {
            assert_eq!(max, expected_max);
            assert_eq!(size, expected_size);
            assert_eq!(spacing, expected_spacing);
        } else {
            panic!("Expected Rating primitive");
        }
    }

    // ========================================================================
    // Color Parameters
    // ========================================================================

    #[test]
    fn test_handle_colors() {
        let mut params = HashMap::new();
        params.insert("fill".to_string(), "FFD700".to_string());
        params.insert("empty".to_string(), "CCCCCC".to_string());

        let result = handle(&["3.5".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Rating {
            fill_color,
            empty_color,
            ..
        })) = result
        {
            assert_eq!(fill_color, "FFD700");
            assert_eq!(empty_color, "CCCCCC");
        } else {
            panic!("Expected Rating primitive");
        }
    }

    // ========================================================================
    // Icon Parameter (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(None, "star")] // default
    #[case(Some("heart"), "heart")]
    #[case(Some("circle"), "circle")]
    fn test_handle_icon(#[case] input: Option<&str>, #[case] expected: &str) {
        let mut params = HashMap::new();
        if let Some(icon) = input {
            params.insert("icon".to_string(), icon.to_string());
        }

        let result = handle(&["4".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Rating { icon, .. })) = result {
            assert_eq!(icon, expected);
        } else {
            panic!("Expected Rating primitive");
        }
    }

    #[test]
    fn test_handle_defaults() {
        let result = handle(&["4".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Rating {
            max,
            size,
            spacing,
            icon,
            ..
        })) = result
        {
            assert_eq!(max, 5);
            assert_eq!(size, 20);
            assert_eq!(spacing, 2);
            assert_eq!(icon, "star");
        } else {
            panic!("Expected Rating primitive");
        }
    }

    // ========================================================================
    // Parameter Clamping Tests
    // ========================================================================

    #[rstest]
    #[case("max", "0", 1)] // below min -> clamped to 1
    #[case("max", "30", 20)] // above max -> clamped to 20
    #[case("size", "0", 1)] // below min -> clamped to 1
    #[case("size", "150", 100)] // above max -> clamped to 100
    #[case("spacing", "60", 50)] // above max -> clamped to 50
    fn test_handle_param_clamping(#[case] key: &str, #[case] value: &str, #[case] expected: u32) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["4".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Rating {
            max, size, spacing, ..
        })) = result
        {
            match key {
                "max" => assert_eq!(max, expected),
                "size" => assert_eq!(size, expected),
                "spacing" => assert_eq!(spacing, expected),
                _ => panic!("Unknown key"),
            }
        } else {
            panic!("Expected Rating primitive");
        }
    }
}
