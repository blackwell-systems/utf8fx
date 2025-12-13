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
- [Badge Templates](#badge-templates)
- [Primitive Templates](#primitive-templates)
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
- Used for: dividers, swatches, tech badges, status indicators

**Examples:**
```markdown
{{ui:divider/}}
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
| Frames | `{{/frame}}` (generic) |
| Badges | `{{/badge}}` (generic) |
| UI Components | `{{/ui}}` (generic) |

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
- Primitive templates: `color=`, `style=`, `colors=`

**Examples:**
```markdown
{{mathbold:separator=dot}}TEXT{{/mathbold}}
{{mathbold:separator=dot:spacing=1}}TEXT{{/mathbold}}
{{shields:block:color=F41C80:style=flat-square/}}
```

**Rules:**
- No spaces around `=`
- Value cannot contain `:`, `/`, or `}`
- Parameters are order-independent: `separator=dot:spacing=1` equals `spacing=1:separator=dot`

#### Parameter Substitution

In component definitions (`components.json`), parameters are substituted:

```json
{
  "template": "## $1\n{{ui:divider/}}",
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
```

### Shipped Components

| Component | Args | Type | Example |
|-----------|------|------|---------|
| `divider` | none | self-closing | `{{ui:divider/}}` |
| `swatch` | color | self-closing | `{{ui:swatch:accent/}}` |
| `tech` | logo_name | self-closing | `{{ui:tech:rust/}}` |
| `status` | level | self-closing | `{{ui:status:success/}}` |
| `header` | none | block | `{{ui:header}}TITLE{{/ui}}` |
| `callout` | type | block | `{{ui:callout:warning}}Msg{{/ui}}` |
| `section` | title | self-closing | `{{ui:section:Features/}}` |
| `callout-github` | type | block | `{{ui:callout-github:info}}Msg{{/ui}}` |
| `statusitem` | label, level, text | self-closing | `{{ui:statusitem:Build:success:passing/}}` |

### Component-Specific Rules

**divider:**
```markdown
{{ui:divider/}}
```
- No arguments
- Expands to shields.io bar with theme colors

**swatch:**
```markdown
{{ui:swatch:COLOR/}}
```
- 1 arg: color name (palette) or 6-digit hex
- Examples: `{{ui:swatch:accent/}}`, `{{ui:swatch:FF6B35/}}`

**tech:**
```markdown
{{ui:tech:LOGO_NAME/}}
```
- 1 arg: Simple Icons slug (lowercase)
- Examples: `{{ui:tech:rust/}}`, `{{ui:tech:python/}}`

**status:**
```markdown
{{ui:status:LEVEL/}}
```
- 1 arg: `success`, `warning`, `error`, `info`
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
- Automatically adds divider below
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

### Available Styles

**Bold & Emphasis:**
- `mathbold` - 𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓
- `fullwidth` - ＢＬＡＣＫＤＯＴ
- `sans-serif-bold` - 𝗕𝗟𝗔𝗖𝗞𝗗𝗢𝗧
- `sans-serif-bold-italic` - 𝘽𝙇𝘼𝘾𝙆𝘿𝙊𝙏

**Boxed:**
- `negative-squared` - 🅱🅻🅰🅲🅺🅳🅾🆃
- `negative-circled` - 🅑🅛🅐🅒🅚🅓🅞🅣
- `squared-latin` - 🄱🄻🄰🄲🄺🄳🄾🅃
- `circled-latin` - Ⓑⓛⓐⓒⓚⓓⓞⓣ

**Elegant:**
- `script` - 𝐵𝐿𝒜𝒞𝒦𝒟𝒪𝒯
- `bold-script` - 𝓑𝓛𝓐𝓒𝓚𝓓𝓞𝓣
- `fraktur` - 𝔅𝔏𝔄ℭ𝔎𝔇𝔒𝔗
- `bold-fraktur` - 𝕭𝕷𝕬𝕮𝕶𝕯𝕺𝕿
- `italic` - 𝐵𝐿𝐴𝐶𝐾𝐷𝑂𝑇
- `bold-italic` - 𝑩𝑳𝑨𝑪𝑲𝑫𝑶𝑻
- `small-caps` - ʙʟᴀᴄᴋᴅᴏᴛ

**Technical:**
- `monospace` - 𝚋𝚕𝚊𝚌𝚔𝚍𝚘𝚝
- `double-struck` - 𝔹𝕃𝔸ℂ𝕂𝔻𝕆𝕋
- `sans-serif` - 𝖡𝖫𝖠𝖢𝖪𝖣𝖮𝖳
- `sans-serif-italic` - 𝘉𝘓𝘈𝘊𝘒𝘋𝘖𝘛

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

**Namespace:** `{{frame:*}}`

Adds decorative prefix/suffix around content.

### Syntax

```markdown
{{frame:frame_type}}CONTENT{{/frame}}
```

### Available Frames

**Gradient Frames:**
- `gradient` - ▓▒░ TEXT ░▒▓
- `gradient-reverse` - ░▒▓ TEXT ▓▒░

**Line Frames:**
- `line-single` - ─ TEXT ─
- `line-double` - ═ TEXT ═
- `line-bold` - ━ TEXT ━

**Solid Frames:**
- `solid-left` - █▌ TEXT
- `solid-right` - TEXT ▐█

**Box Frames:**
- `box-light` - ┌─ TEXT ─┐
- `box-heavy` - ┏━ TEXT ━┓

**Arrow Frames:**
- `arrow-right` - → TEXT →
- `arrow-left` - ← TEXT ←

And 18 more frame styles. Run `mdfx frames list` for complete list.

### Examples

```markdown
{{frame:gradient}}TITLE{{/frame}}
→ ▓▒░ TITLE ░▒▓

{{frame:solid-left}}WARNING{{/frame}}
→ █▌ WARNING
```

---

## Badge Templates

**Namespace:** `{{badge:*}}`

Wraps single characters or digits in Unicode enclosures.

### Syntax

```markdown
{{badge:badge_type}}CHARACTER{{/badge}}
```

### Available Badge Types

- `circle` - ① ② ③ Ⓐ Ⓑ Ⓒ
- `negative-circle` - ❶ ❷ ❸
- `double-circle` - ⓵ ⓶ ⓷
- `paren` - ⑴ ⑵ ⑶ ⒜ ⒝ ⒞
- `period` - ⒈ ⒉ ⒊
- `paren-letter` - 🄐 🄑 🄒

### Rules

- Content must be single character
- Supports: 1-20, A-Z, a-z (depends on badge type)
- Error if character not supported by badge type

### Examples

```markdown
{{badge:circle}}1{{/badge}} → ①
{{badge:circle}}A{{/badge}} → Ⓐ
{{badge:negative-circle}}2{{/badge}} → ❷
{{badge:paren}}a{{/badge}} → ⒜
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

**icon** - Simple Icons logo
```markdown
{{shields:icon:logo=rust:color=FF6B35:style=flat-square/}}
```
- `logo=` - Simple Icons slug
- `color=` - Logo fill color

### When to Use Primitives

**Prefer UI components:**
- `{{ui:divider/}}` instead of `{{shields:bar:...}}`
- `{{ui:tech:rust/}}` instead of `{{shields:icon:logo=rust:...}}`

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
{{ui:divider/}}

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
Error: Invalid syntax: {{ui:divider}}{{/ui}} (expected {{ui:divider/}})
```

**Fix:** Use self-closing syntax: `{{ui:divider/}}`

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
template_name = identifier [ ":" identifier ] ;  (* e.g., "ui:divider" *)
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

## Quick Reference

| Template Type | Self-Closing | Block | Closer | Example |
|---------------|--------------|-------|--------|---------|
| Component | Yes | Yes | `{{/ui}}` | `{{ui:divider/}}` |
| Style | No | Yes | `{{/style}}` | `{{mathbold}}TEXT{{/mathbold}}` |
| Frame | No | Yes | `{{/frame}}` | `{{frame:gradient}}TEXT{{/frame}}` |
| Badge | No | Yes | `{{/badge}}` | `{{badge:circle}}1{{/badge}}` |
| Primitive | Yes | No | N/A | `{{shields:block:color=F41C80/}}` |

**Parameter Syntax:**

| Type | Syntax | Example |
|------|--------|---------|
| Positional | `:arg` | `{{ui:tech:rust/}}` |
| Named | `:key=value` | `{{mathbold:separator=dot}}` |
| Mixed | N/A | Not supported |

---

## See Also

- [API Guide](API-GUIDE.md) - Component reference with Rust examples
- [Architecture](ARCHITECTURE.md) - System design and implementation
- [State Machine Guide](STATE-MACHINE-GUIDE.md) - Parser internals
- [README](../README.md) - User guide and examples

---

**Last Updated:** v1.0.0 (2025-12-13)
