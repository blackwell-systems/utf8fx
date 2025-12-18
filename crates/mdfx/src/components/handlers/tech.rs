//! Tech badge component handler

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use crate::renderer::svg::tech::{get_brand_color, get_logo_color_for_bg};
use std::collections::HashMap;

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
    let rx = params.get("rx").and_then(|v| v.parse().ok());

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
        text_color,
        font,
        source,
    }))
}
