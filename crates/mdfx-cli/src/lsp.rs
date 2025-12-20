//! LSP Server for mdfx template syntax
//!
//! Provides language server protocol support for mdfx template syntax,
//! including autocompletion for glyphs, styles, frames, components, and tech badges.
//!
//! Enable with: `cargo install mdfx-cli --features lsp`

use mdfx::Registry;
use mdfx_icons::{brand_color, list_icons};
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

    /// Build completion items for tech badge names (rust, typescript, docker, etc.)
    fn tech_name_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        list_icons()
            .iter()
            .filter(|name| prefix.is_empty() || name.starts_with(prefix))
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
            .collect()
    }

    /// Build completion items for tech badge parameters
    fn tech_param_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let params = vec![
            // Basic
            ("label", "Custom label text", "label=My Label"),
            ("bg", "Background color (both segments)", "bg=1a1a1a"),
            (
                "bg_left",
                "Left (icon) segment background",
                "bg_left=DEA584",
            ),
            (
                "bg_right",
                "Right (label) segment background",
                "bg_right=B8856E",
            ),
            ("logo", "Icon/logo color", "logo=FFFFFF"),
            ("text", "Label text color", "text=000000"),
            (
                "text_color",
                "Label text color (alias)",
                "text_color=FFFFFF",
            ),
            ("color", "Label text color (alias)", "color=000000"),
            ("font", "Custom font family", "font=Monaco,monospace"),
            (
                "font_family",
                "Custom font family (alias)",
                "font_family=Arial",
            ),
            // Sizing
            (
                "logo_size",
                "Icon size (xs/sm/md/lg/xl/xxl or pixels)",
                "logo_size=lg",
            ),
            (
                "icon_size",
                "Icon size (alias for logo_size)",
                "icon_size=16",
            ),
            ("height", "Badge height in pixels", "height=24"),
            ("raised", "Raised icon effect (pixels)", "raised=4"),
            // Corners & Shape
            ("rx", "Uniform corner radius", "rx=6"),
            (
                "corners",
                "Corner preset (left/right/none/all)",
                "corners=left",
            ),
            ("top_left", "Top-left corner radius", "top_left=8"),
            ("top_right", "Top-right corner radius", "top_right=8"),
            ("bottom_left", "Bottom-left corner radius", "bottom_left=8"),
            (
                "bottom_right",
                "Bottom-right corner radius",
                "bottom_right=8",
            ),
            ("chevron", "Arrow shape (left/right/both)", "chevron=right"),
            // Borders
            ("border", "Border color", "border=61DAFB"),
            ("border_width", "Border thickness", "border_width=2"),
            (
                "border_full",
                "Border around entire badge",
                "border_full=true",
            ),
            ("divider", "Center divider line", "divider=true"),
            // Style
            ("style", "Badge style", "style=flat"),
            // Advanced
            ("icon", "Custom SVG path data", "icon=M12 2L2 7..."),
            (
                "source",
                "Render source (shields for shields.io)",
                "source=shields",
            ),
            (
                "url",
                "Make badge a clickable link",
                "url=https://example.com",
            ),
        ];

        params
            .into_iter()
            .filter(|(name, _, _)| prefix.is_empty() || name.starts_with(prefix))
            .map(|(name, desc, example)| CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(desc.to_string()),
                documentation: Some(Documentation::String(format!(
                    "{}\n\nExample: {}",
                    desc, example
                ))),
                insert_text: Some(format!("{}=", name)),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect()
    }

    /// Build completion items for live data sources (github, npm, crates, pypi, codecov, actions)
    fn live_source_completions(&self, prefix: &str) -> Vec<CompletionItem> {
        let sources = vec![
            (
                "github",
                "GitHub repository metrics",
                "stars, forks, issues, license, language",
            ),
            ("npm", "npm package metrics", "version, license, next, beta"),
            (
                "crates",
                "crates.io package metrics",
                "version, downloads, description",
            ),
            (
                "pypi",
                "PyPI package metrics",
                "version, license, author, python, summary",
            ),
            (
                "codecov",
                "Codecov coverage metrics",
                "coverage, lines, hits, misses, files, branches",
            ),
            (
                "actions",
                "GitHub Actions workflow status",
                "status, conclusion, run_number, workflow",
            ),
        ];

        sources
            .into_iter()
            .filter(|(name, _, _)| prefix.is_empty() || name.starts_with(prefix))
            .map(|(name, desc, metrics)| CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some(desc.to_string()),
                documentation: Some(Documentation::String(format!(
                    "{}\n\nAvailable metrics: {}\n\nExample: {{{{ui:live:{}:query:metric/}}}}",
                    desc, metrics, name
                ))),
                insert_text: Some(format!("{}:", name)),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect()
    }

    /// Build completion items for live source metrics
    fn live_metric_completions(&self, source: &str, prefix: &str) -> Vec<CompletionItem> {
        let metrics: Vec<(&str, &str)> = match source {
            "github" => vec![
                ("stars", "Repository star count"),
                ("forks", "Fork count"),
                ("issues", "Open issue count"),
                ("watchers", "Watcher count"),
                ("license", "SPDX license identifier"),
                ("language", "Primary programming language"),
            ],
            "npm" => vec![
                ("version", "Latest stable version"),
                ("license", "Package license"),
                ("next", "Latest @next tag version"),
                ("beta", "Latest @beta tag version"),
            ],
            "crates" => vec![
                ("version", "Latest version"),
                ("downloads", "Total download count"),
                ("description", "Crate description"),
            ],
            "pypi" => vec![
                ("version", "Latest version"),
                ("license", "Package license"),
                ("author", "Package author"),
                ("python", "Required Python version"),
                ("summary", "Package summary"),
            ],
            "codecov" => vec![
                ("coverage", "Coverage percentage"),
                ("lines", "Total lines tracked"),
                ("hits", "Lines with coverage"),
                ("misses", "Lines without coverage"),
                ("files", "Number of files tracked"),
                ("branches", "Branch coverage count"),
            ],
            "actions" => vec![
                ("status", "Workflow run status (completed, in_progress, queued)"),
                ("conclusion", "Workflow conclusion (success, failure, cancelled)"),
                ("run_number", "Workflow run number"),
                ("workflow", "Workflow name"),
            ],
            _ => vec![],
        };

        metrics
            .into_iter()
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

    /// Build completion items for tech badge parameter values
    fn tech_param_value_completions(&self, param: &str, prefix: &str) -> Vec<CompletionItem> {
        let values: Vec<(&str, &str)> = match param {
            "logo_size" | "icon_size" => vec![
                ("xs", "10px - Extra small"),
                ("sm", "12px - Small"),
                ("md", "14px - Medium (default)"),
                ("lg", "16px - Large"),
                ("xl", "18px - Extra large"),
                ("xxl", "20px - Extra extra large"),
            ],
            "corners" => vec![
                ("left", "Rounded left, square right"),
                ("right", "Square left, rounded right"),
                ("none", "All square corners"),
                ("all", "All rounded corners"),
            ],
            "chevron" => vec![
                ("left", "Left-pointing arrow ←"),
                ("right", "Right-pointing arrow →"),
                ("both", "Both arrows ← →"),
            ],
            "style" => vec![
                ("flat", "Rounded corners (rx=3)"),
                ("flat-square", "Sharp corners (default)"),
                ("plastic", "Shiny gradient overlay"),
                ("for-the-badge", "Tall blocks (height=28)"),
                ("social", "Very rounded (rx=10)"),
                ("outline", "Border-only with transparent fill"),
                ("ghost", "Alias for outline"),
            ],
            "border_full" | "divider" => vec![("true", "Enable"), ("false", "Disable (default)")],
            "source" => vec![("shields", "Use shields.io URL instead of SVG")],
            // For color parameters, return palette completions
            "bg" | "bg_left" | "bg_right" | "logo" | "text" | "text_color" | "color" | "border" => {
                return self.palette_completions(prefix);
            }
            _ => vec![],
        };

        values
            .into_iter()
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
            CompletionContext::TechName(prefix) => self.tech_name_completions(&prefix),
            CompletionContext::TechParam(prefix) => self.tech_param_completions(&prefix),
            CompletionContext::TechParamValue(param, prefix) => {
                self.tech_param_value_completions(&param, &prefix)
            }
            CompletionContext::LiveSource(prefix) => self.live_source_completions(&prefix),
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
}

/// Run the LSP server over stdio
pub async fn run_lsp_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(MdfxLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
