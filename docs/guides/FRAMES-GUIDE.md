# Frames Guide

Frames wrap text content with decorative Unicode borders. They're perfect for headers, callouts, section dividers, and visual emphasis.

## Basic Syntax

```markdown
{{frame:style}}Your content here{{/frame}}
```

Frames support **nesting** for layered effects:

```markdown
{{frame:gradient}}{{frame:line-bold}}NESTED CONTENT{{/frame}}{{/frame}}
```

---

## All Frame Styles

### Gradient Frames

Block element gradients for dramatic emphasis.

| Style | Aliases | Output |
|-------|---------|--------|
| `gradient` | grad, gradient-full | `▓▒░ text ░▒▓` |
| `gradient-light` | gradlight | `▒░ text ░▒` |
| `gradient-reverse` | gradrev | `░▒▓ text ▓▒░` |

```markdown
{{frame:gradient}}DRAMATIC HEADER{{/frame}}
{{frame:gradient-light}}Subtle emphasis{{/frame}}
{{frame:gradient-reverse}}Inverted style{{/frame}}
```

**Rendered:**

▓▒░ DRAMATIC HEADER ░▒▓

▒░ Subtle emphasis ░▒

░▒▓ Inverted style ▓▒░

---

### Solid Block Frames

Heavy block elements for strong visual weight.

| Style | Aliases | Output |
|-------|---------|--------|
| `solid-left` | solidleft, left | `█▌text` |
| `solid-right` | solidright, right | `text▐█` |
| `solid-both` | solid, solidboth | `█▌text▐█` |

```markdown
{{frame:solid-left}}Callout text{{/frame}}
{{frame:solid-right}}Right aligned{{/frame}}
{{frame:solid-both}}Fully framed{{/frame}}
```

**Rendered:**

█▌Callout text

Right aligned▐█

█▌Fully framed▐█

**Use case:** `solid-left` is excellent for callout boxes and blockquote-style content.

---

### Line Frames

Horizontal rules for clean separators.

| Style | Aliases | Output |
|-------|---------|--------|
| `line-light` | line, light | `─── text ───` |
| `line-bold` | bold-line, dashed | `━━━ text ━━━` |
| `line-double` | double, double-line | `═══ text ═══` |
| `line-dashed` | dash, dashes | `╌╌╌ text ╌╌╌` |

```markdown
{{frame:line-light}}Light divider{{/frame}}
{{frame:line-bold}}Bold divider{{/frame}}
{{frame:line-double}}Double line{{/frame}}
{{frame:line-dashed}}Dashed style{{/frame}}
```

**Rendered:**

─── Light divider ───

━━━ Bold divider ━━━

═══ Double line ═══

╌╌╌ Dashed style ╌╌╌

---

### Block Element Frames

Top/bottom block decorations.

| Style | Aliases | Output |
|-------|---------|--------|
| `block-top` | top | `▀▀▀ text ▀▀▀` |
| `block-bottom` | bottom | `▄▄▄ text ▄▄▄` |

```markdown
{{frame:block-top}}Upper block{{/frame}}
{{frame:block-bottom}}Lower block{{/frame}}
```

**Rendered:**

▀▀▀ Upper block ▀▀▀

▄▄▄ Lower block ▄▄▄

---

### Symbol Frames

Decorative symbols with asymmetric prefix/suffix for visual flair.

| Style | Aliases | Output |
|-------|---------|--------|
| `star` | stars, featured | `★ text ☆` |
| `diamond` | diamonds, gem | `◆ text ◇` |
| `triangle-right` | triangles, tri-h | `▶ text ◀` |
| `finger` | fingers, point, manicule | `☞ text ☜` |

```markdown
{{frame:star}}Featured content{{/}}
{{frame:diamond}}Premium item{{/}}
{{frame:finger}}Important note{{/}}
```

**Rendered:**

★ Featured content ☆

◆ Premium item ◇

☞ Important note ☜

> **Note:** For symmetric single-character frames like bullets (•), dots (·), or arrows (→), use glyph frames: `{{frame:glyph:bullet}}text{{/}}`

---

### Quotation & Bracket Frames

International quotation styles and brackets.

