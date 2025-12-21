//! Hover preview generation for mdfx templates
//!
//! Generates SVG previews embedded as data URIs for display in LSP hover popups.

use base64::{engine::general_purpose::STANDARD, Engine};
use mdfx_icons::brand_color;
use std::collections::HashMap;

/// Generate a hover preview for a tech badge
///
/// Returns markdown with an embedded SVG image via data URI
/// The palette is used to resolve color names like "accent" to hex values
pub fn tech_badge_preview(
    tech_name: &str,
    params: &[(String, String)],
    palette: &HashMap<String, String>,
) -> String {
    use badgefx::{render, BadgeBuilder, BadgeStyle, Chevron, Corners};

    // Build the badge
    let mut builder = BadgeBuilder::new(tech_name);

    // Collect border params (need both color and width)
    let mut border_color: Option<&str> = None;
    let mut border_width: Option<u32> = None;
    let mut is_outline = false;

    // First pass: collect border params and check style
    for (key, value) in params {
        match key.as_str() {
            "border" => border_color = Some(value.as_str()),
            "border_width" => border_width = value.parse().ok(),
            "style" if value == "outline" || value == "ghost" => is_outline = true,
            _ => {}
        }
    }

    // Apply parameters
    for (key, value) in params {
        match key.as_str() {
            "style" => {
                let style = BadgeStyle::parse(value);
                builder = builder.style(style);
                if value == "outline" || value == "ghost" {
                    builder = builder.outline();
                }
            }
            "bg" => {
                builder = builder.bg_color(resolve_color(value, palette));
            }
            "bg_left" => {
                builder = builder.bg_left(resolve_color(value, palette));
            }
            "bg_right" => {
                builder = builder.bg_right(resolve_color(value, palette));
            }
            "logo" | "logo_color" => {
                builder = builder.logo_color(resolve_color(value, palette));
            }
            "text" | "text_color" => {
                builder = builder.text_color(resolve_color(value, palette));
            }
            "label" => {
                builder = builder.label(value);
            }
            "rx" => {
                if let Ok(r) = value.parse::<u32>() {
                    builder = builder.corners(Corners::uniform(r));
                }
            }
            "corners" => {
                // Parse "tl,tr,br,bl" format
                let vals: Vec<u32> = value.split(',').filter_map(|s| s.parse().ok()).collect();
                if vals.len() == 4 {
                    builder = builder.corners(Corners::custom(vals[0], vals[1], vals[2], vals[3]));
                }
            }
            "logo_size" | "icon_size" => {
                if let Ok(size) = value.parse::<u32>() {
                    builder = builder.logo_size(size);
                }
            }
            "border_full" => {
                if value == "true" || value == "1" {
                    builder = builder.border_full();
                }
            }
            "divider" => {
                if value == "true" || value == "1" {
                    builder = builder.divider();
                }
            }
            "raised" => {
                if let Ok(px) = value.parse::<u32>() {
                    builder = builder.raised(px);
                }
            }
            "chevron" => {
                let depth = 10.0;
                let chev = match value.as_str() {
                    "left" => Chevron::left(depth),
                    "right" => Chevron::right(depth),
                    "both" => Chevron::both(depth),
                    _ => Chevron::right(depth),
                };
                builder = builder.chevron(chev);
            }
            "font" => {
                builder = builder.font(value);
            }
            "icon" => {
                builder = builder.custom_icon(value);
            }
            _ => {}
        }
    }

    // Apply border if specified
    if let Some(color) = border_color {
        let width = border_width.unwrap_or(if is_outline { 2 } else { 1 });
        builder = builder.border(resolve_color(color, palette), width);
    } else if let Some(width) = border_width {
        builder = builder.border("#FFFFFF", width);
    }

    // If no explicit label was set, use tech_name as-is to preserve user's case
    // e.g., "Rust" stays "Rust", "RUST" stays "RUST"
    if !params.iter().any(|(k, _)| k == "label") {
        builder = builder.label(tech_name);
    }

    let badge = builder.build();
    let svg = render(&badge);

    // Encode as base64 data URI
    let b64 = STANDARD.encode(svg.as_bytes());
    let color_info = brand_color(tech_name)
        .map(|c| format!("Brand color: `#{}`", c))
        .unwrap_or_else(|| "Custom technology".to_string());

    format!(
        "**Tech Badge: {}**\n\n\
        ![preview](data:image/svg+xml;base64,{})\n\n\
        {}",
        tech_name, b64, color_info
    )
}

