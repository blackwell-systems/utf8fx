# Progress Bar Complete Guide

The progress bar component is a versatile horizontal bar for displaying percentages, skill levels, loading states, and interactive sliders. This guide covers every parameter and configuration option.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [All Parameters](#all-parameters)
- [Dimensions](#dimensions)
- [Colors](#colors)
- [Corner Radius](#corner-radius)
- [Floating Fill Effect](#floating-fill-effect)
- [Labels](#labels)
- [Borders](#borders)
- [Slider Mode](#slider-mode)
- [Thumb Shapes](#thumb-shapes)
- [Thumb Width (Oval/Pill)](#thumb-width-ovalpill)
- [Thumb Borders](#thumb-borders)
- [Complete Examples](#complete-examples)
- [Tips & Tricks](#tips--tricks)

---

## Basic Syntax

```markdown
{{ui:progress:PERCENT/}}
```

Where `PERCENT` is a number from 0 to 100.

| Syntax | Result |
|--------|--------|
| `{{ui:progress:0/}}` | ![](assets/progress-guide/progress_3737363d8c3b5daf.svg) |
| `{{ui:progress:25/}}` | ![](assets/progress-guide/progress_7bba39a365d30137.svg) |
| `{{ui:progress:50/}}` | ![](assets/progress-guide/progress_fcca95c901c86487.svg) |
| `{{ui:progress:75/}}` | ![](assets/progress-guide/progress_71859eed66c84737.svg) |
| `{{ui:progress:100/}}` | ![](assets/progress-guide/progress_e8f548aa2f9574de.svg) |

---

## All Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | *required* | Progress percentage (0-100) |
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
| `thumb_width` | number | thumb | Thumb width (for oval/pill shapes) |
| `thumb_color` | color | fill | Thumb color |
| `thumb_shape` | string | circle | Thumb shape: circle, square, diamond |
| `thumb_border` | color | none | Thumb border/stroke color |
| `thumb_border_width` | number | 0 | Thumb border width in pixels |

---

## Dimensions

### Width Variations

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=80/}}` | ![](assets/progress-guide/progress_436af944a59459f0.svg) |
| `{{ui:progress:50:width=120/}}` | ![](assets/progress-guide/progress_8bd3f2eda9fc20a0.svg) |
| `{{ui:progress:50:width=200/}}` | ![](assets/progress-guide/progress_18187201b45f7ac6.svg) |
| `{{ui:progress:50:width=300/}}` | ![](assets/progress-guide/progress_e380c3ecebf9a9cb.svg) |

### Height Variations

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=150:height=4/}}` | ![](assets/progress-guide/progress_e7eff1654b9aed4c.svg) |
| `{{ui:progress:60:width=150:height=8/}}` | ![](assets/progress-guide/progress_88f23662285a9b34.svg) |
| `{{ui:progress:60:width=150:height=16/}}` | ![](assets/progress-guide/progress_3be42669441fee5b.svg) |
| `{{ui:progress:60:width=150:height=24/}}` | ![](assets/progress-guide/progress_2f5cf2504b631511.svg) |

---

## Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:progress:70:width=150:fill=accent/}}` | ![](assets/progress-guide/progress_1f12169f5390a7e3.svg) |
| `{{ui:progress:70:width=150:fill=success/}}` | ![](assets/progress-guide/progress_a935ac416f831853.svg) |
| `{{ui:progress:70:width=150:fill=warning/}}` | ![](assets/progress-guide/progress_77c1375a5b0b85f2.svg) |
| `{{ui:progress:70:width=150:fill=error/}}` | ![](assets/progress-guide/progress_2ae5a600424b6cf8.svg) |
| `{{ui:progress:70:width=150:fill=info/}}` | ![](assets/progress-guide/progress_d1270765a640c6cb.svg) |
| `{{ui:progress:70:width=150:fill=cobalt/}}` | ![](assets/progress-guide/progress_909aea20f64ba41c.svg) |

### Track Colors

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=150:track=slate/}}` | ![](assets/progress-guide/progress_7b00dd2f6651618e.svg) |
| `{{ui:progress:60:width=150:track=ink/}}` | ![](assets/progress-guide/progress_bebd6de0469c9292.svg) |
| `{{ui:progress:60:width=150:track=ui.panel/}}` | ![](assets/progress-guide/progress_80eb4d92cf6dff81.svg) |
| `{{ui:progress:60:width=150:track=333333/}}` | ![](assets/progress-guide/progress_628c34fb38bc6c35.svg) |

### Custom Hex Colors

```markdown
{{ui:progress:75:width=200:fill=FF6B35:track=1a1a2e/}}
{{ui:progress:60:width=200:fill=00FF41:track=0D0D0D/}}
{{ui:progress:85:width=200:fill=FF00FF:track=1a0a1a/}}
```

![](assets/progress-guide/progress_cc8f0cb630b256be.svg)
![](assets/progress-guide/progress_a4c4467e0391369e.svg)
![](assets/progress-guide/progress_3c880475d6523167.svg)

---

## Corner Radius

Control corner roundness with the `rx` parameter:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:65:width=150:height=12:rx=0/}}` | ![](assets/progress-guide/progress_512d04f29cf91b73.svg) |
| `{{ui:progress:65:width=150:height=12:rx=2/}}` | ![](assets/progress-guide/progress_4ccfe5984ac92120.svg) |
| `{{ui:progress:65:width=150:height=12:rx=4/}}` | ![](assets/progress-guide/progress_9a8ad2fa0539c3ca.svg) |
| `{{ui:progress:65:width=150:height=12:rx=6/}}` | ![](assets/progress-guide/progress_24b40391d201d75e.svg) |

### Pill Shape

Set `rx` to half of `height` for perfect pill shape:

```markdown
{{ui:progress:70:width=200:height=16:rx=8/}}
```

![](assets/progress-guide/progress_fd3c8e66bb8f7645.svg)

---

## Floating Fill Effect

When `fill_height` is less than `height`, the fill "floats" inside the track:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=180:height=16:fill_height=16/}}` | ![](assets/progress-guide/progress_35a79674ad3a61b3.svg) |
| `{{ui:progress:60:width=180:height=16:fill_height=12/}}` | ![](assets/progress-guide/progress_39f6f320facf34de.svg) |
| `{{ui:progress:60:width=180:height=16:fill_height=8/}}` | ![](assets/progress-guide/progress_3b30f6780398ae79.svg) |
| `{{ui:progress:60:width=180:height=16:fill_height=4/}}` | ![](assets/progress-guide/progress_2ae6648bbc045fdf.svg) |

### Elegant Thin Fill

```markdown
{{ui:progress:80:width=250:height=20:fill_height=6:fill=accent:track=ui.panel/}}
```

![](assets/progress-guide/progress_162333db8bb9fe3.svg)

---

## Labels

Show percentage text with `label=true`:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:25:width=150:height=18:label=true/}}` | ![](assets/progress-guide/progress_12defa73979cee3f.svg) |
| `{{ui:progress:50:width=150:height=18:label=true/}}` | ![](assets/progress-guide/progress_99b6ab4c33124ae8.svg) |
| `{{ui:progress:75:width=150:height=18:label=true/}}` | ![](assets/progress-guide/progress_53bec15bcfefb94c.svg) |
| `{{ui:progress:100:width=150:height=18:label=true/}}` | ![](assets/progress-guide/progress_d36090babf845fc2.svg) |

### Custom Label Colors

```markdown
{{ui:progress:65:width=180:height=20:label=true:label_color=000000:fill=warning/}}
```

![](assets/progress-guide/progress_da66312bd8579b2f.svg)

---

## Borders

Add borders with `border` and `border_width`:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:60:width=150:height=12:border=accent/}}` | ![](assets/progress-guide/progress_4991308a2678099d.svg) |
| `{{ui:progress:60:width=150:height=12:border=success/}}` | ![](assets/progress-guide/progress_ec515a0138225cd8.svg) |
| `{{ui:progress:60:width=150:height=12:border=FFFFFF/}}` | ![](assets/progress-guide/progress_813d642d1d8908b6.svg) |

### Border Widths

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=150:height=14:border=accent:border_width=1/}}` | ![](assets/progress-guide/progress_9aa0bf5bd52e2556.svg) |
| `{{ui:progress:50:width=150:height=14:border=accent:border_width=2/}}` | ![](assets/progress-guide/progress_1d199bea15188e1b.svg) |
| `{{ui:progress:50:width=150:height=14:border=accent:border_width=3/}}` | ![](assets/progress-guide/progress_f5a1ca2c95b56e9d.svg) |

---

## Slider Mode

Add a `thumb` parameter to enable slider mode with a draggable-style thumb indicator:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:30:width=180:height=6:thumb=14/}}` | ![](assets/progress-guide/progress_6987db8594b603d.svg) |
| `{{ui:progress:50:width=180:height=6:thumb=14/}}` | ![](assets/progress-guide/progress_8da0499ff71d4a64.svg) |
| `{{ui:progress:70:width=180:height=6:thumb=14/}}` | ![](assets/progress-guide/progress_41163926e122300f.svg) |
| `{{ui:progress:90:width=180:height=6:thumb=14/}}` | ![](assets/progress-guide/progress_fbbb1b1b92c4e15d.svg) |

### Thumb Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:height=6:thumb=10/}}` | ![](assets/progress-guide/progress_c4024d059aa13885.svg) |
| `{{ui:progress:50:width=180:height=6:thumb=14/}}` | ![](assets/progress-guide/progress_8da0499ff71d4a64.svg) |
| `{{ui:progress:50:width=180:height=6:thumb=18/}}` | ![](assets/progress-guide/progress_997ac1b6e930dbdf.svg) |
| `{{ui:progress:50:width=180:height=6:thumb=22/}}` | ![](assets/progress-guide/progress_bb660f512b017bf6.svg) |

### Custom Thumb Colors

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=14:thumb_color=accent/}}` | ![](assets/progress-guide/progress_1350575f34e9156b.svg) |
| `{{ui:progress:50:width=180:thumb=14:thumb_color=success/}}` | ![](assets/progress-guide/progress_9cbe0a144e6f715f.svg) |
| `{{ui:progress:50:width=180:thumb=14:thumb_color=warning/}}` | ![](assets/progress-guide/progress_e932bb08a429b148.svg) |
| `{{ui:progress:50:width=180:thumb=14:thumb_color=white/}}` | ![](assets/progress-guide/progress_2240120ce94fbe26.svg) |

---

## Thumb Shapes

Choose between circle, square, or diamond thumbs:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=14:thumb_shape=circle/}}` | ![](assets/progress-guide/progress_782fe7c7afa694a.svg) |
| `{{ui:progress:50:width=180:thumb=14:thumb_shape=square/}}` | ![](assets/progress-guide/progress_4a6e6a78c7d0a031.svg) |
| `{{ui:progress:50:width=180:thumb=14:thumb_shape=diamond/}}` | ![](assets/progress-guide/progress_f6360a59eca8c585.svg) |

### Colored Shape Variations

```markdown
{{ui:progress:65:width=200:thumb=16:thumb_shape=diamond:thumb_color=warning/}}
{{ui:progress:45:width=200:thumb=16:thumb_shape=square:thumb_color=info/}}
```

![](assets/progress-guide/progress_113d4b7249b47eb5.svg)
![](assets/progress-guide/progress_6387372db986c169.svg)

---

## Thumb Width (Oval/Pill)

Use `thumb_width` to create oval or pill-shaped thumbs:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=12:thumb_width=12/}}` | ![](assets/progress-guide/progress_c3052968c0eaf3fa.svg) |
| `{{ui:progress:50:width=180:thumb=12:thumb_width=18/}}` | ![](assets/progress-guide/progress_7c37ff40c227e5f6.svg) |
| `{{ui:progress:50:width=180:thumb=12:thumb_width=24/}}` | ![](assets/progress-guide/progress_92a4d2a724876b64.svg) |
| `{{ui:progress:50:width=180:thumb=12:thumb_width=32/}}` | ![](assets/progress-guide/progress_5ff07b5214d06453.svg) |

### Tall Oval (Vertical)

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=20:thumb_width=10/}}` | ![](assets/progress-guide/progress_8e5e927aee8e47a7.svg) |
| `{{ui:progress:50:width=180:thumb=24:thumb_width=12/}}` | ![](assets/progress-guide/progress_cce5a8306c053d31.svg) |

