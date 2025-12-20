//! Semantic token generation for the mdfx LSP
//!
//! Provides syntax highlighting through LSP semantic tokens.

use crate::lsp::parser::find_templates;
use mdfx::components::params;
use mdfx::Registry;
use mdfx_icons::list_icons;
use std::collections::HashSet;
use tower_lsp::lsp_types::SemanticToken;

/// Token type indices (must match the legend in handlers.rs initialize)
pub const TOKEN_NAMESPACE: u32 = 0; // component prefix
pub const TOKEN_TYPE: u32 = 1; // tech name
pub const TOKEN_PARAMETER: u32 = 2; // parameter name
pub const TOKEN_STRING: u32 = 3; // parameter value
pub const TOKEN_VARIABLE: u32 = 4; // palette color name
pub const TOKEN_KEYWORD: u32 = 5; // style name
pub const TOKEN_FUNCTION: u32 = 6; // frame name
pub const TOKEN_INVALID: u32 = 7; // invalid items

/// Tokenize document for semantic highlighting
/// Returns delta-encoded semantic token data
pub fn tokenize_document(registry: &Registry, text: &str) -> Vec<SemanticToken> {
    let icon_list = list_icons();
    let valid_tech_names: HashSet<&str> = icon_list.iter().map(|s| s.as_ref()).collect();

    let mut tokens = Vec::new();
    let mut prev_line = 0u32;
    let mut prev_char = 0u32;

    for (line_num, line) in text.lines().enumerate() {
        let line_num = line_num as u32;

        // Find all templates in this line using simple string scanning
        for (start, is_closing, _is_self_closing, _is_malformed, content, _end) in
            find_templates(line)
        {
            let template_start = start + 2 + if is_closing { 1 } else { 0 };

            let new_tokens =
                tokenize_template(registry, content, template_start, &valid_tech_names, is_closing);

            // Convert to delta-encoded format
            for (offset, length, token_type, token_modifiers) in new_tokens {
                let char_pos = offset as u32;

                let delta_line = line_num - prev_line;
                let delta_start = if delta_line == 0 {
                    char_pos - prev_char
                } else {
                    char_pos
                };

                tokens.push(SemanticToken {
                    delta_line,
                    delta_start,
                    length: length as u32,
                    token_type,
                    token_modifiers_bitset: token_modifiers,
                });

                prev_line = line_num;
                prev_char = char_pos;
            }
        }
    }

    tokens
}

