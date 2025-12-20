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
| `-i, --in-place` | Modify input file in place | — |
| `--target <TARGET>` | Target platform | `github` |
| `--backend <BACKEND>` | Rendering backend | auto |
| `--assets-dir <DIR>` | Directory for SVG assets | `assets/mdfx` |
| `--assets-prefix <PREFIX>` | Prefix for asset paths in markdown (defaults to assets-dir) | — |
| `--palette <FILE>` | Custom palette JSON | none |
| `--config <FILE>` | Config file (partials, palette) | auto-discover `.mdfx.json` |

**Dynamic badge options** (requires `--features fetch`):

| Flag | Description | Default |
|------|-------------|---------|
| `--offline` | Use cached data only, no network | — |
| `--refresh` | Force refresh cached data | — |
| `--cache-dir <DIR>` | Cache directory for badge data | `.mdfx-cache` |

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

# Write assets to examples/assets/ but markdown references assets/
mdfx process examples/demo.template.md -o examples/demo.md \
  --backend svg --assets-dir examples/assets --assets-prefix assets
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
| `--assets-dir <DIR>` | Directory for SVG assets | `assets/mdfx` |
| `--assets-prefix <PREFIX>` | Prefix for asset paths in markdown (defaults to assets-dir) | — |
| `--palette <FILE>` | Custom palette JSON | none |
| `--config <FILE>` | Config file | auto-discover `.mdfx.json` |
| `--debounce <MS>` | Rebuild delay | `100` |

**Examples:**

```bash
# Watch and rebuild on changes
mdfx watch README.template.md -o README.md

# Watch with custom target
mdfx watch docs/source.md -o docs/output.md --target local --backend svg

# Watch with separate asset directory and markdown prefix
mdfx watch examples/demo.template.md -o examples/demo.md \
  --assets-dir examples/assets --assets-prefix assets
```

---

## Targets

mdfx supports 5 rendering targets, each optimized for different platforms.

### Available Targets

| Target | Platform | Backend | Use Case |
|--------|----------|---------|----------|
| `github` | GitHub | `svg` | GitHub READMEs (default) |
| `gitlab` | GitLab | `svg` | GitLab READMEs |
| `npm` | npm | `svg` | npm package READMEs |
| `local` | Local docs | `svg` | Offline documentation |
| `pypi` | PyPI | `plaintext` | Python package descriptions (ASCII-safe) |

### Target Details

| Target | Notes |
|--------|-------|
| `github` | Default. Full SVG rendering, GitHub-flavored markdown |
| `gitlab` | Like GitHub with more permissive HTML support |
| `npm` | For npmjs.com package READMEs |
| `pypi` | Plain text only - no images, ASCII-safe output |
| `local` | Offline docs with local SVG files, asset manifest |

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

### `mdfx list`

Discover available resources. Lists styles by default, or specify a resource type.

```bash
mdfx list                        # List styles (default)
mdfx list styles --samples       # List styles with sample output
mdfx list components             # List all UI components with params
mdfx list glyphs                 # List glyphs grouped by category
mdfx list frames                 # List frames with previews
mdfx list palette                # List palette colors
```

**Filter results:**
```bash
mdfx list glyphs -f star         # Filter glyphs containing "star"
mdfx list components -f progress # Filter components
```

### `mdfx convert`

Convert text to a Unicode style.

```bash
mdfx convert --style mathbold "HELLO"     # Output: 𝐇𝐄𝐋𝐋𝐎
mdfx convert --style fraktur "Gothic"     # Output: 𝔊𝔬𝔱𝔥𝔦𝔠
```

### `mdfx completions`

Generate shell completion scripts.

```bash
mdfx completions bash > /etc/bash_completion.d/mdfx
mdfx completions zsh > ~/.zsh/completions/_mdfx
mdfx completions fish > ~/.config/fish/completions/mdfx.fish
```

Supported shells: `bash`, `zsh`, `fish`, `elvish`, `powershell`

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

Language Server Protocol commands for editor integration.

Requires: `cargo install mdfx-cli --features lsp`

#### `mdfx lsp install`

Install the VS Code extension automatically:

```bash
mdfx lsp install                  # Install for VS Code (default)
mdfx lsp install --editor vscode  # Explicit editor flag
```

This creates the extension at `~/.vscode/extensions/mdfx-lsp/` and runs `npm install`.

#### `mdfx lsp run`

Start the LSP server (used by editors, not typically run manually):

```bash
mdfx lsp run
```

The server communicates over stdio. See [LSP Guide](LSP-GUIDE.md) for editor configuration.

---

## See Also

- [Template Syntax](../TEMPLATE-SYNTAX.md) - Full syntax reference
- [API Guide](../API-GUIDE.md) - Library API documentation
- [Targets Spec](../TARGETS.md) - Technical target specification
