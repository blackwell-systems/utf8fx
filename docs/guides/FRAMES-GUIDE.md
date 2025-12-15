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

**Output:**
```
▓▒░ DRAMATIC HEADER ░▒▓
▒░ Subtle emphasis ░▒
░▒▓ Inverted style ▓▒░
```

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

**Output:**
```
█▌Callout text
Right aligned▐█
█▌Fully framed▐█
```

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

**Output:**
```
─── Light divider ───
━━━ Bold divider ━━━
═══ Double line ═══
╌╌╌ Dashed style ╌╌╌
```

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

**Output:**
```
▀▀▀ Upper block ▀▀▀
▄▄▄ Lower block ▄▄▄
```

---

### Symbol Frames

Decorative symbols for visual flair.

| Style | Aliases | Output |
|-------|---------|--------|
| `arrow-right` | arrow, arrows | `→ text →` |
| `dot` | dots, middledot | `· text ·` |
| `bullet` | bullets | `• text •` |
| `star` | stars, featured | `★ text ☆` |
| `diamond` | diamonds, gem | `◆ text ◇` |
| `triangle-right` | triangles, tri-h | `▶ text ◀` |
| `finger` | fingers, point, manicule | `☞ text ☜` |
| `fisheye` | bullseye, target | `◉ text ◉` |

```markdown
{{frame:star}}Featured content{{/frame}}
{{frame:diamond}}Premium item{{/frame}}
{{frame:finger}}Important note{{/frame}}
{{frame:arrow}}Navigation{{/frame}}
```

**Output:**
```
★ Featured content ☆
◆ Premium item ◇
☞ Important note ☜
→ Navigation →
```

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

**Output:**
```
❝A memorable quote❞
【Japanese style】
《Chinese brackets》
« French quotation »
```

---

### Special Frames

Unique decorative elements.

| Style | Aliases | Output |
|-------|---------|--------|
| `asterism` | section, divider | `⁂ text ⁂` |
| `arc-top` | arctop, rounded-top | `╭ text ╮` |
| `arc-bottom` | arcbottom, rounded-bottom | `╰ text ╯` |

```markdown
{{frame:asterism}}Section break{{/frame}}
{{frame:arc-top}}Rounded top{{/frame}}
{{frame:arc-bottom}}Rounded bottom{{/frame}}
```

**Output:**
```
⁂ Section break ⁂
╭ Rounded top ╮
╰ Rounded bottom ╯
```

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

**Output:**
```
⚠️ Proceed with caution
ℹ️ Helpful tip here
✅ Operation complete
❌ Something went wrong
```

---

## Nesting Frames

Frames can be nested for layered visual effects:

```markdown
{{frame:gradient}}{{frame:line-bold}}ANNOUNCEMENT{{/frame}}{{/frame}}
```

**Output:**
```
▓▒░ ━━━ ANNOUNCEMENT ━━━ ░▒▓
```

**Triple nesting:**
```markdown
{{frame:solid-left}}{{frame:gradient}}{{frame:star}}VIP{{/frame}}{{/frame}}{{/frame}}
```

**Output:**
```
█▌▓▒░ ★ VIP ☆ ░▒▓
```

---

## Combining with Text Styles

Frames work beautifully with text styles:

```markdown
{{frame:gradient}}{{mathbold}}BOLD HEADER{{/mathbold}}{{/frame}}
{{frame:star}}{{fraktur}}Gothic Text{{/fraktur}}{{/frame}}
{{frame:lenticular}}{{fullwidth}}WIDE TEXT{{/fullwidth}}{{/frame}}
```

**Output:**
```
▓▒░ 𝐁𝐎𝐋𝐃 𝐇𝐄𝐀𝐃𝐄𝐑 ░▒▓
★ 𝔊𝔬𝔱𝔥𝔦𝔠 𝔗𝔢𝔵𝔱 ☆
【ＷＩＤＥ　ＴＥＸＴ】
```

---

## Practical Examples

### Section Header
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}GETTING STARTED{{/mathbold}}{{/frame}}
```

### Callout Box
```markdown
{{frame:solid-left}}{{ui:swatch:warning/}} **Warning:** This action cannot be undone.{{/frame}}
```

### Featured Quote
```markdown
{{frame:heavy-quote}}{{italic}}The best code is no code at all.{{/italic}}{{/frame}}
```

### Navigation Breadcrumb
```markdown
{{frame:arrow}}Home → Products → Details{{/frame}}
```

### Japanese-Style Title
```markdown
{{frame:lenticular}}{{fullwidth}}CHAPTER ONE{{/fullwidth}}{{/frame}}
```

### Status Banner
```markdown
{{frame:gradient}}{{frame:alert-success}}All systems operational{{/frame}}{{/frame}}
```

---

## Quick Reference

| Category | Frames |
|----------|--------|
| **Gradient** | gradient, gradient-light, gradient-reverse |
| **Solid** | solid-left, solid-right, solid-both |
| **Lines** | line-light, line-bold, line-double, line-dashed |
| **Blocks** | block-top, block-bottom |
| **Symbols** | arrow-right, dot, bullet, star, diamond, triangle-right, finger, fisheye |
| **Quotes** | heavy-quote, lenticular, angle, guillemet, guillemet-single |
| **Special** | asterism, arc-top, arc-bottom |
| **Alerts** | alert-warning, alert-info, alert-success, alert-error |

---

## Tips

1. **Keep it readable** - Don't over-nest frames; 2-3 levels max
2. **Match the mood** - Use gradient for headers, solid-left for callouts, alerts for status
3. **Test rendering** - Some Unicode may display differently across fonts/platforms
4. **Use aliases** - Shorter aliases like `grad` and `bold-line` speed up typing
5. **Combine wisely** - Frames + text styles + swatches create rich visual elements