| Style | Aliases | Output |
|-------|---------|--------|
| `heavy-quote` | heavy-quotes, dquote | `❝text❞` |
| `lenticular` | lent, japanese, cjk | `【text】` |
| `angle` | angles, chinese | `《text》` |
| `guillemet` | french, quote | `« text »` |
| `guillemet-single` | french-single, quote-single | `‹ text ›` |

```markdown
{{frame:heavy-quote}}A memorable quote{{/frame}}
{{frame:lenticular}}Japanese style{{/frame}}
{{frame:angle}}Chinese brackets{{/frame}}
{{frame:guillemet}}French quotation{{/frame}}
```

**Rendered:**

❝A memorable quote❞

【Japanese style】

《Chinese brackets》

« French quotation »

---

### Special Frames

Rounded corner decorations.

| Style | Aliases | Output |
|-------|---------|--------|
| `arc-top` | arctop, rounded-top | `╭ text ╮` |
| `arc-bottom` | arcbottom, rounded-bottom | `╰ text ╯` |

```markdown
{{frame:arc-top}}Rounded top{{/}}
{{frame:arc-bottom}}Rounded bottom{{/}}
```

**Rendered:**

╭ Rounded top ╮

╰ Rounded bottom ╯

**Combine arcs for a rounded box effect:**
```markdown
{{frame:arc-top}}╭───────────╮{{/frame}}
{{frame:arc-bottom}}╰───────────╯{{/frame}}
```

---

### Alert Frames

Status indicator prefixes for notifications.

| Style | Aliases | Output |
|-------|---------|--------|
| `alert-warning` | warn, caution | `⚠️ text` |
| `alert-info` | note, tip | `ℹ️ text` |
| `alert-success` | done, ok | `✅ text` |
| `alert-error` | danger, fail | `❌ text` |

```markdown
{{frame:alert-warning}}Proceed with caution{{/frame}}
{{frame:alert-info}}Helpful tip here{{/frame}}
{{frame:alert-success}}Operation complete{{/frame}}
{{frame:alert-error}}Something went wrong{{/frame}}
```

**Rendered:**

⚠️ Proceed with caution

ℹ️ Helpful tip here

✅ Operation complete

❌ Something went wrong

---

## Nesting Frames

Frames can be nested for layered visual effects:

**Syntax:**
```markdown
{{frame:gradient}}{{frame:line-bold}}ANNOUNCEMENT{{/frame}}{{/frame}}
```

**Rendered:**

▓▒░ ━━━ ANNOUNCEMENT ━━━ ░▒▓

**Triple nesting:**

**Syntax:**
```markdown
{{frame:solid-left}}{{frame:gradient}}{{frame:star}}VIP{{/frame}}{{/frame}}{{/frame}}
```

**Rendered:**

█▌▓▒░ ★ VIP ☆ ░▒▓

---

## Combining with Text Styles

Frames work beautifully with text styles:

**Syntax:**
```markdown
{{frame:gradient}}{{mathbold}}BOLD HEADER{{/mathbold}}{{/frame}}
{{frame:star}}{{fraktur}}Gothic Text{{/fraktur}}{{/frame}}
{{frame:lenticular}}{{fullwidth}}WIDE TEXT{{/fullwidth}}{{/frame}}
```

**Rendered:**

▓▒░ 𝐁𝐎𝐋𝐃 𝐇𝐄𝐀𝐃𝐄𝐑 ░▒▓

★ 𝔊𝔬𝔱𝔥𝔦𝔠 𝔗𝔢𝔵𝔱 ☆

【ＷＩＤＥ ＴＥＸＴ】

---

## Practical Examples

### Section Header

