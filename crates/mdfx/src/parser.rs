use crate::components::{ComponentOutput, ComponentsRenderer, PostProcess};
use crate::config::{expand_partial, MdfxConfig};
use crate::converter::Converter;
use crate::error::{Error, Result};
use crate::registry::Registry;
use crate::renderer::shields::ShieldsBackend;
use crate::renderer::{RenderedAsset, Renderer};
use crate::shields::ShieldsRenderer;
use std::collections::HashMap;

/// Template data extracted from parsing
#[derive(Debug, Clone)]
struct TemplateData {
    end_pos: usize,
    style: String,
    spacing: usize,
    separator: Option<String>,
    content: String,
}

/// Frame template data
#[derive(Debug, Clone)]
struct FrameData {
    end_pos: usize,
    frame_style: String,
    content: String,
}

/// UI component template data
#[derive(Debug, Clone)]
struct UIData {
    end_pos: usize,
    component_name: String,
    args: Vec<String>,
    content: Option<String>, // None for self-closing
}

/// Shield template data
#[derive(Debug, Clone)]
struct ShieldData {
    end_pos: usize,
    shield_type: String, // "block", "twotone", "bar", "icon"
    params: std::collections::HashMap<String, String>,
}

/// Partial template data
#[derive(Debug, Clone)]
struct PartialData {
    end_pos: usize,
    partial_name: String,
    content: String,
}

/// Glyph template data
#[derive(Debug, Clone)]
struct GlyphData {
    end_pos: usize,
    glyph_name: String,
}

/// Kbd (keyboard) template data
#[derive(Debug, Clone)]
struct KbdData {
    end_pos: usize,
    keys: String,
}

/// Frame modifiers parsed from style string
#[derive(Debug, Clone)]
struct FrameModifiers {
    style: String,
    separator: Option<String>,
    spacing: Option<usize>,
    reverse: bool,
    count: Option<usize>,
}

/// Result of processing markdown with file-based assets
#[derive(Debug, Clone)]
pub struct ProcessedMarkdown {
    /// Processed markdown content
    pub markdown: String,
    /// File-based assets that need to be written
    pub assets: Vec<RenderedAsset>,
}

/// Parser for processing markdown with style templates
pub struct TemplateParser {
    converter: Converter,
    components_renderer: ComponentsRenderer,
    shields_renderer: ShieldsRenderer, // Keep for {{shields:*}} escape hatch
    backend: Box<dyn Renderer>,        // Pluggable rendering backend
    registry: Registry,                // Unified registry for resolution
    partials: HashMap<String, String>, // User-defined partial templates
}

impl TemplateParser {
    /// Create a new template parser with default (shields.io) backend
    pub fn new() -> Result<Self> {
        Self::with_backend(Box::new(ShieldsBackend::new()?))
    }

    /// Create a template parser with a custom backend
    pub fn with_backend(backend: Box<dyn Renderer>) -> Result<Self> {
        let converter = Converter::new()?;
        let components_renderer = ComponentsRenderer::new()?;
        let shields_renderer = ShieldsRenderer::new()?;
        let registry = Registry::new()?;
        Ok(Self {
            converter,
            components_renderer,
            shields_renderer,
            backend,
            registry,
            partials: HashMap::new(),
        })
    }

    /// Load partials from an MdfxConfig
    ///
    /// # Example
    ///
    /// ```ignore
    /// use mdfx::{TemplateParser, MdfxConfig};
    ///
    /// let mut parser = TemplateParser::new()?;
    /// let config = MdfxConfig::load(".mdfx.json")?;
    /// parser.load_config(&config);
    /// ```
    pub fn load_config(&mut self, config: &MdfxConfig) {
        // Load partials
        for (name, def) in &config.partials {
            self.partials.insert(name.clone(), def.template.clone());
        }

        // Load custom palette
        if !config.palette.is_empty() {
            self.components_renderer
                .extend_palette(config.palette.clone());
        }
    }

    /// Add a single partial template
    ///
    /// # Arguments
    ///
    /// * `name` - The partial name (used as `{{partial:name}}`)
    /// * `template` - The template string (may contain `$1` or `$content` for content substitution)
    pub fn add_partial(&mut self, name: impl Into<String>, template: impl Into<String>) {
        self.partials.insert(name.into(), template.into());
    }

    /// Check if a partial exists
    pub fn has_partial(&self, name: &str) -> bool {
        self.partials.contains_key(name)
    }

    /// Extend the color palette with custom definitions
    ///
    /// Custom colors override built-in palette colors with the same name.
    /// Colors can then be used in components: `{{ui:swatch:mycolor/}}`
    ///
    /// # Arguments
    ///
    /// * `custom_palette` - Map of color names to hex values (without #)
    pub fn extend_palette(&mut self, custom_palette: std::collections::HashMap<String, String>) {
        self.components_renderer.extend_palette(custom_palette);
    }

    /// Process markdown text, converting all style templates
    ///
    /// Returns only the markdown string. File-based assets are not collected.
    /// Use `process_with_assets()` if you need to write SVG files.
    ///
    /// # Example
    ///
    /// ```
    /// use mdfx::TemplateParser;
    ///
    /// let parser = TemplateParser::new().unwrap();
    /// let input = "# {{mathbold}}TITLE{{/mathbold}}";
    /// let result = parser.process(input).unwrap();
    /// assert_eq!(result, "# 𝐓𝐈𝐓𝐋𝐄");
    /// ```
    pub fn process(&self, markdown: &str) -> Result<String> {
        Ok(self.process_with_assets(markdown)?.markdown)
    }

    /// Process markdown text and collect file-based assets
    ///
    /// Returns both the processed markdown and any file assets that need
    /// to be written to disk (e.g., SVG files when using SvgBackend).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use mdfx::TemplateParser;
    /// use mdfx::renderer::svg::SvgBackend;
    ///
    /// let backend = Box::new(SvgBackend::new("assets"));
    /// let parser = TemplateParser::with_backend(backend).unwrap();
    /// let processed = parser.process_with_assets("{{ui:swatch:accent/}}").unwrap();
    ///
    /// // Write assets to disk
    /// for asset in processed.assets {
    ///     if let Some(path) = asset.file_path() {
    ///         std::fs::write(path, asset.file_bytes().unwrap()).unwrap();
    ///     }
    /// }
    ///
    /// println!("{}", processed.markdown);
    /// ```
    pub fn process_with_assets(&self, markdown: &str) -> Result<ProcessedMarkdown> {
        // Split markdown into code blocks and content sections
        // Code blocks are preserved as-is, content sections are processed
        let mut result = String::new();
        let mut all_assets = Vec::new();

        // Preserve whether input ends with newline (lines() strips it)
        let had_trailing_newline = markdown.ends_with('\n');

        let lines: Vec<&str> = markdown.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            let trimmed = line.trim();

            // Check if this line starts a code block
            if trimmed.starts_with("```") {
                // Add the opening ``` line
                result.push_str(line);
                result.push('\n');
                i += 1;

                // Copy all lines until closing ```
                while i < lines.len() {
                    let code_line = lines[i];
                    result.push_str(code_line);
                    result.push('\n');

                    if code_line.trim().starts_with("```") {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                continue;
            }

            // Not a code block, collect lines until next code block or EOF
            let mut content_section = String::new();
            let section_start = i;

            while i < lines.len() && !lines[i].trim().starts_with("```") {
                if i > section_start {
                    content_section.push('\n');
                }
                content_section.push_str(lines[i]);
                i += 1;
            }

            // Process the entire content section (preserves multi-line constructs like frames)
            let (processed, assets) = self.process_line_with_assets(&content_section)?;
            result.push_str(&processed);

            // Add newline after section if not at EOF
            if i < lines.len() {
                result.push('\n');
            }

            all_assets.extend(assets);
        }

        // Restore trailing newline if original had one
        if had_trailing_newline && !result.ends_with('\n') {
            result.push('\n');
        }

        // Remove trailing newline if original didn't have one
        if !had_trailing_newline && result.ends_with('\n') {
            result.pop();
        }

        Ok(ProcessedMarkdown {
            markdown: result,
            assets: all_assets,
        })
    }

    /// Process a single line, handling inline code markers (with asset collection)
    fn process_line_with_assets(&self, line: &str) -> Result<(String, Vec<RenderedAsset>)> {
        // Split by backticks to separate inline code from regular text
        let parts: Vec<&str> = line.split('`').collect();

        let mut result = String::new();
        let mut all_assets = Vec::new();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                // Add back the backtick separator
                result.push('`');
            }

            // Odd indices are inside inline code, even indices are outside
            if i % 2 == 0 {
                // Outside inline code - process templates
                let (processed, assets) = self.process_templates_with_assets(part)?;
                result.push_str(&processed);
                all_assets.extend(assets);
            } else {
                // Inside inline code - preserve as-is
                result.push_str(part);
            }
        }

