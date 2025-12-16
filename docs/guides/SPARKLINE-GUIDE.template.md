# Sparkline Component Guide

Mini inline charts for data visualization in markdown.

## Quick Start

```markdown
{{ui:sparkline:1,3,2,6,4,8,5,7/}}
```

Renders a line chart with the provided comma-separated values.

---

## Chart Types

### Line Chart (Default)

The default chart type renders a smooth polyline connecting data points.

{{ui:sparkline:1,3,2,6,4,8,5,7/}}

{{ui:sparkline:10,20,15,30,25,40,35,45/}}

{{ui:sparkline:5,2,8,1,9,3,7,4,6/}}

### Bar Chart

Vertical bars for each data point.

{{ui:sparkline:1,3,2,6,4,8,5,7:type=bar/}}

{{ui:sparkline:10,20,15,30,25,40,35,45:type=bar/}}

{{ui:sparkline:5,2,8,1,9,3,7,4,6:type=bar/}}

### Area Chart

Filled area under the line for emphasis.

{{ui:sparkline:1,3,2,6,4,8,5,7:type=area/}}

{{ui:sparkline:10,20,15,30,25,40,35,45:type=area/}}

{{ui:sparkline:5,2,8,1,9,3,7,4,6:type=area/}}

---

## Styling Options

### With Dots

Add dots at each data point (line chart only).

{{ui:sparkline:1,3,2,6,4,8,5,7:dots=true/}}

{{ui:sparkline:10,20,15,30,25,40,35,45:dots=true/}}

{{ui:sparkline:5,2,8,1,9,3,7,4,6:dots=true:dot_radius=3/}}

### Custom Sizes

Control width and height.

**Small (60x15):**
{{ui:sparkline:1,3,2,6,4,8,5,7:width=60:height=15/}}

**Medium (100x20) - Default:**
{{ui:sparkline:1,3,2,6,4,8,5,7/}}

**Large (150x30):**
{{ui:sparkline:1,3,2,6,4,8,5,7:width=150:height=30/}}

**Wide (200x25):**
{{ui:sparkline:1,3,2,6,4,8,5,7:width=200:height=25/}}

**Tall (80x40):**
{{ui:sparkline:1,3,2,6,4,8,5,7:width=80:height=40/}}

### Custom Colors

Use named palette colors or hex values.

**Accent (default):**
{{ui:sparkline:1,3,2,6,4,8,5,7/}}

**Success (green):**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=success/}}

**Warning (yellow):**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=warning/}}

**Error (red):**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=error/}}

**Info (blue):**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=info/}}

**Cobalt:**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=cobalt/}}

**Plum:**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=plum/}}

**Custom Hex:**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=FF6B35/}}

### Stroke Customization

Control line width and separate stroke color.

**Thin stroke (1px):**
{{ui:sparkline:1,3,2,6,4,8,5,7:stroke_width=1/}}

**Default stroke (2px):**
{{ui:sparkline:1,3,2,6,4,8,5,7/}}

**Thick stroke (3px):**
{{ui:sparkline:1,3,2,6,4,8,5,7:stroke_width=3/}}

**Custom stroke color:**
{{ui:sparkline:1,3,2,6,4,8,5,7:fill=accent:stroke=white/}}

### Track Color (Background)

Add a background track behind the chart.

{{ui:sparkline:1,3,2,6,4,8,5,7:track=slate/}}

{{ui:sparkline:1,3,2,6,4,8,5,7:type=bar:track=slate/}}

{{ui:sparkline:1,3,2,6,4,8,5,7:type=area:track=slate/}}

---

## Data Patterns

### Trending Up

{{ui:sparkline:1,2,3,4,5,6,7,8,9,10/}}

{{ui:sparkline:1,2,3,4,5,6,7,8,9,10:type=bar:fill=success/}}

{{ui:sparkline:1,2,3,4,5,6,7,8,9,10:type=area:fill=success/}}

### Trending Down

{{ui:sparkline:10,9,8,7,6,5,4,3,2,1/}}

{{ui:sparkline:10,9,8,7,6,5,4,3,2,1:type=bar:fill=error/}}

{{ui:sparkline:10,9,8,7,6,5,4,3,2,1:type=area:fill=error/}}

### Volatile/Fluctuating

{{ui:sparkline:5,9,2,8,3,7,4,6,1,10/}}

{{ui:sparkline:5,9,2,8,3,7,4,6,1,10:type=bar:fill=warning/}}

{{ui:sparkline:5,9,2,8,3,7,4,6,1,10:type=area:fill=warning/}}

### Flat/Stable

