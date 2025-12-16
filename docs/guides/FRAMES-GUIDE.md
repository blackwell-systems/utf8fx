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

---

## Advanced Features

### Shorthand Syntax (`{{fr:}}`)

Use `fr:` instead of `frame:` for shorter templates:

```markdown
{{fr:gradient}}Title{{/}}
{{fr:star}}VIP{{/}}
```

### Universal Closer (`{{/}}`) and Close-All (`{{//}}`)

```markdown
{{fr:gradient}}Title{{/}}                    <!-- universal closer -->
{{fr:gradient}}{{fr:star}}Nested{{//}}       <!-- close all at once -->
```

### Frame Combos (`+`)

Combine multiple frames with `+` for nested effects without verbose syntax:

```markdown
{{fr:gradient+star}}TITLE{{/}}               → ▓▒░ ★ TITLE ☆ ░▒▓
{{fr:gradient+star+diamond}}VIP{{/}}         → ▓▒░ ★ ◆ VIP ◇ ☆ ░▒▓
```

### Count Multiplier (`*N`)

Repeat frame patterns N times (max 20):

```markdown
{{fr:star*3}}Title{{/}}                      → ★★★ Title ☆☆☆
{{fr:diamond*5}}Premium{{/}}                 → ◆◆◆◆◆ Premium ◇◇◇◇◇
{{fr:gradient*2}}X{{/}}                      → ▓▒░▓▒░ X ░▒▓░▒▓
```

### Reverse Modifier (`/reverse`)

Flip any frame's prefix and suffix:

```markdown
{{fr:gradient/reverse}}Title{{/}}           → ░▒▓ Title ▓▒░
{{fr:star/reverse}}VIP{{/}}                 → ☆ VIP ★
{{fr:star*2/reverse}}Title{{/}}             → ☆☆ Title ★★
```

### Separator (`/separator=X`)

Insert characters between pattern glyphs:

```markdown
{{fr:gradient/separator=·}}Title{{/}}       → ▓·▒·░ Title ░·▒·▓
{{fr:star*3/separator=·}}VIP{{/}}           → ★·★·★ VIP ☆·☆·☆
```

Named separators: `dot`, `dash`, `space`, `pipe`, `colon`

### Spacing (`/spacing=N`)

Insert N spaces between pattern glyphs:

```markdown
{{fr:gradient/spacing=1}}Title{{/}}         → ▓ ▒ ░ Title ░ ▒ ▓
{{fr:gradient/spacing=2}}Wide{{/}}          → ▓  ▒  ░ Wide ░  ▒  ▓
```

### Glyph Frames

Create frames from any registered glyph:

```markdown
{{fr:glyph:star}}Title{{/}}                 → ★ Title ★
{{fr:glyph:star*5}}VIP{{/}}                 → ★★★★★ VIP ★★★★★
{{fr:glyph:heart*3/separator=·}}Love{{/}}   → ♥·♥·♥ Love ♥·♥·♥
```

---

## Creative Showcase

These examples demonstrate the full power of the frame system.

### Epic Title Headers

```markdown
{{fr:gradient+star}}{{mathbold:separator=dot}}LEGENDARY{{/}}{{/}}
```
**Output:** `▓▒░ ★ 𝐋·𝐄·𝐆·𝐄·𝐍·𝐃·𝐀·𝐑·𝐘 ☆ ░▒▓`

```markdown
{{fr:gradient*2+diamond}}{{mathbold}}ULTIMATE EDITION{{/}}{{/}}
```
**Output:** `▓▒░▓▒░ ◆ 𝐔𝐋𝐓𝐈𝐌𝐀𝐓𝐄 𝐄𝐃𝐈𝐓𝐈𝐎𝐍 ◇ ░▒▓░▒▓`

---

### VIP & Premium Badges

```markdown
{{fr:star*5}}{{mathbold}}VIP ACCESS{{/}}{{/}}
```
**Output:** `★★★★★ 𝐕𝐈𝐏 𝐀𝐂𝐂𝐄𝐒𝐒 ☆☆☆☆☆`

```markdown
{{fr:diamond*3/separator=·}}{{fraktur}}Premium{{/}}{{/}}
```
**Output:** `◆·◆·◆ 𝔓𝔯𝔢𝔪𝔦𝔲𝔪 ◇·◇·◇`

