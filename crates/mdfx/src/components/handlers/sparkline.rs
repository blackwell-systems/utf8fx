//! Sparkline chart component handler

use super::{
    get_string, parse_bool, parse_param_clamped, resolve_color_opt, resolve_color_with_default,
};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle sparkline component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "sparkline component requires comma-separated values".to_string(),
        ));
    }

    // Parse values from first arg (comma-separated)
    let values: Vec<f32> = args[0]
        .split(',')
        .filter_map(|s| s.trim().parse::<f32>().ok())
        .collect();

    if values.is_empty() {
        return Err(Error::ParseError(
            "sparkline requires at least one numeric value".to_string(),
        ));
    }

    // Width: 1-2000px, Height: 1-500px, Stroke: 1-20px, Dot radius: 1-20px
    let width: u32 = parse_param_clamped(params, "width", 100, 1, 2000);
    let height: u32 = parse_param_clamped(params, "height", 20, 1, 500);
    let stroke_width: u32 = parse_param_clamped(params, "stroke_width", 2, 1, 20);
    let dot_radius: u32 = parse_param_clamped(params, "dot_radius", 2, 1, 20);

    let chart_type = get_string(params, "type", "line");
    let fill_color = resolve_color_with_default(params, "fill", "pink", &resolve_color);
    let stroke_color = resolve_color_opt(params, "stroke", &resolve_color);
    let track_color = resolve_color_opt(params, "track", &resolve_color);

    let show_dots = parse_bool(params, "dots", false);

    Ok(ComponentOutput::Primitive(Primitive::Sparkline {
        values,
        width,
        height,
        chart_type,
        fill_color,
        stroke_color,
        stroke_width,
        track_color,
        show_dots,
        dot_radius,
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
    #[case("10, 20, 30", vec![10.0, 20.0, 30.0])] // with spaces
    #[case("1.5,2.5,3.5", vec![1.5, 2.5, 3.5])] // floats
    #[case("-1,0,1", vec![-1.0, 0.0, 1.0])] // negative values
    #[case("100", vec![100.0])] // single value
    fn test_handle_values(#[case] input: &str, #[case] expected: Vec<f32>) {
        let result = handle(&[input.to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline { values, .. })) = result {
            assert_eq!(values.len(), expected.len());
            for (v, e) in values.iter().zip(expected.iter()) {
                assert!((v - e).abs() < 0.001);
            }
        } else {
            panic!("Expected Sparkline primitive");
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

    #[test]
    fn test_handle_mixed_values_skips_invalid() {
        // Should parse valid numbers and skip invalid ones
        let result = handle(&["1,abc,3".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline { values, .. })) = result {
            assert_eq!(values, vec![1.0, 3.0]);
        } else {
            panic!("Expected Sparkline primitive");
        }
    }

    // ========================================================================
    // Size Parameters (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("width", "200", 200, 20, 2, 2)]
    #[case("height", "40", 100, 40, 2, 2)]
    #[case("stroke_width", "4", 100, 20, 4, 2)]
    #[case("dot_radius", "5", 100, 20, 2, 5)]
    fn test_handle_size_params(
        #[case] key: &str,
        #[case] value: &str,
        #[case] expected_width: u32,
        #[case] expected_height: u32,
        #[case] expected_stroke_width: u32,
        #[case] expected_dot_radius: u32,
    ) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["1,2,3".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline {
            width,
            height,
            stroke_width,
            dot_radius,
            ..
        })) = result
        {
            assert_eq!(width, expected_width);
            assert_eq!(height, expected_height);
            assert_eq!(stroke_width, expected_stroke_width);
            assert_eq!(dot_radius, expected_dot_radius);
        } else {
            panic!("Expected Sparkline primitive");
        }
    }

    // ========================================================================
    // Chart Type (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(None, "line")] // default
    #[case(Some("line"), "line")]
    #[case(Some("bar"), "bar")]
    #[case(Some("area"), "area")]
    fn test_handle_chart_type(#[case] input: Option<&str>, #[case] expected: &str) {
        let mut params = HashMap::new();
        if let Some(t) = input {
            params.insert("type".to_string(), t.to_string());
        }

        let result = handle(&["1,2,3".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline { chart_type, .. })) = result {
            assert_eq!(chart_type, expected);
        } else {
            panic!("Expected Sparkline primitive");
        }
    }

    // ========================================================================
    // Color Parameters
    // ========================================================================

    #[test]
    fn test_handle_colors() {
        let mut params = HashMap::new();
        params.insert("fill".to_string(), "FF0000".to_string());
        params.insert("stroke".to_string(), "0000FF".to_string());
        params.insert("track".to_string(), "CCCCCC".to_string());

        let result = handle(&["1,2,3".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline {
            fill_color,
            stroke_color,
            track_color,
            ..
        })) = result
        {
            assert_eq!(fill_color, "FF0000");
            assert_eq!(stroke_color, Some("0000FF".to_string()));
            assert_eq!(track_color, Some("CCCCCC".to_string()));
        } else {
            panic!("Expected Sparkline primitive");
        }
    }

    // ========================================================================
    // Dots Parameter
    // ========================================================================

    #[rstest]
    #[case("true", true)]
    #[case("1", true)]
    #[case("false", false)]
    fn test_handle_dots(#[case] value: &str, #[case] expected: bool) {
        let mut params = HashMap::new();
        params.insert("dots".to_string(), value.to_string());

        let result = handle(&["1,2,3".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline { show_dots, .. })) = result {
            assert_eq!(show_dots, expected);
        } else {
            panic!("Expected Sparkline primitive");
        }
    }

    #[test]
    fn test_handle_defaults() {
        let result = handle(&["1,2,3".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline {
            width,
            height,
            chart_type,
            stroke_width,
            show_dots,
            dot_radius,
            stroke_color,
            track_color,
            ..
        })) = result
        {
            assert_eq!(width, 100);
            assert_eq!(height, 20);
            assert_eq!(chart_type, "line");
            assert_eq!(stroke_width, 2);
            assert!(!show_dots);
            assert_eq!(dot_radius, 2);
            assert!(stroke_color.is_none());
            assert!(track_color.is_none());
        } else {
            panic!("Expected Sparkline primitive");
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
    #[case("stroke_width", "0", 1)] // below min -> clamped to 1
    #[case("stroke_width", "30", 20)] // above max -> clamped to 20
    #[case("dot_radius", "0", 1)] // below min -> clamped to 1
    #[case("dot_radius", "30", 20)] // above max -> clamped to 20
    fn test_handle_param_clamping(#[case] key: &str, #[case] value: &str, #[case] expected: u32) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["1,2,3".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Sparkline {
            width,
            height,
            stroke_width,
            dot_radius,
            ..
        })) = result
        {
            match key {
                "width" => assert_eq!(width, expected),
                "height" => assert_eq!(height, expected),
                "stroke_width" => assert_eq!(stroke_width, expected),
                "dot_radius" => assert_eq!(dot_radius, expected),
                _ => panic!("Unknown key"),
            }
        } else {
            panic!("Expected Sparkline primitive");
        }
    }
}