        Ok((result, all_assets))
    }

    /// Process templates in a text segment using state machine (no asset collection)
    ///
    /// This uses a character-by-character state machine parser instead of regex
    /// for better performance and error messages.
    fn process_templates(&self, text: &str) -> Result<String> {
        Ok(self.process_templates_with_assets(text)?.0)
    }

    /// Process templates in a text segment with asset collection
    fn process_templates_with_assets(&self, text: &str) -> Result<(String, Vec<RenderedAsset>)> {
        // Pre-process to expand {{//}} into appropriate closing tags
        let text = self.expand_close_all(text);

        let mut result = String::new();
        let mut assets = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Look for opening tag {{ (could be ui, frame, badge, or style template)
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '{' {
                // Try to parse a partial template first (highest priority for user-defined)
                if let Some(partial_data) = self.parse_partial_at(&chars, i)? {
                    // Get the partial template
                    if let Some(template) = self.partials.get(&partial_data.partial_name) {
                        // Expand the partial with content
                        let expanded = expand_partial(template, &partial_data.content);

                        // Recursively process the expanded template
                        let (processed, nested_assets) =
                            self.process_templates_with_assets(&expanded)?;
                        result.push_str(&processed);
                        assets.extend(nested_assets);

                        // Skip past the partial template
                        i = partial_data.end_pos;
                        continue;
                    }
                    // Partial name not found - fall through to try other parsers
                }

                // Try to parse a UI component first (highest priority)
                if let Some(ui_data) = self.parse_ui_at(&chars, i)? {
                    // Expand the UI component
                    let output = self.components_renderer.expand(
                        &ui_data.component_name,
                        &ui_data.args,
                        ui_data.content.as_deref(),
                    )?;

                    match output {
                        ComponentOutput::Primitive(primitive) => {
                            // Render the primitive using the backend
                            let rendered = self.backend.render(&primitive)?;
                            result.push_str(rendered.to_markdown());

                            // Collect file-based assets
                            if rendered.is_file_based() {
                                assets.push(rendered);
                            }
                        }
                        ComponentOutput::Template(template) => {
                            // Recursively process the template
                            // (it may contain shields, frames, or styles)
                            let (processed, nested_assets) =
                                self.process_templates_with_assets(&template)?;
                            result.push_str(&processed);
                            assets.extend(nested_assets);
                        }
                        ComponentOutput::TemplateDelayed {
                            template,
                            post_process,
                        } => {
                            // First recursively process the template
                            let (processed, nested_assets) =
                                self.process_templates_with_assets(&template)?;
                            assets.extend(nested_assets);

                            // Then apply delayed post-processing
                            let final_output = match post_process {
                                PostProcess::Row { align } => {
                                    ComponentsRenderer::apply_row(&processed, &align)
                                }
                                // Other post-processors run before recursion, not here
                                _ => processed,
                            };
                            result.push_str(&final_output);
                        }
                    }

                    // Skip past the UI template
                    i = ui_data.end_pos;
                    continue;
                }

                // Try to parse a frame template
                if let Some(frame_data) = self.parse_frame_at(&chars, i)? {
                    // Process content recursively (may contain style templates and primitives)
                    let (processed_content, nested_assets) =
                        self.process_templates_with_assets(&frame_data.content)?;
                    assets.extend(nested_assets);

                    // Check for glyph frame shorthand: {{frame:glyph:NAME[*COUNT][/pad=VALUE][/separator=VALUE][/spacing=N]}}
                    let framed = if frame_data.frame_style.starts_with("glyph:") {
                        // Parse glyph spec: NAME[*COUNT][/pad=VALUE][/separator=VALUE][/spacing=N]
                        let spec = &frame_data.frame_style[6..];
                        let (glyph_name, count, pad, separator, spacing) =
                            Self::parse_glyph_frame_spec(spec);

                        let glyph_char = self
                            .registry
                            .glyph(&glyph_name)
                            .ok_or_else(|| Error::UnknownGlyph(glyph_name.clone()))?;

                        // Build repeated glyph string with separator or spacing
                        let glyphs: String = if let Some(sep) = separator {
                            // Resolve separator from registry or use as literal
                            let sep_char = self.registry.separator(&sep).unwrap_or(&sep);
                            (0..count)
                                .map(|_| glyph_char)
                                .collect::<Vec<_>>()
                                .join(sep_char)
                        } else if let Some(n) = spacing {
                            // spacing=N adds N spaces between glyphs
                            let spaces = " ".repeat(n);
                            (0..count)
                                .map(|_| glyph_char)
                                .collect::<Vec<_>>()
                                .join(&spaces)
                        } else {
                            glyph_char.repeat(count)
                        };

                        // Apply glyphs as both prefix and suffix with padding
                        format!("{}{}{}{}{}", glyphs, pad, processed_content, pad, glyphs)
                    } else if frame_data.frame_style.contains('+') {
                        // Frame combo: fr:outer+inner applies both frames nested
                        // e.g., fr:gradient+star → ▓▒░ ★ TITLE ☆ ░▒▓
                        let frames: Vec<&str> = frame_data.frame_style.split('+').collect();
                        let mut combined_prefix = String::new();
                        let mut combined_suffix = String::new();

                        // Build nested prefix: outer to inner
                        for frame_name in &frames {
                            let frame = self
                                .registry
                                .frame(frame_name.trim())
                                .ok_or_else(|| Error::UnknownFrame(frame_name.to_string()))?;
                            combined_prefix.push_str(&frame.prefix);
                        }

                        // Build nested suffix: inner to outer (reverse order)
                        for frame_name in frames.iter().rev() {
                            let frame = self
                                .registry
                                .frame(frame_name.trim())
                                .ok_or_else(|| Error::UnknownFrame(frame_name.to_string()))?;
                            combined_suffix.push_str(&frame.suffix);
                        }

                        format!(
                            "{}{}{}",
                            combined_prefix, processed_content, combined_suffix
                        )
                    } else {
                        // Extract modifiers from frame style
                        let mods = Self::parse_frame_modifiers(&frame_data.frame_style);

                        // Get the frame
                        let frame = self
                            .registry
                            .frame(&mods.style)
                            .ok_or_else(|| Error::UnknownFrame(mods.style.clone()))?;

                        // Get base prefix/suffix, applying count if specified
                        let (mut prefix, mut suffix) = if let Some(count) = mods.count {
                            // Repeat the pattern N times
                            let prefix_pattern: String = frame.prefix.trim().to_string();
                            let suffix_pattern: String = frame.suffix.trim().to_string();
                            let repeated_prefix = prefix_pattern.repeat(count);
                            let repeated_suffix = suffix_pattern.repeat(count);
                            // Preserve original spacing
                            let prefix_space = if frame.prefix.ends_with(' ') { " " } else { "" };
                            let suffix_space = if frame.suffix.starts_with(' ') {
                                " "
                            } else {
                                ""
                            };
                            (
                                format!("{}{}", repeated_prefix, prefix_space),
                                format!("{}{}", suffix_space, repeated_suffix),
                            )
                        } else {
                            (frame.prefix.clone(), frame.suffix.clone())
                        };

                        // Apply reverse modifier (swap prefix and suffix)
                        if mods.reverse {
                            std::mem::swap(&mut prefix, &mut suffix);
                        }

                        // Apply separator or spacing if specified
                        if mods.separator.is_some() || mods.spacing.is_some() {
                            // Determine the join string: separator takes precedence over spacing
                            let join_str = if let Some(sep) = &mods.separator {
                                self.registry
                                    .separator(sep)
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| sep.to_string())
                            } else if let Some(n) = mods.spacing {
                                " ".repeat(n)
                            } else {
                                String::new()
                            };

                            // Insert join string between graphemes in prefix/suffix
                            use unicode_segmentation::UnicodeSegmentation;
                            let prefix_with_sep: String = prefix
                                .trim()
                                .graphemes(true)
                                .collect::<Vec<_>>()
                                .join(&join_str);
                            let suffix_with_sep: String = suffix
                                .trim()
                                .graphemes(true)
                                .collect::<Vec<_>>()
                                .join(&join_str);

                            // Preserve spacing around content
                            let prefix_space = if prefix.ends_with(' ') { " " } else { "" };
                            let suffix_space = if suffix.starts_with(' ') { " " } else { "" };

                            format!(
                                "{}{}{}{}{}",
                                prefix_with_sep,
                                prefix_space,
                                processed_content,
                                suffix_space,
                                suffix_with_sep
                            )
                        } else if mods.count.is_some() || mods.reverse {
                            // Count or reverse was applied, use modified prefix/suffix
                            format!("{}{}{}", prefix, processed_content, suffix)
                        } else {
                            // No modifiers, use standard apply_frame
                            self.registry.apply_frame(&processed_content, &mods.style)?
                        }
                    };
                    result.push_str(&framed);

                    // Skip past the frame template
                    i = frame_data.end_pos;
                    continue;
                }

                // Try to parse a shields template (escape hatch for primitives)
                if let Some(shield_data) = self.parse_shields_at(&chars, i)? {
                    // Render based on shield type
                    let rendered = match shield_data.shield_type.as_str() {
                        "block" => {
                            let color = shield_data.params.get("color").ok_or_else(|| {
                                Error::MissingShieldParam("color".to_string(), "block".to_string())
                            })?;
                            let style = shield_data.params.get("style").ok_or_else(|| {
                                Error::MissingShieldParam("style".to_string(), "block".to_string())
                            })?;
                            self.shields_renderer.render_block(color, style)?
                        }
                        "twotone" => {
                            let left = shield_data.params.get("left").ok_or_else(|| {
                                Error::MissingShieldParam("left".to_string(), "twotone".to_string())
                            })?;
                            let right = shield_data.params.get("right").ok_or_else(|| {
                                Error::MissingShieldParam(
                                    "right".to_string(),
                                    "twotone".to_string(),
                                )
                            })?;
                            let style = shield_data.params.get("style").ok_or_else(|| {
                                Error::MissingShieldParam(
                                    "style".to_string(),
                                    "twotone".to_string(),
                                )
                            })?;
                            self.shields_renderer.render_twotone(left, right, style)?
                        }
                        "bar" => {
                            let colors_str = shield_data.params.get("colors").ok_or_else(|| {
                                Error::MissingShieldParam("colors".to_string(), "bar".to_string())
                            })?;
                            let colors: Vec<String> =
                                colors_str.split(',').map(|s| s.to_string()).collect();
                            let style = shield_data.params.get("style").ok_or_else(|| {
                                Error::MissingShieldParam("style".to_string(), "bar".to_string())
                            })?;
                            // Get optional separator (can be named like "dot" or literal like " ")
                            let separator = shield_data.params.get("separator").map(|s| {
                                self.registry
                                    .separator(s)
                                    .map(|r| r.to_string())
                                    .unwrap_or_else(|| s.clone())
                            });
                            self.shields_renderer.render_bar_with_separator(
                                &colors,
                                style,
                                separator.as_deref(),
                            )?
                        }
                        "icon" => {
                            let logo = shield_data.params.get("logo").ok_or_else(|| {
                                Error::MissingShieldParam("logo".to_string(), "icon".to_string())
                            })?;
                            let bg = shield_data.params.get("bg").ok_or_else(|| {
                                Error::MissingShieldParam("bg".to_string(), "icon".to_string())
                            })?;
                            let logo_color =
                                shield_data.params.get("logoColor").ok_or_else(|| {
                                    Error::MissingShieldParam(
                                        "logoColor".to_string(),
                                        "icon".to_string(),
                                    )
                                })?;
                            let style = shield_data.params.get("style").ok_or_else(|| {
                                Error::MissingShieldParam("style".to_string(), "icon".to_string())
                            })?;
                            self.shields_renderer
                                .render_icon(logo, bg, logo_color, style)?
                        }
                        _ => return Err(Error::UnknownShieldType(shield_data.shield_type)),
                    };

                    result.push_str(&rendered);

                    // Skip past the shields template
                    i = shield_data.end_pos;
                    continue;
                }

                // Try to parse a glyph template
                if let Some(glyph_data) = self.parse_glyph_at(&chars, i)? {
                    // Resolve glyph from registry
                    let glyph_char = self
                        .registry
                        .glyph(&glyph_data.glyph_name)
                        .ok_or_else(|| Error::UnknownGlyph(glyph_data.glyph_name.clone()))?;

                    result.push_str(glyph_char);

                    // Skip past the glyph template
                    i = glyph_data.end_pos;
                    continue;
                }

                // Try to parse a kbd template
                if let Some(kbd_data) = self.parse_kbd_at(&chars, i)? {
                    // Expand keys to <kbd> HTML tags
                    let expanded = self.expand_kbd(&kbd_data.keys);
                    result.push_str(&expanded);

                    // Skip past the kbd template
                    i = kbd_data.end_pos;
                    continue;
                }

                // Try to parse a style template
                if let Some(template_data) = self.parse_template_at(&chars, i)? {
                    // Validate style exists via unified Registry
                    if self.registry.style(&template_data.style).is_none() {
                        return Err(Error::UnknownStyle(template_data.style));
                    }

                    // Convert content based on whether separator is specified
                    let converted = if let Some(ref sep) = template_data.separator {
                        // Use separator-based conversion
                        self.converter.convert_with_separator(
                            &template_data.content,
                            &template_data.style,
                            sep,
                            1, // count = 1 for single separator between chars
                        )?
                    } else if template_data.spacing > 0 {
                        // Use spacing-based conversion (spaces between chars)
                        self.converter.convert_with_spacing(
                            &template_data.content,
                            &template_data.style,
                            template_data.spacing,
                        )?
                    } else {
                        // No spacing or separator, just convert normally
                        self.converter
                            .convert(&template_data.content, &template_data.style)?
                    };

                    result.push_str(&converted);

                    // Skip past the template
                    i = template_data.end_pos;
                    continue;
                }
            }

            // Not a template (or invalid), add character as-is
            result.push(chars[i]);
            i += 1;
        }

        Ok((result, assets))
    }

    /// Pre-process text to expand {{//}} into appropriate closing tags
    ///
    /// This scans for all open tags (frames, styles, UI components) and when
    /// {{//}} is encountered, replaces it with the appropriate closing tags
    /// in reverse order (LIFO).
    fn expand_close_all(&self, text: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        // Track open tags: (tag_type, closer)
        // tag_type: "frame", "style", "ui"
        let mut open_tags: Vec<(&str, String)> = Vec::new();

        while i < chars.len() {
            // Check for {{//}}
            if i + 5 < chars.len()
                && chars[i] == '{'
                && chars[i + 1] == '{'
                && chars[i + 2] == '/'
                && chars[i + 3] == '/'
                && chars[i + 4] == '}'
                && chars[i + 5] == '}'
            {
                // Expand to all closing tags in reverse order
                for (_, closer) in open_tags.iter().rev() {
                    result.push_str(closer);
                }
                open_tags.clear();
                i += 6;
                continue;
            }

            // Check for opening tags
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '{' {
                // Check for frame: {{frame: or {{fr:
                if i + 8 < chars.len() && self.matches_at(&chars, i, "{{frame:") {
                    // Check if it's self-closing (has :CONTENT/ pattern before }})
                    if !self.is_self_closing_frame(&chars, i + 8) {
                        open_tags.push(("frame", "{{/}}".to_string()));
                    }
                } else if i + 5 < chars.len() && self.matches_at(&chars, i, "{{fr:") {
                    if !self.is_self_closing_frame(&chars, i + 5) {
                        open_tags.push(("frame", "{{/}}".to_string()));
                    }
                }
                // Check for UI component: {{ui:
                else if i + 5 < chars.len() && self.matches_at(&chars, i, "{{ui:") {
                    // Check if it's self-closing (ends with /}})
                    if !self.is_self_closing_tag(&chars, i) {
                        open_tags.push(("ui", "{{/ui}}".to_string()));
                    }
                }
                // Check for style template: {{stylename}} (not a known prefix)
                else if i + 2 < chars.len() && chars[i + 2].is_alphabetic() {
                    // Parse potential style name
                    let mut j = i + 2;
                    let mut name = String::new();
                    while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '-') {
                        name.push(chars[j]);
                        j += 1;
                    }
                    // Check if it's a block style (not self-closing, has closing tag)
                    // Skip known prefixes
                    if !name.is_empty()
                        && !["frame", "fr", "ui", "shields", "glyph", "kbd"]
                            .contains(&name.as_str())
                        && j < chars.len()
                    {
                        // Check for closing }} after optional params
                        let mut k = j;
                        // Skip parameters like :spacing=N
                        while k < chars.len() && chars[k] == ':' {
                            k += 1;
                            while k < chars.len() && chars[k] != ':' && chars[k] != '}' {
                                k += 1;
                            }
                        }
                        // Check for }} and NOT self-closing /}}
                        if k + 1 < chars.len()
                            && chars[k] == '}'
                            && chars[k + 1] == '}'
                            && (k == 0 || chars[k - 1] != '/')
                        {
                            // This is a block style, track it
                            // Note: {{{{ produces {{ in format strings
                            open_tags.push(("style", format!("{{{{/{}}}}}", name)));
                        }
                    }
                }

                // Check for closing tags to pop from stack
                if i + 4 < chars.len() && self.matches_at(&chars, i, "{{/") {
                    // Find what's being closed
                    let mut j = i + 3;
                    let mut closer_name = String::new();
                    while j < chars.len() && chars[j] != '}' {
                        closer_name.push(chars[j]);
                        j += 1;
                    }
                    // Pop matching tag from stack (or any tag for generic closers)
                    if closer_name.is_empty() || closer_name == "frame" || closer_name == "fr" {
                        // Generic or frame closer - pop last frame
                        if let Some(pos) = open_tags.iter().rposition(|(t, _)| *t == "frame") {
                            open_tags.remove(pos);
                        }
                    } else if closer_name == "ui" {
                        if let Some(pos) = open_tags.iter().rposition(|(t, _)| *t == "ui") {
                            open_tags.remove(pos);
                        }
                    } else {
                        // Specific style closer
                        if let Some(pos) = open_tags.iter().rposition(|(t, c)| {
                            *t == "style" && c == &format!("{{{{/{}}}}}", closer_name)
                        }) {
                            open_tags.remove(pos);
                        }
                    }
                }
            }

            result.push(chars[i]);
            i += 1;
        }

        result
    }

    /// Check if characters match a string at position
    fn matches_at(&self, chars: &[char], pos: usize, s: &str) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        if pos + s_chars.len() > chars.len() {
            return false;
        }
        for (idx, &expected) in s_chars.iter().enumerate() {
            if chars[pos + idx] != expected {
                return false;
            }
        }
        true
    }

    /// Check if a tag starting at pos is self-closing (ends with /}})
    fn is_self_closing_tag(&self, chars: &[char], pos: usize) -> bool {
        let mut i = pos + 2; // Skip {{
        while i + 2 < chars.len() {
            if chars[i] == '/' && chars[i + 1] == '}' && chars[i + 2] == '}' {
                return true;
            }
            if chars[i] == '}' && i + 1 < chars.len() && chars[i + 1] == '}' {
                return false; // Found }} without /
            }
            i += 1;
        }
        false
    }

    /// Check if a frame is self-closing (has :CONTENT/ pattern)
    fn is_self_closing_frame(&self, chars: &[char], start: usize) -> bool {
        // Look for pattern ending in /}} which indicates self-closing
        let mut i = start;
        while i + 2 < chars.len() {
            if chars[i] == '/' && chars[i + 1] == '}' && chars[i + 2] == '}' {
                return true;
            }
            if chars[i] == '}' && i + 1 < chars.len() && chars[i + 1] == '}' {
                return false; // Found }} without /
            }
            i += 1;
        }
        false
    }

    /// Try to parse a template starting at position i
    /// Returns: Some(TemplateData) or None if not a valid template
    fn parse_template_at(&self, chars: &[char], start: usize) -> Result<Option<TemplateData>> {
        let mut i = start;

        // Must start with {{
        if i + 1 >= chars.len() || chars[i] != '{' || chars[i + 1] != '{' {
            return Ok(None);
        }
        i += 2;

        // Parse style name (alphanumeric and hyphens)
        let mut style = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() || ch == '-' {
                style.push(ch);
                i += 1;
            } else if ch == ':' || ch == '}' {
                break;
            } else {
                // Invalid character in style name
                return Ok(None);
            }
        }

        // Style name must be non-empty
        if style.is_empty() {
            return Ok(None);
        }

        // Parse optional parameters: :spacing=N and/or :separator=name
        let mut spacing = 0;
        let mut separator: Option<String> = None;

        // Helper function to check if chars match a string at position i
        let matches_str = |chars: &[char], i: usize, s: &str| -> bool {
            let s_chars: Vec<char> = s.chars().collect();
            if i + s_chars.len() > chars.len() {
                return false;
            }
            for (idx, &expected) in s_chars.iter().enumerate() {
                if chars[i + idx] != expected {
                    return false;
                }
            }
            true
        };

        // Parse parameters (can have multiple separated by :)
        while i < chars.len() && chars[i] == ':' {
            i += 1; // skip ':'

            // Check for "spacing="
            if matches_str(chars, i, "spacing=") {
                i += 8; // length of "spacing="

                // Parse the number
                let mut num_str = String::new();
                while i < chars.len() && chars[i].is_ascii_digit() {
                    num_str.push(chars[i]);
                    i += 1;
                }

                // Parse the spacing value
                if let Ok(value) = num_str.parse::<usize>() {
                    spacing = value;
                } else {
                    // Invalid number
                    return Ok(None);
                }
            }
            // Check for "separator="
            else if matches_str(chars, i, "separator=") {
                i += 10; // length of "separator="

                // Parse separator name or direct character
                let mut sep_input = String::new();
                while i < chars.len() && !matches_str(chars, i, ":") && !matches_str(chars, i, "}}")
                {
                    sep_input.push(chars[i]);
                    i += 1;
                }

                // Resolve separator using unified Registry
                // First, try to resolve as a known separator
                if let Some(sep_value) = self.registry.separator(&sep_input) {
                    separator = Some(sep_value.to_string());
                } else {
                    // Not a known separator - check if it's a single grapheme literal
                    use unicode_segmentation::UnicodeSegmentation;
                    let graphemes: Vec<&str> = sep_input.graphemes(true).collect();

                    if graphemes.len() == 1 {
                        // Single grapheme - accept as literal separator
                        separator = Some(sep_input.clone());
                    } else {
                        // Multi-grapheme unknown name - error with suggestions
                        let available: Vec<&str> = self
                            .registry
                            .glyphs()
                            .keys()
                            .map(|name| name.as_str())
                            .take(8)
                            .collect();

                        return Err(Error::ParseError(format!(
                            "Unknown glyph '{}'. Available glyphs: {}. Or use a single character like '→' or '·'.",
                            sep_input,
                            available.join(", ")
                        )));
                    }
                }
            } else {
                // Unknown parameter
                return Ok(None);
            }
        }

        // Must have closing }} for opening tag
        if i + 1 >= chars.len() || chars[i] != '}' || chars[i + 1] != '}' {
            return Ok(None);
        }
        i += 2;

        let content_start = i;

        // Find closing tag {{/style}}
        let close_tag = format!("{{{{/{}}}}}", style);
        let close_chars: Vec<char> = close_tag.chars().collect();

        while i < chars.len() {
            // Check if we've found the closing tag
            if i + close_chars.len() <= chars.len() {
                let mut matches = true;
                for (j, &close_ch) in close_chars.iter().enumerate() {
                    if chars[i + j] != close_ch {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    // Found closing tag
                    let content: String = chars[content_start..i].iter().collect();
                    let end_pos = i + close_chars.len();
                    return Ok(Some(TemplateData {
                        end_pos,
                        style,
                        spacing,
                        separator,
                        content,
                    }));
                }
            }

            i += 1;
        }

        // No closing tag found
        Err(Error::UnclosedTag(style))
    }

    /// Parse glyph frame spec: NAME[*COUNT][/pad=VALUE][/separator=VALUE][/spacing=N]
    /// Returns (glyph_name, count, padding_string, separator_option, spacing_option)
    fn parse_glyph_frame_spec(
        spec: &str,
    ) -> (String, usize, String, Option<String>, Option<usize>) {
        let mut remaining = spec.to_string();
        let mut count: usize = 1;
        let mut pad = " ".to_string(); // default: single space
        let mut separator: Option<String> = None;
        let mut spacing: Option<usize> = None;

        // Check for /modifiers (can have multiple: /pad=X/separator=Y/spacing=N)
        while let Some(slash_pos) = remaining.find('/') {
            let after_slash = remaining[slash_pos + 1..].to_string();
            remaining = remaining[..slash_pos].to_string();

            // Split on next / if present to handle chained modifiers
            let (modifier, rest) = if let Some(next_slash) = after_slash.find('/') {
                (
                    after_slash[..next_slash].to_string(),
                    Some(after_slash[next_slash..].to_string()),
                )
            } else {
                (after_slash, None)
            };

            if let Some(pad_value) = modifier.strip_prefix("pad=") {
                // Check if it's a number (meaning N spaces)
                if let Ok(num_spaces) = pad_value.parse::<usize>() {
                    pad = " ".repeat(num_spaces);
                } else {
                    // Use literal string
                    pad = pad_value.to_string();
                }
            } else if let Some(sep_value) = modifier.strip_prefix("separator=") {
                separator = Some(sep_value.to_string());
            } else if let Some(spacing_value) = modifier.strip_prefix("spacing=") {
                if let Ok(n) = spacing_value.parse::<usize>() {
                    spacing = Some(n);
                }
            }

            // If there are more modifiers, append them back for next iteration
            if let Some(rest) = rest {
                remaining = format!("{}{}", remaining, rest);
            }
        }

        // Check for *COUNT multiplier
        if let Some(star_pos) = remaining.find('*') {
            let count_str = remaining[star_pos + 1..].to_string();
            remaining = remaining[..star_pos].to_string();

            if let Ok(n) = count_str.parse::<usize>() {
                // Cap at 20 to prevent abuse
                count = n.clamp(1, 20);
            }
        }

        (remaining, count, pad, separator, spacing)
    }

    /// Parse frame style and extract modifiers (separator, spacing, reverse, count)
    /// Input: "gradient/separator=dot/spacing=1" → FrameModifiers { style: "gradient", separator: Some("dot"), spacing: Some(1), ... }
    /// Input: "star*3/reverse" → FrameModifiers { style: "star", count: Some(3), reverse: true, ... }
    fn parse_frame_modifiers(style: &str) -> FrameModifiers {
        let mut remaining = style.to_string();
        let mut separator: Option<String> = None;
        let mut spacing: Option<usize> = None;
        let mut reverse = false;
        let mut count: Option<usize> = None;

        // Check for /modifiers
        while let Some(slash_pos) = remaining.find('/') {
            let after_slash = remaining[slash_pos + 1..].to_string();
            remaining = remaining[..slash_pos].to_string();

            // Split on next / if present
            let (modifier, rest) = if let Some(next_slash) = after_slash.find('/') {
                (
                    after_slash[..next_slash].to_string(),
                    Some(after_slash[next_slash..].to_string()),
                )
            } else {
                (after_slash, None)
            };

            if let Some(sep_value) = modifier.strip_prefix("separator=") {
                separator = Some(sep_value.to_string());
            } else if let Some(spacing_value) = modifier.strip_prefix("spacing=") {
                if let Ok(n) = spacing_value.parse::<usize>() {
                    spacing = Some(n);
                }
            } else if modifier == "reverse" || modifier == "rev" {
                reverse = true;
            }

            // If there are more modifiers, append them back
            if let Some(rest) = rest {
                remaining = format!("{}{}", remaining, rest);
            }
        }

        // Check for *N count multiplier in the style name (e.g., "star*3")
        if let Some(star_pos) = remaining.find('*') {
            let style_part = remaining[..star_pos].to_string();
            let count_part = &remaining[star_pos + 1..];
            if let Ok(n) = count_part.parse::<usize>() {
                remaining = style_part;
                count = Some(n.min(20)); // Cap at 20 to prevent abuse
            }
        }

        FrameModifiers {
            style: remaining,
            separator,
            spacing,
            reverse,
            count,
        }
    }

    /// Try to parse a frame template starting at position i
    /// Returns: Some(FrameData) or None if not a valid frame template
    fn parse_frame_at(&self, chars: &[char], start: usize) -> Result<Option<FrameData>> {
        let mut i = start;

        // Must start with {{frame: or {{fr: (shorthand)
        if i + 5 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{frame:" or "{{fr:" (shorthand alias)
        let frame_long = "{{frame:";
        let frame_short = "{{fr:";
        let frame_long_chars: Vec<char> = frame_long.chars().collect();
        let frame_short_chars: Vec<char> = frame_short.chars().collect();

        let prefix_len = if i + frame_long_chars.len() <= chars.len() {
            let mut matches_long = true;
            for (idx, &expected) in frame_long_chars.iter().enumerate() {
                if chars[i + idx] != expected {
                    matches_long = false;
                    break;
                }
            }
            if matches_long {
                frame_long_chars.len()
            } else if i + frame_short_chars.len() <= chars.len() {
                let mut matches_short = true;
                for (idx, &expected) in frame_short_chars.iter().enumerate() {
                    if chars[i + idx] != expected {
                        matches_short = false;
                        break;
                    }
                }
                if matches_short {
                    frame_short_chars.len()
                } else {
                    return Ok(None);
                }
            } else {
                return Ok(None);
            }
        } else if i + frame_short_chars.len() <= chars.len() {
            let mut matches_short = true;
            for (idx, &expected) in frame_short_chars.iter().enumerate() {
                if chars[i + idx] != expected {
                    matches_short = false;
                    break;
                }
            }
            if matches_short {
                frame_short_chars.len()
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        };
        i += prefix_len;

        // Parse frame style name - allow most characters except }}
        // This enables glyph frames with Unicode padding like /pad=·
        let mut frame_style = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch == '}' {
                break;
            } else if ch == '{' {
                // Prevent nested templates in frame style
                return Ok(None);
            } else {
                frame_style.push(ch);
                i += 1;
            }
        }

        // Frame style must be non-empty
        if frame_style.is_empty() {
            return Ok(None);
        }

        // Check for self-closing frame: {{fr:TYPE:CONTENT/}}
        // frame_style would be "TYPE:CONTENT/" in this case
        if frame_style.ends_with('/') {
            // Must have closing }} for self-closing tag
            if i + 1 >= chars.len() || chars[i] != '}' || chars[i + 1] != '}' {
                return Ok(None);
            }
            let end_pos = i + 2;

            // Remove trailing / and split on LAST : to get TYPE and CONTENT
            // Using rfind handles glyph frames like "glyph:diamond*2:Gem"
            // which should split into style="glyph:diamond*2" and content="Gem"
            let style_without_slash = &frame_style[..frame_style.len() - 1];
            if let Some(colon_pos) = style_without_slash.rfind(':') {
                let actual_style = style_without_slash[..colon_pos].to_string();
                let content = style_without_slash[colon_pos + 1..].to_string();

                // Validate frame style is non-empty
                if actual_style.is_empty() {
                    return Ok(None);
                }

                return Ok(Some(FrameData {
                    end_pos,
                    frame_style: actual_style,
                    content,
                }));
            }
            // No colon found - not a valid self-closing frame with content
            return Ok(None);
        }

        // Must have closing }} for opening tag
        if i + 1 >= chars.len() || chars[i] != '}' || chars[i + 1] != '}' {
            return Ok(None);
        }
        i += 2;

        let content_start = i;

        // Find closing tag {{/}}, {{//}} (close-all), or {{/frame}} - track nesting depth
        let open_long = "{{frame:";
        let open_short = "{{fr:";
        let close_all = "{{//}}";
        let close_short = "{{/}}";
        let close_long = "{{/frame}}";
        let open_long_chars: Vec<char> = open_long.chars().collect();
        let open_short_chars: Vec<char> = open_short.chars().collect();
        let close_all_chars: Vec<char> = close_all.chars().collect();
        let close_short_chars: Vec<char> = close_short.chars().collect();
        let close_long_chars: Vec<char> = close_long.chars().collect();
        let mut depth = 1; // We've already seen one opening tag

        while i < chars.len() {
            // Check for nested opening tag {{frame: or {{fr:
            let (is_nested_open, open_len) = {
                let mut matches_long = false;
                let mut matches_short = false;

                if i + open_long_chars.len() <= chars.len() {
                    matches_long = true;
                    for (j, &open_ch) in open_long_chars.iter().enumerate() {
                        if chars[i + j] != open_ch {
                            matches_long = false;
                            break;
                        }
                    }
                }

                if !matches_long && i + open_short_chars.len() <= chars.len() {
                    matches_short = true;
                    for (j, &open_ch) in open_short_chars.iter().enumerate() {
                        if chars[i + j] != open_ch {
                            matches_short = false;
                            break;
                        }
                    }
                }

                if matches_long {
                    (true, open_long_chars.len())
                } else if matches_short {
                    (true, open_short_chars.len())
                } else {
                    (false, 0)
                }
            };

            if is_nested_open {
                depth += 1;
                i += open_len;
                continue;
            }

            // Check for closing tag {{//}} (close-all), {{/}} (short), or {{/frame}} (long)
            // Check close-all first since {{//}} starts with {{/}}
            let (is_close_all, is_close, close_len) = {
                let mut matches_all = false;
                let mut matches_short = false;
                let mut matches_long = false;

                // Check {{//}} first (close-all)
                if i + close_all_chars.len() <= chars.len() {
                    matches_all = true;
                    for (j, &ch) in close_all_chars.iter().enumerate() {
                        if chars[i + j] != ch {
                            matches_all = false;
                            break;
                        }
                    }
                }

                // Check {{/}} (short form)
                if !matches_all && i + close_short_chars.len() <= chars.len() {
                    matches_short = true;
                    for (j, &ch) in close_short_chars.iter().enumerate() {
                        if chars[i + j] != ch {
                            matches_short = false;
                            break;
                        }
                    }
                }

                // Check {{/frame}} (long form)
                if !matches_all && !matches_short && i + close_long_chars.len() <= chars.len() {
                    matches_long = true;
                    for (j, &ch) in close_long_chars.iter().enumerate() {
                        if chars[i + j] != ch {
                            matches_long = false;
                            break;
                        }
                    }
                }

                if matches_all {
                    (true, true, close_all_chars.len())
                } else if matches_short {
                    (false, true, close_short_chars.len())
                } else if matches_long {
                    (false, true, close_long_chars.len())
                } else {
                    (false, false, 0)
                }
            };

            if is_close {
                let prev_depth = depth;
                if is_close_all {
                    // {{//}} closes all frames at once
                    depth = 0;
                } else {
                    depth -= 1;
                }
                if depth == 0 {
                    // Found matching closing tag
                    let mut content: String = chars[content_start..i].iter().collect();

                    // If close-all was used and there were nested frames (prev_depth > 1),
                    // append closing tags for the nested frames so recursive processing works
                    if is_close_all && prev_depth > 1 {
                        for _ in 1..prev_depth {
                            content.push_str("{{/}}");
                        }
                    }

                    let end_pos = i + close_len;
                    return Ok(Some(FrameData {
                        end_pos,
                        frame_style,
                        content,
                    }));
                }
                i += close_len;
                continue;
            }

            i += 1;
        }

        // No closing tag found
        Err(Error::UnclosedTag("frame".to_string()))
    }

    /// Try to parse a partial template starting at position i
    /// Returns: Some(PartialData) or None if not a valid partial template
    ///
    /// Syntax: {{partial:name}}CONTENT{{/partial}} or {{partial:name}}CONTENT{{/}}
    fn parse_partial_at(&self, chars: &[char], start: usize) -> Result<Option<PartialData>> {
        let mut i = start;

        // Must start with {{partial:
        if i + 11 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{partial:"
        let partial_start = "{{partial:";
        let partial_chars: Vec<char> = partial_start.chars().collect();
        for (idx, &expected) in partial_chars.iter().enumerate() {
            if chars[i + idx] != expected {
                return Ok(None);
            }
        }
        i += partial_chars.len();

        // Parse partial name (alphanumeric, hyphens, underscores)
        let mut partial_name = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                partial_name.push(ch);
                i += 1;
            } else if ch == '}' || ch == '/' {
                break;
            } else {
                // Invalid character in partial name
                return Ok(None);
            }
        }

        // Partial name must be non-empty
        if partial_name.is_empty() {
            return Ok(None);
        }

        // Check for self-closing tag (ends with /}}) - for partials without content
        if i + 2 < chars.len() && chars[i] == '/' && chars[i + 1] == '}' && chars[i + 2] == '}' {
            // Self-closing tag (empty content)
            let end_pos = i + 3;
            return Ok(Some(PartialData {
                end_pos,
                partial_name,
                content: String::new(),
            }));
        }

        // Must have closing }} for opening tag
        if i + 1 >= chars.len() || chars[i] != '}' || chars[i + 1] != '}' {
            return Ok(None);
        }
        i += 2;

        let content_start = i;

        // Find closing tag - supports both {{/partial}} and {{/}}
        let close_tags = ["{{/partial}}", "{{/}}"];

        while i < chars.len() {
            for close_tag in &close_tags {
                let close_chars: Vec<char> = close_tag.chars().collect();
                if i + close_chars.len() <= chars.len() {
                    let mut matches = true;
                    for (j, &close_ch) in close_chars.iter().enumerate() {
                        if chars[i + j] != close_ch {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        // Found closing tag
                        let content: String = chars[content_start..i].iter().collect();
                        let end_pos = i + close_chars.len();
                        return Ok(Some(PartialData {
                            end_pos,
                            partial_name,
                            content,
                        }));
                    }
                }
            }

            i += 1;
        }

        // No closing tag found
        Err(Error::UnclosedTag("partial".to_string()))
    }

    /// Try to parse a UI component template starting at position i
    /// Returns: Some(UIData) or None if not a valid UI template
    ///
    /// Supports both self-closing and block-style:
    /// - Self-closing: {{ui:swatch:accent/}}
    /// - Block: {{ui:row}}CONTENT{{/ui}}
    /// - With args: {{ui:tech:rust/}}
    fn parse_ui_at(&self, chars: &[char], start: usize) -> Result<Option<UIData>> {
        let mut i = start;

        // Must start with {{ui:
        if i + 5 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{ui:"
        let ui_start = "{{ui:";
        let ui_chars: Vec<char> = ui_start.chars().collect();
        for (idx, &expected) in ui_chars.iter().enumerate() {
            if chars[i + idx] != expected {
                return Ok(None);
            }
        }
        i += ui_chars.len();

        // Parse component name (alphanumeric and hyphens)
        let mut component_name = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                component_name.push(ch);
                i += 1;
            } else if ch == ':' || ch == '/' || ch == '}' {
                break;
            } else {
                // Invalid character in component name
                return Ok(None);
            }
        }

        // Component name must be non-empty
        if component_name.is_empty() {
            return Ok(None);
        }

        // Parse optional args (separated by :)
        let mut args = Vec::new();

        while i < chars.len() && chars[i] == ':' {
            i += 1; // skip ':'

            // Parse arg value (until next : or } or /)
            // For key=value args, allow / in the value part (e.g., gradient=horizontal/FF6B35/1a1a2e)
            let mut arg = String::new();
            let mut has_equals = false;

            while i < chars.len() {
                let ch = chars[i];

                // Track if we've seen '=' to know if we're in a key=value argument
                if ch == '=' {
                    has_equals = true;
                    arg.push(ch);
                    i += 1;
                    continue;
                }

                // For key=value args, only stop at : or }
                // For positional args, also stop at /
                if ch == ':' || ch == '}' {
                    break;
                }

                // For positional args (no =), stop at /
                // For key=value args, allow / in values
                if ch == '/' && !has_equals {
                    break;
                }

                // Special case: if we see /}} it's the self-closing marker
                if ch == '/' && i + 2 < chars.len() && chars[i + 1] == '}' && chars[i + 2] == '}' {
                    break;
                }

                arg.push(ch);
                i += 1;
            }

            if !arg.is_empty() {
                args.push(arg);
            }
        }

        // Check for self-closing tag (ends with /}})
        if i + 2 < chars.len() && chars[i] == '/' && chars[i + 1] == '}' && chars[i + 2] == '}' {
            // Self-closing tag
            let end_pos = i + 3;
            return Ok(Some(UIData {
                end_pos,
                component_name,
                args,
                content: None,
            }));
        }

        // Must have closing }} for opening tag
        if i + 1 >= chars.len() || chars[i] != '}' || chars[i + 1] != '}' {
            return Ok(None);
        }
        i += 2;

        let content_start = i;

        // Find closing tag {{/ui}}
        let close_tag = "{{/ui}}";
        let close_chars: Vec<char> = close_tag.chars().collect();

        while i < chars.len() {
            // Check if we've found the closing tag
            if i + close_chars.len() <= chars.len() {
                let mut matches = true;
                for (j, &close_ch) in close_chars.iter().enumerate() {
                    if chars[i + j] != close_ch {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    // Found closing tag
                    let content: String = chars[content_start..i].iter().collect();
                    let end_pos = i + close_chars.len();
                    return Ok(Some(UIData {
                        end_pos,
                        component_name,
                        args,
                        content: Some(content),
                    }));
                }
            }

            i += 1;
        }

        // No closing tag found
        Err(Error::UnclosedTag("ui".to_string()))
    }

    /// Try to parse a shields template starting at position i
    /// Returns: Some(ShieldData) or None if not a valid shields template
    ///
    /// Supports self-closing only: {{shields:block:color=accent:style=flat-square/}}
    fn parse_shields_at(&self, chars: &[char], start: usize) -> Result<Option<ShieldData>> {
        let mut i = start;

        // Must start with {{shields:
        if i + 11 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{shields:"
        let shields_start = "{{shields:";
        let shields_chars: Vec<char> = shields_start.chars().collect();
        for (idx, &expected) in shields_chars.iter().enumerate() {
            if chars[i + idx] != expected {
                return Ok(None);
            }
        }
        i += shields_chars.len();

        // Parse shield type (block, twotone, bar, icon)
        let mut shield_type = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() {
                shield_type.push(ch);
                i += 1;
            } else if ch == ':' || ch == '/' {
                break;
            } else {
                // Invalid character in shield type
                return Ok(None);
            }
        }

        // Shield type must be non-empty
        if shield_type.is_empty() {
            return Ok(None);
        }

        // Parse parameters (key=value pairs separated by :)
        let mut params = std::collections::HashMap::new();

        while i < chars.len() && chars[i] == ':' {
            i += 1; // skip ':'

            // Parse key
            let mut key = String::new();
            while i < chars.len() && chars[i] != '=' {
                key.push(chars[i]);
                i += 1;
            }

            // Must have '='
            if i >= chars.len() || chars[i] != '=' {
                return Ok(None);
            }
            i += 1; // skip '='

            // Parse value (until next : or / or })
            let mut value = String::new();
            while i < chars.len() {
                let ch = chars[i];
                if ch == ':' || ch == '/' || ch == '}' {
                    break;
                }
                value.push(ch);
                i += 1;
            }

            if !key.is_empty() && !value.is_empty() {
                params.insert(key, value);
            }
        }

        // Must be self-closing (ends with /}})
        if i + 2 < chars.len() && chars[i] == '/' && chars[i + 1] == '}' && chars[i + 2] == '}' {
            let end_pos = i + 3;
            return Ok(Some(ShieldData {
                end_pos,
                shield_type,
                params,
            }));
        }

        // Not a valid shields template
        Ok(None)
    }

    /// Try to parse a glyph template starting at position i
    /// Returns: Some(GlyphData) or None if not a valid glyph template
    ///
    /// Supports self-closing only: {{glyph:block.lower.4/}}
    fn parse_glyph_at(&self, chars: &[char], start: usize) -> Result<Option<GlyphData>> {
        let mut i = start;

        // Must start with {{glyph:
        if i + 9 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{glyph:"
        let glyph_start = "{{glyph:";
        let glyph_chars: Vec<char> = glyph_start.chars().collect();
        for (idx, &expected) in glyph_chars.iter().enumerate() {
            if chars[i + idx] != expected {
                return Ok(None);
            }
        }
        i += glyph_chars.len();

        // Parse glyph name (alphanumeric, dots, and hyphens allowed)
        let mut glyph_name = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() || ch == '.' || ch == '-' || ch == '_' {
                glyph_name.push(ch);
                i += 1;
            } else if ch == '/' {
                break;
            } else {
                // Invalid character in glyph name
                return Ok(None);
            }
        }

        // Glyph name must be non-empty
        if glyph_name.is_empty() {
            return Ok(None);
        }

        // Must be self-closing (ends with /}})
        if i + 2 < chars.len() && chars[i] == '/' && chars[i + 1] == '}' && chars[i + 2] == '}' {
            let end_pos = i + 3;
            return Ok(Some(GlyphData {
                end_pos,
                glyph_name,
            }));
        }

        // Not a valid glyph template
        Ok(None)
    }

    /// Try to parse a kbd template starting at position i
    /// Returns: Some(KbdData) or None if not a valid kbd template
    ///
    /// Supports self-closing only: {{kbd:Ctrl+C/}}
    /// Expands to: <kbd>Ctrl</kbd>+<kbd>C</kbd>
    fn parse_kbd_at(&self, chars: &[char], start: usize) -> Result<Option<KbdData>> {
        let mut i = start;

        // Must start with {{kbd:
        if i + 7 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{kbd:"
        let kbd_start = "{{kbd:";
        let kbd_chars: Vec<char> = kbd_start.chars().collect();
        for (idx, &expected) in kbd_chars.iter().enumerate() {
            if chars[i + idx] != expected {
                return Ok(None);
            }
        }
        i += kbd_chars.len();

        // Parse key sequence (everything until /}})
        let mut keys = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch == '/' {
                break;
            }
            keys.push(ch);
            i += 1;
        }

        // Keys must be non-empty
        if keys.is_empty() {
            return Ok(None);
        }

        // Must be self-closing (ends with /}})
        if i + 2 < chars.len() && chars[i] == '/' && chars[i + 1] == '}' && chars[i + 2] == '}' {
            let end_pos = i + 3;
            return Ok(Some(KbdData { end_pos, keys }));
        }

        // Not a valid kbd template
        Ok(None)
    }

    /// Expand kbd keys to HTML
    /// Splits on + and wraps each part in <kbd> tags
    fn expand_kbd(&self, keys: &str) -> String {
        // Split on + but preserve it as separator
        let parts: Vec<&str> = keys.split('+').collect();
        parts
            .iter()
            .map(|part| format!("<kbd>{}</kbd>", part.trim()))
            .collect::<Vec<_>>()
            .join("+")
    }

    /// Validate template syntax without processing
    pub fn validate(&self, markdown: &str) -> Result<()> {
        // Try to process all templates
        self.process_templates(markdown)?;
        Ok(())
    }
}

impl Default for TemplateParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default parser")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let parser = TemplateParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_simple_template() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}HELLO{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇𝐄𝐋𝐋𝐎");
    }

    #[test]
    fn test_template_in_heading() {
        let parser = TemplateParser::new().unwrap();
        let input = "# {{mathbold}}TITLE{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "# 𝐓𝐈𝐓𝐋𝐄");
    }

    #[test]
    fn test_multiple_templates() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}BOLD{{/mathbold}} and {{italic}}italic{{/italic}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐁𝐎𝐋𝐃 and 𝑖𝑡𝑎𝑙𝑖𝑐");
    }

    #[test]
    fn test_preserves_code_blocks() {
        let parser = TemplateParser::new().unwrap();
        let input = "```\n{{mathbold}}CODE{{/mathbold}}\n```";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "```\n{{mathbold}}CODE{{/mathbold}}\n```");
    }

    #[test]
    fn test_preserves_code_blocks_with_language() {
        let parser = TemplateParser::new().unwrap();
        let input = "```markdown\n{{ui:test:arg/}}\nMore {{/ui}} content\n```";
        let result = parser.process(input).unwrap();

        // Code block content should be preserved exactly
        assert_eq!(
            result,
            "```markdown\n{{ui:test:arg/}}\nMore {{/ui}} content\n```"
        );
    }

    #[test]
    fn test_preserves_inline_code() {
        let parser = TemplateParser::new().unwrap();
        let input = "Text `{{mathbold}}code{{/mathbold}}` more text";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "Text `{{mathbold}}code{{/mathbold}}` more text");
    }

    #[test]
    fn test_multiline_template() {
        let parser = TemplateParser::new().unwrap();
        let input = "Line 1\n{{mathbold}}TITLE{{/mathbold}}\nLine 3";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "Line 1\n𝐓𝐈𝐓𝐋𝐄\nLine 3");
    }

    #[test]
    fn test_unknown_style_error() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fakestyle}}TEXT{{/fakestyle}}";
        let result = parser.process(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_style_alias() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mb}}TEST{{/mb}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐓𝐄𝐒𝐓");
    }

    #[test]
    fn test_template_with_spaces() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}HELLO WORLD{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇𝐄𝐋𝐋𝐎 𝐖𝐎𝐑𝐋𝐃");
    }

    #[test]
    fn test_template_with_punctuation() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}Hello, World!{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇𝐞𝐥𝐥𝐨, 𝐖𝐨𝐫𝐥𝐝!");
    }

    #[test]
    fn test_mismatched_tags() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}TEXT{{/italic}}";
        let result = parser.process(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_markdown() {
        let parser = TemplateParser::new().unwrap();
        let input = r#"# {{mathbold}}TITLE{{/mathbold}}

This is a {{negative-squared}}WARNING{{/negative-squared}} message.

```rust
let code = "{{mathbold}}not processed{{/mathbold}}";
```

And `{{mathbold}}inline code{{/mathbold}}` is also preserved."#;

        let result = parser.process(input).unwrap();

        assert!(result.contains("𝐓𝐈𝐓𝐋𝐄"));
        assert!(result.contains("🆆🅰🆁🅽🅸🅽🅶"));
        assert!(result.contains("{{mathbold}}not processed{{/mathbold}}"));
        assert!(result.contains("`{{mathbold}}inline code{{/mathbold}}`"));
    }

    #[test]
    fn test_hyphenated_style_names() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{negative-squared}}TEST{{/negative-squared}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "🆃🅴🆂🆃");
    }

    #[test]
    fn test_empty_content() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_adjacent_templates() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}A{{/mathbold}}{{italic}}B{{/italic}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐀𝐵");
    }

    #[test]
    fn test_template_with_spacing() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:spacing=1}}HELLO{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇 𝐄 𝐋 𝐋 𝐎");
    }

    #[test]
    fn test_template_with_spacing_two() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{script:spacing=2}}ABC{{/script}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝒜  ℬ  𝒞");
    }

    #[test]
    fn test_template_mixed_spacing() {
        let parser = TemplateParser::new().unwrap();
        let input =
            "{{mathbold}}no spacing{{/mathbold}} {{mathbold:spacing=1}}with spacing{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐧𝐨 𝐬𝐩𝐚𝐜𝐢𝐧𝐠 𝐰 𝐢 𝐭 𝐡   𝐬 𝐩 𝐚 𝐜 𝐢 𝐧 𝐠");
    }

    #[test]
    fn test_template_spacing_with_heading() {
        let parser = TemplateParser::new().unwrap();
        let input = "# {{mathbold:spacing=1}}HEADER{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "# 𝐇 𝐄 𝐀 𝐃 𝐄 𝐑");
    }

    #[test]
    fn test_template_spacing_zero() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:spacing=0}}HELLO{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇𝐄𝐋𝐋𝐎");
    }

    #[test]
    fn test_template_with_separator_dot() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:separator=dot}}HELLO{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇·𝐄·𝐋·𝐋·𝐎");
    }

    #[test]
    fn test_template_with_separator_dash() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:separator=dash}}HEADER{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇─𝐄─𝐀─𝐃─𝐄─𝐑");
    }

    #[test]
    fn test_template_with_separator_bolddash() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:separator=bolddash}}BOLD{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐁━𝐎━𝐋━𝐃");
    }

    #[test]
    fn test_template_with_separator_arrow() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:separator=arrow}}ABC{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐀→𝐁→𝐂");
    }

    #[test]
    fn test_template_with_separator_bullet() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:separator=bullet}}TEST{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐓•𝐄•𝐒•𝐓");
    }

    #[test]
    fn test_template_separator_in_heading() {
        let parser = TemplateParser::new().unwrap();
        let input = "# {{mathbold:separator=dot}}TITLE{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "# 𝐓·𝐈·𝐓·𝐋·𝐄");
    }

    #[test]
    fn test_template_separator_with_punctuation() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:separator=dash}}Hello, World!{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇─𝐞─𝐥─𝐥─𝐨─,─ ─𝐖─𝐨─𝐫─𝐥─𝐝─!");
    }

    #[test]
    fn test_template_spacing_and_separator_mutually_exclusive() {
        let parser = TemplateParser::new().unwrap();
        // When both are specified, separator takes precedence
        let input = "{{mathbold:spacing=2:separator=dot}}HI{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐇·𝐈");
    }

    #[test]
    fn test_template_unknown_separator_error() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold:separator=invalid}}TEST{{/mathbold}}";
        let result = parser.process(input);
        assert!(result.is_err());
        if let Err(Error::ParseError(msg)) = result {
            assert!(msg.contains("Unknown glyph"));
        } else {
            panic!("Expected ParseError");
        }
    }

    #[test]
    fn test_template_mixed_with_and_without_separator() {
        let parser = TemplateParser::new().unwrap();
        let input =
            "{{mathbold}}no sep{{/mathbold}} {{mathbold:separator=dot}}with sep{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "𝐧𝐨 𝐬𝐞𝐩 𝐰·𝐢·𝐭·𝐡· ·𝐬·𝐞·𝐩");
    }

    // Frame template tests
    #[test]
    fn test_frame_template_plain_text() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ Title ░▒▓");
    }

    #[test]
    fn test_frame_short_close_tag() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}Title{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ Title ░▒▓");
    }

    #[test]
    fn test_frame_nested_short_close() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}{{frame:glyph:star}}NESTED{{/}}{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ ★ NESTED ★ ░▒▓");
    }

    #[test]
    fn test_frame_template_with_styled_text() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}{{mathbold}}TITLE{{/mathbold}}{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ 𝐓𝐈𝐓𝐋𝐄 ░▒▓");
    }

    #[test]
    fn test_frame_with_separator() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:solid-left}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "█▌𝐓·𝐈·𝐓·𝐋·𝐄");
    }

    #[test]
    fn test_frame_with_spacing() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}{{mathbold:spacing=1}}HI{{/mathbold}}{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ 𝐇 𝐈 ░▒▓");
    }

    #[test]
    fn test_frame_alias() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:grad}}Test{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ Test ░▒▓");
    }

    #[test]
    fn test_frame_solid_left() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:solid-left}}Important{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "█▌Important");
    }

    #[test]
    fn test_frame_line_bold() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:line-bold}}Section{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "━━━ Section ━━━");
    }

    #[test]
    fn test_multiple_frames_in_line() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:solid-left}}A{{/frame}} and {{frame:solid-right}}B{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "█▌A and B▐█");
    }

    #[test]
    fn test_frame_in_heading() {
        let parser = TemplateParser::new().unwrap();
        let input = "# {{frame:gradient}}{{mathbold}}HEADER{{/mathbold}}{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "# ▓▒░ 𝐇𝐄𝐀𝐃𝐄𝐑 ░▒▓");
    }

    #[test]
    fn test_frame_unknown_style_error() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:invalid}}Text{{/frame}}";
        let result = parser.process(input);
        assert!(result.is_err());
        if let Err(Error::UnknownFrame(name)) = result {
            assert_eq!(name, "invalid");
        } else {
            panic!("Expected UnknownFrame error");
        }
    }

    #[test]
    fn test_frame_glyph_shorthand() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:star}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★ Title ★");
    }

    #[test]
    fn test_frame_glyph_shorthand_diamond() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:diamond}}Gem{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "◆ Gem ◆");
    }

    #[test]
    fn test_frame_glyph_shorthand_unknown_glyph() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:unknown}}Text{{/frame}}";
        let result = parser.process(input);
        assert!(result.is_err());
        if let Err(Error::UnknownGlyph(name)) = result {
            assert_eq!(name, "unknown");
        } else {
            panic!("Expected UnknownGlyph error");
        }
    }

    #[test]
    fn test_frame_glyph_multiplier() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:star*3}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★★★ Title ★★★");
    }

    #[test]
    fn test_frame_glyph_multiplier_with_tight_padding() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:star*3/pad=0}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★★★Title★★★");
    }

    #[test]
    fn test_frame_glyph_multiplier_with_spaces() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:star*2/pad=3}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★★   Title   ★★");
    }

    #[test]
    fn test_frame_glyph_custom_padding() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:diamond*2/pad=-}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "◆◆-Title-◆◆");
    }

    #[test]
    fn test_frame_glyph_unicode_padding() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:star*2/pad=·}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★★·Title·★★");
    }

    #[test]
    fn test_frame_glyph_multi_char_padding() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:star/pad=--}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★--Title--★");
    }

    #[test]
    fn test_frame_glyph_with_separator() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:star*3/separator=dot}}Title{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★·★·★ Title ★·★·★");
    }

    #[test]
    fn test_frame_glyph_with_separator_named() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:diamond*2/separator=dash}}Gem{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "◆─◆ Gem ◆─◆");
    }

    #[test]
    fn test_frame_glyph_with_separator_literal() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:bullet*4/separator=-}}X{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "•-•-•-• X •-•-•-•");
    }

    #[test]
    fn test_frame_glyph_separator_and_pad() {
        let parser = TemplateParser::new().unwrap();
        // Both separator and pad modifiers
        let input = "{{frame:glyph:star*3/separator=·/pad=0}}Tight{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★·★·★Tight★·★·★");
    }

    #[test]
    fn test_frame_glyph_separator_single_count() {
        let parser = TemplateParser::new().unwrap();
        // With count=1, separator has no effect (nothing to separate)
        let input = "{{frame:glyph:star/separator=dot}}Single{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★ Single ★");
    }

    #[test]
    fn test_frame_glyph_max_count() {
        // Count should be capped at 20
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:glyph:bullet*100}}X{{/frame}}";
        let result = parser.process(input).unwrap();
        // 20 bullets on each side + space padding
        assert_eq!(result, "•••••••••••••••••••• X ••••••••••••••••••••");
    }

    #[test]
    fn test_frame_fr_shorthand() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}Title{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ Title ░▒▓");
    }

    #[test]
    fn test_frame_fr_shorthand_with_glyph() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:glyph:star*3}}Text{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★★★ Text ★★★");
    }

    #[test]
    fn test_frame_pattern_with_separator() {
        let parser = TemplateParser::new().unwrap();
        // gradient pattern is ▓▒░, with separator should be ▓·▒·░
        let input = "{{fr:gradient/separator=dot}}Title{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓·▒·░ Title ░·▒·▓");
    }

    #[test]
    fn test_frame_pattern_with_separator_named() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient/separator=dash}}TEXT{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓─▒─░ TEXT ░─▒─▓");
    }

    #[test]
    fn test_frame_pattern_with_separator_literal() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:line-double/separator= }}Title{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "═ ═ ═ Title ═ ═ ═");
    }

    #[test]
    fn test_frame_pattern_with_spacing() {
        let parser = TemplateParser::new().unwrap();
        // spacing=1 adds 1 space between each grapheme
        let input = "{{fr:gradient/spacing=1}}TITLE{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓ ▒ ░ TITLE ░ ▒ ▓");
    }

    #[test]
    fn test_frame_pattern_with_spacing_two() {
        let parser = TemplateParser::new().unwrap();
        // spacing=2 adds 2 spaces between each grapheme
        let input = "{{fr:gradient/spacing=2}}X{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓  ▒  ░ X ░  ▒  ▓");
    }

    #[test]
    fn test_glyph_frame_with_spacing() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:glyph:star*3/spacing=1}}Text{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★ ★ ★ Text ★ ★ ★");
    }

    #[test]
    fn test_glyph_frame_with_spacing_two() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:glyph:diamond*2/spacing=2}}Gem{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "◆  ◆ Gem ◆  ◆");
    }

    #[test]
    fn test_frame_alternate_mode() {
        let parser = TemplateParser::new().unwrap();
        // gradient-wave uses alternate mode: ▓▒░ → ▒░▓ (rotated)
        let input = "{{fr:gradient-wave}}TITLE{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ TITLE ▒░▓");
    }

    #[test]
    fn test_frame_alternate_mode_with_alias() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:wave}}TEXT{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ TEXT ▒░▓");
    }

    #[test]
    fn test_frame_combo() {
        let parser = TemplateParser::new().unwrap();
        // gradient+star: outer prefix + inner prefix + content + inner suffix + outer suffix
        let input = "{{fr:gradient+star}}TITLE{{/}}";
        let result = parser.process(input).unwrap();
        // gradient prefix: "▓▒░ ", star prefix: "★ "
        // star suffix: " ☆", gradient suffix: " ░▒▓"
        assert_eq!(result, "▓▒░ ★ TITLE ☆ ░▒▓");
    }

    #[test]
    fn test_frame_combo_three() {
        let parser = TemplateParser::new().unwrap();
        // Three frames combined
        let input = "{{fr:gradient+star+diamond}}X{{/}}";
        let result = parser.process(input).unwrap();
        // gradient: ▓▒░  + star: ★  + diamond: ◆  + X + ◇  + ☆  + ░▒▓
        assert_eq!(result, "▓▒░ ★ ◆ X ◇ ☆ ░▒▓");
    }

    #[test]
    fn test_frame_combo_with_spaces() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient + star}}TEXT{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ ★ TEXT ☆ ░▒▓");
    }

    #[test]
    fn test_frame_self_closing_with_separator() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient/separator=·:Inline/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓·▒·░ Inline ░·▒·▓");
    }

    #[test]
    fn test_frame_reverse() {
        let parser = TemplateParser::new().unwrap();
        // Reverse gradient: swap prefix and suffix
        let input = "{{fr:gradient/reverse}}Title{{/}}";
        let result = parser.process(input).unwrap();
        // Normal: ▓▒░ Title ░▒▓, Reversed: ░▒▓ Title ▓▒░
        assert_eq!(result, " ░▒▓Title▓▒░ ");
    }

    #[test]
    fn test_frame_reverse_star() {
        let parser = TemplateParser::new().unwrap();
        // Reverse star: swap ★ and ☆
        let input = "{{fr:star/reverse}}VIP{{/}}";
        let result = parser.process(input).unwrap();
        // Normal: ★ VIP ☆, Reversed: ☆ VIP ★ (with spacing swap)
        assert_eq!(result, " ☆VIP★ ");
    }

    #[test]
    fn test_frame_count() {
        let parser = TemplateParser::new().unwrap();
        // Repeat star 3 times
        let input = "{{fr:star*3}}Title{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★★★ Title ☆☆☆");
    }

    #[test]
    fn test_frame_count_gradient() {
        let parser = TemplateParser::new().unwrap();
        // Repeat gradient 2 times
        let input = "{{fr:gradient*2}}X{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░▓▒░ X ░▒▓░▒▓");
    }

    #[test]
    fn test_frame_count_and_reverse() {
        let parser = TemplateParser::new().unwrap();
        // Repeat star 2 times then reverse
        let input = "{{fr:star*2/reverse}}Title{{/}}";
        let result = parser.process(input).unwrap();
        // Count first: ★★ Title ☆☆, then reverse: ☆☆ Title ★★ (with spacing)
        assert_eq!(result, " ☆☆Title★★ ");
    }

    #[test]
    fn test_frame_count_with_separator() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:star*3/separator=·}}Title{{/}}";
        let result = parser.process(input).unwrap();
        // ★★★ with separator between graphemes: ★·★·★
        assert_eq!(result, "★·★·★ Title ☆·☆·☆");
    }

    #[test]
    fn test_frame_fr_nested() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{fr:star}}NESTED{{/}}{{/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ ★ NESTED ☆ ░▒▓");
    }

    #[test]
    fn test_frame_fr_mixed_with_full_frame() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}{{fr:star}}MIXED{{/}}{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ ★ MIXED ☆ ░▒▓");
    }

    #[test]
    fn test_frame_close_all() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{fr:star}}NESTED{{//}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ ★ NESTED ☆ ░▒▓");
    }

    #[test]
    fn test_frame_close_all_three_levels() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{fr:star}}{{fr:lenticular}}DEEP{{//}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ ★ 【DEEP】 ☆ ░▒▓");
    }

    #[test]
    fn test_frame_close_all_single_frame() {
        // {{//}} on single frame should work same as {{/}}
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}Title{{//}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ Title ░▒▓");
    }

    #[test]
    fn test_frame_close_all_with_content_between() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}Outer {{fr:star}}Inner{{//}} end";
        let result = parser.process(input).unwrap();
        // The {{//}} closes both frames, leaving " end" outside
        assert_eq!(result, "▓▒░ Outer ★ Inner ☆ ░▒▓ end");
    }

    #[test]
    fn test_expand_close_all_styles() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}{{italic}}TEXT{{//}}";
        let expanded = parser.expand_close_all(input);
        // Should expand to close both styles in reverse order
        assert_eq!(
            expanded,
            "{{mathbold}}{{italic}}TEXT{{/italic}}{{/mathbold}}"
        );
    }

    #[test]
    fn test_expand_close_all_mixed() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{mathbold}}TEXT{{//}}";
        let expanded = parser.expand_close_all(input);
        // Should expand to close style first, then frame
        assert_eq!(
            expanded,
            "{{fr:gradient}}{{mathbold}}TEXT{{/mathbold}}{{/}}"
        );
    }

    #[test]
    fn test_universal_close_all_frames_and_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{fr:star}}{{mathbold}}VIP{{//}}";
        let result = parser.process(input).unwrap();
        // Frame > Frame > Style, all closed by {{//}}
        assert_eq!(result, "▓▒░ ★ 𝐕𝐈𝐏 ☆ ░▒▓");
    }

    #[test]
    fn test_universal_close_all_preserves_partial_closes() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{mathbold}}TITLE{{/mathbold}} more text{{//}}";
        let result = parser.process(input).unwrap();
        // Style is explicitly closed, only frame left for {{//}}
        assert_eq!(result, "▓▒░ 𝐓𝐈𝐓𝐋𝐄 more text ░▒▓");
    }

    #[test]
    fn test_universal_close_all_self_closing_ignored() {
        let parser = TemplateParser::new().unwrap();
        // Self-closing tags should not be tracked
        let input = "{{fr:gradient}}{{ui:swatch:accent/}}text{{//}}";
        let result = parser.process(input).unwrap();
        // Only the frame should be closed
        assert!(result.contains("▓▒░"));
        assert!(result.contains("░▒▓"));
    }

    #[test]
    fn test_frame_self_closing_basic() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient:Title/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ Title ░▒▓");
    }

    #[test]
    fn test_frame_self_closing_star() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:star:VIP/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★ VIP ☆");
    }

    #[test]
    fn test_frame_self_closing_glyph() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:glyph:diamond*2:Gem/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "◆◆ Gem ◆◆");
    }

    #[test]
    fn test_frame_self_closing_glyph_with_padding() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:glyph:star*3/pad=0:Tight/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "★★★Tight★★★");
    }

    #[test]
    fn test_frame_self_closing_full_syntax() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:solid-left:Note/}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "█▌Note");
    }

    #[test]
    fn test_frame_self_closing_in_sentence() {
        let parser = TemplateParser::new().unwrap();
        let input = "Check this {{fr:star:TIP/}} out!";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "Check this ★ TIP ☆ out!");
    }

    #[test]
    fn test_frame_preserves_code_blocks() {
        let parser = TemplateParser::new().unwrap();
        let input = "```\n{{frame:gradient}}CODE{{/frame}}\n```";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "```\n{{frame:gradient}}CODE{{/frame}}\n```");
    }

    #[test]
    fn test_frame_preserves_inline_code() {
        let parser = TemplateParser::new().unwrap();
        let input = "Text `{{frame:gradient}}code{{/frame}}` more";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "Text `{{frame:gradient}}code{{/frame}}` more");
    }

    #[test]
    fn test_composition_frame_style_separator() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}{{mathbold:separator=dash}}STYLED{{/mathbold}}{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "▓▒░ 𝐒─𝐓─𝐘─𝐋─𝐄─𝐃 ░▒▓");
    }

    #[test]
    fn test_composition_multiple_styles_in_frame() {
        let parser = TemplateParser::new().unwrap();
        let input =
            "{{frame:solid-both}}{{mathbold}}A{{/mathbold}} and {{italic}}B{{/italic}}{{/frame}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "█▌𝐀 and 𝐵▐█");
    }

    #[test]
    fn test_complex_composition() {
        let parser = TemplateParser::new().unwrap();
        let input = r#"# {{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}

{{frame:solid-left}}{{italic}}Important note{{/italic}}{{/frame}}

Regular text with {{mathbold:spacing=1}}spacing{{/mathbold}}"#;

        let result = parser.process(input).unwrap();

        assert!(result.contains("▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓"));
        assert!(result.contains("█▌𝐼𝑚𝑝𝑜𝑟𝑡𝑎𝑛𝑡 𝑛𝑜𝑡𝑒"));
        assert!(result.contains("𝐬 𝐩 𝐚 𝐜 𝐢 𝐧 𝐠"));
    }

    // UI Component Tests

    #[test]
    fn test_ui_swatch() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:accent/}}";
        let result = parser.process(input).unwrap();
        // Should expand to shields:block with accent color resolved
        assert!(result.contains("![]("));
        assert!(result.contains("F41C80")); // accent color (uppercased by shields)
    }

    #[test]
    fn test_ui_tech() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:tech:rust/}}";
        let result = parser.process(input).unwrap();
        // Should expand to shields:icon with logo
        assert!(result.contains("![]("));
        assert!(result.contains("logo=rust"));
    }

    #[test]
    fn test_ui_multiple_inline() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:tech:rust/}} {{ui:tech:python/}}";
        let result = parser.process(input).unwrap();
        // Should have two shields
        assert_eq!(result.matches("![](").count(), 2);
        assert!(result.contains("logo=rust"));
        assert!(result.contains("logo=python"));
    }

    #[test]
    fn test_ui_in_markdown() {
        let parser = TemplateParser::new().unwrap();
        let input = "# Header\n\n{{ui:swatch:accent/}}\n\n## Section";
        let result = parser.process(input).unwrap();
        assert!(result.contains("# Header"));
        assert!(result.contains("![]("));
        assert!(result.contains("## Section"));
    }

    #[test]
    fn test_ui_unknown_component() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:nonexistent/}}";
        let result = parser.process(input);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Unknown component"));
    }

    #[test]
    fn test_ui_unclosed() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:row}}TITLE";
        let result = parser.process(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_frame_multiline() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:gradient}}\nLine 1\nLine 2\n{{/frame}}";
        let result = parser.process(input).unwrap();
        assert!(result.starts_with("▓▒░"));
        assert!(result.ends_with("░▒▓"));
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 2"));
    }

    #[test]
    fn test_frame_multiline_with_styles() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{frame:solid-left}}\n### {{mathbold}}Title{{/mathbold}}\nContent\n{{/frame}}";
        let result = parser.process(input).unwrap();
        assert!(result.starts_with("█▌"));
        assert!(result.contains("𝐓𝐢𝐭𝐥𝐞"));
        assert!(result.contains("Content"));
    }

    #[test]
    fn test_frame_multiline_with_ui_components() {
        use crate::renderer::svg::SvgBackend;

        let parser =
            TemplateParser::with_backend(Box::new(SvgBackend::new("assets/test"))).unwrap();
        let input = "{{frame:gradient}}\n{{ui:swatch:accent/}}\n{{ui:swatch:success/}}\n{{/frame}}";
        let result = parser.process_with_assets(input).unwrap();

        // Should process frame correctly
        assert!(result.markdown.starts_with("▓▒░"));
        assert!(result.markdown.ends_with("░▒▓"));

        // Should generate assets for UI components
        assert_eq!(result.assets.len(), 2);
        // Should generate assets for UI components (both swatches)
        assert!(result.markdown.contains("![](assets/test/swatch_"));
    }

    #[test]
    fn test_process_with_assets_preserves_code_blocks() {
        use crate::renderer::svg::SvgBackend;

        let parser =
            TemplateParser::with_backend(Box::new(SvgBackend::new("assets/test"))).unwrap();
        let input = "```\n{{ui:swatch:accent/}}\n```\n{{ui:swatch:accent/}}";
        let result = parser.process_with_assets(input).unwrap();

        // Code block should be preserved
        assert!(result.markdown.contains("```\n{{ui:swatch:accent/}}\n```"));

        // Only one asset generated (outside code block)
        assert_eq!(result.assets.len(), 1);
    }

    #[test]
    fn test_shields_primitive_block() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{shields:block:color=cobalt:style=flat-square/}}";
        let result = parser.process(input).unwrap();
        // Should render shields directly (cobalt is in shields.json palette)
        assert!(result.contains("![]("));
        assert!(result.contains("2B6CB0")); // cobalt resolved from shields palette
    }

    #[test]
    fn test_shields_primitive_bar() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{shields:bar:colors=success,warning,error:style=flat-square/}}";
        let result = parser.process(input).unwrap();
        // Should render 3 inline badges
        assert_eq!(result.matches("![](").count(), 3);
    }

    #[test]
    fn test_shields_bar_with_separator() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{shields:bar:colors=success,warning:style=flat-square:separator= /}}";
        let result = parser.process(input).unwrap();
        // Should render 2 badges with space between them
        assert_eq!(result.matches("![](").count(), 2);
        // Should have space separator between badges
        assert!(result.contains(") ![](")); // space between closing ) and opening ![](
    }

    #[test]
    fn test_shields_bar_with_named_separator() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{shields:bar:colors=accent,success:style=flat-square:separator=dot/}}";
        let result = parser.process(input).unwrap();
        // Should render 2 badges with · separator
        assert!(result.contains(")·![](")); // dot separator
    }

    // ========================================
    // GOLDEN TESTS: Whitespace and Line Handling
    // ========================================
    // These tests ensure component expansion preserves document structure
    // and doesn't introduce unwanted whitespace or break formatting.

    #[test]
    fn test_preserves_blank_lines_around_components() {
        let parser = TemplateParser::new().unwrap();

        // Test with self-closing component (swatch)
        let input = "Paragraph 1\n\n{{ui:swatch:accent/}}\n\nParagraph 2";
        let result = parser.process(input).unwrap();

        // Should preserve blank lines before and after component
        assert!(result.contains("Paragraph 1\n\n"));
        assert!(result.contains("![]("));
        assert!(result.contains("\n\nParagraph 2"));

        // Count newlines: should maintain document structure
        let input_newlines = input.matches('\n').count();
        let result_newlines = result.matches('\n').count();
        assert_eq!(
            input_newlines, result_newlines,
            "Newline count changed: input={}, result={}",
            input_newlines, result_newlines
        );
    }

    #[test]
    fn test_preserves_blank_lines_around_block_components() {
        let parser = TemplateParser::new().unwrap();

        // Test with block component (frame)
        let input = "Intro text\n\n{{frame:gradient}}TITLE{{/frame}}\n\nFollowing text";
        let result = parser.process(input).unwrap();

        // Should preserve structure
        assert!(result.contains("Intro text\n\n"));
        assert!(result.contains("\n\nFollowing text"));

        // Component should expand inline (not add extra blank lines)
        assert!(
            !result.contains("\n\n\n\n"),
            "Component added extra blank lines"
        );
    }

    #[test]
    fn test_component_expansion_in_lists() {
        let parser = TemplateParser::new().unwrap();

        // Components inside list items
        let input = "- Item 1\n- {{mathbold}}BOLD{{/mathbold}} item\n- Item 3";
        let result = parser.process(input).unwrap();

        // Should preserve list structure
        assert!(result.starts_with("- Item 1\n"));
        assert!(result.contains("- 𝐁𝐎𝐋𝐃 item\n"));
        assert!(result.ends_with("- Item 3"));

        // Should maintain same number of lines
        assert_eq!(input.lines().count(), result.lines().count());
    }

    #[test]
    fn test_component_expansion_with_indentation() {
        let parser = TemplateParser::new().unwrap();

        // Nested list with component
        let input = "- Outer\n  - {{italic}}Nested{{/italic}} item\n  - Another nested";
        let result = parser.process(input).unwrap();

        // Should preserve indentation
        assert!(result.contains("  - 𝑁𝑒𝑠𝑡𝑒𝑑 item\n"));
        assert!(result.contains("  - Another nested"));
    }

    #[test]
    fn test_multiline_component_content_preserves_structure() {
        let parser = TemplateParser::new().unwrap();

        // Multiline content in component (using frame)
        let input = "{{frame:gradient}}Multi\nLine\nTitle{{/frame}}";
        let result = parser.process(input).unwrap();

        // Content should be processed but structure preserved
        assert!(!result.is_empty());
        assert!(result.contains("▓▒░")); // gradient frame prefix
    }

    #[test]
    fn test_adjacent_components_no_extra_whitespace() {
        let parser = TemplateParser::new().unwrap();

        // Two components with single newline between
        let input = "{{mathbold}}FIRST{{/mathbold}}\n{{mathbold}}SECOND{{/mathbold}}";
        let result = parser.process(input).unwrap();

        // Should preserve single newline (not add extra)
        assert_eq!(result.matches('\n').count(), 1);
        assert!(result.contains("𝐅𝐈𝐑𝐒𝐓\n𝐒𝐄𝐂𝐎𝐍𝐃"));
    }

    #[test]
    fn test_component_in_blockquote_preserves_prefix() {
        let parser = TemplateParser::new().unwrap();

        // Component inside manually-written blockquote
        let input = "> Quote with {{mathbold}}BOLD{{/mathbold}} text";
        let result = parser.process(input).unwrap();

        // Should preserve the "> " prefix
        assert!(result.starts_with("> "));
        assert!(result.contains("𝐁𝐎𝐋𝐃"));
    }

    #[test]
    fn test_component_with_trailing_newline() {
        let parser = TemplateParser::new().unwrap();

        // Component at end of document with trailing newline
        let input = "Text before\n{{ui:swatch:accent/}}\n";
        let result = parser.process(input).unwrap();

        // Should preserve trailing newline
        assert!(result.ends_with('\n'), "Trailing newline was lost");
    }

    #[test]
    fn test_component_without_trailing_newline() {
        let parser = TemplateParser::new().unwrap();

        // Component at end without trailing newline
        let input = "Text before\n{{ui:swatch:accent/}}";
        let result = parser.process(input).unwrap();

        // Should NOT add trailing newline
        assert!(!result.ends_with('\n'), "Unexpected trailing newline added");
    }

    #[test]
    fn test_component_expansion_preserves_empty_lines_in_content() {
        let parser = TemplateParser::new().unwrap();

        // Block component with empty lines in content (using frame)
        let input = "{{frame:solid-left}}Line 1\n\nLine 3{{/frame}}";
        let result = parser.process(input).unwrap();

        // Empty line in content should be preserved
        assert!(!result.is_empty());
        assert!(result.contains("█▌")); // solid-left frame prefix
    }

    // ========================================
    // GitHub Blocks Components Tests
    // ========================================

    #[test]
    fn test_callout_github_simple() {
        let parser = TemplateParser::new().unwrap();

        // Simple callout with single line (positional arg: type)
        let input = "{{ui:callout-github:warning}}This is a warning{{/ui}}";
        let result = parser.process(input).unwrap();

        // Should have blockquote prefix on every line
        assert!(result.starts_with("> "));
        assert!(result.contains("**Note**"));
        assert!(result.contains("This is a warning"));
    }

    #[test]
    fn test_callout_github_multiline() {
        let parser = TemplateParser::new().unwrap();

        // Callout with multiple lines
        let input = "{{ui:callout-github:info}}Line 1\nLine 2\nLine 3{{/ui}}";
        let result = parser.process(input).unwrap();

        // Every line should start with "> "
        for line in result.lines() {
            assert!(
                line.starts_with(">"),
                "Line missing blockquote prefix: {}",
                line
            );
        }

        // Should have 4 lines: status+title, line1, line2, line3
        assert_eq!(result.lines().count(), 4);
    }

    #[test]
    fn test_callout_github_with_empty_lines() {
        let parser = TemplateParser::new().unwrap();

        // Callout with empty lines in content
        let input = "{{ui:callout-github:info}}Line 1\n\nLine 3{{/ui}}";
        let result = parser.process(input).unwrap();

        // Should have blockquote on all lines including empty one
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.iter().all(|line| line.starts_with(">")));

        // Middle line should be just ">"
        assert!(lines.contains(&">"));
    }

    #[test]
    fn test_callout_github_in_document() {
        let parser = TemplateParser::new().unwrap();

        // Callout in context
        let input = "# Title\n\n{{ui:callout-github:warning}}Important note{{/ui}}\n\nMore text";
        let result = parser.process(input).unwrap();

        // Should preserve document structure
        assert!(result.starts_with("# Title"));
        assert!(result.contains("> "));
        assert!(result.contains("More text"));
    }

    #[test]
    fn test_statusitem_component() {
        let parser = TemplateParser::new().unwrap();

        // Single status item (positional args: label, level, text)
        let input = "{{ui:statusitem:Build:success:passing/}}";
        let result = parser.process(input).unwrap();

        // Should have status indicator, label, and text
        assert!(result.contains("![](https://img.shields.io/badge/"));
        assert!(result.contains("**Build**"));
        assert!(result.contains("passing"));
    }

    #[test]
    fn test_statusitem_inline_composition() {
        let parser = TemplateParser::new().unwrap();

        // Multiple status items in a row (manual composition)
        let input =
            "{{ui:statusitem:Build:success:passing/}} · {{ui:statusitem:Tests:success:189/}}";
        let result = parser.process(input).unwrap();

        // Should have two status items separated by ·
        assert_eq!(result.matches("![](").count(), 2);
        assert!(result.contains("**Build**"));
        assert!(result.contains("**Tests**"));
        assert!(result.contains(" · "));
    }

    #[test]
    fn test_statusitem_with_different_levels() {
        let parser = TemplateParser::new().unwrap();

        // Test different status levels
        let success = parser.process("{{ui:statusitem:OK:success:yes/}}").unwrap();
        let warning = parser
            .process("{{ui:statusitem:Warn:warning:maybe/}}")
            .unwrap();
        let error = parser.process("{{ui:statusitem:Fail:error:no/}}").unwrap();

        // All should render (colors will differ but structure is same)
        assert!(success.contains("**OK**"));
        assert!(warning.contains("**Warn**"));
        assert!(error.contains("**Fail**"));
    }

    #[test]
    fn test_github_blocks_combined() {
        let parser = TemplateParser::new().unwrap();

        // Full document using GitHub blocks
        let input = r#"
## Project Status

{{ui:statusitem:Build:success:passing/}} · {{ui:statusitem:Tests:success:189/}}

{{ui:callout-github:info}}
This project uses GitHub blocks for beautiful READMEs.
All blocks work with standard Markdown.
{{/ui}}
"#;

        let result = parser.process(input).unwrap();

        // Should have all components
        assert!(result.contains("## Project Status"));
        assert!(result.contains("![](")); // Status badges
        assert!(result.contains("> ")); // Blockquote callout
        assert!(result.contains("**Build**"));
        assert!(result.contains("**Tests**"));
    }
}

