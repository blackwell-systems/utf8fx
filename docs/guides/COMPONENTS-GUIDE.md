# Components Guide

Components are reusable UI elements that render to visual primitives like badges and status indicators. They use the `{{ui:component}}` namespace.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [Native Components](#native-components)
  - [swatch](#swatch)
  - [tech](#tech)
  - [row](#row)
- [Expand Components](#expand-components)
  - [callout-github](#callout-github)
  - [statusitem](#statusitem)
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

## Native Components

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

## Expand Components

Components that expand to templates with other mdfx syntax.

### callout-github

GitHub-style blockquote callout with status emoji.

**Syntax:**
```markdown
{{ui:callout-github:type}}Content{{/ui}}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `type` | string | Status type (success, warning, error, info) |

**Expands to:**
```markdown
> {{ui:swatch:type/}} **Note**
> Content
```

**Example:**
```markdown
{{ui:callout-github:info}}Check the documentation for more details.{{/ui}}
```

---

### statusitem

Inline status indicator with label and description.

**Syntax:**
```markdown
{{ui:statusitem:Label:level:Description text/}}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `label` | string | Display label |
| `level` | string | Status color |
| `text` | string | Description |

**Expands to:**
```markdown
{{ui:swatch:level/}} **Label**: Description text
```

**Example:**
```markdown
{{ui:statusitem:Build:success:Completed in 2.3s/}}
{{ui:statusitem:Tests:warning:3 skipped/}}
{{ui:statusitem:Deploy:error:Connection failed/}}
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

### GitHub Callout
```markdown
{{ui:callout-github:warning}}
Breaking changes in v2.0. See migration guide.
{{/ui}}
```

### Color Palette Row
```markdown
{{ui:row:align=center}}
{{ui:swatch:accent/}} {{ui:swatch:success/}} {{ui:swatch:warning/}} {{ui:swatch:error/}} {{ui:swatch:info/}}
{{/ui}}
```

### Build Status Line
```markdown
{{ui:statusitem:Build:success:v1.2.3/}} {{ui:statusitem:Coverage:info:94%/}}
```

---

## Component Reference

| Component | Type | Self-Closing | Context |
|-----------|------|--------------|---------|
| `swatch` | native | yes | inline, block |
| `tech` | native | yes | inline, block |
| `row` | native | no | block |
| `callout-github` | expand | no | block |
| `statusitem` | expand | yes | inline, block |

---

## Tips

1. **Use row for centering** - `{{ui:row:align=center}}` creates GitHub-compatible centered layouts
2. **Consistent styling** - Pick one badge style and use it throughout your document
3. **Palette colors** - Use named colors: `success` (green), `warning` (yellow), `error` (red), `info` (blue)
4. **Inline vs block** - Most components work in both contexts; row is block-only
