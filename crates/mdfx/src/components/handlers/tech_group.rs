//! Tech group component handler
//!
//! Automatically applies corner presets to a group of badges:
//! - First badge: corners=left (rounded left, square right)
//! - Middle badges: corners=none (square corners)
//! - Last badge: corners=right (square left, rounded right)
//!
//! Supports: tech, version, and license badges.
//!
//! Style inheritance: params set on the group are inherited by child badges
//! unless the badge specifies its own value.
//!
//! This creates a seamless "pill" group when badges are placed side-by-side.

use crate::components::ComponentOutput;
use crate::error::Result;
use std::collections::HashMap;

/// Component types that can participate in badge groups
const GROUPABLE_COMPONENTS: &[&str] = &["tech", "version", "license"];

/// Find all groupable component invocations in content.
/// Returns (start, end) positions for each match.
fn find_groupable_components(content: &str) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    let mut pos = 0;

    while let Some(start_rel) = content[pos..].find("{{ui:") {
        let start = pos + start_rel;

        // Find the closing }}
        if let Some(end_rel) = content[start..].find("}}") {
            let end = start + end_rel + 2;

            // Extract component type (between "{{ui:" and next ":" or "/" or "}")
            let after_prefix = &content[start + 5..end];
            let type_end = after_prefix
                .find([':', '/', '}'])
                .unwrap_or(after_prefix.len());
            let comp_type = &after_prefix[..type_end];

            if GROUPABLE_COMPONENTS.contains(&comp_type) {
                results.push((start, end));
            }

            pos = end;
        } else {
            break;
        }
    }

    results
}

