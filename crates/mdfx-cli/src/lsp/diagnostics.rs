//! Diagnostic generation for the mdfx LSP
//!
//! Provides validation and error reporting for mdfx templates.

use crate::lsp::parser::{extract_tag_name, find_templates, is_inherently_self_closing};
use mdfx::components::params;
use mdfx::Registry;
use mdfx_icons::list_icons;
use std::collections::HashSet;
use tower_lsp::lsp_types::*;

/// Generate diagnostics for template syntax errors
pub fn generate_diagnostics(registry: &Registry, text: &str, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // Collect valid tech names for diagnostics
    let icon_list = list_icons();
    let valid_tech_names: HashSet<&str> = icon_list.iter().map(|s| s.as_ref()).collect();

    // Track open tags for matching: (tag_name, line, start_col, end_col)
    let mut tag_stack: Vec<(String, u32, u32, u32)> = Vec::new();

    for (line_num, line) in text.lines().enumerate() {
        for (start, is_closing_tag, is_self_closing, is_malformed, content, end) in
            find_templates(line)
        {
            let start_col = start as u32;
            let end_col = end as u32;
            let line_num = line_num as u32;

            // Handle malformed templates (missing `}}`)
            if is_malformed {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_num,
                            character: start_col,
                        },
                        end: Position {
                            line: line_num,
                            character: end_col,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("mdfx".to_string()),
                    message: format!(
                        "Malformed template '{{{{{}' - missing closing '}}}}'",
                        content
                    ),
                    ..Default::default()
                });
                continue;
            }

            // Handle closing tags - check for matching open tag
            if is_closing_tag {
                // Universal closer {{//}} closes any open tag
                if content.is_empty() {
                    if tag_stack.is_empty() {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num,
                                    character: start_col,
                                },
                                end: Position {
                                    line: line_num,
                                    character: end_col,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mdfx".to_string()),
                            message: "Universal closer {{//}} with no open tag to close"
                                .to_string(),
                            ..Default::default()
                        });
                    } else {
                        tag_stack.pop();
                    }
                    continue;
                }

                // Extract tag name for matching
                let close_tag_name = extract_tag_name(content);

                if let Some((open_tag_name, open_line, open_start, open_end)) = tag_stack.pop() {
                    if open_tag_name != close_tag_name {
                        // Mismatched tags
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num,
                                    character: start_col,
                                },
                                end: Position {
                                    line: line_num,
                                    character: end_col,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mdfx".to_string()),
                            message: format!(
                                "Mismatched closing tag '{{{{/{}}}}}, expected '{{{{/{}}}}}'",
                                close_tag_name, open_tag_name
                            ),
                            related_information: Some(vec![DiagnosticRelatedInformation {
                                location: Location {
                                    uri: uri.clone(),
                                    range: Range {
                                        start: Position {
                                            line: open_line,
                                            character: open_start,
                                        },
                                        end: Position {
                                            line: open_line,
                                            character: open_end,
                                        },
                                    },
                                },
                                message: format!("Opening tag '{{{{{}}}}}' is here", open_tag_name),
                            }]),
                            ..Default::default()
                        });
                    }
                } else {
                    // No open tag to close
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("mdfx".to_string()),
                        message: format!(
                            "Closing tag '{{{{/{}}}}}' with no matching open tag",
                            close_tag_name
                        ),
                        ..Default::default()
                    });
                }
                continue;
            }

            // Handle opening tags (non-self-closing)
            // Skip inherently self-closing templates (ui:, glyph:, swatch:)
            if !is_self_closing {
                if is_inherently_self_closing(content) {
                    // Warn that this template should use /}} syntax
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: end_col - 2,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("mdfx".to_string()),
                        message: "This template should be self-closing. Use '/}}' instead of '}}'"
                            .to_string(),
                        ..Default::default()
                    });
                } else {
                    let tag_name = extract_tag_name(content);
                    tag_stack.push((tag_name, line_num, start_col, end_col));
                }
            }

            // Content validation for opening/self-closing tags
            // Check tech badges: {{ui:tech:NAME...}}
            if let Some(rest) = content.strip_prefix("ui:tech:") {
                let tech_name = rest.split(':').next().unwrap_or("");
                if !tech_name.is_empty() && !valid_tech_names.contains(tech_name) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("mdfx".to_string()),
                        message: format!(
                            "Unknown tech badge '{}'. Use autocomplete to see available badges.",
                            tech_name
                        ),
                        ..Default::default()
                    });
                }
            }
            // Check glyphs: {{glyph:NAME/}}
            else if let Some(glyph_name) = content.strip_prefix("glyph:") {
                if !glyph_name.is_empty() && registry.glyph(glyph_name).is_none() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::WARNING),
                        source: Some("mdfx".to_string()),
                        message: format!(
                            "Unknown glyph '{}'. Use autocomplete to see available glyphs.",
                            glyph_name
                        ),
                        ..Default::default()
                    });
                }
            }
            // Check live badges: {{ui:live:SOURCE:QUERY:METRIC/}}
            else if let Some(rest) = content.strip_prefix("ui:live:") {
                let parts: Vec<&str> = rest.split(':').collect();
                let valid_sources: Vec<&str> = params::valid_live_sources().collect();

                if parts.is_empty() || parts[0].is_empty() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line_num,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num,
                                character: end_col,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("mdfx".to_string()),
                        message:
                            "Incomplete live badge syntax. Expected: {{ui:live:source:query:metric/}}"
                                .to_string(),
                        ..Default::default()
                    });
                } else {
                    let source = parts[0];

                    if !valid_sources.contains(&source) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num,
                                    character: start_col,
                                },
                                end: Position {
                                    line: line_num,
                                    character: end_col,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mdfx".to_string()),
                            message: format!(
                                "Unknown live source '{}'. Valid sources: {}",
                                source,
                                valid_sources.join(", ")
                            ),
                            ..Default::default()
                        });
                    } else if parts.len() > 2 {
                        let metric = parts[2];
                        if !params::is_valid_metric(source, metric) {
                            let valid_metrics: Vec<&str> = params::metrics_for_source(source)
                                .map(|m| m.iter().map(|(name, _)| *name).collect())
                                .unwrap_or_default();
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: line_num,
                                        character: start_col,
                                    },
                                    end: Position {
                                        line: line_num,
                                        character: end_col,
                                    },
                                },
                                severity: Some(DiagnosticSeverity::WARNING),
                                source: Some("mdfx".to_string()),
                                message: format!(
                                    "Unknown metric '{}' for source '{}'. Valid metrics: {}",
                                    metric,
                                    source,
                                    valid_metrics.join(", ")
                                ),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
    }

    // Report any unclosed tags
    for (tag_name, line_num, start_col, end_col) in tag_stack {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: line_num,
                    character: start_col,
                },
                end: Position {
                    line: line_num,
                    character: end_col,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("mdfx".to_string()),
            message: format!(
                "Unclosed tag '{{{{{}}}}}' - missing '{{{{/{}}}}}' or '{{{{//}}}}'",
                tag_name, tag_name
            ),
            ..Default::default()
        });
    }

    diagnostics
}
