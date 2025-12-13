use crate::converter::Converter;
use crate::error::{Error, Result};

/// Parser for processing markdown with style templates
pub struct TemplateParser {
    converter: Converter,
}

impl TemplateParser {
    /// Create a new template parser
    pub fn new() -> Result<Self> {
        let converter = Converter::new()?;
        Ok(Self { converter })
    }

    /// Process markdown text, converting all style templates
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
        // Track if we're in a code block to skip processing
        let mut in_code_block = false;
        let mut result = String::new();

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
            let processed = self.process_line(line)?;
            result.push_str(&processed);
            result.push('\n');
        }

        // Remove trailing newline if original didn't have one
        if !markdown.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }

        Ok(result)
    }

    /// Process a single line, handling inline code markers
    fn process_line(&self, line: &str) -> Result<String> {
        // Split by backticks to separate inline code from regular text
        let parts: Vec<&str> = line.split('`').collect();

        let mut result = String::new();

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                // Add back the backtick separator
                result.push('`');
            }

            // Odd indices are inside inline code, even indices are outside
            if i % 2 == 0 {
                // Outside inline code - process templates
                let processed = self.process_templates(part)?;
                result.push_str(&processed);
            } else {
                // Inside inline code - preserve as-is
                result.push_str(part);
            }
        }

        Ok(result)
    }

    /// Process templates in a text segment using state machine
    ///
    /// This uses a character-by-character state machine parser instead of regex
    /// for better performance and error messages.
    fn process_templates(&self, text: &str) -> Result<String> {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Look for opening tag {{style}}
            if i + 1 < chars.len() && chars[i] == '{' && chars[i + 1] == '{' {
                // Try to parse a complete template
                if let Some((template_end, style, spacing, content)) =
                    self.parse_template_at(&chars, i)?
                {
                    // Validate style exists
                    if !self.converter.has_style(&style) {
                        return Err(Error::UnknownStyle(style));
                    }

                    // Convert and add the styled content with spacing
                    let converted = self
                        .converter
                        .convert_with_spacing(&content, &style, spacing)?;
                    result.push_str(&converted);

                    // Skip past the template
                    i = template_end;
                    continue;
                }
            }

            // Not a template (or invalid), add character as-is
            result.push(chars[i]);
            i += 1;
        }

        Ok(result)
    }

    /// Try to parse a template starting at position i
    /// Returns: Some((end_position, style_name, spacing, content)) or None if not a valid template
    fn parse_template_at(
        &self,
        chars: &[char],
        start: usize,
    ) -> Result<Option<(usize, String, usize, String)>> {
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

        // Parse optional spacing parameter: :spacing=N
        let mut spacing = 0;
        if i < chars.len() && chars[i] == ':' {
            i += 1; // skip ':'

            // Expect "spacing="
            let spacing_str = "spacing=";
            let spacing_chars: Vec<char> = spacing_str.chars().collect();

            // Check if we have "spacing="
            if i + spacing_chars.len() <= chars.len() {
                let mut matches = true;
                for (idx, &expected) in spacing_chars.iter().enumerate() {
                    if chars[i + idx] != expected {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    i += spacing_chars.len();

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
                } else {
                    // Invalid parameter syntax
                    return Ok(None);
                }
            } else {
                // Invalid parameter syntax
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
                    let end = i + close_chars.len();
                    return Ok(Some((end, style, spacing, content)));
                }
            }

            i += 1;
        }

        // No closing tag found
        Err(Error::UnclosedTag(style))
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
}
