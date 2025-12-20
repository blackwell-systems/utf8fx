//! License badge component handler
//!
//! Renders license badges with category-aware coloring.
//! Category detection and rendering is delegated to badgefx.

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::{LicenseConfig, Primitive};
use std::collections::HashMap;

/// Handle license component expansion
///
/// Syntax: {{ui:license:LICENSE/}}
///
/// Examples:
/// - {{ui:license:MIT/}} -> green (permissive)
/// - {{ui:license:GPL-3.0/}} -> yellow (copyleft)
/// - {{ui:license:LGPL-3.0/}} -> blue (weak copyleft)
/// - {{ui:license:CC0/}} -> cyan (public domain)
/// - {{ui:license:Proprietary/}} -> gray
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "license component requires a license name argument".to_string(),
        ));
    }

    let license = &args[0];

    // Build LicenseConfig - badgefx handles category detection
    let config = LicenseConfig {
        license: license.clone(),
        style: style.to_string(),
        label: params.get("label").cloned(),
        bg_color: params.get("bg").map(|c| resolve_color(c)),
        text_color: params
            .get("text")
            .or_else(|| params.get("text_color"))
            .or_else(|| params.get("color"))
            .map(|c| resolve_color(c)),
        border_color: params.get("border").map(|c| resolve_color(c)),
        border_width: params.get("border_width").and_then(|v| v.parse().ok()),
        rx: params.get("rx").and_then(|v| v.parse().ok()),
    };

    Ok(ComponentOutput::Primitive(Primitive::License(config)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_color(c: &str) -> String {
        c.to_string()
    }

    #[test]
    fn test_handle_basic() {
        let result = handle(
            &["MIT".to_string()],
            &HashMap::new(),
            "flat",
            identity_color,
        );
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::License(config))) = result {
            assert_eq!(config.license, "MIT");
            assert_eq!(config.style, "flat");
        } else {
            panic!("Expected License primitive");
        }
    }

    #[test]
    fn test_handle_with_custom_label() {
        let mut params = HashMap::new();
        params.insert("label".to_string(), "Open Source".to_string());

        let result = handle(&["MIT".to_string()], &params, "flat", identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::License(config))) = result {
            assert_eq!(config.label, Some("Open Source".to_string()));
        } else {
            panic!("Expected License primitive");
        }
    }

    #[test]
    fn test_handle_with_colors() {
        let mut params = HashMap::new();
        params.insert("bg".to_string(), "FF0000".to_string());
        params.insert("text".to_string(), "FFFFFF".to_string());

        let result = handle(&["MIT".to_string()], &params, "flat", identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::License(config))) = result {
            assert_eq!(config.bg_color, Some("FF0000".to_string()));
            assert_eq!(config.text_color, Some("FFFFFF".to_string()));
        } else {
            panic!("Expected License primitive");
        }
    }

    #[test]
    fn test_handle_missing_license() {
        let result = handle(&[], &HashMap::new(), "flat", identity_color);
        assert!(result.is_err());
    }
}
