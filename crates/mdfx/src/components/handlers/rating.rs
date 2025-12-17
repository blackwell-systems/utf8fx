//! Rating component handler (stars, hearts, etc.)

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle rating component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "rating component requires a value argument".to_string(),
        ));
    }

    // Parse value (first arg) - can be float like 3.5
    let value: f32 = args[0].parse().map_err(|_| {
        Error::ParseError(format!(
            "Invalid rating value '{}' - must be a number",
            args[0]
        ))
    })?;

    let max: u32 = params.get("max").and_then(|v| v.parse().ok()).unwrap_or(5);

    let size: u32 = params
        .get("size")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);

    let fill_color = params
        .get("fill")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("warning")); // gold/yellow default

    let empty_color = params
        .get("empty")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("slate"));

    let icon = params
        .get("icon")
        .cloned()
        .unwrap_or_else(|| "star".to_string());

    let spacing: u32 = params
        .get("spacing")
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);

    Ok(ComponentOutput::Primitive(Primitive::Rating {
        value,
        max,
        size,
        fill_color,
        empty_color,
        icon,
        spacing,
    }))
}
