//! LSP Server for mdfx template syntax
//!
//! Provides language server protocol support for mdfx template syntax,
//! including autocompletion for glyphs, styles, frames, and components.
//!
//! Enable with: `cargo install mdfx-cli --features lsp`

#![cfg(feature = "lsp")]

use mdfx::Registry;
use std::sync::Arc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

/// The mdfx language server
pub struct MdfxLanguageServer {
    client: Client,
    registry: Arc<Registry>,
}

impl MdfxLanguageServer {
    pub fn new(client: Client) -> Self {
        let registry = Registry::new().expect("Failed to load registry");
        Self {
            client,
            registry: Arc::new(registry),
        }
    }

    /// Build completion items for glyphs
    fn glyph_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.registry
            .glyphs()
            .iter()
            .filter(|(name, _)| prefix.is_empty() || name.starts_with(prefix))
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
            .collect()
    }

    /// Build completion items for styles
    fn style_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.registry
            .styles()
            .iter()
            .filter(|(name, _)| prefix.is_empty() || name.starts_with(prefix))
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
                // Add aliases
                for alias in &style.aliases {
                    if prefix.is_empty() || alias.starts_with(prefix) {
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
                }
                items
            })
            .collect()
    }

    /// Build completion items for frames
    fn frame_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.registry
            .frames()
            .iter()
            .filter(|(name, _)| prefix.is_empty() || name.starts_with(prefix))
            .flat_map(|(name, frame)| {
                let mut items = vec![CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::STRUCT),
                    detail: Some(format!("{} ... {}", frame.prefix.trim(), frame.suffix.trim())),
                    documentation: frame.description.clone().map(Documentation::String),
                    insert_text: Some(format!("{}}}${{1:text}}{{{{/{}}}}}", name, name)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                }];
                // Add aliases
                for alias in &frame.aliases {
                    if prefix.is_empty() || alias.starts_with(prefix) {
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
                }
                items
            })
            .collect()
    }

    /// Build completion items for components
    fn component_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.registry
            .components()
            .iter()
            .filter(|(name, _)| prefix.is_empty() || name.starts_with(prefix))
            .map(|(name, component)| {
                let insert_text = if component.self_closing {
                    if component.args.is_empty() {
                        format!("{}/}}}}", name)
                    } else {
                        // Build snippet with placeholders for args
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
            .collect()
    }

    /// Build completion items for palette colors
    fn palette_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.registry
            .palette()
            .iter()
            .filter(|(name, _)| prefix.is_empty() || name.starts_with(prefix))
            .map(|(name, hex)| CompletionItem {
                label: name.clone(),
                kind: Some(CompletionItemKind::COLOR),
                detail: Some(format!("#{}", hex)),
                documentation: Some(Documentation::String(format!("Hex color: #{}", hex))),
                insert_text: Some(name.clone()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect()
    }

    /// Build completion items for shield styles (flat, flat-square, for-the-badge, etc.)
    fn shield_style_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        self.registry
            .shield_styles()
            .iter()
            .filter(|(name, _)| prefix.is_empty() || name.starts_with(prefix))
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
                // Add aliases
                for alias in &style.aliases {
                    if prefix.is_empty() || alias.starts_with(prefix) {
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
                }
                items
            })
            .collect()
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

            // Check for style= parameter (shield styles like flat, flat-square, for-the-badge)
            if after_open.contains("style=") {
                if let Some(style_pos) = after_open.rfind("style=") {
                    let style_prefix = &after_open[style_pos + 6..];
                    // Don't include any trailing characters after the style value
                    let style_prefix = style_prefix
                        .split(|c| c == ':' || c == '/' || c == '}')
                        .next()
                        .unwrap_or(style_prefix);
                    return CompletionContext::ShieldStyle(style_prefix.to_string());
                }
            }

            // Check for color parameter (e.g., swatch:cobalt or bg=cobalt)
            if after_open.contains(':') && (after_open.contains("bg=") || after_open.contains("fg="))
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

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        // Read the document content
        // Note: In a full implementation, we'd track document content via didOpen/didChange
        // For now, we'll try to read from the file path
        let path = uri.path();
        let text = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Ok(None),
        };

        let context = self.get_completion_context(&text, position);

        let items = match context {
            CompletionContext::None => return Ok(None),
            CompletionContext::TopLevel(prefix) => {
                let mut items = Vec::new();

                // Add "glyph:" as a prefix option
                if "glyph".starts_with(&prefix) || prefix.is_empty() {
                    items.push(CompletionItem {
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
                }

                // Add "frame:" as a prefix option
                if "frame".starts_with(&prefix) || prefix.is_empty() {
                    items.push(CompletionItem {
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
                }

                // Add styles (directly usable like {{mathbold}}text{{/mathbold}})
                items.extend(self.style_completions(&prefix));

                // Add components
                items.extend(self.component_completions(&prefix));

                items
            }
            CompletionContext::Glyph(prefix) => self.glyph_completions(&prefix),
            CompletionContext::Frame(prefix) => self.frame_completions(&prefix),
            CompletionContext::Palette(prefix) => self.palette_completions(&prefix),
            CompletionContext::ShieldStyle(prefix) => self.shield_style_completions(&prefix),
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

        let path = uri.path();
        let text = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Ok(None),
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
                let name = glyph_name
                    .split(|c| c == '/' || c == '}')
                    .next()
                    .unwrap_or(glyph_name);
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
            let style_name = template_start
                .split(|c| c == ':' || c == '}')
                .next()
                .unwrap_or("");
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
            let comp_name = template_start
                .split(|c| c == ':' || c == '/' || c == '}')
                .next()
                .unwrap_or("");
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
}

/// Run the LSP server over stdio
pub async fn run_lsp_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(MdfxLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
