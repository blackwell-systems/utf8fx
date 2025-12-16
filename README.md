# ▓▒░ 𝐌·𝐃·𝐅·𝐗 ░▒▓

[![Blackwell Systems™](https://raw.githubusercontent.com/blackwell-systems/blackwell-docs-theme/main/badge-trademark.svg)](https://github.com/blackwell-systems)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![codecov](https://codecov.io/gh/blackwell-systems/mdfx/graph/badge.svg)](https://codecov.io/gh/blackwell-systems/mdfx)

**Add visual design to markdown without leaving markdown.**

Write `{{mathbold}}TITLE{{/mathbold}}` → get `𝐓𝐈𝐓𝐋𝐄`
Write `{{ui:tech:rust/}}` → get a shields.io badge or local SVG
Write `{{frame:gradient}}HEADER{{/frame}}` → get `▓▒░ HEADER ░▒▓`

mdfx is a compiler: template syntax in, styled markdown out.

## Quick Start

```bash
cargo install mdfx-cli
```

Create `README.template.md`:
```markdown
# {{frame:gradient}}{{mathbold:separator=dot}}PROJECT NAME{{/mathbold}}{{/frame}}

## Tech Stack
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:postgresql/}}

## Status
{{ui:swatch:success/}} All systems operational
```

Process it:
```bash
mdfx process README.template.md -o README.md
```

Output:

# ▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓· ·𝐍·𝐀·𝐌·𝐄 ░▒▓

## Tech Stack
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=rust&logoColor=FFFFFF&label=&labelColor=292A2D) ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=python&logoColor=FFFFFF&label=&labelColor=292A2D) ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=postgresql&logoColor=FFFFFF&label=&labelColor=292A2D)

## Status
![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) All systems operational

---

## Features

### UI Components
High-level semantic components that compile to shields.io badges or local SVGs.

| Component | Example | Output |
|-----------|---------|--------|
| `{{ui:tech:rust/}}` | Tech badge | ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=rust&logoColor=FFFFFF) |
| `{{ui:swatch:F41C80/}}` | Color block | ![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) |
| `{{ui:progress:75/}}` | Progress bar | Horizontal progress indicator |
| `{{ui:donut:80/}}` | Donut chart | Circular progress ring |
| `{{ui:gauge:65/}}` | Gauge meter | Semi-circular dashboard meter |
| `{{ui:sparkline:1,3,2,6,4,8/}}` | Sparkline | Mini inline chart for data |

**Progress bars with sliders:**
```markdown
{{ui:progress:50:thumb=14/}}                        # Basic slider
{{ui:progress:75:thumb=12:thumb_width=20/}}         # Oval/pill thumb
{{ui:progress:60:thumb=14:thumb_color=accent/}}     # Custom thumb color
```

**Donut and gauge with thumbs:**
```markdown
{{ui:donut:75:thumb=12:thumb_color=accent/}}        # Donut slider
{{ui:gauge:50:thumb=16:thumb_color=warning/}}       # Gauge slider
```

**Sparkline charts:**
```markdown
{{ui:sparkline:1,3,2,6,4,8,5,7/}}                   # Line chart (default)
{{ui:sparkline:1,3,2,6,4,8:type=bar/}}              # Bar chart
{{ui:sparkline:1,3,2,6,4,8:type=area/}}             # Area chart
{{ui:sparkline:1,3,2,6,4,8:dots=true/}}             # Line with dots
```

See [Components Guide](docs/guides/COMPONENTS-GUIDE.md) for full reference.

### Text Styles
Transform text into 23 Unicode character styles.

| Style | Example |
|-------|---------|
| `{{mathbold}}TEXT{{/mathbold}}` | 𝐓𝐄𝐗𝐓 |
| `{{fraktur}}TEXT{{/fraktur}}` | 𝔗𝔈𝔛𝔗 |
| `{{script}}TEXT{{/script}}` | 𝒯𝐸𝒳𝒯 |
| `{{double-struck}}TEXT{{/double-struck}}` | 𝕋𝔼𝕏𝕋 |
| `{{circled-latin}}text{{/circled-latin}}` | ⓣⓔⓧⓣ |

With modifiers:
```markdown
{{mathbold:separator=dot}}TITLE{{/mathbold}}  → 𝐓·𝐈·𝐓·𝐋·𝐄
{{mathbold:spacing=1}}HELLO{{/mathbold}}      → 𝐇 𝐄 𝐋 𝐋 𝐎
```

See [Text Styles Guide](docs/guides/TEXT-STYLES-GUIDE.md) for all 23 styles.

### Frames
Decorative Unicode borders around text.

```markdown
{{frame:gradient}}TITLE{{/frame}}     → ▓▒░ TITLE ░▒▓
{{frame:line-double}}TEXT{{/frame}}   → ═ TEXT ═
{{frame:arrows}}NEXT{{/frame}}        → » NEXT «
```

See [Frames Guide](docs/guides/FRAMES-GUIDE.md) for all 29 frame styles.

### Glyphs
504 named Unicode characters for separators, progress bars, and decorative elements.