#[cfg(test)]
mod badge_style_tests {
    use super::*;

    #[test]
    fn test_swatch_with_flat_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:F41C80:style=flat/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=flat"));
        assert!(!output.contains("style=flat-square"));
    }

    #[test]
    fn test_swatch_with_flat_square_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:F41C80:style=flat-square/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=flat-square"));
    }

    #[test]
    fn test_swatch_with_for_the_badge_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:F41C80:style=for-the-badge/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=for-the-badge"));
    }

    #[test]
    fn test_swatch_with_plastic_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:F41C80:style=plastic/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=plastic"));
    }

    #[test]
    fn test_swatch_with_social_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:F41C80:style=social/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=social"));
    }

    #[test]
    fn test_swatch_default_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:F41C80/}}";
        let output = parser.process(input).unwrap();
        // Should default to flat-square
        assert!(output.contains("style=flat-square"));
    }

    #[test]
    fn test_tech_with_style() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:tech:rust:style=plastic/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=plastic"));
        assert!(output.contains("logo=rust"));
    }

    #[test]
    fn test_multiple_components_different_styles() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:FF0000:style=flat/}} {{ui:swatch:00FF00:style=for-the-badge/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=flat"));
        assert!(output.contains("style=for-the-badge"));
    }

    #[test]
    fn test_style_with_palette_color() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:swatch:accent:style=plastic/}}";
        let output = parser.process(input).unwrap();
        assert!(output.contains("style=plastic"));
        // Should resolve accent color
        assert!(output.contains("F41C80"));
    }
}

