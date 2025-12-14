# Swatch Complete Guide

The swatch component is mdfx's most versatile visual primitive. This guide covers every parameter and configuration option.

---

## Basic Syntax

```markdown
{{ui:swatch:COLOR/}}
```

Where `COLOR` is either:
- A **palette name**: `accent`, `success`, `warning`, `error`, `info`, `slate`, `ui.bg`, `ui.surface`, `ui.panel`
- A **hex color**: `FF6B35`, `1a1a2e`, `22C55E` (no `#` prefix)

**Examples:**

```markdown
{{ui:swatch:accent/}}          <!-- Uses palette accent color (F41C80) -->
{{ui:swatch:success/}}         <!-- Uses palette success (22C55E) -->
{{ui:swatch:FF6B35/}}          <!-- Direct hex color -->
{{ui:swatch:1a1a2e/}}          <!-- Dark color -->
```

---

## All Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `color` | hex/palette | *required* | The fill color (first positional argument) |
| `width` | integer | 20 | Width in pixels |
| `height` | integer | 20 | Height in pixels |
| `style` | enum | flat-square | Corner style and effects |
| `opacity` | float | 1.0 | Fill opacity (0.0 to 1.0) |
| `border` | hex/palette | none | Border color |
| `border_width` | integer | 1 | Border thickness in pixels |
| `label` | string | none | Text label centered on swatch |
| `label_color` | hex/palette | white | Label text color |
| `icon` | string | none | Icon name (displays abbreviation) |
| `icon_color` | hex/palette | white | Icon text color |

---

## Dimensions

### Width & Height

Control swatch size for any purpose:

```markdown
<!-- Tiny indicator -->
{{ui:swatch:accent:width=8:height=8/}}

<!-- Standard badge -->
{{ui:swatch:accent:width=20:height=20/}}

<!-- Wide bar -->
{{ui:swatch:accent:width=200:height=10/}}

<!-- Tall column -->
{{ui:swatch:accent:width=10:height=100/}}

<!-- Large panel -->
{{ui:swatch:accent:width=300:height=80/}}
```

### Pixel Art

Create pixel art with small swatches:

```markdown
<!-- 8x8 pixels for detailed art -->
{{ui:swatch:FF0000:width=8:height=8/}}{{ui:swatch:00FF00:width=8:height=8/}}{{ui:swatch:0000FF:width=8:height=8/}}

<!-- 10x10 for visible pixels -->
{{ui:swatch:1a1a2e:width=10:height=10/}}{{ui:swatch:FFFFFF:width=10:height=10/}}
```

---

## Styles

The `style` parameter changes corner radius and effects:

| Style | Corners | Effect | Best For |
|-------|---------|--------|----------|
| `flat-square` | Sharp (rx=0) | None | Modern, technical |
| `flat` | Rounded (rx=3) | None | Friendly, approachable |
| `plastic` | Rounded (rx=3) | Gradient shine | Glossy, 3D look |
| `for-the-badge` | Rounded (rx=3) | Taller (28px) | Headers, emphasis |
| `social` | Very rounded (rx=10) | Pill shape | Buttons, tags |

**Examples:**

```markdown
{{ui:swatch:accent:style=flat-square/}}   <!-- Sharp corners -->
{{ui:swatch:accent:style=flat/}}          <!-- Slightly rounded -->
{{ui:swatch:accent:style=plastic/}}       <!-- Shiny 3D effect -->
{{ui:swatch:accent:style=for-the-badge/}} <!-- Taller badge -->
{{ui:swatch:accent:style=social/}}        <!-- Pill/capsule shape -->
```

---

## Opacity

Control transparency with `opacity` (0.0 to 1.0):

```markdown
{{ui:swatch:accent:opacity=1.0/}}   <!-- Fully opaque (default) -->
{{ui:swatch:accent:opacity=0.75/}}  <!-- 75% visible -->
{{ui:swatch:accent:opacity=0.5/}}   <!-- Half transparent -->
{{ui:swatch:accent:opacity=0.25/}}  <!-- 25% visible -->
{{ui:swatch:accent:opacity=0.1/}}   <!-- Nearly invisible -->
```

### Depth Illusion

Create layered depth effects:

```markdown
{{ui:swatch:F41C80:width=200:height=40:opacity=1.0/}}
{{ui:swatch:F41C80:width=180:height=40:opacity=0.75/}}
{{ui:swatch:F41C80:width=160:height=40:opacity=0.50/}}
{{ui:swatch:F41C80:width=140:height=40:opacity=0.25/}}
```

