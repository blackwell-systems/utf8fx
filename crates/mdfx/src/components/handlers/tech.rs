//! Tech badge component handler

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use crate::renderer::svg::tech::{get_brand_color, get_logo_color_for_bg};
use std::collections::HashMap;

/// Parse corners parameter - supports presets and custom values
/// Returns (uniform_rx, per_corner_radii)
fn parse_corners(
    rx_param: Option<&String>,
    corners_param: Option<&String>,
) -> (Option<u32>, Option<[u32; 4]>) {
    // Check for corners preset first
    if let Some(corners) = corners_param {
        let default_rx = 6u32; // Default corner radius for presets
        match corners.as_str() {
            "left" => return (None, Some([default_rx, 0, 0, default_rx])),
            "right" => return (None, Some([0, default_rx, default_rx, 0])),
            "none" => return (None, Some([0, 0, 0, 0])),
            "all" => return (Some(default_rx), None),
            _ => {}
        }
    }

    // Check if rx contains comma-separated values for per-corner
    if let Some(rx_str) = rx_param {
        if rx_str.contains(',') {
            let parts: Vec<u32> = rx_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            if parts.len() == 4 {
                return (None, Some([parts[0], parts[1], parts[2], parts[3]]));
            }
        } else {
            // Single value - uniform radius
            if let Ok(rx) = rx_str.parse() {
                return (Some(rx), None);
            }
        }
    }

    (None, None)
}

/// Handle tech component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    style: &str,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "tech component requires a technology name argument".to_string(),
        ));
    }
    let name = args[0].clone();

    // Use brand color if available, otherwise fall back to dark1
    let default_bg = get_brand_color(&name)
        .map(|c| c.to_string())
        .unwrap_or_else(|| resolve_color("dark1"));

    // Allow custom bg and logo colors via params
    let bg_color = params
        .get("bg")
        .map(|c| resolve_color(c))
        .unwrap_or(default_bg.clone());

    // Use intelligent logo color based on background luminance if not specified
    let logo_color = params
        .get("logo")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| get_logo_color_for_bg(&bg_color).to_string());

    // Default label to tech name for shields.io style badges
    let label = params.get("label").cloned().or_else(|| Some(name.clone()));
    let border_color = params.get("border").map(|c| resolve_color(c));
    let border_width = params.get("border_width").and_then(|v| v.parse().ok());

    // Parse rx - can be single value or comma-separated for per-corner
    let rx_param = params.get("rx");
    let (rx, corners) = parse_corners(rx_param, params.get("corners"));

    // Text color defaults to intelligent selection based on right segment color
    let text_color = params
        .get("text_color")
        .or_else(|| params.get("text"))
        .or_else(|| params.get("color"))
        .map(|c| resolve_color(c));

    // Font family (optional)
    let font = params
        .get("font")
        .or_else(|| params.get("font_family"))
        .cloned();

    // Rendering source: "svg" (default) or "shields" (shields.io URL)
    let source = params.get("source").cloned();

    Ok(ComponentOutput::Primitive(Primitive::Tech {
        name,
        bg_color,
        logo_color,
        style: style.to_string(),
        label,
        border_color,
        border_width,
        rx,
        corners,
        text_color,
        font,
        source,
    }))
}
