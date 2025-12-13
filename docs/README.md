# mdfx Documentation

[![Blackwell Systems™](https://raw.githubusercontent.com/blackwell-systems/blackwell-docs-theme/main/badge-trademark.svg)](https://github.com/blackwell-systems)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-189_passing-22c55e?style=flat-square)](https://github.com/blackwell-systems/mdfx/actions)
[![Crates.io](https://img.shields.io/crates/v/mdfx.svg)](https://crates.io/crates/mdfx)

Welcome to the mdfx documentation site! This is your navigation hub for all documentation.

**Transform markdown with Unicode text effects and UI components through template syntax.**

## 📚 Getting Started

**New to mdfx?** Start here:
- [Quick Start](#quick-start) - Installation and first examples
- [What is Unicode?](UNICODE-EXPLAINED.md) - Understanding Unicode vs fonts
- [Examples](https://github.com/blackwell-systems/mdfx/tree/main/examples) - Visual showcase and sample files

## 📖 User Documentation

### Core Features
- [19 Unicode Styles](#19-unicode-styles) - Text transformation styles (mathbold, script, fraktur, etc.)
- [Template System](#template-system) - Embedding effects in markdown
- [UI Components](COMPONENTS.md) - Dividers, badges, tech stacks, status indicators
- [Frame System](FRAMES-DESIGN.md) - Visual frames and borders

### Rendering
- [Multi-Backend System](#multi-backend-rendering) - Shields.io vs local SVG
- [CLI Commands](#cli-commands) - Complete command reference

## 🔧 Technical Documentation

- [Architecture](ARCHITECTURE.md) - System design, workspace structure, and multi-backend architecture
- [API Guide](API-GUIDE.md) - Complete library API reference with examples
- [Parser Design](parser-design.md) - Template parsing implementation
- [State Machine Guide](STATE-MACHINE-GUIDE.md) - Parser state machine details

## 🎨 Design Resources

- [Unicode Design Elements](unicode-design-elements.md) - Complete Unicode character reference
- [Planning Document](PLANNING.md) - Design decisions and roadmap

---

**Looking for something specific?**
- Installation → [Quick Start](#quick-start)
- Using in Rust code → [API Guide](API-GUIDE.md#getting-started)
- Component reference → [UI Components](COMPONENTS.md)
- Understanding the architecture → [Architecture](ARCHITECTURE.md#workspace-structure)
- CLI reference → [CLI Commands](#cli-commands)

---

## Quick Start

mdfx is distributed as two packages: a library (`mdfx`) and a CLI tool (`mdfx-cli`).

### CLI Tool

Install the command-line tool:

```bash
cargo install mdfx-cli
```

Create a markdown file with template syntax:

```markdown
# {{ui:header}}PROJECT NAME{{/ui}}

{{ui:divider/}}

## Tech Stack
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:postgresql/}}

## Status
{{ui:status:success/}} All systems operational
```

Process it:

```bash
mdfx process input.md -o output.md
```

The result is rendered markdown with Unicode styling and visual components.

### Library Usage

Add mdfx to your Rust project:

```toml
[dependencies]
mdfx = "1.0"
```

Use it programmatically:

```rust
use mdfx::{Converter, TemplateParser};

// Direct text conversion
let converter = Converter::new()?;
let result = converter.convert("HELLO", "mathbold")?;
// "𝐇𝐄𝐋𝐋𝐎"

// Template processing
let parser = TemplateParser::new()?;
let output = parser.process("{{mathbold}}TITLE{{/mathbold}}")?;
// "𝐓𝐈𝐓𝐋𝐄"
```

## Project Structure

mdfx uses a Cargo workspace with two crates:

- **`crates/mdfx`** - Core library (4 dependencies: serde, serde_json, thiserror, lazy_static)
- **`crates/mdfx-cli`** - CLI tool (depends on mdfx + clap, colored)

This separation ensures library users don't need CLI dependencies. The binary is still named `mdfx` for a seamless user experience.

## Core Features

### 19 Unicode Styles

Transform text into distinctive Unicode styles:

- **Bold Variants**: mathbold, sans-serif-bold
- **Script & Cursive**: script, mathbold-italic
- **Technical**: monospace, fullwidth, fraktur
- **Playful**: circled, negative-circled, squared, negative-squared
- **Elegant**: small-caps, superscript, subscript

Use directly via CLI:

```bash
mdfx convert --style mathbold "BOLD TEXT"
# Output: 𝐁𝐎𝐋𝐃 𝐓𝐄𝐗𝐓

mdfx convert --style script "Elegant"
# Output: ℰ𝓁ℯℊ𝒶𝓃𝓉
```

Or in templates:

```markdown
{{mathbold}}BOLD TEXT{{/mathbold}}
{{script}}Elegant{{/script}}
```

### Template System

Embed styling directly in markdown:

**Basic Templates**
```markdown
{{mathbold}}Bold text{{/mathbold}}
{{script}}Cursive text{{/script}}
{{fullwidth}}Ｆｕｌｌｗｉｄｔｈ{{/fullwidth}}
```

**With Separators**
```markdown
{{mathbold:separator=dot}}SPACED OUT{{/mathbold}}
{{mathbold:separator=arrow}}A→R→R→O→W{{/mathbold}}
```

**With Custom Spacing**
```markdown
{{mathbold:spacing=2}}W I D E{{/mathbold}}
```

### UI Components

High-level semantic components for common visual needs:

**Dividers** - Section separators
```markdown
{{ui:divider/}}
```

**Status Indicators**
```markdown
{{ui:status:success/}}  All systems operational
{{ui:status:warning/}}  Maintenance required
{{ui:status:error/}}    Critical failure
```

**Tech Stack Badges**
```markdown
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:docker/}}
```

**Color Swatches**
```markdown
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:error/}}
```

### Frame System

Wrap content in visual frames:

```markdown
{{frame:gradient}}
### Important Section
This content is visually emphasized
{{/frame}}

{{frame:solid-left}}
Critical information with left border
{{/frame}}
```

Available frame styles: `gradient`, `solid-left`, `solid-right`, `dashed`, `line-bold`

### Multi-Backend Rendering

Choose between shields.io URLs (default) or local SVG files:

**Shields.io Backend** (online badges)
```bash
mdfx process input.md  # Default
```

**SVG Backend** (local files)
```bash
mdfx process input.md --backend svg --assets-dir assets/mdfx
```

The SVG backend generates deterministic filenames (hash-based) for reproducible builds and git-friendly assets.

## Why mdfx?

**Why not just copy/paste Unicode characters?**

- **Repeatability**: Reuse `{{ui:header}}TITLE{{/ui}}` across dozens of files
- **Consistency**: Change style once, regenerate all docs
- **Maintainability**: Source files remain readable ASCII
- **Search & Replace**: Find/replace works on template names
- **Version Control**: Diffs show intent, not character code changes
- **Composability**: Combine components programmatically

Think of it like CSS for text: separate content from presentation, gain power through abstraction.

## CLI Commands

### Convert Text

```bash
mdfx convert --style mathbold "BOLD"
mdfx convert --style script "Elegant"
mdfx convert --style fullwidth "Wide"
mdfx convert --style circled "123"
```

### List Available Styles

```bash
mdfx list
mdfx list --samples
mdfx list --category bold
```

### Process Markdown Files

```bash
# Stdout
mdfx process input.md

# Output file
mdfx process input.md -o output.md

# In-place editing
mdfx process -i input.md

# With SVG backend
mdfx process input.md --backend svg -o output.md
```

### Generate Shell Completions

```bash
mdfx completions bash > /etc/bash_completion.d/mdfx
mdfx completions zsh > ~/.zsh/completions/_mdfx
mdfx completions fish > ~/.config/fish/completions/mdfx.fish
```

## Architecture

mdfx uses a multi-stage processing pipeline:

```
Markdown Input
    ↓
Template Parser (process_with_assets)
    ↓
Component Resolution (native vs expand)
    ↓
Backend Rendering (Shields / SVG)
    ↓
Styled Markdown Output + Asset Files
```

### Multi-Backend System

The rendering layer is pluggable:

- **ShieldsBackend**: Generates shields.io URLs (no local files)
- **SvgBackend**: Generates local SVG files with hash-based names
- **Future**: HTML canvas, PNG rasterization, custom backends

See [Architecture Guide](/ARCHITECTURE.md) for detailed design documentation.

## Examples

Check out the [examples directory](https://github.com/blackwell-systems/mdfx/tree/main/examples) for comprehensive demonstrations:

- **visual-showcase.md** - Extensive showcase of all features
- **simple-test.md** - Quick reference for common patterns

## Contributing

Contributions welcome! See [CONTRIBUTING.md](https://github.com/blackwell-systems/mdfx/blob/main/CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](https://github.com/blackwell-systems/mdfx/blob/main/LICENSE) for details.

---

**Made by [Blackwell Systems™](https://github.com/blackwell-systems)**