### Invisible Spacers

Use `opacity=0` for invisible spacing:

```markdown
{{ui:swatch:000000:width=50:height=20:opacity=0/}}  <!-- Invisible spacer -->
```

---

## Borders

Add borders with `border` (color) and `border_width`:

```markdown
<!-- Simple border -->
{{ui:swatch:1a1a2e:border=F41C80/}}

<!-- Thick border -->
{{ui:swatch:1a1a2e:border=F41C80:border_width=3/}}

<!-- Very thick frame -->
{{ui:swatch:1a1a2e:width=100:height=60:border=22C55E:border_width=5/}}
```

### Border Styles

```markdown
<!-- Subtle outline -->
{{ui:swatch:1a1a2e:width=80:height=40:border=333333:border_width=1/}}

<!-- Bold frame -->
{{ui:swatch:1a1a2e:width=80:height=40:border=FFFFFF:border_width=3/}}

<!-- Accent highlight -->
{{ui:swatch:1a1a2e:width=80:height=40:border=F41C80:border_width=2/}}
```

---

## Labels

Add text labels with `label` and optional `label_color`:

```markdown
<!-- Basic label -->
{{ui:swatch:accent:width=80:height=30:label=ACTIVE/}}

<!-- Custom label color -->
{{ui:swatch:1a1a2e:width=100:height=40:label=DARK MODE:label_color=FFFFFF/}}

<!-- Status labels -->
{{ui:swatch:22C55E:width=120:height=35:label=ONLINE/}}
{{ui:swatch:EF4444:width=120:height=35:label=OFFLINE/}}
{{ui:swatch:EAB308:width=120:height=35:label=PENDING/}}
```

### Complex Labels

```markdown
<!-- Percentage indicators -->
{{ui:swatch:22C55E:width=100:height=30:label=99.9%/}}
{{ui:swatch:3B82F6:width=100:height=30:label=Coverage 94%/}}

<!-- Temperature/metrics -->
{{ui:swatch:FF4500:width=150:height=40:label=2847°C/}}

<!-- Commands -->
{{ui:swatch:1a1a2e:width=200:height=30:border=00FF00:border_width=1:label=> ACCESS GRANTED/}}
```

---

## Icons

Add icon abbreviations with `icon` and optional `icon_color`:

```markdown
<!-- Technology icons -->
{{ui:swatch:000000:width=60:height=40:icon=rust:icon_color=DEA584/}}
{{ui:swatch:3178C6:width=60:height=40:icon=typescript:icon_color=FFFFFF/}}
{{ui:swatch:2496ED:width=60:height=40:icon=docker:icon_color=FFFFFF/}}

<!-- Platform icons -->
{{ui:swatch:000000:width=60:height=40:icon=github:icon_color=FFFFFF/}}
{{ui:swatch:FC6D26:width=60:height=40:icon=gitlab:icon_color=FFFFFF/}}
```

**Note:** Icons render as 3-letter abbreviations (e.g., "RUS" for rust). Full SVG icons require bundling icon libraries.

---

## Color Palette Reference

Built-in palette colors:

| Name | Hex | Usage |
|------|-----|-------|
| `accent` | F41C80 | Primary brand/accent |
| `success` | 22C55E | Success states |
| `warning` | EAB308 | Warnings |
| `error` | EF4444 | Errors |
| `info` | 3B82F6 | Information |
| `slate` | 6B7280 | Neutral gray |
| `ui.bg` | 292A2D | Dark background |
| `ui.surface` | 292C34 | Elevated surface |
| `ui.panel` | 282F3C | Panel background |
| `white` | FFFFFF | White |
| `black` | 000000 | Black |

**Usage:**

```markdown
{{ui:swatch:accent/}}     <!-- Resolves to F41C80 -->
{{ui:swatch:ui.bg/}}      <!-- Resolves to 292A2D -->
{{ui:swatch:success/}}    <!-- Resolves to 22C55E -->
```

---

## Complete Examples

### Status Dashboard

```markdown
{{ui:swatch:22C55E:width=15:height=15:style=social/}} API Server: Online
{{ui:swatch:22C55E:width=15:height=15:style=social/}} Database: Healthy
{{ui:swatch:EAB308:width=15:height=15:style=social/}} Cache: Degraded
{{ui:swatch:EF4444:width=15:height=15:style=social/}} Queue: Critical
```

### Progress Bar