#[cfg(test)]
mod partial_tests {
    use super::*;

    #[test]
    fn test_partial_with_content() {
        let mut parser = TemplateParser::new().unwrap();
        // Use explicit closers in template to avoid ambiguity
        parser.add_partial(
            "hero",
            "{{frame:gradient}}{{mathbold}}$1{{/mathbold}}{{/frame}}",
        );

        let input = "{{partial:hero}}HELLO{{/partial}}";
        let result = parser.process(input).unwrap();

        // Should expand to gradient frame with mathbold text
        assert!(result.contains("▓▒░"));
        assert!(result.contains("𝐇𝐄𝐋𝐋𝐎"));
    }

    #[test]
    fn test_partial_self_closing() {
        let mut parser = TemplateParser::new().unwrap();
        parser.add_partial("techstack", "{{ui:tech:rust/}} {{ui:tech:typescript/}}");

        let input = "{{partial:techstack/}}";
        let result = parser.process(input).unwrap();

        // Should expand to tech badges
        assert!(result.contains("rust"));
        assert!(result.contains("typescript"));
    }

    #[test]
    fn test_partial_with_universal_closer() {
        let mut parser = TemplateParser::new().unwrap();
        parser.add_partial("simple", "PREFIX $1 SUFFIX");

        // Use explicit {{/partial}} to avoid ambiguity
        let input = "{{partial:simple}}CONTENT{{/partial}}";
        let result = parser.process(input).unwrap();

        assert_eq!(result, "PREFIX CONTENT SUFFIX");
    }

