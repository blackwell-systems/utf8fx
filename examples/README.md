# mdfx Examples

Comprehensive examples showcasing mdfx capabilities, from high-level UI components to low-level primitives.

---

## Quick Start

### UI Components (Recommended)

The simplest way to use mdfx is through semantic UI components:

```markdown
# {{frame:gradient}}{{mathbold:separator=dot}}PROJECT NAME{{/mathbold}}{{/frame}}

## Tech Stack
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:postgresql/}}

## Status
{{ui:swatch:success/}} All systems operational
```

**Renders as:**

```markdown
# ▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓· ·𝐍·𝐀·𝐌·𝐄 ░▒▓

## Tech Stack
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=rust&logoColor=FFFFFF&label=&labelColor=292A2D) ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=python&logoColor=FFFFFF&label=&labelColor=292A2D) ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square&logo=postgresql&logoColor=FFFFFF&label=&labelColor=292A2D)

## Status
![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) All systems operational
```

---

## UI Components Reference

### Visual Elements

#### Color Swatches

Single colored blocks:

```markdown
{{ui:swatch:accent/}}
{{ui:swatch:success/}}
{{ui:swatch:slate/}}
```

**Arguments:**
- Palette color name (`accent`, `success`, `warning`, etc.)
- Or direct hex: `{{ui:swatch:ff6b35/}}`

**Use Cases:**
- Color palette documentation
- Status indicators
- Design system swatches

#### Status Indicators

Colored status badges:

```markdown
{{ui:swatch:success/}} Service healthy
{{ui:swatch:warning/}} Degraded performance
{{ui:swatch:error/}} Service down
{{ui:swatch:info/}} Maintenance scheduled
```

**Use Cases:**
- System health dashboards
- CI/CD status
- Service monitoring pages

### Tech Stack Badges

Technology logos using Simple Icons:

```markdown
{{ui:tech:rust/}}
{{ui:tech:python/}}
{{ui:tech:postgresql/}}
{{ui:tech:docker/}}
{{ui:tech:kubernetes/}}
{{ui:tech:react/}}
{{ui:tech:typescript/}}
{{ui:tech:nodejs/}}
```

