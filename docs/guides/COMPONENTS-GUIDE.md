# Components Guide

Components are reusable UI elements that render to visual primitives like badges and status indicators. They use the `{{ui:component}}` namespace.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [Components](#components)
  - [swatch](#swatch)
  - [tech](#tech)
  - [progress](#progress)
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

---

## Component Reference

| Component | Type | Self-Closing | Context |
|-----------|------|--------------|---------|
| `swatch` | native | yes | inline, block |
| `tech` | native | yes | inline, block |
| `progress` | native | yes | inline, block |
| `row` | native | no | block |

---

## Tips

1. **Use row for centering** - `{{ui:row:align=center}}` creates GitHub-compatible centered layouts
2. **Consistent styling** - Pick one badge style and use it throughout your document
3. **Palette colors** - Use named colors: `success` (green), `warning` (yellow), `error` (red), `info` (blue)
4. **Inline vs block** - Most components work in both contexts; row is block-only