### iOS-Style Wide Pill

```markdown
{{ui:progress:60:width=220:height=8:track=ui.panel:fill=info:thumb=18:thumb_width=30:thumb_color=white/}}
```

![](assets/progress-guide/progress_bd2676fa2ee98f19.svg)

### Wide Square (Rounded Rectangle)

```markdown
{{ui:progress:45:width=200:thumb=14:thumb_width=24:thumb_shape=square:thumb_color=accent/}}
```

![](assets/progress-guide/progress_a578588a54e89115.svg)

---

## Thumb Borders

Add a stroke border around thumbs with `thumb_border` and `thumb_border_width`:

| Syntax | Result |
|--------|--------|
| `{{ui:progress:50:width=180:thumb=16:thumb_border=white:thumb_border_width=2/}}` | Thumb with white border |
| `{{ui:progress:50:width=180:thumb=16:thumb_border=000000:thumb_border_width=1/}}` | Thumb with thin black border |

### Bordered Diamond

```markdown
{{ui:progress:65:width=200:thumb=16:thumb_shape=diamond:thumb_border=FFFFFF:thumb_border_width=2/}}
```

Borders help thumbs stand out on tracks with similar colors.

---

## Complete Examples

### Skill Bars

```markdown
| Skill | Level |
|-------|-------|
| Rust | {{ui:progress:95:width=120:fill=success/}} |
| Python | {{ui:progress:85:width=120:fill=info/}} |
| TypeScript | {{ui:progress:80:width=120:fill=cobalt/}} |
| Go | {{ui:progress:70:width=120:fill=warning/}} |
```

