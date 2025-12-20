# Donut & Gauge Complete Guide

Donut and gauge components are circular visualizations perfect for dashboards, completion indicators, and interactive controls. This guide covers every parameter and configuration option for both.

## Table of Contents

- [Donut Component](#donut-component)
  - [Basic Donut Syntax](#basic-donut-syntax)
  - [Donut Parameters](#donut-parameters)
  - [Donut Sizes](#donut-sizes)
  - [Donut Thickness](#donut-thickness)
  - [Donut Colors](#donut-colors)
  - [Donut Labels](#donut-labels)
  - [Donut Slider Mode](#donut-slider-mode)
- [Gauge Component](#gauge-component)
  - [Basic Gauge Syntax](#basic-gauge-syntax)
  - [Gauge Parameters](#gauge-parameters)
  - [Gauge Sizes](#gauge-sizes)
  - [Gauge Thickness](#gauge-thickness)
  - [Gauge Colors](#gauge-colors)
  - [Gauge Labels](#gauge-labels)
  - [Gauge Slider Mode](#gauge-slider-mode)
- [Thumb Customization](#thumb-customization)
- [Complete Examples](#complete-examples)

---

# Donut Component

A circular progress ring showing completion percentage.

## Basic Donut Syntax

```markdown
{{ui:donut:PERCENT/}}
```

Where `PERCENT` is a number from 0 to 100.

| Syntax | Result |
|--------|--------|
| `{{ui:donut:0/}}` | ![](assets/donut-gauge-guide/donut_60bfe4b2791e86a.svg) |
| `{{ui:donut:25/}}` | ![](assets/donut-gauge-guide/donut_4976f08f2b723755.svg) |
| `{{ui:donut:50/}}` | ![](assets/donut-gauge-guide/donut_67d4d80b0c5ba2a0.svg) |
| `{{ui:donut:75/}}` | ![](assets/donut-gauge-guide/donut_137d381a14227ab4.svg) |
| `{{ui:donut:100/}}` | ![](assets/donut-gauge-guide/donut_2c8347511457fcd7.svg) |

---

## Donut Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | *required* | Progress percentage (0-100) |
| `size` | number | 40 | Diameter in pixels |
| `thickness` | number | 4 | Ring thickness in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `label` | boolean | false | Show percentage label in center |
| `label_color` | color | white | Label text color |
| `thumb` | number | none | Thumb size (enables slider mode) |
| `thumb_color` | color | fill | Thumb color |
| `thumb_border` | color | none | Thumb border/stroke color |
| `thumb_border_width` | number | 0 | Thumb border width in pixels |

---

## Donut Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:donut:65:size=24/}}` | ![](assets/donut-gauge-guide/donut_6e62977a0e39d5e2.svg) |
| `{{ui:donut:65:size=40/}}` | ![](assets/donut-gauge-guide/donut_abdd81dc90ecc4a7.svg) |
| `{{ui:donut:65:size=60/}}` | ![](assets/donut-gauge-guide/donut_eca20efe9a9f4877.svg) |
| `{{ui:donut:65:size=80/}}` | ![](assets/donut-gauge-guide/donut_ee55f6b3f54e0f02.svg) |
| `{{ui:donut:65:size=100/}}` | ![](assets/donut-gauge-guide/donut_7a21749b944ebaf3.svg) |

---

## Donut Thickness

| Syntax | Result |
|--------|--------|
| `{{ui:donut:70:size=60:thickness=2/}}` | ![](assets/donut-gauge-guide/donut_da8d2f83c44b5fcc.svg) |
| `{{ui:donut:70:size=60:thickness=4/}}` | ![](assets/donut-gauge-guide/donut_fd31c7d7bce5807d.svg) |
| `{{ui:donut:70:size=60:thickness=8/}}` | ![](assets/donut-gauge-guide/donut_6a0183dae964224b.svg) |
| `{{ui:donut:70:size=60:thickness=12/}}` | ![](assets/donut-gauge-guide/donut_66ee6f3c2ba6d44.svg) |
| `{{ui:donut:70:size=60:thickness=16/}}` | ![](assets/donut-gauge-guide/donut_d8189022cccda866.svg) |

### Thin Elegant Ring

```markdown
{{ui:donut:80:size=80:thickness=2:fill=accent/}}
```

![](assets/donut-gauge-guide/donut_b517c279e55bbd21.svg)

### Thick Chunky Ring

```markdown
{{ui:donut:60:size=80:thickness=18:fill=info/}}
```

![](assets/donut-gauge-guide/donut_800415b31fece2d4.svg)

---

## Donut Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:donut:70:size=50:fill=accent/}}` | ![](assets/donut-gauge-guide/donut_6c409192df33d447.svg) |
| `{{ui:donut:70:size=50:fill=success/}}` | ![](assets/donut-gauge-guide/donut_154fc9124ddf966c.svg) |
| `{{ui:donut:70:size=50:fill=warning/}}` | ![](assets/donut-gauge-guide/donut_9e916d04cbb8a988.svg) |
| `{{ui:donut:70:size=50:fill=error/}}` | ![](assets/donut-gauge-guide/donut_c7293300a97fe711.svg) |
| `{{ui:donut:70:size=50:fill=info/}}` | ![](assets/donut-gauge-guide/donut_ecb45e75af17c45d.svg) |
| `{{ui:donut:70:size=50:fill=cobalt/}}` | ![](assets/donut-gauge-guide/donut_b9562fd89c8144b7.svg) |

### Track Colors

| Syntax | Result |
|--------|--------|
| `{{ui:donut:60:size=50:track=slate/}}` | ![](assets/donut-gauge-guide/donut_f36d138985189c7f.svg) |
| `{{ui:donut:60:size=50:track=ink/}}` | ![](assets/donut-gauge-guide/donut_58a92dea91e79a05.svg) |
| `{{ui:donut:60:size=50:track=ui.panel/}}` | ![](assets/donut-gauge-guide/donut_9ce51a45bdc07553.svg) |

### Custom Hex Colors

```markdown
{{ui:donut:75:size=60:fill=FF6B35:track=1a1a2e/}}
{{ui:donut:60:size=60:fill=00FF41:track=0D0D0D/}}
{{ui:donut:85:size=60:fill=FF00FF:track=1a0a1a/}}
```

![](assets/donut-gauge-guide/donut_abe5ef530f32959.svg) ![](assets/donut-gauge-guide/donut_5e8603b7101a69a3.svg) ![](assets/donut-gauge-guide/donut_8b6067adf41a1691.svg)

---

## Donut Labels

Show percentage in center with `label=true`:

| Syntax | Result |
|--------|--------|
| `{{ui:donut:25:size=60:label=true/}}` | ![](assets/donut-gauge-guide/donut_21bdbcb2e9d57d71.svg) |
| `{{ui:donut:50:size=60:label=true/}}` | ![](assets/donut-gauge-guide/donut_624e3b4ac5e074cf.svg) |
| `{{ui:donut:75:size=60:label=true/}}` | ![](assets/donut-gauge-guide/donut_d22031e548db30b.svg) |
| `{{ui:donut:100:size=60:label=true/}}` | ![](assets/donut-gauge-guide/donut_e2943709001d84b.svg) |

### Custom Label Colors

```markdown
{{ui:donut:85:size=70:label=true:label_color=accent:fill=success/}}
```

![](assets/donut-gauge-guide/donut_404c1e151a1aeba1.svg)

---

## Donut Slider Mode

Add a `thumb` parameter to show an indicator at the fill position:

| Syntax | Result |
|--------|--------|
| `{{ui:donut:30:size=60:thumb=12/}}` | ![](assets/donut-gauge-guide/donut_420b82876393738c.svg) |
| `{{ui:donut:50:size=60:thumb=12/}}` | ![](assets/donut-gauge-guide/donut_d428761f7c3a190c.svg) |
| `{{ui:donut:70:size=60:thumb=12/}}` | ![](assets/donut-gauge-guide/donut_181c7608c0cb706a.svg) |
| `{{ui:donut:90:size=60:thumb=12/}}` | ![](assets/donut-gauge-guide/donut_62be9abe08993fb7.svg) |

### Thumb Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:donut:60:size=60:thumb=8/}}` | ![](assets/donut-gauge-guide/donut_53496b22a6f008ab.svg) |
| `{{ui:donut:60:size=60:thumb=12/}}` | ![](assets/donut-gauge-guide/donut_5c4528d280d66ce9.svg) |
| `{{ui:donut:60:size=60:thumb=16/}}` | ![](assets/donut-gauge-guide/donut_8f683d50cfd6cd4f.svg) |
| `{{ui:donut:60:size=60:thumb=20/}}` | ![](assets/donut-gauge-guide/donut_6397fed2fd723142.svg) |

---

# Gauge Component

A semi-circular meter for dashboard-style displays.

## Basic Gauge Syntax

```markdown
{{ui:gauge:PERCENT/}}
```

Where `PERCENT` is a number from 0 to 100.

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:0/}}` | ![](assets/donut-gauge-guide/gauge_23f217ef7400eb.svg) |
| `{{ui:gauge:25/}}` | ![](assets/donut-gauge-guide/gauge_e88045b13327fa57.svg) |
| `{{ui:gauge:50/}}` | ![](assets/donut-gauge-guide/gauge_8e0bbd4f53287ff0.svg) |
| `{{ui:gauge:75/}}` | ![](assets/donut-gauge-guide/gauge_4836b33960fb4af3.svg) |
| `{{ui:gauge:100/}}` | ![](assets/donut-gauge-guide/gauge_2ff5a5c77cbab7a6.svg) |

---

## Gauge Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `percent` | number | *required* | Progress percentage (0-100) |
| `size` | number | 80 | Width in pixels |
| `thickness` | number | 8 | Arc thickness in pixels |
| `track` | color | slate | Track (background) color |
| `fill` | color | accent | Fill (progress) color |
| `label` | boolean | false | Show percentage label below arc |
| `label_color` | color | white | Label text color |
| `thumb` | number | none | Thumb size (enables slider mode) |
| `thumb_color` | color | fill | Thumb color |
| `thumb_border` | color | none | Thumb border/stroke color |
| `thumb_border_width` | number | 0 | Thumb border width in pixels |

---

## Gauge Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:65:size=50/}}` | ![](assets/donut-gauge-guide/gauge_a9bdd0f0fae8e951.svg) |
| `{{ui:gauge:65:size=80/}}` | ![](assets/donut-gauge-guide/gauge_f072ec001b88a117.svg) |
| `{{ui:gauge:65:size=100/}}` | ![](assets/donut-gauge-guide/gauge_4c977f251e5ddd43.svg) |
| `{{ui:gauge:65:size=120/}}` | ![](assets/donut-gauge-guide/gauge_7ee67f1a5dc20187.svg) |

---

## Gauge Thickness

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:70:size=100:thickness=4/}}` | ![](assets/donut-gauge-guide/gauge_70fdb0206bc2a819.svg) |
| `{{ui:gauge:70:size=100:thickness=8/}}` | ![](assets/donut-gauge-guide/gauge_971accc126cfb69e.svg) |
| `{{ui:gauge:70:size=100:thickness=12/}}` | ![](assets/donut-gauge-guide/gauge_b821ed9f1f7b43a2.svg) |
| `{{ui:gauge:70:size=100:thickness=16/}}` | ![](assets/donut-gauge-guide/gauge_325cfade06e47570.svg) |

### Hairline Gauge

```markdown
{{ui:gauge:80:size=120:thickness=3:fill=accent/}}
```

![](assets/donut-gauge-guide/gauge_60f42f8d76426fa6.svg)

### Chunky Gauge

```markdown
{{ui:gauge:60:size=120:thickness=20:fill=info/}}
```

![](assets/donut-gauge-guide/gauge_521d6e61d50a1302.svg)

---

## Gauge Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:70:size=80:fill=accent/}}` | ![](assets/donut-gauge-guide/gauge_697640d9d1fc77fb.svg) |
| `{{ui:gauge:70:size=80:fill=success/}}` | ![](assets/donut-gauge-guide/gauge_b013dabbbdd4acec.svg) |
| `{{ui:gauge:70:size=80:fill=warning/}}` | ![](assets/donut-gauge-guide/gauge_ce0c4d93e692bae2.svg) |
| `{{ui:gauge:70:size=80:fill=error/}}` | ![](assets/donut-gauge-guide/gauge_4fb1172f2d412068.svg) |
| `{{ui:gauge:70:size=80:fill=info/}}` | ![](assets/donut-gauge-guide/gauge_126642c98de6a71b.svg) |

### Speedometer Style

```markdown
{{ui:gauge:25:size=100:thickness=10:fill=success/}}
{{ui:gauge:55:size=100:thickness=10:fill=warning/}}
{{ui:gauge:85:size=100:thickness=10:fill=error/}}
```

![](assets/donut-gauge-guide/gauge_182961c2d3691731.svg) ![](assets/donut-gauge-guide/gauge_fdfcb4159304dc99.svg) ![](assets/donut-gauge-guide/gauge_b2e252af1ca27f88.svg)

### Neon Colors

```markdown
{{ui:gauge:75:size=100:fill=00FF41:track=0D0D0D/}}
{{ui:gauge:60:size=100:fill=FF00FF:track=0D0D0D/}}
{{ui:gauge:85:size=100:fill=00FFFF:track=0D0D0D/}}
```

![](assets/donut-gauge-guide/gauge_ce04453e96157e94.svg) ![](assets/donut-gauge-guide/gauge_b138de923c70fbdd.svg) ![](assets/donut-gauge-guide/gauge_2cfe0cf5eee3e688.svg)

---

## Gauge Labels

Show percentage with `label=true`:

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:25:size=100:label=true/}}` | ![](assets/donut-gauge-guide/gauge_10e37342b0d4dff4.svg) |
| `{{ui:gauge:50:size=100:label=true/}}` | ![](assets/donut-gauge-guide/gauge_2fc101a56d7592bf.svg) |
| `{{ui:gauge:75:size=100:label=true/}}` | ![](assets/donut-gauge-guide/gauge_c378b59ae2f379ce.svg) |
| `{{ui:gauge:100:size=100:label=true/}}` | ![](assets/donut-gauge-guide/gauge_879e98a989bc4a2c.svg) |

---

## Gauge Slider Mode

Add a `thumb` parameter for interactive-style indicator:

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:30:size=100:thumb=14/}}` | ![](assets/donut-gauge-guide/gauge_f5ad48ee8e5b7d2f.svg) |
| `{{ui:gauge:50:size=100:thumb=14/}}` | ![](assets/donut-gauge-guide/gauge_ffed1ec483f5fe1.svg) |
| `{{ui:gauge:70:size=100:thumb=14/}}` | ![](assets/donut-gauge-guide/gauge_ae4ff7f36e1a874a.svg) |
| `{{ui:gauge:90:size=100:thumb=14/}}` | ![](assets/donut-gauge-guide/gauge_713380368085d0b4.svg) |

---

# Thumb Customization

Both donut and gauge support custom thumb colors and borders:

### Donut Thumb Colors

| Syntax | Result |
|--------|--------|
| `{{ui:donut:60:size=60:thumb=14:thumb_color=accent/}}` | ![](assets/donut-gauge-guide/donut_be6b9d8b54140657.svg) |
| `{{ui:donut:60:size=60:thumb=14:thumb_color=success/}}` | ![](assets/donut-gauge-guide/donut_56a3dfff231755ed.svg) |
| `{{ui:donut:60:size=60:thumb=14:thumb_color=warning/}}` | ![](assets/donut-gauge-guide/donut_c093ac29b7dc5c2e.svg) |
| `{{ui:donut:60:size=60:thumb=14:thumb_color=white/}}` | ![](assets/donut-gauge-guide/donut_389bec4fb2066248.svg) |

### Gauge Thumb Colors

| Syntax | Result |
|--------|--------|
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=accent/}}` | ![](assets/donut-gauge-guide/gauge_d90e23e10c164306.svg) |
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=success/}}` | ![](assets/donut-gauge-guide/gauge_4f6830222c186442.svg) |
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=warning/}}` | ![](assets/donut-gauge-guide/gauge_d567d241322ea6ad.svg) |
| `{{ui:gauge:60:size=100:thumb=16:thumb_color=error/}}` | ![](assets/donut-gauge-guide/gauge_ffac00d7073c5e92.svg) |

### Contrasting Thumb

```markdown
{{ui:donut:75:size=70:fill=info:thumb=16:thumb_color=accent/}}
{{ui:gauge:65:size=110:fill=success:thumb=18:thumb_color=error/}}
```

![](assets/donut-gauge-guide/donut_823f45e337ec2d67.svg) ![](assets/donut-gauge-guide/gauge_d58590afd958787b.svg)

### Thumb Borders

Add a stroke border to thumbs with `thumb_border` and `thumb_border_width`:

```markdown
{{ui:donut:60:size=70:thumb=16:thumb_border=white:thumb_border_width=2/}}
{{ui:gauge:60:size=100:thumb=18:thumb_border=000000:thumb_border_width=2/}}
```

Borders help thumbs stand out against similar-colored backgrounds.

---

# Complete Examples

### Dashboard Metrics

```markdown
| Metric | Status |
|--------|--------|
| CPU | {{ui:gauge:73:size=60:thickness=6:fill=info/}} |
| Memory | {{ui:gauge:45:size=60:thickness=6:fill=success/}} |
| Disk | {{ui:gauge:88:size=60:thickness=6:fill=warning/}} |
| Network | {{ui:gauge:32:size=60:thickness=6:fill=cobalt/}} |
```

| Metric | Status |
|--------|--------|
| CPU | ![](assets/donut-gauge-guide/gauge_2fe8a5cca934f0c1.svg) |
| Memory | ![](assets/donut-gauge-guide/gauge_847751dd2447d359.svg) |
| Disk | ![](assets/donut-gauge-guide/gauge_984bba1b6951986c.svg) |
| Network | ![](assets/donut-gauge-guide/gauge_69c649b019067cb5.svg) |

### Task Completion Donuts

```markdown
| Task | Progress |
|------|----------|
| Research | {{ui:donut:100:size=40:fill=success:label=true/}} |
| Design | {{ui:donut:75:size=40:fill=info:label=true/}} |
| Development | {{ui:donut:45:size=40:fill=warning:label=true/}} |
| Testing | {{ui:donut:20:size=40:fill=error:label=true/}} |
```

| Task | Progress |
|------|----------|
| Research | ![](assets/donut-gauge-guide/donut_58ced3f47a022213.svg) |
| Design | ![](assets/donut-gauge-guide/donut_73c5f4edf523a2c.svg) |
| Development | ![](assets/donut-gauge-guide/donut_51e2d043ea9c1b57.svg) |
| Testing | ![](assets/donut-gauge-guide/donut_d0f292710d6df70d.svg) |

### Volume Control Wheel

```markdown
{{ui:donut:30:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}}
{{ui:donut:60:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}}
{{ui:donut:90:size=80:thickness=10:thumb=18:thumb_color=white:fill=cobalt/}}
```

![](assets/donut-gauge-guide/donut_7c1885a93adfa016.svg) ![](assets/donut-gauge-guide/donut_9e6fdf880b03aac3.svg) ![](assets/donut-gauge-guide/donut_271dec50a3882434.svg)

### Speedometer Dashboard

```markdown
{{ui:gauge:20:size=120:thickness=12:fill=22C55E:label=true/}}
{{ui:gauge:60:size=120:thickness=12:fill=EAB308:label=true/}}
{{ui:gauge:95:size=120:thickness=12:fill=EF4444:label=true/}}
```

![](assets/donut-gauge-guide/gauge_5f33f78938de0d2d.svg) ![](assets/donut-gauge-guide/gauge_938b0d41ddbcb0.svg) ![](assets/donut-gauge-guide/gauge_be8fc1200d8c9365.svg)

### Comparison: Donut vs Gauge vs Progress

Same data, different visualizations:

| Task | Donut | Gauge | Bar |
|------|-------|-------|-----|
| API | ![](assets/donut-gauge-guide/donut_f93d19d3265f9216.svg) | ![](assets/donut-gauge-guide/gauge_baca01a9bb105e80.svg) | ![](assets/donut-gauge-guide/progress_e7f5b75bbf0a1a9e.svg) |
| Tests | ![](assets/donut-gauge-guide/donut_9d69435c4383e4fe.svg) | ![](assets/donut-gauge-guide/gauge_8f94c8a55d793b6e.svg) | ![](assets/donut-gauge-guide/progress_1dcb85164099b0a.svg) |
| Docs | ![](assets/donut-gauge-guide/donut_faab25effff769cd.svg) | ![](assets/donut-gauge-guide/gauge_96adc6bc8e44a1e7.svg) | ![](assets/donut-gauge-guide/progress_a41686b9cb0a298a.svg) |

---

## See Also

- [Progress Guide](PROGRESS-GUIDE.md)
- [Swatch Guide](SWATCH-GUIDE.md)
- [Components Reference](../COMPONENTS.md)
- [CLI Guide](CLI-GUIDE.md)
