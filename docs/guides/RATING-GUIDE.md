# Rating Component Guide

The rating component displays visual ratings using stars, hearts, or circles with support for partial fills (like 3.5 out of 5 stars).

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [All Parameters](#all-parameters)
- [Icon Types](#icon-types)
- [Partial Ratings](#partial-ratings)
- [Colors](#colors)
- [Sizes](#sizes)
- [Spacing](#spacing)
- [Complete Examples](#complete-examples)

---

## Basic Syntax

```markdown
{{ui:rating:VALUE/}}
```

Where `VALUE` is a number (can be a decimal like 3.5).

| Syntax | Result |
|--------|--------|
| `{{ui:rating:1/}}` | ![](docs/guides/assets/rating-guide/rating_c24bc0f91ceea47b.svg) |
| `{{ui:rating:2/}}` | ![](docs/guides/assets/rating-guide/rating_6474259b1179ffb5.svg) |
| `{{ui:rating:3/}}` | ![](docs/guides/assets/rating-guide/rating_c574237c5d3ed985.svg) |
| `{{ui:rating:4/}}` | ![](docs/guides/assets/rating-guide/rating_56d5186d93550a79.svg) |
| `{{ui:rating:5/}}` | ![](docs/guides/assets/rating-guide/rating_a623ffc1bd46bab0.svg) |

---

## All Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | number | *required* | Rating value (can be decimal) |
| `max` | number | 5 | Maximum number of icons |
| `size` | number | 20 | Icon size in pixels |
| `fill` | color | warning | Fill color for rated icons |
| `empty` | color | gray | Color for empty icons |
| `icon` | string | star | Icon type: star, heart, circle |
| `spacing` | number | 2 | Spacing between icons |

---

## Icon Types

### Stars (default)

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:icon=star/}}` | ![](docs/guides/assets/rating-guide/rating_c574237c5d3ed985.svg) |
| `{{ui:rating:4:icon=star/}}` | ![](docs/guides/assets/rating-guide/rating_56d5186d93550a79.svg) |
| `{{ui:rating:5:icon=star/}}` | ![](docs/guides/assets/rating-guide/rating_a623ffc1bd46bab0.svg) |

### Hearts

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:icon=heart/}}` | ![](docs/guides/assets/rating-guide/rating_c5b16ee03bc5a7a6.svg) |
| `{{ui:rating:4:icon=heart:fill=error/}}` | ![](docs/guides/assets/rating-guide/rating_73e4064b14970563.svg) |
| `{{ui:rating:5:icon=heart:fill=pink/}}` | ![](docs/guides/assets/rating-guide/rating_60187efacfa4f8ae.svg) |

### Circles

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:icon=circle/}}` | ![](docs/guides/assets/rating-guide/rating_e4092408addd969b.svg) |
| `{{ui:rating:4:icon=circle:fill=info/}}` | ![](docs/guides/assets/rating-guide/rating_40cc3e40be961aee.svg) |
| `{{ui:rating:5:icon=circle:fill=success/}}` | ![](docs/guides/assets/rating-guide/rating_4dafc51043959805.svg) |

---

## Partial Ratings

The rating component supports decimal values for partial fills:

| Syntax | Result |
|--------|--------|
| `{{ui:rating:2.5/}}` | ![](docs/guides/assets/rating-guide/rating_3e00995c5613e974.svg) |
| `{{ui:rating:3.5/}}` | ![](docs/guides/assets/rating-guide/rating_14e26f4745533ece.svg) |
| `{{ui:rating:4.5/}}` | ![](docs/guides/assets/rating-guide/rating_2485a6dc7b105ee1.svg) |
| `{{ui:rating:3.7/}}` | ![](docs/guides/assets/rating-guide/rating_5df4b9b3f9aa3076.svg) |
| `{{ui:rating:4.2/}}` | ![](docs/guides/assets/rating-guide/rating_677a1a83a6573bcd.svg) |

### Partial Hearts

| Syntax | Result |
|--------|--------|
| `{{ui:rating:2.5:icon=heart:fill=error/}}` | ![](docs/guides/assets/rating-guide/rating_65dc4fe5be49bbee.svg) |
| `{{ui:rating:3.5:icon=heart:fill=error/}}` | ![](docs/guides/assets/rating-guide/rating_1d06bfc94d8aa7fd.svg) |

---

## Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:rating:4:fill=warning/}}` | ![](docs/guides/assets/rating-guide/rating_56d5186d93550a79.svg) |
| `{{ui:rating:4:fill=success/}}` | ![](docs/guides/assets/rating-guide/rating_668cec7df93cf368.svg) |
| `{{ui:rating:4:fill=error/}}` | ![](docs/guides/assets/rating-guide/rating_79148d7d1e0c2c73.svg) |
| `{{ui:rating:4:fill=info/}}` | ![](docs/guides/assets/rating-guide/rating_beb22f730b32d084.svg) |
| `{{ui:rating:4:fill=accent/}}` | ![](docs/guides/assets/rating-guide/rating_845c97f65123306e.svg) |

### Empty Colors

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:empty=slate/}}` | ![](docs/guides/assets/rating-guide/rating_a626952bdf847959.svg) |
| `{{ui:rating:3:empty=ink/}}` | ![](docs/guides/assets/rating-guide/rating_cd1ac7f7488dc12e.svg) |
| `{{ui:rating:3:empty=dark2/}}` | ![](docs/guides/assets/rating-guide/rating_f5eb40a5a34cb2a0.svg) |

### Custom Hex Colors

```markdown
{{ui:rating:4:fill=FFD700/}}
{{ui:rating:4:fill=FF6B35:empty=1a1a2e/}}
```

![](docs/guides/assets/rating-guide/rating_f14326a73aa49df0.svg)
![](docs/guides/assets/rating-guide/rating_80686cc244ac6659.svg)

---

## Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:rating:4:size=12/}}` | ![](docs/guides/assets/rating-guide/rating_c3824a0fb06f8fe1.svg) |
| `{{ui:rating:4:size=16/}}` | ![](docs/guides/assets/rating-guide/rating_94cddd85443945cf.svg) |
| `{{ui:rating:4:size=20/}}` | ![](docs/guides/assets/rating-guide/rating_56d5186d93550a79.svg) |
| `{{ui:rating:4:size=24/}}` | ![](docs/guides/assets/rating-guide/rating_e98a6d8622dfd9ba.svg) |
| `{{ui:rating:4:size=32/}}` | ![](docs/guides/assets/rating-guide/rating_52be755a2b31fbad.svg) |

---

## Spacing

| Syntax | Result |
|--------|--------|
| `{{ui:rating:4:spacing=0/}}` | ![](docs/guides/assets/rating-guide/rating_beedb7ca95fca55c.svg) |
| `{{ui:rating:4:spacing=2/}}` | ![](docs/guides/assets/rating-guide/rating_56d5186d93550a79.svg) |
| `{{ui:rating:4:spacing=4/}}` | ![](docs/guides/assets/rating-guide/rating_cf7a04a49dacf40d.svg) |
| `{{ui:rating:4:spacing=8/}}` | ![](docs/guides/assets/rating-guide/rating_1eeea74c89a8d779.svg) |

---

## Maximum Rating

Change the number of icons with `max`:

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:max=5/}}` | ![](docs/guides/assets/rating-guide/rating_c574237c5d3ed985.svg) |
| `{{ui:rating:5:max=7/}}` | ![](docs/guides/assets/rating-guide/rating_434112324377eecf.svg) |
| `{{ui:rating:8:max=10/}}` | ![](docs/guides/assets/rating-guide/rating_f1606965a89f6733.svg) |

---

## Complete Examples

### Product Reviews

| Product | Rating |
|---------|--------|
| Excellent | ![](docs/guides/assets/rating-guide/rating_1321d3875d034f8c.svg) |
| Great | ![](docs/guides/assets/rating-guide/rating_2b2bcfd24b6da61c.svg) |
| Good | ![](docs/guides/assets/rating-guide/rating_94cddd85443945cf.svg) |
| Average | ![](docs/guides/assets/rating-guide/rating_ca6af64dce13482b.svg) |
| Poor | ![](docs/guides/assets/rating-guide/rating_ebe13bcc4012860f.svg) |

### Difficulty Levels

| Level | Difficulty |
|-------|------------|
| Easy | ![](docs/guides/assets/rating-guide/rating_1b4d69b1ea36b0e0.svg) |
| Medium | ![](docs/guides/assets/rating-guide/rating_7893c08f655f0943.svg) |
| Hard | ![](docs/guides/assets/rating-guide/rating_ff816d0010604f4e.svg) |

### Heart-Based Likes

![](docs/guides/assets/rating-guide/rating_c6e5d9943b8fbe0d.svg)

### Large Feature Rating

![](docs/guides/assets/rating-guide/rating_586cdf405dbf9f2.svg)

---

## See Also

- [Progress Guide](PROGRESS-GUIDE.md)
- [Donut & Gauge Guide](DONUT-GAUGE-GUIDE.md)
- [Components Reference](COMPONENTS-GUIDE.md)
