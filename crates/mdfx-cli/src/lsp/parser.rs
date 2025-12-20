//! Template parsing utilities for the mdfx LSP
//!
//! Provides functions for finding and parsing mdfx templates in document text.

/// Find all mdfx templates in a line without regex
/// Returns: Vec<(start, is_closing_tag, is_self_closing, is_malformed, content, end)>
/// - is_closing_tag: starts with {{/ (e.g., {{/bold}})
/// - is_self_closing: ends with /}} (e.g., {{glyph:star/}})
/// - is_malformed: missing closing `}}` (e.g., {{name} instead of {{name}})
pub fn find_templates(line: &str) -> Vec<(usize, bool, bool, bool, &str, usize)> {
    let mut results = Vec::new();
    let mut pos = 0;
    let bytes = line.as_bytes();
    let len = bytes.len();

    while pos + 3 < len {
        // Look for {{
        if bytes[pos] == b'{' && bytes[pos + 1] == b'{' {
            let start = pos;
            pos += 2;

            // Check for closing tag marker /
            let is_closing_tag = pos < len && bytes[pos] == b'/';
            if is_closing_tag {
                pos += 1;
            }

            let content_start = pos;
            let mut found_end = false;
            let mut is_malformed = false;

            // Find the end: either /}} or }}
            while pos < len {
                // Check for malformed: single } followed by non-}
                if bytes[pos] == b'}'
                    && pos + 1 < len
                    && bytes[pos + 1] != b'}'
                    && bytes[pos + 1] != b'/'
                {
                    // Found single } - this is malformed {{name} syntax
                    let content = &line[content_start..pos];
                    results.push((start, is_closing_tag, false, true, content, pos + 1));
                    pos += 1;
                    found_end = true;
                    is_malformed = true;
                    break;
                }

                // Check for another {{ before finding }} - indicates malformed
                if bytes[pos] == b'{' && pos + 1 < len && bytes[pos + 1] == b'{' {
                    // Found {{ inside template - the current template is unclosed
                    // Don't consume this {{, let the next iteration handle it
                    let content = &line[content_start..pos];
                    results.push((start, is_closing_tag, false, true, content, pos));
                    found_end = true;
                    is_malformed = true;
                    break;
                }

                if bytes[pos] == b'}' && pos + 1 < len && bytes[pos + 1] == b'}' {
                    // Found }} - not self-closing
                    let content = &line[content_start..pos];
                    results.push((start, is_closing_tag, false, false, content, pos + 2));
                    pos += 2;
                    found_end = true;
                    break;
                } else if bytes[pos] == b'/'
                    && pos + 2 < len
                    && bytes[pos + 1] == b'}'
                    && bytes[pos + 2] == b'}'
                {
                    // Found /}} - self-closing
                    let content = &line[content_start..pos];
                    results.push((start, is_closing_tag, true, false, content, pos + 3));
                    pos += 3;
                    found_end = true;
                    break;
                }
                pos += 1;
            }

            // If we didn't find any end marker, mark as malformed
            if !found_end {
                let content = &line[content_start..len];
                results.push((start, is_closing_tag, false, true, content, len));
                break;
            }

            let _ = is_malformed; // suppress warning
        } else {
            pos += 1;
        }
    }

    results
}

/// Extract tag name from template content for matching
/// e.g., "bold" from "bold", "frame:gradient" from "frame:gradient"
pub fn extract_tag_name(content: &str) -> String {
    // For frame:name, keep the full "frame:name"
    if content.starts_with("frame:") {
        let name = content.strip_prefix("frame:").unwrap_or("");
        let name = name.split(':').next().unwrap_or(name);
        return format!("frame:{}", name);
    }
    // For styles/components, just the name before any args
    content.split(':').next().unwrap_or(content).to_string()
}

