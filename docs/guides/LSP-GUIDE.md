# LSP Server Guide

The mdfx Language Server Protocol (LSP) server provides IDE integration with autocompletion, hover documentation, and validation for mdfx template syntax.

## Installation

The LSP server is an optional feature to keep the default install lightweight:

```bash
# Install with LSP support
cargo install mdfx-cli --features lsp
```

## Quick Start: VS Code

The fastest way to get LSP support in VS Code:

```bash
# Install the VS Code extension automatically
mdfx lsp install
```

This creates the extension at `~/.vscode/extensions/mdfx-lsp/` and installs dependencies. After installation:

1. Reload VS Code (`Cmd+Shift+P` → "Developer: Reload Window")
2. Open any `.md` file
3. Type `{{ui:tech:` to see completions

## Usage

Start the LSP server manually (for other editors):

```bash
mdfx lsp run
```

The server communicates over stdio using the LSP protocol.

## Editor Configuration

### VS Code

**Option 1: Automatic Installation (Recommended)**

```bash
mdfx lsp install --editor vscode
```

**Option 2: Manual Setup**

<details>
<summary>Click to expand manual setup instructions...</summary>

Create a VS Code extension manually with this `package.json`:

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
  },
  "dependencies": {
    "vscode-languageclient": "^9.0.1"
  }
}
```

With `extension.js`:

```javascript
const { LanguageClient, TransportKind } = require('vscode-languageclient/node');

let client;