| Skill | Level |
|-------|-------|
| Rust | ![](assets/progress-guide/progress_478cef77093fa246.svg) |
| Python | ![](assets/progress-guide/progress_a0fe5e9b03b4df74.svg) |
| TypeScript | ![](assets/progress-guide/progress_191cf5d62d5b4e8c.svg) |
| Go | ![](assets/progress-guide/progress_efaf459128b467c1.svg) |

### Volume Sliders

```markdown
{{ui:progress:30:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
{{ui:progress:60:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
{{ui:progress:90:width=150:height=4:thumb=12:thumb_color=white:track=ink/}}
```

![](assets/progress-guide/progress_6a1ae399a8fb6576.svg)
![](assets/progress-guide/progress_4071bb24159cec96.svg)
![](assets/progress-guide/progress_f425b8fca73d7025.svg)

### Loading States

```markdown
{{ui:progress:10:width=200:height=6:fill=info/}}
{{ui:progress:45:width=200:height=6:fill=info/}}
{{ui:progress:80:width=200:height=6:fill=info/}}
{{ui:progress:100:width=200:height=6:fill=success/}}
```

![](assets/progress-guide/progress_b3467ee9258ebfb5.svg)
![](assets/progress-guide/progress_6c08e11e55c0ee40.svg)
![](assets/progress-guide/progress_8062b59d8a2901de.svg)
![](assets/progress-guide/progress_ae747c949ffde275.svg)

