//! LSP Server for mdfx template syntax
//!
//! Provides language server protocol support for mdfx template syntax,
//! including autocompletion for glyphs, styles, frames, components, and tech badges.
//!
//! Enable with: `cargo install mdfx-cli --features lsp`

use mdfx::components::params::{self, LIVE_SOURCES, TECH_PARAMS};
use mdfx::Registry;
use mdfx_icons::{brand_color, list_icons};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// Cached completion items built at startup for performance
#[allow(dead_code)]
struct CachedCompletions {
    /// All glyph completions
    glyphs: Vec<CompletionItem>,
    /// All style completions (includes aliases) - used via top_level
    styles: Vec<CompletionItem>,
    /// All frame completions (includes aliases)
    frames: Vec<CompletionItem>,
    /// All component completions - used via top_level
    components: Vec<CompletionItem>,
    /// All palette color completions
    palette: Vec<CompletionItem>,
    /// All shield style completions
    shield_styles: Vec<CompletionItem>,
    /// All tech name completions
    tech_names: Vec<CompletionItem>,
    /// All tech parameter completions
    tech_params: Vec<CompletionItem>,
    /// All live source completions
    live_sources: Vec<CompletionItem>,
    /// Top-level completions (glyph:, frame:, styles, components)
    top_level: Vec<CompletionItem>,
}

/// The mdfx language server
pub struct MdfxLanguageServer {
    client: Client,
    registry: Arc<Registry>,
    /// Cached document contents (URI -> text)
    documents: Arc<RwLock<HashMap<String, String>>>,
    /// Pre-built completion items for fast responses
    cached: Arc<CachedCompletions>,
}

impl MdfxLanguageServer {
    pub fn new(client: Client) -> Self {
        let registry = Registry::new().expect("Failed to load registry");

        // Pre-build all completion items at startup for fast responses
        let cached = Self::build_cached_completions(&registry);

        Self {
            client,
            registry: Arc::new(registry),
            documents: Arc::new(RwLock::new(HashMap::new())),
            cached: Arc::new(cached),
        }
    }

