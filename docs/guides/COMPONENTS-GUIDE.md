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
  - [sparkline](#sparkline)
  - [rating](#rating)
  - [row](#row)
  - [waveform](#waveform)
  - [tech-group](#tech-group)
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

Displays a technology logo badge using Simple Icons with two-segment design.

**Syntax:**
```markdown
{{ui:tech:logo-name/}}
{{ui:tech:logo-name:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | string | required | Simple Icons name (rust, python, docker, etc.) |
| `style` | string | flat-square | Badge style |
| `bg` | color | brand color | Background color (uses brand color if available) |
| `logo` | color | auto | Logo color (auto-selects black/white based on bg luminance) |
| `label` | string | name | Label text (defaults to tech name) |
| `text_color` | color | auto | Label text color (aliases: `text`, `color`) |
| `font` | string | Verdana | Font family (alias: `font_family`) |
| `border` | color | none | Border color |
| `border_width` | number | none | Border width in pixels |
| `rx` | number | 0 | Corner radius |
| `source` | enum | svg | Rendering source: `svg` (local file) or `shields` (shields.io URL) |

**Basic Examples:**
```markdown
{{ui:tech:rust/}}
{{ui:tech:python/}}
{{ui:tech:typescript/}}
{{ui:tech:docker:style=for-the-badge/}}
```

**Intelligent Logo Colors:**

Logo colors automatically select black or white based on background luminance:

| Background | Logo Color | Example |
|------------|------------|---------|
| Light (orange, cyan) | Black | Rust, Go |
| Dark (blue, black) | White | PostgreSQL, Docker |

Override with `logo` parameter:
```markdown
{{ui:tech:rust:logo=white/}}           <!-- Force white logo -->
{{ui:tech:docker:logo=000000/}}        <!-- Force black logo -->
```

**Text Customization:**
```markdown
{{ui:tech:rust:text_color=white/}}           <!-- White text -->
{{ui:tech:python:font=Monaco,monospace/}}    <!-- Custom font -->
{{ui:tech:go:text=000000:font=Arial/}}       <!-- Both -->
```

**Custom Background:**
```markdown
{{ui:tech:rust:bg=000000/}}            <!-- Black background -->
{{ui:tech:docker:bg=accent/}}          <!-- Theme color background -->
```

**Custom Label:**
```markdown
{{ui:tech:typescript:label=TS/}}       <!-- Short label -->
{{ui:tech:rust:label=Rust 1.75/}}      <!-- Version label -->
```

**Common logos:** rust, python, typescript, javascript, go, docker, kubernetes, react, vue, svelte, nodejs, postgresql, redis, github, gitlab, npm, aws, azure, googlecloud

**Note:** Tech badges use brand colors from Simple Icons by default. The two-segment design shows the logo on the left and label on the right.

See [TECH-GUIDE.md](TECH-GUIDE.md) for complete documentation.

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
| `thumb` | number | none | Thumb height (enables slider mode) |
| `thumb_width` | number | thumb | Thumb width (defaults to thumb for circle) |
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

**Wide/Pill-shaped Thumb:**
```markdown
{{ui:progress:50:thumb=12:thumb_width=20/}}
{{ui:progress:75:thumb=10:thumb_width=24:thumb_color=accent/}}
```

**Note:** When `thumb` is set, the progress bar renders as a slider with a thin track and positioned thumb indicator. Use `thumb_width` to create oval or pill-shaped thumbs.

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

### sparkline

Renders a compact inline chart for trend visualization.

**Syntax:**
```markdown
{{ui:sparkline:values/}}
{{ui:sparkline:values:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `values` | string | required | Comma-separated numbers |
| `width` | number | 100 | Chart width in pixels |
| `height` | number | 20 | Chart height in pixels |
| `type` | string | line | Chart type: line, bar, area |
| `fill` | color | accent | Fill/bar color |
| `stroke` | color | fill | Line stroke color |
| `stroke_width` | number | 2 | Line thickness |
| `track` | color | none | Background color |
| `dots` | boolean | false | Show data points |
| `dot_radius` | number | 2 | Dot radius |

**Basic Examples:**
```markdown
{{ui:sparkline:10,25,15,30,20,35/}}
{{ui:sparkline:5,8,3,9,4,7:type=bar/}}
{{ui:sparkline:1,4,2,5,3,6:type=area:fill=info/}}
```

**With Data Points:**
```markdown
{{ui:sparkline:10,20,15,25,30:dots=true/}}
{{ui:sparkline:5,15,10,20:dots=true:dot_radius=3:stroke=success/}}
```

**Custom Sizing:**
```markdown
{{ui:sparkline:20,40,35,50,45,60:width=150:height=30/}}
{{ui:sparkline:10,5,15,8,20:width=80:height=16:type=bar/}}
```

**With Background:**
```markdown
{{ui:sparkline:30,45,20,55,40:track=ui.bg:fill=accent/}}
```

---

### rating

Renders a star/heart/circle rating with partial fill support.

**Syntax:**
```markdown
{{ui:rating:value/}}
{{ui:rating:value:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | float | required | Rating value (e.g., 3.5) |
| `max` | number | 5 | Maximum rating (number of icons) |
| `size` | number | 20 | Icon size in pixels |
| `fill` | color | warning | Filled icon color |
| `empty` | color | slate | Empty icon color |
| `icon` | string | star | Icon: star, heart, circle |
| `spacing` | number | 2 | Space between icons |

**Basic Examples:**
```markdown
{{ui:rating:4/}}
{{ui:rating:3.5/}}
{{ui:rating:2.5:max=5/}}
```

**Different Icons:**
```markdown
{{ui:rating:4:icon=star/}}
{{ui:rating:4:icon=heart:fill=error/}}
{{ui:rating:3:icon=circle:fill=info/}}
```

**Custom Scale:**
```markdown
{{ui:rating:7.5:max=10/}}
{{ui:rating:8:max=10:size=16/}}
```

**Custom Colors:**
```markdown
{{ui:rating:4.5:fill=warning:empty=ui.bg/}}
{{ui:rating:3:fill=accent:empty=slate/}}
```

**Large Size:**
```markdown
{{ui:rating:5:size=32:spacing=4/}}
```

**Note:** Ratings support decimal values for partial fills. A value of 3.5 shows 3 full icons and half of the 4th icon filled.

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

### waveform

Renders an audio-style waveform visualization with bars above/below center.

**Syntax:**
```markdown
{{ui:waveform:values/}}
{{ui:waveform:values:param=value/}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `values` | string | required | Comma-separated floats (-1 to 1) |
| `width` | number | 200 | Width in pixels |
| `height` | number | 40 | Height in pixels |
| `positive` | color | success | Positive bar color |
| `negative` | color | error | Negative bar color |
| `bar_width` | number | 3 | Width of each bar |
| `spacing` | number | 1 | Space between bars |
| `track` | color | none | Background color |
| `center` | boolean | true | Show center line |
| `center_color` | color | slate | Center line color |

**Basic Examples:**
```markdown
{{ui:waveform:0.5,0.8,-0.3,0.6,-0.7,0.4,-0.2,0.9,-0.5,0.3/}}
{{ui:waveform:0.2,0.5,0.8,0.4,0.1,-0.2,-0.5,-0.8,-0.4,-0.1/}}
```

**Custom Colors:**
```markdown
{{ui:waveform:0.5,-0.5,0.8,-0.3:positive=accent:negative=warning/}}
```

**Note:** Values range from -1 to 1. Positive values render above center, negative below.

---

### tech-group

Groups multiple tech badges with automatic corner handling for seamless pill-style display.

**Syntax:**
```markdown
{{ui:tech-group:gap=value}}
{{ui:tech:name/}}
{{ui:tech:name/}}
{{/ui}}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `gap` | number | 0 | Gap between badges in pixels |

**Examples:**
```markdown
{{ui:tech-group}}
{{ui:tech:rust/}}
{{ui:tech:typescript/}}
{{ui:tech:docker/}}
{{/ui}}

{{ui:tech-group:gap=4}}
{{ui:tech:python/}}
{{ui:tech:go/}}
{{/ui}}
```

**Note:** When gap=0, badges merge into a single pill. When gap>0, they appear as separate rounded badges.

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
| `tech-group` | native | no | block |
| `progress` | native | yes | inline, block |
| `donut` | native | yes | inline, block |
| `gauge` | native | yes | inline, block |
| `sparkline` | native | yes | inline, block |
| `waveform` | native | yes | inline, block |
| `rating` | native | yes | inline, block |
| `row` | native | no | block |

---

## Tips

1. **Use row for centering** - `{{ui:row:align=center}}` creates GitHub-compatible centered layouts
2. **Consistent styling** - Pick one badge style and use it throughout your document
3. **Palette colors** - Use named colors: `success` (green), `warning` (yellow), `error` (red), `info` (blue)
4. **Inline vs block** - Most components work in both contexts; row is block-only
