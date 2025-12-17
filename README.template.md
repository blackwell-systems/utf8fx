# {{frame:gradient}}{{mathbold:separator=dot}}MDFX{{/mathbold}}{{/frame}}

{{ui:row}}
{{ui:tech:rust/}} {{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:info/}} {{ui:swatch:warning/}}
{{/ui}}

**Design for markdown.** Transform templates into styled output with Unicode typography, SVG components, and decorative frames.

**Zero runtime dependencies.** Generate self-contained SVG assets that render anywhere—no shields.io, no CDNs, no external requests. Your documentation works offline, forever.

---

## {{frame:lenticular}}At a Glance{{/frame}}

{{ui:row}}
{{ui:progress:85:width=100:height=10:fill=00FF94:track=1a1a2e/}} {{ui:donut:72:size=28:thickness=3:fill=00D4FF:track=1a1a2e/}} {{ui:gauge:58:size=44:thickness=5:fill=FF6B35:track=1a1a2e/}} {{ui:sparkline:2,5,3,8,4,9,6,7:width=80:height=16:fill=F41C80:track=1a1a2e/}}
{{/ui}}

Write this:
```markdown
{{mathbold}}CHAPTER ONE{{/mathbold}}
{{ui:progress:75/}}
{{frame:star}}FEATURED{{/frame}}
```

Get this:

> {{mathbold}}CHAPTER ONE{{/mathbold}}
>
> {{ui:progress:75:width=80:height=8/}}
>
> {{frame:star}}FEATURED{{/frame}}

---

## {{mathbold}}Install{{/mathbold}}

```bash
cargo install mdfx-cli
```

---

## {{frame:diamond}}Typography{{/frame}}

24 Unicode text styles. No fonts required.

| | |
|---|---|
| {{mathbold}}MATHBOLD{{/mathbold}} | {{fraktur}}FRAKTUR{{/fraktur}} |
| {{script}}SCRIPT{{/script}} | {{double-struck}}DOUBLESTRUCK{{/double-struck}} |
| {{circled-latin}}circled{{/circled-latin}} | {{monospace}}MONOSPACE{{/monospace}} |
| {{small-caps}}smallcaps{{/small-caps}} | {{negative-squared}}NEG{{/negative-squared}} |

**Separators and spacing:**

{{mathbold:separator=dot}}DOTTED{{/mathbold}} · {{script:separator=arrow}}ARROWS{{/script}} · {{small-caps:spacing=1}}S P A C E D{{/small-caps}}

---

## {{frame:diamond}}Frames{{/frame}}

29 decorative borders. Pure Unicode.

| | |
|---|---|
| {{frame:gradient}}GRADIENT{{/frame}} | {{frame:lenticular}}LENTICULAR{{/frame}} |
| {{frame:star}}STARRED{{/frame}} | {{frame:guillemet}}QUOTED{{/frame}} |
| {{frame:triangle-right}}TRIANGLES{{/frame}} | {{frame:line-double}}DOUBLE{{/frame}} |
| {{frame:finger}}POINTED{{/frame}} | {{frame:angle}}ANGLED{{/frame}} |

---

## {{frame:diamond}}Glyphs{{/frame}}

500+ named Unicode symbols. Use inline or as separators.

| Category | Examples |
|----------|----------|
| Stars | {{glyph:star.filled/}} {{glyph:star.empty/}} {{glyph:star.4/}} {{glyph:star.sparkle/}} |
| Arrows | {{glyph:arrow.right/}} {{glyph:arrow.left/}} {{glyph:arrow.up/}} {{glyph:arrow.down/}} |
| Shapes | {{glyph:square.filled/}} {{glyph:circle.filled/}} {{glyph:circle.empty/}} {{glyph:square.empty/}} |
| Cards | {{glyph:card.spade/}} {{glyph:card.heart/}} {{glyph:card.diamond/}} {{glyph:card.club/}} |
| Chess | {{glyph:chess.king.white/}} {{glyph:chess.queen.white/}} {{glyph:chess.rook.white/}} {{glyph:chess.knight.white/}} |
| Music | {{glyph:music.eighth/}} {{glyph:music.beamed/}} {{glyph:music.flat/}} {{glyph:music.sharp/}} |

**As separators:**

| Syntax | Output |
|--------|--------|
| `{{mathbold:separator=star}}STARS{{/mathbold}}` | {{mathbold:separator=star}}STARS{{/mathbold}} |
| `{{mathbold:separator=diamond}}DIAMONDS{{/mathbold}}` | {{mathbold:separator=diamond}}DIAMONDS{{/mathbold}} |
| `{{mathbold:separator=card.heart}}LOVE{{/mathbold}}` | {{mathbold:separator=card.heart}}LOVE{{/mathbold}} |

