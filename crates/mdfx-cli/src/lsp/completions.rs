//! Completion building and context analysis for the mdfx LSP
//!
//! Provides cached completion items and context detection for autocompletion.

use mdfx::components::params::{params_for_visualization, LIVE_SOURCES, TECH_PARAMS};
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
    /// UI namespace completions (tech:, version:, license:, row, etc.)
    pub ui_namespace: Vec<CompletionItem>,
}

/// Context for completions
pub enum CompletionContext {
    None,
    TopLevel(String),    // After {{ - show styles, frames, components, glyph:
    UiNamespace(String), // After {{ui: - show UI components (tech:, version:, license:, etc.)
    Glyph(String),       // After {{glyph: - show glyph names
    Frame(String),       // After {{frame: - show frame names
    Palette(String),     // Inside color parameter - show palette colors
    ShieldStyle(String), // After style= - show shield styles (flat, flat-square, etc.)
    TechName(String),    // After {{ui:tech: - show tech names (rust, typescript, etc.)
    TechParam(String),   // After {{ui:tech:NAME: - show parameter names
    TechParamValue(String, String), // After {{ui:tech:NAME:param= - show values for param
    LiveSource(String),  // After {{ui:live: - show live data sources (github, npm, etc.)
    LiveMetric(String, String), // After {{ui:live:SOURCE:QUERY: - show metrics for source
    VisualizationParam(String, String), // After {{ui:gauge:VALUE: - (component_type, prefix)
    VisualizationParamValue(String, String, String), // After {{ui:gauge:VALUE:param= - (component, param, prefix)
}

