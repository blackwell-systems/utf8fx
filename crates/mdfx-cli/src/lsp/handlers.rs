//! LSP protocol handlers for the mdfx LSP
//!
//! Implements the LanguageServer trait for handling LSP requests.

use crate::lsp::code_actions::generate_code_actions;
use crate::lsp::color::{create_color_presentation, find_document_colors};
use crate::lsp::completions::{
    build_visualization_param_completions, build_visualization_param_value_completions,
    filter_completions, get_completion_context, CompletionContext,
};
use crate::lsp::diagnostics::generate_diagnostics;
use crate::lsp::inlay_hints::generate_inlay_hints;
use crate::lsp::parser::find_templates;
use crate::lsp::semantic_tokens::tokenize_document;
use crate::lsp::MdfxLanguageServer;
use mdfx::components::params::{self, TECH_PARAMS};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::LanguageServer;

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
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: vec![
                                    SemanticTokenType::NAMESPACE, // 0: component prefix (ui:tech, glyph)
                                    SemanticTokenType::TYPE,      // 1: tech name (rust, typescript)
                                    SemanticTokenType::PARAMETER, // 2: parameter name
                                    SemanticTokenType::STRING,    // 3: parameter value
                                    SemanticTokenType::VARIABLE,  // 4: palette color name
                                    SemanticTokenType::KEYWORD,   // 5: style name
                                    SemanticTokenType::FUNCTION,  // 6: frame name
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
                        },
                    ),
                ),
                color_provider: Some(ColorProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![CodeActionKind::QUICKFIX]),
                        ..Default::default()
                    },
                )),
                inlay_hint_provider: Some(OneOf::Left(true)),
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

        let diagnostics = generate_diagnostics(&self.registry, &text, &uri);
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

            let diagnostics = generate_diagnostics(&self.registry, &change.text, &uri);
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

        let context = get_completion_context(&self.registry, &text, position);

        // Use cached completions with filtering for fast responses
        let items = match context {
            CompletionContext::None => return Ok(None),
            CompletionContext::TopLevel(prefix) => {
                filter_completions(&self.cached.top_level, &prefix)
            }
            CompletionContext::UiNamespace(prefix) => {
                filter_completions(&self.cached.ui_namespace, &prefix)
            }
            CompletionContext::Glyph(prefix) => filter_completions(&self.cached.glyphs, &prefix),
            CompletionContext::Frame(prefix) => filter_completions(&self.cached.frames, &prefix),
            CompletionContext::Palette(prefix) => filter_completions(&self.cached.palette, &prefix),
            CompletionContext::ShieldStyle(prefix) => {
                filter_completions(&self.cached.shield_styles, &prefix)
            }
            CompletionContext::TechName(prefix) => {
                filter_completions(&self.cached.tech_names, &prefix)
            }
            CompletionContext::TechParam(prefix) => {
                filter_completions(&self.cached.tech_params, &prefix)
            }
            CompletionContext::TechParamValue(param, prefix) => {
                self.tech_param_value_completions(&param, &prefix)
            }
            CompletionContext::LiveSource(prefix) => {
                filter_completions(&self.cached.live_sources, &prefix)
            }
            CompletionContext::LiveMetric(source, prefix) => {
                self.live_metric_completions(&source, &prefix)
            }
            CompletionContext::VisualizationParam(component, prefix) => {
                build_visualization_param_completions(&component, &prefix)
            }
            CompletionContext::VisualizationParamValue(component, param, prefix) => {
                build_visualization_param_value_completions(
                    &component,
                    &param,
                    &prefix,
                    &self.cached.palette,
                )
            }
        };

        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(CompletionResponse::Array(items)))
        }
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        let text = match self.get_document_content(uri) {
            Some(content) => content,
            None => return Ok(None),
        };

        let actions =
            generate_code_actions(&self.registry, &text, uri, &params.context.diagnostics);

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
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

            // Check for UI components with preview
            if let Some(rest) = template_start.strip_prefix("ui:") {
                // Get full template content (before + after until / or }})
                let after_part = after.split(['/', '}']).next().unwrap_or("");
                let full_rest = format!("{}{}", rest, after_part);

                // Parse the template
                use super::preview::*;
                let (component_and_args, params) = parse_template_params(&full_rest);
                let parts: Vec<&str> = component_and_args.split(':').collect();

                if let Some(component_type) = parts.first() {
                    let palette = self.registry.palette();
                    let preview = match *component_type {
                        "tech" => {
                            // ui:tech:NAME:params
                            if let Some(tech_name) = parts.get(1) {
                                // Collect remaining parts as potential params
                                let mut all_params = params.clone();
                                for part in parts.iter().skip(2) {
                                    if let Some((k, v)) = part.split_once('=') {
                                        all_params.push((k.to_string(), v.to_string()));
                                    }
                                }
                                Some(tech_badge_preview(tech_name, &all_params, palette))
                            } else {
                                None
                            }
                        }
                        "swatch" => {
                            // ui:swatch:COLOR:params
                            if let Some(color) = parts.get(1) {
                                let color = color.trim_end_matches('/');
                                let resolved = resolve_color_hex(color, palette);
                                let size = get_param_u32(&params, "size", 40);
                                Some(swatch_preview(&resolved, size))
                            } else {
                                None
                            }
                        }
                        "progress" => {
                            // ui:progress:PERCENT:params
                            if let Some(pct) = parts.get(1) {
                                let percent: u8 = pct.parse().unwrap_or(50);
                                let width = get_param_u32(&params, "width", 100);
                                let height = get_param_u32(&params, "height", 10);
                                let fill = resolve_color_hex(
                                    get_param(&params, "fill", "F472B6"),
                                    palette,
                                );
                                let track = resolve_color_hex(
                                    get_param(&params, "track", "4B5563"),
                                    palette,
                                );
                                Some(progress_preview(percent, width, height, &fill, &track))
                            } else {
                                None
                            }
                        }
                        "donut" => {
                            // ui:donut:PERCENT:params
                            if let Some(pct) = parts.get(1) {
                                let percent: u8 = pct.parse().unwrap_or(50);
                                let size = get_param_u32(&params, "size", 40);
                                let thickness = get_param_u32(&params, "thickness", 4);
                                let fill = resolve_color_hex(
                                    get_param(&params, "fill", "F472B6"),
                                    palette,
                                );
                                let track = resolve_color_hex(
                                    get_param(&params, "track", "4B5563"),
                                    palette,
                                );
                                Some(donut_preview(percent, size, thickness, &fill, &track))
                            } else {
                                None
                            }
                        }
                        "gauge" => {
                            // ui:gauge:PERCENT:params
                            if let Some(pct) = parts.get(1) {
                                let percent: u8 = pct.parse().unwrap_or(50);
                                let size = get_param_u32(&params, "size", 80);
                                let thickness = get_param_u32(&params, "thickness", 8);
                                let fill = resolve_color_hex(
                                    get_param(&params, "fill", "F472B6"),
                                    palette,
                                );
                                let track = resolve_color_hex(
                                    get_param(&params, "track", "4B5563"),
                                    palette,
                                );
                                Some(gauge_preview(percent, size, thickness, &fill, &track))
                            } else {
                                None
                            }
                        }
                        "rating" => {
                            // ui:rating:VALUE:params
                            if let Some(val) = parts.get(1) {
                                let value: f32 = val.parse().unwrap_or(3.5);
                                let max = get_param_u32(&params, "max", 5);
                                let size = get_param_u32(&params, "size", 20);
                                let fill = resolve_color_hex(
                                    get_param(&params, "fill", "EAB308"),
                                    palette,
                                );
                                let empty = resolve_color_hex(
                                    get_param(&params, "empty", "4B5563"),
                                    palette,
                                );
                                Some(rating_preview(value, max, size, &fill, &empty))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    if let Some(preview_content) = preview {
                        return Ok(Some(Hover {
                            contents: HoverContents::Markup(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: preview_content,
                            }),
                            range: None,
                        }));
                    }
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
            for (start, is_closing, _is_self_closing, _is_malformed, template_content, end) in
                find_templates(line)
            {
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
                    (
                        SymbolKind::CONSTANT,
                        format!("tech:{}", tech_name),
                        Some("Tech Badge".to_string()),
                    )
                } else if template_content.starts_with("ui:live:") {
                    let parts: Vec<&str> = template_content
                        .strip_prefix("ui:live:")
                        .unwrap_or("")
                        .split(':')
                        .collect();
                    let source = parts.first().unwrap_or(&"unknown");
                    (
                        SymbolKind::VARIABLE,
                        format!("live:{}", source),
                        Some("Live Badge".to_string()),
                    )
                } else if template_content.starts_with("glyph:") {
                    let glyph_name = template_content.strip_prefix("glyph:").unwrap_or("unknown");
                    (
                        SymbolKind::STRING,
                        format!("glyph:{}", glyph_name),
                        Some("Glyph".to_string()),
                    )
                } else if template_content.starts_with("swatch:") {
                    let color = template_content
                        .strip_prefix("swatch:")
                        .unwrap_or("")
                        .split(':')
                        .next()
                        .unwrap_or("unknown");
                    (
                        SymbolKind::CONSTANT,
                        format!("swatch:{}", color),
                        Some("Color Swatch".to_string()),
                    )
                } else {
                    // Check if it's a style
                    let name = template_content
                        .split([':', '}'])
                        .next()
                        .unwrap_or(template_content);
                    if self.registry.style(name).is_some() {
                        (
                            SymbolKind::FUNCTION,
                            name.to_string(),
                            Some("Style".to_string()),
                        )
                    } else if self.registry.component(name).is_some() {
                        (
                            SymbolKind::MODULE,
                            name.to_string(),
                            Some("Component".to_string()),
                        )
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

        let tokens = tokenize_document(&self.registry, &text);

        if tokens.is_empty() {
            Ok(None)
        } else {
            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                result_id: None,
                data: tokens,
            })))
        }
    }

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        let uri = &params.text_document.uri;
        let text = match self.get_document_content(uri) {
            Some(content) => content,
            None => return Ok(vec![]),
        };

        Ok(find_document_colors(&text))
    }

    async fn color_presentation(
        &self,
        params: ColorPresentationParams,
    ) -> Result<Vec<ColorPresentation>> {
        Ok(create_color_presentation(&params.color, params.range))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = &params.text_document.uri;
        let text = match self.get_document_content(uri) {
            Some(content) => content,
            None => return Ok(None),
        };

        let palette = self.registry.palette().clone();
        let hints = generate_inlay_hints(&text, &palette, &params.range);

        if hints.is_empty() {
            Ok(None)
        } else {
            Ok(Some(hints))
        }
    }
}

impl MdfxLanguageServer {
    /// Build completion items for live source metrics using shared definitions
    pub(crate) fn live_metric_completions(
        &self,
        source: &str,
        prefix: &str,
    ) -> Vec<CompletionItem> {
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
    pub(crate) fn tech_param_value_completions(
        &self,
        param: &str,
        prefix: &str,
    ) -> Vec<CompletionItem> {
        // Color parameters return palette completions from cache
        match param {
            "bg" | "bg_left" | "bg_right" | "logo" | "text" | "text_color" | "color" | "border" => {
                return filter_completions(&self.cached.palette, prefix);
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
}
