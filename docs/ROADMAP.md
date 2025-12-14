# mdfx Roadmap

Planned features for future versions.

---

## v1.1.0 - Discoverability & Caching

### CLI Discovery Commands

Add commands to explore available components without reading docs:

```bash
mdfx components              # List all components
mdfx components show divider # Show component details
mdfx palette                 # List palette colors
mdfx frames                  # List frame styles
```

### Asset Manifest

Generate `manifest.json` for CI caching and reproducible builds:

```json
{
  "version": "1.0",
  "backend": "svg",
  "assets": [
    { "path": "swatch_541bbacc.svg", "sha256": "...", "type": "swatch" }
  ]
}
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
- Grapheme cluster support for emoji separators

---

## Contributing

Feature requests: https://github.com/blackwell-systems/mdfx/issues
