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
| [Text Styles](TEXT-STYLES-GUIDE.md) | 23 Unicode typography styles (bold, script, gothic, subscript, etc.) |
| [Glyphs](GLYPHS-GUIDE.md) | 493 Unicode symbols organized by category |

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
{{fr:gradient+star}}TITLE{{/}}           <!-- frame combo -->
{{fr:gradient/spacing=1}}Spaced{{/}}     <!-- with spacing -->
```

**Text Style:**
```markdown
{{mathbold}}BOLD TEXT{{/mathbold}}
{{fraktur}}Gothic Text{{/fraktur}}
{{subscript}}H2O{{/subscript}}
```

**Keyboard Keys:**
```markdown
{{kbd:Ctrl+C/}}
{{kbd:⌘+Shift+P/}}
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