**2000+ logos available** at [simpleicons.org](https://simpleicons.org/)

**Use Cases:**
- README tech stack sections
- Project dependencies
- Integration showcases

### Content Blocks

#### Row Layout

Horizontally align content with HTML wrapper (GitHub-compatible):

```markdown
{{ui:row:align=center}}
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:warning/}}
{{/ui}}
```

**Alignment options:** `left`, `center`, `right`

**Use Cases:**
- Center badges and swatches
- Horizontal layouts on GitHub
- Aligned icon rows

---

## Complete Examples

### Project README

```markdown
# {{frame:gradient}}{{mathbold:separator=dot}}BLACKWELL SYSTEMS{{/mathbold}}{{/frame}}

## Overview

Enterprise-grade system architecture platform.

## Built With

{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:postgresql/}} {{ui:tech:docker/}} {{ui:tech:kubernetes/}}

## System Status

{{ui:swatch:success/}} API Server: Operational
{{ui:swatch:success/}} Database: Healthy
{{ui:swatch:warning/}} Cache: Degraded
{{ui:swatch:success/}} Queue: Processing

## Quick Start

> **Note:** Requires Rust 1.70+ and Docker

1. Clone the repository
2. Run `make setup`
3. Start services: `docker-compose up`

> **Warning:** First-time setup may take 10-15 minutes
```

### Status Dashboard

```markdown
# System Health Dashboard

## Production Environment

{{ui:swatch:success/}} **API Gateway** - 99.9% uptime
{{ui:swatch:success/}} **Web Servers** - All 12 nodes healthy
{{ui:swatch:warning/}} **Cache Layer** - 1 node degraded
{{ui:swatch:success/}} **Database** - Primary + 2 replicas OK

## Staging Environment

{{ui:swatch:success/}} All services operational

## Development Environment

{{ui:swatch:success/}} All services operational

**Last Updated:** 2025-12-12 10:30 UTC
```

### Release Notes

```markdown
# Release v2.0.0

> **Warning:** This release contains breaking changes

## New Features

- Feature A
- Feature B
- Feature C

## Breaking Changes

> **Error:** The old API has been removed

1. Update imports from `v1` to `v2`
2. Migrate configuration files
3. Run database migrations

> **Info:** See migration guide for detailed instructions

## Tech Stack Updates

{{ui:tech:rust/}} Rust 1.75
{{ui:tech:postgresql/}} PostgreSQL 16
{{ui:tech:docker/}} Docker 24

{{ui:swatch:success/}} All tests passing
```

### Documentation Page

```markdown
# {{frame:gradient}}{{mathbold:separator=dot}}INSTALLATION GUIDE{{/mathbold}}{{/frame}}

## Prerequisites

You'll need:

- {{ui:tech:rust/}} Rust 1.70+
- {{ui:tech:docker/}} Docker 20+
- {{ui:tech:postgresql/}} PostgreSQL 14+

> **Note:** macOS users: Install via Homebrew

## Step 1: Install Dependencies

\`\`\`bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Docker
brew install --cask docker
\`\`\`

> **Info:** Windows users: Use WSL2 for best compatibility

## Step 2: Clone Repository

\`\`\`bash
git clone https://github.com/example/project
cd project
\`\`\`

## Step 3: Build

\`\`\`bash
cargo build --release
\`\`\`

{{ui:swatch:success/}} Build complete!
```

---

## Text Styles (Character Transformation)

Transform text into Unicode character styles:

```markdown
{{mathbold}}PROFESSIONAL{{/mathbold}}
→ 𝐏𝐑𝐎𝐅𝐄𝐒𝐒𝐈𝐎𝐍𝐀𝐋

{{script}}Elegant{{/script}}
→ ℰ𝓁ℯℊ𝒶𝓃𝓉

{{fullwidth}}WIDE{{/fullwidth}}
→ ＷＩＤＥ

{{fraktur}}Gothic{{/fraktur}}
→ 𝔊𝔬𝔱𝔥𝔦𝔠

{{monospace}}code{{/monospace}}
→ 𝚌𝚘𝚍𝚎
```

### With Separators

Add characters between letters:

```markdown
{{mathbold:separator=dot}}TITLE{{/mathbold}}
→ 𝐓·𝐈·𝐓·𝐋·𝐄

{{mathbold:separator=arrow}}FLOW{{/mathbold}}
→ 𝐅→𝐋→𝐎→𝐖

{{script:separator=bullet}}Note{{/script}}
→ 𝒩•𝑜•𝓉•ℯ
```

**Named Separators:**
- `dot` (·) - Middle dot for elegant spacing
- `bullet` (•) - Bullet point for emphasis
- `dash` (─) - Box drawing light horizontal line
- `bolddash` (━) - Box drawing heavy horizontal line
- `arrow` (→) - Rightward arrow for flow visualization
- `star` (★) - Black star for decorative separation
- `diamond` (◆) - Black diamond for geometric separation
- `square` (■) - Black square for bold separation
- `circle` (●) - Black circle for soft separation
- `pipe` (|) - Vertical bar for technical contexts
- `slash` (/) - Forward slash for path-like separation
- `tilde` (~) - Tilde for wave-like separation

**Direct Unicode Characters:**
Any single Unicode character works:
```markdown
{{mathbold:separator=⚡}}LIGHTNING{{/mathbold}}
{{mathbold:separator=★}}STARS{{/mathbold}}
{{mathbold:separator=|}}PIPES{{/mathbold}}
```

**Validation & Error Handling:**
- Whitespace is automatically trimmed: `separator=  dot  ` works
- Invalid separators show "did you mean" suggestions
- Template delimiters (`:`, `/`, `}`) are rejected
- Helpful error messages list all available options

Run `mdfx separators` to see all available named separators.

### With Spacing

Add spaces between characters:

```markdown
{{mathbold:spacing=1}}SPACED{{/mathbold}}
→ 𝐒 𝐏 𝐀 𝐂 𝐄 𝐃

{{mathbold:spacing=2}}WIDE{{/mathbold}}
→ 𝐖  𝐈  𝐃  𝐄
```

### Available Styles

**19 styles total:**

- `mathbold` - Mathematical Bold (𝐀𝐁𝐂)
- `fullwidth` - Full-Width (ＡＢＣ)
- `negative-squared` - Negative Squared (🅰🅱🅲)
- `negative-circled` - Negative Circled (🅐🅑🅒)
- `squared-latin` - Squared Latin (🄰🄱🄲)
- `circled-latin` - Circled Latin (Ⓐⓑⓒ)
- `small-caps` - Small Capitals (ᴀʙᴄ)
- `monospace` - Monospace (𝙰𝙱𝙲)
- `double-struck` - Double-Struck (𝔸𝔹ℂ)
- `sans-serif` - Sans-Serif (𝖠𝖡𝖢)
- `sans-serif-bold` - Sans-Serif Bold (𝗔𝗕𝗖)
- `sans-serif-italic` - Sans-Serif Italic (𝘈𝘉𝘊)
- `sans-serif-bold-italic` - Sans-Serif Bold Italic (𝘼𝘽𝘾)
- `italic` - Italic (𝐴𝐵𝐶)
- `bold-italic` - Bold Italic (𝑨𝑩𝑪)
- `script` - Script/Cursive (𝒜ℬ𝒞)
- `bold-script` - Bold Script (𝓐𝓑𝓒)
- `fraktur` - Fraktur/Gothic (𝔄𝔅ℭ)
- `bold-fraktur` - Bold Fraktur (𝕬𝕭𝕮)

---

## Advanced: Primitives

For power users who need direct control, primitives are available as an escape hatch.

### Inline Frames

Decorative prefix/suffix:

```markdown
{{frame:gradient}}TITLE{{/frame}}
→ ▓▒░ TITLE ░▒▓

{{frame:solid-left}}WARNING{{/frame}}
→ █▌ WARNING

{{frame:line-double}}HEADER{{/frame}}
→ ═══ HEADER ═══
```

**27 frame styles available.** See `mdfx frames list`.

### Shields (Direct)

Generate shields.io badges directly:

```markdown
{{shields:block:color=F41C80:style=flat-square/}}
→ ![](https://img.shields.io/badge/-F41C80?style=flat-square)

{{shields:bar:colors=success,warning,error:style=flat-square/}}
→ ![](https://img.shields.io/badge/-22C55E_EAB308_EF4444?style=flat-square)
```

**Note:** Most users should use `{{ui:*}}` components instead. Shields primitives are verbose but powerful.

### Composition

Combine primitives for custom effects:

```markdown
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
→ ▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓

{{frame:solid-left}}{{glyph:circle.1}} {{mathbold}}FIRST{{/mathbold}}{{/frame}}
→ █▌① 𝐅𝐈𝐑𝐒𝐓
```

---

## Design Tokens

Components use named colors from the palette:

| Token | Hex | Usage |
|-------|-----|-------|
| `accent` | F41C80 | Primary brand |
| `success` | 22C55E | Success states |
| `warning` | EAB308 | Warnings |
| `error` | EF4444 | Errors |
| `info` | 3B82F6 | Information |
| `slate` | 6B7280 | Neutral gray |
| `ui.bg` | 292A2D | Dark background |
| `ui.surface` | 292C34 | Elevated surface |
| `ui.panel` | 282F3C | Panel |

**Use in components:**
```markdown
{{ui:swatch:accent/}}
{{ui:swatch:success/}}
```

---

## Try the Examples

### Process a File

```bash
# Create example
echo "# {{mathbold}}TEST{{/mathbold}}" > example.md

# Process it
mdfx process example.md

# Output:
# 𝐓𝐄𝐒𝐓
```

### View Demo Files

```bash
# View original template
cat examples/demo-input.md

# Process and view result
mdfx process examples/demo-input.md

# Compare with saved output
diff <(mdfx process examples/demo-input.md) examples/demo-output.md
```

---

## Integration Examples

### GitHub Actions

```yaml
name: Process README
on: [push]
jobs:
  process:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install mdfx
        run: cargo install mdfx
      - name: Process README
        run: mdfx process README.template.md > README.md
      - name: Commit changes
        run: |
          git add README.md
          git commit -m "Update README"
          git push
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Process README before committing
mdfx process README.template.md > README.md
git add README.md
```

### Makefile

```makefile
.PHONY: docs build

docs:
	mdfx process README.template.md > README.md
	mdfx process docs/**/*.template.md

build: docs
	cargo build --release
```

### Hugo Blog

```bash
# Process all post templates
find content/posts -name "*.template.md" | while read file; do
  mdfx process "$file" > "${file%.template.md}.md"
done

hugo build
```

---

## Creative Showcases

### Gothic & Industrial Edition

See [opus-creativity-showcase.md](opus-creativity-showcase.md) for an advanced creative example featuring:

- **Pixel Art**: Stained glass windows, warding sigils, mechanical eyes
- **Gothic Typography**: Fraktur, script, and decorated text styles
- **Industrial UI**: Reactor status panels, hazard warnings, decay meters
- **Color Poetry**: Emotional gradients, abyssal palettes, forge heat
- **Nested Frames**: Multi-layer decorative frames for dramatic effect

**To regenerate:**

```bash
cd examples
mdfx process opus-creativity.template.md --backend svg --assets-dir assets -o opus-creativity-showcase.md
```

The SVG backend generates individual `.svg` files for swatches, enabling:
- Custom dimensions and shapes
- Borders and opacity
- Labels and icons
- Pixel-art compositions

---

## Documentation

- [Main README](../README.md)
- [API Guide](../docs/API-GUIDE.md)
- [Components Design](../docs/COMPONENTS.md)
- [Architecture](../docs/ARCHITECTURE.md)
