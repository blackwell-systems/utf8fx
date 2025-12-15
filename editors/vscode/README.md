# mdfx for VS Code

Language support for [mdfx](https://github.com/blackwell-systems/mdfx) template syntax in Markdown files.

## Features

- **Autocompletion** for glyphs, styles, frames, components, and colors
- **Hover documentation** showing character previews and descriptions
- **Shield style completions** for badge customization

## Requirements

Install mdfx with LSP support:

```bash
cargo install mdfx-cli --features lsp
```

## Usage

Open any Markdown file and start typing mdfx templates:

```markdown
{{glyph:star.filled/}}  <!-- Autocomplete for 493 glyphs -->
{{mathbold}}Title{{/mathbold}}  <!-- Style completions -->
{{swatch:cobalt/}}  <!-- Component + palette completions -->
{{divider:style=flat-square/}}  <!-- Shield style completions -->
```

## Completion Triggers

| Trigger | Completions |
|---------|-------------|
| `{{` | Styles, components, keywords |
| `{{glyph:` | 493 glyph names |
| `{{frame:` | Frame names |
| `{{swatch:` | Palette colors |
| `style=` | Shield styles |

## Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `mdfx.enable` | `true` | Enable the language server |
| `mdfx.path` | `mdfx` | Path to mdfx executable |
| `mdfx.trace.server` | `off` | LSP trace level |

## Troubleshooting

### "mdfx not found"

Ensure mdfx is installed and in your PATH:

```bash
which mdfx  # Should print path
mdfx lsp --help  # Should show help
```

If installed via cargo, add `~/.cargo/bin` to your PATH.

### Extension not activating

1. Check Output panel (View > Output > mdfx)
2. Verify `mdfx.enable` is `true`
3. Reload VS Code window

## Links

- [mdfx Documentation](https://github.com/blackwell-systems/mdfx)
- [Report Issues](https://github.com/blackwell-systems/mdfx/issues)
