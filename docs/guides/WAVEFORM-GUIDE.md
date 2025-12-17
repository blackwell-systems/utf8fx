# Waveform Component Guide

Audio-style waveform visualizations for displaying oscillating or signed data.

## Quick Start

```markdown
{{ui:waveform:0.2,0.5,0.8,0.6,0.3,-0.2,-0.5,-0.7,-0.4,-0.1,0.3,0.6/}}
```

Renders vertical bars above and below a center line, perfect for audio waveforms, signal data, or any oscillating values.

---

## Examples

### Basic Waveform

![](../../assets/mdfx/waveform_56ab4d5217fc0559.svg)

```markdown
{{ui:waveform:0.3,0.7,0.5,0.9,0.4,-0.2,-0.6,-0.8,-0.5,-0.3,0.1,0.4,0.7,0.5:width=200:height=40/}}
```

### Audio Pattern

![](../../assets/mdfx/waveform_13c4c9c1aa662dfc.svg)

```markdown
{{ui:waveform:0.2,0.4,0.6,0.8,0.5,0.3,-0.1,-0.4,-0.7,-0.5,-0.2,0.1,0.3,0.5,0.7,0.4,0.2,-0.1,-0.3,-0.6,-0.4,-0.2,0.1,0.3:width=200:height=40/}}
```

### Monochrome (Accent)

![](../../assets/mdfx/waveform_7d21d7d64a5780d4.svg)

```markdown
{{ui:waveform:...:positive=accent:negative=accent:bar=2:spacing=1/}}
```

---

## Styling Options

### Custom Colors

Control positive (above center) and negative (below center) bar colors.

**Cyan/Magenta:**

![](../../assets/mdfx/waveform_a629597cb1bbd719.svg)

```markdown
{{ui:waveform:...:positive=cyan:negative=magenta/}}
```

**Neon Style:**

![](../../assets/mdfx/waveform_6c32fd425342bef5.svg)

```markdown
{{ui:waveform:...:positive=neon:negative=neon/}}
```

**Success/Error:**

![](../../assets/mdfx/waveform_23c4307026a2691e.svg)

```markdown
{{ui:waveform:...:positive=success:negative=error/}}
```

### Bar Width and Spacing

**Wide Bars:**

![](../../assets/mdfx/waveform_aa9d36d1a5dfd6ac.svg)

```markdown
{{ui:waveform:...:bar=8:spacing=4/}}
```

**Thin Bars:**

![](../../assets/mdfx/waveform_28be028c8851135f.svg)

```markdown
{{ui:waveform:...:bar=2:spacing=1/}}
```

---

## Parameter Reference

| Parameter | Aliases | Default | Description |
|-----------|---------|---------|-------------|
| `width` | - | `100` | Total width in pixels |
| `height` | - | `40` | Total height in pixels |
| `positive` | `up` | `success` | Color for values > 0 |
| `negative` | `down` | `error` | Color for values < 0 |
| `bar_width` | `bar` | `3` | Width of each bar |
| `spacing` | - | `1` | Gap between bars |
| `track` | - | none | Background track color |
| `center` | - | `false` | Show center line |
| `center_color` | - | `white` | Center line color |

---

## Use Cases

### Audio Visualization

Perfect for showing audio waveforms in music or podcast READMEs.

### Signal Processing

Show signal data with distinct positive/negative coloring.

### Stock/Financial Data

Display price movements with gains (positive) and losses (negative).

### Sentiment Analysis

Visualize positive/negative sentiment scores.

---

## Tips

1. **Normalize your data** - Values between -1 and 1 give the best visual results
2. **Match bar count to width** - For smooth appearance, ensure `(bar_width + spacing) * bar_count ≈ width`
3. **Use monochrome for subtle effect** - Same color for positive/negative creates a classic audio waveform look
4. **Add center line for reference** - Helps viewers understand the zero crossing point