```markdown
{{fr:glyph:crown*3+star}}{{script}}Royalty{{/}}{{/}}
```
**Output:** `👑👑👑 ★ 𝑅𝑜𝑦𝑎𝑙𝑡𝑦 ☆ 👑👑👑`

---

### Retro & Wave Effects

```markdown
{{fr:gradient/spacing=1}}{{monospace}}R E T R O{{/}}{{/}}
```
**Output:** `▓ ▒ ░ 𝚁 𝙴 𝚃 𝚁 𝙾 ░ ▒ ▓`

```markdown
{{fr:gradient-wave/separator=·}}{{fullwidth}}SYNTHWAVE{{/}}{{/}}
```
**Output:** `▓·▒·░ ＳＹＮＴＨＷＡＶＥ ▒·░·▓`

```markdown
{{fr:gradient/reverse+gradient}}ECHO{{/}}
```
**Output:** `░▒▓ ▓▒░ ECHO ░▒▓ ▓▒░`

---

### Section Dividers

```markdown
{{fr:glyph:line.h.bold*10/separator=·}}{{/}}
```
**Output:** `━·━·━·━·━·━·━·━·━·━  ━·━·━·━·━·━·━·━·━·━`

```markdown
{{fr:asterism+line-bold}}CHAPTER 3{{/}}
```
**Output:** `⁂ ━━━ CHAPTER 3 ━━━ ⁂`

```markdown
{{fr:glyph:diamond*7/spacing=1}}{{/}}
```
**Output:** `◆ ◆ ◆ ◆ ◆ ◆ ◆  ◆ ◆ ◆ ◆ ◆ ◆ ◆`

---

### Status & Alert Bars

```markdown
{{fr:solid-left}}{{fr:glyph:check*1}} {{mathbold}}BUILD PASSED{{/}}{{/}}
```
**Output:** `█▌✓ 𝐁𝐔𝐈𝐋𝐃 𝐏𝐀𝐒𝐒𝐄𝐃`

```markdown
{{fr:gradient/reverse}}{{fr:alert-warning}}{{mathbold}}DEPRECATED{{/}}{{/}}{{/}}
```
**Output:** `░▒▓ ⚠️ 𝐃𝐄𝐏𝐑𝐄𝐂𝐀𝐓𝐄𝐃 ▓▒░`

```markdown
{{fr:glyph:fire*3+solid-left}}{{sans-serif-bold}}HOT NEW FEATURE{{/}}{{/}}
```
**Output:** `🔥🔥🔥 █▌𝗛𝗢𝗧 𝗡𝗘𝗪 𝗙𝗘𝗔𝗧𝗨𝗥𝗘`

---

### International & Cultural Styles

```markdown
{{fr:lenticular}}{{fullwidth}}日本語{{/}}{{/}}
```
**Output:** `【日本語】`

```markdown
{{fr:guillemet+star}}{{italic}}L'élégance française{{/}}{{/}}
```
**Output:** `« ★ 𝐿'é𝑙é𝑔𝑎𝑛𝑐𝑒 𝑓𝑟𝑎𝑛ç𝑎𝑖𝑠𝑒 ☆ »`

```markdown
{{fr:angle}}{{bold-fraktur}}Der Meister{{/}}{{/}}
```
**Output:** `《𝕯𝖊𝖗 𝕸𝖊𝖎𝖘𝖙𝖊𝖗》`

---

### Gaming & Achievement Styles

```markdown
{{fr:star*3+gradient}}{{mathbold}}ACHIEVEMENT UNLOCKED{{/}}{{/}}
```
**Output:** `★★★ ▓▒░ 𝐀𝐂𝐇𝐈𝐄𝐕𝐄𝐌𝐄𝐍𝐓 𝐔𝐍𝐋𝐎𝐂𝐊𝐄𝐃 ░▒▓ ☆☆☆`

```markdown
{{fr:glyph:sword*2+diamond}}{{mathbold}}BOSS DEFEATED{{/}}{{/}}
```
**Output:** `⚔⚔ ◆ 𝐁𝐎𝐒𝐒 𝐃𝐄𝐅𝐄𝐀𝐓𝐄𝐃 ◇ ⚔⚔`