impl CachedCompletions {
    /// Build all completion items once at startup
    #[allow(clippy::vec_init_then_push)]
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
                insert_text: Some(format!("{}/", name)),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            })
            .collect();

        // Build style completions (includes aliases)
        // Note: insert_text is just the name; closing }} comes from editor auto-pair
        let styles: Vec<CompletionItem> = registry
            .styles()
            .iter()
            .flat_map(|(name, style)| {
                let mut items = vec![CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(style.name.clone()),
                    documentation: style.description.clone().map(Documentation::String),
                    insert_text: Some(name.clone()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    ..Default::default()
                }];
                for alias in &style.aliases {
                    items.push(CompletionItem {
                        label: alias.clone(),
                        kind: Some(CompletionItemKind::FUNCTION),
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

        // Build frame completions (includes aliases)
        // Note: insert_text is just the name; closing }} comes from editor auto-pair
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
                    insert_text: Some(name.clone()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
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
                        insert_text: Some(alias.clone()),
                        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
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
                Example: {{ui:version:1.0.0/}}, {{ui:version:2.0.0-beta.1/}}"
                    .to_string(),
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
                Example: {{ui:license:MIT/}}, {{ui:license:Apache-2.0/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:license:".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });

        // Add "ui:tech:" prefix for technology badges
        top_level.push(CompletionItem {
            label: "ui:tech:".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Technology badge".to_string()),
            documentation: Some(Documentation::String(
                "Render technology/language badges with brand colors.\n\n\
                Supports 600+ technologies with brand colors and icons.\n\n\
                Parameters:\n\
                - style: flat, flat-square, plastic, for-the-badge, social\n\
                - bg: Background color override\n\
                - text: Text color override\n\
                - label: Custom label text\n\n\
                Example: {{ui:tech:rust/}}, {{ui:tech:typescript:style=flat/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:tech:".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });

        // Add "ui:live:" prefix for live data badges
        top_level.push(CompletionItem {
            label: "ui:live:".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Live data badge".to_string()),
            documentation: Some(Documentation::String(
                "Render live data badges from external sources.\n\n\
                Sources:\n\
                - github: GitHub repository stats (stars, forks, issues, etc.)\n\
                - npm: npm package stats (downloads, version)\n\
                - crates: crates.io stats (downloads, version)\n\
                - pypi: PyPI package stats\n\n\
                Example: {{ui:live:github:owner/repo:stars/}}\n\
                Example: {{ui:live:npm:package-name:downloads/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:live:".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });

        // Add "ui:progress:" progress bar component
        top_level.push(CompletionItem {
            label: "ui:progress:".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Progress bar".to_string()),
            documentation: Some(Documentation::String(
                "Render progress bar visualization.\n\n\
                Parameters:\n\
                - width: Bar width in pixels (default: 100)\n\
                - height: Bar height in pixels (default: 10)\n\
                - fill: Fill color (default: pink)\n\
                - track: Track color (default: gray)\n\
                - label: Show percentage label\n\n\
                Example: {{ui:progress:75/}}\n\
                Example: {{ui:progress:50:fill=success:width=200/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:progress:${1:value}/".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        // Add "ui:donut:" donut chart component
        top_level.push(CompletionItem {
            label: "ui:donut:".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Donut chart".to_string()),
            documentation: Some(Documentation::String(
                "Render donut/ring chart visualization.\n\n\
                Parameters:\n\
                - size: Diameter in pixels (default: 40)\n\
                - thickness: Ring thickness (default: 4)\n\
                - fill: Fill color (default: pink)\n\
                - track: Track color (default: gray)\n\
                - label: Show percentage label\n\n\
                Example: {{ui:donut:75/}}\n\
                Example: {{ui:donut:50:size=60:fill=accent/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:donut:${1:value}/".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        // Add "ui:gauge:" gauge meter component
        top_level.push(CompletionItem {
            label: "ui:gauge:".to_string(),
            kind: Some(CompletionItemKind::MODULE),
            detail: Some("Gauge meter".to_string()),
            documentation: Some(Documentation::String(
                "Render gauge/speedometer visualization.\n\n\
                Parameters:\n\
                - size: Width in pixels (default: 80)\n\
                - thickness: Arc thickness (default: 8)\n\
                - fill: Fill color (default: pink)\n\
                - track: Track color (default: gray)\n\
                - label: Show percentage label\n\n\
                Example: {{ui:gauge:75/}}\n\
                Example: {{ui:gauge:50:size=100:fill=warning/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:gauge:${1:value}/".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
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
                Example: {{ui:row:align=left}}...{{/ui}}"
                    .to_string(),
            )),
            insert_text: Some("ui:row".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
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
                Example: {{ui:tech-group:style=flat:gap=2}}...{{/ui}}"
                    .to_string(),
            )),
            insert_text: Some("ui:tech-group".to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
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
                Example: {{ui:sparkline:1,2,3:type=bar:fill=accent/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:sparkline:${1:values}/".to_string()),
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
                Example: {{ui:rating:3:max=5:icon=heart:fill=error/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:rating:${1:value}/".to_string()),
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
                Example: {{ui:waveform:1,-1,0.5:positive=accent:negative=pink/}}"
                    .to_string(),
            )),
            insert_text: Some("ui:waveform:${1:values}/".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        // Add styles and components to top-level
        top_level.extend(styles.clone());
        top_level.extend(components.clone());

        // Build UI namespace completions (shown after {{ui:)
        // Note: insert_text does NOT include closing }} since many editors auto-pair brackets
        let ui_namespace = vec![
            CompletionItem {
                label: "tech:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Technology badge".to_string()),
                documentation: Some(Documentation::String(
                    "Render technology/language badges with brand colors.\n\n\
                    Example: {{ui:tech:rust/}}, {{ui:tech:typescript:style=flat/}}"
                        .to_string(),
                )),
                insert_text: Some("tech:".to_string()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            },
            CompletionItem {
                label: "version:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Version badge".to_string()),
                documentation: Some(Documentation::String(
                    "Render version badges with auto-detected status coloring.\n\n\
                    Example: {{ui:version:1.0.0/}}, {{ui:version:2.0.0-beta.1/}}"
                        .to_string(),
                )),
                insert_text: Some("version:${1:version}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "license:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("License badge".to_string()),
                documentation: Some(Documentation::String(
                    "Render license badges with category-aware coloring.\n\n\
                    Example: {{ui:license:MIT/}}, {{ui:license:Apache-2.0/}}"
                        .to_string(),
                )),
                insert_text: Some("license:${1:license}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "live:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Live data badge".to_string()),
                documentation: Some(Documentation::String(
                    "Render live data badges from external sources.\n\n\
                    Example: {{ui:live:github:owner/repo:stars/}}"
                        .to_string(),
                )),
                insert_text: Some("live:".to_string()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            },
            CompletionItem {
                label: "row".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Horizontal row of badges".to_string()),
                documentation: Some(Documentation::String(
                    "Horizontal row of badges with alignment control.\n\n\
                    Example: {{ui:row}}...{{/ui}}"
                        .to_string(),
                )),
                insert_text: Some("row".to_string()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            },
            CompletionItem {
                label: "tech-group".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Group of badges with auto corner handling".to_string()),
                documentation: Some(Documentation::String(
                    "Group badges with automatic corner handling.\n\n\
                    Example: {{ui:tech-group}}...{{/ui}}"
                        .to_string(),
                )),
                insert_text: Some("tech-group".to_string()),
                insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                ..Default::default()
            },
            CompletionItem {
                label: "progress:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Progress bar".to_string()),
                documentation: Some(Documentation::String(
                    "Render progress bar visualization.\n\n\
                    Example: {{ui:progress:75/}}"
                        .to_string(),
                )),
                insert_text: Some("progress:${1:value}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "donut:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Donut chart".to_string()),
                documentation: Some(Documentation::String(
                    "Render donut/pie chart visualization.\n\n\
                    Example: {{ui:donut:75/}}"
                        .to_string(),
                )),
                insert_text: Some("donut:${1:value}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "gauge:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Gauge meter".to_string()),
                documentation: Some(Documentation::String(
                    "Render gauge/speedometer visualization.\n\n\
                    Example: {{ui:gauge:75/}}"
                        .to_string(),
                )),
                insert_text: Some("gauge:${1:value}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "sparkline:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Mini inline chart".to_string()),
                documentation: Some(Documentation::String(
                    "Mini inline chart for data visualization.\n\n\
                    Example: {{ui:sparkline:1,3,2,5,4/}}"
                        .to_string(),
                )),
                insert_text: Some("sparkline:${1:values}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "rating:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Star/heart rating".to_string()),
                documentation: Some(Documentation::String(
                    "Star/heart rating display.\n\n\
                    Example: {{ui:rating:4.5/}}"
                        .to_string(),
                )),
                insert_text: Some("rating:${1:value}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "waveform:".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Audio waveform".to_string()),
                documentation: Some(Documentation::String(
                    "Audio-style waveform visualization.\n\n\
                    Example: {{ui:waveform:0.5,-0.3,0.8/}}"
                        .to_string(),
                )),
                insert_text: Some("waveform:${1:values}/".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
        ];

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
            ui_namespace,
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

/// Build completion items for visualization component parameters
pub fn build_visualization_param_completions(
    component: &str,
    prefix: &str,
) -> Vec<CompletionItem> {
    let params = match params_for_visualization(component) {
        Some(p) => p,
        None => return vec![],
    };

    params
        .iter()
        .filter(|p| prefix.is_empty() || p.name.starts_with(prefix))
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

/// Build completion items for visualization parameter values
pub fn build_visualization_param_value_completions(
    component: &str,
    param: &str,
    prefix: &str,
    palette: &[CompletionItem],
) -> Vec<CompletionItem> {
    let params = match params_for_visualization(component) {
        Some(p) => p,
        None => return vec![],
    };

    // Check if this is a color parameter
    let color_params = ["fill", "track", "empty", "positive", "negative", "up", "down", "stroke", "thumb_color", "thumb_border"];
    if color_params.contains(&param) {
        return filter_completions(palette, prefix);
    }

    // Look up enumerated values for this parameter
    let param_info = params.iter().find(|p| p.name == param);
    if let Some(info) = param_info {
        if let Some(values) = info.values {
            return values
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
                .collect();
        }
    }

    vec![]
}

/// Analyze the text around the cursor to determine completion context
pub fn get_completion_context(
    registry: &Registry,
    text: &str,
    position: Position,
) -> CompletionContext {
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

        // Check for UI namespace: {{ui: (but not {{ui:tech: or {{ui:live: etc.)
        if let Some(rest) = after_open.strip_prefix("ui:") {
            // Check for visualization components first: {{ui:gauge:55:, {{ui:progress:75:, etc.
            for viz_type in &[
                "progress:",
                "donut:",
                "gauge:",
                "sparkline:",
                "rating:",
                "waveform:",
            ] {
                if let Some(viz_rest) = rest.strip_prefix(viz_type) {
                    let component = viz_type.trim_end_matches(':');
                    // Parse: VALUE:param=value:param2=...
                    let parts: Vec<&str> = viz_rest.split(':').collect();

                    // If we have a value and at least one colon after it
                    if !parts.is_empty() && viz_rest.contains(':') {
                        // Check if we're in a parameter value (after =)
                        if let Some(eq_pos) = viz_rest.rfind('=') {
                            let after_eq = &viz_rest[eq_pos + 1..];
                            let before_eq = &viz_rest[..eq_pos];
                            if let Some(colon_pos) = before_eq.rfind(':') {
                                let param_name = &before_eq[colon_pos + 1..];
                                return CompletionContext::VisualizationParamValue(
                                    component.to_string(),
                                    param_name.to_string(),
                                    after_eq.to_string(),
                                );
                            }
                        }

                        // We're ready for parameter name completion
                        let last_part = parts.last().unwrap_or(&"");
                        if !last_part.contains('=') {
                            return CompletionContext::VisualizationParam(
                                component.to_string(),
                                last_part.to_string(),
                            );
                        }
                    }
                }
            }

            // If it's a more specific prefix, let those handlers deal with it
            if rest.starts_with("tech:")
                || rest.starts_with("live:")
                || rest.starts_with("version:")
                || rest.starts_with("license:")
                || rest.starts_with("row")
                || rest.starts_with("tech-group")
            {
                // Fall through to more specific handlers below
            } else if rest.starts_with("progress:")
                || rest.starts_with("donut:")
                || rest.starts_with("gauge:")
                || rest.starts_with("sparkline:")
                || rest.starts_with("rating:")
                || rest.starts_with("waveform:")
            {
                // Already handled above, but might not have enough context yet
                // Fall through
            } else {
                // Just {{ui: or {{ui:partial - show UI namespace completions
                return CompletionContext::UiNamespace(rest.to_string());
            }
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
        if after_open.contains(':') && (after_open.contains("bg=") || after_open.contains("fg=")) {
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
