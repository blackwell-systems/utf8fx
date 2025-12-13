# What is Unicode?

> Understanding the character encoding standard that powers mdfx

## The Core Concept

**Unicode is not a font.** It's a character encoding standard that defines over 150,000 distinct characters across 159 writing systems and symbol sets.

When you see styled text like **𝐁𝐎𝐋𝐃** or 𝓈𝒸𝓇𝒾𝓅𝓉, those aren't special fonts applied to regular letters. Each styled character is a completely separate, distinct character in the Unicode standard with its own unique code point.

## Characters vs. Fonts

### Fonts (How Text Appears)
A font is a **styling instruction** that tells the computer how to draw characters:

```
"A" in Arial     → Arial's drawing of the letter A
"A" in Times     → Times's drawing of the letter A
"A" in Comic Sans → Comic Sans's drawing of the letter A
```

Same character (U+0041 "LATIN CAPITAL LETTER A"), different visual representations.

### Unicode Characters (What Text Is)
Unicode defines **distinct characters** that exist independently of fonts:

```
A    → U+0041 (LATIN CAPITAL LETTER A)
𝐀    → U+1D400 (MATHEMATICAL BOLD CAPITAL A)
𝐴    → U+1D434 (MATHEMATICAL ITALIC CAPITAL A)
𝒜    → U+1D49C (MATHEMATICAL SCRIPT CAPITAL A)
𝔸    → U+1D538 (MATHEMATICAL DOUBLE-STRUCK CAPITAL A)
Ａ    → U+FF21 (FULLWIDTH LATIN CAPITAL LETTER A)
```

Each line above is a **different character**, not the same character with different styling.

## Why Do These Characters Exist?

Unicode was designed to represent **all human writing systems and mathematical/scientific notation**. The styled alphabets in mdfx come from:

### 1. Mathematical Notation
Mathematicians need to distinguish between different types of variables:

- **Regular variables**: x, y
- **Bold variables**: 𝐱, 𝐲 (vectors)
- **Script variables**: 𝓍, 𝓎 (sheaves, categories)
- **Double-struck**: ℝ, ℂ (number systems like reals, complex numbers)

These distinctions carry **semantic meaning** in mathematics, so Unicode encodes them as distinct characters.

### 2. East Asian Typography
Full-width characters (Ａ, Ｂ, Ｃ) exist because:

- CJK (Chinese, Japanese, Korean) characters are typically square-shaped
- Mixing half-width Latin letters with full-width CJK looks awkward
- Full-width Latin allows uniform spacing in mixed-language text

### 3. Historical Writing Systems
- **Fraktur** (𝔉𝔯𝔞𝔨𝔱𝔲𝔯): Historic German blackletter script
- **Enclosed alphanumerics**: Ⓐ, Ⓑ, Ⓒ (used in lists, diagrams, annotations)

### 4. Accessibility and Plain Text
These characters work in **any context** that supports Unicode:
- File names
- URLs (sometimes)
- Plain text files
- Email subject lines
- Terminal output
- Markdown files
- Source code comments

## The Unicode Block System

Unicode organizes characters into "blocks" - contiguous ranges of related characters:

### Basic Latin (U+0000 to U+007F)
```
A B C D E F ... a b c d e f ... 0 1 2 3 4 5
```
Your standard ASCII characters. 128 characters total.

### Mathematical Alphanumeric Symbols (U+1D400 to U+1D7FF)
```
𝐀 𝐁 𝐂 ... (Mathematical Bold)
𝐴 𝐵 𝐶 ... (Mathematical Italic)
𝑨 𝑩 𝑪 ... (Mathematical Bold Italic)
𝒜 𝒷 𝒸 ... (Mathematical Script)
𝔄 𝔅 ℭ ... (Mathematical Fraktur)
𝕬 𝕭 𝕮 ... (Mathematical Bold Fraktur)
𝖠 𝖡 𝖢 ... (Mathematical Sans-Serif)
```
Over 996 characters dedicated to mathematical notation!

### Enclosed Alphanumerics (U+2460 to U+24FF)
```
① ② ③ ... (Circled Digits)
⑴ ⑵ ⑶ ... (Parenthesized Digits)
Ⓐ Ⓑ Ⓒ ... (Circled Latin Letters)
```

### Enclosed CJK Letters and Months (U+3200 to U+32FF)
```
㉑ ㉒ ㉓ ... (Circled Numbers 21+)
```

### Halfwidth and Fullwidth Forms (U+FF00 to U+FFEF)
```
Ａ Ｂ Ｃ ... (Fullwidth Latin Letters)
０ １ ２ ... (Fullwidth Digits)
```

## What mdfx Actually Does

mdfx performs **character-to-character mapping** between Unicode blocks:

```
Input:  "HELLO"  (U+0048 U+0045 U+004C U+004C U+004F)
         ↓ map to Mathematical Bold block
Output: "𝐇𝐄𝐋𝐋𝐎"  (U+1D407 U+1D404 U+1D40B U+1D40B U+1D40E)
```

This is **not** font substitution. It's character substitution.

### The Mapping Process

For each input character:
1. Determine its position in Basic Latin (A = 0, B = 1, C = 2...)
2. Calculate offset into target Unicode block
3. Return the character at that position

Example - Converting 'A' to Mathematical Bold:
```
'A' = U+0041 (position 0 in uppercase Latin)
Mathematical Bold block starts at U+1D400
Result: U+1D400 + 0 = U+1D400 = '𝐀'
```

## Limitations and Gotchas

### 1. Not All Characters Have All Styles
```
✓ A → 𝐀 𝐴 𝒜 𝔸 𝕬 𝖠 𝗔 𝘈 𝘼 𝙰 (many styles)
✗ á → Limited styled variants (accented letters often missing)
```