{{ui:sparkline:5,5,5,5,5,5,5,5/}}

{{ui:sparkline:5,5.1,4.9,5,5.2,4.8,5,5:type=area:fill=info/}}

### Peak in Middle

{{ui:sparkline:1,3,5,8,10,8,5,3,1/}}

{{ui:sparkline:1,3,5,8,10,8,5,3,1:type=bar:fill=plum/}}

{{ui:sparkline:1,3,5,8,10,8,5,3,1:type=area:fill=plum/}}

### Valley in Middle

{{ui:sparkline:10,8,5,3,1,3,5,8,10/}}

{{ui:sparkline:10,8,5,3,1,3,5,8,10:type=bar:fill=cobalt/}}

{{ui:sparkline:10,8,5,3,1,3,5,8,10:type=area:fill=cobalt/}}

---

## Combined Options

### Large Line with Dots

{{ui:sparkline:1,3,2,6,4,8,5,7:width=200:height=40:dots=true:dot_radius=4:stroke_width=3/}}

### Bar Chart with Track

{{ui:sparkline:3,7,2,8,4,9,5,6:type=bar:width=150:height=30:track=slate:fill=success/}}

### Area Chart Custom Style

{{ui:sparkline:1,4,2,7,3,8,5,9:type=area:width=180:height=35:fill=plum:track=ui.surface/}}

### Compact Inline Indicators

Uptrend: {{ui:sparkline:1,2,3,4,5:width=40:height=12:fill=success/}}
Downtrend: {{ui:sparkline:5,4,3,2,1:width=40:height=12:fill=error/}}
Stable: {{ui:sparkline:3,3,3,3,3:width=40:height=12:fill=info/}}

---

## Use Cases

### Dashboard Metrics

**CPU Usage:** {{ui:sparkline:45,52,48,65,72,58,63,55,48,52:width=80:height=16:fill=info/}}

**Memory:** {{ui:sparkline:65,68,72,75,73,78,82,80,77,75:width=80:height=16:fill=warning/}}

**Disk I/O:** {{ui:sparkline:20,35,28,42,55,38,45,32,28,25:width=80:height=16:fill=success/}}

### Stock/Financial Data

**Stock A:** {{ui:sparkline:100,105,102,110,108,115,120,118,125,130:fill=success:width=100:height=20/}} +30%

**Stock B:** {{ui:sparkline:100,98,95,92,88,85,82,80,78,75:fill=error:width=100:height=20/}} -25%

**Stock C:** {{ui:sparkline:100,102,98,104,96,105,95,103,97,100:fill=slate:width=100:height=20/}} 0%

### Activity Timeline

**Weekly commits:** {{ui:sparkline:5,12,8,15,22,18,10:type=bar:width=120:height=20:fill=plum/}}

**Daily visitors:** {{ui:sparkline:120,145,132,168,155,189,142:type=area:width=120:height=20:fill=cobalt/}}

---

## Parameter Reference

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `values` | comma-separated numbers | (required) | Data points to visualize |
| `width` | number | 100 | Chart width in pixels |
| `height` | number | 20 | Chart height in pixels |
| `type` | line, bar, area | line | Chart type |
| `fill` | color | accent | Fill/stroke color |
| `stroke` | color | same as fill | Line stroke color |
| `stroke_width` | number | 2 | Line thickness |
| `track` | color | none | Background color |
| `dots` | boolean | false | Show dots on line chart |
| `dot_radius` | number | 2 | Dot size when dots=true |

---

## Color Palette Quick Reference

| Color | Value | Preview |
|-------|-------|---------|
| accent | F41C80 | {{ui:swatch:accent/}} |
| success | 22C55E | {{ui:swatch:success/}} |
| warning | EAB308 | {{ui:swatch:warning/}} |
| error | EF4444 | {{ui:swatch:error/}} |
| info | 3B82F6 | {{ui:swatch:info/}} |
| cobalt | 2B6CB0 | {{ui:swatch:cobalt/}} |
| plum | 6B46C1 | {{ui:swatch:plum/}} |
| slate | 6B7280 | {{ui:swatch:slate/}} |

---

## Rendering Backends

| Backend | Output | Best For |
|---------|--------|----------|
| SVG | Local `.svg` files | Offline docs, full features |
| Shields.io | Fallback badge showing range | GitHub README |
| Plaintext | Unicode blocks ▁▂▃▄▅▆▇█ | Terminal, ASCII-only |

**SVG rendering (recommended):**
```bash
mdfx process input.md -o output.md --backend svg --assets-dir assets/
```

**Shields.io fallback:**
```bash
mdfx process input.md -o output.md  # Default
```
