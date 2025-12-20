//! Version badge component handler
//!
//! Renders semantic version badges with status-aware coloring.
//! Status detection and rendering is delegated to badgefx.

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::{Primitive, VersionConfig};
use std::collections::HashMap;

/// Handle version component expansion
///
/// Syntax: {{ui:version:VERSION/}} or {{ui:version:VERSION:status=STATUS/}}
///
/// Examples:
/// - {{ui:version:1.0.0/}} -> green (stable)
/// - {{ui:version:2.0.0-beta.1/}} -> yellow (auto-detected beta)
/// - {{ui:version:0.1.0/}} -> yellow (0.x is beta)
/// - {{ui:version:1.5.0:status=deprecated/}} -> red (override)
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "version component requires a version string argument".to_string(),
        ));
    }

    let version = &args[0];

    // Build VersionConfig - badgefx handles status detection
    let config = VersionConfig {
        version: version.clone(),
        style: style.to_string(),
        status: params.get("status").cloned(),
        bg_color: params.get("bg").map(|c| resolve_color(c)),
        text_color: params
            .get("text")
            .or_else(|| params.get("text_color"))
            .or_else(|| params.get("color"))
            .map(|c| resolve_color(c)),
        prefix: params.get("prefix").cloned(),
        border_color: params.get("border").map(|c| resolve_color(c)),
        border_width: params.get("border_width").and_then(|v| v.parse().ok()),
        rx: params.get("rx").and_then(|v| v.parse().ok()),
    };

    Ok(ComponentOutput::Primitive(Primitive::Version(config)))
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
            &["1.0.0".to_string()],
            &HashMap::new(),
            "flat",
            identity_color,
        );
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Version(config))) = result {
            assert_eq!(config.version, "1.0.0");
            assert_eq!(config.style, "flat");
        } else {
            panic!("Expected Version primitive");
        }
    }

    #[test]
    fn test_handle_with_status_override() {
        let mut params = HashMap::new();
        params.insert("status".to_string(), "deprecated".to_string());

        let result = handle(&["1.0.0".to_string()], &params, "flat", identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Version(config))) = result {
            assert_eq!(config.status, Some("deprecated".to_string()));
        } else {
            panic!("Expected Version primitive");
        }
    }

    #[test]
    fn test_handle_with_colors() {
        let mut params = HashMap::new();
        params.insert("bg".to_string(), "FF0000".to_string());
        params.insert("text".to_string(), "FFFFFF".to_string());

        let result = handle(&["1.0.0".to_string()], &params, "flat", identity_color);
        assert!(result.is_ok());
        if let Ok(ComponentOutput::Primitive(Primitive::Version(config))) = result {
            assert_eq!(config.bg_color, Some("FF0000".to_string()));
            assert_eq!(config.text_color, Some("FFFFFF".to_string()));
        } else {
            panic!("Expected Version primitive");
        }
    }

    #[test]
    fn test_handle_missing_version() {
        let result = handle(&[], &HashMap::new(), "flat", identity_color);
        assert!(result.is_err());
    }
}
