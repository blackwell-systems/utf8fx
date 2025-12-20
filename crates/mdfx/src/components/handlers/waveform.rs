//! Waveform visualization component handler

use super::{parse_bool, parse_param_clamped, resolve_color_opt, resolve_color_with_fallback};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle waveform component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "waveform component requires values argument".to_string(),
        ));
    }

    // Parse values (comma-separated, can be negative)
    let values: Vec<f32> = args[0]
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if values.is_empty() {
        return Err(Error::ParseError(
            "waveform values must contain at least one number".to_string(),
        ));
    }

    // Width: 1-2000px, Height: 1-500px, Spacing: 0-50px, Bar: 1-50px
    let width: u32 = parse_param_clamped(params, "width", 100, 1, 2000);
    let height: u32 = parse_param_clamped(params, "height", 40, 1, 500);
    let spacing: u32 = parse_param_clamped(params, "spacing", 1, 0, 50);

    // bar_width with "bar" alias, clamped to 1-50px
    let bar_width: u32 = params
        .get("bar_width")
        .or_else(|| params.get("bar"))
        .and_then(|v| v.parse().ok())
        .unwrap_or(3)
        .clamp(1, 50);

    let positive_color =
        resolve_color_with_fallback(params, &["positive", "up"], "success", &resolve_color);
    let negative_color =
        resolve_color_with_fallback(params, &["negative", "down"], "error", &resolve_color);
    let track_color = resolve_color_opt(params, "track", &resolve_color);

    let show_center_line = parse_bool(params, "center", false);
    let center_line_color = resolve_color_opt(params, "center_color", &resolve_color);

    Ok(ComponentOutput::Primitive(Primitive::Waveform {
        values,
        width,
        height,
        positive_color,
        negative_color,
        bar_width,
        spacing,
        track_color,
        show_center_line,
        center_line_color,
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
    // Basic Value Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("1,2,3,4,5", vec![1.0, 2.0, 3.0, 4.0, 5.0])]
    #[case("-1,0,1", vec![-1.0, 0.0, 1.0])] // with negatives
    #[case("0.5,-0.5,1.0", vec![0.5, -0.5, 1.0])] // floats
    #[case("100", vec![100.0])] // single value
    fn test_handle_values(#[case] input: &str, #[case] expected: Vec<f32>) {
        let result = handle(&[input.to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform { values, .. })) = result {
            assert_eq!(values.len(), expected.len());
            for (v, e) in values.iter().zip(expected.iter()) {
                assert!((v - e).abs() < 0.001);
            }
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    #[test]
    fn test_handle_missing_args() {
        let result = handle(&[], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_no_numeric_values() {
        let result = handle(&["abc,def".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    // ========================================================================
    // Size Parameters (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("width", "200", 200, 40, 3, 1)]
    #[case("height", "60", 100, 60, 3, 1)]
    #[case("bar_width", "5", 100, 40, 5, 1)]
    #[case("spacing", "2", 100, 40, 3, 2)]
    fn test_handle_size_params(
        #[case] key: &str,
        #[case] value: &str,
        #[case] expected_width: u32,
        #[case] expected_height: u32,
        #[case] expected_bar_width: u32,
        #[case] expected_spacing: u32,
    ) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["1,-1,2".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform {
            width,
            height,
            bar_width,
            spacing,
            ..
        })) = result
        {
            assert_eq!(width, expected_width);
            assert_eq!(height, expected_height);
            assert_eq!(bar_width, expected_bar_width);
            assert_eq!(spacing, expected_spacing);
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    #[test]
    fn test_handle_bar_alias() {
        // "bar" should work as alias for "bar_width"
        let mut params = HashMap::new();
        params.insert("bar".to_string(), "6".to_string());

        let result = handle(&["1,-1".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform { bar_width, .. })) = result {
            assert_eq!(bar_width, 6);
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    // ========================================================================
    // Color Parameters
    // ========================================================================

    #[test]
    fn test_handle_colors() {
        let mut params = HashMap::new();
        params.insert("positive".to_string(), "00FF00".to_string());
        params.insert("negative".to_string(), "FF0000".to_string());
        params.insert("track".to_string(), "CCCCCC".to_string());

        let result = handle(&["1,-1".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform {
            positive_color,
            negative_color,
            track_color,
            ..
        })) = result
        {
            assert_eq!(positive_color, "00FF00");
            assert_eq!(negative_color, "FF0000");
            assert_eq!(track_color, Some("CCCCCC".to_string()));
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    #[test]
    fn test_handle_color_aliases() {
        // "up" and "down" should work as aliases
        let mut params = HashMap::new();
        params.insert("up".to_string(), "00FF00".to_string());
        params.insert("down".to_string(), "FF0000".to_string());

        let result = handle(&["1,-1".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform {
            positive_color,
            negative_color,
            ..
        })) = result
        {
            assert_eq!(positive_color, "00FF00");
            assert_eq!(negative_color, "FF0000");
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    // ========================================================================
    // Center Line
    // ========================================================================

    #[rstest]
    #[case("true", true)]
    #[case("1", true)]
    #[case("false", false)]
    fn test_handle_center(#[case] value: &str, #[case] expected: bool) {
        let mut params = HashMap::new();
        params.insert("center".to_string(), value.to_string());

        let result = handle(&["1,-1".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform {
            show_center_line, ..
        })) = result
        {
            assert_eq!(show_center_line, expected);
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    #[test]
    fn test_handle_center_color() {
        let mut params = HashMap::new();
        params.insert("center".to_string(), "true".to_string());
        params.insert("center_color".to_string(), "888888".to_string());

        let result = handle(&["1,-1".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform {
            show_center_line,
            center_line_color,
            ..
        })) = result
        {
            assert!(show_center_line);
            assert_eq!(center_line_color, Some("888888".to_string()));
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    #[test]
    fn test_handle_defaults() {
        let result = handle(&["1,-1".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform {
            width,
            height,
            bar_width,
            spacing,
            show_center_line,
            track_color,
            center_line_color,
            ..
        })) = result
        {
            assert_eq!(width, 100);
            assert_eq!(height, 40);
            assert_eq!(bar_width, 3);
            assert_eq!(spacing, 1);
            assert!(!show_center_line);
            assert!(track_color.is_none());
            assert!(center_line_color.is_none());
        } else {
            panic!("Expected Waveform primitive");
        }
    }

    // ========================================================================
    // Parameter Clamping Tests
    // ========================================================================

    #[rstest]
    #[case("width", "0", 1)] // below min -> clamped to 1
    #[case("width", "3000", 2000)] // above max -> clamped to 2000
    #[case("height", "0", 1)] // below min -> clamped to 1
    #[case("height", "600", 500)] // above max -> clamped to 500
    #[case("spacing", "60", 50)] // above max -> clamped to 50
    #[case("bar_width", "0", 1)] // below min -> clamped to 1
    #[case("bar_width", "60", 50)] // above max -> clamped to 50
    fn test_handle_param_clamping(#[case] key: &str, #[case] value: &str, #[case] expected: u32) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["1,-1,2".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Waveform {
            width,
            height,
            bar_width,
            spacing,
            ..
        })) = result
        {
            match key {
                "width" => assert_eq!(width, expected),
                "height" => assert_eq!(height, expected),
                "bar_width" => assert_eq!(bar_width, expected),
                "spacing" => assert_eq!(spacing, expected),
                _ => panic!("Unknown key"),
            }
        } else {
            panic!("Expected Waveform primitive");
        }
    }
}