```markdown
{{fr:glyph:lightning*3/separator=·}}{{sans-serif-bold}}COMBO x99{{/}}{{/}}
```
**Output:** `⚡·⚡·⚡ 𝗖𝗢𝗠𝗕𝗢 𝘅𝟵𝟵 ⚡·⚡·⚡`

---

### Tech & Code Styles

```markdown
{{fr:gradient}}{{monospace}}fn main() → Result{{/}}{{/}}
```
**Output:** `▓▒░ 𝚏𝚗 𝚖𝚊𝚒𝚗() → 𝚁𝚎𝚜𝚞𝚕𝚝 ░▒▓`

```markdown
{{fr:glyph:gear*2+line-bold}}{{monospace}}CONFIG{{/}}{{/}}
```
**Output:** `⚙⚙ ━━━ 𝙲𝙾𝙽𝙵𝙸𝙶 ━━━ ⚙⚙`

```markdown
{{fr:solid-left}}{{double-struck}}API v2.0{{/}}{{/}}
```
**Output:** `█▌𝔸ℙ𝕀 𝕧𝟚.𝟘`

---

### Artistic Compositions

**Layered Fade:**
```markdown
{{fr:gradient/spacing=2}}{{fr:gradient/spacing=1}}{{fr:gradient}}CENTER{{/}}{{/}}{{/}}
```
**Output:** `▓  ▒  ░ ▓ ▒ ░ ▓▒░ CENTER ░▒▓ ░ ▒ ▓ ░  ▒  ▓`

**Symmetric Star Burst:**
```markdown
{{fr:star*2+diamond*2+star}}CORE{{/}}
```
**Output:** `★★ ◆◆ ★ CORE ☆ ◇◇ ☆☆`

**Breathing Gradient:**
```markdown
{{fr:gradient+gradient/reverse+gradient}}PULSE{{/}}
```
**Output:** `▓▒░ ░▒▓ ▓▒░ PULSE ░▒▓ ▓▒░ ░▒▓`

---

### Logo & Brand Headers

**Product Launch:**
```markdown
{{fr:glyph:rocket*1+gradient*2}}{{mathbold:separator=·}}LAUNCH DAY{{/}}{{/}}
```
**Output:** `🚀 ▓▒░▓▒░ 𝐋·𝐀·𝐔·𝐍·𝐂·𝐇· ·𝐃·𝐀·𝐘 ░▒▓░▒▓ 🚀`

**Open Source:**
```markdown
{{fr:glyph:heart*3/separator= }}{{fr:star}}{{mathbold}}OPEN SOURCE{{/}}{{/}}{{/}}
```
**Output:** `♥ ♥ ♥ ★ 𝐎𝐏𝐄𝐍 𝐒𝐎𝐔𝐑𝐂𝐄 ☆ ♥ ♥ ♥`

**Version Badge:**
```markdown
{{fr:solid-both}}{{fr:glyph:tag*1}} {{monospace}}v3.0.0{{/}}{{/}}
```
**Output:** `█▌🏷 𝚟𝟹.𝟶.𝟶▐█`

---

## Combining Everything

The ultimate example combining all features:

```markdown
{{fr:gradient*2+star*3+diamond/separator=·}}{{mathbold:separator=dot}}MDFX{{/}}{{/}}
```

**Breakdown:**
- `gradient*2` - Double gradient pattern
- `+star*3` - Nested with triple stars
- `+diamond` - Nested with diamond
- `/separator=·` - Dots between glyphs
- `mathbold:separator=dot` - Bold text with dot separators

**Output:** `▓·▒·░·▓·▒·░ ★·★·★ ◆ 𝐌·𝐃·𝐅·𝐗 ◇ ☆·☆·☆ ░·▒·▓·░·▒·▓`

---

## See Also

- [CLI Guide](CLI-GUIDE.md) - Command-line usage
- [Text Styles Guide](TEXT-STYLES-GUIDE.md) - Unicode text transformations
- [Glyphs Guide](GLYPHS-GUIDE.md) - Available Unicode symbols
- [Template Syntax](../TEMPLATE-SYNTAX.md) - Full syntax reference
