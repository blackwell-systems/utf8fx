//! Corner grouping for badge rows - extracted from mdfx tech_group.rs

/// Generate corner radii for a badge in a group layout
///
/// This function calculates appropriate corner radii for badges arranged in rows,
/// ensuring visual continuity by making internal corners square while keeping
/// external corners rounded.
///
/// # Arguments
///
/// * `position` - Position in the group: "first", "middle", "last", or "single"  
/// * `base_radius` - The base corner radius to use for external corners
///
/// # Returns
///
/// Array of corner radii in order: [top-left, top-right, bottom-right, bottom-left]
///
/// # Examples
///
/// ```rust
/// use badgefx::group::corner_radii_for_position;
///
/// // First badge in a row - rounded left, square right
/// let radii = corner_radii_for_position("first", 4);
/// assert_eq!(radii, [4, 0, 0, 4]);
///
/// // Middle badge - all square corners
/// let radii = corner_radii_for_position("middle", 4);  
/// assert_eq!(radii, [0, 0, 0, 0]);
///
/// // Last badge in a row - square left, rounded right
/// let radii = corner_radii_for_position("last", 4);
/// assert_eq!(radii, [0, 4, 4, 0]);
///
/// // Single badge - all corners rounded
/// let radii = corner_radii_for_position("single", 4);
/// assert_eq!(radii, [4, 4, 4, 4]);
/// ```
pub fn corner_radii_for_position(position: &str, base_radius: u32) -> [u32; 4] {
    match position {
        "first" => [base_radius, 0, 0, base_radius], // Rounded left, square right
        "middle" => [0, 0, 0, 0],                    // All square (internal badge)
        "last" => [0, base_radius, base_radius, 0],  // Square left, rounded right
        _ => [base_radius, base_radius, base_radius, base_radius], // Single or unknown - all rounded
    }
}

/// Determine the position of a badge within a group layout
///
/// Given the index of a badge and the total count, determines its semantic
/// position for corner radius calculation.
///
/// # Arguments
///
/// * `index` - Zero-based index of the current badge
/// * `total` - Total number of badges in the group
///
/// # Returns
///
/// Position string: "first", "middle", "last", or "single"
///
/// # Examples
///
/// ```rust
/// use badgefx::group::position_in_group;
///
/// // Single badge
/// assert_eq!(position_in_group(0, 1), "single");
///
/// // Three badges  
/// assert_eq!(position_in_group(0, 3), "first");
/// assert_eq!(position_in_group(1, 3), "middle");
/// assert_eq!(position_in_group(2, 3), "last");
/// ```
pub fn position_in_group(index: usize, total: usize) -> &'static str {
    if total <= 1 {
        "single"
    } else if index == 0 {
        "first"
    } else if index == total - 1 {
        "last"
    } else {
        "middle"
    }
}

/// Calculate spacing between badges in a group
///
/// Determines the horizontal gap between badges when arranged in a row.
/// Different styles may have different spacing requirements.
///
/// # Arguments
///
/// * `style` - Badge style name (e.g., "flat", "plastic", etc.)
/// * `total_badges` - Number of badges in the group
///
/// # Returns
///
/// Spacing in pixels between adjacent badges
pub fn badge_spacing(style: &str, total_badges: usize) -> u32 {
    if total_badges <= 1 {
        return 0;
    }

    match style.to_lowercase().as_str() {
        "plastic" => 2,       // Plastic badges need more separation for shadows
        "social" => 4,        // Social style uses larger gaps
        "for-the-badge" => 6, // Large badges need proportional spacing
        _ => 1,               // Flat styles use minimal spacing
    }
}

/// Generate SVG group layout for multiple badges
///
/// Creates an SVG group element that positions multiple badges in a row
/// with appropriate spacing and corner adjustments.
///
/// # Arguments
///
/// * `badges` - Vector of badge SVG strings
/// * `style` - Badge style for spacing calculation
/// * `vertical_align` - Vertical alignment: "top", "middle", "bottom"
///
/// # Returns
///
/// Complete SVG group element containing all positioned badges
///
/// # Examples
///
/// ```rust
/// use badgefx::group::group_badges_svg;
///
/// let badges = vec![
///     r#"<svg width="50" height="20">badge1</svg>"#.to_string(),
///     r#"<svg width="60" height="20">badge2</svg>"#.to_string(),
/// ];
/// let group_svg = group_badges_svg(badges, "flat", "middle");
/// assert!(group_svg.contains("badge-group"));
/// ```
pub fn group_badges_svg(badges: Vec<String>, style: &str, vertical_align: &str) -> String {
    if badges.is_empty() {
        return String::new();
    }

    if badges.len() == 1 {
        return badges.into_iter().next().unwrap();
    }

    let spacing = badge_spacing(style, badges.len());
    let mut group_svg = String::from(r#"<g class="badge-group">"#);
    let mut current_x = 0u32;

    // Get height from first badge (assumes uniform height)
    let (_, badge_height) = extract_badge_dimensions(&badges[0]);
    let mut last_badge_width = 0u32;

    for (index, badge_svg) in badges.iter().enumerate() {
        // Extract dimensions for this badge
        let (badge_width, _) = extract_badge_dimensions(badge_svg);
        last_badge_width = badge_width;

        // Calculate vertical offset based on alignment
        let y_offset = match vertical_align {
            "top" => 0,
            "bottom" => 0, // Badges naturally align to bottom
            _ => 0,        // "middle" - no offset needed for uniform height badges
        };

        // Position each badge
        group_svg.push_str(&format!(
            r#"<g transform="translate({}, {})">"#,
            current_x, y_offset
        ));

        // Extract just the content (remove outer SVG wrapper)
        let badge_content = extract_svg_content(badge_svg);
        group_svg.push_str(&badge_content);
        group_svg.push_str("</g>");

        // Advance position for next badge
        if index < badges.len() - 1 {
            current_x += badge_width + spacing;
        }
    }

    // Calculate total dimensions
    let total_width = current_x + last_badge_width;
    let total_height = badge_height;

    // Wrap in final SVG container
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">{}</g></svg>"#,
        total_width, total_height, total_width, total_height, group_svg
    )
}