Not every character in every Unicode block has a mathematical variant.

### 2. Font Support Required
While these are distinct characters, **your font must support them** to display correctly:

- ✓ Modern system fonts (Arial, Times New Roman, etc.) → Usually good
- ✗ Old/specialized fonts → May show boxes (□) or question marks (�)

### 3. Search and Indexing
```
Search for "HELLO"
Won't match: "𝐇𝐄𝐋𝐋𝐎" or "ℋℰℒℒ𝒪"
```

These are **different characters** to search engines and text editors.

### 4. Screen Readers
Screen readers may read styled text differently:
- "𝐇𝐄𝐋𝐋𝐎" might be read as "mathematical bold H E L L O"
- This can affect accessibility

## Why This Matters for Markdown

Traditional markdown has limited styling:
```markdown
**bold** → HTML/rendering-time styling
*italic* → HTML/rendering-time styling
```

With Unicode character substitution:
```markdown
𝐁𝐨𝐥𝐝 → Works in plain text, file names, anywhere
𝘐𝘵𝘢𝘭𝘪𝘤 → No HTML rendering required
```

### Use Cases Where Unicode Wins

1. **Plain Text Emails**
   ```
   Subject: 𝐔𝐑𝐆𝐄𝐍𝐓: Action Required
   ```
   Bold works even in plain text!

2. **GitHub Markdown**
   ```markdown
   ## ▓▒░ 𝐌𝐘 𝐏𝐑𝐎𝐉𝐄𝐂𝐓 ░▒▓
   ```
   Renders identically everywhere markdown is rendered.

3. **File Names**
   ```
   📋 𝐐𝐮𝐚𝐫𝐭𝐞𝐫𝐥𝐲 𝐑𝐞𝐩𝐨𝐫𝐭.pdf
   ```
   Styled text in file names (where fonts don't apply).

4. **Source Code Comments**
   ```rust
   // 𝐈𝐌𝐏𝐎𝐑𝐓𝐀𝐍𝐓: This function is performance-critical
   ```
   No syntax highlighting needed for emphasis.

## How mdfx Leverages Unicode

mdfx provides a **controlled interface** to Unicode's mathematical and styled characters:

### 1. Character Mapping
```bash
mdfx convert --style mathbold "TEXT"
# Maps Basic Latin → Mathematical Bold block
```

### 2. Template Processing
```markdown
{{mathbold}}BOLD{{/mathbold}}
# Processed at build-time, output is pure Unicode
```

### 3. Preservation
The styled characters mdfx produces are **permanent**:
- Copy/paste anywhere
- Work in any Unicode-aware application
- No CSS, no fonts, no rendering dependencies

## Common Misconceptions

### ❌ "These are just fancy fonts"
No. Each character has its own code point in Unicode.

### ❌ "This is like CSS styling"
No. CSS changes how existing characters are displayed. mdfx replaces characters with different Unicode characters.

### ❌ "These characters are new/non-standard"
No. Mathematical Alphanumeric Symbols were added to Unicode in 2001 (Unicode 3.1). They're 20+ years old and universally supported.

### ❌ "This breaks text"
Depends on use case:
- ✓ Visual emphasis in markdown → Works great
- ✓ Headers and titles → Perfect
- ✗ Body text that needs searching → Not ideal
- ✗ Programmatic string matching → Will fail

## Technical Details

### Unicode Standard
- **Current version**: Unicode 15.1 (September 2023)
- **Total characters**: 149,186
- **Blocks**: 308 named blocks
- **Scripts**: 159 distinct writing systems

### UTF-8 Encoding
mdfx uses UTF-8 encoding (hence the original name "utf8fx"):
```
'A' (U+0041)     → 1 byte:  41
'𝐀' (U+1D400)    → 4 bytes: F0 9D 90 80
```

Mathematical symbols require more bytes than Basic Latin characters.

### Character Properties
Unicode assigns properties to each character:
- **Category**: Letter, Number, Punctuation, etc.
- **Case**: Uppercase, Lowercase, Titlecase
- **Numeric Value**: For digit characters
- **Bidirectionality**: Left-to-right, Right-to-left

## Further Reading

### Official Resources
- [Unicode Consortium](https://unicode.org/) - The organization maintaining Unicode
- [Unicode Standard](https://www.unicode.org/versions/Unicode15.1.0/) - Official specification
- [Unicode Charts](https://www.unicode.org/charts/) - Visual reference for all blocks

### Specific Blocks Used by mdfx
- [Mathematical Alphanumeric Symbols](https://unicode.org/charts/PDF/U1D400.pdf)
- [Enclosed Alphanumerics](https://unicode.org/charts/PDF/U2460.pdf)
- [Halfwidth and Fullwidth Forms](https://unicode.org/charts/PDF/UFF00.pdf)

### Tools
- [Unicode Character Inspector](https://unicode-table.com/)
- [Shapecatcher](https://shapecatcher.com/) - Draw character to find it
- [Amp What](http://www.amp-what.com/) - Unicode character search

## Summary

Unicode is a **character encoding standard** that defines over 150,000 distinct characters, including multiple styled variants of Latin letters designed for mathematical notation.

**mdfx leverages these pre-existing Unicode characters** to provide text styling that works anywhere Unicode is supported - no fonts, no CSS, no rendering dependencies required.

The styled characters you see in mdfx output aren't font tricks - they're real, distinct characters that have existed in the Unicode standard for over 20 years.

---

**Related Documentation:**
- [mdfx Architecture](/ARCHITECTURE.md) - How mdfx implements character mapping
- [API Guide](/API-GUIDE.md) - Programmatic usage
- [Unicode Design Elements](/unicode-design-elements.md) - Visual reference
