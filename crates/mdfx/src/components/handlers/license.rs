//! License badge component handler
//!
//! Renders license badges with category-aware coloring.
//! Supports common SPDX license identifiers.

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// License categories for coloring
#[derive(Debug, Clone, Copy, PartialEq)]
enum LicenseCategory {
    Permissive,   // Green - MIT, Apache, BSD, ISC
    WeakCopyleft, // Blue - LGPL, MPL, EPL
    Copyleft,     // Yellow - GPL, AGPL
    Proprietary,  // Gray - closed source
    PublicDomain, // Cyan - CC0, Unlicense
    Unknown,      // Slate - unrecognized
}

impl LicenseCategory {
    /// Get the background color for this category
    fn bg_color(&self) -> &'static str {
        match self {
            LicenseCategory::Permissive => "22C55E",   // success green
            LicenseCategory::WeakCopyleft => "3B82F6", // info blue
            LicenseCategory::Copyleft => "EAB308",     // warning yellow
            LicenseCategory::Proprietary => "6B7280",  // gray
            LicenseCategory::PublicDomain => "06B6D4", // cyan
            LicenseCategory::Unknown => "475569",      // slate
        }
    }

    /// Get the text color for this category (for contrast)
    fn text_color(&self) -> &'static str {
        match self {
            LicenseCategory::Permissive => "FFFFFF",
            LicenseCategory::WeakCopyleft => "FFFFFF",
            LicenseCategory::Copyleft => "000000",
            LicenseCategory::Proprietary => "FFFFFF",
            LicenseCategory::PublicDomain => "000000",
            LicenseCategory::Unknown => "FFFFFF",
        }
    }
}

/// Categorize a license by its SPDX identifier or common name
fn categorize_license(license: &str) -> LicenseCategory {
    let upper = license.to_uppercase();
    let normalized = upper.replace('-', "").replace(' ', "");

    // Permissive licenses
    if matches!(
        normalized.as_str(),
        "MIT" | "APACHE" | "APACHE2" | "APACHE20" | "APACHE2.0"
            | "BSD" | "BSD2" | "BSD2CLAUSE" | "BSD3" | "BSD3CLAUSE"
            | "ISC" | "ZLIB" | "WTFPL" | "BOOST" | "BSL" | "BSL1" | "BSL10" | "BSL1.0"
    ) {
        return LicenseCategory::Permissive;
    }

    // Weak copyleft licenses
    if normalized.starts_with("LGPL") || normalized.starts_with("MPL") || normalized.starts_with("EPL") || normalized.starts_with("CDDL") {
        return LicenseCategory::WeakCopyleft;
    }

    // Strong copyleft licenses
    if normalized.starts_with("GPL") || normalized.starts_with("AGPL") || normalized.starts_with("SSPL") {
        return LicenseCategory::Copyleft;
    }

    // Public domain
    if matches!(
        normalized.as_str(),
        "CC0" | "CC01" | "CC010" | "CC0UNIVERSAL" | "UNLICENSE" | "PUBLICDOMAIN" | "PD"
    ) {
        return LicenseCategory::PublicDomain;
    }

    // Proprietary/closed source
    if matches!(
        normalized.as_str(),
        "PROPRIETARY" | "COMMERCIAL" | "CLOSED" | "ALLRIGHTSRESERVED" | "ARR"
    ) {
        return LicenseCategory::Proprietary;
    }

    LicenseCategory::Unknown
}

