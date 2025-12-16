# Template Syntax Reference

**Canonical syntax specification for mdfx templates**

---

## Table of Contents

- [Overview](#overview)
- [Basic Syntax](#basic-syntax)
  - [Template Delimiters](#template-delimiters)
  - [Tag Types](#tag-types)
  - [Parameters](#parameters)
- [Component Templates](#component-templates)
- [Style Templates](#style-templates)
- [Frame Templates](#frame-templates)
- [Primitive Templates](#primitive-templates)
- [Partial Templates](#partial-templates)
- [Advanced Features](#advanced-features)
  - [Nesting and Composition](#nesting-and-composition)
  - [Post-Processing](#post-processing)
- [Edge Cases](#edge-cases)
- [Error Messages](#error-messages)
- [Formal Grammar](#formal-grammar)

---

## Overview

mdfx templates use a double-brace syntax similar to Handlebars or Mustache, but optimized for markdown processing. Templates are embedded in markdown files and processed by the `TemplateParser`.

**Design Principles:**
- Human-readable syntax
- Minimal typing overhead
- Clear error messages
- Code block preservation
- Composable and nestable

---

## Basic Syntax

### Template Delimiters

All templates use double-brace delimiters:

```
Opening:  {{
Closing:  }}
```

**Important:** Templates are case-sensitive. `{{mathbold}}` works, `{{MathBold}}` does not.

### Tag Types

#### Self-Closing Tags

For templates without content:

```markdown
{{template_name/}}
{{template_name:arg1/}}
{{template_name:arg1:arg2/}}
```

**Rules:**
- End with `/}}`
- Cannot have closing tag
- Used for: swatches, tech badges, status indicators

**Examples:**
```markdown
{{ui:tech:rust/}}
{{ui:swatch:accent/}}
{{ui:section:Installation/}}
{{ui:statusitem:Build:success:passing/}}
```

#### Block Tags

For templates with content:

```markdown
{{template_name}}CONTENT{{/template_name}}
{{template_name:arg}}CONTENT{{/closer}}
```

**Rules:**
- Opening tag: `{{name}}` or `{{name:params}}`
- Closing tag: `{{/name}}` (exact match) or `{{/ui}}` (component generic closer)
- Content can be multiline
- Content can contain other templates (nesting)

**Examples:**
```markdown
{{mathbold}}HELLO{{/mathbold}}
{{ui:header}}TITLE{{/ui}}
{{ui:callout:warning}}Message{{/ui}}
{{frame:gradient}}{{script}}Text{{/script}}{{/frame}}
```

#### Closing Tag Rules

| Template Type | Closer |
|---------------|--------|
| Styles | `{{/style_name}}` (specific) |
| Frames | `{{/}}` or `{{/frame}}` |
| Badges | `{{/badge}}` (generic) |
| UI Components | `{{/ui}}` (generic) |

**Universal closer `{{/}}`:**
- Frames support `{{/}}` as a shorthand for `{{/frame}}`
- Less typing: `{{frame:gradient}}Title{{/}}` instead of `{{frame:gradient}}Title{{/frame}}`
- Works correctly with nesting

**Close-all `{{//}}`:**
- Closes all open tags at once (frames, styles, UI components)
- Perfect for nested content: `{{fr:gradient}}{{mathbold}}Title{{//}}`
- No need to count closing tags
- Closes in reverse order (LIFO)

**Why generic closers?**
- Less typing: `{{/ui}}` instead of `{{/callout-github}}`
- Easier refactoring: change opening tag without updating closer
- Cleaner markdown: more scannable

### Parameters

#### Positional Parameters

Arguments separated by colons (`:`)

```markdown
{{template:arg1:arg2:arg3}}
```

**Used by:**
- UI components: `{{ui:component:arg1:arg2}}`
- All self-closing tags

**Examples:**
```markdown
{{ui:tech:rust/}}                              ← 1 positional arg
{{ui:callout:warning}}Message{{/ui}}           ← 1 positional arg
{{ui:statusitem:Label:Level:Text/}}            ← 3 positional args
{{ui:section:Getting Started/}}                ← 1 positional arg
```

**Rules:**
- No spaces around colons
- No equals signs
- Args are strings (no quotes needed)
- Empty args not allowed: `{{ui:tech:/}}` is invalid

#### Named Parameters

Key-value pairs with equals (`=`)

```markdown
{{template:key=value}}
{{template:key1=value1:key2=value2}}
```

**Used by:**
- Style templates: `separator=`, `spacing=`
- Primitive templates: `color=`, `colors=`
- UI components: `style=` (badge style control)

**Examples:**
```markdown
{{mathbold:separator=dot}}TEXT{{/mathbold}}
{{mathbold:separator=dot:spacing=1}}TEXT{{/mathbold}}
{{shields:block:color=F41C80:style=flat-square/}}
{{ui:swatch:F41C80:style=flat/}}
{{ui:tech:rust:style=for-the-badge/}}
```

**Rules:**
- No spaces around `=`
- Value cannot contain `:`, `/`, or `}`
- Parameters are order-independent: `separator=dot:spacing=1` equals `spacing=1:separator=dot`

#### Parameter Substitution

In component definitions (`components.json`), parameters are substituted:

```json
{
  "template": "## $1",
  "args": ["title"]
}
```

**Substitution Variables:**
- `$1`, `$2`, `$3`, ... → Positional arguments
- `$content` → Block content
- No named parameter substitution (yet)

---

## Component Templates

**Namespace:** `{{ui:*}}`

Components are high-level semantic elements that expand to other templates.

### Syntax

```markdown
{{ui:component_name/}}                         ← Self-closing
{{ui:component_name:arg1/}}                    ← With positional args
{{ui:component_name:arg1:arg2}}content{{/ui}}  ← Block with args
{{ui:component_name:arg:style=STYLE/}}         ← With badge style control
```

### Badge Style Control

All primitive-based components (swatch, tech, status) support optional `style=` parameter:

| Style | Appearance | Shields.io | SVG |
|-------|------------|-----------|-----|
| `flat` | Rounded corners | ✓ | rx=3, h=20 |
| `flat-square` | Sharp corners (default) | ✓ | rx=0, h=20 |
| `for-the-badge` | Tall blocks | ✓ | rx=3, h=28 |
| `plastic` | Shiny gradient | ✓ | rx=3, gradient overlay |
| `social` | Very rounded | ✓ | rx=10, h=20 |

**Syntax:**
```markdown
{{ui:swatch:F41C80:style=flat/}}              ← Rounded corners
{{ui:swatch:F41C80:style=flat-square/}}       ← Sharp corners
{{ui:swatch:F41C80:style=for-the-badge/}}     ← Tall block
{{ui:swatch:F41C80:style=plastic/}}           ← Shiny effect
{{ui:swatch:F41C80:style=social/}}            ← Very rounded
{{ui:swatch:F41C80/}}                         ← Defaults to flat-square
```

**Applies to:**
- `{{ui:swatch:COLOR:style=STYLE/}}`
- `{{ui:tech:LOGO:style=STYLE/}}`
- `{{ui:status:LEVEL:style=STYLE/}}`

**Mix styles for design variety ("Minecraft bricks"):**
```markdown
{{ui:swatch:FF0000:style=flat/}}{{ui:swatch:00FF00:style=for-the-badge/}}{{ui:swatch:0000FF:style=plastic/}}
```

### Shipped Components

| Component | Args | Type | Example |
|-----------|------|------|---------|
| `swatch` | color | self-closing | `{{ui:swatch:accent/}}` |
| `tech` | logo_name | self-closing | `{{ui:tech:rust/}}` |
| `status` | level | self-closing | `{{ui:status:success/}}` |
| `header` | none | block | `{{ui:header}}TITLE{{/ui}}` |
| `callout` | type | block | `{{ui:callout:warning}}Msg{{/ui}}` |
| `section` | title | self-closing | `{{ui:section:Features/}}` |
| `callout-github` | type | block | `{{ui:callout-github:info}}Msg{{/ui}}` |
| `statusitem` | label, level, text | self-closing | `{{ui:statusitem:Build:success:passing/}}` |

### Component-Specific Rules

**swatch:**
```markdown
{{ui:swatch:COLOR/}}
{{ui:swatch:COLOR:style=STYLE/}}
{{ui:swatch:COLOR:label=TEXT:icon=LOGO/}}
```
- 1 arg: color name (palette) or 6-digit hex
- Optional parameters:
  - `style=` - Badge style (flat, flat-square, for-the-badge, plastic, social)
  - `opacity=` - Opacity 0.0-1.0 (SVG-only)
  - `width=` - Width in pixels (default: 20)
  - `height=` - Height in pixels (style-dependent)
  - `border=` - Border color (hex or palette, SVG-only)
  - `border_width=` - Border width in pixels (SVG-only)
  - `label=` - Text label inside swatch
  - `label_color=` - Label text color (default: white)
  - `icon=` - Simple Icons logo name (e.g., "rust")
  - `icon_color=` - Icon color (default: white)
- Examples:
  - `{{ui:swatch:accent/}}` - Basic swatch
  - `{{ui:swatch:FF6B35:style=flat/}}` - With style
  - `{{ui:swatch:cobalt:label=v1/}}` - With label
  - `{{ui:swatch:000000:icon=rust:icon_color=FFFFFF/}}` - With icon

**tech:**
```markdown
{{ui:tech:LOGO_NAME/}}
{{ui:tech:LOGO_NAME:style=STYLE/}}
```
- 1 arg: Simple Icons slug (lowercase)
- Optional: `style=` (flat, flat-square, for-the-badge, plastic, social)
- Examples: `{{ui:tech:rust/}}`, `{{ui:tech:python:style=plastic/}}`

**status:**
```markdown
{{ui:status:LEVEL/}}
{{ui:status:LEVEL:style=STYLE/}}
```
- 1 arg: `success`, `warning`, `error`, `info`
- Optional: `style=` (flat, flat-square, for-the-badge, plastic, social)
- Renders colored block badge

**header:**
```markdown
{{ui:header}}CONTENT{{/ui}}
```
- No args
- Content is transformed to mathbold with dot separators
- Wrapped in gradient frame

**callout:**
```markdown
{{ui:callout:TYPE}}CONTENT{{/ui}}
```
- 1 arg: `success`, `warning`, `error`, `info`
- Uses inline frame (not blockquote)
- Content can be multiline

**section:**
```markdown
{{ui:section:TITLE/}}
```
- 1 arg: section title (becomes `## TITLE`)
- GitHub Blocks feature

**callout-github:**
```markdown
{{ui:callout-github:TYPE}}
CONTENT
{{/ui}}
```
- 1 arg: `success`, `warning`, `error`, `info`
- Uses blockquote post-processing (every line gets `> `)
- Multiline content supported
- Empty lines become `>`
- GitHub Blocks feature

**statusitem:**
```markdown
{{ui:statusitem:LABEL:LEVEL:TEXT/}}
```
- 3 args: label, level, text
- Example: `{{ui:statusitem:Build:success:passing/}}`
- Compose multiple with ` · ` separator for status rows
- GitHub Blocks feature

---

## Style Templates

**Namespace:** Direct style names (no prefix)

Converts text to Unicode character styles.

### Syntax

```markdown
{{style_name}}TEXT{{/style_name}}
{{style_name:separator=SEP}}TEXT{{/style_name}}
{{style_name:spacing=N}}TEXT{{/style_name}}
{{style_name:separator=SEP:spacing=N}}TEXT{{/style_name}}
```

### Parameters

**separator=** - Character between letters
- Named separators: `dot`, `bullet`, `dash`, `bolddash`, `arrow`, `star`, `diamond`, `square`, `circle`, `pipe`, `slash`, `tilde`
- Direct Unicode: Any single grapheme cluster (e.g., `⚡`, `→`, `👉`)
- Examples: `separator=dot`, `separator=⚡`

**spacing=** - Number of spaces between characters
- Value: 0-9 (single digit)
- Examples: `spacing=1`, `spacing=3`

**Rules:**
- Cannot use both separator and spacing (mutually exclusive)
- Separator cannot be `:`, `/`, or `}` (template delimiters)
- Whitespace in separator is trimmed

### Available Styles (19 total)

**Bold & Emphasis:**
- `mathbold` - 𝐇𝐄𝐋𝐋𝐎 (Mathematical bold serif)
- `fullwidth` - ＨＥＬＬＯ (Wide characters)
- `sans-serif-bold` - 𝗛𝗘𝗟𝗟𝗢 (Modern bold)
- `sans-serif-bold-italic` - 𝙃𝙀𝙇𝙇𝙊 (Bold italic sans)

**Boxed:**
- `negative-squared` - 🅷🅴🅻🅻🅾 (White on black squares)
- `negative-circled` - 🅗🅔🅛🅛🅞 (White on black circles)
- `squared-latin` - 🄷🄴🄻🄻🄾 (Letters in boxes)
- `circled-latin` - Ⓗⓔⓛⓛⓞ (Letters in circles)

**Elegant:**
- `script` - ℋℯ𝓁𝓁ℴ (Calligraphic)
- `bold-script` - 𝓗𝓮𝓵𝓵𝓸 (Heavy cursive)
- `fraktur` - ℌ𝔢𝔩𝔩𝔬 (Gothic blackletter)
- `bold-fraktur` - 𝕳𝖊𝖑𝖑𝖔 (Heavy Gothic)
- `italic` - 𝐻𝑒𝑙𝑙𝑜 (Slanted)
- `bold-italic` - 𝑯𝒆𝒍𝒍𝒐 (Bold slanted)
- `small-caps` - ʜᴇʟʟᴏ (Understated capitals)

**Technical:**
- `monospace` - 𝙷𝚎𝚕𝚕𝚘 (Fixed-width)
- `double-struck` - ℍ𝕖𝕝𝕝𝕠 (Outline/blackboard)
- `sans-serif` - 𝖧𝖾𝗅𝗅𝗈 (Clean modern)
- `sans-serif-italic` - 𝘏𝘦𝘭𝘭𝘰 (Slanted modern)

### Examples

```markdown
{{mathbold}}HELLO{{/mathbold}}
→ 𝐇𝐄𝐋𝐋𝐎

{{mathbold:separator=dot}}TITLE{{/mathbold}}
→ 𝐓·𝐈·𝐓·𝐋·𝐄

{{script:spacing=2}}Elegant{{/script}}
→ 𝐸  𝓁  𝑒  𝑔  𝒶  𝓃  𝓉

{{fullwidth:separator=⚡}}POWER{{/fullwidth}}
→ Ｐ⚡Ｏ⚡Ｗ⚡Ｅ⚡Ｒ
```

---

## Frame Templates

**Namespace:** `{{frame:*}}` or `{{fr:*}}` (shorthand)

Adds decorative prefix/suffix around content.

### Syntax

```markdown
{{frame:frame_type}}CONTENT{{/}}
{{fr:frame_type}}CONTENT{{/}}              <!-- shorthand -->
{{fr:glyph:NAME}}CONTENT{{/}}
{{fr:glyph:NAME*COUNT/pad=VALUE}}CONTENT{{/}}
{{fr:a}}{{fr:b}}{{fr:c}}NESTED{{//}}       <!-- close-all -->
{{fr:frame_type:CONTENT/}}                 <!-- self-closing -->
{{fr:outer+inner}}CONTENT{{/}}             <!-- frame combo -->
{{fr:frame/separator=X}}CONTENT{{/}}       <!-- with separator -->
{{fr:frame/spacing=N}}CONTENT{{/}}         <!-- with spacing -->
{{fr:frame/reverse}}CONTENT{{/}}           <!-- reverse (swap prefix/suffix) -->
{{fr:frame*N}}CONTENT{{/}}                 <!-- repeat pattern N times -->
```

### Self-Closing Frames

For short inline content, use the self-closing syntax:

```markdown
{{fr:STYLE:CONTENT/}}
```

This is equivalent to `{{fr:STYLE}}CONTENT{{/}}` but more compact.

**Examples:**
```markdown
{{fr:gradient:Title/}}                → ▓▒░ Title ░▒▓
{{fr:star:VIP/}}                      → ★ VIP ☆
{{fr:glyph:diamond*2:Gem/}}           → ◆◆ Gem ◆◆
{{fr:glyph:star*3/pad=0:Tight/}}      → ★★★Tight★★★
```

**Note:** The content is everything after the last `:` and before `/}}`. This allows glyph frames with modifiers to work correctly.

### Available Frames (27 predefined + unlimited glyph frames)

**Gradient Frames:**
- `gradient` - ▓▒░ TEXT ░▒▓
- `gradient-light` - ▒░ TEXT ░▒
- `gradient-reverse` - ░▒▓ TEXT ▓▒░
- `gradient-wave` - ▓▒░ TEXT ▒░▓ (alternate mode: rotated suffix)

**Solid Frames:**
- `solid-left` - █▌TEXT
- `solid-right` - TEXT▐█
- `solid-both` - █▌TEXT▐█

**Line Frames:**
- `line-light` - ─── TEXT ───
- `line-bold` - ━━━ TEXT ━━━
- `line-double` - ═══ TEXT ═══
- `line-dashed` - ╌╌╌ TEXT ╌╌╌

**Block Frames:**
- `block-top` - ▀▀▀ TEXT ▀▀▀
- `block-bottom` - ▄▄▄ TEXT ▄▄▄

**Symbol Frames (asymmetric):**
- `star` - ★ TEXT ☆
- `diamond` - ◆ TEXT ◇
- `triangle-right` - ▶ TEXT ◀
- `finger` - ☞ TEXT ☜

**Bracket Frames:**
- `lenticular` - 【TEXT】
- `angle` - 《TEXT》
- `guillemet` - « TEXT »
- `guillemet-single` - ‹ TEXT ›
- `heavy-quote` - ❝TEXT❞

**Arc Frames:**
- `arc-top` - ╭ TEXT ╮
- `arc-bottom` - ╰ TEXT ╯

**Alert Frames (with emoji prefixes):**
- `alert-warning` - ⚠️ TEXT
- `alert-info` - ℹ️ TEXT
- `alert-success` - ✅ TEXT
- `alert-error` - ❌ TEXT
- `color-accent` - [accent swatch] TEXT

### Glyph Frames

Create symmetric frames from any glyph:

```markdown
{{frame:glyph:NAME}}TEXT{{/}}           → GLYPH TEXT GLYPH
{{frame:glyph:NAME*3}}TEXT{{/}}         → GLYPH×3 TEXT GLYPH×3
{{frame:glyph:NAME*3/pad=0}}TEXT{{/}}   → tight (no spacing)
{{frame:glyph:NAME/pad=·}}TEXT{{/}}     → custom padding char
```

**Examples:**
- `{{frame:glyph:bullet}}Item{{/}}` → • Item •
- `{{frame:glyph:star*3}}Title{{/}}` → ★★★ Title ★★★
- `{{frame:glyph:arrow*2/pad=0}}Go{{/}}` → →→Go→→

### Frame Combos

Combine multiple frames with `+` for nested effects:

```markdown
{{fr:outer+inner}}CONTENT{{/}}          → Nested frames
{{fr:gradient+star}}TITLE{{/}}          → ▓▒░ ★ TITLE ☆ ░▒▓
{{fr:gradient+star+diamond}}VIP{{/}}    → ▓▒░ ★ ◆ VIP ◇ ☆ ░▒▓
```

**Order:** Outer frames wrap inner frames. Prefix builds left-to-right, suffix builds right-to-left.

**Equivalent to:**
```markdown
{{fr:gradient}}{{fr:star}}TITLE{{/}}{{/}}
```

### Frame Modifiers

Frames support separator and spacing modifiers:

**Separator (`/separator=X`):**
```markdown
{{fr:gradient/separator=·}}Title{{/}}   → ▓·▒·░ Title ░·▒·▓
{{fr:gradient/separator=dot}}Title{{/}} → ▓·▒·░ Title ░·▒·▓
```

Named separators: `dot`, `dash`, `space`, `pipe`, `colon`

**Spacing (`/spacing=N`):**
```markdown
{{fr:gradient/spacing=1}}Title{{/}}     → ▓ ▒ ░ Title ░ ▒ ▓
{{fr:gradient/spacing=2}}Wide{{/}}      → ▓  ▒  ░ Wide ░  ▒  ▓
```

**Combined:**
```markdown
{{fr:gradient/separator=·/spacing=1}}X{{/}} → ▓ · ▒ · ░ X ░ · ▒ · ▓
```

**Reverse (`/reverse`):**
```markdown
{{fr:gradient/reverse}}Title{{/}}          → ░▒▓ Title ▓▒░
{{fr:star/rev}}VIP{{/}}                    → ☆ VIP ★
```

Swaps prefix and suffix of any frame. Alias: `/rev`

**Count (`*N`):**
```markdown
{{fr:star*3}}Title{{/}}                    → ★★★ Title ☆☆☆
{{fr:gradient*2}}X{{/}}                    → ▓▒░▓▒░ X ░▒▓░▒▓
{{fr:star*3/separator=·}}Title{{/}}        → ★·★·★ Title ☆·☆·☆
```

Repeats frame pattern N times (max 20). Works with predefined frames.

**Glyph frames also support modifiers:**
```markdown
{{fr:glyph:star*3/separator=·}}Title{{/}}  → ★·★·★ Title ★·★·★
{{fr:glyph:diamond*4/spacing=1}}Gem{{/}}   → ◆ ◆ ◆ ◆ Gem ◆ ◆ ◆ ◆
```

### Examples

```markdown
{{fr:gradient}}TITLE{{/}}
→ ▓▒░ TITLE ░▒▓

{{fr:solid-left}}WARNING{{/}}
→ █▌ WARNING

{{fr:glyph:diamond*2}}VIP{{/}}
→ ◆◆ VIP ◆◆
```

---

## Primitive Templates

**Namespace:** `{{shields:*}}`

Low-level shield rendering (escape hatch for advanced users).

### Syntax

```markdown
{{shields:type:param1=value:param2=value/}}
```

### Available Primitives

**block** - Single color block
```markdown
{{shields:block:color=F41C80:style=flat-square/}}
```
- `color=` - 6-digit hex or palette name
- `style=` - shields.io style (default: flat-square)

**twotone** - Two-color split
```markdown
{{shields:twotone:left_color=FF6B35:right_color=4A9EFF:style=flat-square/}}
```

**bar** - Multi-color segments
```markdown
{{shields:bar:colors=success,warning,error:style=flat-square/}}
```
- `colors=` - Comma-separated list
- `separator=` - Optional separator between blocks (e.g., ` ` for space, `dot`, `·`)

```markdown
{{shields:bar:colors=accent,success:style=flat-square:separator= /}}
```

**icon** - Simple Icons logo
```markdown
{{shields:icon:logo=rust:color=FF6B35:style=flat-square/}}
```
- `logo=` - Simple Icons slug
- `color=` - Logo fill color

### When to Use Primitives

**Prefer UI components:**
- `{{ui:tech:rust/}}` instead of `{{shields:icon:logo=rust:...}}`
- `{{ui:swatch:accent/}}` instead of `{{shields:block:color=accent:...}}`

**Use primitives when:**
- Component doesn't exist for your use case
- Need precise control over colors/styles
- Building custom components in `components.json`

---

## Advanced Features

### Nesting and Composition

Templates can be nested to create complex effects:

```markdown
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
→ ▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓
```

**Rules:**
- Inner templates processed first (inside-out)
- No depth limit (but keep readable)
- Code blocks are preserved (see Edge Cases)

**Common Patterns:**

1. **Styled frame content:**
```markdown
{{frame:gradient}}{{mathbold}}HELLO{{/mathbold}}{{/frame}}
```

2. **Component with styled content:**
```markdown
{{ui:header}}{{script}}Elegant{{/script}}{{/ui}}
```

3. **Multiple styles:**
```markdown
{{frame:line-bold}}{{mathbold:separator=arrow}}CODE{{/mathbold}}{{/frame}}
```

### Post-Processing

Some components apply transformations after template expansion.

**PostProcess enum:**
- `None` - No post-processing (default)
- `Blockquote` - Prefix every line with `"> "`

**Components with post-processing:**
- `callout-github` - Uses `Blockquote` post-processor

**How it works:**

```markdown
{{ui:callout-github:info}}
Line 1
Line 2
{{/ui}}
```

**Step 1 - Expand template:**
```
{{ui:status:info/}} **Note**
Line 1
Line 2
```

**Step 2 - Apply blockquote post-processing:**
```
> {{ui:status:info/}} **Note**
> Line 1
> Line 2
```

**Step 3 - Render primitives:**
```
> 🔵 **Note**
> Line 1
> Line 2
```

**Empty lines:** Become `>` (no trailing space) for GitHub compatibility.

---

## Edge Cases

### Code Block Preservation

Templates inside code blocks are NOT processed:

````markdown
Here's an example:
```markdown
{{mathbold}}This is not processed{{/mathbold}}
```
````

**Rules:**
- Fenced code blocks (` ``` `) are preserved
- Indented code blocks (4 spaces) are preserved
- Inline code (` ` `) is preserved

**Parser implementation:** State machine tracks code block boundaries.

### Inline Code

Templates in inline code are preserved:

```markdown
Use `{{mathbold}}TEXT{{/mathbold}}` in your markdown.
```

Output:
```
Use `{{mathbold}}TEXT{{/mathbold}}` in your markdown.
```

### Whitespace Handling

**Trailing newlines:** Preserved
```markdown
{{ui:swatch:accent/}}

Next paragraph
```

**Empty lines in block content:** Preserved
```markdown
{{ui:callout-github:info}}
First paragraph

Second paragraph
{{/ui}}
```

**Indentation:** Preserved in multiline content
```markdown
{{ui:callout-github:warning}}
- Item 1
- Item 2
  - Nested
{{/ui}}
```

### Special Characters

**Characters not allowed in parameters:**
- `:` (delimiter)
- `/` (closing delimiter)
- `}` (closing brace)

**Workaround:** Use component templates or primitives.

### Unclosed Tags

Error if tag not closed:

```markdown
{{mathbold}}HELLO
```

**Error:**
```
Error: Unclosed tag: {{mathbold}} (expected {{/mathbold}})
```

### Mismatched Tags

Error if closer doesn't match opener:

```markdown
{{mathbold}}HELLO{{/script}}
```

**Error:**
```
Error: Mismatched tags: opened {{mathbold}}, closed with {{/script}}
```

---

## Error Messages

mdfx provides precise error messages with context:

### Unclosed Tag

```
Error: Unclosed tag: {{mathbold}} (expected {{/mathbold}})
```

**Fix:** Add closing tag `{{/mathbold}}`

### Mismatched Tags

```
Error: Mismatched tags: opened {{mathbold}}, closed with {{/script}}
```

**Fix:** Use matching closer `{{/mathbold}}`

### Unknown Component

```
Error: Component not found: 'foo'
```

**Fix:** Check spelling or run `mdfx list` to see available components

### Unknown Style

```
Error: Style not found: 'boldmath' (did you mean 'mathbold'?)
```

**Fix:** Use correct style name. Check `mdfx list` for available styles.

### Unknown Separator

```
Error: Separator 'dots' not found (did you mean 'dot'?)
Available: dot, bullet, dash, bolddash, arrow, ...
```

**Fix:** Use correct separator name or direct Unicode character

### Invalid Self-Closing Syntax

```
Error: Invalid syntax: {{ui:swatch:accent}}{{/ui}} (expected {{ui:swatch:accent/}})
```

**Fix:** Use self-closing syntax: `{{ui:swatch:accent/}}`

---

## Formal Grammar

**EBNF-style grammar specification:**

```ebnf
(* Top-level *)
document = { text | template | code_block } ;

(* Templates *)
template = self_closing_template | block_template ;

self_closing_template = "{{" template_name [ parameters ] "/}}" ;

block_template = opening_tag content closing_tag ;
opening_tag = "{{" template_name [ parameters ] "}}" ;
closing_tag = "{{/" closer_name "}}" ;
content = { text | template | code_block } ;

(* Template names *)
template_name = identifier [ ":" identifier ] ;  (* e.g., "ui:swatch" *)
identifier = letter { letter | digit | "-" | "_" } ;

(* Parameters *)
parameters = ":" ( positional_params | named_params ) ;
positional_params = param_value { ":" param_value } ;
named_params = named_param { ":" named_param } ;
named_param = identifier "=" param_value ;
param_value = { character - ( ":" | "/" | "}" ) } ;

(* Code blocks *)
code_block = fenced_code_block | indented_code_block | inline_code ;
fenced_code_block = "```" [ language ] newline { line } "```" ;
indented_code_block = { "    " line } ;
inline_code = "`" { character - "`" } "`" ;

(* Basic elements *)
text = { character - ( "{{" | "`" ) } ;
character = ? any Unicode character ? ;
letter = ? Unicode letter ? ;
digit = ? Unicode digit ? ;
newline = "\n" | "\r\n" ;
line = { character } newline ;
```

**Key Grammar Rules:**

1. **Greedy matching:** Parser consumes longest valid token
2. **Code blocks:** Take precedence over templates (checked first)
3. **Nesting:** Block templates can contain other templates recursively
4. **Whitespace:** Significant inside param_value, insignificant around delimiters
5. **Case sensitivity:** All identifiers are case-sensitive

---

## Partial Templates

**Namespace:** `{{partial:*}}`

User-defined reusable template snippets loaded from `.mdfx.json` configuration.

### Syntax

```markdown
{{partial:name}}CONTENT{{/partial}}    ← Block with content
{{partial:name/}}                       ← Self-closing (empty content)
```

### Configuration

Define partials in `.mdfx.json`:

```json
{
  "partials": {
    "hero": {
      "template": "{{frame:gradient}}{{mathbold}}$1{{/mathbold}}{{/frame}}",
      "description": "Hero header with gradient frame"
    },
    "techstack": {
      "template": "{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}}"
    }
  }
}
```

### Content Substitution

Use `$1` or `$content` as placeholders for the content between tags:

```markdown
{{partial:hero}}MY TITLE{{/partial}}
→ {{frame:gradient}}{{mathbold}}MY TITLE{{/mathbold}}{{/frame}}
→ ▓▒░ 𝐌𝐘 𝐓𝐈𝐓𝐋𝐄 ░▒▓
```

### Rules

- Partial names: alphanumeric, hyphens, underscores
- Templates can contain any valid mdfx syntax
- Partials are expanded first, then the result is processed
- Use `{{/partial}}` or `{{/}}` as closing tag

### Use Cases

- Project-wide branding (consistent headers, callouts)
- Reusable component patterns
- Team style guidelines

---

## Quick Reference

| Template Type | Self-Closing | Block | Closer | Example |
|---------------|--------------|-------|--------|---------|
| Component | Yes | Yes | `{{/ui}}` or `{{/}}` | `{{ui:swatch:accent/}}` |
| Style | No | Yes | `{{/style}}` | `{{mathbold}}TEXT{{/mathbold}}` |
| Frame | No | Yes | `{{/frame}}` | `{{frame:gradient}}TEXT{{/frame}}` |
| Badge | No | Yes | `{{/badge}}` | `{{badge:circle}}1{{/badge}}` |
| Partial | Yes | Yes | `{{/partial}}` | `{{partial:hero}}TEXT{{/partial}}` |
| Primitive | Yes | No | N/A | `{{shields:block:color=F41C80/}}` |

**Parameter Syntax:**

| Type | Syntax | Example |
|------|--------|---------|
| Positional | `:arg` | `{{ui:tech:rust/}}` |
| Named | `:key=value` | `{{mathbold:separator=dot}}` |
| Frame Modifier | `/key=value` | `{{fr:gradient/spacing=1}}` |
| Frame Reverse | `/reverse` | `{{fr:gradient/reverse}}` |
| Frame Count | `*N` | `{{fr:star*3}}` |
| Frame Combo | `+` | `{{fr:gradient+star}}` |
| Mixed | N/A | Not supported |

---

## See Also

- [API Guide](API-GUIDE.md) - Component reference with Rust examples
- [Architecture](ARCHITECTURE.md) - System design and implementation
- [State Machine Guide](STATE-MACHINE-GUIDE.md) - Parser internals
- [README](../README.md) - User guide and examples

---

**Last Updated:** v1.0.0 (2025-12-14)
