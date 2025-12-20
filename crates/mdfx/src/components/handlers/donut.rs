//! Donut/ring chart component handler

use super::{
    parse_bool, parse_param, parse_param_opt, resolve_color_opt, resolve_color_with_default,
};
use crate::components::ComponentOutput;
use crate::error::{Error, Result};
use crate::primitive::Primitive;
use std::collections::HashMap;

/// Handle donut component expansion
pub fn handle(
    args: &[String],
    params: &HashMap<String, String>,
    resolve_color: impl Fn(&str) -> String,
) -> Result<ComponentOutput> {
    if args.is_empty() {
        return Err(Error::ParseError(
            "donut component requires a percentage argument".to_string(),
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

    let size: u32 = parse_param(params, "size", 40);
    let thickness: u32 = parse_param(params, "thickness", 4);

    let track_color = resolve_color_with_default(params, "track", "gray", &resolve_color);
    let fill_color = resolve_color_with_default(params, "fill", "pink", &resolve_color);

    let show_label = parse_bool(params, "label", false);
    let label_color = resolve_color_opt(params, "label_color", &resolve_color);

    // Thumb (slider mode)
    let thumb_size: Option<u32> = parse_param_opt(params, "thumb");
    let thumb_color = resolve_color_opt(params, "thumb_color", &resolve_color);
    let thumb_border = resolve_color_opt(params, "thumb_border", &resolve_color);
    let thumb_border_width: u32 = parse_param(params, "thumb_border_width", 0);

    Ok(ComponentOutput::Primitive(Primitive::Donut {
        percent,
        size,
        thickness,
        track_color,
        fill_color,
        show_label,
        label_color,
        thumb_size,
        thumb_color,
        thumb_border,
        thumb_border_width,
    }))
}