/// Handle tech-group component expansion
///
/// Transforms content containing badges to automatically apply
/// appropriate corner presets for a connected badge group.
///
/// Style inheritance: Any params on the group (bg, border, text_color, etc.)
/// are inherited by child badges unless the badge specifies its own value.
pub fn handle(params: &HashMap<String, String>, content: Option<&str>) -> Result<ComponentOutput> {
    let content = content.unwrap_or("");

    // Find all groupable component invocations
    let matches = find_groupable_components(content);
    let count = matches.len();

    if count == 0 {
        // No tech badges found, return content as-is
        return Ok(ComponentOutput::Template(content.to_string()));
    }

    // Extract gap parameter (spacing between badges in pixels, for row layout)
    let gap = params
        .get("gap")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(0);

    // Collect all params from group (except group-only params like "gap")
    let inherited: Vec<(&str, &str)> = params
        .iter()
        .filter(|(k, _)| *k != "gap")
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    let mut result = content.to_string();

    // Process badges in reverse order to preserve positions
    for (i, &(start, end)) in matches.iter().enumerate().rev() {
        let full_match = &content[start..end];

        // Determine which corner preset to apply
        let corner_preset = if count == 1 {
            // Single badge: keep all corners rounded (no modification needed)
            None
        } else if i == 0 {
            // First badge: rounded left, square right
            Some("left")
        } else if i == count - 1 {
            // Last badge: square left, rounded right
            Some("right")
        } else {
            // Middle badges: no rounded corners
            Some("none")
        };

        // Build params to inject (inherited + corners)
        let mut inject_params = String::new();

        // Add inherited params if not already specified in the badge
        for (key, value) in &inherited {
            let check_key = format!("{}=", key);
            if !full_match.contains(&check_key) {
                inject_params.push_str(&format!(":{}={}", key, value));
            }
        }

        // Add corners if needed
        if let Some(preset) = corner_preset {
            if !full_match.contains("corners=") {
                inject_params.push_str(&format!(":corners={}", preset));
            }
        }

        // Apply modifications if we have any params to inject
        if !inject_params.is_empty() {
            let modified = if let Some(inner) = full_match.strip_suffix("/}}") {
                // Self-closing tag
                format!("{}{}/}}}}", inner, inject_params)
            } else if let Some(inner) = full_match.strip_suffix("}}") {
                // Regular closing tag
                format!("{}{}}}}}", inner, inject_params)
            } else {
                full_match.to_string()
            };

            result.replace_range(start..end, &modified);
        }
    }

    // If gap is specified, wrap in row for proper alignment
    if gap > 0 {
        // Add spacing between badges using row component
        result = format!("{{{{ui:row}}}}{}{}", result, "{{/ui}}");
    }

    Ok(ComponentOutput::Template(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Corner Preset Assignment (Parameterized)
    // ========================================================================

    #[rstest]
    // Single badge - no corner modification
    #[case("{{ui:tech:rust/}}", &[], "{{ui:tech:rust/}}")]
    // Two badges - left and right corners
    #[case("{{ui:tech:rust/}}{{ui:tech:typescript/}}", &["corners=left", "corners=right"], "")]
    // Three badges - left, none, right
    #[case("{{ui:tech:rust/}}{{ui:tech:typescript/}}{{ui:tech:docker/}}", &["corners=left", "corners=none", "corners=right"], "")]
    fn test_corner_assignments(
        #[case] content: &str,
        #[case] expected_corners: &[&str],
        #[case] exact_match: &str,
    ) {
        let params = HashMap::new();
        let result = handle(&params, Some(content)).unwrap();

        let ComponentOutput::Template(template) = result else {
            unreachable!("Expected Template output");
        };
        if !exact_match.is_empty() {
            assert_eq!(template, exact_match);
        } else {
            for corner in expected_corners {
                assert!(
                    template.contains(corner),
                    "Expected {} in {}",
                    corner,
                    template
                );
            }
        }
    }

    #[test]
    fn test_existing_corners_not_overwritten() {
        let params = HashMap::new();
        let content = "{{ui:tech:rust:corners=all/}}{{ui:tech:typescript/}}";

        let result = handle(&params, Some(content)).unwrap();

        let ComponentOutput::Template(template) = result else {
            unreachable!("Expected Template output");
        };
        // First badge already has corners, should not be modified
        assert!(template.contains("corners=all"));
        assert!(template.contains("corners=right"));
    }

    // ========================================================================
    // Style Inheritance (Parameterized)
    // ========================================================================

    #[rstest]
    // Basic inheritance - bg and border passed to both badges
    #[case(
        &[("bg", "1a1a2e"), ("border", "00ff00")],
        "{{ui:tech:rust/}}{{ui:tech:go/}}",
        &["bg=1a1a2e", "border=00ff00"],
        2, 2  // bg count, border count
    )]
    // Override - second badge has its own bg, shouldn't inherit
    #[case(
        &[("bg", "1a1a2e")],
        "{{ui:tech:rust/}}{{ui:tech:go:bg=custom/}}",
        &["bg=1a1a2e", "bg=custom"],
        1, 0  // bg=1a1a2e appears once, no border
    )]
    // Single badge inherits styles but no corners
    #[case(
        &[("border", "ff0000")],
        "{{ui:tech:rust/}}",
        &["border=ff0000"],
        0, 1  // no bg, border appears once
    )]
    fn test_style_inheritance(
        #[case] params_input: &[(&str, &str)],
        #[case] content: &str,
        #[case] expected_contains: &[&str],
        #[case] bg_count: usize,
        #[case] border_count: usize,
    ) {
        let mut params = HashMap::new();
        for (k, v) in params_input {
            params.insert(k.to_string(), v.to_string());
        }

        let result = handle(&params, Some(content)).unwrap();

        let ComponentOutput::Template(template) = result else {
            unreachable!("Expected Template output");
        };
        for expected in expected_contains {
            assert!(
                template.contains(expected),
                "Expected {} in {}",
                expected,
                template
            );
        }
        if bg_count > 0 {
            assert_eq!(template.matches("bg=1a1a2e").count(), bg_count);
        }
        if border_count > 0 {
            let border_key = if params_input.iter().any(|(k, _)| *k == "border") {
                params_input.iter().find(|(k, _)| *k == "border").unwrap().1
            } else {
                "00ff00"
            };
            assert_eq!(
                template.matches(&format!("border={}", border_key)).count(),
                border_count
            );
        }
    }

    // ========================================================================
    // Mixed Badge Types (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(
        "{{ui:version:1.0.0/}}{{ui:tech:rust/}}{{ui:license:MIT/}}",
        &["version:1.0.0:corners=left", "tech:rust:corners=none", "license:MIT:corners=right"]
    )]
    #[case(
        "{{ui:tech:docker/}}{{ui:license:Apache/}}",
        &["corners=left", "corners=right"]
    )]
    fn test_mixed_badge_types(#[case] content: &str, #[case] expected: &[&str]) {
        let params = HashMap::new();
        let result = handle(&params, Some(content)).unwrap();

        let ComponentOutput::Template(template) = result else {
            unreachable!("Expected Template output");
        };
        for exp in expected {
            assert!(template.contains(exp), "Expected {} in {}", exp, template);
        }
    }

    #[test]
    fn test_mixed_with_style_inheritance() {
        let mut params = HashMap::new();
        params.insert("bg".to_string(), "1a1a2e".to_string());
        let content = "{{ui:version:2.0.0/}}{{ui:tech:docker/}}";

        let result = handle(&params, Some(content)).unwrap();

        let ComponentOutput::Template(template) = result else {
            unreachable!("Expected Template output");
        };
        // Both should inherit bg
        assert_eq!(template.matches("bg=1a1a2e").count(), 2);
        // Corner assignments
        assert!(template.contains("corners=left"));
        assert!(template.contains("corners=right"));
    }

    #[test]
    fn test_ignores_non_groupable_components() {
        let params = HashMap::new();
        let content = "{{ui:swatch:ff0000/}}{{ui:tech:rust/}}";

        let result = handle(&params, Some(content)).unwrap();

        let ComponentOutput::Template(template) = result else {
            unreachable!("Expected Template output");
        };
        // Only tech badge should be found (single badge = no corners)
        assert!(!template.contains("corners="));
        // Swatch should be unchanged
        assert!(template.contains("{{ui:swatch:ff0000/}}"));
    }
}