/// Tokenize a single template's content
/// Returns: Vec<(offset, length, token_type, token_modifiers)>
pub fn tokenize_template(
    registry: &Registry,
    content: &str,
    base_offset: usize,
    valid_tech_names: &HashSet<&str>,
    is_closing: bool,
) -> Vec<(usize, usize, u32, u32)> {
    let mut tokens = Vec::new();
    let offset = base_offset;

    // Handle universal closer {{//}}
    if content.is_empty() && is_closing {
        // Universal closer - just highlight as keyword
        tokens.push((offset - 1, 1, TOKEN_KEYWORD, 0)); // The "/" in {{//}}
        return tokens;
    }

    let mut offset = offset;

    // Handle ui:tech: prefix
    if let Some(rest) = content.strip_prefix("ui:tech:") {
        // "ui:tech" as namespace
        tokens.push((offset, 7, TOKEN_NAMESPACE, 0));
        offset += 8; // "ui:tech:"

        // Parse tech name and parameters
        let parts: Vec<&str> = rest.split(':').collect();
        if !parts.is_empty() {
            let tech_name = parts[0];
            let token_type = if valid_tech_names.contains(tech_name) {
                TOKEN_TYPE
            } else {
                TOKEN_INVALID
            };
            tokens.push((offset, tech_name.len(), token_type, 0));
            offset += tech_name.len() + 1; // +1 for ':'

            // Parse parameters
            for part in &parts[1..] {
                if let Some(eq_pos) = part.find('=') {
                    let param_name = &part[..eq_pos];
                    let param_value = &part[eq_pos + 1..];

                    // Parameter name
                    let param_type = if params::is_valid_tech_param(param_name) {
                        TOKEN_PARAMETER
                    } else {
                        TOKEN_INVALID
                    };
                    tokens.push((offset, param_name.len(), param_type, 0));
                    offset += eq_pos + 1; // param_name + '='

                    // Parameter value
                    let value_type = if is_color_param(param_name)
                        && registry.palette().contains_key(param_value)
                    {
                        TOKEN_VARIABLE
                    } else {
                        TOKEN_STRING
                    };
                    tokens.push((offset, param_value.len(), value_type, 0));
                    offset += param_value.len() + 1;
                } else {
                    // Part without = (could be trailing part)
                    offset += part.len() + 1;
                }
            }
        }
    }
    // Handle ui:live: prefix
    else if let Some(rest) = content.strip_prefix("ui:live:") {
        tokens.push((offset, 7, TOKEN_NAMESPACE, 0)); // "ui:live"
        offset += 8;

        let parts: Vec<&str> = rest.split(':').collect();
        if !parts.is_empty() {
            // Source name
            let source = parts[0];
            let valid_sources: Vec<&str> = params::valid_live_sources().collect();
            let token_type = if valid_sources.contains(&source) {
                TOKEN_TYPE
            } else {
                TOKEN_INVALID
            };
            tokens.push((offset, source.len(), token_type, 0));
            offset += source.len() + 1;

            // Query (second part)
            if parts.len() > 1 {
                let query = parts[1];
                tokens.push((offset, query.len(), TOKEN_STRING, 0));
                offset += query.len() + 1;
            }

            // Metric (third part)
            if parts.len() > 2 {
                let metric = parts[2];
                let metric_type = if params::is_valid_metric(source, metric) {
                    TOKEN_PARAMETER
                } else {
                    TOKEN_INVALID
                };
                tokens.push((offset, metric.len(), metric_type, 0));
            }
        }
    }
    // Handle ui:progress:, ui:donut:, ui:gauge: prefixes
    else if let Some(rest) = content.strip_prefix("ui:progress:") {
        tokens.push((offset, 11, TOKEN_NAMESPACE, 0)); // "ui:progress"
        offset += 12;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    } else if let Some(rest) = content.strip_prefix("ui:donut:") {
        tokens.push((offset, 8, TOKEN_NAMESPACE, 0)); // "ui:donut"
        offset += 9;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    } else if let Some(rest) = content.strip_prefix("ui:gauge:") {
        tokens.push((offset, 8, TOKEN_NAMESPACE, 0)); // "ui:gauge"
        offset += 9;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    }
    // Handle ui:version: prefix
    else if let Some(rest) = content.strip_prefix("ui:version:") {
        tokens.push((offset, 10, TOKEN_NAMESPACE, 0)); // "ui:version"
        offset += 11;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    }
    // Handle ui:license: prefix
    else if let Some(rest) = content.strip_prefix("ui:license:") {
        tokens.push((offset, 10, TOKEN_NAMESPACE, 0)); // "ui:license"
        offset += 11;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    }
    // Handle ui:row block component
    else if let Some(rest) = content.strip_prefix("ui:row") {
        tokens.push((offset, 6, TOKEN_NAMESPACE, 0)); // "ui:row"
        offset += 6;
        // Handle optional params after :
        if let Some(params) = rest.strip_prefix(':') {
            offset += 1;
            tokenize_ui_component_args(registry, params, offset, &mut tokens);
        }
    }
    // Handle ui:tech-group block component
    else if let Some(rest) = content.strip_prefix("ui:tech-group") {
        tokens.push((offset, 13, TOKEN_NAMESPACE, 0)); // "ui:tech-group"
        offset += 13;
        // Handle optional params after :
        if let Some(params) = rest.strip_prefix(':') {
            offset += 1;
            tokenize_ui_component_args(registry, params, offset, &mut tokens);
        }
    }
    // Handle ui:sparkline: prefix
    else if let Some(rest) = content.strip_prefix("ui:sparkline:") {
        tokens.push((offset, 12, TOKEN_NAMESPACE, 0)); // "ui:sparkline"
        offset += 13;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    }
    // Handle ui:rating: prefix
    else if let Some(rest) = content.strip_prefix("ui:rating:") {
        tokens.push((offset, 9, TOKEN_NAMESPACE, 0)); // "ui:rating"
        offset += 10;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    }
    // Handle ui:waveform: prefix
    else if let Some(rest) = content.strip_prefix("ui:waveform:") {
        tokens.push((offset, 11, TOKEN_NAMESPACE, 0)); // "ui:waveform"
        offset += 12;
        tokenize_ui_component_args(registry, rest, offset, &mut tokens);
    }
    // Handle glyph: prefix
    else if let Some(glyph_name) = content.strip_prefix("glyph:") {
        tokens.push((offset, 5, TOKEN_NAMESPACE, 0)); // "glyph"
        offset += 6;

        let token_type = if registry.glyph(glyph_name).is_some() {
            TOKEN_STRING
        } else {
            TOKEN_INVALID
        };
        tokens.push((offset, glyph_name.len(), token_type, 0));
    }
    // Handle frame: prefix (opening and closing)
    else if let Some(frame_name) = content.strip_prefix("frame:") {
        tokens.push((offset, 5, TOKEN_NAMESPACE, 0)); // "frame"
        offset += 6;

        let name = frame_name.split(':').next().unwrap_or(frame_name);
        let token_type = if registry.frame(name).is_some() {
            TOKEN_FUNCTION
        } else {
            TOKEN_INVALID
        };
        tokens.push((offset, name.len(), token_type, 0));
    }
    // Handle swatch: prefix
    else if let Some(rest) = content.strip_prefix("swatch:") {
        tokens.push((offset, 6, TOKEN_NAMESPACE, 0)); // "swatch"
        offset += 7;

        let color = rest.split(':').next().unwrap_or(rest);
        let token_type = if registry.palette().contains_key(color) {
            TOKEN_VARIABLE
        } else {
            TOKEN_INVALID
        };
        tokens.push((offset, color.len(), token_type, 0));
    }
    // Handle style/component names (both opening and closing tags)
    else {
        let name = content.split([':', '/']).next().unwrap_or(content);
        if registry.style(name).is_some() {
            tokens.push((offset, name.len(), TOKEN_KEYWORD, 0));
        } else if registry.component(name).is_some() {
            tokens.push((offset, name.len(), TOKEN_NAMESPACE, 0));
            // Tokenize component arguments if not a closing tag
            if !is_closing && content.len() > name.len() && content.chars().nth(name.len()) == Some(':')
            {
                let args_str = &content[name.len() + 1..];
                tokenize_component_args(registry, args_str, offset + name.len() + 1, &mut tokens);
            }
        }
    }

    tokens
}

