# LSP Server Guide

The mdfx Language Server Protocol (LSP) server provides IDE integration with autocompletion, hover documentation, and validation for mdfx template syntax.

## Installation

The LSP server is an optional feature to keep the default install lightweight:

```bash
# Install with LSP support
cargo install mdfx-cli --features lsp
```

## Usage

Start the LSP server (communicates over stdio):

```bash
mdfx lsp
```

## Editor Configuration

### VS Code

Create `.vscode/settings.json` in your project:

```json
{
  "files.associations": {
    "*.md": "markdown"
  }
}
```

For full LSP support, you'll need a generic LSP client extension like [vscode-languageclient](https://github.com/AviAvni/vscode-languageclient) or create a simple extension.

Alternatively, create a VS Code extension with this `package.json`:

```json
{
  "name": "mdfx-lsp",
  "main": "./extension.js",
  "activationEvents": ["onLanguage:markdown"],
  "contributes": {
    "configuration": {
      "type": "object",
      "properties": {
        "mdfx.path": {
          "type": "string",
          "default": "mdfx",
          "description": "Path to mdfx executable"
        }
      }
    }
  }
}
```

With `extension.js`:

```javascript
const { LanguageClient } = require('vscode-languageclient/node');

let client;

function activate(context) {
  const serverOptions = {
    command: 'mdfx',
    args: ['lsp']
  };

  const clientOptions = {
    documentSelector: [{ scheme: 'file', language: 'markdown' }]
  };

  client = new LanguageClient('mdfx', 'mdfx LSP', serverOptions, clientOptions);
  client.start();
}

function deactivate() {
  return client?.stop();
}

module.exports = { activate, deactivate };
```

### Neovim

Using [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig):

```lua
-- Add mdfx as a custom server
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.mdfx then
  configs.mdfx = {
    default_config = {
      cmd = { 'mdfx', 'lsp' },
      filetypes = { 'markdown' },
      root_dir = function(fname)
        return lspconfig.util.find_git_ancestor(fname) or vim.fn.getcwd()
      end,
      settings = {},
    },
  }
end

lspconfig.mdfx.setup({})
```

### Helix

Add to `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "markdown"
language-servers = ["mdfx"]

[language-server.mdfx]
command = "mdfx"
args = ["lsp"]
```

### Emacs

Using [lsp-mode](https://emacs-lsp.github.io/lsp-mode/):

```elisp
(require 'lsp-mode)

(add-to-list 'lsp-language-id-configuration '(markdown-mode . "markdown"))

(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection '("mdfx" "lsp"))
  :major-modes '(markdown-mode)
  :server-id 'mdfx))

(add-hook 'markdown-mode-hook #'lsp)
```

Using [eglot](https://github.com/joaotavora/eglot):

```elisp
(require 'eglot)

(add-to-list 'eglot-server-programs
             '(markdown-mode . ("mdfx" "lsp")))

(add-hook 'markdown-mode-hook 'eglot-ensure)
```

### Sublime Text

Using [LSP](https://packagecontrol.io/packages/LSP):

1. Install the LSP package
2. Open `Preferences > Package Settings > LSP > Settings`
3. Add:

```json
{
  "clients": {
    "mdfx": {
      "enabled": true,
      "command": ["mdfx", "lsp"],
      "selector": "text.html.markdown"
    }
  }
}
```

### Zed

Add to `~/.config/zed/settings.json`:

```json
{
  "lsp": {
    "mdfx": {
      "binary": {
        "path": "mdfx",
        "arguments": ["lsp"]
      }
    }
  },
  "languages": {
    "Markdown": {
      "language_servers": ["mdfx"]
    }
  }
}
```

## Features

### Completions

The LSP provides completions for:

| Trigger | Completions |
|---------|-------------|
| `{{` | All styles, components, `glyph:`, `frame:` |
| `{{glyph:` | All 389 glyph names |
| `{{frame:` | All frame names |
| `{{ui:tech:` | All 90+ tech badge names (rust, typescript, docker, etc.) |
| `{{ui:tech:rust:` | Tech badge parameters (border, logo_size, corners, etc.) |
| `{{ui:tech:rust:logo_size=` | Parameter values (xs, sm, md, lg, xl, xxl) |
| `{{swatch:` | Palette colors |
| `bg=` / `fg=` | Palette colors |
| `style=` | Shield styles (flat, flat-square, for-the-badge, plastic, social) |

### Tech Badge Completions

Full IntelliSense support for tech badges with contextual completions:

```markdown
{{ui:tech:|}}           <!-- Shows all 90+ tech names: rust, typescript, docker... -->
{{ui:tech:rust:|}}      <!-- Shows all parameters: border, logo_size, corners... -->
{{ui:tech:rust:logo_size=|}}  <!-- Shows size presets: xs, sm, md, lg, xl, xxl -->
{{ui:tech:rust:corners=|}}    <!-- Shows corner presets: left, right, none, all -->
{{ui:tech:rust:style=|}}      <!-- Shows badge styles: flat, outline, ghost... -->
{{ui:tech:rust:bg=|}}         <!-- Shows palette colors: accent, cobalt, plum... -->
```

**Supported parameter completions:**

| Parameter | Values |
|-----------|--------|
| `logo_size` / `icon_size` | xs (10px), sm (12px), md (14px), lg (16px), xl (18px), xxl (20px) |
| `corners` | left, right, none, all |
| `chevron` | left, right, both |
| `style` | flat, flat-square, plastic, for-the-badge, social, outline, ghost |
| `border_full` / `divider` | true, false |
| `source` | shields |
| Color params (`bg`, `logo`, `border`, etc.) | All palette colors |

### Shield/SVG Style Completion

When using components that support badge styles, type `style=` to get completions:

```markdown
{{swatch:cobalt:style=|}}  <!-- cursor here triggers shield style completions -->
{{swatch:accent:style=flat-square/}}
{{ui:tech:rust:style=for-the-badge/}}
```

Available styles:
- `flat` - Rounded corners
- `flat-square` - Square corners (default)
- `for-the-badge` - Tall header bar style
- `plastic` - Glossy plastic look
- `social` - Social media style
- `outline` / `ghost` - Border-only with transparent fill

### Hover Documentation

Hover over any template element to see:

- **Glyphs**: Character preview and Unicode codepoint
- **Styles**: Name, description, aliases, supported characters
- **Components**: Type, arguments, description

### Snippets

Completions include smart snippets:

```
{{mathbold}} → {{mathbold}}${1:text}{{/mathbold}}
{{swatch:}}  → {{swatch:${1:color}/}}
{{tech:}}    → {{tech:${1:logo}/}}
```

## Troubleshooting

### LSP not starting

1. Verify mdfx is installed with LSP support:
   ```bash
   mdfx lsp --help
   ```

2. Check the path is correct:
   ```bash
   which mdfx
   ```

### No completions appearing

1. Ensure file is recognized as markdown
2. Check LSP client logs for errors
3. Verify trigger characters are configured (`{`, `:`, `=`)

### Performance

The LSP server loads the full registry (~504 glyphs, 24 styles, 29 frames) at startup. Initial completion requests may have a small delay while building the completion list.