/// Resolve a color value - either a palette name or a hex color
/// Returns the hex color with # prefix
pub fn resolve_color(color: &str, palette: &HashMap<String, String>) -> String {
    // Check if it's a palette color name
    if let Some(hex) = palette.get(color) {
        return format!("#{}", hex);
    }
    // Otherwise treat as hex color
    if color.starts_with('#') {
        color.to_string()
    } else {
        format!("#{}", color)
    }
}

/// Resolve a color and strip the # prefix (for SVG fill attributes)
pub fn resolve_color_hex(color: &str, palette: &HashMap<String, String>) -> String {
    resolve_color(color, palette).trim_start_matches('#').to_string()
}

/// Generate a hover preview for a color swatch
pub fn swatch_preview(color: &str, size: u32) -> String {
    // Simple SVG swatch
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
  <rect width="{}" height="{}" fill="#{}"/>
</svg>"##,
        size, size, size, size, color
    );

    let b64 = STANDARD.encode(svg.as_bytes());

    format!(
        "**Color Swatch**\n\n\
        ![preview](data:image/svg+xml;base64,{})\n\n\
        Hex: `#{}`",
        b64, color
    )
}

/// Generate a hover preview for a progress bar
pub fn progress_preview(percent: u8, width: u32, height: u32, fill: &str, track: &str) -> String {
    let fill_width = (width as f32 * percent as f32 / 100.0) as u32;
    let rx = height / 3;

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
  <rect x="0" y="0" width="{}" height="{}" rx="{}" fill="#{}"/>
  <rect x="0" y="0" width="{}" height="{}" rx="{}" fill="#{}"/>
</svg>"##,
        width, height, width, height, rx, track, fill_width, height, rx, fill
    );

    let b64 = STANDARD.encode(svg.as_bytes());

    format!(
        "**Progress Bar: {}%**\n\n\
        ![preview](data:image/svg+xml;base64,{})\n\n\
        Size: {}×{}px",
        percent, b64, width, height
    )
}

/// Generate a hover preview for a donut chart
pub fn donut_preview(percent: u8, size: u32, thickness: u32, fill: &str, track: &str) -> String {
    let center = size as f32 / 2.0;
    let radius = center - thickness as f32 / 2.0;
    let circumference = 2.0 * std::f32::consts::PI * radius;
    let dash = circumference * percent as f32 / 100.0;
    let gap = circumference - dash;

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <circle cx="{}" cy="{}" r="{}" fill="none" stroke="#{}" stroke-width="{}"/>
  <circle cx="{}" cy="{}" r="{}" fill="none" stroke="#{}" stroke-width="{}"
          stroke-dasharray="{} {}" transform="rotate(-90 {} {})"/>
</svg>"##,
        size,
        size,
        size,
        size,
        center,
        center,
        radius,
        track,
        thickness,
        center,
        center,
        radius,
        fill,
        thickness,
        dash,
        gap,
        center,
        center
    );

    let b64 = STANDARD.encode(svg.as_bytes());

    format!(
        "**Donut Chart: {}%**\n\n\
        ![preview](data:image/svg+xml;base64,{})\n\n\
        Size: {}px, Thickness: {}px",
        percent, b64, size, thickness
    )
}

