# mdfx API Guide

**Version:** 1.0.0
**Last Updated:** 2025-12-14

Complete API reference for the mdfx **markdown compiler** library.

> **Note:** This guide covers the **library API** (`mdfx` crate). For CLI usage, run `mdfx --help` or see the [README](../README.md).

---

## Overview

mdfx is a **markdown compiler** that transforms template syntax into styled output. The compiler pipeline includes:
- **Lexing/Parsing:** Template syntax to Primitive AST
- **Semantic Analysis:** Registry resolution, EvalContext filtering
- **Code Generation:** Backend-specific output (shields.io or SVG)

---

## Workspace Structure

mdfx uses a Cargo workspace with two packages:

| Package | Purpose | Dependencies |
|---------|---------|--------------|
| **`mdfx`** | Core compiler | 8 deps (serde, serde_json, thiserror, unicode-segmentation, sha2, chrono) |
| **`mdfx-cli`** | CLI tool | mdfx + CLI deps (clap, colored, serde_json) |

**For library usage**, add only the `mdfx` crate - no CLI dependencies included.

---

## Table of Contents

- [Getting Started](#getting-started)
- [Target System](#target-system) 🆕
- [Custom Palette Support](#custom-palette-support) 🆕
- [Configuration & Partials](#configuration--partials) 🆕
- [ComponentsRenderer API](#componentsrenderer-api) ⭐ **Primary API**
- [ShieldsRenderer API](#shieldsrenderer-api)
- [Converter API](#converter-api)
- [FrameRenderer API](#framerenderer-api)
- [BadgeRenderer API](#badgerenderer-api)
- [TemplateParser API](#templateparser-api)
- [Multi-Backend Rendering](#multi-backend-rendering)
- [Enhanced Swatch Options](#enhanced-swatch-options) 🆕
- [Registry API](#registry-api) 🆕
- [Error Handling](#error-handling)
- [Advanced Usage](#advanced-usage)
- [Performance Tips](#performance-tips)

---

## Getting Started

### Installation

Add mdfx as a library dependency:

```toml
[dependencies]
mdfx = "1.0"
```

**Do NOT** install `mdfx-cli` as a library dependency - that's the CLI tool package.

### Quick Start (Recommended: Components)

```rust
use mdfx::TemplateParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = TemplateParser::new()?;
    
    // Process UI components (primary API)
    let input = "# {{ui:header}}PROJECT{{/ui}}";
    let output = parser.process(input)?;
    
    println!("{}", output);
    // Output: # ▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓 ░▒▓
    
    Ok(())
}
```

### Direct API Usage

For programmatic use without templates:

```rust
use mdfx::{ComponentsRenderer, Converter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Components API
    let components = ComponentsRenderer::new()?;
    let expanded = components.expand("tech", &["rust".to_string()], None)?;
    
    // Converter API (character transformation)
    let converter = Converter::new()?;
    let result = converter.convert("HELLO", "mathbold")?;
    println!("{}", result); // 𝐇𝐄𝐋𝐋𝐎
    
    Ok(())
}
```

---

## Target System

**Version:** 1.0.0

The Target system allows the compiler to adapt output for different deployment platforms.

### Available Targets

| Target | Backend | Use Case |
|--------|---------|----------|
| `github` | shields.io | GitHub READMEs, wikis |
| `npm` | shields.io | npm package docs |
| `local` | SVG | Offline documentation |
| `auto` | (detected) | Infer from output path |

### Using Targets

```rust
use mdfx::{TemplateParser, get_target};

// Explicit target selection
let target = get_target("github");
let parser = TemplateParser::with_target(target)?;

// Auto-detection from path
use mdfx::detect_target_from_path;
use std::path::Path;

let target = detect_target_from_path(Path::new("README.md"));
// Returns GitHubTarget

let target = detect_target_from_path(Path::new("docs/local/guide.md"));
// Returns LocalDocsTarget
```

### Target Trait

```rust
use mdfx::{Target, BackendType, EvalContext};

// All targets implement this trait
pub trait Target {
    fn backend_type(&self) -> BackendType;
    fn eval_context(&self) -> EvalContext;
    fn name(&self) -> &str;
}
```

### Listing Available Targets

```rust
use mdfx::available_targets;

for target in available_targets() {
    println!("{}: {:?}", target.name(), target.backend_type());
}
```

---

## Custom Palette Support

**Version:** 1.0.0

Custom palettes allow projects to define their own named colors.

### Extending the Palette Programmatically

```rust
use mdfx::ComponentsRenderer;
use std::collections::HashMap;

let mut renderer = ComponentsRenderer::new()?;

// Add custom colors
let custom_colors: HashMap<String, String> = [
    ("brand-primary".to_string(), "FF6B35".to_string()),
    ("brand-secondary".to_string(), "2B6CB0".to_string()),
].into_iter().collect();

renderer.extend_palette(custom_colors);

// Now "brand-primary" resolves in swatch expansion
let result = renderer.expand("swatch", &["brand-primary".to_string()], None)?;
```

### Via TemplateParser

```rust
use mdfx::TemplateParser;
use std::collections::HashMap;

let mut parser = TemplateParser::new()?;

parser.extend_palette(HashMap::from([
    ("brand".to_string(), "FF6B35".to_string()),
]));

// Templates can now use the custom color
let output = parser.process("{{ui:swatch:brand/}}")?;
```

### Resolution Order

1. Custom palette (highest priority)
2. Built-in registry palette
3. Direct hex code (6-digit, e.g., `FF6B35`)

---

## Configuration & Partials

**Version:** 1.0.0

mdfx supports project-level configuration via `.mdfx.json` files, including user-defined template partials.

### MdfxConfig API

```rust
use mdfx::{MdfxConfig, PartialDef};

// Load from file
let config = MdfxConfig::load(".mdfx.json")?;

// Auto-discover (searches current + parent directories)
let config = MdfxConfig::discover();

// Check partials
if config.has_partial("hero") {
    let partial = config.get_partial("hero").unwrap();
    println!("Template: {}", partial.template);
}

// Iterate partials
for name in config.partial_names() {
    println!("Partial: {}", name);
}
```

### Loading Config into Parser

```rust
use mdfx::{TemplateParser, MdfxConfig};

let mut parser = TemplateParser::new()?;

// Load from config file
let config = MdfxConfig::load(".mdfx.json")?;
parser.load_config(&config);

// Or add partials programmatically
parser.add_partial("hero", "{{frame:gradient}}{{mathbold}}$1{{/mathbold}}{{/frame}}");
parser.add_partial("techstack", "{{ui:tech:rust/}} {{ui:tech:typescript/}}");

// Check if partial exists
if parser.has_partial("hero") {
    println!("Hero partial loaded!");
}

// Process with partials
let input = "{{partial:hero}}MY TITLE{{/partial}}";
let output = parser.process(input)?;
// Output: ▓▒░ 𝐌𝐘 𝐓𝐈𝐓𝐋𝐄 ░▒▓
```

### Config File Format

```json
{
  "partials": {
    "hero": {
      "template": "{{frame:gradient}}{{mathbold}}$1{{/mathbold}}{{/frame}}",
      "description": "Hero header with gradient frame"
    },
    "techstack": {
      "template": "{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}}"
    },
    "warning-box": {
      "template": "{{frame:solid-left}}⚠️ $content{{/frame}}"
    }
  },
  "palette": {
    "brand": "FF5500",
    "primary": "2B6CB0"
  }
}
```

### Content Substitution

Partials support `$1` and `$content` placeholders:

```rust
// Template: "Hello, $1!"
// Input: {{partial:greeting}}World{{/partial}}
// Output: Hello, World!

// Template: "[ $content ]"
// Input: {{partial:wrapper}}TEXT{{/partial}}
// Output: [ TEXT ]
```

### PartialDef Struct

```rust
use mdfx::PartialDef;

#[derive(Debug, Clone)]
pub struct PartialDef {
    /// The template string (may contain $1, $content placeholders)
    pub template: String,

    /// Optional description for documentation
    pub description: Option<String>,
}
```

### Merging Configs

```rust
use mdfx::MdfxConfig;

let mut config1 = MdfxConfig::load("base.mdfx.json")?;
let config2 = MdfxConfig::load("project.mdfx.json")?;

// Merge (config2 takes precedence)
config1.merge(config2);
```

---

## ComponentsRenderer API

⭐ **This is the primary user-facing API.** Components provide high-level semantic elements that expand to primitives.

### Overview

```rust
use mdfx::ComponentsRenderer;

let renderer = ComponentsRenderer::new()?;
```

The `ComponentsRenderer` expands UI components (like `{{ui:header}}`) into primitive templates (like `{{frame:*}}{{mathbold}}...{{/mathbold}}{{/frame}}`).

### Creating a Renderer

```rust
use mdfx::ComponentsRenderer;

let renderer = ComponentsRenderer::new()?;
```

**Error:** Returns `Error::ParseError` if `registry.json` is malformed.

**Data Source:**
- `data/registry.json` - Unified registry (components, palette, styles, frames, badges, separators)

### Methods

#### `expand(component: &str, args: &[String], content: Option<&str>) -> Result<String>`

Expand a component into its primitive template.

**Parameters:**
- `component` - Component name (e.g., "divider", "tech", "header")
- `args` - Positional arguments (e.g., `["rust"]` for `tech:rust`)
- `content` - Inner content for non-self-closing components

**Returns:** Expanded template string ready for parsing

**Examples:**

```rust
// Self-closing component (no content)
let result = renderer.expand("divider", &[], None)?;
// Returns: "{{shields:bar:colors=292a2d,292c34,f41c80,282f3c:style=flat-square/}}"

// Component with positional arg
let result = renderer.expand("tech", &["rust".to_string()], None)?;
// Returns: "{{shields:icon:logo=rust:bg=292A2D:logoColor=FFFFFF:style=flat-square/}}"

// Component with content
let result = renderer.expand("header", &[], Some("TITLE"))?;
// Returns: "{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}"

// Component with arg + content
let result = renderer.expand("callout", &["warning".to_string()], Some("Breaking change"))?;
// Returns: "{{frame:solid-left}}{{shields:block:color=eab308:style=flat-square/}} Breaking change{{/frame}}"
```

**Color Resolution:**

Args are automatically resolved against the palette:

```rust
// "accent" resolves to "f41c80"
let result = renderer.expand("swatch", &["accent".to_string()], None)?;
// Returns: "{{shields:block:color=f41c80:style=flat-square/}}"

// Hex codes pass through
let result = renderer.expand("swatch", &["abc123".to_string()], None)?;
// Returns: "{{shields:block:color=abc123:style=flat-square/}}"
```

#### `has(name: &str) -> bool`

Check if a component exists.

```rust
if renderer.has("divider") {
    println!("Component exists!");
}

if renderer.has("nonexistent") {
    println!("This won't print");
}
```

#### `list() -> Vec<(&String, &ComponentDef)>`

Get all available components, sorted alphabetically.

```rust
for (name, def) in renderer.list() {
    println!("{}: {}", name, def.description);
    println!("  Self-closing: {}", def.self_closing);
    println!("  Type: {}", def.component_type);
}
```

**Output:**
```
callout: Framed messages with indicators
  Self-closing: false
  Type: expand
divider: Visual divider bar with themed colors
  Self-closing: true
  Type: expand
header: Section header with gradient frame and bold text
  Self-closing: false
  Type: expand
...
```

#### `list_palette() -> Vec<(&String, &String)>`

Get all palette colors (design tokens).

```rust
for (name, hex) in renderer.list_palette() {
    println!("{}: #{}", name, hex);
}
```

**Output:**
```
accent: #f41c80
error: #ef4444
info: #3b82f6
slate: #6b7280
success: #22c55e
ui.bg: #292a2d
ui.panel: #282f3c
ui.surface: #292c34
warning: #eab308
white: #ffffff
...
```

#### `get(name: &str) -> Option<&ComponentDef>`

Get a component definition.

```rust
if let Some(def) = renderer.get("header") {
    println!("Template: {}", def.template);
    println!("Args: {:?}", def.args);
}
```

### Shipped Components

#### divider

**Type:** Self-closing
**Args:** None
**Usage:** `{{ui:divider/}}`

```rust
let result = renderer.expand("divider", &[], None)?;
```

**Output:** 4-color bar using theme colors

**Template:**
```
{{shields:bar:colors=ui.bg,ui.surface,accent,ui.panel:style=flat-square/}}
```

#### swatch

**Type:** Self-closing
**Args:** `[color]`
**Usage:** `{{ui:swatch:accent/}}`

```rust
let result = renderer.expand("swatch", &["accent".to_string()], None)?;
```

**Output:** Single colored block

**Template:**
```
{{shields:block:color=$1:style=flat-square/}}
```

#### tech

**Type:** Self-closing
**Args:** `[logo]`
**Usage:** `{{ui:tech:rust/}}`

```rust
let result = renderer.expand("tech", &["rust".to_string()], None)?;
```

**Output:** Technology logo badge (uses Simple Icons)

**Template:**
```
{{shields:icon:logo=$1:bg=ui.bg:logoColor=white:style=flat-square/}}
```

**Available Logos:** 2000+ from [Simple Icons](https://simpleicons.org/) (rust, python, postgresql, docker, kubernetes, etc.)

#### status

**Type:** Self-closing
**Args:** `[level]`
**Usage:** `{{ui:status:success/}}`

```rust
let result = renderer.expand("status", &["success".to_string()], None)?;
```

**Output:** Colored status indicator

**Common levels:** `success` (green), `warning` (yellow), `error` (red), `info` (blue)

**Template:**
```
{{shields:block:color=$1:style=flat-square/}}
```

#### header

**Type:** Block (requires content)
**Args:** None
**Usage:** `{{ui:header}}TITLE{{/ui}}`

```rust
let result = renderer.expand("header", &[], Some("TITLE"))?;
```

**Output:** Gradient frame with bold dotted text

**Template:**
```
{{frame:gradient}}{{mathbold:separator=dot}}$content{{/mathbold}}{{/frame}}
```

#### callout

**Type:** Block (requires content)
**Args:** `[level]`
**Usage:** `{{ui:callout:warning}}Message{{/ui}}`

```rust
let result = renderer.expand("callout", &["warning".to_string()], Some("Message"))?;
```

**Output:** Left-framed message with colored indicator

**Template:**
```
{{frame:solid-left}}{{shields:block:color=$1:style=flat-square/}} $content{{/frame}}
```

### Design Tokens (Palette)

Components use named colors from `palette.json`. These resolve during expansion.

**Shipped Tokens:**

| Token | Hex | Purpose |
|-------|-----|---------|
| `accent` | F41C80 | Primary brand color |
| `slate` | 6B7280 | Neutral gray |
| `success` | 22C55E | Success states |
| `warning` | EAB308 | Warning states |
| `error` | EF4444 | Error states |
| `info` | 3B82F6 | Info states |
| `ui.bg` | 292A2D | Dark background |
| `ui.surface` | 292C34 | Elevated surface |
| `ui.panel` | 282F3C | Panel background |
| `ui.raised` | 263143 | Raised element |
| `white` | FFFFFF | Pure white |
| `black` | 000000 | Pure black |
| `ink` | 111111 | Near-black text |
| `cobalt` | 2B6CB0 | Blue accent |
| `plum` | 6B46C1 | Purple accent |

**Example:**

```rust
// Using palette color
let result = renderer.expand("swatch", &["accent".to_string()], None)?;
// accent → f41c80

// Using hex directly
let result = renderer.expand("swatch", &["ff6b35".to_string()], None)?;
// Passes through as-is
```

### Expansion Algorithm

The renderer performs these steps:

1. **Lookup:** Find component definition in `components.json`
2. **Substitute args:** Replace `$1`, `$2`, ... with provided args
3. **Substitute content:** Replace `$content` with inner text (if applicable)
4. **Resolve palette:** Replace color names with hex codes
5. **Return:** Expanded template string

**Example Trace:**

```rust
renderer.expand("tech", &["rust".to_string()], None)

1. Lookup: components.json["tech"]
   → template: "{{shields:icon:logo=$1:bg=ui.bg:logoColor=white:style=flat-square/}}"

2. Substitute $1:
   → "{{shields:icon:logo=rust:bg=ui.bg:logoColor=white:style=flat-square/}}"

3. Resolve palette (ui.bg → 292A2D, white → FFFFFF):
   → "{{shields:icon:logo=rust:bg=292A2D:logoColor=FFFFFF:style=flat-square/}}"

4. Return expanded string
```

### Error Handling

```rust
match renderer.expand("nonexistent", &[], None) {
    Ok(result) => println!("{}", result),
    Err(e) => eprintln!("Error: {}", e),
    // Prints: "Error: Unknown component 'nonexistent'"
}
```

**Common Errors:**
- `ParseError` - Component not found or invalid definition
- Component validation happens at expansion time

---

## ShieldsRenderer API

The `ShieldsRenderer` generates shields.io badge URLs as Markdown image links. **This is a primitive API**—most users should use `ComponentsRenderer` instead.

### Overview

```rust
use mdfx::ShieldsRenderer;

let renderer = ShieldsRenderer::new()?;
```

Generates URLs like:
```markdown
![](https://img.shields.io/badge/-%20-2B6CB0?style=flat-square)
```

### Creating a Renderer

```rust
use mdfx::ShieldsRenderer;

let renderer = ShieldsRenderer::new()?;
```

**Error:** Returns `Error::ParseError` if `shields.json` is malformed.

**Data Source:** `data/shields.json` - Shield styles and palette

### Methods

#### `render_block(color: &str, style: &str) -> Result<String>`

Generate a single solid color block.

```rust
let result = renderer.render_block("2B6CB0", "flat-square")?;
// Returns: "![](https://img.shields.io/badge/-%20-2B6CB0?style=flat-square)"

// Using palette color
let result = renderer.render_block("cobalt", "flat-square")?;
// Returns: "![](https://img.shields.io/badge/-%20-2B6CB0?style=flat-square)"
```

**Parameters:**
- `color` - Palette name or 6-digit hex (no `#`)
- `style` - Shield style ID or alias

**Use Cases:**
- Status indicators
- Color swatches
- Simple dividers

#### `render_twotone(left: &str, right: &str, style: &str) -> Result<String>`

Generate a two-color block (left/right split).

```rust
let result = renderer.render_twotone("111111", "2B6CB0", "flat-square")?;
// Returns: "![](https://img.shields.io/badge/-%20-2B6CB0?style=flat-square&label=&labelColor=111111)"
```

**Parameters:**
- `left` - Left side color (palette name or hex)
- `right` - Right side color (palette name or hex)
- `style` - Shield style

**Use Cases:**
- Dual-tone design elements
- Before/after comparisons

#### `render_bar(colors: &[String], style: &str) -> Result<String>`

Generate multiple inline color blocks.

```rust
let colors = vec![
    "22C55E".to_string(),  // success
    "F59E0B".to_string(),  // warning
    "DC2626".to_string(),  // error
];
let result = renderer.render_bar(&colors, "flat-square")?;
// Returns: "![](...)![](...)![](...)"  (3 inline badges)
```

**Parameters:**
- `colors` - Slice of colors (palette names or hex)
- `style` - Shield style

**Use Cases:**
- Progress bars
- Multi-color dividers
- Status dashboards

#### `render_icon(logo: &str, bg: &str, logo_color: &str, style: &str) -> Result<String>`

Generate a logo chip with Simple Icons.

```rust
let result = renderer.render_icon("rust", "000000", "white", "flat-square")?;
// Returns: "![](https://img.shields.io/badge/-%20-000000?style=flat-square&logo=rust&logoColor=FFFFFF&label=&labelColor=000000)"
```

**Parameters:**
- `logo` - Simple Icons slug (e.g., "rust", "python", "postgresql")
- `bg` - Background color (palette name or hex)
- `logo_color` - Logo color (palette name or hex)
- `style` - Shield style

**Available Logos:** 2000+ from [simpleicons.org](https://simpleicons.org/)

**Use Cases:**
- Tech stack badges
- Tool/service indicators
- Integration logos

#### `resolve_color(color: &str) -> Result<String>`

Resolve a color from palette or validate hex.

```rust
let hex = renderer.resolve_color("cobalt")?;
// Returns: "2B6CB0"

let hex = renderer.resolve_color("ABC123")?;
// Returns: "ABC123" (validated and uppercased)

let hex = renderer.resolve_color("invalid");
// Err: InvalidColor
```

**Validation:**
- Palette lookup first
- Falls back to hex validation (must be 6 hex digits)
- Returns uppercase hex

#### `has_style(name: &str) -> bool`

Check if a shield style exists (by ID or alias).

```rust
if renderer.has_style("flat-square") {
    println!("Style exists!");
}

if renderer.has_style("flat") {  // Alias
    println!("Alias works too!");
}
```

#### `list_styles() -> Vec<&ShieldStyle>`

Get all available shield styles.

```rust
for style in renderer.list_styles() {
    println!("{}: {}", style.id, style.name);
    if !style.aliases.is_empty() {
        println!("  Aliases: {:?}", style.aliases);
    }
}
```

**Output:**
```
flat-square: Flat Square
  Aliases: ["flat", "square"]
for-the-badge: For The Badge
  Aliases: ["badge", "header"]
plastic: Plastic
social: Social
```

#### `list_palette() -> Vec<(&String, &String)>`

Get all palette colors.

```rust
for (name, hex) in renderer.list_palette() {
    println!("{}: #{}", name, hex);
}
```

### Shield Styles

**Shipped Styles:**

| ID | Name | Aliases | Visual |
|----|------|---------|--------|
| `flat-square` | Flat Square | `flat`, `square` | Minimal, clean |
| `for-the-badge` | For The Badge | `badge`, `header` | Tall, bold |
| `plastic` | Plastic | - | Glossy, 3D |
| `social` | Social | - | GitHub-style |

### Palette

Same 15 colors as ComponentsRenderer (shared `shields.json`).

### Error Handling

```rust
// Invalid color
match renderer.render_block("invalid", "flat-square") {
    Ok(_) => {},
    Err(e) => eprintln!("{}", e),
    // Prints: "Invalid color 'invalid' (use palette name or 6-digit hex)"
}

// Unknown style
match renderer.render_block("cobalt", "nonexistent") {
    Ok(_) => {},
    Err(e) => eprintln!("{}", e),
    // Prints: "Unknown shield style 'nonexistent'"
}
```

---


## Converter API

The `Converter` transforms text using Unicode character mappings.

### Creating a Converter

```rust
use mdfx::Converter;

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
- `★` (U+2605) - Black star
- `◆` (U+25C6) - Black diamond
- `|` (U+007C) - Vertical bar
- `⚡` - Lightning bolt (any Unicode character works!)

**Note:** In template syntax, use named separators from `data/separators.json` (e.g., `separator=dot`) or any single Unicode character directly (e.g., `separator=⚡`). Named separators provide discoverability - run `mdfx separators` to see all 12 predefined options.

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
use mdfx::FrameRenderer;

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
use mdfx::BadgeRenderer;

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
use mdfx::TemplateParser;

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

For complete template syntax reference including all tag types, parameters, nesting rules, and edge cases, see **[Template Syntax Reference](TEMPLATE-SYNTAX.md)**.

**Quick examples:**
```markdown
{{style}}text{{/style}}                    ← Style template
{{style:separator=dot}}text{{/style}}      ← With parameter
{{frame:type}}text{{/frame}}               ← Frame template
{{badge:type}}char{{/badge}}               ← Badge template
{{ui:component/}}                          ← Self-closing component
{{ui:component:arg}}content{{/ui}}         ← Block component
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

## Multi-Backend Rendering

mdfx supports multiple rendering backends for generating visual primitives:
- **ShieldsBackend**: Generates shields.io badge URLs (inline Markdown) - Default
- **SvgBackend**: Generates local SVG files with asset manifest - Shipped in v1.0.0

### Renderer Trait

The core rendering abstraction is the `Renderer` trait:

```rust
use mdfx::renderer::{Renderer, RenderedAsset};
use mdfx::primitive::Primitive;

pub trait Renderer {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
}
```

### RenderedAsset Types

Rendering produces one of two asset types:

```rust
pub enum RenderedAsset {
    /// Inline Markdown (e.g., shields.io URL wrapped in ![](url))
    InlineMarkdown(String),

    /// File-based asset (e.g., generated SVG file)
    File {
        /// Relative path to the generated file
        relative_path: String,
        /// File contents as bytes
        bytes: Vec<u8>,
        /// Markdown reference to embed
        markdown_ref: String,
        /// The primitive that generated this asset
        primitive: Primitive,
    },
}
```

### Using ShieldsBackend

The `ShieldsBackend` generates inline shields.io URLs:

```rust
use mdfx::renderer::shields::ShieldsBackend;
use mdfx::renderer::Renderer;
use mdfx::primitive::Primitive;

let backend = ShieldsBackend::new()?;

let primitive = Primitive::simple_swatch("F41C80", "flat-square");
// Equivalent to:
// Primitive::Swatch {
//     color: "F41C80".to_string(),
//     style: "flat-square".to_string(),
//     opacity: None, width: None, height: None,
//     border_color: None, border_width: None, label: None,
// }

let asset = backend.render(&primitive)?;

// asset.to_markdown() returns: "![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)"
println!("{}", asset.to_markdown());
```

### Backend Selection

When using the `TemplateParser`, backends are selected via the `--backend` CLI flag:

```bash
# Use shields.io URLs (default, inline)
mdfx process input.md --backend shields

# Generate local SVG files
mdfx process input.md --backend svg --assets-dir assets/mdfx
```

**API Usage:**

The parser defaults to `ShieldsBackend`, but you can specify a different backend at construction:

```rust
use mdfx::renderer::SvgBackend;

// Use SVG backend instead of shields.io
let backend = Box::new(SvgBackend::new("assets/mdfx")?);
let parser = TemplateParser::with_backend(backend)?;

let (output, assets) = parser.process_with_assets(input)?;

// Write output markdown
std::fs::write("output.md", output)?;

// Write SVG asset files
for asset in assets {
    std::fs::write(&asset.relative_path, asset.bytes)?;
}
```

### Asset Characteristics

**InlineMarkdown:**
- No file I/O required
- Instant rendering
- Depends on external service (shields.io)
- No caching needed

**File (SVG):**
- Requires file writing
- Deterministic filenames (content-hashed)
- Offline rendering
- Tracked in manifest (see Asset Manifest System below)

---

## Separators System

The Separators System provides data-driven character resolution for text styling, supporting both named separators and direct Unicode characters.

### Overview

Separators are used with text converters to add visual spacing between characters:

```rust
let converter = Converter::new()?;
let result = converter.convert_with_separator("TITLE", "mathbold", "·", 1)?;
// Output: 𝐓·𝐈·𝐓·𝐋·𝐄
```

### Separator Resolution

The system supports two input methods:

1. **Named Separators**: Predefined characters from the unified registry
2. **Direct Unicode**: Any single Unicode character (grapheme cluster)

### SeparatorsData API

```rust
use mdfx::separators::SeparatorsData;

// Singleton instance (lazy_static)
let separators = &mdfx::separators::SEPARATORS;

// Resolve named separator
let char = separators.resolve("dot")?;  // Returns "·"

// Resolve direct Unicode
let char = separators.resolve("⚡")?;   // Returns "⚡"

// List all named separators
for (id, sep) in separators.list() {
    println!("{}: {} ({})", id, sep.char, sep.name);
}
```

### Named Separators

**Shipped Separators:**

| ID | Character | Unicode | Name | Example |
|----|-----------|---------|------|---------|
| `dot` | `·` | U+00B7 | Middle Dot | 𝐓·𝐈·𝐓·𝐋·𝐄 |
| `bullet` | `•` | U+2022 | Bullet | 𝐓•𝐈•𝐓•𝐋•𝐄 |
| `dash` | `─` | U+2500 | Light Horizontal | 𝐓─𝐈─𝐓─𝐋─𝐄 |
| `bolddash` | `━` | U+2501 | Heavy Horizontal | 𝐓━𝐈━𝐓━𝐋━𝐄 |
| `arrow` | `→` | U+2192 | Rightward Arrow | 𝐓→𝐈→𝐓→𝐋→𝐄 |
| `star` | `★` | U+2605 | Black Star | 𝐓★𝐈★𝐓★𝐋★𝐄 |
| `diamond` | `◆` | U+25C6 | Black Diamond | 𝐓◆𝐈◆𝐓◆𝐋◆𝐄 |
| `pipe` | `|` | U+007C | Vertical Line | 𝐓|𝐈|𝐓|𝐋|𝐄 |
| `slash` | `/` | U+002F | Solidus | 𝐓/𝐈/𝐓/𝐋/𝐄 |
| `double` | `═` | U+2550 | Double Horizontal | 𝐓═𝐈═𝐓═𝐋═𝐄 |
| `wave` | `∼` | U+223C | Tilde Operator | 𝐓∼𝐈∼𝐓∼𝐋∼𝐄 |
| `section` | `§` | U+00A7 | Section Sign | 𝐓§𝐈§𝐓§𝐋§𝐄 |

### Using Separators in Templates

**Named separator:**
```markdown
{{mathbold:separator=dot}}TITLE{{/mathbold}}
```

**Direct Unicode:**
```markdown
{{mathbold:separator=⚡}}TITLE{{/mathbold}}
```

**Programmatic usage:**
```rust
// Using named separator
let result = converter.convert_with_separator("BOLD", "mathbold", "·", 1)?;

// Using any Unicode character
let result = converter.convert_with_separator("FLOW", "mathbold", "→", 1)?;

// Using emoji
let result = converter.convert_with_separator("ZAP", "mathbold", "⚡", 1)?;
```

### Grapheme Cluster Support

The system properly handles complex Unicode using grapheme clusters (via `unicode-segmentation` crate):

```rust
// These all work correctly as single separators
separators.resolve("👨‍💻")?;  // Emoji with variation selector
separators.resolve("🇺🇸")?;  // Flag emoji
separators.resolve("é")?;    // Composed character
```

**Why this matters:**
- Simple character counting breaks for emoji: `"👨‍💻".chars().count()` returns 5
- Grapheme counting works: `"👨‍💻".graphemes(true).count()` returns 1
- This ensures emoji and complex Unicode work as single separators

### Validation Rules

The resolver applies these validations:

1. **Whitespace trimming**: Leading/trailing spaces removed
2. **Single grapheme**: Input must be exactly one grapheme cluster
3. **Reserved characters**: Cannot use `:`, `/`, `}` (template delimiters)
4. **Empty rejection**: Empty strings are rejected

**Examples:**

```rust
// Valid
separators.resolve("·")?;      // ✓ Named separator
separators.resolve("★")?;      // ✓ Direct Unicode
separators.resolve("⚡")?;      // ✓ Emoji

// Invalid
separators.resolve("")?;       // ✗ Empty string
separators.resolve("  · ")?;   // ✓ Trimmed to "·"
separators.resolve(":")?;      // ✗ Reserved for templates
separators.resolve("abc")?;    // ✗ Multiple graphemes
```

### Error Messages

The resolver provides helpful error messages with suggestions:

```rust
match separators.resolve("dott") {
    Err(msg) => println!("{}", msg),
    // Prints:
    // Unknown separator 'dott'.
    //   Did you mean: dot?
    //   Available named separators: dot, bullet, dash, ...
    //   Or use any single Unicode character (e.g., separator=⚡)
    _ => {}
}
```

### CLI Commands

**List all separators:**
```bash
mdfx separators
```

**Output:**
```
Available Separators
────────────────────

dot          · (U+00B7)  Middle Dot
             Example: 𝐓·𝐈·𝐓·𝐋·𝐄

bullet       • (U+2022)  Bullet
             Example: 𝐓•𝐈•𝐓•𝐋•𝐄
...
```

### Data Format

Separators are defined in the unified `registry.json`:

```json
{
  "separators": {
    "dot": {
      "name": "Middle Dot",
      "char": "·",
      "unicode": "U+00B7",
      "description": "Middle dot separator for elegant spacing",
      "example": "𝐓·𝐈·𝐓·𝐋·𝐄"
    }
  }
}
```

**Note:** Separators are embedded at compile time from the registry. Runtime customization is not currently supported for separators.

---

## Asset Manifest System

The Asset Manifest System tracks generated SVG assets with SHA-256 hashing for verification, cleanup, and CI optimization.

### Overview

When using the `svg` backend, mdfx generates:
1. **SVG files** in the assets directory (e.g., `assets/mdfx/divider_a3f8e2.svg`)
2. **manifest.json** listing all assets with metadata

### AssetManifest API

```rust
use mdfx::manifest::{AssetManifest, AssetEntry, VerificationResult};
use std::path::Path;

// Create new manifest
let mut manifest = AssetManifest::new("svg".to_string(), "assets/mdfx".to_string());

// Add asset
let bytes = b"<svg>...</svg>";
let primitive = Primitive::Divider { /* ... */ };
manifest.add_asset(
    "assets/mdfx/divider_abc123.svg".to_string(),
    bytes,
    &primitive,
    "svg".to_string(),
);

// Save manifest
manifest.write(Path::new("assets/mdfx/manifest.json"))?;

// Load manifest
let manifest = AssetManifest::load(Path::new("assets/mdfx/manifest.json"))?;

// Verify assets
let results = manifest.verify(Path::new("."));
for result in results {
    match result {
        VerificationResult::Valid { path } => println!("✓ {}", path),
        VerificationResult::Missing { path } => println!("✗ {} (missing)", path),
        VerificationResult::HashMismatch { path, expected, actual } => {
            println!("✗ {} (hash mismatch)", path);
        }
        _ => {}
    }
}
```

### Manifest Structure

**manifest.json format:**

```json
{
  "version": "1.0.0",
  "created_at": "2025-12-13T10:30:00Z",
  "backend": "svg",
  "assets_dir": "assets/mdfx",
  "total_assets": 3,
  "assets": [
    {
      "path": "assets/mdfx/divider_a3f8e2.svg",
      "sha256": "a3f8e2d1c4b5a6f7e8d9c0b1a2f3e4d5c6b7a8f9e0d1c2b3a4f5e6d7c8b9a0f1",
      "type": "svg",
      "primitive": {
        "kind": "Bar",
        "colors": ["292a2d", "292c34", "f41c80", "282f3c"],
        "style": "flat-square"
      },
      "size_bytes": 1234
    }
  ]
}
```

### CLI Commands

**Verify asset integrity:**
```bash
mdfx verify --assets-dir assets/mdfx
```

**Output:**
```
Manifest: assets/mdfx/manifest.json (2025-12-13T10:30:00Z)
Backend: svg
Total assets: 3

Verifying assets...
  ✓ assets/mdfx/divider_a3f8e2.svg
  ✓ assets/mdfx/swatch_f41c80.svg
  ✗ assets/mdfx/badge_rust.svg (missing)

Summary:
  ✓ Valid: 2
  ✗ Missing: 1
  ✗ Hash mismatches: 0
```

**Clean unreferenced assets:**
```bash
# Dry run (preview)
mdfx clean --assets-dir assets/mdfx --dry-run

# Delete orphaned files
mdfx clean --assets-dir assets/mdfx
```

**Output:**
```
Manifest: assets/mdfx/manifest.json
Backend: svg

Scanning for unreferenced assets...
  - assets/mdfx/old_badge_123abc.svg (orphaned)
  - assets/mdfx/temp_file.svg (orphaned)

Would delete 2 files (1.2 KB)
Run without --dry-run to delete.
```

### Use Cases

#### 1. CI Caching

Cache SVG assets across CI runs:

```yaml
# .github/workflows/docs.yml
- name: Cache mdfx assets
  uses: actions/cache@v3
  with:
    path: assets/mdfx
    key: mdfx-${{ hashFiles('assets/mdfx/manifest.json') }}
```

**How it works:**
- Manifest content hash changes only when assets change
- CI restores cached assets if manifest unchanged
- Faster builds by skipping SVG regeneration

#### 2. Asset Verification

Verify assets haven't been corrupted or manually edited:

```bash
# In CI or pre-commit hook
mdfx verify --assets-dir assets/mdfx
if [ $? -ne 0 ]; then
  echo "Asset verification failed!"
  exit 1
fi
```

#### 3. Cleanup

Remove old assets after refactoring:

```bash
# Regenerate all assets
mdfx process docs/*.md --backend svg

# Remove orphaned files
mdfx clean --assets-dir assets/mdfx
```

### Deterministic Builds

Assets use content-based filenames (SHA-256 hash prefix):

```
divider_a3f8e2.svg  ← Hash of SVG content
swatch_f41c80.svg   ← Hash of SVG content
```

**Benefits:**
- Same primitive → same filename (reproducible builds)
- Different content → different filename (no overwrites)
- CI can detect changes by comparing manifest

### Manifest Metadata

Each asset entry includes:

| Field | Type | Description |
|-------|------|-------------|
| `path` | string | Relative path from repo root |
| `sha256` | string | Content hash (64 hex chars) |
| `type` | string | Asset type (always "svg" for now) |
| `primitive` | object | Primitive that generated this asset |
| `size_bytes` | number | File size in bytes |

**Primitive tracking** enables:
- Reproducible builds (same input → same output)
- Debug tracing (which template generated this?)
- Selective regeneration (only rebuild changed primitives)

### Future Enhancements

Planned features:

- **Incremental updates**: Only regenerate changed primitives
- **Asset deduplication**: Reuse identical assets
- **Compression tracking**: SVG optimization metadata
- **Multi-format support**: Track PNG, WebP alongside SVG

---

## GitHub Blocks

GitHub Blocks are specialized components optimized for GitHub's Markdown renderer, using blockquotes and badges to create professional READMEs within GitHub's constraints.

### Overview

Three components provide GitHub-compatible layouts:
- **section** - Headers with dividers
- **callout-github** - Blockquote callouts
- **statusitem** - Inline status badges

### section Component

Creates section headers with automatic visual dividers.

**Syntax:** `{{ui:section:TITLE/}}`

**Component Type:** Expand (self-closing)

**Template Expansion:**
```markdown
## TITLE
{{ui:divider/}}
```

**Example:**
```rust
use mdfx::TemplateParser;

let parser = TemplateParser::new()?;
let input = "{{ui:section:Installation/}}";
let result = parser.process(input)?;

// Outputs:
// ## Installation
// ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)...
```

**Use Cases:**
- README section organization
- Documentation structure
- Visual hierarchy

### callout-github Component

Creates GitHub-compatible blockquote callouts with status indicators.

**Syntax:** `{{ui:callout-github:TYPE}}CONTENT{{/ui}}`

**Component Type:** Expand (block, with blockquote post-processing)

**Types:**
| Type | Color | Use For |
|------|-------|---------|
| `success` | Green (#22C55E) | Achievements, releases |
| `info` | Blue (#3B82F6) | Information, tips |
| `warning` | Yellow (#EAB308) | Breaking changes, deprecations |
| `error` | Red (#EF4444) | Security notices, critical issues |

**Expansion Process:**
1. Template substitution: `{{ui:status:TYPE/}} **Note**\nCONTENT`
2. **Post-processing:** Apply blockquote formatter

**Blockquote Post-Processor:**
```rust
// Prefixes every line with "> "
// Empty lines become ">" (no trailing space)
content.lines()
    .map(|line| {
        if line.trim().is_empty() {
            ">".to_string()
        } else {
            format!("> {}", line)
        }
    })
    .join("\n")
```

**Example:**
```rust
use mdfx::TemplateParser;

let parser = TemplateParser::new()?;
let input = r#"{{ui:callout-github:warning}}
**Breaking Changes**
API v1 will be removed in v2.0.
{{/ui}}"#;

let result = parser.process(input)?;

// Outputs:
// > ![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) **Note**
// >
// > **Breaking Changes**
// > API v1 will be removed in v2.0.
```

**Multiline Handling:**
- Preserves empty lines as `">"` in blockquote
- Supports nested Markdown (lists, bold, links)
- Maintains indentation within content

### statusitem Component

Creates inline status badges with labels.

**Syntax:** `{{ui:statusitem:LABEL:LEVEL:TEXT/}}`

**Component Type:** Expand (self-closing)

**Args:**
- `LABEL` - Display label (e.g., "Build", "Tests")
- `LEVEL` - Status level (`success`, `warning`, `error`, `info`)
- `TEXT` - Status text (e.g., "passing", "189")

**Template Expansion:**
```markdown
{{ui:status:LEVEL/}} **LABEL**: TEXT
```

**Example:**
```rust
use mdfx::TemplateParser;

let parser = TemplateParser::new()?;
let input = "{{ui:statusitem:Build:success:passing/}}";
let result = parser.process(input)?;

// Outputs:
// ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Build**: passing
```

**Composing Status Rows:**
```rust
let input = r#"{{ui:statusitem:Build:success:✓/}} · {{ui:statusitem:Tests:success:217/}} · {{ui:statusitem:Coverage:info:94%/}}"#;
let result = parser.process(input)?;

// Outputs inline row:
// ![](badge1) **Build**: ✓ · ![](badge2) **Tests**: 217 · ![](badge3) **Coverage**: 94%
```

### PostProcess API

The post-processing system applies transformations after template expansion.

**Enum Definition:**
```rust
use mdfx::PostProcess;

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum PostProcess {
    #[default]
    None,
    Blockquote,
}
```

**Usage in components.json:**
```json
{
  "callout-github": {
    "type": "expand",
    "self_closing": false,
    "template": "{{ui:status:$1/}} **Note**\n$content",
    "post_process": "blockquote"
  }
}
```

**Adding Custom Post-Processors:**

1. Add variant to `PostProcess` enum
2. Implement handler in `ComponentsRenderer::expand_template()`
3. Update component definitions with `"post_process": "your_processor"`

**Example (future):**
```rust
pub enum PostProcess {
    None,
    Blockquote,
    JoinWithSeparator(String),  // Future: auto-join items
    IndentBy(usize),            // Future: add indentation
}
```

### GitHub Blocks Examples

**Complete README Section:**
```rust
let template = r#"
{{ui:section:Features/}}

{{ui:callout-github:success}}
**Production Ready**
Used by Blackwell Systems in production since 2025.
{{/ui}}

{{ui:statusitem:Build:success:passing/}} · {{ui:statusitem:Tests:success:217/}}
"#;

let parser = TemplateParser::new()?;
let result = parser.process(template)?;
```

**Output:**
```markdown
## Features
![](divider_badges...)

> ![](green_badge) **Note**
>
> **Production Ready**
> Used by Blackwell Systems in production since 2025.

![](green_badge) **Build**: passing · ![](green_badge) **Tests**: 217
```

### Design Rationale

**Why blockquotes instead of custom HTML?**
- Works in GitHub issues, PRs, discussions
- Renders in email notifications
- Screen reader compatible
- No CSP violations

**Why shields.io for status indicators?**
- Widely cached CDN
- Standard badge format
- Customizable colors
- No maintenance burden

**Why manual composition for status rows?**
- Keeps initial implementation simple
- Users can customize separator (` · `, ` | `, emoji)
- Auto-joining planned for v1.2 (`statusrow` component)

### Best Practices

**Section Headers:**
- Use for major document breaks
- Keep titles concise (1-3 words)
- Use sentence case

**Callouts:**
- 2-4 lines ideal
- Match type to content severity
- Include actionable information

**Status Rows:**
- Group related metrics
- Use consistent levels (all success, or mixed intentionally)
- Keep to 3-5 items per row

### See Also

- [examples/github-blocks.md](../examples/github-blocks.md) - Complete gallery
- [GITHUB-BLOCKS-PLAN.md](GITHUB-BLOCKS-PLAN.md) - Implementation details
- [GitHub Blocks section in ARCHITECTURE.md](ARCHITECTURE.md#github-blocks)

---

## Enhanced Swatch Options

**Version:** 1.0.0

Swatch primitives support advanced SVG-only styling options.

### Available Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `opacity` | `f32` | `1.0` | Transparency (0.0 = transparent, 1.0 = opaque) |
| `width` | `u32` | `20` | Width in pixels |
| `height` | `u32` | style-dependent | Height in pixels |
| `border_color` | `String` | none | Border color (hex or palette name) |
| `border_width` | `u32` | `0` | Border width in pixels |
| `label` | `String` | none | Text label inside swatch |

### API Usage

```rust
use mdfx::Primitive;

// Simple swatch (helper method)
let simple = Primitive::simple_swatch("F41C80", "flat-square");

// Enhanced swatch (all options)
let enhanced = Primitive::Swatch {
    color: "F41C80".to_string(),
    style: "flat-square".to_string(),
    opacity: Some(0.8),
    width: Some(40),
    height: Some(30),
    border_color: Some("FFFFFF".to_string()),
    border_width: Some(2),
    label: Some("v1".to_string()),
};
```

### Template Syntax

```markdown
{{ui:swatch:FF6B35:opacity=0.5/}}
{{ui:swatch:accent:width=40:height=30/}}
{{ui:swatch:cobalt:border=FFFFFF:border_width=2/}}
{{ui:swatch:F41C80:label=v1/}}
```

### Backend Compatibility

| Feature | shields.io | SVG |
|---------|------------|-----|
| Basic color | ✅ | ✅ |
| Style | ✅ | ✅ |
| Opacity | ❌ (ignored) | ✅ |
| Custom size | ❌ (ignored) | ✅ |
| Border | ❌ (ignored) | ✅ |
| Label | ❌ (ignored) | ✅ |

Enhanced options gracefully degrade on shields.io backend.

---

## Registry API

**Version:** 1.0.0

The unified registry provides access to all compiler data.

### Loading the Registry

```rust
use mdfx::Registry;

let registry = Registry::new()?;
```

### Available Data

```rust
// Palette colors
for (name, hex) in &registry.palette {
    println!("{}: #{}", name, hex);
}

// Text styles
for (id, style) in &registry.styles {
    println!("{}: {}", id, style.name);
}

// Frames
for (id, frame) in &registry.frames {
    println!("{}: {}{{}}{}", id, frame.prefix, frame.suffix);
}

// Badges
for (id, badge) in &registry.badges {
    println!("{}: {}", id, badge.name);
}

// Separators
for (id, sep) in &registry.separators {
    println!("{}: {} ({})", id, sep.char, sep.unicode);
}

// Shield styles
for (id, style) in &registry.shield_styles {
    println!("{}: {}", id, style.name);
}

// Components
for (id, comp) in &registry.components {
    println!("{}: {}", id, comp.description);
}
```

### Resolving Renderables

```rust
use mdfx::{Registry, EvalContext, ResolvedRenderable};

let registry = Registry::new()?;

// Resolve a frame with context filtering
if let Some(resolved) = registry.resolve("gradient", EvalContext::GitHub) {
    match resolved {
        ResolvedRenderable::Frame(f) => println!("Frame: {}{{}}{}", f.prefix, f.suffix),
        ResolvedRenderable::Badge(b) => println!("Badge: {}", b.name),
        ResolvedRenderable::Style(s) => println!("Style: {}", s.name),
        ResolvedRenderable::Component(c) => println!("Component: {}", c.description),
    }
}
```

### EvalContext Filtering

```rust
use mdfx::EvalContext;

// Available contexts
let contexts = [
    EvalContext::Cli,      // Command line
    EvalContext::GitHub,   // GitHub README
    EvalContext::Npm,      // npm docs
    EvalContext::Local,    // Local/offline
];

// Get renderables valid for a context
let github_renderables = registry.list_for_context(EvalContext::GitHub);
```

---

## Error Handling

All errors implement `std::error::Error` and use the `thiserror` crate.

### Error Types

```rust
use mdfx::Error;

match result {
    Err(Error::UnknownStyle(name)) => {
        eprintln!("Style '{}' not found", name);
        eprintln!("Run `mdfx list` to see available styles");
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
use mdfx::TemplateParser;

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
use mdfx::{Converter, FrameRenderer, TemplateParser};
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
use mdfx::TemplateParser;
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

**Last Updated:** 2025-12-14

**Version:** 1.0.0 - Added Target System, Custom Palette Support, Enhanced Swatch Options, Registry API
