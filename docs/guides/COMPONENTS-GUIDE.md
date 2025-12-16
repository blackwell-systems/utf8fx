# Components Guide

Components are reusable UI elements that render to visual primitives like badges and status indicators. They use the `{{ui:component}}` namespace.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [Components](#components)
  - [swatch](#swatch)
  - [tech](#tech)
  - [progress](#progress)
  - [donut](#donut)
  - [gauge](#gauge)
  - [row](#row)
- [Badge Styles](#badge-styles)
- [Practical Examples](#practical-examples)
- [Component Reference](#component-reference)
- [Tips](#tips)

## Basic Syntax

**Self-closing:**
```markdown
{{ui:component:arg1:arg2/}}
```

**With content:**
```markdown
{{ui:component:arg}}Content here{{/ui}}
```

**With optional parameters:**
```markdown
{{ui:component:arg:param=value/}}
```

---

## Components

### swatch

Renders a colored block. The foundation for visual elements.

**Syntax:**
```markdown
{{ui:swatch:color/}}
{{ui:swatch:color:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `color` | string | required | Palette name or hex code |
| `style` | string | flat-square | Badge style |
| `width` | number | 40 | Width in pixels |
| `height` | number | 20 | Height in pixels |
| `opacity` | float | 1.0 | Transparency (0.0-1.0) |
| `border` | string | none | Border color |
| `border_width` | number | 1 | Border thickness |
| `label` | string | none | Text overlay |
| `label_color` | string | white | Label text color |
| `icon` | string | none | Simple Icons logo |
| `icon_color` | string | white | Icon color |

**Examples:**
```markdown
{{ui:swatch:accent/}}
{{ui:swatch:FF5500/}}
{{ui:swatch:success:style=plastic/}}
{{ui:swatch:info:width=100:height=30:label=Status/}}
```

See [SWATCH-GUIDE.md](SWATCH-GUIDE.md) for complete documentation.

---

### tech

Displays a technology logo badge using Simple Icons.

**Syntax:**
```markdown
{{ui:tech:logo-name/}}
{{ui:tech:logo-name:style=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `logo` | string | required | Simple Icons name (rust, python, docker, etc.) |
| `style` | string | flat-square | Badge style |

**Examples:**
```markdown
{{ui:tech:rust/}} {{ui:tech:python/}} {{ui:tech:typescript/}}
{{ui:tech:docker:style=for-the-badge/}}
```

**Common logos:** rust, python, typescript, javascript, go, docker, kubernetes, react, vue, svelte, nodejs, postgresql, redis, github, gitlab

**Note:** Background uses `ui.bg` color for consistent dark theme appearance.

---

### progress

Renders a progress bar or slider with customizable colors, sizes, and optional slider mode.

**Syntax:**
```markdown
{{ui:progress:percent/}}
{{ui:progress:percent:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | required | Progress percentage (0-100) |
| `width` | number | 100 | Total width in pixels |
| `height` | number | 10 | Track height in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `fill_height` | number | height | Fill height (less creates floating effect) |
| `rx` | number | 3 | Corner radius |
| `label` | boolean | false | Show percentage label |
| `label_color` | color | white | Label text color |
| `border` | color | none | Border color |
| `border_width` | number | 1 | Border width in pixels |
| `thumb` | number | none | Thumb size (enables slider mode) |
| `thumb_color` | color | fill | Thumb color |
| `thumb_shape` | string | circle | Thumb shape: circle, square, diamond |

**Basic Examples:**
```markdown
{{ui:progress:75/}}
{{ui:progress:50:width=200:fill=success/}}
{{ui:progress:80:height=12:fill_height=8/}}
{{ui:progress:65:label=true/}}
```

**With Border:**
```markdown
{{ui:progress:70:border=accent/}}
{{ui:progress:85:border=success:border_width=2/}}
```

**Slider Mode:**
```markdown
{{ui:progress:50:thumb=14/}}
{{ui:progress:75:thumb=16:thumb_color=accent/}}
{{ui:progress:30:thumb=14:thumb_shape=square/}}
{{ui:progress:60:thumb=14:thumb_shape=diamond:thumb_color=warning/}}
```

**Note:** When `thumb` is set, the progress bar renders as a slider with a thin track and positioned thumb indicator.

---

### donut

Renders a circular progress/ring chart showing a percentage.

**Syntax:**
```markdown
{{ui:donut:percent/}}
{{ui:donut:percent:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | required | Progress percentage (0-100) |
| `size` | number | 40 | Diameter in pixels |
| `thickness` | number | 4 | Ring thickness in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `label` | boolean | false | Show percentage label in center |
| `label_color` | color | white | Label text color |
| `thumb` | number | none | Thumb size in pixels (enables slider mode) |
| `thumb_color` | color | fill | Thumb color (defaults to fill color) |

**Basic Examples:**
```markdown
{{ui:donut:75/}}
{{ui:donut:50:size=60:thickness=8/}}
{{ui:donut:90:fill=success/}}
```

**With Label:**
```markdown
{{ui:donut:75:label=true/}}
{{ui:donut:50:size=60:label=true:label_color=accent/}}
```

**Custom Colors:**
```markdown
{{ui:donut:80:fill=success:track=slate/}}
{{ui:donut:25:fill=warning:track=error/}}
```

**Slider Mode (with thumb):**
```markdown
{{ui:donut:75:thumb=12/}}
{{ui:donut:50:size=60:thumb=16:thumb_color=accent/}}
```

**Note:** Donuts use SVG stroke-dasharray for smooth, scalable rendering. The percentage fills clockwise from the top. When `thumb` is set, a circular indicator appears at the fill endpoint.

---

### gauge

Renders a semi-circular gauge/meter showing a percentage. Perfect for dashboards and speedometer-style displays.

**Syntax:**
```markdown
{{ui:gauge:percent/}}
{{ui:gauge:percent:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | required | Progress percentage (0-100) |
| `size` | number | 80 | Width in pixels |
| `thickness` | number | 8 | Arc thickness in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `label` | boolean | false | Show percentage label below arc |
| `label_color` | color | white | Label text color |
| `thumb` | number | none | Thumb size in pixels (enables slider mode) |
| `thumb_color` | color | fill | Thumb color (defaults to fill color) |

**Basic Examples:**
```markdown
{{ui:gauge:75/}}
{{ui:gauge:50:size=120:thickness=12/}}
{{ui:gauge:90:fill=success/}}
```

**With Label:**
```markdown
{{ui:gauge:75:label=true/}}
{{ui:gauge:85:size=100:label=true:label_color=accent/}}
```

**Speedometer Style:**
```markdown
{{ui:gauge:25:fill=success:size=100:thickness=10/}}
{{ui:gauge:55:fill=warning:size=100:thickness=10/}}
{{ui:gauge:85:fill=error:size=100:thickness=10/}}
```

**Neon Colors:**
```markdown
{{ui:gauge:75:fill=00FF41:track=0D0D0D:size=100:thickness=8/}}
```

**Slider Mode (with thumb):**
```markdown
{{ui:gauge:75:thumb=14/}}
{{ui:gauge:50:size=100:thumb=18:thumb_color=accent/}}
```

**Note:** Gauges use SVG arc paths with stroke-dasharray for smooth semi-circular rendering. The arc spans from left to right (180°). When `thumb` is set, a circular indicator appears at the fill endpoint.

---

### row

Wraps content in an HTML container with horizontal alignment. Converts markdown images to HTML for GitHub compatibility.

**Syntax:**
```markdown
{{ui:row:align=value}}
content
{{/ui}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `align` | enum | center | Horizontal alignment (left, center, right) |

**Examples:**
```markdown
{{ui:row:align=center}}
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:warning/}}
{{/ui}}

{{ui:row:align=right}}
{{ui:tech:rust/}} {{ui:tech:go/}}
{{/ui}}
```

**Output (HTML):**
```html
<p align="center">
<img src="...accent..."> <img src="...success..."> <img src="...warning...">
</p>
```

---

## Badge Styles

All components that render badges support these styles:

| Style | Description |
|-------|-------------|
| `flat-square` | Square corners, flat design (default) |
| `flat` | Rounded corners, flat design |
| `plastic` | Glossy plastic appearance |
| `for-the-badge` | Tall header bar style |
| `social` | Social media pill shape |

**Example:**
```markdown
{{ui:swatch:accent:style=flat/}}
{{ui:swatch:accent:style=plastic/}}
{{ui:swatch:accent:style=for-the-badge/}}
{{ui:swatch:accent:style=social/}}
```

---

## Practical Examples

### Tech Stack Display
```markdown
{{ui:row:align=center}}
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}}
{{/ui}}
```

### Status Dashboard
```markdown
| Service | Status |
|---------|--------|
| API | {{ui:swatch:success/}} |
| Database | {{ui:swatch:success/}} |
| Cache | {{ui:swatch:warning/}} |
```

### Color Palette Row
```markdown
{{ui:row:align=center}}
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:warning/}} {{ui:swatch:error/}} {{ui:swatch:info/}}
{{/ui}}
```

### Skill Bars
```markdown
| Skill | Level |
|-------|-------|
| Rust | {{ui:progress:90:width=120:fill=success/}} |
| Python | {{ui:progress:75:width=120/}} |
| Go | {{ui:progress:60:width=120:fill=info/}} |
```

### Volume Slider
```markdown
Volume: {{ui:progress:65:width=150:thumb=12/}}
```

### Completion Donuts
```markdown
| Task | Status |
|------|--------|
| Tests | {{ui:donut:100:fill=success:label=true/}} |
| Coverage | {{ui:donut:85:fill=info:label=true/}} |
| Docs | {{ui:donut:60:fill=warning:label=true/}} |
```

### System Gauges
```markdown
| Metric | Status |
|--------|--------|
| CPU | {{ui:gauge:73:fill=info:size=60:thickness=6/}} |
| Memory | {{ui:gauge:45:fill=success:size=60:thickness=6/}} |
| Disk | {{ui:gauge:88:fill=warning:size=60:thickness=6/}} |
```

---

## Component Reference

| Component | Type | Self-Closing | Context |
|-----------|------|--------------|---------|
| `swatch` | native | yes | inline, block |
| `tech` | native | yes | inline, block |
| `progress` | native | yes | inline, block |
| `donut` | native | yes | inline, block |
| `gauge` | native | yes | inline, block |
| `row` | native | no | block |

---

## Tips

1. **Use row for centering** - `{{ui:row:align=center}}` creates GitHub-compatible centered layouts
2. **Consistent styling** - Pick one badge style and use it throughout your document
3. **Palette colors** - Use named colors: `success` (green), `warning` (yellow), `error` (red), `info` (blue)
4. **Inline vs block** - Most components work in both contexts; row is block-only
