# Waveform Component Guide

Audio-style waveform visualizations for displaying oscillating or signed data.

## Quick Start

```markdown
{{ui:waveform:0.2,0.5,0.8,0.6,0.3,-0.2,-0.5,-0.7,-0.4,-0.1,0.3,0.6/}}
```

Renders vertical bars above and below a center line, perfect for audio waveforms, signal data, or any oscillating values.

---

## Basic Usage

### Simple Waveform

Values between -1 and 1 work best, but any range is supported.

```markdown
{{ui:waveform:0.3,0.7,0.5,0.9,0.4,-0.2,-0.6,-0.8,-0.5,-0.3,0.1,0.4,0.7,0.5/}}
```

### Random Audio Pattern

```markdown
{{ui:waveform:0.2,0.4,0.6,0.8,0.5,0.3,-0.1,-0.4,-0.7,-0.5,-0.2,0.1,0.3,0.5,0.7,0.4,0.2,-0.1,-0.3,-0.6,-0.4,-0.2,0.1,0.3/}}
```

---

## Styling Options

### Custom Colors

Control positive (above center) and negative (below center) bar colors.

```markdown
{{ui:waveform:...:positive=neon:negative=purple/}}
{{ui:waveform:...:up=cyan:down=magenta/}}
```

### Monochrome Style

Use the same color for both directions.

```markdown
{{ui:waveform:...:positive=accent:negative=accent/}}
```

### Custom Dimensions

```markdown
{{ui:waveform:...:width=150:height=50/}}
{{ui:waveform:...:width=200:height=30/}}
```

### Bar Width and Spacing

Control the thickness and gap between bars.

```markdown
{{ui:waveform:...:bar_width=2:spacing=1/}}
{{ui:waveform:...:bar=5:spacing=2/}}
```

### Center Line

Show a horizontal center line for reference.

```markdown
{{ui:waveform:...:center=true/}}
{{ui:waveform:...:center=true:center_color=white/}}
```

### Track Background

Add a background track color.

```markdown
{{ui:waveform:...:track=dark2/}}
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

```markdown
{{ui:waveform:0.1,0.3,0.5,0.7,0.9,0.7,0.5,0.3,0.1,-0.1,-0.3,-0.5,-0.7,-0.9,-0.7,-0.5,-0.3,-0.1:positive=neon:negative=neon:width=200:height=40/}}
```

### Signal Processing

Show signal data with distinct positive/negative coloring.

```markdown
{{ui:waveform:1,0.5,0,-0.5,-1,-0.5,0,0.5,1,0.5,0,-0.5,-1:positive=cyan:negative=magenta:center=true/}}
```

### Stock/Financial Data

Display price movements with gains (positive) and losses (negative).

```markdown
{{ui:waveform:0.02,0.05,-0.03,0.08,-0.02,-0.06,0.04,0.07,-0.01,-0.04,0.03:positive=success:negative=error:width=120/}}
```

### Sentiment Analysis

Visualize positive/negative sentiment scores.

```markdown
{{ui:waveform:0.8,0.6,0.2,-0.3,-0.5,0.1,0.4,0.7,-0.2,-0.4,0.3:positive=info:negative=warning/}}
```

---

## Tips

1. **Normalize your data** - Values between -1 and 1 give the best visual results
2. **Match bar count to width** - For smooth appearance, ensure `(bar_width + spacing) * bar_count ≈ width`
3. **Use monochrome for subtle effect** - Same color for positive/negative creates a classic audio waveform look
4. **Add center line for reference** - Helps viewers understand the zero crossing point
