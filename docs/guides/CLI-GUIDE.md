# CLI Guide

Command-line usage for mdfx, including multi-target rendering.

## Table of Contents

- [Quick Start](#quick-start)
- [Commands](#commands)
  - [mdfx process](#mdfx-process)
  - [mdfx build](#mdfx-build)
  - [mdfx watch](#mdfx-watch)
- [Targets](#targets)
  - [Available Targets](#available-targets)
  - [Target Details](#target-details)
- [Backends](#backends)
  - [Backend Selection](#backend-selection)
  - [Tech Badges with shields.io](#tech-badges-with-shieldsio)
  - [Incremental Asset Generation](#incremental-asset-generation)
- [Configuration File](#configuration-file)
  - [Auto-Discovery](#auto-discovery)
  - [Config File Format](#config-file-format)
  - [Using Partials](#using-partials)
- [Custom Palettes](#custom-palettes)
- [Tech Badges](#tech-badges)
  - [Intelligent Logo Colors](#intelligent-logo-colors)
  - [Text Customization](#text-customization)
- [Common Workflows](#common-workflows)
- [Other Commands](#other-commands)
  - [mdfx verify](#mdfx-verify)
  - [mdfx clean](#mdfx-clean)
- [See Also](#see-also)

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
| `github` | GitHub | SVG | GitHub READMEs (default) |
| `gitlab` | GitLab | SVG | GitLab READMEs |
| `npm` | npm | SVG | npm package READMEs |
| `local` | Local docs | SVG | Offline documentation |
| `pypi` | PyPI | plaintext | Python package descriptions |

### Target Details

#### `github` (default)

Best for GitHub READMEs. Uses SVG assets for full-fidelity rendering.

```bash
mdfx process input.md --target github -o README.md --assets-dir assets
```

- Local SVG files for charts and badges
- Full customization (borders, fonts, colors)
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
| `svg` | Local SVG files | Full-fidelity rendering (default) |
| `shields` | shields.io URLs | Legacy, limited features |
| `plaintext` | ASCII text | PyPI, limited platforms |

### Backend Selection

All targets now default to SVG for full-fidelity rendering:

| Target | Preferred Backend |
|--------|-------------------|
| `github` | `svg` |
| `gitlab` | `svg` |
| `npm` | `svg` |
| `local` | `svg` |
| `pypi` | `plaintext` |

Override with `--backend`:

```bash
# Use shields.io for legacy compatibility
mdfx process input.md --backend shields -o README.md

# Explicit SVG (default)
mdfx process input.md --backend svg --assets-dir assets -o README.md
```

### Tech Badges with shields.io

For tech badges specifically, use `source=shields` to render via shields.io without switching the entire backend:

```markdown
{{ui:tech:rust:source=shields/}}
```

This is useful when you want SVG for charts but shields.io for tech badges.

### Incremental Asset Generation

When using the SVG backend, mdfx only writes new assets:

```bash
$ mdfx process input.md --backend svg -o output.md
Info: Assets: 2 written, 39 unchanged (assets/mdfx)
```

- **Hash-based filenames** - Same component parameters produce identical filenames
- **Skip existing** - Files that already exist are not rewritten
- **Faster rebuilds** - Repeated builds only write changed/new assets

This makes watch mode and CI builds much faster when most assets haven't changed.

---

## Configuration File

mdfx supports a `.mdfx.json` configuration file for project-wide settings, including reusable template partials and custom palettes.

### Auto-Discovery

By default, mdfx searches for `.mdfx.json` in the current directory and parent directories:

```bash
# Automatically loads .mdfx.json if found
mdfx process input.md -o output.md
```

### Explicit Config Path

```bash
mdfx process input.md --config myconfig.json -o output.md
```

### Config File Format

```json
{
  "partials": {
    "hero": {
      "template": "{{frame:gradient}}{{mathbold}}$1{{/mathbold}}{{/frame}}",
      "description": "Hero header with gradient frame"
    },
    "techstack": {
      "template": "{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}}"
    },
    "warning-box": {
      "template": "{{frame:solid-left}}⚠️ $content{{/frame}}"
    }
  },
  "palette": {
    "brand": "FF5500",
    "primary": "2B6CB0"
  }
}
```

### Using Partials

In your markdown:

```markdown
{{partial:hero}}MY TITLE{{/partial}}
{{partial:techstack/}}
{{partial:warning-box}}Watch out!{{/partial}}
```

**Output:**
```
▓▒░ 𝐌𝐘 𝐓𝐈𝐓𝐋𝐄 ░▒▓
[rust badge] [typescript badge] [docker badge]
█▌⚠️ Watch out!
```

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

## Tech Badges

Tech badges display technology logos with brand colors using Simple Icons.

### Basic Usage

```markdown
{{ui:tech:rust/}}
{{ui:tech:python/}}
{{ui:tech:typescript/}}
```

### Intelligent Logo Colors

Logo colors are automatically selected based on background luminance:

| Background | Logo Color | Example |
|------------|------------|---------|
| Light (orange, cyan) | Black | Rust, Go |
| Dark (blue, black) | White | PostgreSQL, Docker |

Override with `logo` parameter:

```markdown
{{ui:tech:rust:logo=white/}}      <!-- Force white logo -->
{{ui:tech:docker:logo=000000/}}   <!-- Force black logo -->
```

### Text Customization

Customize the label text appearance:

```markdown
{{ui:tech:rust:text_color=white/}}           <!-- White text -->
{{ui:tech:python:font=Monaco,monospace/}}    <!-- Custom font -->
{{ui:tech:go:text=000000:font=Arial/}}       <!-- Both -->
```

**Parameters:**

| Parameter | Aliases | Description |
|-----------|---------|-------------|
| `text_color` | `text`, `color` | Label text color (hex) |
| `font` | `font_family` | Font family for label |
| `logo` | - | Logo color override (hex) |
| `bg` | - | Background color override |
| `label` | - | Custom label text |

### Examples

```markdown
<!-- Default: auto logo color based on brand -->
{{ui:tech:rust/}}

<!-- Custom styling -->
{{ui:tech:postgresql:text_color=FFFFFF:font=monospace/}}

<!-- Override background -->
{{ui:tech:docker:bg=000000/}}

<!-- Custom label -->
{{ui:tech:typescript:label=TS/}}
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

### `mdfx verify`

Verify asset integrity against manifest.

```bash
mdfx verify --assets-dir assets/mdfx
```

Checks that all assets in `manifest.json` exist on disk with correct hashes. Useful for detecting corruption or verifying CI caches.

---

### `mdfx clean`

Remove unreferenced assets from the assets directory.

```bash
mdfx clean [OPTIONS]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--assets-dir <DIR>` | Assets directory containing manifest.json | `assets/mdfx` |
| `--dry-run` | Show what would be deleted without deleting | false |
| `--scan <PATTERN>` | Glob pattern for markdown files to scan | none |

**Examples:**

```bash
# Clean assets not in manifest.json
mdfx clean --assets-dir assets/mdfx

# Preview what would be deleted
mdfx clean --dry-run

# Scan markdown files to find actually referenced assets
mdfx clean --scan "docs/**/*.md" --assets-dir docs/assets

# Preview scan-based cleanup
mdfx clean --scan "examples/*-rendered.md" --assets-dir examples/assets --dry-run
```

**Modes:**

1. **Manifest mode** (default): Removes SVG files not listed in `manifest.json`
2. **Scan mode** (`--scan`): Parses markdown files for image references and removes assets not found in any scanned file

Scan mode is useful after refactoring when you've removed components from your markdown but the SVG files remain.

---

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
