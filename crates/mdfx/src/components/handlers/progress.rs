//! Progress bar component handler

use super::{
    parse_bool, parse_param, parse_thumb_config, resolve_color_opt, resolve_color_with_default,
    resolve_color_with_fallback,
};
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

    let width: u32 = parse_param(params, "width", 100);
    let height: u32 = parse_param(params, "height", 10);
    let fill_height: u32 = parse_param(params, "fill_height", height);
    let rx: u32 = parse_param(params, "rx", 3);

    let track_color =
        resolve_color_with_fallback(params, &["track", "color"], "gray", &resolve_color);
    let fill_color = resolve_color_with_default(params, "fill", "pink", &resolve_color);

    let show_label = parse_bool(params, "label", false);
    let label_color = resolve_color_opt(params, "label_color", &resolve_color);
    let border_color = resolve_color_opt(params, "border", &resolve_color);
    let border_width: u32 = parse_param(
        params,
        "border_width",
        if border_color.is_some() { 1 } else { 0 },
    );

    // Parse thumb configuration (enables slider mode)
    let thumb = parse_thumb_config(params, &resolve_color);

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
        thumb,
    }))
}