    /// Build all completion items once at startup
    fn build_cached_completions(registry: &Registry) -> CachedCompletions {
        // Build glyph completions
        let glyphs: Vec<CompletionItem> = registry
            .glyphs()
            .iter()
            .map(|(name, char)| CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some(format!("Glyph: {}", char)),
                documentation: Some(Documentation::String(format!(
                    "Renders as: {} (U+{:04X})",
                    char,
                    char.chars().next().unwrap_or(' ') as u32
                ))),
                insert_text: Some(format!("{}/ }}}}", name)),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect();

        // Build style completions (includes aliases)
        let styles: Vec<CompletionItem> = registry
            .styles()
            .iter()
            .flat_map(|(name, style)| {
                let mut items = vec![CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(style.name.clone()),
                    documentation: style.description.clone().map(Documentation::String),
                    insert_text: Some(format!("{}}}${{1:text}}{{{{/{}}}}}", name, name)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                }];
                for alias in &style.aliases {
                    items.push(CompletionItem {
                        label: alias.clone(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        detail: Some(format!("{} (alias for {})", style.name, name)),
                        documentation: style.description.clone().map(Documentation::String),
                        insert_text: Some(format!("{}}}${{1:text}}{{{{/{}}}}}", alias, alias)),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    });
                }
                items
            })
            .collect();

        // Build frame completions (includes aliases)
        let frames: Vec<CompletionItem> = registry
            .frames()
            .iter()
            .flat_map(|(name, frame)| {
                let mut items = vec![CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::STRUCT),
                    detail: Some(format!(
                        "{} ... {}",
                        frame.prefix.trim(),
                        frame.suffix.trim()
                    )),
                    documentation: frame.description.clone().map(Documentation::String),
                    insert_text: Some(format!("{}}}${{1:text}}{{{{/{}}}}}", name, name)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                }];
                for alias in &frame.aliases {
                    items.push(CompletionItem {
                        label: alias.clone(),
                        kind: Some(CompletionItemKind::STRUCT),
                        detail: Some(format!(
                            "{} ... {} (alias for {})",
                            frame.prefix.trim(),
                            frame.suffix.trim(),
                            name
                        )),
                        documentation: frame.description.clone().map(Documentation::String),
                        insert_text: Some(format!("{}}}${{1:text}}{{{{/{}}}}}", alias, alias)),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        ..Default::default()
                    });
                }
                items
            })
            .collect();

        // Build component completions
        let components: Vec<CompletionItem> = registry
            .components()
            .iter()
            .map(|(name, component)| {
                let insert_text = if component.self_closing {
                    if component.args.is_empty() {
                        format!("{}/}}}}", name)
                    } else {
                        let args: String = component
                            .args
                            .iter()
                            .enumerate()
                            .map(|(i, arg)| format!("${{{}:{}}}", i + 1, arg))
                            .collect::<Vec<_>>()
                            .join(":");
                        format!("{}:{}/}}}}", name, args)
                    }
                } else {
                    format!("{}}}${{1:content}}{{{{/{}}}}}", name, name)
                };

                CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: component.description.clone(),
                    documentation: Some(Documentation::String(format!(
                        "Type: {}\nSelf-closing: {}\nArgs: {}",
                        component.component_type,
                        component.self_closing,
                        if component.args.is_empty() {
                            "none".to_string()
                        } else {
                            component.args.join(", ")
                        }
                    ))),
                    insert_text: Some(insert_text),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                }
            })
            .collect();

        // Build palette color completions
        let palette: Vec<CompletionItem> = registry
            .palette()
            .iter()
            .map(|(name, hex)| CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::COLOR),
                detail: Some(format!("#{}", hex)),
                documentation: Some(Documentation::String(format!("Hex color: #{}", hex))),
                insert_text: Some(name.clone()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect();

        // Build shield style completions
        let shield_styles: Vec<CompletionItem> = registry
            .shield_styles()
            .iter()
            .flat_map(|(name, style)| {
                let mut items = vec![CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some(style.name.clone()),
                    documentation: style.description.clone().map(Documentation::String),
                    insert_text: Some(name.clone()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    ..Default::default()
                }];
                for alias in &style.aliases {
                    items.push(CompletionItem {
                        label: alias.clone(),
                        kind: Some(CompletionItemKind::ENUM_MEMBER),
                        detail: Some(format!("{} (alias for {})", style.name, name)),
                        documentation: style.description.clone().map(Documentation::String),
                        insert_text: Some(alias.clone()),
                        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                        ..Default::default()
                    });
                }
                items
            })
            .collect();

        // Build tech name completions
        let tech_names: Vec<CompletionItem> = list_icons()
            .iter()
            .map(|name| {
                let color = brand_color(name).unwrap_or("unknown");
                CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::VALUE),
                    detail: Some(format!("Tech badge: #{}", color)),
                    documentation: Some(Documentation::String(format!(
                        "Technology: {}\nBrand color: #{}\n\nUsage: {{{{ui:tech:{}/}}}}",
                        name, color, name
                    ))),
                    insert_text: Some(format!("{}:", name)),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    ..Default::default()
                }
            })
            .collect();

        // Build tech parameter completions
        let tech_params = Self::build_tech_param_completions();

        // Build live source completions
        let live_sources = Self::build_live_source_completions();

        // Build top-level completions
        let mut top_level = Vec::new();

        // Add "glyph:" prefix
        top_level.push(CompletionItem {
            label: "glyph:".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Insert a glyph character".to_string()),
            documentation: Some(Documentation::String(
                "Access 389 Unicode glyphs by name.\nExamples: {{glyph:dot/}}, {{glyph:block.full/}}, {{glyph:star.filled/}}".to_string()
            )),
            insert_text: Some("glyph:".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });

        // Add "frame:" prefix
        top_level.push(CompletionItem {
            label: "frame:".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Wrap text with decorative frame".to_string()),
            documentation: Some(Documentation::String(
                "Apply decorative prefix/suffix to text.\nExample: {{frame:gradient}}text{{/frame:gradient}}".to_string()
            )),
            insert_text: Some("frame:".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });

        // Add styles and components to top-level
        top_level.extend(styles.clone());
        top_level.extend(components.clone());

        CachedCompletions {
            glyphs,
            styles,
            frames,
            components,
            palette,
            shield_styles,
            tech_names,
            tech_params,
            live_sources,
            top_level,
        }
    }

    /// Build tech parameter completion items from shared definitions
    fn build_tech_param_completions() -> Vec<CompletionItem> {
        TECH_PARAMS
            .iter()
            .map(|param| CompletionItem {
                label: param.name.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(param.description.to_string()),
                documentation: Some(Documentation::String(format!(
                    "{}\n\nExample: {}",
                    param.description, param.example
                ))),
                insert_text: Some(format!("{}=", param.name)),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect()
    }

    /// Build live source completion items from shared definitions
    fn build_live_source_completions() -> Vec<CompletionItem> {
        LIVE_SOURCES
            .iter()
            .map(|(name, desc, metrics)| {
                let metrics_list: Vec<&str> = metrics.iter().map(|(m, _)| *m).collect();
                CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: Some(desc.to_string()),
                    documentation: Some(Documentation::String(format!(
                        "{}\n\nAvailable metrics: {}\n\nExample: {{{{ui:live:{}:query:metric/}}}}",
                        desc,
                        metrics_list.join(", "),
                        name
                    ))),
                    insert_text: Some(format!("{}:", name)),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    ..Default::default()
                }
            })
            .collect()
    }

    /// Get document content from cache or try to read from disk
    fn get_document_content(&self, uri: &Url) -> Option<String> {
        // First check the cache
        if let Ok(docs) = self.documents.read() {
            if let Some(content) = docs.get(uri.as_str()) {
                return Some(content.clone());
            }
        }

        // Fallback: try to read from disk (handles file:// URIs)
        if uri.scheme() == "file" {
            if let Ok(path) = uri.to_file_path() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    return Some(content);
                }
            }
        }

        None
    }