---

## {{frame:diamond}}Components{{/frame}}

### Progress

{{ui:progress:20:width=200:height=10:fill=error/}}
{{ui:progress:50:width=200:height=10:fill=warning/}}
{{ui:progress:80:width=200:height=10:fill=success/}}

### Sliders

{{ui:progress:40:width=200:height=6:thumb=12:thumb_color=accent/}}
{{ui:progress:70:width=200:height=6:thumb=10:thumb_width=18:fill=info/}}

### Circular

{{ui:row}}
{{ui:donut:25:size=40:thickness=4:fill=error/}} {{ui:donut:50:size=40:thickness=4:fill=warning:thumb=6:thumb_color=white/}} {{ui:donut:75:size=40:thickness=4:fill=info:thumb=6:thumb_color=white/}} {{ui:donut:100:size=40:thickness=4:fill=success/}}
{{/ui}}

### Gauges

{{ui:row}}
{{ui:gauge:25:size=60:thickness=6:fill=error/}} {{ui:gauge:55:size=60:thickness=6:fill=warning:thumb=8:thumb_color=white/}} {{ui:gauge:85:size=60:thickness=6:fill=success/}}
{{/ui}}

### Sparklines

| Type | Chart |
|------|-------|
| Line | {{ui:sparkline:3,7,2,9,5,8,4,6:width=100:height=16:fill=accent/}} |
| Bar | {{ui:sparkline:3,7,2,9,5,8,4,6:type=bar:width=100:height=16:fill=info/}} |
| Area | {{ui:sparkline:3,7,2,9,5,8,4,6:type=area:width=100:height=16:fill=plum/}} |

### Waveforms

{{ui:waveform:0.2,0.5,0.8,0.6,0.3,-0.2,-0.6,-0.9,-0.5,-0.2,0.1,0.4,0.7,0.5,0.2,-0.1,-0.4,-0.7,-0.4,-0.1,0.2,0.5,0.8,0.5:width=200:height=32:positive=accent:negative=accent:bar=2:spacing=1/}}
{{ui:waveform:0.3,0.6,0.9,0.5,0.2,-0.3,-0.7,-0.8,-0.4,-0.1,0.2,0.5,0.8,0.4,0.1,-0.2,-0.5,-0.9,-0.6,-0.2,0.1,0.4,0.7,0.3:width=200:height=32:positive=success:negative=error:bar=2:spacing=1/}}

### Swatches

{{ui:row}}
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:warning/}} {{ui:swatch:error/}} {{ui:swatch:info/}} {{ui:swatch:cobalt/}} {{ui:swatch:plum/}} {{ui:swatch:slate/}}
{{/ui}}

### Tech

{{ui:row}}
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:typescript/}} {{ui:tech:go/}} {{ui:tech:docker/}} {{ui:tech:postgresql/}}
{{/ui}}

---

## {{mathbold}}Usage{{/mathbold}}

```bash
# Process a template
mdfx process README.template.md -o README.md

# SVG backend for local docs
mdfx process input.md -o output.md --backend svg --assets-dir assets/

# Multi-target build
mdfx build input.md --all-targets
```

**Targets:**
- `github` / `gitlab` → shields.io badges
- `local` → SVG files
- `pypi` → Unicode plaintext

---

## {{mathbold}}Documentation{{/mathbold}}

| | |
|---|---|
| [Components](docs/guides/COMPONENTS-GUIDE.md) | UI primitives |
| [Progress](docs/guides/PROGRESS-GUIDE.md) | Progress bars |
| [Donut & Gauge](docs/guides/DONUT-GAUGE-GUIDE.md) | Circular charts |
| [Sparklines](docs/guides/SPARKLINE-GUIDE.md) | Inline charts |
| [Waveforms](docs/guides/WAVEFORM-GUIDE.md) | Audio viz |
| [Text Styles](docs/guides/TEXT-STYLES-GUIDE.md) | 24 styles |
| [Frames](docs/guides/FRAMES-GUIDE.md) | 29 borders |
| [Glyphs](docs/guides/GLYPHS-GUIDE.md) | 500+ symbols |
| [CLI](docs/guides/CLI-GUIDE.md) | Commands |

---

{{frame:gradient-light}}{{small-caps}}MIT License{{/small-caps}}{{/frame}} · [GitHub](https://github.com/blackwell-systems/mdfx) · [Changelog](CHANGELOG.md)
