//! Tech badge component handler

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
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

    // Allow custom bg and logo colors via params, with defaults
    let bg_color = params
        .get("bg")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("ui.bg"));
    let logo_color = params
        .get("logo")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("white"));

    let label = params.get("label").cloned();
    let border_color = params.get("border").map(|c| resolve_color(c));
    let border_width = params
        .get("border_width")
        .and_then(|v| v.parse().ok());
    let rx = params.get("rx").and_then(|v| v.parse().ok());

    Ok(ComponentOutput::Primitive(Primitive::Tech {
        name,
        bg_color,
        logo_color,
        style: style.to_string(),
        label,
        border_color,
        border_width,
        rx,
    }))
}
