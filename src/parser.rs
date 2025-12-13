use crate::badges::BadgeRenderer;
use crate::components::{ComponentOutput, ComponentsRenderer};
use crate::converter::Converter;
use crate::error::{Error, Result};
use crate::frames::FrameRenderer;
use crate::renderer::shields::ShieldsBackend;
use crate::renderer::{RenderedAsset, Renderer};
use crate::shields::ShieldsRenderer;

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

/// Badge template data
#[derive(Debug, Clone)]
struct BadgeData {
    end_pos: usize,
    badge_type: String,
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
    frame_renderer: FrameRenderer,
    badge_renderer: BadgeRenderer,
    components_renderer: ComponentsRenderer,
    shields_renderer: ShieldsRenderer, // Keep for {{shields:*}} escape hatch
    backend: Box<dyn Renderer>,        // Pluggable rendering backend
}

impl TemplateParser {
    /// Create a new template parser with default (shields.io) backend
    pub fn new() -> Result<Self> {
        Self::with_backend(Box::new(ShieldsBackend::new()?))
    }

    /// Create a template parser with a custom backend
    pub fn with_backend(backend: Box<dyn Renderer>) -> Result<Self> {
        let converter = Converter::new()?;
        let frame_renderer = FrameRenderer::new()?;
        let badge_renderer = BadgeRenderer::new()?;
        let components_renderer = ComponentsRenderer::new()?;
        let shields_renderer = ShieldsRenderer::new()?;
        Ok(Self {
            converter,
            frame_renderer,
            badge_renderer,
            components_renderer,
            shields_renderer,
            backend,
        })
    }

    /// Process markdown text, converting all style templates
    ///
    /// Returns only the markdown string. File-based assets are not collected.
    /// Use `process_with_assets()` if you need to write SVG files.
    ///
    /// # Example
    ///
    /// ```
    /// use utf8fx::TemplateParser;
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
    /// use utf8fx::TemplateParser;
    /// use utf8fx::renderer::svg::SvgBackend;
    ///
    /// let backend = Box::new(SvgBackend::new("assets"));
    /// let parser = TemplateParser::with_backend(backend).unwrap();
    /// let processed = parser.process_with_assets("{{ui:divider/}}").unwrap();
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
        // Track if we're in a code block to skip processing
        let mut in_code_block = false;
        let mut result = String::new();
        let mut all_assets = Vec::new();

        for line in markdown.lines() {
            let trimmed = line.trim();

            // Track code block state
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                result.push_str(line);
                result.push('\n');
                continue;
            }

            // Skip processing inside code blocks
            if in_code_block {
                result.push_str(line);
                result.push('\n');
                continue;
            }