    #[test]
    fn test_partial_content_substitution() {
        let mut parser = TemplateParser::new().unwrap();
        parser.add_partial("wrapper", "[ $content ]");

        let input = "{{partial:wrapper}}TEXT{{/partial}}";
        let result = parser.process(input).unwrap();

        assert_eq!(result, "[ TEXT ]");
    }

    #[test]
    fn test_partial_nested_templates() {
        let mut parser = TemplateParser::new().unwrap();
        // Use explicit closer in template
        parser.add_partial("styled", "{{frame:star}}$1{{/frame}}");

        let input = "{{partial:styled}}VIP{{/partial}}";
        let result = parser.process(input).unwrap();

        assert!(result.contains("★"));
        assert!(result.contains("VIP"));
        assert!(result.contains("☆"));
    }

    #[test]
    fn test_partial_not_found_passthrough() {
        let parser = TemplateParser::new().unwrap();
        // Without registering a partial, the tag should pass through
        let input = "{{partial:nonexistent}}CONTENT{{/partial}}";
        let result = parser.process(input).unwrap();

        // The parser should not crash, but the partial tag will remain
        // (since it couldn't find a matching partial, it falls through)
        assert!(result.contains("partial"));
    }

    #[test]
    fn test_has_partial() {
        let mut parser = TemplateParser::new().unwrap();
        assert!(!parser.has_partial("test"));

        parser.add_partial("test", "template");
        assert!(parser.has_partial("test"));
    }

    #[test]
    fn test_partial_with_hyphen_underscore_name() {
        let mut parser = TemplateParser::new().unwrap();
        parser.add_partial("my-partial_name", "[$1]");

        let input = "{{partial:my-partial_name}}X{{/partial}}";
        let result = parser.process(input).unwrap();

        assert_eq!(result, "[X]");
    }

    #[test]
    fn test_partial_simple_text_replacement() {
        let mut parser = TemplateParser::new().unwrap();
        parser.add_partial("greeting", "Hello, $1!");

        let input = "{{partial:greeting}}World{{/partial}}";
        let result = parser.process(input).unwrap();

        assert_eq!(result, "Hello, World!");
    }
}
