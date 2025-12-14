# Glyphs Guide

Glyphs are single Unicode characters for inline use. They provide consistent, semantic access to common symbols without remembering Unicode codes.

## Basic Syntax

```markdown
{{glyph:name/}}
```

Glyphs are self-closing and render as single characters.

---

## All Glyphs

### Separators & Punctuation

| Glyph | Character | Unicode | Description |
|-------|-----------|---------|-------------|
| `dot` | · | U+00B7 | Middle dot separator |
| `bullet` | • | U+2022 | Bullet point |
| `pipe` | \| | U+007C | Vertical bar |
| `slash` | / | U+002F | Forward slash |
| `tilde` | ~ | U+007E | Tilde |

**Examples:**
```markdown
Item A {{glyph:dot/}} Item B {{glyph:dot/}} Item C
{{glyph:bullet/}} First point
Value {{glyph:pipe/}} Other value
```

**Output:**
```
Item A · Item B · Item C
• First point
Value | Other value
```

---

### Dashes & Lines

| Glyph | Character | Unicode | Description |
|-------|-----------|---------|-------------|
| `dash` | ─ | U+2500 | Light horizontal line |
| `bolddash` | ━ | U+2501 | Heavy horizontal line |
| `em-dash` | — | U+2014 | Em dash |
| `en-dash` | – | U+2013 | En dash |

**Examples:**
```markdown
{{glyph:dash/}}{{glyph:dash/}}{{glyph:dash/}} Section {{glyph:dash/}}{{glyph:dash/}}{{glyph:dash/}}
10{{glyph:en-dash/}}20 items
Wait{{glyph:em-dash/}}what?
```

**Output:**
```
─── Section ───
10–20 items
Wait—what?
```

---

### Symbols

| Glyph | Character | Unicode | Description |
|-------|-----------|---------|-------------|
| `arrow` | → | U+2192 | Rightward arrow |
| `star` | ★ | U+2605 | Black star |
| `diamond` | ◆ | U+25C6 | Black diamond |
| `square` | ■ | U+25A0 | Black square |
| `circle` | ● | U+25CF | Black circle |
| `ellipsis` | … | U+2026 | Horizontal ellipsis |

**Examples:**
```markdown
Home {{glyph:arrow/}} Products {{glyph:arrow/}} Details
{{glyph:star/}} Featured item
{{glyph:diamond/}} Premium tier
Loading{{glyph:ellipsis/}}
```

**Output:**
```
Home → Products → Details
★ Featured item
◆ Premium tier
Loading…
```

---

### Status Indicators

| Glyph | Character | Unicode | Description |
|-------|-----------|---------|-------------|
| `check` | ✓ | U+2713 | Check mark |
| `cross` | ✗ | U+2717 | Cross mark |
| `warning-icon` | ⚠ | U+26A0 | Warning sign |
| `info-icon` | ℹ | U+2139 | Information sign |

**Examples:**
```markdown
{{glyph:check/}} Task completed
{{glyph:cross/}} Failed validation
{{glyph:warning-icon/}} Caution required
{{glyph:info-icon/}} Additional details
```

**Output:**
```
✓ Task completed
✗ Failed validation
⚠ Caution required
ℹ Additional details
```

---

### Whitespace

| Glyph | Character | Unicode | Description |
|-------|-----------|---------|-------------|
| `space` | (space) | U+0020 | Space character |
| `newline` | (newline) | U+000A | Line break |

**Note:** `newline` only works in block context, not inline.

**Examples:**
```markdown
Word{{glyph:space/}}{{glyph:space/}}{{glyph:space/}}Gap
Line one{{glyph:newline/}}Line two
```

---

## Quick Reference

### By Category

**Separators:** `dot` · `bullet` • `pipe` | `slash` / `tilde` ~

**Lines:** `dash` ─ `bolddash` ━ `em-dash` — `en-dash` –

**Shapes:** `arrow` → `star` ★ `diamond` ◆ `square` ■ `circle` ●

**Status:** `check` ✓ `cross` ✗ `warning-icon` ⚠ `info-icon` ℹ

**Text:** `ellipsis` … `space` `newline`

---

## Practical Examples

### Breadcrumb Navigation

```markdown
{{glyph:arrow/}} Home {{glyph:arrow/}} Docs {{glyph:arrow/}} API
```

**Output:** `→ Home → Docs → API`

---

### Feature List

```markdown
{{glyph:check/}} Fast compilation
{{glyph:check/}} Type safety
{{glyph:check/}} Zero-cost abstractions
{{glyph:cross/}} Garbage collection
```

---

### Inline Separator

```markdown
Rust {{glyph:dot/}} TypeScript {{glyph:dot/}} Go
```

**Output:** `Rust · TypeScript · Go`

---

### Range Notation

```markdown
Pages 10{{glyph:en-dash/}}25
The answer{{glyph:em-dash/}}if there is one{{glyph:em-dash/}}remains unclear.
```

---

### Custom Divider Line

```markdown
{{glyph:bolddash/}}{{glyph:bolddash/}}{{glyph:bolddash/}}{{glyph:bolddash/}}{{glyph:bolddash/}}
```

**Output:** `━━━━━`

---

### Status Table

```markdown
| Feature | Status |
|---------|--------|
| Auth | {{glyph:check/}} |
| API | {{glyph:check/}} |
| UI | {{glyph:cross/}} |
```

---

### Warning Message

```markdown
{{glyph:warning-icon/}} **Warning:** This action is irreversible.
```

---

### Loading Indicator

```markdown
Processing{{glyph:ellipsis/}}
```

---

## Combining with Other Elements

### With Frames

```markdown
{{frame:dot}}{{glyph:star/}} Featured {{glyph:star/}}{{/frame}}
```

**Output:** `· ★ Featured ★ ·`

---

### With Text Styles

```markdown
{{mathbold}}{{glyph:arrow/}} NEXT SECTION{{/mathbold}}
```

---

### With Badges

```markdown
{{badge:circle:1/}} {{glyph:arrow/}} First step
{{badge:circle:2/}} {{glyph:arrow/}} Second step
```

---

## Unicode Reference

| Glyph | Unicode | Block |
|-------|---------|-------|
| dot | U+00B7 | Latin-1 Supplement |
| bullet | U+2022 | General Punctuation |
| dash | U+2500 | Box Drawing |
| bolddash | U+2501 | Box Drawing |
| arrow | U+2192 | Arrows |
| star | U+2605 | Miscellaneous Symbols |
| diamond | U+25C6 | Geometric Shapes |
| square | U+25A0 | Geometric Shapes |
| circle | U+25CF | Geometric Shapes |
| pipe | U+007C | Basic Latin |
| slash | U+002F | Basic Latin |
| tilde | U+007E | Basic Latin |
| newline | U+000A | Basic Latin |
| space | U+0020 | Basic Latin |
| em-dash | U+2014 | General Punctuation |
| en-dash | U+2013 | General Punctuation |
| ellipsis | U+2026 | General Punctuation |
| check | U+2713 | Dingbats |
| cross | U+2717 | Dingbats |
| warning-icon | U+26A0 | Miscellaneous Symbols |
| info-icon | U+2139 | Letterlike Symbols |

---

## Tips

1. **Semantic names** - Use glyph names instead of copying Unicode characters for maintainability
2. **Consistency** - Pick one separator style (dot vs bullet vs pipe) per document
3. **Context matters** - `newline` only works in block context
4. **Font support** - Most glyphs render well across modern fonts
5. **Accessibility** - Screen readers handle standard Unicode characters
