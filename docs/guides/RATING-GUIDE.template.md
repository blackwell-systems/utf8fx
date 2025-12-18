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
| `{{ui:rating:1/}}` | {{ui:rating:1/}} |
| `{{ui:rating:2/}}` | {{ui:rating:2/}} |
| `{{ui:rating:3/}}` | {{ui:rating:3/}} |
| `{{ui:rating:4/}}` | {{ui:rating:4/}} |
| `{{ui:rating:5/}}` | {{ui:rating:5/}} |

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
| `{{ui:rating:3:icon=star/}}` | {{ui:rating:3:icon=star/}} |
| `{{ui:rating:4:icon=star/}}` | {{ui:rating:4:icon=star/}} |
| `{{ui:rating:5:icon=star/}}` | {{ui:rating:5:icon=star/}} |

### Hearts

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:icon=heart/}}` | {{ui:rating:3:icon=heart/}} |
| `{{ui:rating:4:icon=heart:fill=error/}}` | {{ui:rating:4:icon=heart:fill=error/}} |
| `{{ui:rating:5:icon=heart:fill=pink/}}` | {{ui:rating:5:icon=heart:fill=pink/}} |

### Circles

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:icon=circle/}}` | {{ui:rating:3:icon=circle/}} |
| `{{ui:rating:4:icon=circle:fill=info/}}` | {{ui:rating:4:icon=circle:fill=info/}} |
| `{{ui:rating:5:icon=circle:fill=success/}}` | {{ui:rating:5:icon=circle:fill=success/}} |

---

## Partial Ratings

The rating component supports decimal values for partial fills:

| Syntax | Result |
|--------|--------|
| `{{ui:rating:2.5/}}` | {{ui:rating:2.5/}} |
| `{{ui:rating:3.5/}}` | {{ui:rating:3.5/}} |
| `{{ui:rating:4.5/}}` | {{ui:rating:4.5/}} |
| `{{ui:rating:3.7/}}` | {{ui:rating:3.7/}} |
| `{{ui:rating:4.2/}}` | {{ui:rating:4.2/}} |

### Partial Hearts

| Syntax | Result |
|--------|--------|
| `{{ui:rating:2.5:icon=heart:fill=error/}}` | {{ui:rating:2.5:icon=heart:fill=error/}} |
| `{{ui:rating:3.5:icon=heart:fill=error/}}` | {{ui:rating:3.5:icon=heart:fill=error/}} |

---

## Colors

### Fill Colors

| Syntax | Result |
|--------|--------|
| `{{ui:rating:4:fill=warning/}}` | {{ui:rating:4:fill=warning/}} |
| `{{ui:rating:4:fill=success/}}` | {{ui:rating:4:fill=success/}} |
| `{{ui:rating:4:fill=error/}}` | {{ui:rating:4:fill=error/}} |
| `{{ui:rating:4:fill=info/}}` | {{ui:rating:4:fill=info/}} |
| `{{ui:rating:4:fill=accent/}}` | {{ui:rating:4:fill=accent/}} |

### Empty Colors

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:empty=slate/}}` | {{ui:rating:3:empty=slate/}} |
| `{{ui:rating:3:empty=ink/}}` | {{ui:rating:3:empty=ink/}} |
| `{{ui:rating:3:empty=dark2/}}` | {{ui:rating:3:empty=dark2/}} |

### Custom Hex Colors

```markdown
{{ui:rating:4:fill=FFD700/}}
{{ui:rating:4:fill=FF6B35:empty=1a1a2e/}}
```

{{ui:rating:4:fill=FFD700/}}
{{ui:rating:4:fill=FF6B35:empty=1a1a2e/}}

---

## Sizes

| Syntax | Result |
|--------|--------|
| `{{ui:rating:4:size=12/}}` | {{ui:rating:4:size=12/}} |
| `{{ui:rating:4:size=16/}}` | {{ui:rating:4:size=16/}} |
| `{{ui:rating:4:size=20/}}` | {{ui:rating:4:size=20/}} |
| `{{ui:rating:4:size=24/}}` | {{ui:rating:4:size=24/}} |
| `{{ui:rating:4:size=32/}}` | {{ui:rating:4:size=32/}} |

---

## Spacing

| Syntax | Result |
|--------|--------|
| `{{ui:rating:4:spacing=0/}}` | {{ui:rating:4:spacing=0/}} |
| `{{ui:rating:4:spacing=2/}}` | {{ui:rating:4:spacing=2/}} |
| `{{ui:rating:4:spacing=4/}}` | {{ui:rating:4:spacing=4/}} |
| `{{ui:rating:4:spacing=8/}}` | {{ui:rating:4:spacing=8/}} |

---

## Maximum Rating

Change the number of icons with `max`:

| Syntax | Result |
|--------|--------|
| `{{ui:rating:3:max=5/}}` | {{ui:rating:3:max=5/}} |
| `{{ui:rating:5:max=7/}}` | {{ui:rating:5:max=7/}} |
| `{{ui:rating:8:max=10/}}` | {{ui:rating:8:max=10/}} |

---

## Complete Examples

### Product Reviews

| Product | Rating |
|---------|--------|
| Excellent | {{ui:rating:5:size=16/}} |
| Great | {{ui:rating:4.5:size=16/}} |
| Good | {{ui:rating:4:size=16/}} |
| Average | {{ui:rating:3:size=16/}} |
| Poor | {{ui:rating:2:size=16/}} |

### Difficulty Levels

| Level | Difficulty |
|-------|------------|
| Easy | {{ui:rating:1:max=5:icon=circle:fill=success:size=14/}} |
| Medium | {{ui:rating:3:max=5:icon=circle:fill=warning:size=14/}} |
| Hard | {{ui:rating:5:max=5:icon=circle:fill=error:size=14/}} |

### Heart-Based Likes

{{ui:rating:4:icon=heart:fill=error:size=24/}}

### Large Feature Rating

{{ui:rating:4.5:size=32:fill=warning/}}

---

## See Also

- [Progress Guide](PROGRESS-GUIDE.md)
- [Donut & Gauge Guide](DONUT-GAUGE-GUIDE.md)
- [Components Reference](COMPONENTS-GUIDE.md)