function activate(context) {
  const serverOptions = {
    command: 'mdfx',
    args: ['lsp', 'run'],
    transport: TransportKind.stdio
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

</details>

### Neovim

Using [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig):

```lua
-- Add mdfx as a custom server
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.mdfx then
  configs.mdfx = {
    default_config = {
      cmd = { 'mdfx', 'lsp', 'run' },
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
args = ["lsp", "run"]
```

### Emacs

Using [lsp-mode](https://emacs-lsp.github.io/lsp-mode/):

```elisp
(require 'lsp-mode)

(add-to-list 'lsp-language-id-configuration '(markdown-mode . "markdown"))

(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection '("mdfx" "lsp" "run"))
  :major-modes '(markdown-mode)
  :server-id 'mdfx))

(add-hook 'markdown-mode-hook #'lsp)
```

Using [eglot](https://github.com/joaotavora/eglot):

```elisp
(require 'eglot)

(add-to-list 'eglot-server-programs
             '(markdown-mode . ("mdfx" "lsp" "run")))

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
      "command": ["mdfx", "lsp", "run"],
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
        "arguments": ["lsp", "run"]
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

### Syntax Highlighting

The VS Code extension includes TextMate grammar for mdfx template syntax highlighting:

- **Template delimiters**: `{{` and `}}` highlighted as punctuation
- **Component names**: `ui:tech`, `glyph`, `frame`, etc. highlighted as tags
- **Parameters**: `border=`, `logo_size=` highlighted as variables
- **Values**: Parameter values highlighted as strings

Syntax highlighting is automatically injected into markdown files. The grammar recognizes:

```markdown
{{ui:tech:rust:border=accent:logo_size=lg/}}  <!-- Self-closing component -->
{{mathbold}}text{{/mathbold}}                  <!-- Block style -->
{{glyph:star.filled/}}                         <!-- Glyph reference -->
```

### Semantic Tokens

Beyond TextMate grammar, the LSP provides semantic tokens for context-aware highlighting. This enables richer highlighting based on semantic understanding of your templates:

| Token Type | Description | Example |
|------------|-------------|---------|
| `namespace` | Component prefixes | `ui:tech`, `ui:live`, `ui:progress`, `glyph`, `frame`, `swatch` |
| `type` | Tech/source names | `rust`, `typescript`, `github`, `npm` |
| `parameter` | Parameter names, metrics | `border`, `logo_size`, `stars`, `downloads` |
| `string` | Parameter values, queries, args | `lg`, `true`, `50`, `owner/repo` |
| `variable` | Palette color names | `accent`, `cobalt`, `pink` |
| `keyword` | Style names, universal closer | `mathbold`, `italic`, `{{//}}` |
| `function` | Frame names | `gradient`, `box`, `parentheses` |
| `invalid` | Unknown/invalid items | Unknown tech names, invalid params/metrics |

**Full Template Coverage:**

| Template Type | Tokenized Elements |
|--------------|-------------------|
| `{{ui:tech:rust:border=accent/}}` | namespace, tech name, param names, param values |
| `{{ui:live:github:owner/repo:stars/}}` | namespace, source, query, metric |
| `{{ui:progress:50:100:fg=accent/}}` | namespace, positional args, named params |
| `{{ui:donut:75:bg=cobalt/}}` | namespace, value, color params |
| `{{ui:gauge:80:min=0:max=100/}}` | namespace, value, range params |
| `{{glyph:star.filled/}}` | namespace, glyph name |
| `{{frame:gradient}}...{{/frame:gradient}}` | namespace, frame name (both tags) |
| `{{swatch:cobalt/}}` | namespace, color name |
| `{{mathbold}}...{{/mathbold}}` | style name (both opening and closing) |
| `{{progress:50:100/}}` | component name, arguments |
| `{{//}}` | universal closer |

**Benefits over TextMate:**
- **Validation highlighting**: Invalid tech names, glyph names, metrics, and parameters are highlighted differently
- **Context awareness**: Color names are highlighted as variables when used in color parameters
- **Semantic accuracy**: Token types reflect actual meaning, not just syntax patterns
- **Complete coverage**: All template types including closing tags and universal closer

Editors that support LSP semantic tokens will use these for highlighting when available.

### Completions

The LSP provides completions for:

| Trigger | Completions |
|---------|-------------|
| `{{` | All styles, components, `glyph:`, `frame:`, `ui:` prefixes |
| `{{glyph:` | All 389 glyph names |
| `{{frame:` | All frame names |
| `{{ui:tech:` | All 90+ tech badge names (rust, typescript, docker, etc.) |
| `{{ui:tech:rust:` | Tech badge parameters (border, logo_size, corners, etc.) |
| `{{ui:tech:rust:logo_size=` | Parameter values (xs, sm, md, lg, xl, xxl) |
| `{{ui:version:` | Version badges with status detection |
| `{{ui:license:` | License badges with category coloring |
| `{{ui:row` | Horizontal badge row layout |
| `{{ui:tech-group` | Grouped badges with auto corner handling |
| `{{ui:live:` | Live data sources (github, npm, crates, pypi) |
| `{{ui:sparkline:` | Mini inline charts (line, bar, area) |
| `{{ui:rating:` | Star/heart/circle ratings |
| `{{ui:waveform:` | Audio-style waveform visualization |
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

### Color Picker

The LSP provides color picker support for hex colors in templates. When editing color parameters, you'll see:

- **Inline color swatches** next to hex values
- **Color picker UI** for visually selecting colors

Supported color formats:
- 6-character hex: `bg=FF5733`
- 3-character hex: `bg=F53` (expanded to FF5533)

Example:
```markdown
{{ui:tech:rust:bg=DEA584:logo=FFFFFF/}}
{{swatch:1a1a2e/}}
```

When you click on a color swatch or use your editor's color picker command, you can visually adjust the color and the hex value will be updated automatically.

### Code Actions / Quick Fixes

The LSP provides interactive quick fixes for common issues:

**"Add /}}" Quick Fix**

When a self-closing template is missing `/}}`:
```markdown
{{ui:tech:rust}}  ⚠️ Should be self-closing
```
Click the lightbulb or use `Cmd+.` to apply: "Add self-closing syntax '/}}'"

**"Did you mean X?" Suggestions**

For unknown tech badges or glyphs, the LSP suggests similar names using fuzzy matching:
```markdown
{{ui:tech:typescipt/}}  ⚠️ Unknown tech badge 'typescipt'
                        💡 Did you mean 'typescript'?
                        💡 Did you mean 'javascript'?
```

Suggestions are ranked by edit distance, showing the closest matches first.

### Document Symbols (Outline View)

The LSP provides document symbols for the editor's outline view. Open your editor's outline panel (Cmd+Shift+O in VS Code) to see all mdfx templates in the current file:

- **Tech badges**: Listed as `tech:rust`, `tech:typescript`, etc.
- **Live badges**: Listed as `live:github`, `live:npm`, etc.
- **Glyphs**: Listed as `glyph:star.filled`, `glyph:block.full`, etc.
- **Swatches**: Listed as `swatch:cobalt`, `swatch:pink`, etc.
- **Styles**: Listed by name (`mathbold`, `italic`, etc.)
- **Components**: Listed by name (`row`, `progress`, etc.)

### Diagnostics

The LSP validates your templates and shows errors/warnings:

**Tech Badge Validation:**
```markdown
{{ui:tech:unknown-tech/}}  ⚠️ Unknown tech badge 'unknown-tech'
```

**Glyph Validation:**
```markdown
{{glyph:invalid.name/}}  ⚠️ Unknown glyph 'invalid.name'
```

**Live Badge Validation:**
```markdown
{{ui:live:invalid:query:metric/}}  ❌ Unknown live source 'invalid'
{{ui:live:github:owner/repo:bad/}}  ⚠️ Unknown metric 'bad' for source 'github'
```

**Tag Pair Validation:**
```markdown
{{bold}}text{{/italic}}      ❌ Mismatched tags - opened 'bold', closed 'italic'
{{mathbold}}text             ❌ Unclosed tag 'mathbold' - missing '{{/mathbold}}' or '{{//}}'
{{/bold}}                    ❌ Extra closing tag with no open tag
{{//}}                       ❌ Universal closer with no open tag
```

**Self-Closing Warnings:**
```markdown
{{ui:tech:rust}}             ⚠️ Template should be self-closing with '/}}'
{{glyph:star}}               ⚠️ Template should be self-closing with '/}}'
```

**Malformed Template Detection:**
```markdown
{{blackboard}text            ❌ Malformed template '{{blackboard' - missing closing '}}'
```

Diagnostics appear inline in your editor and in the problems panel.

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
   mdfx lsp run --help
   ```

2. Check the path is correct:
   ```bash
   which mdfx
   ```

3. For VS Code, try reinstalling the extension:
   ```bash
   mdfx lsp install --editor vscode
   ```

### No completions appearing

1. Ensure file is recognized as markdown
2. Check LSP client logs for errors
3. Verify trigger characters are configured (`{`, `:`, `=`)

### Performance

The LSP server is optimized for fast responses:

- **Cached completions**: All completion items (glyphs, styles, frames, tech names, etc.) are pre-built at server startup
- **Shared parameter definitions**: Tech badge and live source parameters use a single source of truth shared with the renderer
- **Byte-level template parsing**: Template detection uses direct byte manipulation instead of regex
- **Optimized fuzzy matching**: "Did you mean?" suggestions use bounded Levenshtein distance with:
  - Two-row O(n) space algorithm instead of O(m×n) matrix
  - Early termination when distance exceeds threshold
  - Length difference pre-check to skip obviously dissimilar names

The registry load (~504 glyphs, 24 styles, 29 frames, 90+ tech badges) happens at startup with no impact on completion response times.
