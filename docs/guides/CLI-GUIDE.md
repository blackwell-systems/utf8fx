# CLI Guide

Command-line usage for mdfx, including multi-target rendering.

## Quick Start

```bash
# Basic processing
mdfx process input.md -o output.md

# With target platform
mdfx process input.md --target github -o README.md

# Build for multiple targets at once
mdfx build input.md --all-targets
```

---

## Commands

### `mdfx process`

Process a single markdown file.

```bash
mdfx process <INPUT> [OPTIONS]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `-o, --output <FILE>` | Output file path | stdout |
| `--target <TARGET>` | Target platform | `github` |
| `--backend <BACKEND>` | Rendering backend | auto |
| `--assets-dir <DIR>` | Directory for SVG assets | `assets/mdfx` |
| `--palette <FILE>` | Custom palette JSON | none |

**Examples:**

```bash
# Process for GitHub README
mdfx process README.template.md --target github -o README.md

# Process for local documentation (generates SVG files)
mdfx process docs/index.template.md --target local -o docs/index.md --assets-dir docs/assets

# Process for PyPI (plain text fallbacks)
mdfx process README.template.md --target pypi -o PKG-INFO.md

# Override backend (use SVG even for GitHub)
mdfx process README.template.md --target github --backend svg -o README.md
```

---

### `mdfx build`

Build for multiple targets at once.

```bash
mdfx build <INPUT> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `--all-targets` | Build for all 5 targets |
| `--targets <LIST>` | Comma-separated target list |
| `-o, --output-dir <DIR>` | Output directory | `dist/` |
| `--palette <FILE>` | Custom palette JSON |

**Examples:**

```bash
# Build for all targets
mdfx build README.template.md --all-targets

# Build for specific targets
mdfx build README.template.md --targets github,pypi,npm

# Custom output directory
mdfx build README.template.md --all-targets -o release/
```

**Output structure:**

```
dist/
├── readme_github.md
├── readme_gitlab.md
├── readme_npm.md
├── readme_pypi.md
├── readme_local.md
└── assets/
    └── local/
        ├── swatch_abc123.svg
        └── manifest.json
```

---

### `mdfx watch`

Watch for changes and rebuild automatically.

```bash
mdfx watch <INPUT> [OPTIONS]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `-o, --output <FILE>` | Output file | required |
| `--target <TARGET>` | Target platform | `github` |
| `--backend <BACKEND>` | Rendering backend | auto |
| `--debounce <MS>` | Rebuild delay | `100` |

**Examples:**

```bash
# Watch and rebuild on changes
mdfx watch README.template.md -o README.md

# Watch with custom target
mdfx watch docs/source.md -o docs/output.md --target local --backend svg
```

---

## Targets

mdfx supports 5 rendering targets, each optimized for different platforms.

### Available Targets

| Target | Platform | Backend | Use Case |
|--------|----------|---------|----------|
| `github` | GitHub | shields.io | GitHub READMEs (default) |
| `gitlab` | GitLab | shields.io | GitLab READMEs |
| `npm` | npm | shields.io | npm package READMEs |
| `pypi` | PyPI | plaintext | Python package descriptions |
| `local` | Local docs | SVG files | Offline documentation |

### Target Details

#### `github` (default)

Best for GitHub READMEs. Uses shields.io badges for fast loading.

```bash
mdfx process input.md --target github -o README.md
```

- External images via shields.io
- Full Unicode support
- GitHub-flavored markdown

#### `gitlab`

For GitLab repositories. Similar to GitHub with some extra features.

```bash
mdfx process input.md --target gitlab -o README.md
```

- More permissive HTML support
- GitLab-specific alert syntax

#### `npm`

For npm package READMEs. Optimized for npmjs.com display.

```bash
mdfx process input.md --target npm -o README.md
```

- Similar constraints to GitHub
- shields.io badges

#### `pypi`

For PyPI package descriptions. Uses plain text fallbacks.

```bash
mdfx process input.md --target pypi -o PKG-INFO.md
```

- No embedded images
- ASCII-safe output
- Plain text fallbacks for visual elements

#### `local`

For offline documentation. Generates local SVG files.

```bash
mdfx process input.md --target local -o docs/index.md --assets-dir docs/assets
```

- No external dependencies
- SVG files stored locally
- Asset manifest for tracking

---

## Backends

Backends determine how visual elements are rendered.

| Backend | Output | Use Case |
|---------|--------|----------|
| `shields` | shields.io URLs | Online READMEs |
| `svg` | Local SVG files | Offline docs |
| `plaintext` | ASCII text | PyPI, limited platforms |
| `hybrid` | Auto-selects | Best of both worlds |

### Backend Selection

Each target has a preferred backend:

| Target | Preferred Backend |
|--------|-------------------|
| `github` | `shields` |
| `gitlab` | `shields` |
| `npm` | `shields` |
| `pypi` | `plaintext` |
| `local` | `svg` |

Override with `--backend`:

```bash
# Use SVG for GitHub (offline-capable README)
mdfx process input.md --target github --backend svg -o README.md

# Use shields.io for local docs
mdfx process input.md --target local --backend shields -o docs/index.md
```

### Hybrid Backend

Automatically selects shields.io or SVG based on feature usage:

```bash
mdfx process input.md --backend hybrid -o output.md
```

- Simple swatches → shields.io (fast, no files)
- Gradients, shadows, custom borders → SVG (full features)

---

## Custom Palettes

Define custom colors in a JSON file:

```json
{
  "brand": "FF5500",
  "primary": "2B6CB0",
  "secondary": "48BB78"
}
```

Use with `--palette`:

```bash
mdfx process input.md --palette brand-colors.json -o output.md
```

Then reference in templates:

```markdown
{{ui:swatch:brand/}}
{{ui:swatch:primary/}}
```

---

## Common Workflows

### GitHub README

```bash
# Development: watch mode
mdfx watch README.template.md -o README.md

# Production: single build
mdfx process README.template.md --target github -o README.md
```

### Multi-Platform Package

```bash
# Build for GitHub, npm, and PyPI
mdfx build README.template.md --targets github,npm,pypi

# Or build all at once
mdfx build README.template.md --all-targets
```

### Documentation Site

```bash
# Generate with local SVG assets
mdfx process docs/index.template.md \
  --target local \
  --backend svg \
  --assets-dir docs/assets \
  -o docs/index.md
```

### CI/CD Pipeline

```yaml
# GitHub Actions example
- name: Build README
  run: mdfx process README.template.md --target github -o README.md

- name: Build for all platforms
  run: mdfx build README.template.md --all-targets -o dist/
```

---

## Other Commands

### `mdfx styles`

List available text styles.

```bash
mdfx styles
mdfx styles --examples
```

### `mdfx frames`

List available frames.

```bash
mdfx frames
mdfx frames --examples
```

### `mdfx glyphs`

List available glyphs.

```bash
mdfx glyphs
mdfx glyphs --category stars
```

### `mdfx separators`

List available separator characters.

```bash
mdfx separators
mdfx separators --examples
```

### `mdfx lsp`

Start the Language Server Protocol server (for editor integration).

```bash
mdfx lsp
```

Requires: `cargo install mdfx-cli --features lsp`

---

## See Also

- [Template Syntax](../TEMPLATE-SYNTAX.md) - Full syntax reference
- [API Guide](../API-GUIDE.md) - Library API documentation
- [Targets Spec](../TARGETS.md) - Technical target specification
