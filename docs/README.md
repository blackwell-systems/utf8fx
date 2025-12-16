# mdfx Documentation

A markdown compiler that transforms template syntax into styled output.

## Quick Start

```bash
cargo install mdfx-cli
```

Create `input.md`:
```markdown
# {{frame:gradient}}{{mathbold}}PROJECT{{/mathbold}}{{/frame}}
{{ui:tech:rust/}} {{ui:tech:python/}}
```

Process:
```bash
mdfx process input.md -o output.md
```

## Design Guides

| Guide | Description |
|-------|-------------|
| [Swatches](guides/SWATCH-GUIDE.md) | Color blocks, pixel art |
| [Components](guides/COMPONENTS-GUIDE.md) | swatch, tech, row, progress |
| [Frames](guides/FRAMES-GUIDE.md) | Decorative Unicode borders |
| [Text Styles](guides/TEXT-STYLES-GUIDE.md) | Unicode typography |
| [Glyphs](guides/GLYPHS-GUIDE.md) | Unicode glyphs and symbols |

## Reference

- [Template Syntax](TEMPLATE-SYNTAX.md) - Full syntax specification
- [API Guide](API-GUIDE.md) - Library usage

## CLI

```bash
mdfx convert --style mathbold "TEXT"   # Convert text
mdfx list                               # List styles
mdfx process input.md -o output.md     # Process file
mdfx process --backend svg input.md    # Local SVG files
```

## Library

```rust
use mdfx::{Converter, TemplateParser};

let converter = Converter::new()?;
let bold = converter.convert("HELLO", "mathbold")?;
// "𝐇𝐄𝐋𝐋𝐎"

let parser = TemplateParser::new()?;
let output = parser.process("{{mathbold}}TITLE{{/mathbold}}")?;
// "𝐓𝐈𝐓𝐋𝐄"
```

## Links

- [Examples](https://github.com/blackwell-systems/mdfx/tree/main/examples)
- [GitHub](https://github.com/blackwell-systems/mdfx)
- [Contributing](https://github.com/blackwell-systems/mdfx/blob/main/CONTRIBUTING.md)