/// Check if a template is inherently self-closing (never needs a closing tag)
/// These templates render inline content and don't wrap text
pub fn is_inherently_self_closing(content: &str) -> bool {
    // Block ui: components that wrap content (NOT self-closing)
    if content.starts_with("ui:row") || content.starts_with("ui:tech-group") {
        return false;
    }
    // Self-closing ui: components (tech, version, license, progress, donut, gauge, live, swatch)
    content.starts_with("ui:")
        // Glyphs
        || content.starts_with("glyph:")
        // Standalone swatch (shorthand)
        || content.starts_with("swatch:")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // (start, is_closing, is_self_closing, is_malformed, content, end)
    #[case("{{glyph:star/}}", vec![(0, false, true, false, "glyph:star", 15)])]
    #[case("{{bold}}text{{/bold}}", vec![(0, false, false, false, "bold", 8), (12, true, false, false, "bold", 21)])]
    #[case("{{//}}", vec![(0, true, true, false, "", 6)])] // Universal closer ends with /}}
    #[case("text {{ui:tech:rust/}} more", vec![(5, false, true, false, "ui:tech:rust", 22)])]
    #[case("no templates here", vec![])]
    #[case("{{a}}{{b/}}{{/c}}", vec![(0, false, false, false, "a", 5), (5, false, true, false, "b", 11), (11, true, false, false, "c", 17)])]
    fn test_find_templates(
        #[case] input: &str,
        #[case] expected: Vec<(usize, bool, bool, bool, &str, usize)>,
    ) {
        let result = find_templates(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("bold", "bold")]
    #[case("italic:arg", "italic")]
    #[case("frame:gradient", "frame:gradient")]
    #[case("frame:gradient:args", "frame:gradient")]
    #[case("ui:tech:rust", "ui")]
    fn test_extract_tag_name(#[case] content: &str, #[case] expected: &str) {
        let result = extract_tag_name(content);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_templates_edge_cases() {
        // Incomplete template (no closing) - now marked as malformed
        let result = find_templates("{{incomplete");
        assert_eq!(result, vec![(0, false, false, true, "incomplete", 12)]);

        // Empty line
        assert_eq!(find_templates(""), vec![]);

        // Single brace (not a template)
        assert_eq!(find_templates("{not}a{template}"), vec![]);

        // Adjacent templates
        let result = find_templates("{{a}}{{b}}");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_find_templates_self_closing_variants() {
        // Self-closing with content
        let result = find_templates("{{swatch:red/}}");
        assert_eq!(result, vec![(0, false, true, false, "swatch:red", 15)]);

        // Regular closing (not self-closing)
        let result = find_templates("{{/bold}}");
        assert_eq!(result, vec![(0, true, false, false, "bold", 9)]);
    }

    #[test]
    fn test_find_templates_malformed() {
        // Malformed: {{name} missing second }
        let result = find_templates("{{blackboard}text{{/blackboard}}");
        // First template is malformed (missing `}`), second is valid closing
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (0, false, false, true, "blackboard", 13));
        assert_eq!(result[1], (17, true, false, false, "blackboard", 32));

        // Malformed with nested {{ before closing
        let result = find_templates("{{open{{another}}");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (0, false, false, true, "open", 6)); // malformed
        assert_eq!(result[1], (6, false, false, false, "another", 17)); // valid
    }

    #[rstest]
    #[case("ui:tech:rust", true)]
    #[case("ui:progress:50", true)]
    #[case("ui:donut:75", true)]
    #[case("ui:gauge:80", true)]
    #[case("ui:live:github:stars", true)]
    #[case("ui:swatch:red", true)]
    #[case("ui:version:1.0.0", true)]
    #[case("ui:license:MIT", true)]
    #[case("glyph:star", true)]
    #[case("swatch:blue", true)]
    #[case("ui:row", false)]           // Block component, NOT self-closing
    #[case("ui:row:align=center", false)]
    #[case("ui:tech-group", false)]    // Block component, NOT self-closing
    #[case("ui:tech-group:gap=2", false)]
    #[case("bold", false)]
    #[case("italic", false)]
    #[case("frame:gradient", false)]
    #[case("sup", false)]
    fn test_is_inherently_self_closing(#[case] content: &str, #[case] expected: bool) {
        assert_eq!(is_inherently_self_closing(content), expected);
    }
}
