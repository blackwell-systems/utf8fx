# Components Guide

Components are reusable UI elements that render to visual primitives like badges, dividers, and status indicators. They use the `{{ui:component}}` namespace.

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

### divider

Creates a gradient color bar for section separation.

**Syntax:**
```markdown
{{ui:divider/}}
{{ui:divider:style=plastic/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `style` | string | flat-square | Badge style |

**Colors:** Uses theme gradient: `ui.bg` → `ui.surface` → `accent` → `ui.panel`

**Example:**
```markdown
## Section Title
{{ui:divider/}}

Content goes here...
```

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

### status

Renders a colored status indicator block.

**Syntax:**
```markdown
{{ui:status:level/}}
{{ui:status:level:style=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `level` | string | required | Status level (success, warning, error, info) |
| `style` | string | flat-square | Badge style |

**Examples:**
```markdown
{{ui:status:success/}} All tests passing
{{ui:status:warning/}} Deprecated feature
{{ui:status:error/}} Build failed
{{ui:status:info/}} New version available
```

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

### header

Section header with gradient frame and bold mathematical text.

**Syntax:**
```markdown
{{ui:header}}Title Text{{/ui}}
```

**Expands to:**
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}Title Text{{/mathbold}}{{/frame}}
```

**Example:**
```markdown
{{ui:header}}GETTING STARTED{{/ui}}
```

**Output:**
```
▓▒░ 𝐆𝐄𝐓𝐓𝐈𝐍𝐆 · 𝐒𝐓𝐀𝐑𝐓𝐄𝐃 ░▒▓
```

---

### section

Creates a markdown heading with gradient divider underneath.

**Syntax:**
```markdown
{{ui:section:Title Text/}}
```

**Expands to:**
```markdown
## Title Text
{{ui:divider/}}
```

**Example:**
```markdown
{{ui:section:Installation/}}

Follow these steps to install...
```

---

### callout

Callout box with status color indicator.

**Syntax:**
```markdown
{{ui:callout:level}}Content{{/ui}}
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `level` | string | Status color (success, warning, error, info) |

**Expands to:**
```markdown
{{frame:solid-left}}{{shields:block:color=level:style=flat-square/}} Content{{/frame}}
```

**Example:**
```markdown
{{ui:callout:warning}}This action cannot be undone.{{/ui}}
```

---

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
> {{ui:status:type/}} **Note**
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
{{ui:status:level/}} **Label**: Description text
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
| API | {{ui:status:success/}} |
| Database | {{ui:status:success/}} |
| Cache | {{ui:status:warning/}} |
```

### Section with Divider
```markdown
{{ui:section:Features/}}

- Fast compilation
- Type safety
- Zero-cost abstractions
```

### Callout Box
```markdown
{{ui:callout:warning}}
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
| `divider` | native | yes | block |
| `tech` | native | yes | inline, block |
| `status` | native | yes | inline, block |
| `row` | native | no | block |
| `header` | expand | no | block |
| `section` | expand | yes | block |
| `callout` | expand | no | block |
| `callout-github` | expand | no | block |
| `statusitem` | expand | yes | inline, block |

---

## Tips

1. **Use row for centering** - `{{ui:row:align=center}}` creates GitHub-compatible centered layouts
2. **Consistent styling** - Pick one badge style and use it throughout your document
3. **Status semantics** - success=green, warning=yellow, error=red, info=blue
4. **Section organization** - Use `{{ui:section:Title/}}` for consistent heading styles
5. **Inline vs block** - Most components work in both contexts; row is block-only
