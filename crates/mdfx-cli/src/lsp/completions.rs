//! Completion building and context analysis for the mdfx LSP
//!
//! Provides cached completion items and context detection for autocompletion.

use mdfx::components::params::{LIVE_SOURCES, TECH_PARAMS};
use mdfx::Registry;
use mdfx_icons::{brand_color, list_icons};
use tower_lsp::lsp_types::*;

/// Cached completion items built at startup for performance
#[allow(dead_code)]
pub struct CachedCompletions {
    /// All glyph completions
    pub glyphs: Vec<CompletionItem>,
    /// All style completions (includes aliases) - used via top_level
    pub styles: Vec<CompletionItem>,
    /// All frame completions (includes aliases)
    pub frames: Vec<CompletionItem>,
    /// All component completions - used via top_level
    pub components: Vec<CompletionItem>,
    /// All palette color completions
    pub palette: Vec<CompletionItem>,
    /// All shield style completions
    pub shield_styles: Vec<CompletionItem>,
    /// All tech name completions
    pub tech_names: Vec<CompletionItem>,
    /// All tech parameter completions
    pub tech_params: Vec<CompletionItem>,
    /// All live source completions
    pub live_sources: Vec<CompletionItem>,
    /// Top-level completions (glyph:, frame:, styles, components)
    pub top_level: Vec<CompletionItem>,
}

/// Context for completions
pub enum CompletionContext {
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

impl CachedCompletions {
    /// Build all completion items once at startup
    pub fn build(registry: &Registry) -> Self {
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

        // Add "ui:version:" prefix
        top_level.push(CompletionItem {
            label: "ui:version:".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Semantic version badge".to_string()),
            documentation: Some(Documentation::String(
                "Render version badges with auto-detected status coloring.\n\n\
                Status detection:\n\
                - Stable (green): 1.0.0, 2.5.3\n\
                - Beta (yellow): 0.x.x, -beta, -rc, -preview\n\
                - Alpha (orange): -alpha\n\
                - Dev (purple): -dev, -snapshot, -nightly\n\
                - Deprecated (red): -deprecated, -eol\n\n\
                Example: {{ui:version:1.0.0/}}, {{ui:version:2.0.0-beta.1/}}".to_string()
            )),
            insert_text: Some("ui:version:".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });

        // Add "ui:license:" prefix
        top_level.push(CompletionItem {
            label: "ui:license:".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("License badge".to_string()),
            documentation: Some(Documentation::String(
                "Render license badges with category-aware coloring.\n\n\
                Categories:\n\
                - Permissive (green): MIT, Apache-2.0, BSD, ISC\n\
                - Weak Copyleft (blue): LGPL, MPL, EPL\n\
                - Copyleft (yellow): GPL, AGPL\n\
                - Public Domain (cyan): CC0, Unlicense\n\
                - Proprietary (gray): Proprietary, Commercial\n\n\
                Example: {{ui:license:MIT/}}, {{ui:license:Apache-2.0/}}".to_string()
            )),
            insert_text: Some("ui:license:".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });

        // Add "ui:row" block component
        top_level.push(CompletionItem {
            label: "ui:row".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Horizontal row of badges".to_string()),
            documentation: Some(Documentation::String(
                "Horizontal row of badges with alignment control.\n\
                Wraps in HTML for GitHub compatibility.\n\n\
                Parameters:\n\
                - align: left, center (default), right\n\n\
                Example: {{ui:row}}{{ui:tech:rust/}}{{ui:tech:go/}}{{/ui}}\n\
                Example: {{ui:row:align=left}}...{{/ui}}".to_string()
            )),
            insert_text: Some("ui:row}}$1{{/ui}}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        // Add "ui:tech-group" block component
        top_level.push(CompletionItem {
            label: "ui:tech-group".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Group of badges with auto corner handling".to_string()),
            documentation: Some(Documentation::String(
                "Group badges (tech, version, license) with automatic corner handling.\n\
                First badge gets left corners, last gets right corners, middle badges are square.\n\
                All parameters are inherited by child badges unless overridden.\n\n\
                Parameters:\n\
                - gap: Gap between badges in pixels (default: 0)\n\
                - style, bg, text, etc.: Inherited by all children\n\n\
                Example: {{ui:tech-group}}{{ui:tech:rust/}}{{ui:tech:go/}}{{/ui}}\n\
                Example: {{ui:tech-group:style=flat:gap=2}}...{{/ui}}".to_string()
            )),
            insert_text: Some("ui:tech-group}}$1{{/ui}}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        // Add "ui:sparkline:" data visualization component
        top_level.push(CompletionItem {
            label: "ui:sparkline:".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Mini inline chart".to_string()),
            documentation: Some(Documentation::String(
                "Mini inline chart for data visualization.\n\n\
                Parameters:\n\
                - type: line, bar, or area (default: line)\n\
                - width: Chart width in pixels (default: 100)\n\
                - height: Chart height in pixels (default: 20)\n\
                - fill: Line/bar color (default: pink)\n\
                - stroke: Line stroke color\n\
                - dots: Show dots at data points\n\n\
                Example: {{ui:sparkline:1,3,2,5,4/}}\n\
                Example: {{ui:sparkline:1,2,3:type=bar:fill=accent/}}".to_string()
            )),
            insert_text: Some("ui:sparkline:${1:values}/}}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        // Add "ui:rating:" star/heart rating component
        top_level.push(CompletionItem {
            label: "ui:rating:".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Star/heart rating display".to_string()),
            documentation: Some(Documentation::String(
                "Star/heart/circle rating display with partial fill support.\n\n\
                Parameters:\n\
                - max: Maximum rating value (default: 5)\n\
                - icon: star, heart, or circle (default: star)\n\
                - size: Icon size in pixels (default: 20)\n\
                - fill: Filled icon color (default: warning/yellow)\n\
                - empty: Empty icon color (default: gray)\n\n\
                Example: {{ui:rating:4.5/}}\n\
                Example: {{ui:rating:3:max=5:icon=heart:fill=error/}}".to_string()
            )),
            insert_text: Some("ui:rating:${1:value}/}}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        // Add "ui:waveform:" audio visualization component
        top_level.push(CompletionItem {
            label: "ui:waveform:".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Audio-style waveform visualization".to_string()),
            documentation: Some(Documentation::String(
                "Audio-style waveform with bars above/below center.\n\n\
                Parameters:\n\
                - width: Total width in pixels (default: 100)\n\
                - height: Total height in pixels (default: 40)\n\
                - positive/up: Color for bars above zero (default: success)\n\
                - negative/down: Color for bars below zero (default: error)\n\
                - bar_width/bar: Width of each bar (default: 3)\n\
                - center: Show center line (default: false)\n\n\
                Example: {{ui:waveform:0.5,-0.3,0.8,-0.6/}}\n\
                Example: {{ui:waveform:1,-1,0.5:positive=accent:negative=pink/}}".to_string()
            )),
            insert_text: Some("ui:waveform:${1:values}/}}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
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
}

/// Filter cached completions by prefix
pub fn filter_completions(items: &[CompletionItem], prefix: &str) -> Vec<CompletionItem> {
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

/// Analyze the text around the cursor to determine completion context
pub fn get_completion_context(registry: &Registry, text: &str, position: Position) -> CompletionContext {
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
                if registry.component(comp_name).is_some() {
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