### Neon Style

```markdown
{{ui:progress:75:width=220:height=8:fill=00FF41:track=0D0D0D/}}
{{ui:progress:60:width=220:height=8:fill=FF00FF:track=0D0D0D/}}
{{ui:progress:85:width=220:height=8:fill=00FFFF:track=0D0D0D/}}
```

![](assets/progress-guide/progress_217d45f069d706d7.svg)
![](assets/progress-guide/progress_74d4f82841a7d94.svg)
![](assets/progress-guide/progress_cfe895c180bc78d.svg)

### Music Player Seek Bar

```markdown
{{ui:progress:35:width=280:height=4:track=slate:fill=success:thumb=14:thumb_width=22/}}
```

![](assets/progress-guide/progress_d15fa28e78d61b91.svg)

### System Resource Meters

| Resource | Usage |
|----------|-------|
| CPU | ![](assets/progress-guide/progress_de7581d46dd39be7.svg) |
| Memory | ![](assets/progress-guide/progress_cf80636c9823277b.svg) |
| Disk | ![](assets/progress-guide/progress_548b3420e7993c08.svg) |

---

## Tips & Tricks

### 1. Minimal Track Slider

```markdown
{{ui:progress:50:width=200:height=2:thumb=16:thumb_color=accent/}}
```

![](assets/progress-guide/progress_b9d11549966f6387.svg)

### 2. Thick Track with Small Thumb

```markdown
{{ui:progress:65:width=200:height=16:thumb=12:thumb_color=white/}}
```

![](assets/progress-guide/progress_e3abefa3739d261d.svg)

### 3. Color-Coded Status

```markdown
{{ui:progress:25:width=150:fill=error/}}
{{ui:progress:55:width=150:fill=warning/}}
{{ui:progress:85:width=150:fill=success/}}
```

![](assets/progress-guide/progress_d9777242ad99f54d.svg)
![](assets/progress-guide/progress_dbaf29e33cdcef84.svg)
![](assets/progress-guide/progress_68c224e3a3a84434.svg)

### 4. Contrast Border Effect

```markdown
{{ui:progress:70:width=200:height=14:fill=accent:border=white:border_width=2/}}
```

![](assets/progress-guide/progress_e9f94e0dca09a2a1.svg)

---

## See Also

- [Donut & Gauge Guide](DONUT-GAUGE-GUIDE.md)
- [Swatch Guide](SWATCH-GUIDE.md)
- [Components Reference](../COMPONENTS.md)
- [CLI Guide](CLI-GUIDE.md)