/// Format license name for display
fn format_license_name(license: &str) -> String {
    // Common SPDX mappings to prettier display names
    let upper = license.to_uppercase();

    match upper.as_str() {
        "MIT" => "MIT".to_string(),
        "APACHE-2.0" | "APACHE2.0" | "APACHE2" | "APACHE-2" => "Apache 2.0".to_string(),
        "GPL-3.0" | "GPL3.0" | "GPL3" | "GPL-3" => "GPL 3.0".to_string(),
        "GPL-2.0" | "GPL2.0" | "GPL2" | "GPL-2" => "GPL 2.0".to_string(),
        "LGPL-3.0" | "LGPL3.0" | "LGPL3" | "LGPL-3" => "LGPL 3.0".to_string(),
        "LGPL-2.1" | "LGPL2.1" | "LGPL21" | "LGPL-21" => "LGPL 2.1".to_string(),
        "AGPL-3.0" | "AGPL3.0" | "AGPL3" | "AGPL-3" => "AGPL 3.0".to_string(),
        "BSD-3-CLAUSE" | "BSD3CLAUSE" | "BSD3" | "BSD-3" => "BSD 3-Clause".to_string(),
        "BSD-2-CLAUSE" | "BSD2CLAUSE" | "BSD2" | "BSD-2" => "BSD 2-Clause".to_string(),
        "MPL-2.0" | "MPL2.0" | "MPL2" | "MPL-2" => "MPL 2.0".to_string(),
        "CC0-1.0" | "CC010" | "CC0" => "CC0".to_string(),
        "UNLICENSE" => "Unlicense".to_string(),
        "ISC" => "ISC".to_string(),
        "WTFPL" => "WTFPL".to_string(),
        _ => license.to_string(), // Keep original casing
    }
}

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
    let category = categorize_license(license);

    // Allow color overrides
    let bg_color = params
        .get("bg")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| category.bg_color().to_string());

    let text_color = params
        .get("text")
        .or_else(|| params.get("text_color"))
        .or_else(|| params.get("color"))
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| category.text_color().to_string());

    // Label: use formatted name or custom label
    let label = params
        .get("label")
        .cloned()
        .unwrap_or_else(|| format_license_name(license));

    // Optional icon (e.g., "scale" for legal, "file" for document)
    let icon = params.get("icon").cloned();

    // Border parameters
    let border_color = params.get("border").map(|c| resolve_color(c));
    let border_width = params.get("border_width").and_then(|v| v.parse().ok());

    // Corner radius
    let rx = params.get("rx").and_then(|v| v.parse().ok());

    // URL for linking to license text
    let _url = params.get("url").cloned();

    Ok(ComponentOutput::Primitive(Primitive::Swatch {
        color: bg_color,
        style: style.to_string(),
        opacity: None,
        width: None,
        height: None,
        border_color,
        border_width,
        label: Some(label),
        label_color: Some(text_color),
        icon,
        icon_color: None,
        rx,
        ry: None,
        shadow: None,
        gradient: None,
        stroke_dash: None,
        logo_size: None,
        border_top: None,
        border_right: None,
        border_bottom: None,
        border_left: None,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_permissive() {
        assert_eq!(categorize_license("MIT"), LicenseCategory::Permissive);
        assert_eq!(categorize_license("Apache-2.0"), LicenseCategory::Permissive);
        assert_eq!(categorize_license("BSD-3-Clause"), LicenseCategory::Permissive);
        assert_eq!(categorize_license("ISC"), LicenseCategory::Permissive);
    }

    #[test]
    fn test_categorize_weak_copyleft() {
        assert_eq!(categorize_license("LGPL-3.0"), LicenseCategory::WeakCopyleft);
        assert_eq!(categorize_license("MPL-2.0"), LicenseCategory::WeakCopyleft);
        assert_eq!(categorize_license("EPL-2.0"), LicenseCategory::WeakCopyleft);
    }

    #[test]
    fn test_categorize_copyleft() {
        assert_eq!(categorize_license("GPL-3.0"), LicenseCategory::Copyleft);
        assert_eq!(categorize_license("AGPL-3.0"), LicenseCategory::Copyleft);
        assert_eq!(categorize_license("GPL-2.0"), LicenseCategory::Copyleft);
    }

    #[test]
    fn test_categorize_public_domain() {
        assert_eq!(categorize_license("CC0"), LicenseCategory::PublicDomain);
        assert_eq!(categorize_license("Unlicense"), LicenseCategory::PublicDomain);
    }

    #[test]
    fn test_categorize_proprietary() {
        assert_eq!(categorize_license("Proprietary"), LicenseCategory::Proprietary);
        assert_eq!(categorize_license("Commercial"), LicenseCategory::Proprietary);
    }

    #[test]
    fn test_format_license_name() {
        assert_eq!(format_license_name("MIT"), "MIT");
        assert_eq!(format_license_name("apache-2.0"), "Apache 2.0");
        assert_eq!(format_license_name("GPL-3.0"), "GPL 3.0");
        assert_eq!(format_license_name("BSD-3-Clause"), "BSD 3-Clause");
    }
}