/// Tokenize UI component arguments (progress, donut, gauge)
fn tokenize_ui_component_args(
    registry: &Registry,
    args: &str,
    mut offset: usize,
    tokens: &mut Vec<(usize, usize, u32, u32)>,
) {
    for part in args.split(':') {
        if part.is_empty() {
            offset += 1;
            continue;
        }

        if let Some(eq_pos) = part.find('=') {
            let param_name = &part[..eq_pos];
            let param_value = &part[eq_pos + 1..];

            // Parameter name
            tokens.push((offset, param_name.len(), TOKEN_PARAMETER, 0));
            offset += eq_pos + 1;

            // Parameter value - check if it's a color
            let value_type = if is_color_param(param_name)
                && registry.palette().contains_key(param_value)
            {
                TOKEN_VARIABLE
            } else {
                TOKEN_STRING
            };
            tokens.push((offset, param_value.len(), value_type, 0));
            offset += param_value.len() + 1;
        } else {
            // Positional argument (number or string)
            tokens.push((offset, part.len(), TOKEN_STRING, 0));
            offset += part.len() + 1;
        }
    }
}

/// Tokenize component arguments (like progress:50:100)
fn tokenize_component_args(
    registry: &Registry,
    args: &str,
    mut offset: usize,
    tokens: &mut Vec<(usize, usize, u32, u32)>,
) {
    for part in args.split(':') {
        if part.is_empty() {
            offset += 1;
            continue;
        }

        // Check if it's a palette color
        let token_type = if registry.palette().contains_key(part) {
            TOKEN_VARIABLE
        } else {
            TOKEN_STRING
        };
        tokens.push((offset, part.len(), token_type, 0));
        offset += part.len() + 1;
    }
}

/// Check if a parameter expects a color value
pub fn is_color_param(param: &str) -> bool {
    matches!(
        param,
        "bg" | "bg_left" | "bg_right" | "logo" | "text" | "text_color" | "color" | "border"
    )
}
