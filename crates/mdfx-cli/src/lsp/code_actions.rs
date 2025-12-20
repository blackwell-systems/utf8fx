//! Code actions and quick fixes for the mdfx LSP
//!
//! Provides quick fixes for common issues like typos and missing syntax.

use mdfx::Registry;
use mdfx_icons::list_icons;
use tower_lsp::lsp_types::*;

/// Generate code actions for diagnostics
pub fn generate_code_actions(
    registry: &Registry,
    text: &str,
    uri: &Url,
    diagnostics: &[Diagnostic],
) -> Vec<CodeActionOrCommand> {
    let mut actions = Vec::new();

    // Process diagnostics to generate quick fixes
    for diagnostic in diagnostics {
        // Quick fix: Add /}} for self-closing templates
        if diagnostic.message.contains("should be self-closing") {
            let fix_range = diagnostic.range;
            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                title: "Add self-closing syntax '/}}'".to_string(),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![diagnostic.clone()]),
                edit: Some(WorkspaceEdit {
                    changes: Some(std::collections::HashMap::from([(
                        uri.clone(),
                        vec![TextEdit {
                            range: Range {
                                start: Position {
                                    line: fix_range.end.line,
                                    character: fix_range.end.character - 2,
                                },
                                end: fix_range.end,
                            },
                            new_text: "/}}".to_string(),
                        }],
                    )])),
                    ..Default::default()
                }),
                is_preferred: Some(true),
                ..Default::default()
            }));
        }

        // Quick fix: Suggest similar tech badge names
        if diagnostic.message.contains("Unknown tech badge") {
            if let Some(tech_name) = extract_quoted_value(&diagnostic.message) {
                let suggestions = find_similar_tech_names(&tech_name);
                for suggestion in suggestions.into_iter().take(3) {
                    let line = diagnostic.range.start.line as usize;
                    if let Some(line_text) = text.lines().nth(line) {
                        if let Some(edit_range) = find_tech_name_range(
                            line_text,
                            &tech_name,
                            diagnostic.range.start.character as usize,
                        ) {
                            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                                title: format!("Did you mean '{}'?", suggestion),
                                kind: Some(CodeActionKind::QUICKFIX),
                                diagnostics: Some(vec![diagnostic.clone()]),
                                edit: Some(WorkspaceEdit {
                                    changes: Some(std::collections::HashMap::from([(
                                        uri.clone(),
                                        vec![TextEdit {
                                            range: Range {
                                                start: Position {
                                                    line: diagnostic.range.start.line,
                                                    character: edit_range.0 as u32,
                                                },
                                                end: Position {
                                                    line: diagnostic.range.start.line,
                                                    character: edit_range.1 as u32,
                                                },
                                            },
                                            new_text: suggestion.clone(),
                                        }],
                                    )])),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }));
                        }
                    }
                }
            }
        }

        // Quick fix: Suggest similar glyph names
        if diagnostic.message.contains("Unknown glyph") {
            if let Some(glyph_name) = extract_quoted_value(&diagnostic.message) {
                let suggestions = find_similar_glyph_names(registry, &glyph_name);
                for suggestion in suggestions.into_iter().take(3) {
                    let line = diagnostic.range.start.line as usize;
                    if let Some(line_text) = text.lines().nth(line) {
                        if let Some(edit_range) = find_glyph_name_range(
                            line_text,
                            &glyph_name,
                            diagnostic.range.start.character as usize,
                        ) {
                            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                                title: format!("Did you mean '{}'?", suggestion),
                                kind: Some(CodeActionKind::QUICKFIX),
                                diagnostics: Some(vec![diagnostic.clone()]),
                                edit: Some(WorkspaceEdit {
                                    changes: Some(std::collections::HashMap::from([(
                                        uri.clone(),
                                        vec![TextEdit {
                                            range: Range {
                                                start: Position {
                                                    line: diagnostic.range.start.line,
                                                    character: edit_range.0 as u32,
                                                },
                                                end: Position {
                                                    line: diagnostic.range.start.line,
                                                    character: edit_range.1 as u32,
                                                },
                                            },
                                            new_text: suggestion.clone(),
                                        }],
                                    )])),
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }));
                        }
                    }
                }
            }
        }
    }

    actions
}

