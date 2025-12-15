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

**Syntax:**

```markdown
{{ui:swatch:accent/}}          <!-- Uses palette accent color (F41C80) -->
{{ui:swatch:success/}}         <!-- Uses palette success (22C55E) -->
{{ui:swatch:FF6B35/}}          <!-- Direct hex color -->
{{ui:swatch:1a1a2e/}}          <!-- Dark color -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) ![](https://img.shields.io/badge/-%20-FF6B35?style=flat-square) ![](https://img.shields.io/badge/-%20-1A1A2E?style=flat-square)

---

## All Parameters

| Parameter | Type | Default | Backend | Description |
|-----------|------|---------|---------|-------------|
| `color` | hex/palette | *required* | Both | The fill color (first positional argument) |
| `style` | enum | flat-square | Both | Corner style and effects |
| `label` | string | none | Both | Text label centered on swatch |
| `label_color` | hex/palette | white | Both | Label text color |
| `icon` | string | none | Both | Simple Icons logo name |
| `icon_color` | hex/palette | white | Both | Icon color |
| `width` | integer | 20 | **SVG only** | Width in pixels |
| `height` | integer | 20 | **SVG only** | Height in pixels |
| `opacity` | float | 1.0 | **SVG only** | Fill opacity (0.0 to 1.0) |
| `border` | hex/palette | none | **SVG only** | Border color |
| `border_width` | integer | 1 | **SVG only** | Border thickness in pixels |

> **Note:** Parameters marked "SVG only" require `--backend svg`. The default GitHub target uses shields.io which ignores these parameters.

---

## Dimensions (SVG Backend Only)

> ⚠️ **SVG backend required.** Use `mdfx process --backend svg` to enable custom dimensions.

### Width & Height

Control swatch size for any purpose:

```markdown
{{ui:swatch:accent:width=8:height=8/}}      <!-- Tiny indicator -->
{{ui:swatch:accent:width=20:height=20/}}    <!-- Standard badge -->
{{ui:swatch:accent:width=200:height=10/}}   <!-- Wide bar -->
{{ui:swatch:accent:width=10:height=100/}}   <!-- Tall column -->
{{ui:swatch:accent:width=300:height=80/}}   <!-- Large panel -->
```

### Pixel Art

Create pixel art with small swatches:

```markdown
{{ui:swatch:FF0000:width=8:height=8/}}{{ui:swatch:00FF00:width=8:height=8/}}{{ui:swatch:0000FF:width=8:height=8/}}
```

*See SVG examples in the repository for rendered output.*

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

**Syntax:**

```markdown
{{ui:swatch:accent:style=flat-square/}}   <!-- Sharp corners -->
{{ui:swatch:accent:style=flat/}}          <!-- Slightly rounded -->
{{ui:swatch:accent:style=plastic/}}       <!-- Shiny 3D effect -->
{{ui:swatch:accent:style=for-the-badge/}} <!-- Taller badge -->
{{ui:swatch:accent:style=social/}}        <!-- Pill/capsule shape -->
```

**Rendered:**

![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) flat-square
![](https://img.shields.io/badge/-%20-F41C80?style=flat) flat
![](https://img.shields.io/badge/-%20-F41C80?style=plastic) plastic
![](https://img.shields.io/badge/-%20-F41C80?style=for-the-badge) for-the-badge
![](https://img.shields.io/badge/-%20-F41C80?style=social) social

---

## Opacity (SVG Backend Only)

> ⚠️ **SVG backend required.** Use `mdfx process --backend svg` to enable opacity.

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

## Borders (SVG Backend Only)

> ⚠️ **SVG backend required.** Use `mdfx process --backend svg` to enable borders.

Add borders with `border` (color) and `border_width`:

```markdown
{{ui:swatch:1a1a2e:border=F41C80/}}                                         <!-- Simple border -->
{{ui:swatch:1a1a2e:border=F41C80:border_width=3/}}                          <!-- Thick border -->
{{ui:swatch:1a1a2e:width=100:height=60:border=22C55E:border_width=5/}}      <!-- Very thick frame -->
```

### Border Styles

```markdown
{{ui:swatch:1a1a2e:width=80:height=40:border=333333:border_width=1/}}  <!-- Subtle outline -->
{{ui:swatch:1a1a2e:width=80:height=40:border=FFFFFF:border_width=3/}}  <!-- Bold frame -->
{{ui:swatch:1a1a2e:width=80:height=40:border=F41C80:border_width=2/}}  <!-- Accent highlight -->
```

*See SVG examples in the repository for rendered output.*

---

## Labels

Add text labels with `label` and optional `label_color`. **Labels work with both backends.**

**Syntax:**

```markdown
{{ui:swatch:accent:label=ACTIVE/}}
{{ui:swatch:1a1a2e:label=DARK MODE/}}
{{ui:swatch:22C55E:label=ONLINE/}}
{{ui:swatch:EF4444:label=OFFLINE/}}
{{ui:swatch:EAB308:label=PENDING/}}
```

**Rendered:**

![](https://img.shields.io/badge/-ACTIVE-F41C80?style=flat-square)
![](https://img.shields.io/badge/-DARK%20MODE-1A1A2E?style=flat-square)
![](https://img.shields.io/badge/-ONLINE-22C55E?style=flat-square)
![](https://img.shields.io/badge/-OFFLINE-EF4444?style=flat-square)
![](https://img.shields.io/badge/-PENDING-EAB308?style=flat-square)

### Status Labels

**Syntax:**

```markdown
{{ui:swatch:22C55E:label=99.9%/}}
{{ui:swatch:3B82F6:label=Coverage 94%/}}
{{ui:swatch:1a1a2e:label=ACCESS GRANTED/}}
```

**Rendered:**

![](https://img.shields.io/badge/-99.9%-22C55E?style=flat-square)
![](https://img.shields.io/badge/-Coverage%2094%-3B82F6?style=flat-square)
![](https://img.shields.io/badge/-ACCESS%20GRANTED-1A1A2E?style=flat-square)

---

## Icons

Add Simple Icons logos with `icon` and optional `icon_color`. **Icons work with both backends.**

**Syntax:**

```markdown
{{ui:swatch:000000:icon=rust:icon_color=DEA584/}}
{{ui:swatch:3178C6:icon=typescript:icon_color=FFFFFF/}}
{{ui:swatch:2496ED:icon=docker:icon_color=FFFFFF/}}
{{ui:swatch:000000:icon=github:icon_color=FFFFFF/}}
{{ui:swatch:FC6D26:icon=gitlab:icon_color=FFFFFF/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-000000?style=flat-square&logo=rust&logoColor=DEA584&label=&labelColor=000000)
![](https://img.shields.io/badge/-%20-3178C6?style=flat-square&logo=typescript&logoColor=FFFFFF&label=&labelColor=3178C6)
![](https://img.shields.io/badge/-%20-2496ED?style=flat-square&logo=docker&logoColor=FFFFFF&label=&labelColor=2496ED)
![](https://img.shields.io/badge/-%20-000000?style=flat-square&logo=github&logoColor=FFFFFF&label=&labelColor=000000)
![](https://img.shields.io/badge/-%20-FC6D26?style=flat-square&logo=gitlab&logoColor=FFFFFF&label=&labelColor=FC6D26)

**Note:** Uses [Simple Icons](https://simpleicons.org/) via shields.io. Browse the site for available logos.

---

## Color Palette Reference

Built-in palette colors:

| Name | Hex | Swatch |
|------|-----|--------|
| `accent` | F41C80 | ![](https://img.shields.io/badge/-%20-F41C80?style=flat-square) |
| `success` | 22C55E | ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) |
| `warning` | EAB308 | ![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) |
| `error` | EF4444 | ![](https://img.shields.io/badge/-%20-EF4444?style=flat-square) |
| `info` | 3B82F6 | ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) |
| `slate` | 6B7280 | ![](https://img.shields.io/badge/-%20-6B7280?style=flat-square) |
| `ui.bg` | 292A2D | ![](https://img.shields.io/badge/-%20-292A2D?style=flat-square) |
| `ui.surface` | 292C34 | ![](https://img.shields.io/badge/-%20-292C34?style=flat-square) |
| `ui.panel` | 282F3C | ![](https://img.shields.io/badge/-%20-282F3C?style=flat-square) |
| `white` | FFFFFF | ![](https://img.shields.io/badge/-%20-FFFFFF?style=flat-square) |
| `black` | 000000 | ![](https://img.shields.io/badge/-%20-000000?style=flat-square) |

---

## Complete Examples

### Status Dashboard

**Syntax:**

```markdown
{{ui:swatch:22C55E:style=social/}} API Server: Online
{{ui:swatch:22C55E:style=social/}} Database: Healthy
{{ui:swatch:EAB308:style=social/}} Cache: Degraded
{{ui:swatch:EF4444:style=social/}} Queue: Critical
```

**Rendered:**

![](https://img.shields.io/badge/-%20-22C55E?style=social) API Server: Online
![](https://img.shields.io/badge/-%20-22C55E?style=social) Database: Healthy
![](https://img.shields.io/badge/-%20-EAB308?style=social) Cache: Degraded
![](https://img.shields.io/badge/-%20-EF4444?style=social) Queue: Critical

### Progress Bar (SVG Only)

Custom width progress bars require the SVG backend:

```markdown
{{ui:swatch:22C55E:width=200:height=20:label=Progress 75%/}}{{ui:swatch:333333:width=67:height=20/}}
```

For shields.io, use labels instead:

**Syntax:**

```markdown
{{ui:swatch:22C55E:label=75%/}}{{ui:swatch:333333/}}
```

**Rendered:**

![](https://img.shields.io/badge/-75%-22C55E?style=flat-square)![](https://img.shields.io/badge/-%20-333333?style=flat-square)

### Color Palette Documentation

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:1a1a2e:label=Primary/}}
{{ui:swatch:2d2d44:label=Secondary/}}
{{ui:swatch:4a4a6a:label=Tertiary/}}
{{ui:swatch:6b6b8d:label=Muted/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-Primary-1A1A2E?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-Secondary-2D2D44?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-Tertiary-4A4A6A?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-Muted-6B6B8D?style=flat-square">
</p>

### Color Gradient

**Syntax:**

```markdown
{{ui:swatch:0a0a0a/}}{{ui:swatch:1a0a0a/}}{{ui:swatch:3a1010/}}{{ui:swatch:5a1a1a/}}{{ui:swatch:8B2500/}}{{ui:swatch:CD3700/}}{{ui:swatch:FF4500/}}{{ui:swatch:FF6347/}}{{ui:swatch:FFD700/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-0A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-1A0A0A?style=flat-square)![](https://img.shields.io/badge/-%20-3A1010?style=flat-square)![](https://img.shields.io/badge/-%20-5A1A1A?style=flat-square)![](https://img.shields.io/badge/-%20-8B2500?style=flat-square)![](https://img.shields.io/badge/-%20-CD3700?style=flat-square)![](https://img.shields.io/badge/-%20-FF4500?style=flat-square)![](https://img.shields.io/badge/-%20-FF6347?style=flat-square)![](https://img.shields.io/badge/-%20-FFD700?style=flat-square)

### Tech Stack with Icons

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:DEA584:icon=rust:icon_color=000000/}}
{{ui:swatch:F7DF1E:icon=javascript:icon_color=000000/}}
{{ui:swatch:3178C6:icon=typescript:icon_color=FFFFFF/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-%20-DEA584?style=flat-square&logo=rust&logoColor=000000&label=&labelColor=DEA584"> <img alt="" src="https://img.shields.io/badge/-%20-F7DF1E?style=flat-square&logo=javascript&logoColor=000000&label=&labelColor=F7DF1E"> <img alt="" src="https://img.shields.io/badge/-%20-3178C6?style=flat-square&logo=typescript&logoColor=FFFFFF&label=&labelColor=3178C6">
</p>

### Warning Labels

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:FFFF00:label=RADIATION/}}
{{ui:swatch:FF6600:label=BIOHAZARD/}}
{{ui:swatch:FF0000:label=DANGER/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-RADIATION-FFFF00?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-BIOHAZARD-FF6600?style=flat-square"> <img alt="" src="https://img.shields.io/badge/-DANGER-FF0000?style=flat-square">
</p>

---

## SVG-Only Examples

The following examples require `--backend svg` for full effect:

### Glassmorphism Effect

```markdown
{{ui:swatch:FFFFFF:width=200:height=60:opacity=0.15:border=FFFFFF:border_width=1/}}
{{ui:swatch:FFFFFF:width=180:height=50:opacity=0.2:border=FFFFFF:border_width=1:label=Glass Card/}}
```

### Pixel Art

Custom dimensions enable pixel art:

```markdown
{{ui:swatch:0a0a0a:width=8:height=8/}}{{ui:swatch:708090:width=8:height=8/}}...
```

*Pixel art requires `--backend svg` for precise dimensions.*

---

## Backend Differences

### Shields Backend (default)

Uses shields.io badges. Supports:
- Color (palette names or hex)
- Style (5 badge styles)
- Labels (text on badges)
- Icons (Simple Icons logos)

```bash
mdfx process template.md --target github
```

### SVG Backend

Full control over all parameters:

```bash
mdfx process template.md --backend svg --assets-dir assets
```

Additional SVG-only features:
- Custom width/height
- Opacity/transparency
- Borders with custom colors/width

---

## Tips & Tricks

### 1. Center with Row

**Syntax:**

```markdown
{{ui:row:align=center}}
{{ui:swatch:accent:label=CENTERED/}}
{{/ui}}
```

**Rendered:**

<p align="center">
<img alt="" src="https://img.shields.io/badge/-CENTERED-F41C80?style=flat-square">
</p>

### 2. Style Variations

Different styles for different contexts:

**Syntax:**

```markdown
{{ui:swatch:accent:style=flat-square:label=Default/}}
{{ui:swatch:accent:style=for-the-badge:label=Prominent/}}
{{ui:swatch:accent:style=social:label=Subtle/}}
```

**Rendered:**

![](https://img.shields.io/badge/-Default-F41C80?style=flat-square)
![](https://img.shields.io/badge/-Prominent-F41C80?style=for-the-badge)
![](https://img.shields.io/badge/-Subtle-F41C80?style=social)

### 3. Icon + Label Combinations

```markdown
{{ui:swatch:000000:icon=github:icon_color=FFFFFF:label=GitHub/}}
```

**Rendered:**

![](https://img.shields.io/badge/-%20-000000?style=flat-square&logo=github&logoColor=FFFFFF&label=&labelColor=000000)

### 4. SVG-Only: Opacity for Layering

*Requires `--backend svg`:*

```markdown
{{ui:swatch:accent:width=100:height=30:opacity=1.0/}}
{{ui:swatch:accent:width=90:height=30:opacity=0.7/}}
{{ui:swatch:accent:width=80:height=30:opacity=0.4/}}
```

### 5. SVG-Only: Border Highlights

*Requires `--backend svg`:*

```markdown
{{ui:swatch:1a1a2e:width=200:height=60:border=F41C80:border_width=3:label=HIGHLIGHTED/}}
```

---

## See Also

- [Components Guide](COMPONENTS-GUIDE.md)
- [API Guide](../API-GUIDE.md)
- [Examples README](../../examples/README.md)

---

<p align="center">
ʀᴇɴᴅᴇʀᴇᴅ ᴡɪᴛʜ ᴍᴅꜰx
</p>
