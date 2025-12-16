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

![](../../assets/mdfx/sparkline_9217789088470abf.svg)

![](../../assets/mdfx/sparkline_f9a1450e73f9a885.svg)

![](../../assets/mdfx/sparkline_c9c8656b7d8e1fe0.svg)

### Bar Chart

Vertical bars for each data point.

![](../../assets/mdfx/sparkline_d9398b9b2dc12379.svg)

![](../../assets/mdfx/sparkline_919f0b1f4ec496be.svg)

![](../../assets/mdfx/sparkline_d383fb0a4bbb055a.svg)

### Area Chart

Filled area under the line for emphasis.

![](../../assets/mdfx/sparkline_f50e566c927807b3.svg)

![](../../assets/mdfx/sparkline_47bc95df1edf87db.svg)

![](../../assets/mdfx/sparkline_b5f82f70e08a5191.svg)

---

## Styling Options

### With Dots

Add dots at each data point (line chart only).

![](../../assets/mdfx/sparkline_6a6d01f4f0b32c6c.svg)

![](../../assets/mdfx/sparkline_54dd2b1a4e0310ae.svg)

![](../../assets/mdfx/sparkline_4e65cb6c027328a.svg)

### Custom Sizes

Control width and height.

**Small (60x15):**
![](../../assets/mdfx/sparkline_19a0ecf4a31f98a6.svg)

**Medium (100x20) - Default:**
![](../../assets/mdfx/sparkline_9217789088470abf.svg)

**Large (150x30):**
![](../../assets/mdfx/sparkline_f6631ce52b023f82.svg)

**Wide (200x25):**
![](../../assets/mdfx/sparkline_f85a11596ff55cdf.svg)

**Tall (80x40):**
![](../../assets/mdfx/sparkline_aaf1753030eb36cd.svg)

### Custom Colors

Use named palette colors or hex values.

**Accent (default):**
![](../../assets/mdfx/sparkline_9217789088470abf.svg)

**Success (green):**
![](../../assets/mdfx/sparkline_678792dd316262af.svg)

**Warning (yellow):**
![](../../assets/mdfx/sparkline_f8750fb9d8dc4f73.svg)

**Error (red):**
![](../../assets/mdfx/sparkline_de0f1027cfb55333.svg)

**Info (blue):**
![](../../assets/mdfx/sparkline_45710a3206d71e22.svg)

**Cobalt:**
![](../../assets/mdfx/sparkline_84495ca027b2072.svg)

**Plum:**
![](../../assets/mdfx/sparkline_edcf8b8666d5779d.svg)

**Custom Hex:**
![](../../assets/mdfx/sparkline_ae9c9bab2cfc9ef9.svg)

### Stroke Customization

Control line width and separate stroke color.

**Thin stroke (1px):**
![](../../assets/mdfx/sparkline_c61bf008a67283eb.svg)

**Default stroke (2px):**
![](../../assets/mdfx/sparkline_9217789088470abf.svg)

**Thick stroke (3px):**
![](../../assets/mdfx/sparkline_c11d3c61afe9a350.svg)

**Custom stroke color:**
![](../../assets/mdfx/sparkline_3c5a79de965b9eef.svg)

### Track Color (Background)

Add a background track behind the chart.

![](../../assets/mdfx/sparkline_cb213ca62a14ef5f.svg)

![](../../assets/mdfx/sparkline_74fb225da257658c.svg)

![](../../assets/mdfx/sparkline_755dd64e95489938.svg)

---

## Data Patterns

### Trending Up

![](../../assets/mdfx/sparkline_5c34e67a9235ac21.svg)

![](../../assets/mdfx/sparkline_c3df1ef7e55b9f4b.svg)

![](../../assets/mdfx/sparkline_4ef4874273d0793d.svg)

### Trending Down

![](../../assets/mdfx/sparkline_96fb7ccc523b5021.svg)

![](../../assets/mdfx/sparkline_eabd7256e34df7c3.svg)

![](../../assets/mdfx/sparkline_91e4e71632667e13.svg)

### Volatile/Fluctuating

![](../../assets/mdfx/sparkline_fa09ab2db26472e1.svg)

![](../../assets/mdfx/sparkline_4ca65ee024b77fbf.svg)

![](../../assets/mdfx/sparkline_bfcc7c3ea966df20.svg)

### Flat/Stable

![](../../assets/mdfx/sparkline_c68314fa43996ecb.svg)

![](../../assets/mdfx/sparkline_5c9bc3f7a9afe601.svg)

### Peak in Middle

![](../../assets/mdfx/sparkline_9dea700925b112e9.svg)

![](../../assets/mdfx/sparkline_5d65cfb4a4c26622.svg)

![](../../assets/mdfx/sparkline_a244716032b4232d.svg)

### Valley in Middle

![](../../assets/mdfx/sparkline_af3967fdc0a73b54.svg)

![](../../assets/mdfx/sparkline_8591cf42b6aa0e2e.svg)

![](../../assets/mdfx/sparkline_adf1f4c5af0631c.svg)

---

## Combined Options

### Large Line with Dots

![](../../assets/mdfx/sparkline_1c0f6d6b2ee5ee47.svg)

### Bar Chart with Track

![](../../assets/mdfx/sparkline_1ff714353ccac707.svg)

### Area Chart Custom Style

![](../../assets/mdfx/sparkline_52cb45a32b8ab15.svg)

### Compact Inline Indicators

Uptrend: ![](../../assets/mdfx/sparkline_56d0c185680c0cc2.svg)
Downtrend: ![](../../assets/mdfx/sparkline_e955783d48d6ab57.svg)
Stable: ![](../../assets/mdfx/sparkline_caf01f5a23d3c529.svg)

---

## Use Cases

### Dashboard Metrics

**CPU Usage:** ![](../../assets/mdfx/sparkline_7b0740b7130bb3ec.svg)

**Memory:** ![](../../assets/mdfx/sparkline_c61701fbb6de69f2.svg)

**Disk I/O:** ![](../../assets/mdfx/sparkline_cfce9d1185b5967e.svg)

### Stock/Financial Data

**Stock A:** ![](../../assets/mdfx/sparkline_9e842d06bbac0291.svg) +30%

**Stock B:** ![](../../assets/mdfx/sparkline_9be84eb9f9310f28.svg) -25%

**Stock C:** ![](../../assets/mdfx/sparkline_54d8ee934c88cbd2.svg) 0%

### Activity Timeline

**Weekly commits:** ![](../../assets/mdfx/sparkline_b320b38c769cab14.svg)

**Daily visitors:** ![](../../assets/mdfx/sparkline_1eadcd8f672f7d55.svg)

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
| accent | F41C80 | ![](../../assets/mdfx/swatch_8010e28a060480ec.svg) |
| success | 22C55E | ![](../../assets/mdfx/swatch_9548868f54f0a66e.svg) |
| warning | EAB308 | ![](../../assets/mdfx/swatch_e4795ff410c7b4fe.svg) |
| error | EF4444 | ![](../../assets/mdfx/swatch_e666c671e27adcb2.svg) |
| info | 3B82F6 | ![](../../assets/mdfx/swatch_b4740ff4b229ace7.svg) |
| cobalt | 2B6CB0 | ![](../../assets/mdfx/swatch_518ded146f6f965a.svg) |
| plum | 6B46C1 | ![](../../assets/mdfx/swatch_c056f66b5750e2ba.svg) |
| slate | 6B7280 | ![](../../assets/mdfx/swatch_5ae9a07e7148661a.svg) |

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
