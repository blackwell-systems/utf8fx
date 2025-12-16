# {{frame:gradient}}{{mathbold:separator=dot}}MDFX{{/mathbold}}{{/frame}}

[![Blackwell Systems™](https://raw.githubusercontent.com/blackwell-systems/blackwell-docs-theme/main/badge-trademark.svg)](https://github.com/blackwell-systems)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-174_passing-22c55e?style=flat-square)](https://github.com/blackwell-systems/mdfx/actions)

{{sans-serif-bold}}Unicode text effects for markdown and beyond{{/sans-serif-bold}}

Transform text into various Unicode styles through a powerful template system. Create distinctive visual elements
for READMEs, documentation, and presentations without images or external dependencies.

## {{mathbold}}Quick Start{{/mathbold}}

```markdown
# {{frame:gradient}}{{mathbold:separator=dot}}PROJECT NAME{{/mathbold}}{{/frame}}

## Tech Stack
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:postgresql/}}

## Status
{{ui:swatch:success/}} All systems operational
```

Renders as:

# ▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓·𝐍·𝐀·𝐌·𝐄 ░▒▓

![](https://img.shields.io/badge/...) (colorful divider bar)

🦀 🐍 🐘 (tech badges)

🟢 All systems operational

## {{mathbold}}Motivation{{/mathbold}}

Unicode offers diverse styling options—from elegant 𝓼𝓬𝓻𝓲𝓹𝓽 to bold 𝔣𝔯𝔞𝔨𝔱𝔲𝔯 to playful Ⓒⓘⓡⓒⓛⓔⓢ—but they're
cumbersome to use. Finding glyphs requires hunting through Unicode tables and manually spacing them.

**mdfx** makes Unicode styling simple and repeatable. Use intuitive templates like `{{mathbold}}TITLE{{/mathbold}}`
or CLI commands like `mdfx convert --style script "Elegant"`.

Transform Unicode styling from a frustrating manual process into something as easy as markdown formatting.

## {{mathbold}}Why mdfx?{{/mathbold}}

**Why not just copy/paste Unicode characters?**

- **Repeatability**: Reuse `{{mathbold}}TITLE{{/mathbold}}` across dozens of files
- **Consistency**: Change style once, regenerate all docs - instant rebrand
- **Maintainability**: Source files remain readable ASCII, styled output is generated
- **Search & Replace**: Find/replace works on template names, not opaque glyphs
- **Version Control**: Diffs show intent, not character code changes
- **Composability**: Combine components in ways copy/paste can't match

Think of it like CSS for text: separate content from presentation, gain power through abstraction.

## {{mathbold}}UI Components{{/mathbold}}

mdfx provides high-level semantic components for common use cases. These compile down to shields.io badges,
frames, and character transformations.

### Visual Elements

**Color Swatches** - Single color blocks
```markdown
{{ui:swatch:accent/}}
{{ui:swatch:success/}}
```

**Status Indicators** - Colored badges
```markdown
{{ui:swatch:success/}}  → 🟢 Green block
{{ui:swatch:warning/}}  → 🟡 Yellow block
{{ui:swatch:error/}}    → 🔴 Red block
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
{{ui:swatch:success/}}
```

## {{mathbold}}Text Styles{{/mathbold}}

Transform text into 23 different Unicode character styles.

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
```

Available separators: `dot` (·), `bullet` (•), `dash` (─), `bolddash` (━), `arrow` (→)

**Spacing** - Add spaces between characters
```markdown
{{mathbold:spacing=1}}HELLO{{/mathbold}}  → 𝐇 𝐄 𝐋 𝐋 𝐎
{{mathbold:spacing=2}}WIDE{{/mathbold}}   → 𝐖  𝐈  𝐃  𝐄
```

## {{mathbold}}Installation{{/mathbold}}

mdfx is distributed as two packages: a library crate (`mdfx`) and a CLI tool (`mdfx-cli`).

### CLI Tool

Install the command-line tool:

```bash
cargo install mdfx-cli
```

### Library

Add to your Rust project:

```toml
[dependencies]
mdfx = "1.0"
```

### From Source

```bash
git clone https://github.com/blackwell-systems/mdfx
cd mdfx
cargo build --release --workspace
./target/release/mdfx --version
```

## {{mathbold}}Project Structure{{/mathbold}}

Cargo workspace with two crates:
- **`crates/mdfx`** - Core library (4 dependencies)
- **`crates/mdfx-cli`** - CLI tool (binary: `mdfx`)

## {{mathbold}}Usage{{/mathbold}}

### Library API

```rust
use mdfx::{Converter, TemplateParser};

let converter = Converter::new()?;
let result = converter.convert("HELLO", "mathbold")?;
// "𝐇𝐄𝐋𝐋𝐎"
```

### CLI - Process Markdown Files
```bash
# Process a template file
mdfx process README.template.md > README.md

# Process and save
mdfx process input.md --output output.md

# Process from stdin
echo "{{mathbold}}HELLO{{/mathbold}}" | mdfx process -
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
    let input = "# {{mathbold}}PROJECT{{/mathbold}}";
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

## {{mathbold}}Advanced Features{{/mathbold}}

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

### Low-Level Primitives (Escape Hatch)

For advanced users, direct shield rendering is available:
```markdown
{{shields:block:color=F41C80:style=flat-square/}}
{{shields:bar:colors=success,warning,error:style=flat-square/}}
```

UI components are recommended for most use cases.

## {{mathbold}}Examples{{/mathbold}}

### Project README Header
```markdown
# {{frame:gradient}}{{mathbold:separator=dot}}BLACKWELL SYSTEMS{{/mathbold}}{{/frame}}

## Built With
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:postgresql/}}
```

### Status Dashboard
```markdown
## System Status

{{ui:swatch:success/}} API Server: Operational
{{ui:swatch:success/}} Database: Healthy
{{ui:swatch:warning/}} Cache: Degraded
```

## {{mathbold}}How It Works{{/mathbold}}

mdfx uses a three-layer architecture:

1. **UI Components** (`{{ui:*}}`) - High-level semantic elements you author
2. **Primitives** (`{{shields:*}}`, `{{frame:*}}` / `{{fr:*}}`) - Rendering engines
3. **Styles** (`{{mathbold}}`) - Character transformations

When you write `{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}`, mdfx:
1. Parses the nested templates (frame → style)
2. Applies the frame decoration (gradient prefix/suffix)
3. Transforms characters with mathbold
4. Adds dot separators

This composition model keeps your markdown readable while enabling powerful visual effects.

## {{mathbold}}Configuration{{/mathbold}}

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
{{ui:swatch:accent/}}
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

## {{mathbold}}Template Syntax{{/mathbold}}

### Self-Closing Tags
For contentless elements:
```markdown
{{ui:tech:rust/}}
{{ui:swatch:accent/}}
{{ui:swatch:success/}}
```

### Block Tags
For elements with content:
```markdown
{{mathbold}}TEXT{{/mathbold}}
{{frame:gradient}}TITLE{{/frame}}
{{ui:row:align=center}}badges{{/ui}}
```

Note: UI components use generic `{{/ui}}` closer. Other templates use specific closers (`{{/mathbold}}`, `{{/frame}}`).

### Parameters
Colon-separated key=value pairs:
```markdown
{{mathbold:separator=dot:spacing=1}}TEXT{{/mathbold}}
{{ui:tech:rust/}}    ← Positional arg
{{ui:swatch:success/}}    ← Positional arg
```

## {{mathbold}}Project Status{{/mathbold}}

**Current Version:** v0.1.0 (Pre-release)

**Shipped:**
- 23 Unicode text styles with aliases
- 4 UI components (swatch, tech, row, progress)
- 27 inline frames
- 6 alphanumeric badge types
- Custom separators and spacing
- Design token system (palette.json)
- Template composition and nesting
- CLI and Rust library

**Planned:**
- Additional UI components (tables, diagrams)
- WASM bindings for browser/Node.js
- VS Code extension with preview
- Watch mode for live regeneration
- Component marketplace/gallery

## {{mathbold}}Contributing{{/mathbold}}

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## {{mathbold}}License{{/mathbold}}

MIT License - see [LICENSE](LICENSE) for details.

## {{mathbold}}Links{{/mathbold}}

- [Documentation](docs/)
- [Examples](examples/)
- [Architecture Design](docs/ARCHITECTURE.md)
- [API Guide](docs/API-GUIDE.md)
- [Components Design](docs/COMPONENTS.md)
