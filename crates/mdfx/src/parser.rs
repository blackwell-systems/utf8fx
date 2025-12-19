use crate::components::{ComponentOutput, ComponentsRenderer, PostProcess};
use crate::config::{expand_partial, MdfxConfig};
use crate::converter::Converter;
use crate::error::{Error, Result};
use crate::registry::Registry;
use crate::renderer::shields::ShieldsBackend;
use crate::renderer::{RenderedAsset, Renderer};
use crate::shields::ShieldsRenderer;
use std::collections::HashMap;

/// Variation Selector 15 - forces text presentation for Unicode characters
/// that have both text and emoji variants (e.g., ☢ renders as glyph, not emoji)
const VS15: char = '\u{FE0E}';

/// Append VS15 to each non-whitespace character to force text-style rendering
fn text_style(glyph: &str) -> String {
    let mut result = String::with_capacity(glyph.len() * 2);
    for c in glyph.chars() {
        result.push(c);
        // Only add VS15 to non-whitespace characters (emoji variants)
        if !c.is_whitespace() {
            result.push(VS15);
        }
    }
    result
}

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

    /// Set the fetch context for dynamic badges (GitHub, npm, etc.)
    ///
    /// This enables dynamic components like `{{ui:github:owner/repo:stars/}}`
    /// that fetch live data from external APIs.
    #[cfg(feature = "fetch")]
    pub fn set_fetch_context(&mut self, ctx: crate::components::FetchContext) {
        self.components_renderer.set_fetch_context(ctx);
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
    /// let processed = parser.process_with_assets("{{ui:swatch:pink/}}").unwrap();
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

    // ========================================================================
    // Template Handlers - each returns Option<(output, assets, end_pos)>
    // ========================================================================

    /// Handle partial template expansion
    fn handle_partial(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(String, Vec<RenderedAsset>, usize)>> {
        if let Some(data) = self.parse_partial_at(chars, start)? {
            if let Some(template) = self.partials.get(&data.partial_name) {
                let expanded = expand_partial(template, &data.content);
                let (processed, assets) = self.process_templates_with_assets(&expanded)?;
                return Ok(Some((processed, assets, data.end_pos)));
            }
        }
        Ok(None)
    }

    /// Handle UI component expansion
    fn handle_ui(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(String, Vec<RenderedAsset>, usize)>> {
        let Some(data) = self.parse_ui_at(chars, start)? else {
            return Ok(None);
        };

        let output = self.components_renderer.expand(
            &data.component_name,
            &data.args,
            data.content.as_deref(),
        )?;

        let (result, assets) = match output {
            ComponentOutput::Primitive(primitive) => {
                let rendered = self.backend.render(&primitive)?;
                let markdown = rendered.to_markdown().to_string();
                let assets = if rendered.is_file_based() {
                    vec![rendered]
                } else {
                    vec![]
                };
                (markdown, assets)
            }
            ComponentOutput::Template(template) => self.process_templates_with_assets(&template)?,
            ComponentOutput::TemplateDelayed {
                template,
                post_process,
            } => {
                let (processed, assets) = self.process_templates_with_assets(&template)?;
                let final_output = match post_process {
                    PostProcess::Row { align } => ComponentsRenderer::apply_row(&processed, &align),
                    _ => processed,
                };
                (final_output, assets)
            }
        };

        Ok(Some((result, assets, data.end_pos)))
    }

    /// Handle frame template expansion
    fn handle_frame(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(String, Vec<RenderedAsset>, usize)>> {
        let Some(data) = self.parse_frame_at(chars, start)? else {
            return Ok(None);
        };

        // Process content recursively
        let (content, assets) = self.process_templates_with_assets(&data.content)?;

        let framed = if data.frame_style.starts_with("glyph:") {
            self.apply_glyph_frame(&data.frame_style[6..], &content)?
        } else if data.frame_style.contains('+') {
            self.apply_combo_frame(&data.frame_style, &content)?
        } else {
            self.apply_standard_frame(&data.frame_style, &content)?
        };

        Ok(Some((framed, assets, data.end_pos)))
    }

    /// Handle shields template (escape hatch)
    fn handle_shields(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(String, Vec<RenderedAsset>, usize)>> {
        let Some(data) = self.parse_shields_at(chars, start)? else {
            return Ok(None);
        };

        let rendered = match data.shield_type.as_str() {
            "block" => {
                let color = data.params.get("color").ok_or_else(|| {
                    Error::MissingShieldParam("color".to_string(), "block".to_string())
                })?;
                let style = data.params.get("style").ok_or_else(|| {
                    Error::MissingShieldParam("style".to_string(), "block".to_string())
                })?;
                self.shields_renderer.render_block(color, style)?
            }
            "twotone" => {
                let left = data.params.get("left").ok_or_else(|| {
                    Error::MissingShieldParam("left".to_string(), "twotone".to_string())
                })?;
                let right = data.params.get("right").ok_or_else(|| {
                    Error::MissingShieldParam("right".to_string(), "twotone".to_string())
                })?;
                let style = data.params.get("style").ok_or_else(|| {
                    Error::MissingShieldParam("style".to_string(), "twotone".to_string())
                })?;
                self.shields_renderer.render_twotone(left, right, style)?
            }
            "bar" => {
                let colors_str = data.params.get("colors").ok_or_else(|| {
                    Error::MissingShieldParam("colors".to_string(), "bar".to_string())
                })?;
                let colors: Vec<String> = colors_str.split(',').map(|s| s.to_string()).collect();
                let style = data.params.get("style").ok_or_else(|| {
                    Error::MissingShieldParam("style".to_string(), "bar".to_string())
                })?;
                let separator = data.params.get("separator").map(|s| {
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
                let logo = data.params.get("logo").ok_or_else(|| {
                    Error::MissingShieldParam("logo".to_string(), "icon".to_string())
                })?;
                let bg = data.params.get("bg").ok_or_else(|| {
                    Error::MissingShieldParam("bg".to_string(), "icon".to_string())
                })?;
                let logo_color = data.params.get("logoColor").ok_or_else(|| {
                    Error::MissingShieldParam("logoColor".to_string(), "icon".to_string())
                })?;
                let style = data.params.get("style").ok_or_else(|| {
                    Error::MissingShieldParam("style".to_string(), "icon".to_string())
                })?;
                self.shields_renderer
                    .render_icon(logo, bg, logo_color, style)?
            }
            _ => return Err(Error::UnknownShieldType(data.shield_type)),
        };

        Ok(Some((rendered, vec![], data.end_pos)))
    }

    /// Handle glyph template
    fn handle_glyph(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(String, Vec<RenderedAsset>, usize)>> {
        let Some(data) = self.parse_glyph_at(chars, start)? else {
            return Ok(None);
        };

        let glyph_char = self
            .registry
            .glyph(&data.glyph_name)
            .ok_or_else(|| Error::UnknownGlyph(data.glyph_name.clone()))?;

        Ok(Some((text_style(glyph_char), vec![], data.end_pos)))
    }

    /// Handle kbd template
    fn handle_kbd(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(String, Vec<RenderedAsset>, usize)>> {
        let Some(data) = self.parse_kbd_at(chars, start)? else {
            return Ok(None);
        };

        let expanded = self.expand_kbd(&data.keys);
        Ok(Some((expanded, vec![], data.end_pos)))
    }

    /// Handle style template
    fn handle_style(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(String, Vec<RenderedAsset>, usize)>> {
        let Some(data) = self.parse_template_at(chars, start)? else {
            return Ok(None);
        };

        if self.registry.style(&data.style).is_none() {
            return Err(Error::UnknownStyle(data.style));
        }

        let converted = if let Some(ref sep) = data.separator {
            self.converter
                .convert_with_separator(&data.content, &data.style, sep, 1)?
        } else if data.spacing > 0 {
            self.converter
                .convert_with_spacing(&data.content, &data.style, data.spacing)?
        } else {
            self.converter.convert(&data.content, &data.style)?
        };

        Ok(Some((converted, vec![], data.end_pos)))
    }

    // ========================================================================
    // Frame application helpers
    // ========================================================================

    /// Apply glyph-based frame (e.g., glyph:star*3/pad=0)
    fn apply_glyph_frame(&self, spec: &str, content: &str) -> Result<String> {
        let (glyph_name, count, pad, separator, spacing) = Self::parse_glyph_frame_spec(spec);

        let glyph_raw = self
            .registry
            .glyph(&glyph_name)
            .ok_or_else(|| Error::UnknownGlyph(glyph_name.clone()))?;

        // Apply text-style (VS15) to force glyph rendering, not emoji
        let glyph_char = text_style(glyph_raw);

        let glyphs: String = if let Some(sep) = separator {
            let sep_char = self.registry.separator(&sep).unwrap_or(&sep);
            (0..count)
                .map(|_| glyph_char.as_str())
                .collect::<Vec<_>>()
                .join(sep_char)
        } else if let Some(n) = spacing {
            let spaces = " ".repeat(n);
            (0..count)
                .map(|_| glyph_char.as_str())
                .collect::<Vec<_>>()
                .join(&spaces)
        } else {
            glyph_char.repeat(count)
        };

        Ok(format!("{}{}{}{}{}", glyphs, pad, content, pad, glyphs))
    }

    /// Apply combo frame (e.g., gradient+star)
    fn apply_combo_frame(&self, style: &str, content: &str) -> Result<String> {
        let frames: Vec<&str> = style.split('+').collect();
        let mut prefix = String::new();
        let mut suffix = String::new();

        for frame_name in &frames {
            let frame = self
                .registry
                .frame(frame_name.trim())
                .ok_or_else(|| Error::UnknownFrame(frame_name.to_string()))?;
            prefix.push_str(&frame.prefix);
        }

        for frame_name in frames.iter().rev() {
            let frame = self
                .registry
                .frame(frame_name.trim())
                .ok_or_else(|| Error::UnknownFrame(frame_name.to_string()))?;
            suffix.push_str(&frame.suffix);
        }

        // Apply VS15 to prefix/suffix for text-style rendering
        Ok(format!(
            "{}{}{}",
            text_style(&prefix),
            content,
            text_style(&suffix)
        ))
    }

    /// Apply standard frame with modifiers
    fn apply_standard_frame(&self, style: &str, content: &str) -> Result<String> {
        let mods = Self::parse_frame_modifiers(style);

        let frame = self
            .registry
            .frame(&mods.style)
            .ok_or_else(|| Error::UnknownFrame(mods.style.clone()))?;

        // Get base prefix/suffix, applying count if specified
        let (mut prefix, mut suffix) = if let Some(count) = mods.count {
            let prefix_pattern: String = frame.prefix.trim().to_string();
            let suffix_pattern: String = frame.suffix.trim().to_string();
            let repeated_prefix = prefix_pattern.repeat(count);
            let repeated_suffix = suffix_pattern.repeat(count);
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

        // Apply reverse modifier
        if mods.reverse {
            std::mem::swap(&mut prefix, &mut suffix);
        }

        // Apply separator or spacing
        if mods.separator.is_some() || mods.spacing.is_some() {
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

            use unicode_segmentation::UnicodeSegmentation;
            // Apply VS15 to each grapheme for text-style rendering
            let prefix_with_sep: String = prefix
                .trim()
                .graphemes(true)
                .map(text_style)
                .collect::<Vec<_>>()
                .join(&join_str);
            let suffix_with_sep: String = suffix
                .trim()
                .graphemes(true)
                .map(text_style)
                .collect::<Vec<_>>()
                .join(&join_str);

            let prefix_space = if prefix.ends_with(' ') { " " } else { "" };
            let suffix_space = if suffix.starts_with(' ') { " " } else { "" };

            Ok(format!(
                "{}{}{}{}{}",
                prefix_with_sep, prefix_space, content, suffix_space, suffix_with_sep
            ))
        } else if mods.count.is_some() || mods.reverse {
            // Apply VS15 to prefix/suffix for text-style rendering
            Ok(format!(
                "{}{}{}",
                text_style(&prefix),
                content,
                text_style(&suffix)
            ))
        } else {
            self.registry.apply_frame(content, &mods.style)
        }
    }

    // ========================================================================
    // Main parsing loop
    // ========================================================================

    /// Process templates in a text segment with asset collection
    fn process_templates_with_assets(&self, text: &str) -> Result<(String, Vec<RenderedAsset>)> {
        let text = self.expand_close_all(text);
        let chars: Vec<char> = text.chars().collect();
        let mut result = String::new();
        let mut assets = Vec::new();
        let mut i = 0;

        while i < chars.len() {
            // Check for template start
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '{' {
                // Try each handler in priority order
                if let Some((out, new_assets, end)) = self.handle_partial(&chars, i)? {
                    result.push_str(&out);
                    assets.extend(new_assets);
                    i = end;
                    continue;
                }
                if let Some((out, new_assets, end)) = self.handle_ui(&chars, i)? {
                    result.push_str(&out);
                    assets.extend(new_assets);
                    i = end;
                    continue;
                }
                if let Some((out, new_assets, end)) = self.handle_frame(&chars, i)? {
                    result.push_str(&out);
                    assets.extend(new_assets);
                    i = end;
                    continue;
                }
                if let Some((out, new_assets, end)) = self.handle_shields(&chars, i)? {
                    result.push_str(&out);
                    assets.extend(new_assets);
                    i = end;
                    continue;
                }
                if let Some((out, new_assets, end)) = self.handle_glyph(&chars, i)? {
                    result.push_str(&out);
                    assets.extend(new_assets);
                    i = end;
                    continue;
                }
                if let Some((out, new_assets, end)) = self.handle_kbd(&chars, i)? {
                    result.push_str(&out);
                    assets.extend(new_assets);
                    i = end;
                    continue;
                }
                if let Some((out, new_assets, end)) = self.handle_style(&chars, i)? {
                    result.push_str(&out);
                    assets.extend(new_assets);
                    i = end;
                    continue;
                }
            }

            // Not a template, add character as-is
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
    /// - Self-closing: {{ui:swatch:pink/}}
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

        // Find closing tag {{/ui}}, handling nested UI components
        let close_tag = "{{/ui}}";
        let close_chars: Vec<char> = close_tag.chars().collect();
        let open_prefix = "{{ui:";
        let open_prefix_chars: Vec<char> = open_prefix.chars().collect();

        // Track nesting depth (1 for the current opening tag)
        let mut depth = 1;

        while i < chars.len() {
            // Check for nested opening tag {{ui:...}}
            if i + open_prefix_chars.len() < chars.len() {
                let mut is_open = true;
                for (j, &open_ch) in open_prefix_chars.iter().enumerate() {
                    if chars[i + j] != open_ch {
                        is_open = false;
                        break;
                    }
                }

                if is_open {
                    // Found {{ui:, check if it's self-closing (/}}) or block (}})
                    // Search forward for }} or /}}
                    let mut k = i + open_prefix_chars.len();
                    let mut is_self_closing = false;
                    while k < chars.len() {
                        if k + 2 < chars.len()
                            && chars[k] == '/'
                            && chars[k + 1] == '}'
                            && chars[k + 2] == '}'
                        {
                            // Self-closing, don't increment depth
                            is_self_closing = true;
                            i = k + 3; // Skip past /}}
                            break;
                        }
                        if chars[k] == '}' && k + 1 < chars.len() && chars[k + 1] == '}' {
                            // Block opening tag, increment depth
                            depth += 1;
                            i = k + 2; // Skip past }}
                            break;
                        }
                        k += 1;
                    }
                    if is_self_closing || k < chars.len() {
                        continue;
                    }
                }
            }

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
                    depth -= 1;
                    if depth == 0 {
                        // Found matching closing tag
                        let content: String = chars[content_start..i].iter().collect();
                        let end_pos = i + close_chars.len();
                        return Ok(Some(UIData {
                            end_pos,
                            component_name,
                            args,
                            content: Some(content),
                        }));
                    } else {
                        // Skip this closing tag, it belongs to a nested component
                        i += close_chars.len();
                        continue;
                    }
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
    /// Supports self-closing only: {{shields:block:color=cobalt:style=flat-square/}}
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
    use crate::{test_process, test_process_bookends, test_process_contains, test_process_err, test_process_unchanged};

    #[test]
    fn test_parser_new() {
        let parser = TemplateParser::new();
        assert!(parser.is_ok());
    }

    // Basic template tests using macros
    #[test]
    fn test_simple_template() {
        test_process!("{{mathbold}}HELLO{{/mathbold}}" => "𝐇𝐄𝐋𝐋𝐎");
    }

    #[test]
    fn test_template_in_heading() {
        test_process!("# {{mathbold}}TITLE{{/mathbold}}" => "# 𝐓𝐈𝐓𝐋𝐄");
    }

    #[test]
    fn test_multiple_templates() {
        test_process!(
            "{{mathbold}}BOLD{{/mathbold}} and {{italic}}italic{{/italic}}"
            => "𝐁𝐎𝐋𝐃 and 𝑖𝑡𝑎𝑙𝑖𝑐"
        );
    }

    // Code preservation tests
    #[test]
    fn test_preserves_code_blocks() {
        test_process_unchanged!("```\n{{mathbold}}CODE{{/mathbold}}\n```");
    }

    #[test]
    fn test_preserves_code_blocks_with_language() {
        test_process_unchanged!("```markdown\n{{ui:test:arg/}}\nMore {{/ui}} content\n```");
    }

    #[test]
    fn test_preserves_inline_code() {
        test_process_unchanged!("Text `{{mathbold}}code{{/mathbold}}` more text");
    }

    #[test]
    fn test_multiline_template() {
        test_process!(
            "Line 1\n{{mathbold}}TITLE{{/mathbold}}\nLine 3"
            => "Line 1\n𝐓𝐈𝐓𝐋𝐄\nLine 3"
        );
    }

    #[test]
    fn test_unknown_style_error() {
        test_process_err!("{{fakestyle}}TEXT{{/fakestyle}}");
    }

    #[test]
    fn test_style_alias() {
        test_process!("{{mb}}TEST{{/mb}}" => "𝐓𝐄𝐒𝐓");
    }

    #[test]
    fn test_template_with_spaces() {
        test_process!("{{mathbold}}HELLO WORLD{{/mathbold}}" => "𝐇𝐄𝐋𝐋𝐎 𝐖𝐎𝐑𝐋𝐃");
    }

    #[test]
    fn test_template_with_punctuation() {
        test_process!("{{mathbold}}Hello, World!{{/mathbold}}" => "𝐇𝐞𝐥𝐥𝐨, 𝐖𝐨𝐫𝐥𝐝!");
    }

    #[test]
    fn test_mismatched_tags() {
        test_process_err!("{{mathbold}}TEXT{{/italic}}");
    }

    #[test]
    fn test_complex_markdown() {
        test_process_contains!(
            r#"# {{mathbold}}TITLE{{/mathbold}}

This is a {{negative-squared}}WARNING{{/negative-squared}} message.

```rust
let code = "{{mathbold}}not processed{{/mathbold}}";
```

And `{{mathbold}}inline code{{/mathbold}}` is also preserved."#
            => [
                "𝐓𝐈𝐓𝐋𝐄",
                "🆆🅰🆁🅽🅸🅽🅶",
                "{{mathbold}}not processed{{/mathbold}}",
                "`{{mathbold}}inline code{{/mathbold}}`"
            ]
        );
    }

    #[test]
    fn test_hyphenated_style_names() {
        test_process!("{{negative-squared}}TEST{{/negative-squared}}" => "🆃🅴🆂🆃");
    }

    #[test]
    fn test_empty_content() {
        test_process!("{{mathbold}}{{/mathbold}}" => "");
    }

    #[test]
    fn test_adjacent_templates() {
        test_process!("{{mathbold}}A{{/mathbold}}{{italic}}B{{/italic}}" => "𝐀𝐵");
    }

    #[test]
    fn test_template_with_spacing() {
        test_process!("{{mathbold:spacing=1}}HELLO{{/mathbold}}" => "𝐇 𝐄 𝐋 𝐋 𝐎");
    }

    #[test]
    fn test_template_with_spacing_two() {
        test_process!("{{script:spacing=2}}ABC{{/script}}" => "𝒜  ℬ  𝒞");
    }

    #[test]
    fn test_template_mixed_spacing() {
        test_process!(
            "{{mathbold}}no spacing{{/mathbold}} {{mathbold:spacing=1}}with spacing{{/mathbold}}"
            => "𝐧𝐨 𝐬𝐩𝐚𝐜𝐢𝐧𝐠 𝐰 𝐢 𝐭 𝐡   𝐬 𝐩 𝐚 𝐜 𝐢 𝐧 𝐠"
        );
    }

    #[test]
    fn test_template_spacing_with_heading() {
        test_process!("# {{mathbold:spacing=1}}HEADER{{/mathbold}}" => "# 𝐇 𝐄 𝐀 𝐃 𝐄 𝐑");
    }

    #[test]
    fn test_template_spacing_zero() {
        test_process!("{{mathbold:spacing=0}}HELLO{{/mathbold}}" => "𝐇𝐄𝐋𝐋𝐎");
    }

    #[test]
    fn test_template_with_separator_dot() {
        test_process!("{{mathbold:separator=dot}}HELLO{{/mathbold}}" => "𝐇·𝐄·𝐋·𝐋·𝐎");
    }

    #[test]
    fn test_template_with_separator_dash() {
        test_process!("{{mathbold:separator=dash}}HEADER{{/mathbold}}" => "𝐇─𝐄─𝐀─𝐃─𝐄─𝐑");
    }

    #[test]
    fn test_template_with_separator_bolddash() {
        test_process!("{{mathbold:separator=bolddash}}BOLD{{/mathbold}}" => "𝐁━𝐎━𝐋━𝐃");
    }

    #[test]
    fn test_template_with_separator_arrow() {
        test_process!("{{mathbold:separator=arrow}}ABC{{/mathbold}}" => "𝐀→𝐁→𝐂");
    }

    #[test]
    fn test_template_with_separator_bullet() {
        test_process!("{{mathbold:separator=bullet}}TEST{{/mathbold}}" => "𝐓•𝐄•𝐒•𝐓");
    }

    #[test]
    fn test_template_separator_in_heading() {
        test_process!("# {{mathbold:separator=dot}}TITLE{{/mathbold}}" => "# 𝐓·𝐈·𝐓·𝐋·𝐄");
    }

    #[test]
    fn test_template_separator_with_punctuation() {
        test_process!(
            "{{mathbold:separator=dash}}Hello, World!{{/mathbold}}"
            => "𝐇─𝐞─𝐥─𝐥─𝐨─,─ ─𝐖─𝐨─𝐫─𝐥─𝐝─!"
        );
    }

    #[test]
    fn test_template_spacing_and_separator_mutually_exclusive() {
        // When both are specified, separator takes precedence
        test_process!("{{mathbold:spacing=2:separator=dot}}HI{{/mathbold}}" => "𝐇·𝐈");
    }

    #[test]
    fn test_template_unknown_separator_error() {
        test_process_err!("{{mathbold:separator=invalid}}TEST{{/mathbold}}");
    }

    #[test]
    fn test_template_mixed_with_and_without_separator() {
        test_process!(
            "{{mathbold}}no sep{{/mathbold}} {{mathbold:separator=dot}}with sep{{/mathbold}}"
            => "𝐧𝐨 𝐬𝐞𝐩 𝐰·𝐢·𝐭·𝐡· ·𝐬·𝐞·𝐩"
        );
    }

    // Frame template tests
    #[test]
    fn test_frame_template_plain_text() {
        test_process!("{{frame:gradient}}Title{{/frame}}" => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} Title ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}");
    }

    #[test]
    fn test_frame_short_close_tag() {
        test_process!("{{frame:gradient}}Title{{/}}" => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} Title ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}");
    }

    #[test]
    fn test_frame_nested_short_close() {
        test_process!(
            "{{frame:gradient}}{{frame:glyph:star}}NESTED{{/}}{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} NESTED ★\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_template_with_styled_text() {
        test_process!(
            "{{frame:gradient}}{{mathbold}}TITLE{{/mathbold}}{{/frame}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} 𝐓𝐈𝐓𝐋𝐄 ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_with_separator() {
        test_process!(
            "{{frame:solid-left}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}"
            => "█\u{fe0e}▌\u{fe0e}𝐓·𝐈·𝐓·𝐋·𝐄"
        );
    }

    #[test]
    fn test_frame_with_spacing() {
        test_process!(
            "{{frame:gradient}}{{mathbold:spacing=1}}HI{{/mathbold}}{{/frame}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} 𝐇 𝐈 ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_alias() {
        test_process!("{{frame:grad}}Test{{/frame}}" => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} Test ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}");
    }

    #[test]
    fn test_frame_solid_left() {
        test_process!("{{frame:solid-left}}Important{{/frame}}" => "█\u{fe0e}▌\u{fe0e}Important");
    }

    #[test]
    fn test_frame_line_bold() {
        test_process!("{{frame:line-bold}}Section{{/frame}}" => "━\u{fe0e}━\u{fe0e}━\u{fe0e} Section ━\u{fe0e}━\u{fe0e}━\u{fe0e}");
    }

    #[test]
    fn test_multiple_frames_in_line() {
        test_process!(
            "{{frame:solid-left}}A{{/frame}} and {{frame:solid-right}}B{{/frame}}"
            => "█\u{fe0e}▌\u{fe0e}A and B▐\u{fe0e}█\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_in_heading() {
        test_process!(
            "# {{frame:gradient}}{{mathbold}}HEADER{{/mathbold}}{{/frame}}"
            => "# ▓\u{fe0e}▒\u{fe0e}░\u{fe0e} 𝐇𝐄𝐀𝐃𝐄𝐑 ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
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
        test_process!("{{frame:glyph:star}}Title{{/frame}}" => "★\u{fe0e} Title ★\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_shorthand_diamond() {
        test_process!("{{frame:glyph:diamond}}Gem{{/frame}}" => "◆\u{fe0e} Gem ◆\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_shorthand_unknown_glyph() {
        // Keep verbose form - tests specific error variant
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
        test_process!(
            "{{frame:glyph:star*3}}Title{{/frame}}"
            => "★\u{fe0e}★\u{fe0e}★\u{fe0e} Title ★\u{fe0e}★\u{fe0e}★\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_glyph_multiplier_with_tight_padding() {
        test_process!(
            "{{frame:glyph:star*3/pad=0}}Title{{/frame}}"
            => "★\u{fe0e}★\u{fe0e}★\u{fe0e}Title★\u{fe0e}★\u{fe0e}★\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_glyph_multiplier_with_spaces() {
        test_process!("{{frame:glyph:star*2/pad=3}}Title{{/frame}}" => "★\u{fe0e}★\u{fe0e}   Title   ★\u{fe0e}★\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_custom_padding() {
        test_process!("{{frame:glyph:diamond*2/pad=-}}Title{{/frame}}" => "◆\u{fe0e}◆\u{fe0e}-Title-◆\u{fe0e}◆\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_unicode_padding() {
        test_process!("{{frame:glyph:star*2/pad=·}}Title{{/frame}}" => "★\u{fe0e}★\u{fe0e}·Title·★\u{fe0e}★\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_multi_char_padding() {
        test_process!("{{frame:glyph:star/pad=--}}Title{{/frame}}" => "★\u{fe0e}--Title--★\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_with_separator() {
        test_process!(
            "{{frame:glyph:star*3/separator=dot}}Title{{/frame}}"
            => "★\u{fe0e}·★\u{fe0e}·★\u{fe0e} Title ★\u{fe0e}·★\u{fe0e}·★\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_glyph_with_separator_named() {
        test_process!("{{frame:glyph:diamond*2/separator=dash}}Gem{{/frame}}" => "◆\u{fe0e}─◆\u{fe0e} Gem ◆\u{fe0e}─◆\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_with_separator_literal() {
        test_process!(
            "{{frame:glyph:bullet*4/separator=-}}X{{/frame}}"
            => "•\u{fe0e}-•\u{fe0e}-•\u{fe0e}-•\u{fe0e} X •\u{fe0e}-•\u{fe0e}-•\u{fe0e}-•\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_glyph_separator_and_pad() {
        // Both separator and pad modifiers
        test_process!(
            "{{frame:glyph:star*3/separator=·/pad=0}}Tight{{/frame}}"
            => "★\u{fe0e}·★\u{fe0e}·★\u{fe0e}Tight★\u{fe0e}·★\u{fe0e}·★\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_glyph_separator_single_count() {
        // With count=1, separator has no effect (nothing to separate)
        test_process!("{{frame:glyph:star/separator=dot}}Single{{/frame}}" => "★\u{fe0e} Single ★\u{fe0e}");
    }

    #[test]
    fn test_frame_glyph_max_count() {
        // Count should be capped at 20 (20 bullets on each side + space padding)
        test_process!(
            "{{frame:glyph:bullet*100}}X{{/frame}}"
            => "•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e} X •\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}•\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_fr_shorthand() {
        test_process!("{{fr:gradient}}Title{{/}}" => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} Title ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}");
    }

    #[test]
    fn test_frame_fr_shorthand_with_glyph() {
        test_process!(
            "{{fr:glyph:star*3}}Text{{/}}"
            => "★\u{fe0e}★\u{fe0e}★\u{fe0e} Text ★\u{fe0e}★\u{fe0e}★\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_pattern_with_separator() {
        // gradient pattern is ▓▒░, with separator should be ▓·▒·░
        test_process!(
            "{{fr:gradient/separator=dot}}Title{{/}}"
            => "▓\u{fe0e}·▒\u{fe0e}·░\u{fe0e} Title ░\u{fe0e}·▒\u{fe0e}·▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_pattern_with_separator_named() {
        test_process!(
            "{{frame:gradient/separator=dash}}TEXT{{/frame}}"
            => "▓\u{fe0e}─▒\u{fe0e}─░\u{fe0e} TEXT ░\u{fe0e}─▒\u{fe0e}─▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_pattern_with_separator_literal() {
        test_process!(
            "{{fr:line-double/separator= }}Title{{/}}"
            => "═\u{fe0e} ═\u{fe0e} ═\u{fe0e} Title ═\u{fe0e} ═\u{fe0e} ═\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_pattern_with_spacing() {
        // spacing=1 adds 1 space between each grapheme
        test_process!(
            "{{fr:gradient/spacing=1}}TITLE{{/}}"
            => "▓\u{fe0e} ▒\u{fe0e} ░\u{fe0e} TITLE ░\u{fe0e} ▒\u{fe0e} ▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_pattern_with_spacing_two() {
        // spacing=2 adds 2 spaces between each grapheme
        test_process!(
            "{{fr:gradient/spacing=2}}X{{/}}"
            => "▓\u{fe0e}  ▒\u{fe0e}  ░\u{fe0e} X ░\u{fe0e}  ▒\u{fe0e}  ▓\u{fe0e}"
        );
    }

    #[test]
    fn test_glyph_frame_with_spacing() {
        test_process!(
            "{{fr:glyph:star*3/spacing=1}}Text{{/}}"
            => "★\u{fe0e} ★\u{fe0e} ★\u{fe0e} Text ★\u{fe0e} ★\u{fe0e} ★\u{fe0e}"
        );
    }

    #[test]
    fn test_glyph_frame_with_spacing_two() {
        test_process!("{{fr:glyph:diamond*2/spacing=2}}Gem{{/}}" => "◆\u{fe0e}  ◆\u{fe0e} Gem ◆\u{fe0e}  ◆\u{fe0e}");
    }

    #[test]
    fn test_frame_alternate_mode() {
        // gradient-wave uses alternate mode: ▓▒░ → ▒░▓ (rotated)
        test_process!(
            "{{fr:gradient-wave}}TITLE{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} TITLE ▒\u{fe0e}░\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_alternate_mode_with_alias() {
        test_process!(
            "{{fr:wave}}TEXT{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} TEXT ▒\u{fe0e}░\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_combo() {
        // gradient+star: outer prefix + inner prefix + content + inner suffix + outer suffix
        test_process!(
            "{{fr:gradient+star}}TITLE{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} TITLE ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_combo_three() {
        // gradient: ▓▒░  + star: ★  + diamond: ◆  + X + ◇  + ☆  + ░▒▓
        test_process!(
            "{{fr:gradient+star+diamond}}X{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} ◆\u{fe0e} X ◇\u{fe0e} ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_combo_with_spaces() {
        test_process!(
            "{{fr:gradient + star}}TEXT{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} TEXT ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_self_closing_with_separator() {
        test_process!(
            "{{fr:gradient/separator=·:Inline/}}"
            => "▓\u{fe0e}·▒\u{fe0e}·░\u{fe0e} Inline ░\u{fe0e}·▒\u{fe0e}·▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_reverse() {
        // Reverse gradient: swap prefix and suffix
        // Normal: ▓▒░ Title ░▒▓, Reversed: ░▒▓ Title ▓▒░
        test_process!(
            "{{fr:gradient/reverse}}Title{{/}}"
            => " ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}Title▓\u{fe0e}▒\u{fe0e}░\u{fe0e} "
        );
    }

    #[test]
    fn test_frame_reverse_star() {
        // Reverse star: swap ★ and ☆
        // Normal: ★ VIP ☆, Reversed: ☆ VIP ★ (with spacing swap)
        test_process!("{{fr:star/reverse}}VIP{{/}}" => " ☆\u{fe0e}VIP★\u{fe0e} ");
    }

    #[test]
    fn test_frame_count() {
        // Repeat star 3 times
        test_process!(
            "{{fr:star*3}}Title{{/}}"
            => "★\u{fe0e}★\u{fe0e}★\u{fe0e} Title ☆\u{fe0e}☆\u{fe0e}☆\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_count_gradient() {
        // Repeat gradient 2 times
        test_process!(
            "{{fr:gradient*2}}X{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e}▓\u{fe0e}▒\u{fe0e}░\u{fe0e} X ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_count_and_reverse() {
        // Repeat star 2 times then reverse
        // Count first: ★★ Title ☆☆, then reverse: ☆☆ Title ★★ (with spacing)
        test_process!("{{fr:star*2/reverse}}Title{{/}}" => " ☆\u{fe0e}☆\u{fe0e}Title★\u{fe0e}★\u{fe0e} ");
    }

    #[test]
    fn test_frame_count_with_separator() {
        // ★★★ with separator between graphemes: ★·★·★
        test_process!(
            "{{fr:star*3/separator=·}}Title{{/}}"
            => "★\u{fe0e}·★\u{fe0e}·★\u{fe0e} Title ☆\u{fe0e}·☆\u{fe0e}·☆\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_fr_nested() {
        test_process!(
            "{{fr:gradient}}{{fr:star}}NESTED{{/}}{{/}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} NESTED ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_fr_mixed_with_full_frame() {
        test_process!(
            "{{frame:gradient}}{{fr:star}}MIXED{{/}}{{/frame}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} MIXED ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_close_all() {
        test_process!(
            "{{fr:gradient}}{{fr:star}}NESTED{{//}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} NESTED ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_close_all_three_levels() {
        test_process!(
            "{{fr:gradient}}{{fr:star}}{{fr:lenticular}}DEEP{{//}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} 【\u{fe0e}DEEP】\u{fe0e} ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_close_all_single_frame() {
        // {{//}} on single frame should work same as {{/}}
        test_process!(
            "{{fr:gradient}}Title{{//}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} Title ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_close_all_with_content_between() {
        // The {{//}} closes both frames, leaving " end" outside
        test_process!(
            "{{fr:gradient}}Outer {{fr:star}}Inner{{//}} end"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} Outer ★\u{fe0e} Inner ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e} end"
        );
    }

    #[test]
    fn test_expand_close_all_styles() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold}}{{italic}}TEXT{{//}}";
        let expanded = parser.expand_close_all(input);
        // Should expand to close both styles in reverse order
        assert_eq!(expanded, "{{mathbold}}{{italic}}TEXT{{/italic}}{{/mathbold}}");
    }

    #[test]
    fn test_expand_close_all_mixed() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{mathbold}}TEXT{{//}}";
        let expanded = parser.expand_close_all(input);
        // Should expand to close style first, then frame
        assert_eq!(expanded, "{{fr:gradient}}{{mathbold}}TEXT{{/mathbold}}{{/}}");
    }

    #[test]
    fn test_universal_close_all_frames_and_style() {
        // Frame > Frame > Style, all closed by {{//}}
        test_process!(
            "{{fr:gradient}}{{fr:star}}{{mathbold}}VIP{{//}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} ★\u{fe0e} 𝐕𝐈𝐏 ☆\u{fe0e} ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_universal_close_all_preserves_partial_closes() {
        // Style is explicitly closed, only frame left for {{//}}
        test_process!(
            "{{fr:gradient}}{{mathbold}}TITLE{{/mathbold}} more text{{//}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} 𝐓𝐈𝐓𝐋𝐄 more text ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_universal_close_all_self_closing_ignored() {
        // Self-closing tags should not be tracked - only the frame should be closed
        let parser = TemplateParser::new().unwrap();
        let input = "{{fr:gradient}}{{ui:swatch:pink/}}text{{//}}";
        let result = parser.process(input).unwrap();
        assert!(result.contains("▓\u{fe0e}▒\u{fe0e}░\u{fe0e}"));
        assert!(result.contains("░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"));
    }

    #[test]
    fn test_frame_self_closing_basic() {
        test_process!("{{fr:gradient:Title/}}" => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} Title ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}");
    }

    #[test]
    fn test_frame_self_closing_star() {
        test_process!("{{fr:star:VIP/}}" => "★\u{fe0e} VIP ☆\u{fe0e}");
    }

    #[test]
    fn test_frame_self_closing_glyph() {
        test_process!("{{fr:glyph:diamond*2:Gem/}}" => "◆\u{fe0e}◆\u{fe0e} Gem ◆\u{fe0e}◆\u{fe0e}");
    }

    #[test]
    fn test_frame_self_closing_glyph_with_padding() {
        test_process!(
            "{{fr:glyph:star*3/pad=0:Tight/}}"
            => "★\u{fe0e}★\u{fe0e}★\u{fe0e}Tight★\u{fe0e}★\u{fe0e}★\u{fe0e}"
        );
    }

    #[test]
    fn test_frame_self_closing_full_syntax() {
        test_process!("{{frame:solid-left:Note/}}" => "█\u{fe0e}▌\u{fe0e}Note");
    }

    #[test]
    fn test_frame_self_closing_in_sentence() {
        test_process!("Check this {{fr:star:TIP/}} out!" => "Check this ★\u{fe0e} TIP ☆\u{fe0e} out!");
    }

    #[test]
    fn test_frame_preserves_code_blocks() {
        test_process_unchanged!("```\n{{frame:gradient}}CODE{{/frame}}\n```");
    }

    #[test]
    fn test_frame_preserves_inline_code() {
        test_process_unchanged!("Text `{{frame:gradient}}code{{/frame}}` more");
    }

    #[test]
    fn test_composition_frame_style_separator() {
        test_process!(
            "{{frame:gradient}}{{mathbold:separator=dash}}STYLED{{/mathbold}}{{/frame}}"
            => "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} 𝐒─𝐓─𝐘─𝐋─𝐄─𝐃 ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"
        );
    }

    #[test]
    fn test_composition_multiple_styles_in_frame() {
        test_process!(
            "{{frame:solid-both}}{{mathbold}}A{{/mathbold}} and {{italic}}B{{/italic}}{{/frame}}"
            => "█\u{fe0e}▌\u{fe0e}𝐀 and 𝐵▐\u{fe0e}█\u{fe0e}"
        );
    }

    #[test]
    fn test_complex_composition() {
        test_process_contains!(
            r#"# {{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}

{{frame:solid-left}}{{italic}}Important note{{/italic}}{{/frame}}

Regular text with {{mathbold:spacing=1}}spacing{{/mathbold}}"#
            => [
                "▓\u{fe0e}▒\u{fe0e}░\u{fe0e} 𝐓·𝐈·𝐓·𝐋·𝐄 ░\u{fe0e}▒\u{fe0e}▓\u{fe0e}",
                "█\u{fe0e}▌\u{fe0e}𝐼𝑚𝑝𝑜𝑟𝑡𝑎𝑛𝑡 𝑛𝑜𝑡𝑒",
                "𝐬 𝐩 𝐚 𝐜 𝐢 𝐧 𝐠"
            ]
        );
    }

    // UI Component Tests

    #[test]
    fn test_ui_swatch() {
        // Should expand to shields:block with pink color resolved
        test_process_contains!("{{ui:swatch:pink/}}" => ["![](", "F41C80"]);
    }

    #[test]
    fn test_ui_tech() {
        // Should expand to shields:icon with logo
        test_process_contains!("{{ui:tech:rust/}}" => ["![](", "logo=rust"]);
    }

    #[test]
    fn test_ui_multiple_inline() {
        // Should have two shields with different logos
        test_process_contains!("{{ui:tech:rust/}} {{ui:tech:python/}}" => ["logo=rust", "logo=python"]);
    }

    #[test]
    fn test_ui_in_markdown() {
        test_process_contains!("# Header\n\n{{ui:swatch:pink/}}\n\n## Section" => ["# Header", "![](", "## Section"]);
    }

    #[test]
    fn test_ui_unknown_component() {
        // Keep verbose - tests specific error message content
        let parser = TemplateParser::new().unwrap();
        let result = parser.process("{{ui:nonexistent/}}");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown component"));
    }

    #[test]
    fn test_ui_unclosed() {
        test_process_err!("{{ui:row}}TITLE");
    }

    #[test]
    fn test_frame_multiline() {
        test_process_bookends!(
            "{{frame:gradient}}\nLine 1\nLine 2\n{{/frame}}"
            => starts "▓\u{fe0e}▒\u{fe0e}░\u{fe0e}", ends "░\u{fe0e}▒\u{fe0e}▓\u{fe0e}",
            contains ["Line 1", "Line 2"]
        );
    }

    #[test]
    fn test_frame_multiline_with_styles() {
        test_process_contains!(
            "{{frame:solid-left}}\n### {{mathbold}}Title{{/mathbold}}\nContent\n{{/frame}}"
            => ["█\u{fe0e}▌\u{fe0e}", "𝐓𝐢𝐭𝐥𝐞", "Content"]
        );
    }

    #[test]
    fn test_frame_multiline_with_ui_components() {
        use crate::renderer::svg::SvgBackend;

        let parser =
            TemplateParser::with_backend(Box::new(SvgBackend::new("assets/test"))).unwrap();
        let input = "{{frame:gradient}}\n{{ui:swatch:pink/}}\n{{ui:swatch:success/}}\n{{/frame}}";
        let result = parser.process_with_assets(input).unwrap();

        // Should process frame correctly
        assert!(result.markdown.starts_with("▓\u{fe0e}▒\u{fe0e}░\u{fe0e}"));
        assert!(result.markdown.ends_with("░\u{fe0e}▒\u{fe0e}▓\u{fe0e}"));

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
        let input = "```\n{{ui:swatch:pink/}}\n```\n{{ui:swatch:pink/}}";
        let result = parser.process_with_assets(input).unwrap();

        // Code block should be preserved
        assert!(result.markdown.contains("```\n{{ui:swatch:pink/}}\n```"));

        // Only one asset generated (outside code block)
        assert_eq!(result.assets.len(), 1);
    }

    #[test]
    fn test_shields_primitive_block() {
        // Should render shields directly (cobalt is in shields.json palette)
        test_process_contains!(
            "{{shields:block:color=cobalt:style=flat-square/}}"
            => ["![](", "2B6CB0"]
        );
    }

    #[test]
    fn test_shields_primitive_bar() {
        // Should render 3 inline badges - keep verbose for count check
        let parser = TemplateParser::new().unwrap();
        let result = parser.process("{{shields:bar:colors=success,warning,error:style=flat-square/}}").unwrap();
        assert_eq!(result.matches("![](").count(), 3);
    }

    #[test]
    fn test_shields_bar_with_separator() {
        // Should render 2 badges with space between them
        test_process_contains!(
            "{{shields:bar:colors=success,warning:style=flat-square:separator= /}}"
            => [") ![]("]
        );
    }

    #[test]
    fn test_shields_bar_with_named_separator() {
        // Should render 2 badges with · separator
        test_process_contains!(
            "{{shields:bar:colors=pink,success:style=flat-square:separator=dot/}}"
            => [")·![]("]
        );
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
        let input = "Paragraph 1\n\n{{ui:swatch:pink/}}\n\nParagraph 2";
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
        // Should preserve indentation in nested lists
        test_process_contains!(
            "- Outer\n  - {{italic}}Nested{{/italic}} item\n  - Another nested"
            => ["  - 𝑁𝑒𝑠𝑡𝑒𝑑 item\n", "  - Another nested"]
        );
    }

    #[test]
    fn test_multiline_component_content_preserves_structure() {
        // Content should be processed but structure preserved
        test_process_contains!(
            "{{frame:gradient}}Multi\nLine\nTitle{{/frame}}"
            => ["▓\u{fe0e}▒\u{fe0e}░\u{fe0e}"]
        );
    }

    #[test]
    fn test_adjacent_components_no_extra_whitespace() {
        // Should preserve single newline (not add extra)
        test_process!(
            "{{mathbold}}FIRST{{/mathbold}}\n{{mathbold}}SECOND{{/mathbold}}"
            => "𝐅𝐈𝐑𝐒𝐓\n𝐒𝐄𝐂𝐎𝐍𝐃"
        );
    }

    #[test]
    fn test_component_in_blockquote_preserves_prefix() {
        // Should preserve the "> " prefix
        test_process_bookends!(
            "> Quote with {{mathbold}}BOLD{{/mathbold}} text"
            => starts "> ", ends "text",
            contains ["𝐁𝐎𝐋𝐃"]
        );
    }

    #[test]
    fn test_component_with_trailing_newline() {
        // Should preserve trailing newline - keep verbose for ends_with check
        let parser = TemplateParser::new().unwrap();
        let result = parser.process("Text before\n{{ui:swatch:pink/}}\n").unwrap();
        assert!(result.ends_with('\n'), "Trailing newline was lost");
    }

    #[test]
    fn test_component_without_trailing_newline() {
        // Should NOT add trailing newline - keep verbose for ends_with check
        let parser = TemplateParser::new().unwrap();
        let result = parser.process("Text before\n{{ui:swatch:pink/}}").unwrap();
        assert!(!result.ends_with('\n'), "Unexpected trailing newline added");
    }

    #[test]
    fn test_component_expansion_preserves_empty_lines_in_content() {
        // Empty line in content should be preserved
        test_process_contains!(
            "{{frame:solid-left}}Line 1\n\nLine 3{{/frame}}"
            => ["█\u{fe0e}▌\u{fe0e}"]
        );
    }
}

#[cfg(test)]
mod badge_style_tests {
    use super::*;
    use crate::test_process_contains;

    #[test]
    fn test_swatch_with_flat_style() {
        // Note: style=flat should NOT contain style=flat-square (keep verbose)
        let parser = TemplateParser::new().unwrap();
        let output = parser.process("{{ui:swatch:F41C80:style=flat/}}").unwrap();
        assert!(output.contains("style=flat"));
        assert!(!output.contains("style=flat-square"));
    }

    #[test]
    fn test_swatch_with_flat_square_style() {
        test_process_contains!("{{ui:swatch:F41C80:style=flat-square/}}" => ["style=flat-square"]);
    }

    #[test]
    fn test_swatch_with_for_the_badge_style() {
        test_process_contains!("{{ui:swatch:F41C80:style=for-the-badge/}}" => ["style=for-the-badge"]);
    }

    #[test]
    fn test_swatch_with_plastic_style() {
        test_process_contains!("{{ui:swatch:F41C80:style=plastic/}}" => ["style=plastic"]);
    }

    #[test]
    fn test_swatch_with_social_style() {
        test_process_contains!("{{ui:swatch:F41C80:style=social/}}" => ["style=social"]);
    }

    #[test]
    fn test_swatch_default_style() {
        // Should default to flat-square
        test_process_contains!("{{ui:swatch:F41C80/}}" => ["style=flat-square"]);
    }

    #[test]
    fn test_tech_with_style() {
        test_process_contains!("{{ui:tech:rust:style=plastic/}}" => ["style=plastic", "logo=rust"]);
    }

    #[test]
    fn test_multiple_components_different_styles() {
        test_process_contains!(
            "{{ui:swatch:FF0000:style=flat/}} {{ui:swatch:00FF00:style=for-the-badge/}}"
            => ["style=flat", "style=for-the-badge"]
        );
    }

    #[test]
    fn test_style_with_palette_color() {
        // Should resolve pink color
        test_process_contains!("{{ui:swatch:pink:style=plastic/}}" => ["style=plastic", "F41C80"]);
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
        assert!(result.contains("▓\u{fe0e}▒\u{fe0e}░\u{fe0e}"));
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