/// Generate a hover preview for a gauge meter
pub fn gauge_preview(percent: u8, size: u32, thickness: u32, fill: &str, track: &str) -> String {
    let width = size;
    let height = size / 2 + thickness;
    let center_x = size as f32 / 2.0;
    let center_y = size as f32 / 2.0;
    let radius = center_x - thickness as f32 / 2.0;

    // Semi-circle arc (180 degrees)
    let circumference = std::f32::consts::PI * radius;
    let dash = circumference * percent as f32 / 100.0;
    let gap = circumference - dash;

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <path d="M {} {} A {} {} 0 0 1 {} {}" fill="none" stroke="#{}" stroke-width="{}" stroke-linecap="round"/>
  <path d="M {} {} A {} {} 0 0 1 {} {}" fill="none" stroke="#{}" stroke-width="{}" stroke-linecap="round"
        stroke-dasharray="{} {}"/>
</svg>"##,
        width,
        height,
        width,
        height,
        // Track arc
        thickness as f32 / 2.0,
        center_y,
        radius,
        radius,
        size as f32 - thickness as f32 / 2.0,
        center_y,
        track,
        thickness,
        // Fill arc
        thickness as f32 / 2.0,
        center_y,
        radius,
        radius,
        size as f32 - thickness as f32 / 2.0,
        center_y,
        fill,
        thickness,
        dash,
        gap
    );

    let b64 = STANDARD.encode(svg.as_bytes());

    format!(
        "**Gauge: {}%**\n\n\
        ![preview](data:image/svg+xml;base64,{})\n\n\
        Size: {}px, Thickness: {}px",
        percent, b64, size, thickness
    )
}

/// Generate a hover preview for a rating
pub fn rating_preview(value: f32, max: u32, size: u32, fill: &str, empty: &str) -> String {
    let total_width = max * size + (max - 1) * 2;
    let mut stars = String::new();

    for i in 0..max {
        let x = i * (size + 2);
        let fill_amount = (value - i as f32).clamp(0.0, 1.0);

        // Star path (scaled to size)
        let scale = size as f32 / 24.0;
        let star_path = "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z";

        if fill_amount >= 1.0 {
            // Full star
            stars.push_str(&format!(
                r##"<g transform="translate({}, 0) scale({})"><path d="{}" fill="#{}"/></g>"##,
                x, scale, star_path, fill
            ));
        } else if fill_amount > 0.0 {
            // Partial star with clip
            let clip_id = format!("clip{}", i);
            let clip_width = (24.0 * fill_amount) as u32;
            stars.push_str(&format!(
                r##"<defs><clipPath id="{}"><rect x="0" y="0" width="{}" height="24"/></clipPath></defs>
<g transform="translate({}, 0) scale({})">
  <path d="{}" fill="#{}"/>
  <path d="{}" fill="#{}" clip-path="url(#{})"/>
</g>"##,
                clip_id, clip_width, x, scale, star_path, empty, star_path, fill, clip_id
            ));
        } else {
            // Empty star
            stars.push_str(&format!(
                r##"<g transform="translate({}, 0) scale({})"><path d="{}" fill="#{}"/></g>"##,
                x, scale, star_path, empty
            ));
        }
    }

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">{}</svg>"#,
        total_width, size, stars
    );

    let b64 = STANDARD.encode(svg.as_bytes());

    format!(
        "**Rating: {:.1}/{max}**\n\n\
        ![preview](data:image/svg+xml;base64,{})\n\n\
        Icon size: {}px",
        value, b64, size
    )
}

/// Parse parameters from a template string like "tech:rust:style=flat:bg=FF0000"
/// Returns (positional_parts joined by ':', key=value params)
/// Example: "tech:rust:style=flat" -> ("tech:rust", [(style, flat)])
pub fn parse_template_params(template: &str) -> (String, Vec<(String, String)>) {
    let parts: Vec<&str> = template.split(':').collect();

    let mut positional = Vec::new();
    let mut params = Vec::new();

    for part in parts {
        if let Some((k, v)) = part.split_once('=') {
            params.push((k.to_string(), v.to_string()));
        } else {
            positional.push(part);
        }
    }

    (positional.join(":"), params)
}

/// Get a parameter value or default
pub fn get_param<'a>(params: &'a [(String, String)], key: &str, default: &'a str) -> &'a str {
    params
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
        .unwrap_or(default)
}