| Category | Examples | Glyph Names |
|----------|----------|-------------|
| **Shades** | ░▒▓█ | `shade.light`, `shade.medium`, `shade.dark`, `block.full` |
| **Braille Bars** | ⡀⡄⡆⡇⣇⣧⣷⣿ | `braille.bar.1` through `braille.bar.8` |
| **Circled Numbers** | ①②③④⑤ | `circle.1` through `circle.20` |
| **Box Drawing** | ┌─┬─┐ │ ├─┼─┤ └─┴─┘ | `box.light.*`, `box.heavy.*`, `box.double.*` |
| **Shapes** | ■□●○▲△◆◇★☆ | `square.*`, `circle.*`, `tri.*`, `diamond.*`, `star.*` |
| **Checkmarks** | ☐☑☒✓✗ | `check.empty`, `check.yes`, `check.no`, `check.mark`, `check.x` |
| **Arrows** | ←↑→↓⇐⇑⇒⇓ | `arrow.*`, `arrow.double-*` |
| **Math** | ∑∏∫√∞≈≠≤≥ | `math.sum`, `math.product`, `math.integral`, etc. |

```markdown
{{glyph:star.filled/}} Rating: {{glyph:star.filled/}}{{glyph:star.filled/}}{{glyph:star.filled/}}{{glyph:star.empty/}}{{glyph:star.empty/}}
```
→ ★ Rating: ★★★☆☆

```markdown
Progress: {{glyph:braille.bar.8/}}{{glyph:braille.bar.8/}}{{glyph:braille.bar.8/}}{{glyph:braille.bar.4/}}{{glyph:braille.empty/}}
```
→ Progress: ⣿⣿⣿⡇⠀

See [Glyphs Guide](docs/guides/GLYPHS-GUIDE.md) for all 504 glyphs.

## Installation

### CLI
```bash
cargo install mdfx-cli
```

### Library
```toml
[dependencies]
mdfx = "1.0"
```

```rust
use mdfx::{Converter, TemplateParser};

// Direct conversion
let converter = Converter::new()?;
let bold = converter.convert("HELLO", "mathbold")?;  // 𝐇𝐄𝐋𝐋𝐎

// Template processing
let parser = TemplateParser::new()?;
let output = parser.process("{{mathbold}}TITLE{{/mathbold}}")?;  // 𝐓𝐈𝐓𝐋𝐄
```

### From Source
```bash
git clone https://github.com/blackwell-systems/mdfx
cd mdfx
cargo build --release --workspace
```

## CLI Usage

```bash
# Process template files
mdfx process input.md -o output.md
mdfx process README.template.md > README.md

# Process and save
mdfx process input.md --output output.md

# Process from stdin
echo "{{mathbold}}HELLO{{/mathbold}}" | mdfx process -
```

### CLI - Multi-Target Rendering
```bash
# Target-specific output (github, gitlab, npm, pypi, local)
mdfx process input.md --target github -o README.md      # shields.io badges
mdfx process input.md --target pypi -o PKG-INFO.md      # plain text fallbacks
mdfx process input.md --target local -o docs/index.md   # local SVG files

# Build for multiple targets at once
mdfx build input.md --all-targets                       # all 5 targets
mdfx build input.md --targets github,pypi,npm           # selected targets
```

See [CLI Guide](docs/guides/CLI-GUIDE.md) for complete multi-target documentation.

### CLI - Direct Conversion
```bash
# Convert text directly
mdfx convert --style mathbold "HELLO WORLD"
# Output: 𝐇𝐄𝐋𝐋𝐎 𝐖𝐎𝐑𝐋𝐃

# With separator
mdfx convert --style mathbold --separator dot "TITLE"

# List available styles
mdfx list
mdfx frames list
```

### CLI - Asset Management
```bash
# Verify asset integrity
mdfx verify --assets-dir assets/mdfx

# Clean unused assets (compare against manifest)
mdfx clean --assets-dir assets/mdfx --dry-run

# Clean unused assets (scan markdown files for references)
mdfx clean --scan "docs/**/*.md" --assets-dir docs/assets
```

## Rendering Backends

By default, mdfx generates shields.io URLs. For offline docs or reproducible builds, use the SVG backend:

```bash
# Shields.io (default) - URLs render on GitHub
mdfx process input.md -o output.md

# SVG backend - generates local files
mdfx process input.md -o output.md --backend svg --assets-dir assets/
```

See [Architecture](docs/ARCHITECTURE.md) for backend details.

## Documentation

| Guide | Description |
|-------|-------------|
| [Swatches](docs/guides/SWATCH-GUIDE.md) | Color blocks, pixel art |
| [Progress Bars](docs/guides/PROGRESS-GUIDE.md) | Progress indicators with sliders |
| [Donut & Gauge](docs/guides/DONUT-GAUGE-GUIDE.md) | Circular charts and meters |
| [Sparklines](docs/guides/SPARKLINE-GUIDE.md) | Mini inline data charts |
| [Components](docs/guides/COMPONENTS-GUIDE.md) | swatch, tech, row |
| [Frames](docs/guides/FRAMES-GUIDE.md) | 29 decorative Unicode borders |
| [Text Styles](docs/guides/TEXT-STYLES-GUIDE.md) | 23 Unicode typography styles |
| [Glyphs](docs/guides/GLYPHS-GUIDE.md) | Unicode glyphs and symbols |
| [Template Syntax](docs/TEMPLATE-SYNTAX.md) | Full syntax reference |
| [API Guide](docs/API-GUIDE.md) | Library usage |

## Links

- [Examples](examples/)
- [Architecture](docs/ARCHITECTURE.md)
- [Contributing](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)
- [License](LICENSE) (MIT)
