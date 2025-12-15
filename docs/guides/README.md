# mdfx User Guides

Comprehensive guides for every mdfx feature. Each guide covers syntax, parameters, examples, and best practices.

## Getting Started

| Guide | Description |
|-------|-------------|
| [CLI Guide](CLI-GUIDE.md) | Command-line usage, multi-target builds, watch mode |

## Visual Components

| Guide | Description |
|-------|-------------|
| [Swatches](SWATCH-GUIDE.md) | Color blocks with 10 parameters, 5 styles, pixel art techniques |
| [Components](COMPONENTS-GUIDE.md) | UI elements: divider, tech, status, row, header, callout |
| [Frames](FRAMES-GUIDE.md) | 30+ decorative frames with combos, modifiers, and glyph frames |

## Text & Symbols

| Guide | Description |
|-------|-------------|
| [Text Styles](TEXT-STYLES-GUIDE.md) | 23 Unicode typography styles (bold, script, gothic, subscript, etc.) |
| [Glyphs](GLYPHS-GUIDE.md) | 493 Unicode symbols organized by category |

## Editor Integration

| Guide | Description |
|-------|-------------|
| [LSP Guide](LSP-GUIDE.md) | Language Server Protocol for IDE autocompletion |
| [VS Code Publishing](VSCODE-PUBLISHING.md) | Publishing the VS Code extension |

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
{{fr:star*3}}VIP{{/}}                    <!-- repeated -->
```

**Text Style:**
```markdown
{{mathbold}}BOLD TEXT{{/mathbold}}
{{fraktur}}Gothic Text{{/fraktur}}
{{subscript}}H2O{{/subscript}}
```

**Component:**
```markdown
{{ui:tech:rust/}} {{ui:tech:python/}}
{{ui:divider/}}
{{ui:status:success/}}
```

**Multi-Target Build:**
```bash
mdfx build README.template.md --all-targets
mdfx process input.md --target github -o README.md
mdfx process input.md --target pypi -o PKG-INFO.md
```

---

## See Also

- [Template Syntax](../TEMPLATE-SYNTAX.md) - Full syntax specification
- [Targets Spec](../TARGETS.md) - Multi-target rendering details
- [Architecture](../ARCHITECTURE.md) - System design overview
- [API Guide](../API-GUIDE.md) - Library API reference