/// Get a parameter value as u32 or default
pub fn get_param_u32(params: &[(String, String)], key: &str, default: u32) -> u32 {
    params
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_matches_renderer_for_complex_badge() {
        // Test: {{ui:tech:rust:bg=1a0a0a:logo=DEA584:border=DEA584:border_width=2:rx=6/}}
        use badgefx::{render, BadgeBuilder, Corners};

        let palette = HashMap::new();

        // What preview generates
        let params = vec![
            ("bg".to_string(), "1a0a0a".to_string()),
            ("logo".to_string(), "DEA584".to_string()),
            ("border".to_string(), "DEA584".to_string()),
            ("border_width".to_string(), "2".to_string()),
            ("rx".to_string(), "6".to_string()),
        ];

        // Get preview markdown (contains base64 SVG)
        let preview_md = tech_badge_preview("rust", &params, &palette);
        assert!(preview_md.contains("data:image/svg+xml;base64,"));

        // Manually build what we expect - note: label preserves user's case
        let expected_badge = BadgeBuilder::new("rust")
            .label("rust") // preserves user's original case
            .bg_color("#1a0a0a")
            .logo_color("#DEA584")
            .border("#DEA584", 2)
            .corners(Corners::uniform(6))
            .build();
        let expected_svg = render(&expected_badge);

        // Extract base64 from preview and decode
        let b64_start = preview_md.find("base64,").unwrap() + 7;
        let b64_end = preview_md[b64_start..].find(')').unwrap() + b64_start;
        let b64 = &preview_md[b64_start..b64_end];
        let preview_svg = String::from_utf8(STANDARD.decode(b64).unwrap()).unwrap();

        assert_eq!(
            preview_svg, expected_svg,
            "Preview SVG should match expected SVG"
        );
    }

    #[test]
    fn test_preview_preserves_case() {
        use badgefx::{render, BadgeBuilder};

        let palette = HashMap::new();

        // Test uppercase preserves case
        let preview_upper = tech_badge_preview("RUST", &[], &palette);
        let expected_upper = BadgeBuilder::new("RUST").label("RUST").build();
        let expected_upper_svg = render(&expected_upper);

        let b64_start = preview_upper.find("base64,").unwrap() + 7;
        let b64_end = preview_upper[b64_start..].find(')').unwrap() + b64_start;
        let b64 = &preview_upper[b64_start..b64_end];
        let preview_svg = String::from_utf8(STANDARD.decode(b64).unwrap()).unwrap();

        assert_eq!(preview_svg, expected_upper_svg);
        assert!(preview_svg.contains(">RUST<")); // Label text should be uppercase

        // Test mixed case preserves case
        let preview_mixed = tech_badge_preview("Rust", &[], &palette);
        let expected_mixed = BadgeBuilder::new("Rust").label("Rust").build();
        let expected_mixed_svg = render(&expected_mixed);

        let b64_start = preview_mixed.find("base64,").unwrap() + 7;
        let b64_end = preview_mixed[b64_start..].find(')').unwrap() + b64_start;
        let b64 = &preview_mixed[b64_start..b64_end];
        let preview_svg = String::from_utf8(STANDARD.decode(b64).unwrap()).unwrap();

        assert_eq!(preview_svg, expected_mixed_svg);
        assert!(preview_svg.contains(">Rust<")); // Label text should be mixed case
    }

    #[test]
    fn test_resolve_color_with_palette() {
        let mut palette = HashMap::new();
        palette.insert("accent".to_string(), "FF5500".to_string());
        palette.insert("primary".to_string(), "007BFF".to_string());

        // Palette color resolves
        assert_eq!(resolve_color("accent", &palette), "#FF5500");
        assert_eq!(resolve_color("primary", &palette), "#007BFF");

        // Hex colors pass through
        assert_eq!(resolve_color("FF0000", &palette), "#FF0000");
        assert_eq!(resolve_color("#00FF00", &palette), "#00FF00");

        // Unknown names treated as hex
        assert_eq!(resolve_color("unknown", &palette), "#unknown");
    }

    #[test]
    fn test_parse_template_params() {
        let (parts, params) = parse_template_params("tech:rust:style=flat:bg=FF0000");
        assert_eq!(parts, "tech:rust");
        assert_eq!(
            params,
            vec![
                ("style".to_string(), "flat".to_string()),
                ("bg".to_string(), "FF0000".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_template_params_no_params() {
        let (parts, params) = parse_template_params("tech:rust");
        assert_eq!(parts, "tech:rust");
        assert!(params.is_empty());
    }
}
