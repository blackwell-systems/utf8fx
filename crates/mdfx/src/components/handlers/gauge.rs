//! Gauge (semi-circular meter) component handler

use super::{
    parse_bool, parse_param_clamped, parse_thumb_config, resolve_color_opt,
    resolve_color_with_default,
};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle gauge component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "gauge component requires a percentage argument".to_string(),
        ));
    }

    // Parse percentage (first arg)
    let percent: u8 = args[0].parse().map_err(|_| {
        Error::ParseError(format!(
            "Invalid percentage '{}' - must be a number 0-100",
            args[0]
        ))
    })?;
    let percent = percent.min(100);

    // Size: 10-500px, Thickness: 1-50px
    let size: u32 = parse_param_clamped(params, "size", 80, 10, 500);
    let thickness: u32 = parse_param_clamped(params, "thickness", 8, 1, 50);

    let track_color = resolve_color_with_default(params, "track", "gray", &resolve_color);
    let fill_color = resolve_color_with_default(params, "fill", "pink", &resolve_color);

    let show_label = parse_bool(params, "label", false);
    let label_color = resolve_color_opt(params, "label_color", &resolve_color);

    // Parse thumb configuration (enables slider mode)
    let thumb = parse_thumb_config(params, &resolve_color);

    Ok(ComponentOutput::Primitive(Primitive::Gauge {
        percent,
        size,
        thickness,
        track_color,
        fill_color,
        show_label,
        label_color,
        thumb,
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
    // Basic Gauge Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("0", 0)]
    #[case("50", 50)]
    #[case("75", 75)]
    #[case("100", 100)]
    #[case("150", 100)] // clamped to 100
    fn test_handle_percent(#[case] input: &str, #[case] expected: u8) {
        let result = handle(&[input.to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Gauge { percent, .. })) = result {
            assert_eq!(percent, expected);
        } else {
            panic!("Expected Gauge primitive");
        }
    }

    #[test]
    fn test_handle_missing_args() {
        let result = handle(&[], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_invalid_percent() {
        let result = handle(&["abc".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_err());
    }

    // ========================================================================
    // Size and Thickness Parameters
    // ========================================================================

    #[rstest]
    #[case("size", "100", 100, 8)] // custom size, default thickness
    #[case("thickness", "12", 80, 12)] // default size, custom thickness
    fn test_handle_size_params(
        #[case] key: &str,
        #[case] value: &str,
        #[case] expected_size: u32,
        #[case] expected_thickness: u32,
    ) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Gauge {
            size, thickness, ..
        })) = result
        {
            assert_eq!(size, expected_size);
            assert_eq!(thickness, expected_thickness);
        } else {
            panic!("Expected Gauge primitive");
        }
    }

    // ========================================================================
    // Color Parameters
    // ========================================================================

    #[test]
    fn test_handle_colors() {
        let mut params = HashMap::new();
        params.insert("track".to_string(), "AABBCC".to_string());
        params.insert("fill".to_string(), "FF0000".to_string());

        let result = handle(&["75".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Gauge {
            track_color,
            fill_color,
            ..
        })) = result
        {
            assert_eq!(track_color, "AABBCC");
            assert_eq!(fill_color, "FF0000");
        } else {
            panic!("Expected Gauge primitive");
        }
    }

    // ========================================================================
    // Label Configuration
    // ========================================================================

    #[rstest]
    #[case("true", true)]
    #[case("1", true)]
    #[case("false", false)]
    fn test_handle_label(#[case] value: &str, #[case] expected: bool) {
        let mut params = HashMap::new();
        params.insert("label".to_string(), value.to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Gauge { show_label, .. })) = result {
            assert_eq!(show_label, expected);
        } else {
            panic!("Expected Gauge primitive");
        }
    }

    // ========================================================================
    // Thumb Configuration
    // ========================================================================

    #[test]
    fn test_handle_thumb() {
        let mut params = HashMap::new();
        params.insert("thumb".to_string(), "12".to_string());
        params.insert("thumb_color".to_string(), "00FF00".to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Gauge { thumb, .. })) = result {
            assert!(thumb.is_some());
            let thumb = thumb.unwrap();
            assert_eq!(thumb.size, 12);
            assert_eq!(thumb.color, Some("00FF00".to_string()));
        } else {
            panic!("Expected Gauge primitive");
        }
    }

    #[test]
    fn test_handle_defaults() {
        let result = handle(&["50".to_string()], &HashMap::new(), identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Gauge {
            size,
            thickness,
            show_label,
            thumb,
            ..
        })) = result
        {
            assert_eq!(size, 80); // default
            assert_eq!(thickness, 8); // default
            assert!(!show_label); // default false
            assert!(thumb.is_none());
        } else {
            panic!("Expected Gauge primitive");
        }
    }

    // ========================================================================
    // Parameter Clamping Tests
    // ========================================================================

    #[rstest]
    #[case("size", "5", 10)] // below min -> clamped to 10
    #[case("size", "600", 500)] // above max -> clamped to 500
    #[case("thickness", "0", 1)] // below min -> clamped to 1
    #[case("thickness", "60", 50)] // above max -> clamped to 50
    fn test_handle_param_clamping(#[case] key: &str, #[case] value: &str, #[case] expected: u32) {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());

        let result = handle(&["50".to_string()], &params, identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Gauge {
            size, thickness, ..
        })) = result
        {
            match key {
                "size" => assert_eq!(size, expected),
                "thickness" => assert_eq!(thickness, expected),
                _ => panic!("Unknown key"),
            }
        } else {
            panic!("Expected Gauge primitive");
        }
    }
}