            // Process the line, handling inline code
            let (processed, assets) = self.process_line_with_assets(line)?;
            result.push_str(&processed);
            result.push('\n');
            all_assets.extend(assets);
        }

        // Remove trailing newline if original didn't have one
        if !markdown.ends_with('\n') && result.ends_with('\n') {
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
        let mut result = String::new();
        let mut assets = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Look for opening tag {{ (could be ui, frame, badge, or style template)
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '{' {
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
                    }

                    // Skip past the UI template
                    i = ui_data.end_pos;
                    continue;
                }

                // Try to parse a frame template
                if let Some(frame_data) = self.parse_frame_at(&chars, i)? {
                    // Validate frame exists
                    if !self.frame_renderer.has_frame(&frame_data.frame_style) {
                        return Err(Error::UnknownFrame(frame_data.frame_style));
                    }

                    // Process content recursively (may contain style templates and primitives)
                    let (processed_content, nested_assets) =
                        self.process_templates_with_assets(&frame_data.content)?;
                    assets.extend(nested_assets);

                    // Apply frame to processed content
                    let framed = self
                        .frame_renderer
                        .apply_frame(&processed_content, &frame_data.frame_style)?;
                    result.push_str(&framed);

                    // Skip past the frame template
                    i = frame_data.end_pos;
                    continue;
                }

                // Try to parse a badge template
                if let Some(badge_data) = self.parse_badge_at(&chars, i)? {
                    // Validate badge exists
                    if !self.badge_renderer.has_badge(&badge_data.badge_type) {
                        return Err(Error::UnknownBadge(badge_data.badge_type));
                    }

                    // Apply badge to content (badges don't support recursive processing)
                    let badged = self
                        .badge_renderer
                        .apply_badge(&badge_data.content, &badge_data.badge_type)?;
                    result.push_str(&badged);

                    // Skip past the badge template
                    i = badge_data.end_pos;
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
                            self.shields_renderer.render_bar(&colors, style)?
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

                // Try to parse a style template
                if let Some(template_data) = self.parse_template_at(&chars, i)? {
                    // Validate style exists
                    if !self.converter.has_style(&template_data.style) {
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

                // Parse separator name (letters only)
                let mut sep_name = String::new();
                while i < chars.len() && chars[i].is_alphabetic() {
                    sep_name.push(chars[i]);
                    i += 1;
                }

                // Map separator name to Unicode character
                separator = Some(
                    match sep_name.as_str() {
                        "dot" => "·",
                        "bullet" => "•",
                        "dash" => "─",
                        "bolddash" => "━",
                        "arrow" => "→",
                        _ => {
                            return Err(Error::ParseError(format!(
                            "Unknown separator '{}'. Available: dot, bullet, dash, bolddash, arrow",
                            sep_name
                        )))
                        }
                    }
                    .to_string(),
                );
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

    /// Try to parse a frame template starting at position i
    /// Returns: Some(FrameData) or None if not a valid frame template
    fn parse_frame_at(&self, chars: &[char], start: usize) -> Result<Option<FrameData>> {
        let mut i = start;

        // Must start with {{frame:
        if i + 8 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{frame:"
        let frame_start = "{{frame:";
        let frame_chars: Vec<char> = frame_start.chars().collect();
        for (idx, &expected) in frame_chars.iter().enumerate() {
            if chars[i + idx] != expected {
                return Ok(None);
            }
        }
        i += frame_chars.len();

        // Parse frame style name (alphanumeric and hyphens)
        let mut frame_style = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() || ch == '-' {
                frame_style.push(ch);
                i += 1;
            } else if ch == '}' {
                break;
            } else {
                // Invalid character in frame style name
                return Ok(None);
            }
        }

        // Frame style must be non-empty
        if frame_style.is_empty() {
            return Ok(None);
        }

        // Must have closing }} for opening tag
        if i + 1 >= chars.len() || chars[i] != '}' || chars[i + 1] != '}' {
            return Ok(None);
        }
        i += 2;

        let content_start = i;

        // Find closing tag {{/frame}}
        let close_tag = "{{/frame}}";
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
                    return Ok(Some(FrameData {
                        end_pos,
                        frame_style,
                        content,
                    }));
                }
            }

            i += 1;
        }

        // No closing tag found
        Err(Error::UnclosedTag("frame".to_string()))
    }

    /// Try to parse a badge template starting at position i
    /// Returns: Some(BadgeData) or None if not a valid badge template
    fn parse_badge_at(&self, chars: &[char], start: usize) -> Result<Option<BadgeData>> {
        let mut i = start;

        // Must start with {{badge:
        if i + 8 >= chars.len() {
            return Ok(None);
        }

        // Check for "{{badge:"
        let badge_start = "{{badge:";
        let badge_chars: Vec<char> = badge_start.chars().collect();
        for (idx, &expected) in badge_chars.iter().enumerate() {
            if chars[i + idx] != expected {
                return Ok(None);
            }
        }
        i += badge_chars.len();

        // Parse badge type name (alphanumeric and hyphens)
        let mut badge_type = String::new();
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() || ch == '-' {
                badge_type.push(ch);
                i += 1;
            } else if ch == '}' {
                break;
            } else {
                // Invalid character in badge type name
                return Ok(None);
            }
        }

        // Badge type must be non-empty
        if badge_type.is_empty() {
            return Ok(None);
        }

        // Must have closing }} for opening tag
        if i + 1 >= chars.len() || chars[i] != '}' || chars[i + 1] != '}' {
            return Ok(None);
        }
        i += 2;

        let content_start = i;

        // Find closing tag {{/badge}}
        let close_tag = "{{/badge}}";
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
                    return Ok(Some(BadgeData {
                        end_pos,
                        badge_type,
                        content,
                    }));
                }
            }

            i += 1;
        }

        // No closing tag found
        Err(Error::UnclosedTag("badge".to_string()))
    }

    /// Try to parse a UI component template starting at position i
    /// Returns: Some(UIData) or None if not a valid UI template
    ///
    /// Supports both self-closing and block-style:
    /// - Self-closing: {{ui:divider/}}
    /// - Block: {{ui:header}}CONTENT{{/ui}}
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
            let mut arg = String::new();
            while i < chars.len() {
                let ch = chars[i];
                if ch == ':' || ch == '}' || ch == '/' {
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
            assert!(msg.contains("Unknown separator"));
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

    // Badge template tests
    #[test]
    fn test_badge_circle() {
        let parser = TemplateParser::new().unwrap();
        let input = "Step {{badge:circle}}1{{/badge}}: Install";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "Step ①: Install");
    }

    #[test]
    fn test_badge_circle_multi_digit() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:circle}}10{{/badge}} items";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "⑩ items");
    }

    #[test]
    fn test_badge_paren() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:paren}}5{{/badge}} points";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "⑸ points");
    }

    #[test]
    fn test_badge_paren_letter() {
        let parser = TemplateParser::new().unwrap();
        let input = "Option {{badge:paren-letter}}a{{/badge}}: Yes";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "Option ⒜: Yes");
    }

    #[test]
    fn test_badge_negative_circle() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:negative-circle}}3{{/badge}} warnings";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "❸ warnings");
    }

    #[test]
    fn test_badge_double_circle() {
        let parser = TemplateParser::new().unwrap();
        let input = "Priority {{badge:double-circle}}1{{/badge}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "Priority ⓵");
    }

    #[test]
    fn test_badge_period() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:period}}7{{/badge}} days";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "🄇 days");
    }

    #[test]
    fn test_badge_alias() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:circled}}2{{/badge}} items";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "② items");
    }

    #[test]
    fn test_badge_unknown() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:invalid}}1{{/badge}}";
        let result = parser.process(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_badge_unsupported_char() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:circle}}99{{/badge}}";
        let result = parser.process(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_badge_multiple() {
        let parser = TemplateParser::new().unwrap();
        let input =
            "{{badge:circle}}1{{/badge}} {{badge:circle}}2{{/badge}} {{badge:circle}}3{{/badge}}";
        let result = parser.process(input).unwrap();
        assert_eq!(result, "① ② ③");
    }

    #[test]
    fn test_badge_in_markdown() {
        let parser = TemplateParser::new().unwrap();
        let input = "# Steps\n\n{{badge:circle}}1{{/badge}} First step\n{{badge:circle}}2{{/badge}} Second step";
        let result = parser.process(input).unwrap();
        assert!(result.contains("① First step"));
        assert!(result.contains("② Second step"));
    }

    #[test]
    fn test_badge_with_styled_text() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:circle}}1{{/badge}} {{mathbold}}Important{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert!(result.contains("① 𝐈𝐦𝐩𝐨𝐫𝐭𝐚𝐧𝐭"));
    }

    #[test]
    fn test_badge_unclosed() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{badge:circle}}1";
        let result = parser.process(input);
        assert!(result.is_err());
    }

    // UI Component Tests

    #[test]
    fn test_ui_divider() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:divider/}}";
        let result = parser.process(input).unwrap();
        // Should expand to shields:bar and render as Markdown image
        assert!(result.contains("![]("));
        assert!(result.contains("img.shields.io"));
    }

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
    fn test_ui_status() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:status:success/}}";
        let result = parser.process(input).unwrap();
        // Should expand to shields:block with success color
        assert!(result.contains("![]("));
        assert!(result.contains("22C55E")); // success color (uppercased)
    }

    #[test]
    fn test_ui_header_with_content() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:header}}TITLE{{/ui}}";
        let result = parser.process(input).unwrap();
        // Should expand to frame+mathbold and render
        assert!(result.contains("▓▒░")); // gradient frame prefix
        assert!(result.contains("𝐓")); // mathbold T
    }

    #[test]
    fn test_ui_callout_with_content() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{ui:callout:warning}}Breaking change{{/ui}}";
        let result = parser.process(input).unwrap();
        // Should have frame + shield + content
        assert!(result.contains("Breaking change"));
        assert!(result.contains("![]("));
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
        let input = "# Header\n\n{{ui:divider/}}\n\n## Section";
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
        let input = "{{ui:header}}TITLE";
        let result = parser.process(input);
        assert!(result.is_err());
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
}
