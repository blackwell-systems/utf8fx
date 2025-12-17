# mdfx Roadmap

Planned features for future versions.

---

## v1.1.0 - Discoverability & Caching

### CLI Discovery Commands

Expand `mdfx list` to explore all available resources:

```bash
mdfx list                    # List styles (current)
mdfx list components         # List all UI components
mdfx list glyphs             # List named glyphs
mdfx list frames             # List frame styles
mdfx list palette            # List palette colors
```

### Smart Caching

Skip writing unchanged SVG files for faster rebuilds.

---

## v1.2.0 - Tooling

### Inline SVG Mode

Embed SVGs as data URIs for single-file output:

```bash
mdfx process --backend svg-inline input.md
```

### Template Formatter

Normalize template formatting for team consistency:

```bash
mdfx fmt README.template.md
mdfx fmt --check README.template.md  # CI mode
```

### Strict Mode

Fail on warnings for CI enforcement:

```bash
mdfx process --strict input.md
```

---

## v1.3.0+ - Extensions

- Spacer primitive for layout control
- Rule primitive for lines
- BadgeGroup for consistent badge spacing
- **Template includes** - `{{include:_header.md/}}` to compose documents from partials
- **Conditional blocks** - `{{#if target=github}}GitHub-only content{{/if}}`

---

## Contributing

Feature requests: https://github.com/blackwell-systems/mdfx/issues
