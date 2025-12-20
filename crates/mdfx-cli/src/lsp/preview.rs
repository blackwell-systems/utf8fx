//! Hover preview generation for mdfx templates
//!
//! Generates SVG previews embedded as data URIs for display in LSP hover popups.

use base64::{engine::general_purpose::STANDARD, Engine};
use mdfx_icons::brand_color;

/// Generate a hover preview for a tech badge
///
/// Returns markdown with an embedded SVG image via data URI
pub fn tech_badge_preview(tech_name: &str, params: &[(String, String)]) -> String {
    use badgefx::{render, BadgeBuilder, BadgeStyle};

    // Build the badge
    let mut builder = BadgeBuilder::new(tech_name);

    // Apply parameters
    for (key, value) in params {
        match key.as_str() {
            "style" => {
                builder = match value.as_str() {
                    "flat" => builder.style(BadgeStyle::Flat),
                    "flat-square" | "square" => builder.style(BadgeStyle::FlatSquare),
                    "plastic" => builder.style(BadgeStyle::Plastic),
                    "for-the-badge" | "badge" => builder.style(BadgeStyle::ForTheBadge),
                    "social" => builder.style(BadgeStyle::Social),
                    _ => builder,
                };
            }
            "bg" => {
                builder = builder.bg_color(value);
            }
            "text" => {
                builder = builder.text_color(value);
            }
            "label" => {
                builder = builder.label(value);
            }
            _ => {}
        }
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

/// Parse parameters from a template string like "rust:style=flat:bg=FF0000"
pub fn parse_template_params(template: &str) -> (String, Vec<(String, String)>) {
    let parts: Vec<&str> = template.split(':').collect();
    let name = parts.first().unwrap_or(&"").to_string();

    let params: Vec<(String, String)> = parts
        .iter()
        .skip(1)
        .filter_map(|p| {
            let kv: Vec<&str> = p.splitn(2, '=').collect();
            if kv.len() == 2 {
                Some((kv[0].to_string(), kv[1].to_string()))
            } else {
                None
            }
        })
        .collect();

    (name, params)
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