```markdown
{{ui:swatch:22C55E:width=200:height=20:label=Progress 75%/}}{{ui:swatch:333333:width=67:height=20/}}
```

### Color Palette Documentation

```markdown
{{ui:row:align=center}}
{{ui:swatch:1a1a2e:width=60:height=80:label=Primary/}}
{{ui:swatch:2d2d44:width=60:height=80:label=Secondary/}}
{{ui:swatch:4a4a6a:width=60:height=80:label=Tertiary/}}
{{ui:swatch:6b6b8d:width=60:height=80:label=Muted/}}
{{/ui}}
```

### Heat Map Row

```markdown
{{ui:swatch:0a0a0a:width=30:height=30/}}{{ui:swatch:1a0a0a:width=30:height=30/}}{{ui:swatch:3a1010:width=30:height=30/}}{{ui:swatch:5a1a1a:width=30:height=30/}}{{ui:swatch:8B2500:width=30:height=30/}}{{ui:swatch:CD3700:width=30:height=30/}}{{ui:swatch:FF4500:width=30:height=30/}}{{ui:swatch:FF6347:width=30:height=30/}}{{ui:swatch:FFD700:width=30:height=30/}}
```

### Tech Stack Monument

```markdown
{{ui:row:align=center}}
{{ui:swatch:DEA584:width=60:height=40:icon=rust:icon_color=000000:border=DEA584:border_width=2/}}
{{ui:swatch:F7DF1E:width=60:height=40:icon=javascript:icon_color=000000:border=F7DF1E:border_width=2/}}
{{ui:swatch:3178C6:width=60:height=40:icon=typescript:icon_color=FFFFFF:border=3178C6:border_width=2/}}
{{/ui}}
```

### Hazard Warnings

```markdown
{{ui:row:align=center}}
{{ui:swatch:FFFF00:width=80:height=50:border=000000:border_width=3:label=☢ RADIATION/}}
{{ui:swatch:FF6600:width=80:height=50:border=000000:border_width=3:label=☣ BIOHAZARD/}}
{{ui:swatch:FF0000:width=80:height=50:border=000000:border_width=3:label=⚠ DANGER/}}
{{/ui}}
```

### Glassmorphism Effect

```markdown
{{ui:swatch:FFFFFF:width=200:height=60:opacity=0.15:border=FFFFFF:border_width=1/}}
{{ui:swatch:FFFFFF:width=180:height=50:opacity=0.2:border=FFFFFF:border_width=1:label=Glass Card/}}
```

### Pixel Art Eye (10x10)

```markdown
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}
{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:DC143C:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:1a1a1a:width=8:height=8/}}{{ui:swatch:C0C0C0:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:0a0a0a:width=8:height=8/}}
```

---

## Backend Differences

### Shields Backend (default)

Uses shields.io badges. Limited to:
- Color
- Style
- Basic dimensions through badge sizing

```bash
mdfx process template.md
```

### SVG Backend

Full control over all parameters:

```bash
mdfx process template.md --backend svg --assets-dir assets
```

Generates local `.svg` files with:
- Custom width/height
- Opacity
- Borders
- Labels
- Icons

---

## Tips & Tricks

### 1. Combine with Row for Centering

```markdown
{{ui:row:align=center}}
{{ui:swatch:accent:width=100:height=50:label=CENTERED/}}
{{/ui}}
```

### 2. Use Opacity for Layering

Stack swatches with decreasing opacity for depth:

```markdown
{{ui:swatch:accent:width=100:height=30:opacity=1.0/}}
{{ui:swatch:accent:width=90:height=30:opacity=0.7/}}
{{ui:swatch:accent:width=80:height=30:opacity=0.4/}}
```

### 3. Invisible Spacers for Layout

```markdown
{{ui:swatch:000:width=50:height=1:opacity=0/}}  <!-- Horizontal space -->
```

### 4. Border as Highlight

Use bright borders on dark swatches:

```markdown
{{ui:swatch:1a1a2e:width=200:height=60:border=F41C80:border_width=3:label=HIGHLIGHTED/}}
```

### 5. Combine Multiple Parameters

```markdown
{{ui:swatch:1a1a2e:width=300:height=80:style=social:opacity=0.9:border=00FF00:border_width=2:label=COMPLETE EXAMPLE/}}
```

---

## See Also

- [Components Reference](COMPONENTS.md)
- [API Guide](API-GUIDE.md)
- [Examples README](../examples/README.md)
- [Creative Showcase](../examples/opus-creativity-showcase.md)
