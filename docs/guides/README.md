# mdfx User Guides

Comprehensive guides for every mdfx feature. Each guide covers syntax, parameters, examples, and best practices.

## Visual Components

| Guide | Description |
|-------|-------------|
| [Swatches](SWATCH-GUIDE.md) | Color blocks with 10 parameters, 5 styles, pixel art techniques |
| [Components](COMPONENTS-GUIDE.md) | UI elements: divider, tech, status, row, header, callout |
| [Frames](FRAMES-GUIDE.md) | 29 decorative Unicode borders and visual wrappers |

## Text Transformation

| Guide | Description |
|-------|-------------|
| [Text Styles](TEXT-STYLES-GUIDE.md) | 19 Unicode typography styles (bold, script, gothic, etc.) |
| [Badges](BADGES-GUIDE.md) | 6 numeral/letter badge styles for lists and markers |

---

## Quick Examples

**Swatch:**
```markdown
{{ui:swatch:accent/}}
{{ui:swatch:FF5500:width=100:height=30/}}
```

**Frame:**
```markdown
{{frame:gradient}}HEADER{{/frame}}
```

**Text Style:**
```markdown
{{mathbold}}BOLD TEXT{{/mathbold}}
{{fraktur}}Gothic Text{{/fraktur}}
```

**Badge:**
```markdown
{{badge:circle:1/}} First step
{{badge:circle:2/}} Second step
```

**Component:**
```markdown
{{ui:tech:rust/}} {{ui:tech:python/}}
{{ui:row:align=center}}content{{/ui}}
```

---

## See Also

- [Template Syntax](../TEMPLATE-SYNTAX.md) - Full syntax specification
- [Architecture](../ARCHITECTURE.md) - System design overview
- [API Guide](../API-GUIDE.md) - Library API reference