/// Extract width and height from an SVG string
///
/// Parses the SVG element to extract its width and height attributes.
/// Returns default values if parsing fails.
fn extract_badge_dimensions(svg: &str) -> (u32, u32) {
    // Simple regex-free parsing for width and height
    let width = svg
        .find("width=\"")
        .and_then(|start| {
            let start = start + 7; // Skip 'width="'
            svg[start..]
                .find('"')
                .and_then(|end| svg[start..start + end].parse().ok())
        })
        .unwrap_or(100); // Default width

    let height = svg
        .find("height=\"")
        .and_then(|start| {
            let start = start + 8; // Skip 'height="'
            svg[start..]
                .find('"')
                .and_then(|end| svg[start..start + end].parse().ok())
        })
        .unwrap_or(20); // Default height

    (width, height)
}

/// Extract the inner content of an SVG (everything between <svg> and </svg>)
///
/// Removes the outer SVG wrapper to allow content to be embedded in a group.
fn extract_svg_content(svg: &str) -> String {
    // Find the closing > of the opening <svg> tag
    if let Some(start) = svg.find('>') {
        // Find the start of the closing </svg> tag
        if let Some(end) = svg.rfind("</svg>") {
            return svg[start + 1..end].to_string();
        }
    }

    // Fallback: return original if parsing fails
    svg.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Corner Radii (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("first", 4, [4, 0, 0, 4])]
    #[case("middle", 4, [0, 0, 0, 0])]
    #[case("last", 4, [0, 4, 4, 0])]
    #[case("single", 4, [4, 4, 4, 4])]
    #[case("unknown", 4, [4, 4, 4, 4])]
    fn test_corner_radii_positions(
        #[case] position: &str,
        #[case] radius: u32,
        #[case] expected: [u32; 4],
    ) {
        assert_eq!(corner_radii_for_position(position, radius), expected);
    }

    // ========================================================================
    // Position in Group (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(0, 0, "single")]
    #[case(0, 1, "single")]
    #[case(0, 2, "first")]
    #[case(1, 2, "last")]
    #[case(0, 3, "first")]
    #[case(1, 3, "middle")]
    #[case(2, 3, "last")]
    #[case(0, 5, "first")]
    #[case(1, 5, "middle")]
    #[case(2, 5, "middle")]
    #[case(3, 5, "middle")]
    #[case(4, 5, "last")]
    fn test_position_in_group(#[case] index: usize, #[case] total: usize, #[case] expected: &str) {
        assert_eq!(position_in_group(index, total), expected);
    }

    // ========================================================================
    // Badge Spacing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("flat", 1, 0)]
    #[case("flat", 3, 1)]
    #[case("plastic", 3, 2)]
    #[case("social", 3, 4)]
    #[case("for-the-badge", 3, 6)]
    #[case("unknown", 3, 1)]
    fn test_badge_spacing(#[case] style: &str, #[case] count: usize, #[case] expected: u32) {
        assert_eq!(badge_spacing(style, count), expected);
    }

    // ========================================================================
    // Dimension Extraction (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(r#"<svg width="100" height="20">content</svg>"#, (100, 20))]
    #[case(r#"<svg height="25" width="150">content</svg>"#, (150, 25))]
    #[case("<svg>content</svg>", (100, 20))] // defaults
    fn test_extract_badge_dimensions(#[case] svg: &str, #[case] expected: (u32, u32)) {
        assert_eq!(extract_badge_dimensions(svg), expected);
    }

    // ========================================================================
    // SVG Content Extraction (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(r#"<svg width="100" height="20"><path d="..."/></svg>"#, r#"<path d="..."/>"#)]
    #[case(r#"<svg xmlns="..." width="100" height="20"><g><path/></g></svg>"#, r#"<g><path/></g>"#)]
    fn test_extract_svg_content(#[case] svg: &str, #[case] expected: &str) {
        assert_eq!(extract_svg_content(svg), expected);
    }

    #[test]
    fn test_group_badges_svg_empty() {
        let result = group_badges_svg(vec![], "flat", "middle");
        assert_eq!(result, "");
    }

    #[test]
    fn test_group_badges_svg_single() {
        let badges = vec!["<svg>single</svg>".to_string()];
        let result = group_badges_svg(badges.clone(), "flat", "middle");
        assert_eq!(result, badges[0]);
    }

    #[test]
    fn test_group_badges_svg_multiple() {
        let badges = vec![
            r#"<svg width="50" height="20">badge1</svg>"#.to_string(),
            r#"<svg width="60" height="20">badge2</svg>"#.to_string(),
        ];
        let result = group_badges_svg(badges, "flat", "middle");

        assert!(result.contains("badge1"));
        assert!(result.contains("badge2"));
        assert!(result.contains("translate(0, 0)"));
        assert!(result.contains("translate(51, 0)")); // 50 + 1 spacing
        assert!(result.contains(r#"width="111""#)); // 50 + 1 + 60
    }
}
