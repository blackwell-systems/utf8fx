# ▓▒░ 𝐌·𝐃·𝐅·𝐗 ░▒▓

[![Blackwell Systems™](https://raw.githubusercontent.com/blackwell-systems/blackwell-docs-theme/main/badge-trademark.svg)](https://github.com/blackwell-systems)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-217_passing-22c55e?style=flat-square)](https://github.com/blackwell-systems/mdfx/actions)

𝗠𝗮𝗿𝗸𝗱𝗼𝘄𝗻 𝗲𝗳𝗳𝗲𝗰𝘁𝘀: 𝗨𝗻𝗶𝗰𝗼𝗱𝗲 𝘁𝗲𝘅𝘁 𝘀𝘁𝘆𝗹𝗶𝗻𝗴 𝗮𝗻𝗱 𝗨𝗜 𝗰𝗼𝗺𝗽𝗼𝗻𝗲𝗻𝘁𝘀

Transform text into various Unicode styles through a powerful template system. Create distinctive visual elements
for READMEs, documentation, and presentations without images or external dependencies.

## 𝐐𝐮𝐢𝐜𝐤 𝐒𝐭𝐚𝐫𝐭

```markdown
# {{ui:header}}PROJECT NAME{{/ui}}

{{ui:divider/}}

## Tech Stack
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:postgresql/}}

## Status
{{ui:status:success/}} All systems operational
```

Renders as:

# ▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓·𝐍·𝐀·𝐌·𝐄 ░▒▓

![](https://img.shields.io/badge/...) (colorful divider bar)

🦀 🐍 🐘 (tech badges)

🟢 All systems operational

## 𝐌𝐨𝐭𝐢𝐯𝐚𝐭𝐢𝐨𝐧

Unicode offers diverse styling options—from elegant 𝓼𝓬𝓻𝓲𝓹𝓽 to bold 𝔣𝔯𝔞𝔨𝔱𝔲𝔯 to playful Ⓒⓘⓡⓒⓛⓔⓢ—but they're
cumbersome to use. Finding glyphs requires hunting through Unicode tables and manually spacing them.

**mdfx** makes Unicode styling simple and repeatable. Use intuitive templates like `{{ui:header}}TITLE{{/ui}}`
or CLI commands like `mdfx convert --style script "Elegant"`.

Transform Unicode styling from a frustrating manual process into something as easy as markdown formatting.

## 𝐖𝐡𝐲 𝐦𝐝𝐟𝐱?

**Why not just copy/paste Unicode characters?**

- **Repeatability**: Reuse `{{ui:header}}TITLE{{/ui}}` across dozens of files
- **Consistency**: Change style once, regenerate all docs - instant rebrand
- **Maintainability**: Source files remain readable ASCII, styled output is generated
- **Search & Replace**: Find/replace works on template names, not opaque glyphs
- **Version Control**: Diffs show intent, not character code changes
- **Composability**: Combine components in ways copy/paste can't match

Think of it like CSS for text: separate content from presentation, gain power through abstraction.

## 𝐔𝐈 𝐂𝐨𝐦𝐩𝐨𝐧𝐞𝐧𝐭𝐬

mdfx provides high-level semantic components for common use cases. These compile down to shields.io badges,
frames, and character transformations.

### Visual Elements

**Dividers** - Section separators
```markdown
{{ui:divider/}}
```

**Color Swatches** - Single color blocks
```markdown
{{ui:swatch:accent/}}
{{ui:swatch:success/}}
```

**Status Indicators** - Colored badges
```markdown
{{ui:status:success/}}  → 🟢 Green block
{{ui:status:warning/}}  → 🟡 Yellow block
{{ui:status:error/}}    → 🔴 Red block
```

### Tech Stack Badges

**Technology Logos** - Simple Icons integration
```markdown
{{ui:tech:rust/}}
{{ui:tech:python/}}
{{ui:tech:postgresql/}}
{{ui:tech:docker/}}
{{ui:tech:kubernetes/}}
```

Uses [Simple Icons](https://simpleicons.org/) logo library (2000+ logos available).

### Content Blocks

**Section Headers** - Gradient frames with bold text
```markdown
{{ui:header}}INSTALLATION{{/ui}}
{{ui:header}}API REFERENCE{{/ui}}
```

**Callouts** - Framed messages with indicators
```markdown
{{ui:callout:info}}Remember to run tests{{/ui}}
{{ui:callout:warning}}Breaking change in v2.0{{/ui}}
{{ui:callout:error}}Deprecated{{/ui}}
```

### GitHub Blocks 🆕

**Section Headers** - Headers with automatic dividers
```markdown
{{ui:section:Installation/}}
{{ui:section:Features/}}
```

**GitHub Callouts** - Blockquote-style callouts optimized for GitHub
```markdown
{{ui:callout-github:warning}}
Breaking changes in v2.0!
{{/ui}}
```

**Status Items** - Inline status badges for project metadata
```markdown
{{ui:statusitem:Build:success:passing/}} · {{ui:statusitem:Tests:success:217/}}
```

These components work within GitHub's Markdown constraints (no custom HTML/CSS), using blockquotes and shields.io badges. See [examples/github-blocks.md](examples/github-blocks.md) for a complete gallery.

### Design Tokens

Components use named colors from `palette.json`:

| Token | Hex | Use |
|-------|-----|-----|
| `accent` | F41C80 | Primary brand color |
| `success` | 22C55E | Success states |
| `warning` | EAB308 | Warning states |
| `error` | EF4444 | Error states |
| `slate` | 6B7280 | Neutral gray |
| `ui.bg` | 292A2D | Dark background |
| `ui.surface` | 292C34 | Elevated surface |
| `ui.panel` | 282F3C | Panel background |

You can reference these in any component:
```markdown
{{ui:swatch:accent/}}
{{ui:status:success/}}
```

## 𝐓𝐞𝐱𝐭 𝐒𝐭𝐲𝐥𝐞𝐬

Transform text into 19 different Unicode character styles.

### Bold & Emphasis
| Style | Example | Use Case |
|-------|---------|----------|
| `mathbold` | 𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓 | Professional headers |
| `fullwidth` | ＢＬＡＣＫＤＯＴ | Substantial emphasis |
| `sans-serif-bold` | 𝗕𝗟𝗔𝗖𝗞𝗗𝗢𝗧 | Modern, strong |
| `sans-serif-bold-italic` | 𝘽𝙇𝘼𝘾𝙆𝘿𝙊𝙏 | Maximum emphasis |

### Boxed Styles
| Style | Example | Use Case |
|-------|---------|----------|
| `negative-squared` | 🅱🅻🅰🅲🅺🅳🅾🆃 | Maximum contrast |
| `negative-circled` | 🅑🅛🅐🅒🅚🅓🅞🅣 | Bold, rounded |
| `squared-latin` | 🄱🄻🄰🄲🄺🄳🄾🅃 | Elegant boxes |
| `circled-latin` | Ⓑⓛⓐⓒⓚⓓⓞⓣ | Playful circles |

### Elegant & Script
| Style | Example | Use Case |
|-------|---------|----------|
| `script` | 𝐵𝐿𝒜𝒞𝒦𝒟𝒪𝒯 | Elegant cursive |
| `bold-script` | 𝓑𝓛𝓐𝓒𝓚𝓓𝓞𝓣 | Heavy cursive |
| `fraktur` | 𝔅𝔏𝔄ℭ𝔎𝔇𝔒𝔗 | Gothic/blackletter |
| `bold-fraktur` | 𝕭𝕷𝕬𝕮𝕶𝕯𝕺𝕿 | Heavy Gothic |
| `italic` | 𝐵𝐿𝐴𝐶𝐾𝐷𝑂𝑇 | Flowing emphasis |
| `bold-italic` | 𝑩𝑳𝑨𝑪𝑲𝑫𝑶𝑻 | Strong + flow |
| `small-caps` | ʙʟᴀᴄᴋᴅᴏᴛ | Subtle elegance |

### Technical
| Style | Example | Use Case |
|-------|---------|----------|
| `monospace` | 𝚋𝚕𝚊𝚌𝚔𝚍𝚘𝚝 | Code-like |
| `double-struck` | 𝔹𝕃𝔸ℂ𝕂𝔻𝕆𝕋 | Outline style |
| `sans-serif` | 𝖡𝖫𝖠𝖢𝖪𝖣𝖮𝖳 | Clean, modern |
| `sans-serif-italic` | 𝘉𝘓𝘈𝘊𝘒𝘋𝘖𝘛 | Modern slant |

### Style Modifiers

**Separators** - Add characters between letters
```markdown
{{mathbold:separator=dot}}TITLE{{/mathbold}}     → 𝐓·𝐈·𝐓·𝐋·𝐄
{{mathbold:separator=bullet}}CODE{{/mathbold}}   → 𝐂•𝐎•𝐃•𝐄
{{mathbold:separator=arrow}}FLOW{{/mathbold}}    → 𝐅→𝐎→𝐖
{{mathbold:separator=⚡}}POWER{{/mathbold}}       → 𝐏⚡𝐎⚡𝐖⚡𝐄⚡𝐑
```

**12 named separators:** `dot`, `bullet`, `dash`, `bolddash`, `arrow`, `star`, `diamond`, `square`, `circle`, `pipe`, `slash`, `tilde`

**Or use any Unicode character:** Any single character works directly. Run `mdfx separators` for details.

**Spacing** - Add spaces between characters
```markdown
{{mathbold:spacing=1}}HELLO{{/mathbold}}  → 𝐇 𝐄 𝐋 𝐋 𝐎
{{mathbold:spacing=2}}WIDE{{/mathbold}}   → 𝐖  𝐈  𝐃  𝐄
```

## 𝐈𝐧𝐬𝐭𝐚𝐥𝐥𝐚𝐭𝐢𝐨𝐧

mdfx is distributed as two packages: a library crate (`mdfx`) and a CLI tool (`mdfx-cli`).

### CLI Tool

Install the command-line tool to process markdown files:

```bash
cargo install mdfx-cli
```

This installs the `mdfx` binary for terminal use.

### Library

Add mdfx as a dependency in your Rust project:

```toml
[dependencies]
mdfx = "1.0"
```

Then use it programmatically:

```rust
use mdfx::{Converter, TemplateParser};

let converter = Converter::new()?;
let result = converter.convert("HELLO", "mathbold")?;
// result: "𝐇𝐄𝐋𝐋𝐎"
```

### From Source

```bash
git clone https://github.com/blackwell-systems/mdfx
cd mdfx
cargo build --release --workspace
./target/release/mdfx --version
```

## 𝐏𝐫𝐨𝐣𝐞𝐜𝐭 𝐒𝐭𝐫𝐮𝐜𝐭𝐮𝐫𝐞

mdfx uses a Cargo workspace with separate library and CLI crates:

```
mdfx/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── mdfx/                     # Library crate
│   │   ├── Cargo.toml           # Package: mdfx
│   │   ├── data/                # JSON data files
│   │   └── src/                 # Core library
│   └── mdfx-cli/                # CLI crate
│       ├── Cargo.toml           # Package: mdfx-cli
│       └── src/main.rs          # Binary: mdfx
```

**Benefits:**
- Library users don't need CLI dependencies (clap, colored)
- Clean separation of concerns
- Binary still named `mdfx` for user experience

## 𝐔𝐬𝐚𝐠𝐞

### Library API

Use mdfx programmatically in your Rust projects:

```rust
use mdfx::{Converter, TemplateParser};

// Convert text to Unicode styles
let converter = Converter::new()?;
let bold = converter.convert("HELLO", "mathbold")?;
// "𝐇𝐄𝐋𝐋𝐎"

// Process markdown templates
let parser = TemplateParser::new()?;
let result = parser.process("{{mathbold}}TITLE{{/mathbold}}")?;
// "𝐓𝐈𝐓𝐋𝐄"
```

See [API Guide](docs/API-GUIDE.md) for comprehensive library documentation.

### CLI - Process Markdown Files
```bash
# Process a template file
mdfx process README.template.md > README.md

# Process and save
mdfx process input.md --output output.md

# Process from stdin
echo "{{ui:header}}HELLO{{/ui}}" | mdfx process -
```

### CLI - Direct Conversion
```bash
# Convert text directly
mdfx convert --style mathbold "HELLO WORLD"
# Output: 𝐇𝐄𝐋𝐋𝐎 𝐖𝐎𝐑𝐋𝐃

# With separator
mdfx convert --style mathbold --separator dot "TITLE"
# Output: 𝐓·𝐈·𝐓·𝐋·𝐄

# With spacing
mdfx convert --style script --spacing 2 "Elegant"
# Output: 𝐸  𝓁  𝑒  𝑔  𝒶  𝓃  𝓉
```

### CLI - List Styles
```bash
mdfx list                    # List all styles
mdfx list --category bold    # Filter by category
mdfx frames list             # List frame styles
mdfx badges list             # List badge types
```

### Rust Library
```rust
use mdfx::TemplateParser;

fn main() {
    let parser = TemplateParser::new().unwrap();

    // Process templates
    let input = "# {{ui:header}}PROJECT{{/ui}}";
    let output = parser.process(input).unwrap();

    println!("{}", output);
}
```

### Direct Conversion API
```rust
use mdfx::Converter;

fn main() {
    let converter = Converter::new().unwrap();

    // Convert with style
    let result = converter.convert("HELLO", "mathbold").unwrap();
    println!("{}", result);  // 𝐇𝐄𝐋𝐋𝐎

    // Convert with separator
    let result = converter.convert_with_separator(
        "TITLE", "mathbold", "·", 1
    ).unwrap();
    println!("{}", result);  // 𝐓·𝐈·𝐓·𝐋·𝐄
}
```

## 𝐑𝐞𝐧𝐝𝐞𝐫𝐢𝐧𝐠 𝐁𝐚𝐜𝐤𝐞𝐧𝐝𝐬

mdfx supports two rendering backends for UI components (dividers, swatches, tech badges, status indicators):

### Shields.io Backend (Default)

Generates online badge URLs that render when viewed on GitHub or in browsers.

**CLI Usage:**
```bash
mdfx process input.md -o output.md
# or explicitly:
mdfx process input.md -o output.md --backend shields
```

**Library Usage:**
```rust
use mdfx::TemplateParser;

let parser = TemplateParser::new()?;  // Uses shields.io by default
let output = parser.process(input)?;
```

**Output Example:**
```markdown
![](https://img.shields.io/badge/-22C55E?style=flat-square)
```

**When to use:**
- GitHub READMEs (renders automatically)
- Online documentation
- No local file management needed
- Always up-to-date badges

### SVG Backend

Generates local SVG files with deterministic hash-based filenames. Perfect for offline docs, version control, and reproducible builds.

**CLI Usage:**
```bash
mdfx process input.md -o output.md --backend svg --assets-dir assets/mdfx
```

**Library Usage:**
```rust
use mdfx::{TemplateParser, renderer::SvgBackend};

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

**Output Example:**
```markdown
![](assets/mdfx/swatch_8490176a786b203c.svg)
```

**Generated Files:**
```
assets/mdfx/
├── swatch_8490176a786b203c.svg
├── divider_3f7a2b1c4d5e6f89.svg
├── tech_rust_1a2b3c4d5e6f7a8b.svg
└── manifest.json
```

**Benefits:**
- **Offline-first**: No internet required to view docs
- **Version control**: SVG files tracked in git
- **Reproducible**: Same input = same filenames (deterministic hashing)
- **Fast**: No network latency
- **Privacy**: No external requests
- **Portable**: Works in any markdown viewer

### Asset Manifest

When using `--backend svg`, mdfx generates a `manifest.json` file tracking all assets:

```json
{
  "version": "1.0.0",
  "created_at": "2025-12-13T18:30:00Z",
  "backend": "svg",
  "assets_dir": "assets/mdfx",
  "total_assets": 7,
  "assets": [
    {
      "path": "assets/mdfx/swatch_8490176a786b203c.svg",
      "sha256": "2c932535cd177cd4a8e4f9b6d1a3c7e5...",
      "type": "swatch",
      "primitive": {
        "kind": "Swatch",
        "color": "f41c80",
        "style": "flat-square"
      },
      "size_bytes": 143
    }
  ]
}
```

**Use cases:**
- Verify asset integrity (SHA-256 checksums)
- Track what assets are used
- Clean up unused assets
- Audit badge parameters

### Backend Comparison

| Feature | Shields.io (Default) | SVG Backend |
|---------|---------------------|-------------|
| **Requires internet** | Yes | No |
| **File generation** | No files | Generates .svg files |
| **GitHub rendering** | Automatic | Requires committed files |
| **Version control** | URLs only | SVG files in git |
| **Reproducible builds** | No (shields.io changes) | Yes (deterministic hashing) |
| **Offline docs** | No | Yes |
| **Initial setup** | None | Need assets directory |
| **Best for** | GitHub READMEs, online docs | Offline docs, reproducible builds |

**Recommendation:**
- **GitHub projects**: Use shields.io (default)
- **Local documentation**: Use SVG backend
- **CI/CD reproducibility**: Use SVG backend

See [Architecture Guide](docs/ARCHITECTURE.md#multi-backend-rendering-architecture) for technical implementation details.

---

## 𝐀𝐝𝐯𝐚𝐧𝐜𝐞𝐝 𝐅𝐞𝐚𝐭𝐮𝐫𝐞𝐬

### Composition

Nest templates for complex effects:
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
```
Output: `▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓`

### Inline Frames

Add decorative prefix/suffix around text:
```markdown
{{frame:gradient}}TITLE{{/frame}}       → ▓▒░ TITLE ░▒▓
{{frame:solid-left}}WARNING{{/frame}}   → █▌ WARNING
{{frame:line-double}}HEADER{{/frame}}   → ═ HEADER ═
```

27 frame styles available. See `mdfx frames list`.

### Alphanumeric Badges

Enclose numbers and letters:
```markdown
{{badge:circle}}1{{/badge}}         → ①
{{badge:circle}}A{{/badge}}         → Ⓐ
{{badge:negative-circle}}2{{/badge}} → ❷
{{badge:paren}}a{{/badge}}          → ⒜
```

6 badge types available. See `mdfx badges list`.

### Low-Level Primitives (Escape Hatch)

For advanced users, direct shield rendering is available:
```markdown
{{shields:block:color=F41C80:style=flat-square/}}
{{shields:bar:colors=success,warning,error:style=flat-square/}}
```

UI components are recommended for most use cases.

## 𝐄𝐱𝐚𝐦𝐩𝐥𝐞𝐬

### Project README Header
```markdown
# {{ui:header}}BLACKWELL SYSTEMS{{/ui}}

{{ui:divider/}}

## Built With
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:postgresql/}}
```

### Status Dashboard
```markdown
## System Status

{{ui:status:success/}} API Server: Operational
{{ui:status:success/}} Database: Healthy
{{ui:status:warning/}} Cache: Degraded
```

### Documentation Sections
```markdown
{{ui:header}}INSTALLATION{{/ui}}

Follow these steps...

{{ui:divider/}}

{{ui:header}}CONFIGURATION{{/ui}}

Configure your environment...
```

### Release Notes
```markdown
# Release v2.0.0

{{ui:callout:warning}}Breaking changes in this release{{/ui}}

## New Features
- Feature A
- Feature B

{{ui:callout:info}}See migration guide for upgrade path{{/ui}}
```

## 𝐇𝐨𝐰 𝐈𝐭 𝐖𝐨𝐫𝐤𝐬

mdfx uses a three-layer architecture:

1. **UI Components** (`{{ui:*}}`) - High-level semantic elements you author
2. **Primitives** (`{{shields:*}}`, `{{frame:*}}`, `{{badge:*}}`) - Rendering engines
3. **Styles** (`{{mathbold}}`) - Character transformations

When you write `{{ui:header}}TITLE{{/ui}}`, mdfx:
1. Expands the component to `{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}`
2. Applies the frame decoration
3. Transforms characters with mathbold
4. Adds dot separators

This expansion model keeps your markdown concise while allowing full customization when needed.

## 𝐂𝐨𝐧𝐟𝐢𝐠𝐮𝐫𝐚𝐭𝐢𝐨𝐧

### Custom Palette

Create `palette.json` in your project:
```json
{
  "version": "1.0.0",
  "colors": {
    "brand": "FF6B35",
    "accent": "F41C80",
    "success": "22C55E"
  }
}
```

Then use in components:
```markdown
{{ui:swatch:brand/}}
{{ui:status:accent/}}
```

### Custom Components

Create `components.json`:
```json
{
  "version": "1.0.0",
  "components": {
    "myheader": {
      "type": "expand",
      "self_closing": false,
      "template": "{{frame:solid-left}}{{mathbold}}$content{{/mathbold}}{{/frame}}"
    }
  }
}
```

Use as:
```markdown
{{ui:myheader}}CUSTOM{{/ui}}
```

## 𝐓𝐞𝐦𝐩𝐥𝐚𝐭𝐞 𝐒𝐲𝐧𝐭𝐚𝐱

mdfx uses double-brace template syntax with two tag types:

**Self-closing** (no content):
```markdown
{{ui:divider/}}
{{ui:tech:rust/}}
```

**Block tags** (with content):
```markdown
{{ui:header}}TITLE{{/ui}}
{{mathbold}}TEXT{{/mathbold}}
```

**Parameters** (colon-separated):
```markdown
{{mathbold:separator=dot}}STYLED{{/mathbold}}
{{ui:callout:warning}}Message{{/ui}}
```

For complete syntax reference including all tag types, parameters, nesting rules, and edge cases, see **[Template Syntax Reference](docs/TEMPLATE-SYNTAX.md)**.

## 𝐏𝐫𝐨𝐣𝐞𝐜𝐭 𝐒𝐭𝐚𝐭𝐮𝐬

**Current Version:** v1.0.0

**Shipped:**
- 19 Unicode text styles with aliases
- 9 UI components (divider, swatch, tech, status, header, callout, section, callout-github, statusitem)
- 27 inline frames
- 6 alphanumeric badge types
- Data-driven separator system (12 named + direct Unicode)
- Asset manifest system (SHA-256 verification, cleanup)
- GitHub Blocks (blockquote callouts, section headers, status rows)
- Design token system (palette.json)
- Template composition and nesting
- Multi-backend rendering (shields.io, SVG)
- CLI and Rust library
- 217 passing tests

**v1.2.0 Roadmap:**
- Grid component (table generation)
- StatusRow component (auto-joining)
- Custom callout titles

**Future:**
- Pill primitive (message badges)
- WASM bindings for browser/Node.js
- VS Code extension with preview
- Watch mode for live regeneration

## 𝐂𝐨𝐧𝐭𝐫𝐢𝐛𝐮𝐭𝐢𝐧𝐠

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 𝐋𝐢𝐜𝐞𝐧𝐬𝐞

MIT License - see [LICENSE](LICENSE) for details.

## 𝐋𝐢𝐧𝐤𝐬

- [Documentation](docs/)
- [Examples](examples/)
- [Architecture Design](docs/ARCHITECTURE.md)
- [API Guide](docs/API-GUIDE.md)
- [Components Design](docs/COMPONENTS.md)
