# utf8fx API Guide

**Version:** 1.0.0
**Last Updated:** 2025-12-12

Complete API reference for using utf8fx in your Rust projects.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Converter API](#converter-api)
- [FrameRenderer API](#framerenderer-api)
- [BadgeRenderer API](#badgerenderer-api)
- [TemplateParser API](#templateparser-api)
- [Error Handling](#error-handling)
- [Advanced Usage](#advanced-usage)
- [Performance Tips](#performance-tips)

---

## Getting Started

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
utf8fx = "1.0"
```

### Basic Usage

```rust
use utf8fx::{Converter, FrameRenderer, BadgeRenderer, TemplateParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Convert text to Unicode styles
    let converter = Converter::new()?;
    let result = converter.convert("HELLO", "mathbold")?;
    println!("{}", result); // 𝐇𝐄𝐋𝐋𝐎

    Ok(())
}
```

---

## Converter API

The `Converter` transforms text using Unicode character mappings.

### Creating a Converter

```rust
use utf8fx::Converter;

let converter = Converter::new()?;
```

**Error:** Returns `Error::InvalidJson` if `styles.json` is malformed.

### Methods

#### `convert(text: &str, style: &str) -> Result<String>`

Convert text to a Unicode style without spacing.

```rust
let result = converter.convert("Hello World", "mathbold")?;
// Output: 𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝

let result = converter.convert("CODE", "monospace")?;
// Output: 𝙲𝙾𝙳𝙴

let result = converter.convert("elegant", "script")?;
// Output: ℯ𝓁ℯℊ𝒶𝓃𝓉
```

**Parameters:**
- `text` - Input text to transform
- `style` - Style ID or alias (see Available Styles)

**Returns:** Transformed string with Unicode characters

**Errors:**
- `Error::UnknownStyle` - Style doesn't exist

#### `convert_with_spacing(text: &str, style: &str, spacing: usize) -> Result<String>`

Add spaces between each character after conversion.

```rust
let result = converter.convert_with_spacing("HELLO", "mathbold", 1)?;
// Output: 𝐇 𝐄 𝐋 𝐋 𝐎

let result = converter.convert_with_spacing("CODE", "mathbold", 2)?;
// Output: 𝐂  𝐎  𝐃  𝐄

let result = converter.convert_with_spacing("BIG", "fullwidth", 3)?;
// Output: Ｂ   Ｉ   Ｇ
```

**Parameters:**
- `text` - Input text
- `style` - Style ID or alias
- `spacing` - Number of spaces between characters

**Use Cases:**
- Headers with dramatic spacing
- Design elements requiring character separation
- ASCII art and banners

#### `convert_with_separator(text: &str, style: &str, separator: &str, count: usize) -> Result<String>`

Add custom separator characters between each character.

```rust
let result = converter.convert_with_separator("TITLE", "mathbold", "·", 1)?;
// Output: 𝐓·𝐈·𝐓·𝐋·𝐄

let result = converter.convert_with_separator("FLOW", "mathbold", "→", 1)?;
// Output: 𝐅→𝐋→𝐎→𝐖

let result = converter.convert_with_separator("BOLD", "mathbold", "━", 2)?;
// Output: 𝐁━━𝐎━━𝐋━━𝐃
```

**Parameters:**
- `text` - Input text
- `style` - Style ID or alias
- `separator` - Unicode separator character(s)
- `count` - Number of separator repetitions between characters

**Popular Separators:**
- `·` (U+00B7) - Middle dot
- `•` (U+2022) - Bullet
- `─` (U+2500) - Box drawing horizontal
- `━` (U+2501) - Box drawing heavy horizontal
- `→` (U+2192) - Rightward arrow
- `⋅` (U+22C5) - Dot operator

#### `has_style(name: &str) -> bool`

Check if a style exists (by ID or alias).

```rust
if converter.has_style("mathbold") {
    println!("Style exists!");
}

if converter.has_style("mb") {
    println!("Alias works too!");
}
```

#### `list_styles() -> Vec<&Style>`

Get all available styles, sorted by ID.

```rust
for style in converter.list_styles() {
    println!("{}: {} ({})",
        style.id,
        style.name,
        style.category
    );
}
```

**Output:**
```
bold-fraktur: Bold Fraktur (Elegant)
bold-italic: Bold Italic (Elegant)
bold-script: Bold Script (Elegant)
circled-latin: Circled Latin (Boxed)
double-struck: Double-Struck (Technical)
fraktur: Fraktur (Elegant)
fullwidth: Full-Width (Bold)
italic: Italic (Elegant)
mathbold: Mathematical Bold (Bold)
monospace: Monospace (Technical)
negative-circled: Negative Circled (Boxed)
negative-squared: Negative Squared (Boxed)
sans-serif: Sans-Serif (Technical)
sans-serif-bold: Sans-Serif Bold (Bold)
sans-serif-bold-italic: Sans-Serif Bold Italic (Bold)
sans-serif-italic: Sans-Serif Italic (Technical)
script: Script (Elegant)
small-caps: Small Caps (Elegant)
squared-latin: Squared Latin (Boxed)
```

#### `get_style(name: &str) -> Result<&Style>`

Get a specific style by ID or alias.

```rust
let style = converter.get_style("mathbold")?;
println!("Name: {}", style.name);
println!("Category: {}", style.category);
println!("Aliases: {:?}", style.aliases);
```

### Available Styles

| ID | Aliases | Category | Example |
|----|---------|----------|---------|
| `mathbold` | `mb` | Bold | 𝐇𝐄𝐋𝐋𝐎 |
| `fullwidth` | `fw` | Bold | ＨＥＬＬＯ |
| `negative-squared` | `neg-sq` | Boxed | 🅷🅴🅻🅻🅾 |
| `negative-circled` | `neg-circle` | Boxed | 🅗🅔🅛🅛🅞 |
| `squared-latin` | `sq-latin` | Boxed | 🄷🄴🄻🄻🄾 |
| `circled-latin` | `circled`, `circle` | Boxed | Ⓗⓔⓛⓛⓞ |
| `small-caps` | `sc` | Elegant | ʜᴇʟʟᴏ |
| `monospace` | `mono` | Technical | 𝙷𝙴𝙻𝙻𝙾 |
| `double-struck` | `ds` | Technical | ℍ𝔼𝕃𝕃𝕆 |
| `sans-serif` | `ss`, `sans` | Technical | 𝖧𝖤𝖫𝖫𝖮 |
| `sans-serif-bold` | `ssb` | Bold | 𝗛𝗘𝗟𝗟𝗢 |
| `sans-serif-italic` | `ssi` | Technical | 𝘏𝘌𝘓𝘓𝘖 |
| `sans-serif-bold-italic` | `ssbi` | Bold | 𝙃𝙀𝙇𝙇𝙊 |
| `italic` | `it` | Elegant | 𝐻𝐸𝐿𝐿𝑂 |
| `bold-italic` | `bi` | Elegant | 𝑯𝑬𝑳𝑳𝑶 |
| `script` | `scr`, `cursive` | Elegant | ℋℰℒℒ𝒪 |
| `bold-script` | `bscr` | Elegant | 𝓗𝓔𝓛𝓛𝓞 |
| `fraktur` | `fr`, `gothic` | Elegant | ℌ𝔈𝔏𝔏𝔒 |
| `bold-fraktur` | `bfr` | Elegant | 𝕳𝕰𝕷𝕷𝕺 |

### Character Support

Not all styles support all characters:

```rust
// Uppercase, lowercase, numbers all supported
let result = converter.convert("Hello123", "mathbold")?;
// Output: 𝐇𝐞𝐥𝐥𝐨𝟏𝟐𝟑

// Small caps only has lowercase
let result = converter.convert("hello", "small-caps")?;
// Output: ʜᴇʟʟᴏ

// Unsupported characters pass through unchanged
let result = converter.convert("Hello!", "mathbold")?;
// Output: 𝐇𝐞𝐥𝐥𝐨! (exclamation unchanged)

// Whitespace always preserved
let result = converter.convert("Hello World", "mathbold")?;
// Output: 𝐇𝐞𝐥𝐥𝐨 𝐖𝐨𝐫𝐥𝐝
```

---

## FrameRenderer API

The `FrameRenderer` adds decorative prefix/suffix around text.

### Creating a FrameRenderer

```rust
use utf8fx::FrameRenderer;

let renderer = FrameRenderer::new()?;
```

**Error:** Returns `Error::InvalidJson` if `frames.json` is malformed.

### Methods

#### `apply_frame(text: &str, frame_style: &str) -> Result<String>`

Wrap text with decorative elements.

```rust
let result = renderer.apply_frame("TITLE", "gradient")?;
// Output: ▓▒░ TITLE ░▒▓

let result = renderer.apply_frame("Note", "solid-left")?;
// Output: █▌Note

let result = renderer.apply_frame("Header", "line-bold")?;
// Output: ━━━ Header ━━━
```

**Parameters:**
- `text` - Text to wrap (can be pre-styled)
- `frame_style` - Frame ID or alias

**Returns:** Text with prefix and suffix added

**Errors:**
- `Error::UnknownFrame` - Frame doesn't exist

#### `has_frame(name: &str) -> bool`

Check if a frame exists.

```rust
if renderer.has_frame("gradient") {
    println!("Frame exists!");
}
```

#### `list_frames() -> Vec<&FrameType>`

Get all available frames, sorted by ID.

```rust
for frame in renderer.list_frames() {
    println!("{}: {} - {}",
        frame.id,
        frame.name,
        frame.description
    );
}
```

#### `get_frame(name: &str) -> Result<&FrameType>`

Get a specific frame by ID or alias.

```rust
let frame = renderer.get_frame("gradient")?;
println!("Name: {}", frame.name);
println!("Prefix: {}", frame.prefix);
println!("Suffix: {}", frame.suffix);
```

### Available Frames

**Gradient & Blocks:**
```rust
// gradient: ▓▒░ ... ░▒▓
renderer.apply_frame("TEXT", "gradient")?;

// solid-left: █▌...
renderer.apply_frame("TEXT", "solid-left")?;

// solid-right: ...▐█
renderer.apply_frame("TEXT", "solid-right")?;

// solid-both: █▌...▐█
renderer.apply_frame("TEXT", "solid-both")?;

// block-top: ▀▀▀ ... ▀▀▀
renderer.apply_frame("TEXT", "block-top")?;

// block-bottom: ▄▄▄ ... ▄▄▄
renderer.apply_frame("TEXT", "block-bottom")?;
```

**Lines:**
```rust
// line-light: ─── ... ───
renderer.apply_frame("TEXT", "line-light")?;

// line-bold: ━━━ ... ━━━
renderer.apply_frame("TEXT", "line-bold")?;

// line-double: ═══ ... ═══
renderer.apply_frame("TEXT", "line-double")?;

// line-dashed: ╌╌╌ ... ╌╌╌
renderer.apply_frame("TEXT", "line-dashed")?;
```

**Symbols:**
```rust
// arrow-right: → ... →
renderer.apply_frame("TEXT", "arrow-right")?;

// dot: · ... ·
renderer.apply_frame("TEXT", "dot")?;

// bullet: • ... •
renderer.apply_frame("TEXT", "bullet")?;

// star: ★ ... ☆
renderer.apply_frame("TEXT", "star")?;

// diamond: ◆ ... ◇
renderer.apply_frame("TEXT", "diamond")?;
```

**Brackets:**
```rust
// lenticular: 【...】
renderer.apply_frame("TEXT", "lenticular")?;

// angle: 《...》
renderer.apply_frame("TEXT", "angle")?;

// guillemet: « ... »
renderer.apply_frame("TEXT", "guillemet")?;

// guillemet-single: ‹ ... ›
renderer.apply_frame("TEXT", "guillemet-single")?;

// heavy-quote: ❝...❞
renderer.apply_frame("TEXT", "heavy-quote")?;
```

**Special:**
```rust
// triangle-right: ▶ ... ◀
renderer.apply_frame("TEXT", "triangle-right")?;

// finger: ☞ ... ☜
renderer.apply_frame("TEXT", "finger")?;

// fisheye: ◉ ... ◉
renderer.apply_frame("TEXT", "fisheye")?;

// asterism: ⁂ ... ⁂
renderer.apply_frame("TEXT", "asterism")?;

// arc-top: ╭ ... ╮
renderer.apply_frame("TEXT", "arc-top")?;

// arc-bottom: ╰ ... ╯
renderer.apply_frame("TEXT", "arc-bottom")?;
```

### Combining Styles and Frames

Frames work with styled text:

```rust
let converter = Converter::new()?;
let renderer = FrameRenderer::new()?;

// Style first, then frame
let styled = converter.convert("HEADER", "mathbold")?;
let framed = renderer.apply_frame(&styled, "gradient")?;
// Output: ▓▒░ 𝐇𝐄𝐀𝐃𝐄𝐑 ░▒▓

// With separator
let styled = converter.convert_with_separator("TITLE", "mathbold", "·", 1)?;
let framed = renderer.apply_frame(&styled, "solid-left")?;
// Output: █▌𝐓·𝐈·𝐓·𝐋·𝐄
```

---

## BadgeRenderer API

The `BadgeRenderer` encloses numbers (0-20) and letters (a-z) with pre-composed Unicode characters.

### Creating a BadgeRenderer

```rust
use utf8fx::BadgeRenderer;

let renderer = BadgeRenderer::new()?;
```

**Error:** Returns `Error::InvalidJson` if `badges.json` is malformed.

### Methods

#### `apply_badge(text: &str, badge_type: &str) -> Result<String>`

Enclose text in a badge character.

```rust
let result = renderer.apply_badge("1", "circle")?;
// Output: ①

let result = renderer.apply_badge("a", "paren-letter")?;
// Output: ⒜

let result = renderer.apply_badge("10", "circle")?;
// Output: ⑩
```

**Parameters:**
- `text` - Text to enclose (must be in badge's supported charset)
- `badge_type` - Badge ID or alias

**Returns:** Single Unicode character containing the enclosed text

**Errors:**
- `Error::UnknownBadge` - Badge type doesn't exist
- `Error::UnsupportedChar` - Text not in badge's charset

#### `has_badge(name: &str) -> bool`

Check if a badge type exists.

```rust
if renderer.has_badge("circle") {
    println!("Badge exists!");
}
```

#### `list_badges() -> Vec<&BadgeType>`

Get all available badge types, sorted by ID.

```rust
for badge in renderer.list_badges() {
    println!("{}: {} - {}",
        badge.id,
        badge.name,
        badge.description
    );
}
```

#### `get_badge(name: &str) -> Result<&BadgeType>`

Get a specific badge by ID or alias.

```rust
let badge = renderer.get_badge("circle")?;
println!("Name: {}", badge.name);
println!("Supported chars: {}", badge.mappings.len());
```

### Available Badge Types

**Number Badges (0-20):**

```rust
// circle: ①②③④⑤⑥⑦⑧⑨⑩⑪...⑳
renderer.apply_badge("1", "circle")?;     // ①
renderer.apply_badge("10", "circle")?;    // ⑩
renderer.apply_badge("0", "circle")?;     // ⓪

// negative-circle: ❶❷❸❹❺❻❼❽❾❿⓫...⓴
renderer.apply_badge("1", "negative-circle")?;  // ❶
renderer.apply_badge("20", "negative-circle")?; // ⓴

// double-circle: ⓵⓶⓷⓸⓹⓺⓻⓼⓽⓾ (1-10 only)
renderer.apply_badge("1", "double-circle")?;  // ⓵
renderer.apply_badge("5", "double-circle")?;  // ⓹

// paren: ⑴⑵⑶⑷⑸⑹⑺⑻⑼⑽⑾...⒇ (1-20)
renderer.apply_badge("3", "paren")?;  // ⑶

// period: 🄁🄂🄃🄄🄅🄆🄇🄈🄉🄊🄋...🄔
renderer.apply_badge("7", "period")?;  // 🄇
```

**Letter Badges (a-z):**

```rust
// paren-letter: ⒜⒝⒞⒟⒠⒡⒢⒣⒤⒥⒦⒧⒨⒩⒪⒫⒬⒭⒮⒯⒰⒱⒲⒳⒴⒵
renderer.apply_badge("a", "paren-letter")?;  // ⒜
renderer.apply_badge("z", "paren-letter")?;  // ⒵
```

### Charset Limitations

Badges have strict charset support:

```rust
// Supported
renderer.apply_badge("1", "circle")?;    // ①
renderer.apply_badge("20", "circle")?;   // ⑳
renderer.apply_badge("a", "paren-letter")?;  // ⒜

// Unsupported - returns Error::UnsupportedChar
renderer.apply_badge("99", "circle")?;   // Error: not in 0-20 range
renderer.apply_badge("A", "paren-letter")?;  // Error: uppercase not supported
renderer.apply_badge("21", "circle")?;   // Error: above maximum
```

**Why the limitation?**
Badges use pre-composed Unicode characters (U+2460-24FF). These blocks only contain specific numbers and lowercase letters.

### Use Cases

**Step Indicators:**
```rust
println!("{}  Install dependencies", renderer.apply_badge("1", "circle")?);
println!("{}  Configure settings", renderer.apply_badge("2", "circle")?);
println!("{}  Run application", renderer.apply_badge("3", "circle")?);
```

**Priority Labels:**
```rust
println!("Priority {} Critical bug", renderer.apply_badge("1", "negative-circle")?);
println!("Priority {} Feature request", renderer.apply_badge("2", "negative-circle")?);
```

**Option Lists:**
```rust
println!("{}  Accept changes", renderer.apply_badge("a", "paren-letter")?);
println!("{}  Reject changes", renderer.apply_badge("b", "paren-letter")?);
println!("{}  Request review", renderer.apply_badge("c", "paren-letter")?);
```

---

## TemplateParser API

The `TemplateParser` processes markdown files with embedded template syntax.

### Creating a TemplateParser

```rust
use utf8fx::TemplateParser;

let parser = TemplateParser::new()?;
```

**Note:** Initializes all three renderers (Converter, FrameRenderer, BadgeRenderer).

### Methods

#### `process(content: &str) -> Result<String>`

Process markdown with template syntax.

```rust
let input = "# {{mathbold}}TITLE{{/mathbold}}";
let output = parser.process(input)?;
// Output: # 𝐓𝐈𝐓𝐋𝐄
```

**Parameters:**
- `content` - Markdown text with template syntax

**Returns:** Processed markdown with Unicode characters

**Errors:**
- `Error::UnknownStyle` - Style doesn't exist
- `Error::UnknownFrame` - Frame doesn't exist
- `Error::UnknownBadge` - Badge doesn't exist
- `Error::UnclosedTag` - Template not closed
- `Error::MismatchedTags` - Opening/closing tags don't match

### Template Syntax

**Style Templates:**
```markdown
{{style}}text{{/style}}
{{style:spacing=N}}text{{/style}}
{{style:separator=name}}text{{/style}}
```

**Frame Templates:**
```markdown
{{frame:type}}text{{/frame}}
```

**Badge Templates:**
```markdown
{{badge:type}}text{{/badge}}
```

### Examples

**Basic Styling:**
```rust
let input = r#"
# {{mathbold}}Project Title{{/mathbold}}

{{italic}}A Unicode text transformation library{{/italic}}
"#;

let output = parser.process(input)?;
```

**With Parameters:**
```rust
let input = r#"
## {{mathbold:spacing=1}}S P A C E D{{/mathbold}}

{{mathbold:separator=dot}}D O T S{{/mathbold}}
"#;

let output = parser.process(input)?;
```

**Composition:**
```rust
let input = r#"
{{frame:gradient}}{{mathbold:separator=dash}}HEADER{{/mathbold}}{{/frame}}
"#;

let output = parser.process(input)?;
// Output: ▓▒░ 𝐇─𝐄─𝐀─𝐃─𝐄─𝐑 ░▒▓
```

**Badges:**
```rust
let input = r#"
## Installation Steps

{{badge:circle}}1{{/badge}} Download the package
{{badge:circle}}2{{/badge}} Install dependencies
{{badge:circle}}3{{/badge}} Run the application
"#;

let output = parser.process(input)?;
```

### Code Block Preservation

Templates inside code blocks are NOT processed:

````rust
let input = r#"
# Example

Use this syntax:

```
{{mathbold}}Not processed{{/mathbold}}
```

But {{mathbold}}this is{{/mathbold}} processed.
"#;

let output = parser.process(input)?;
// Code block content remains unchanged
````

### Inline Code Preservation

Templates inside backticks are NOT processed:

```rust
let input = "Use `{{mathbold}}TITLE{{/mathbold}}` syntax";
let output = parser.process(input)?;
// Output: Use `{{mathbold}}TITLE{{/mathbold}}` syntax
```

---

## Error Handling

All errors implement `std::error::Error` and use the `thiserror` crate.

### Error Types

```rust
use utf8fx::Error;

match result {
    Err(Error::UnknownStyle(name)) => {
        eprintln!("Style '{}' not found", name);
        eprintln!("Run `utf8fx list` to see available styles");
    }
    Err(Error::UnknownFrame(name)) => {
        eprintln!("Frame '{}' not found", name);
    }
    Err(Error::UnknownBadge(name)) => {
        eprintln!("Badge '{}' not found", name);
    }
    Err(Error::UnsupportedChar(badge, ch)) => {
        eprintln!("Badge '{}' doesn't support '{}'", badge, ch);
    }
    Err(Error::UnclosedTag(tag)) => {
        eprintln!("Template {{{{{}}}}} was never closed", tag);
    }
    Err(Error::MismatchedTags(expected, found)) => {
        eprintln!("Expected {{{{/{}}}}}, found {{{{{}}}}}", expected, found);
    }
    Err(Error::InvalidStyleName(name)) => {
        eprintln!("Invalid style name: {}", name);
    }
    Ok(result) => println!("{}", result),
}
```

### Complete Error List

| Error | When It Occurs | Recovery |
|-------|----------------|----------|
| `UnknownStyle(String)` | Style ID/alias doesn't exist | Check with `has_style()` first |
| `UnknownFrame(String)` | Frame ID/alias doesn't exist | Check with `has_frame()` first |
| `UnknownBadge(String)` | Badge ID/alias doesn't exist | Check with `has_badge()` first |
| `UnsupportedChar(String, String)` | Badge doesn't support character | Validate charset before calling |
| `ParseError(String)` | Generic parse error | Check input format |
| `UnclosedTag(String)` | Template not closed | Add closing tag |
| `MismatchedTags(String, String)` | Wrong closing tag | Match opening/closing tags |
| `InvalidStyleName(String)` | Style name has invalid characters | Use alphanumeric + hyphens only |
| `FileNotFound(PathBuf)` | File doesn't exist | Check file path |
| `PermissionDenied(PathBuf)` | No file access | Check permissions |
| `IoError(std::io::Error)` | I/O operation failed | Check filesystem |
| `InvalidJson(serde_json::Error)` | JSON parse error | Validate JSON files |
| `InvalidUtf8(FromUtf8Error)` | Invalid UTF-8 | Ensure valid UTF-8 input |

### Graceful Error Handling

```rust
fn process_safely(text: &str, style: &str) -> String {
    let converter = match Converter::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to initialize: {}", e);
            return text.to_string(); // Fallback to original
        }
    };

    match converter.convert(text, style) {
        Ok(result) => result,
        Err(Error::UnknownStyle(_)) => {
            eprintln!("Unknown style, using fallback");
            text.to_string()
        }
        Err(e) => {
            eprintln!("Conversion error: {}", e);
            text.to_string()
        }
    }
}
```

---

## Advanced Usage

### Custom Style Addition

Add your own Unicode mappings by editing `data/styles.json`:

```json
{
  "id": "my-custom",
  "name": "My Custom Style",
  "category": "Custom",
  "description": "Custom Unicode transformation",
  "aliases": ["custom", "mc"],
  "uppercase": {
    "A": "𝒜",
    "B": "ℬ",
    "C": "𝒞"
  },
  "lowercase": {
    "a": "𝒶",
    "b": "𝒷",
    "c": "𝒸"
  },
  "digits": {
    "0": "𝟢",
    "1": "𝟣",
    "2": "𝟤"
  }
}
```

Then use immediately:

```rust
let result = converter.convert("ABC", "my-custom")?;
```

### Batch Processing

Process multiple files efficiently:

```rust
use std::fs;
use utf8fx::TemplateParser;

fn process_directory(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parser = TemplateParser::new()?; // Initialize once

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |e| e == "md") {
            let content = fs::read_to_string(&path)?;
            let processed = parser.process(&content)?;

            // Write to .processed.md
            let mut output_path = path.clone();
            output_path.set_extension("processed.md");
            fs::write(output_path, processed)?;
        }
    }

    Ok(())
}
```

### Caching Converter

Reuse converters across multiple operations:

```rust
struct StyleCache {
    converter: Converter,
    frame_renderer: FrameRenderer,
    badge_renderer: BadgeRenderer,
}

impl StyleCache {
    fn new() -> Result<Self, Error> {
        Ok(Self {
            converter: Converter::new()?,
            frame_renderer: FrameRenderer::new()?,
            badge_renderer: BadgeRenderer::new()?,
        })
    }

    fn style(&self, text: &str, style: &str) -> Result<String, Error> {
        self.converter.convert(text, style)
    }

    fn frame(&self, text: &str, frame: &str) -> Result<String, Error> {
        self.frame_renderer.apply_frame(text, frame)
    }

    fn badge(&self, text: &str, badge: &str) -> Result<String, Error> {
        self.badge_renderer.apply_badge(text, badge)
    }
}
```

### Complex Composition

Build complex styled text programmatically:

```rust
fn create_header(
    text: &str,
    converter: &Converter,
    renderer: &FrameRenderer,
) -> Result<String, Error> {
    // Convert with separator
    let styled = converter.convert_with_separator(
        text,
        "mathbold",
        "·",
        1
    )?;

    // Add frame
    let framed = renderer.apply_frame(&styled, "gradient")?;

    // Add markdown header
    Ok(format!("# {}\n\n", framed))
}

// Usage
let header = create_header("TITLE", &converter, &renderer)?;
// Output: # ▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓
```

### Dynamic Style Selection

Choose styles at runtime:

```rust
fn style_by_category(
    text: &str,
    category: &str,
    converter: &Converter,
) -> Result<String, Error> {
    let styles = converter.list_styles();

    let style = styles
        .iter()
        .find(|s| s.category.to_lowercase() == category.to_lowercase())
        .ok_or_else(|| Error::ParseError(format!("No style in category: {}", category)))?;

    converter.convert(text, &style.id)
}

// Usage
let bold_text = style_by_category("TITLE", "bold", &converter)?;
let elegant_text = style_by_category("Subtitle", "elegant", &converter)?;
```

### Validation Before Processing

Check validity before attempting conversion:

```rust
fn safe_convert(
    text: &str,
    style: &str,
    converter: &Converter,
) -> Result<String, Error> {
    // Validate style exists
    if !converter.has_style(style) {
        return Err(Error::UnknownStyle(style.to_string()));
    }

    // Validate text is not empty
    if text.is_empty() {
        return Ok(String::new());
    }

    // Perform conversion
    converter.convert(text, style)
}
```

---

## Performance Tips

### 1. Reuse Component Instances

**Don't create new instances repeatedly:**
```rust
for text in texts {
    let converter = Converter::new()?; // Expensive!
    let result = converter.convert(text, "mathbold")?;
}
```

**Create once, reuse many times:**
```rust
let converter = Converter::new()?; // Create once
for text in texts {
    let result = converter.convert(text, "mathbold")?;
}
```

### 2. Use Aliases for Shorter Code

```rust
// Instead of
converter.convert(text, "mathematical-bold")?;

// Use
converter.convert(text, "mb")?;
```

### 3. Pre-validate in Batch Operations

```rust
let style = "mathbold";
if !converter.has_style(style) {
    return Err(Error::UnknownStyle(style.to_string()));
}

for text in texts {
    // No validation overhead per item
    let result = converter.convert(text, style)?;
}
```

### 4. Avoid Unnecessary Spacing

```rust
// These are equivalent but second is more efficient
converter.convert_with_spacing(text, style, 0)?; // Extra overhead
converter.convert(text, style)?;                  // Direct
```

### 5. String Allocation

Results allocate new strings. For very large documents, consider:

```rust
use std::fmt::Write;

let mut output = String::with_capacity(input.len() * 2); // Pre-allocate
for line in input.lines() {
    let processed = parser.process(line)?;
    writeln!(output, "{}", processed)?;
}
```

### Performance Characteristics

**Time Complexity:**
- Style conversion: O(n) where n = input length
- Template parsing: O(n) single-pass
- Frame application: O(1) string concatenation
- Badge lookup: O(1) HashMap lookup

**Memory:**
- JSON data embedded in binary: ~50KB
- Component initialization: One-time allocation
- Per-conversion: Allocates output String (~2x input for styled text)

**Note:** Specific benchmarks depend on hardware, input characteristics, and style complexity. Run `cargo bench` (when implemented) for measurements on your system.

---

## Real-World Examples

### README Generator

```rust
use utf8fx::{Converter, FrameRenderer, TemplateParser};
use std::fs;

fn generate_readme() -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string("README.template.md")?;
    let parser = TemplateParser::new()?;
    let output = parser.process(&template)?;
    fs::write("README.md", output)?;
    Ok(())
}
```

### CLI Banner

```rust
fn print_banner(app_name: &str, version: &str) -> Result<(), Error> {
    let converter = Converter::new()?;
    let renderer = FrameRenderer::new()?;

    let title = converter.convert_with_spacing(app_name, "mathbold", 1)?;
    let framed = renderer.apply_frame(&title, "gradient")?;

    println!("\n{}", framed);
    println!("Version: {}\n", version);
    Ok(())
}
```

### Status Messages

```rust
fn status_message(level: &str, text: &str) -> Result<String, Error> {
    let converter = Converter::new()?;

    let styled_level = match level {
        "error" => converter.convert(level, "negative-squared")?,
        "warning" => converter.convert(level, "fullwidth")?,
        "info" => converter.convert(level, "mathbold")?,
        _ => level.to_string(),
    };

    Ok(format!("[{}] {}", styled_level, text))
}
```

### Documentation Builder

```rust
use utf8fx::TemplateParser;
use std::path::Path;

fn build_docs(src_dir: &Path, out_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let parser = TemplateParser::new()?;

    for entry in std::fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |e| e == "md") {
            let content = std::fs::read_to_string(&path)?;
            let processed = parser.process(&content)?;

            let out_path = out_dir.join(path.file_name().unwrap());
            std::fs::write(out_path, processed)?;
        }
    }

    Ok(())
}
```

---

## See Also

- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture and design
- [parser-design.md](parser-design.md) - State machine implementation details
- [PLANNING.md](PLANNING.md) - Development roadmap
- [examples/README.md](../examples/README.md) - Template syntax examples

---

**Last Updated:** 2025-12-12
**Version:** 1.0.0
