//! Progress bar component handler

use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle progress component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "progress component requires a percentage argument".to_string(),
        ));
    }

    // Parse percentage (first arg)
    let percent: u8 = args[0].parse().map_err(|_| {
        Error::ParseError(format!(
            "Invalid percentage '{}' - must be a number 0-100",
            args[0]
        ))
    })?;
    let percent = percent.min(100);

    let width: u32 = params
        .get("width")
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);

    let height: u32 = params
        .get("height")
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);

    let fill_height: u32 = params
        .get("fill_height")
        .and_then(|v| v.parse().ok())
        .unwrap_or(height);

    let track_color = params
        .get("track")
        .or(params.get("color"))
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("slate"));

    let fill_color = params
        .get("fill")
        .map(|c| resolve_color(c))
        .unwrap_or_else(|| resolve_color("accent"));

    let rx: u32 = params.get("rx").and_then(|v| v.parse().ok()).unwrap_or(3);

    let show_label = params
        .get("label")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let label_color = params.get("label_color").map(|c| resolve_color(c));

    let border_color = params.get("border").map(|c| resolve_color(c));

    let border_width: u32 = params
        .get("border_width")
        .and_then(|v| v.parse().ok())
        .unwrap_or(if border_color.is_some() { 1 } else { 0 });

    // Slider/thumb mode params
    let thumb_size: Option<u32> = params.get("thumb").and_then(|v| v.parse().ok());
    let thumb_width: Option<u32> = params.get("thumb_width").and_then(|v| v.parse().ok());
    let thumb_color = params.get("thumb_color").map(|c| resolve_color(c));
    let thumb_shape = params
        .get("thumb_shape")
        .cloned()
        .unwrap_or_else(|| "circle".to_string());

    Ok(ComponentOutput::Primitive(Primitive::Progress {
        percent,
        width,
        height,
        track_color,
        fill_color,
        fill_height,
        rx,
        show_label,
        label_color,
        border_color,
        border_width,
        thumb_size,
        thumb_width,
        thumb_color,
        thumb_shape,
    }))
}