**Syntax:**
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}GETTING STARTED{{/mathbold}}{{/frame}}
```

**Rendered:**

▓▒░ 𝐆·𝐄·𝐓·𝐓·𝐈·𝐍·𝐆· ·𝐒·𝐓·𝐀·𝐑·𝐓·𝐄·𝐃 ░▒▓

### Callout Box

**Syntax:**
```markdown
{{frame:solid-left}}{{ui:swatch:warning/}} **Warning:** This action cannot be undone.{{/frame}}
```

**Rendered:**

█▌![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) **Warning:** This action cannot be undone.

### Featured Quote

**Syntax:**
```markdown
{{frame:heavy-quote}}{{italic}}The best code is no code at all.{{/italic}}{{/frame}}
```

**Rendered:**

❝𝑇ℎ𝑒 𝑏𝑒𝑠𝑡 𝑐𝑜𝑑𝑒 𝑖𝑠 𝑛𝑜 𝑐𝑜𝑑𝑒 𝑎𝑡 𝑎𝑙𝑙.❞

### Navigation Breadcrumb

**Syntax:**
```markdown
{{frame:arrow}}Home → Products → Details{{/frame}}
```

**Rendered:**

→ Home → Products → Details →

### Japanese-Style Title

**Syntax:**
```markdown
{{frame:lenticular}}{{fullwidth}}CHAPTER ONE{{/fullwidth}}{{/frame}}
```

**Rendered:**

【ＣＨＡＰＴＥＲ ＯＮＥ】

### Status Banner

**Syntax:**
```markdown
{{frame:gradient}}{{frame:alert-success}}All systems operational{{/frame}}{{/frame}}
```

**Rendered:**

▓▒░ ✅ All systems operational ░▒▓

---

## Glyph Frame Shorthand

Create custom frames using any glyph as the decorative element. This is more flexible than predefined frames.

### Basic Syntax

```markdown
{{frame:glyph:NAME}}content{{/frame}}
```

**Example:**
```markdown
{{frame:glyph:star}}Featured{{/frame}}
```

**Rendered:** ★ Featured ★

### Multiplier (*N)

Repeat the glyph N times (max 20):

```markdown
{{frame:glyph:star*3}}Title{{/frame}}
{{frame:glyph:diamond*5}}Premium{{/frame}}
```

**Rendered:**

★★★ Title ★★★

◆◆◆◆◆ Premium ◆◆◆◆◆

### Padding Control (/pad=VALUE)

Control spacing between glyphs and content:

| Syntax | Effect |
|--------|--------|
| `/pad=0` | No padding (tight) |
| `/pad=1` | Single space (default) |
| `/pad=3` | Three spaces |
| `/pad=-` | Custom character `-` |
| `/pad=·` | Custom character `·` |
| `/pad=--` | Multi-character `--` |

**Examples:**

```markdown
{{frame:glyph:star*3/pad=0}}Tight{{/frame}}
{{frame:glyph:star*3/pad=3}}Wide{{/frame}}
{{frame:glyph:diamond*2/pad=·}}Dotted{{/frame}}
{{frame:glyph:bullet*4/pad=--}}Dashed{{/frame}}
```

**Rendered:**

★★★Tight★★★

★★★   Wide   ★★★

◆◆·Dotted·◆◆

••••--Dashed--••••

### Replacing Line Frames

Glyph frames can replicate line frames dynamically:

```markdown
{{frame:glyph:line.h.light*3}}Title{{/frame}}
{{frame:glyph:line.h.bold*3}}Title{{/frame}}
```

**Rendered:**

─── Title ───

━━━ Title ━━━

---

## Quick Reference

| Category | Frames |
|----------|--------|
| **Gradient** | gradient, gradient-light, gradient-reverse |
| **Solid** | solid-left, solid-right, solid-both |
| **Lines** | line-light, line-bold, line-double, line-dashed |
| **Blocks** | block-top, block-bottom |
| **Symbols** | star, diamond, triangle-right, finger |
| **Quotes** | heavy-quote, lenticular, angle, guillemet, guillemet-single |
| **Arcs** | arc-top, arc-bottom |
| **Alerts** | alert-warning, alert-info, alert-success, alert-error |
| **Glyphs** | `{{frame:glyph:NAME}}` - any glyph as symmetric frame |

---

## Tips

1. **Keep it readable** - Don't over-nest frames; 2-3 levels max
2. **Match the mood** - Use gradient for headers, solid-left for callouts, alerts for status
3. **Test rendering** - Some Unicode may display differently across fonts/platforms
4. **Use aliases** - Shorter aliases like `grad` and `bold-line` speed up typing
5. **Combine wisely** - Frames + text styles + swatches create rich visual elements

---

<p align="center">
ʀᴇɴᴅᴇʀᴇᴅ ᴡɪᴛʜ ᴍᴅꜰx
</p>