/// Extract a single-quoted value from a diagnostic message
/// e.g., "Unknown tech badge 'rustlang'" -> "rustlang"
fn extract_quoted_value(message: &str) -> Option<String> {
    let start = message.find('\'')?;
    let rest = &message[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
}

/// Find tech badge names similar to the given name using edit distance
fn find_similar_tech_names(name: &str) -> Vec<String> {
    let icons = list_icons();
    // Use bounded search with max_distance for early termination
    let max_distance = 3.max(name.len() / 2);
    let mut scored: Vec<(String, usize)> = icons
        .iter()
        .filter_map(|icon| {
            let dist = levenshtein_bounded(name, icon, max_distance);
            if dist != usize::MAX {
                Some((icon.to_string(), dist))
            } else {
                None
            }
        })
        .collect();

    scored.sort_by_key(|(_, dist)| *dist);
    scored.into_iter().map(|(name, _)| name).collect()
}

/// Find glyph names similar to the given name using edit distance
fn find_similar_glyph_names(registry: &Registry, name: &str) -> Vec<String> {
    let glyphs = registry.glyphs();
    // Use bounded search with max_distance for early termination
    let max_distance = 3.max(name.len() / 2);
    let mut scored: Vec<(String, usize)> = glyphs
        .keys()
        .filter_map(|glyph| {
            let dist = levenshtein_bounded(name, glyph, max_distance);
            if dist != usize::MAX {
                Some((glyph.clone(), dist))
            } else {
                None
            }
        })
        .collect();

    scored.sort_by_key(|(_, dist)| *dist);
    scored.into_iter().map(|(name, _)| name).collect()
}

/// Optimized Levenshtein distance with early termination and O(n) space
/// Returns usize::MAX if distance exceeds max_distance (for early termination)
pub fn levenshtein_bounded(a: &str, b: &str, max_distance: usize) -> usize {
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    let a_chars: Vec<char> = a_lower.chars().collect();
    let b_chars: Vec<char> = b_lower.chars().collect();

    let m = a_chars.len();
    let n = b_chars.len();

    // Early termination: if length difference exceeds max_distance, no point computing
    if m.abs_diff(n) > max_distance {
        return usize::MAX;
    }

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    // Use two-row optimization: O(n) space instead of O(m*n)
    let mut prev_row: Vec<usize> = (0..=n).collect();
    let mut curr_row = vec![0; n + 1];

    for i in 1..=m {
        curr_row[0] = i;
        let mut row_min = i; // Track minimum in current row for early termination

        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);

            row_min = row_min.min(curr_row[j]);
        }

        // Early termination: if entire row exceeds max_distance, abort
        if row_min > max_distance {
            return usize::MAX;
        }

        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[n]
}

/// Find the range of a tech name within a line
/// Returns (start, end) character positions
fn find_tech_name_range(line: &str, _tech_name: &str, start_hint: usize) -> Option<(usize, usize)> {
    // Look for {{ui:tech:NAME pattern
    let search_start = if start_hint > 10 {
        start_hint - 10
    } else {
        0
    };
    let prefix = "ui:tech:";
    if let Some(prefix_pos) = line[search_start..].find(prefix) {
        let name_start = search_start + prefix_pos + prefix.len();
        // Find end of tech name (next : or / or })
        let rest = &line[name_start..];
        let name_end = rest
            .find(|c| c == ':' || c == '/' || c == '}')
            .unwrap_or(rest.len());
        return Some((name_start, name_start + name_end));
    }
    None
}

/// Find the range of a glyph name within a line
fn find_glyph_name_range(
    line: &str,
    _glyph_name: &str,
    start_hint: usize,
) -> Option<(usize, usize)> {
    // Look for {{glyph:NAME pattern
    let search_start = if start_hint > 10 {
        start_hint - 10
    } else {
        0
    };
    let prefix = "glyph:";
    if let Some(prefix_pos) = line[search_start..].find(prefix) {
        let name_start = search_start + prefix_pos + prefix.len();
        // Find end of glyph name (next / or })
        let rest = &line[name_start..];
        let name_end = rest
            .find(|c| c == '/' || c == '}')
            .unwrap_or(rest.len());
        return Some((name_start, name_start + name_end));
    }
    None
}