    /// Filter cached completions by prefix
    fn filter_completions(items: &[CompletionItem], prefix: &str) -> Vec<CompletionItem> {
        if prefix.is_empty() {
            items.to_vec()
        } else {
            items
                .iter()
                .filter(|item| item.label.starts_with(prefix))
                .cloned()
                .collect()
        }
    }

    /// Tokenize document for semantic highlighting
    /// Returns delta-encoded semantic token data
    fn tokenize_document(&self, text: &str) -> Vec<SemanticToken> {
        let icon_list = list_icons();
        let valid_tech_names: std::collections::HashSet<&str> =
            icon_list.iter().map(|s| s.as_ref()).collect();

        let mut tokens = Vec::new();
        let mut prev_line = 0u32;
        let mut prev_char = 0u32;

        for (line_num, line) in text.lines().enumerate() {
            let line_num = line_num as u32;

            // Find all templates in this line using simple string scanning
            for (start, is_closing, content, _end) in Self::find_templates(line) {
                let template_start = start + 2 + if is_closing { 1 } else { 0 };

                let new_tokens =
                    self.tokenize_template(content, template_start, &valid_tech_names, is_closing);

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

    /// Find all mdfx templates in a line without regex
    /// Returns: Vec<(start_pos, is_closing, content, end_pos)>
    fn find_templates(line: &str) -> Vec<(usize, bool, &str, usize)> {
        let mut results = Vec::new();
        let mut pos = 0;
        let bytes = line.as_bytes();
        let len = bytes.len();

        while pos + 3 < len {
            // Look for {{
            if bytes[pos] == b'{' && bytes[pos + 1] == b'{' {
                let start = pos;
                pos += 2;

                // Check for closing tag marker /
                let is_closing = pos < len && bytes[pos] == b'/';
                if is_closing {
                    pos += 1;
                }

                let content_start = pos;

                // Find the end: either /}} or }}
                while pos < len {
                    if bytes[pos] == b'}' && pos + 1 < len && bytes[pos + 1] == b'}' {
                        // Found }}
                        let content = &line[content_start..pos];
                        results.push((start, is_closing, content, pos + 2));
                        pos += 2;
                        break;
                    } else if bytes[pos] == b'/' && pos + 2 < len && bytes[pos + 1] == b'}' && bytes[pos + 2] == b'}' {
                        // Found /}}
                        let content = &line[content_start..pos];
                        results.push((start, is_closing, content, pos + 3));
                        pos += 3;
                        break;
                    }
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }

        results
    }

    /// Tokenize a single template's content
    /// Returns: Vec<(offset, length, token_type, token_modifiers)>
    fn tokenize_template(
        &self,
        content: &str,
        base_offset: usize,
        valid_tech_names: &std::collections::HashSet<&str>,
        is_closing: bool,
    ) -> Vec<(usize, usize, u32, u32)> {
        // Token type indices (must match the legend in initialize)
        const TOKEN_NAMESPACE: u32 = 0; // component prefix
        const TOKEN_TYPE: u32 = 1; // tech name
        const TOKEN_PARAMETER: u32 = 2; // parameter name
        const TOKEN_STRING: u32 = 3; // parameter value
        const TOKEN_VARIABLE: u32 = 4; // palette color name
        const TOKEN_KEYWORD: u32 = 5; // style name
        const TOKEN_FUNCTION: u32 = 6; // frame name
        const TOKEN_INVALID: u32 = 7; // invalid items

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
                        let value_type = if self.is_color_param(param_name)
                            && self.registry.palette().contains_key(param_value)
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
            self.tokenize_ui_component_args(rest, offset, &mut tokens);
        } else if let Some(rest) = content.strip_prefix("ui:donut:") {
            tokens.push((offset, 8, TOKEN_NAMESPACE, 0)); // "ui:donut"
            offset += 9;
            self.tokenize_ui_component_args(rest, offset, &mut tokens);
        } else if let Some(rest) = content.strip_prefix("ui:gauge:") {
            tokens.push((offset, 8, TOKEN_NAMESPACE, 0)); // "ui:gauge"
            offset += 9;
            self.tokenize_ui_component_args(rest, offset, &mut tokens);
        }
        // Handle glyph: prefix
        else if let Some(glyph_name) = content.strip_prefix("glyph:") {
            tokens.push((offset, 5, TOKEN_NAMESPACE, 0)); // "glyph"
            offset += 6;

            let token_type = if self.registry.glyph(glyph_name).is_some() {
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
            let token_type = if self.registry.frame(name).is_some() {
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
            let token_type = if self.registry.palette().contains_key(color) {
                TOKEN_VARIABLE
            } else {
                TOKEN_INVALID
            };
            tokens.push((offset, color.len(), token_type, 0));
        }
        // Handle style/component names (both opening and closing tags)
        else {
            let name = content.split([':', '/']).next().unwrap_or(content);
            if self.registry.style(name).is_some() {
                tokens.push((offset, name.len(), TOKEN_KEYWORD, 0));
            } else if self.registry.component(name).is_some() {
                tokens.push((offset, name.len(), TOKEN_NAMESPACE, 0));
                // Tokenize component arguments if not a closing tag
                if !is_closing && content.len() > name.len() && content.chars().nth(name.len()) == Some(':') {
                    let args_str = &content[name.len() + 1..];
                    self.tokenize_component_args(args_str, offset + name.len() + 1, &mut tokens);
                }
            }
        }

        tokens
    }

    /// Tokenize UI component arguments (progress, donut, gauge)
    fn tokenize_ui_component_args(
        &self,
        args: &str,
        mut offset: usize,
        tokens: &mut Vec<(usize, usize, u32, u32)>,
    ) {
        const TOKEN_PARAMETER: u32 = 2;
        const TOKEN_STRING: u32 = 3;
        const TOKEN_VARIABLE: u32 = 4;

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
                let value_type = if self.is_color_param(param_name)
                    && self.registry.palette().contains_key(param_value)
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
        &self,
        args: &str,
        mut offset: usize,
        tokens: &mut Vec<(usize, usize, u32, u32)>,
    ) {
        const TOKEN_STRING: u32 = 3;
        const TOKEN_VARIABLE: u32 = 4;

        for part in args.split(':') {
            if part.is_empty() {
                offset += 1;
                continue;
            }

            // Check if it's a palette color
            let token_type = if self.registry.palette().contains_key(part) {
                TOKEN_VARIABLE
            } else {
                TOKEN_STRING
            };
            tokens.push((offset, part.len(), token_type, 0));
            offset += part.len() + 1;
        }
    }

    /// Check if a parameter expects a color value
    fn is_color_param(&self, param: &str) -> bool {
        matches!(
            param,
            "bg" | "bg_left" | "bg_right" | "logo" | "text" | "text_color" | "color" | "border"
        )
    }

    /// Build completion items for live source metrics using shared definitions
    fn live_metric_completions(&self, source: &str, prefix: &str) -> Vec<CompletionItem> {
        let metrics = params::metrics_for_source(source).unwrap_or(&[]);

        metrics
            .iter()
            .filter(|(name, _)| prefix.is_empty() || name.starts_with(prefix))
            .map(|(name, desc)| CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(desc.to_string()),
                insert_text: Some(format!("{}/}}}}", name)),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect()
    }

    /// Build completion items for tech badge parameter values using shared definitions
    fn tech_param_value_completions(&self, param: &str, prefix: &str) -> Vec<CompletionItem> {
        // Color parameters return palette completions from cache
        match param {
            "bg" | "bg_left" | "bg_right" | "logo" | "text" | "text_color" | "color" | "border" => {
                return Self::filter_completions(&self.cached.palette, prefix);
            }
            _ => {}
        }

        // Look up values from shared TECH_PARAMS
        let values = TECH_PARAMS
            .iter()
            .find(|p| p.name == param)
            .and_then(|p| p.values)
            .unwrap_or(&[]);

        values
            .iter()
            .filter(|(val, _)| prefix.is_empty() || val.starts_with(prefix))
            .map(|(val, desc)| CompletionItem {
                label: val.to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                detail: Some(desc.to_string()),
                insert_text: Some(val.to_string()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect()
    }

    /// Generate diagnostics for template syntax errors
    fn generate_diagnostics(&self, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Collect valid tech names for diagnostics
        let icon_list = list_icons();
        let valid_tech_names: std::collections::HashSet<&str> =
            icon_list.iter().map(|s| s.as_ref()).collect();

        for (line_num, line) in text.lines().enumerate() {
            for (start, is_closing, content, end) in Self::find_templates(line) {
                // Skip closing tags for validation
                if is_closing {
                    continue;
                }

                let start_col = start as u32;
                let end_col = end as u32;

                // Check tech badges: {{ui:tech:NAME...}}
                if let Some(rest) = content.strip_prefix("ui:tech:") {
                    let tech_name = rest.split(':').next().unwrap_or("");
                    if !tech_name.is_empty() && !valid_tech_names.contains(tech_name) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_num as u32, character: start_col },
                                end: Position { line: line_num as u32, character: end_col },
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
                    if !glyph_name.is_empty() && self.registry.glyph(glyph_name).is_none() {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_num as u32, character: start_col },
                                end: Position { line: line_num as u32, character: end_col },
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
                        // Incomplete - no source
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_num as u32, character: start_col },
                                end: Position { line: line_num as u32, character: end_col },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mdfx".to_string()),
                            message: "Incomplete live badge syntax. Expected: {{ui:live:source:query:metric/}}".to_string(),
                            ..Default::default()
                        });
                    } else {
                        let source = parts[0];

                        // Check if source is valid
                        if !valid_sources.contains(&source) {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position { line: line_num as u32, character: start_col },
                                    end: Position { line: line_num as u32, character: end_col },
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
                            // Check metric validity
                            let metric = parts[2];
                            if !params::is_valid_metric(source, metric) {
                                let valid_metrics: Vec<&str> = params::metrics_for_source(source)
                                    .map(|m| m.iter().map(|(name, _)| *name).collect())
                                    .unwrap_or_default();
                                diagnostics.push(Diagnostic {
                                    range: Range {
                                        start: Position { line: line_num as u32, character: start_col },
                                        end: Position { line: line_num as u32, character: end_col },
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

        diagnostics
    }

    /// Analyze the text around the cursor to determine completion context
    fn get_completion_context(&self, text: &str, position: Position) -> CompletionContext {
        let lines: Vec<&str> = text.lines().collect();
        let line_idx = position.line as usize;

        if line_idx >= lines.len() {
            return CompletionContext::None;
        }

        let line = lines[line_idx];
        let col = position.character as usize;
        let prefix = if col <= line.len() {
            &line[..col]
        } else {
            line
        };

        // Look for opening {{ pattern
        if let Some(open_pos) = prefix.rfind("{{") {
            let after_open = &prefix[open_pos + 2..];

            // Check for glyph: prefix
            if let Some(rest) = after_open.strip_prefix("glyph:") {
                return CompletionContext::Glyph(rest.to_string());
            }

            // Check for frame: prefix
            if let Some(rest) = after_open.strip_prefix("frame:") {
                return CompletionContext::Frame(rest.to_string());
            }

            // Check for live data source context: {{ui:live:...
            if let Some(rest) = after_open.strip_prefix("ui:live:") {
                let parts: Vec<&str> = rest.split(':').collect();

                if parts.is_empty() || (parts.len() == 1 && !rest.contains(':')) {
                    // Just after {{ui:live: - show sources
                    return CompletionContext::LiveSource(rest.to_string());
                }

                // We have a source, check if we need metric completion
                let source = parts[0];
                if parts.len() >= 2 {
                    // We have source:query, check for metric
                    if parts.len() == 2 && rest.ends_with(':') {
                        // After {{ui:live:source:query: - show metrics
                        return CompletionContext::LiveMetric(source.to_string(), String::new());
                    }
                    if parts.len() >= 3 {
                        // Completing metric name
                        let metric_prefix = parts.get(2).unwrap_or(&"");
                        return CompletionContext::LiveMetric(
                            source.to_string(),
                            metric_prefix.to_string(),
                        );
                    }
                }

                // Still entering source name
                return CompletionContext::LiveSource(rest.to_string());
            }

            // Check for tech badge context: {{ui:tech:...
            if let Some(rest) = after_open.strip_prefix("ui:tech:") {
                // Parse the rest to determine context
                // Format: NAME:param1=value1:param2=value2
                let parts: Vec<&str> = rest.split(':').collect();

                if parts.is_empty() || (parts.len() == 1 && !rest.contains(':')) {
                    // Just after {{ui:tech: - show tech names
                    return CompletionContext::TechName(rest.to_string());
                }

                // We have at least a tech name
                let _tech_name = parts[0];

                // Check if we're in a parameter value (after =)
                if let Some(eq_pos) = rest.rfind('=') {
                    let after_eq = &rest[eq_pos + 1..];
                    // Find which parameter we're completing a value for
                    let before_eq = &rest[..eq_pos];
                    if let Some(colon_pos) = before_eq.rfind(':') {
                        let param_name = &before_eq[colon_pos + 1..];
                        return CompletionContext::TechParamValue(
                            param_name.to_string(),
                            after_eq.to_string(),
                        );
                    }
                }

                // Check if we're after a colon (ready for param name)
                if rest.ends_with(':') || parts.len() > 1 {
                    // Get the last incomplete part as prefix for param completion
                    let last_part = parts.last().unwrap_or(&"");
                    // If it contains =, we're in a value, otherwise param name
                    if !last_part.contains('=') {
                        return CompletionContext::TechParam(last_part.to_string());
                    }
                }

                return CompletionContext::TechParam(String::new());
            }

            // Check for style= parameter (shield styles like flat, flat-square, for-the-badge)
            if after_open.contains("style=") {
                if let Some(style_pos) = after_open.rfind("style=") {
                    let style_prefix = &after_open[style_pos + 6..];
                    // Don't include any trailing characters after the style value
                    let style_prefix = style_prefix
                        .split([':', '/', '}'])
                        .next()
                        .unwrap_or(style_prefix);
                    return CompletionContext::ShieldStyle(style_prefix.to_string());
                }
            }

            // Check for color parameter (e.g., swatch:cobalt or bg=cobalt)
            if after_open.contains(':')
                && (after_open.contains("bg=") || after_open.contains("fg="))
            {
                // Find the part after the last = sign
                if let Some(eq_pos) = after_open.rfind('=') {
                    let color_prefix = &after_open[eq_pos + 1..];
                    return CompletionContext::Palette(color_prefix.to_string());
                }
            }

            // Check for component with args (e.g., swatch:)
            if after_open.contains(':') {
                let parts: Vec<&str> = after_open.splitn(2, ':').collect();
                if !parts.is_empty() {
                    let comp_name = parts[0];
                    if self.registry.component(comp_name).is_some() {
                        // Inside component args - could be palette for swatch
                        if comp_name == "swatch" {
                            let arg_prefix = parts.get(1).unwrap_or(&"");
                            return CompletionContext::Palette(arg_prefix.to_string());
                        }
                    }
                }
            }

            // Just after {{ - show all top-level completions
            if !after_open.contains(':') && !after_open.contains('/') {
                return CompletionContext::TopLevel(after_open.to_string());
            }
        }

        CompletionContext::None
    }
}

/// Context for completions
enum CompletionContext {
    None,
    TopLevel(String),    // After {{ - show styles, frames, components, glyph:
    Glyph(String),       // After {{glyph: - show glyph names
    Frame(String),       // After {{frame: - show frame names
    Palette(String),     // Inside color parameter - show palette colors
    ShieldStyle(String), // After style= - show shield styles (flat, flat-square, etc.)
    TechName(String),    // After {{ui:tech: - show tech names (rust, typescript, etc.)
    TechParam(String),   // After {{ui:tech:NAME: - show parameter names
    TechParamValue(String, String), // After {{ui:tech:NAME:param= - show values for param
    LiveSource(String),  // After {{ui:live: - show live data sources (github, npm, etc.)
    LiveMetric(String, String), // After {{ui:live:SOURCE:QUERY: - show metrics for source
}

#[tower_lsp::async_trait]
impl LanguageServer for MdfxLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        "{".to_string(),
                        ":".to_string(),
                        "=".to_string(),
                    ]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::NAMESPACE,  // 0: component prefix (ui:tech, glyph)
                                SemanticTokenType::TYPE,       // 1: tech name (rust, typescript)
                                SemanticTokenType::PARAMETER,  // 2: parameter name
                                SemanticTokenType::STRING,     // 3: parameter value
                                SemanticTokenType::VARIABLE,   // 4: palette color name
                                SemanticTokenType::KEYWORD,    // 5: style name
                                SemanticTokenType::FUNCTION,   // 6: frame name
                                SemanticTokenType::new("invalid"), // 7: invalid/unknown items
                            ],
                            token_modifiers: vec![
                                SemanticTokenModifier::DEFINITION,
                                SemanticTokenModifier::new("valid"),
                                SemanticTokenModifier::new("invalid"),
                            ],
                        },
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        range: Some(false),
                        ..Default::default()
                    }),
                ),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "mdfx-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "mdfx LSP server initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;

        // Cache the document content
        if let Ok(mut docs) = self.documents.write() {
            docs.insert(params.text_document.uri.to_string(), text.clone());
        }

        let diagnostics = self.generate_diagnostics(&text);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().last() {
            // Update the cached document content
            if let Ok(mut docs) = self.documents.write() {
                docs.insert(params.text_document.uri.to_string(), change.text.clone());
            }

            let diagnostics = self.generate_diagnostics(&change.text);
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Remove document from cache
        if let Ok(mut docs) = self.documents.write() {
            docs.remove(params.text_document.uri.as_str());
        }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        // Get document content from cache (updated via didOpen/didChange)
        let text = match self.get_document_content(&uri) {
            Some(content) => content,
            None => return Ok(None),
        };

        let context = self.get_completion_context(&text, position);

        // Use cached completions with filtering for fast responses
        let items = match context {
            CompletionContext::None => return Ok(None),
            CompletionContext::TopLevel(prefix) => {
                Self::filter_completions(&self.cached.top_level, &prefix)
            }
            CompletionContext::Glyph(prefix) => {
                Self::filter_completions(&self.cached.glyphs, &prefix)
            }
            CompletionContext::Frame(prefix) => {
                Self::filter_completions(&self.cached.frames, &prefix)
            }
            CompletionContext::Palette(prefix) => {
                Self::filter_completions(&self.cached.palette, &prefix)
            }
            CompletionContext::ShieldStyle(prefix) => {
                Self::filter_completions(&self.cached.shield_styles, &prefix)
            }
            CompletionContext::TechName(prefix) => {
                Self::filter_completions(&self.cached.tech_names, &prefix)
            }
            CompletionContext::TechParam(prefix) => {
                Self::filter_completions(&self.cached.tech_params, &prefix)
            }
            CompletionContext::TechParamValue(param, prefix) => {
                self.tech_param_value_completions(&param, &prefix)
            }
            CompletionContext::LiveSource(prefix) => {
                Self::filter_completions(&self.cached.live_sources, &prefix)
            }
            CompletionContext::LiveMetric(source, prefix) => {
                self.live_metric_completions(&source, &prefix)
            }
        };

        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CompletionResponse::Array(items)))
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        // Get document content from cache
        let text = match self.get_document_content(&uri) {
            Some(content) => content,
            None => return Ok(None),
        };

        let lines: Vec<&str> = text.lines().collect();
        let line_idx = position.line as usize;
        if line_idx >= lines.len() {
            return Ok(None);
        }

        let line = lines[line_idx];
        let col = position.character as usize;

        // Find the template at the cursor position
        // Look for {{ before and }} or / after
        let before = &line[..col.min(line.len())];
        let after = &line[col.min(line.len())..];

        if let Some(open_pos) = before.rfind("{{") {
            let template_start = &before[open_pos + 2..];

            // Check for glyph
            if let Some(glyph_name) = template_start.strip_prefix("glyph:") {
                // Find end of glyph name
                let name = glyph_name.split(['/', '}']).next().unwrap_or(glyph_name);
                let full_name = if let Some(end_pos) = after.find('/') {
                    format!("{}{}", name, &after[..end_pos])
                } else {
                    name.to_string()
                };

                if let Some(char) = self.registry.glyph(&full_name) {
                    return Ok(Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: format!(
                                "**Glyph: {}**\n\nCharacter: `{}`\n\nUnicode: U+{:04X}",
                                full_name,
                                char,
                                char.chars().next().unwrap_or(' ') as u32
                            ),
                        }),
                        range: None,
                    }));
                }
            }

            // Check for style
            let style_name = template_start.split([':', '}']).next().unwrap_or("");
            if let Some(style) = self.registry.style(style_name) {
                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "**Style: {}**\n\n{}\n\nAliases: {}\n\nSupports: uppercase={}, lowercase={}, numbers={}",
                            style.name,
                            style.description.as_deref().unwrap_or(""),
                            if style.aliases.is_empty() { "none".to_string() } else { style.aliases.join(", ") },
                            style.supports.uppercase,
                            style.supports.lowercase,
                            style.supports.numbers,
                        ),
                    }),
                    range: None,
                }));
            }

            // Check for component
            let comp_name = template_start.split([':', '/', '}']).next().unwrap_or("");
            if let Some(component) = self.registry.component(comp_name) {
                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!(
                            "**Component: {}**\n\n{}\n\nType: {}\nSelf-closing: {}\nArgs: {}",
                            comp_name,
                            component.description.as_deref().unwrap_or(""),
                            component.component_type,
                            component.self_closing,
                            if component.args.is_empty() {
                                "none".to_string()
                            } else {
                                component.args.join(", ")
                            }
                        ),
                    }),
                    range: None,
                }));
            }
        }

        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let text = match self.get_document_content(&uri) {
            Some(content) => content,
            None => return Ok(None),
        };

        let mut symbols = Vec::new();

        for (line_num, line) in text.lines().enumerate() {
            for (start, is_closing, template_content, end) in Self::find_templates(line) {
                // Skip closing tags for symbols
                if is_closing {
                    continue;
                }

                let start_col = start as u32;
                let end_col = end as u32;

                // Determine symbol type and name
                let (symbol_kind, name, detail) = if template_content.starts_with("ui:tech:") {
                    let tech_name = template_content
                        .strip_prefix("ui:tech:")
                        .unwrap_or("")
                        .split(':')
                        .next()
                        .unwrap_or("unknown");
                    (SymbolKind::CONSTANT, format!("tech:{}", tech_name), Some("Tech Badge".to_string()))
                } else if template_content.starts_with("ui:live:") {
                    let parts: Vec<&str> = template_content
                        .strip_prefix("ui:live:")
                        .unwrap_or("")
                        .split(':')
                        .collect();
                    let source = parts.first().unwrap_or(&"unknown");
                    (SymbolKind::VARIABLE, format!("live:{}", source), Some("Live Badge".to_string()))
                } else if template_content.starts_with("glyph:") {
                    let glyph_name = template_content
                        .strip_prefix("glyph:")
                        .unwrap_or("unknown");
                    (SymbolKind::STRING, format!("glyph:{}", glyph_name), Some("Glyph".to_string()))
                } else if template_content.starts_with("swatch:") {
                    let color = template_content
                        .strip_prefix("swatch:")
                        .unwrap_or("")
                        .split(':')
                        .next()
                        .unwrap_or("unknown");
                    (SymbolKind::CONSTANT, format!("swatch:{}", color), Some("Color Swatch".to_string()))
                } else {
                    // Check if it's a style
                    let name = template_content.split([':', '}']).next().unwrap_or(template_content);
                    if self.registry.style(name).is_some() {
                        (SymbolKind::FUNCTION, name.to_string(), Some("Style".to_string()))
                    } else if self.registry.component(name).is_some() {
                        (SymbolKind::MODULE, name.to_string(), Some("Component".to_string()))
                    } else {
                        continue; // Skip unknown templates
                    }
                };

                #[allow(deprecated)]
                symbols.push(SymbolInformation {
                    name,
                    kind: symbol_kind,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: start_col,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: end_col,
                            },
                        },
                    },
                    container_name: detail,
                });
            }
        }

        if symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DocumentSymbolResponse::Flat(symbols)))
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let text = match self.get_document_content(&uri) {
            Some(content) => content,
            None => return Ok(None),
        };

        let tokens = self.tokenize_document(&text);

        if tokens.is_empty() {
            Ok(None)
        } else {
            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data: tokens,
            })))
        }
    }
}

/// Run the LSP server over stdio
pub async fn run_lsp_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(MdfxLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
