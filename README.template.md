# {{frame:gradient}}{{mathbold:separator=dot}}U T F 8 F X{{/mathbold}}{{/frame}} {{badge:circle}}1{{/badge}}.{{badge:circle}}0{{/badge}}

[![Blackwell Systems™](https://raw.githubusercontent.com/blackwell-systems/blackwell-docs-theme/main/badge-trademark.svg)](https://github.com/blackwell-systems)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)

{{sans-serif-bold}}Unicode text effects for markdown and beyond{{/sans-serif-bold}}

Transform text into various Unicode styles (mathematical bold, full-width, negative squared, and more)
through a powerful markdown preprocessing system. Perfect for READMEs, documentation, slide decks, or
anywhere you want your project branding to stand out without images.

## {{mathbold}}Motivation{{/mathbold}}

Unicode offers a plethora of diverse and interesting styling options—from elegant 𝓼𝓬𝓻𝓲𝓹𝓽 to bold 𝔣𝔯𝔞𝔨𝔱𝔲𝔯 to playful
Ⓒⓘⓡⓒⓛⓔⓢ—but they feel hidden and cumbersome to interact with. Finding the right glyphs requires hunting
through Unicode tables, manually copying characters, and tediously spacing them by hand.

**utf8fx** makes Unicode styling simple and repeatable. Instead of searching for individual characters,
you use intuitive template syntax like `{{mathbold}}TITLE{{/mathbold}}` or CLI commands like
`utf8fx convert --style script "Elegant"`. Need spaced letters for a header? Just add `:spacing=2` and you're done.

This tool transforms Unicode styling from a frustrating manual process into something as easy as markdown
formatting—perfect for README files, documentation, and any text where you want distinctive visual
elements without leaving your editor.

## {{mathbold}}Why utf8fx?{{/mathbold}}

**Why not just copy/paste Unicode characters?**

You could manually copy `𝐓𝐈𝐓𝐋𝐄` from a Unicode table, but:

- **Repeatability**: With templates, you can reuse `{{mathbold}}TITLE{{/mathbold}}` across dozens of files
- **Consistency**: Change `mathbold` to `script` once, regenerate all docs - instant rebrand
- **Maintainability**: Your source files remain readable ASCII, styled output is generated
- **Search & Replace**: Find/replace works on template names, not opaque Unicode glyphs
- **Version Control**: Diffs show intent (`mathbold` → `script`), not character code changes
- **Composability**: Combine styles + separators + frames in ways copy/paste can't match

**The difference:**

```markdown
# Manual approach (fragile):
𝐓·𝐈·𝐓·𝐋·𝐄  ← You copied each character. Now change the style...

# utf8fx approach (maintainable):
{{mathbold:separator=dot}}TITLE{{/mathbold}}  ← Change style="script" in one place
```

Think of it like CSS for text: separate content from presentation, gain power through abstraction.

## {{mathbold}}Features{{/mathbold}}

- Convert text to {{negative-squared}}19{{/negative-squared}} different Unicode styles
- Custom separators (dots, dashes, arrows) between characters
- Decorative frames around text (gradient, solid, lines)
- Enclosed alphanumeric badges (①②③, ⒜⒝⒞, ❶❷❸)
- Composable templates (style + separator + frame + badges)
- Style aliases for shorter names (e.g., `mb` for `mathbold`)
- Preserves whitespace, punctuation, and unsupported characters
- Zero-copy operations for maximum performance
- Comprehensive error handling
- Rust library with CLI and planned WASM bindings

## {{mathbold}}Available Styles{{/mathbold}}

### {{sans-serif-bold}}Bold & Emphasis{{/sans-serif-bold}}
| Style | Example | Use Case |
|-------|---------|----------|
| `mathbold` | 𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓 | Professional headers |
| `fullwidth` | ＢＬＡＣＫＤＯＴ | Substantial emphasis |
| `sans-serif-bold` | 𝗕𝗟𝗔𝗖𝗞𝗗𝗢𝗧 | Modern, strong |
| `sans-serif-bold-italic` | 𝘽𝙇𝘼𝘾𝙆𝘿𝙊𝙏 | Maximum emphasis |

### {{sans-serif-bold}}Boxed Styles{{/sans-serif-bold}}
| Style | Example | Use Case |
|-------|---------|----------|
| `negative-squared` | 🅱🅻🅰🅲🅺🅳🅾🆃 | Maximum contrast |
| `negative-circled` | 🅑🅛🅐🅒🅚🅓🅞🅣 | Bold, rounded |
| `squared-latin` | 🄱🄻🄰🄲🄺🄳🄾🅃 | Elegant boxes |
| `circled-latin` | Ⓑⓛⓐⓒⓚⓓⓞⓣ | Playful circles |

### {{sans-serif-bold}}Elegant & Script{{/sans-serif-bold}}
| Style | Example | Use Case |
|-------|---------|----------|
| `script` | 𝐵𝐿𝒜𝒞𝒦𝒟𝒪𝒯 | Elegant cursive |
| `bold-script` | 𝓑𝓛𝓐𝓒𝓚𝓓𝓞𝓣 | Heavy cursive |
| `fraktur` | 𝔅𝔏𝔄ℭ𝔎𝔇𝔒𝔗 | Gothic/blackletter |
| `bold-fraktur` | 𝕭𝕷𝕬𝕮𝕶𝕯𝕺𝕿 | Heavy Gothic |
| `italic` | 𝐵𝐿𝐴𝐶𝐾𝐷𝑂𝑇 | Flowing emphasis |
| `bold-italic` | 𝑩𝑳𝑨𝑪𝑲𝑫𝑶𝑻 | Strong + flow |
| `small-caps` | ʙʟᴀᴄᴋᴅᴏᴛ | Subtle elegance |

### {{sans-serif-bold}}Technical{{/sans-serif-bold}}
| Style | Example | Use Case |
|-------|---------|----------|
| `monospace` | 𝚋𝚕𝚊𝚌𝚔𝚍𝚘𝚝 | Code-like |
| `double-struck` | 𝔹𝕃𝔸ℂ𝕂𝔻𝕆𝕋 | Outline style |
| `sans-serif` | 𝖡𝖫𝖠𝖢𝖪𝖣𝖮𝖳 | Clean, modern |
| `sans-serif-italic` | 𝘉𝘓𝘈𝘊𝘒𝘋𝘖𝘛 | Modern slant |

### {{sans-serif-bold}}Adding Custom Styles{{/sans-serif-bold}}

Want to add your own Unicode style? It's just JSON:

1. **Find your Unicode range** (e.g., [Unicode Mathematical Alphanumeric Symbols](https://en.wikipedia.org/wiki/Mathematical_Alphanumeric_Symbols))
2. **Edit `data/styles.json`** and add your mappings:

```json
{
  "id": "my-custom-style",
  "name": "My Custom Style",
  "category": "Custom",
  "description": "Your custom Unicode transformation",
  "aliases": ["custom", "mcs"],
  "uppercase": {
    "A": "𝒜",
    "B": "ℬ",
    ...
  },
  "lowercase": {
    "a": "𝒶",
    "b": "𝒷",
    ...
  },
  "digits": {
    "0": "𝟢",
    ...
  }
}
```

3. **Use it immediately**: `{{my-custom-style}}TEXT{{/my-custom-style}}`

No code changes needed - utf8fx automatically picks up new styles from the JSON file.

## {{mathbold}}Quick Start{{/mathbold}}

### {{sans-serif-bold}}Library Usage{{/sans-serif-bold}}

```rust
use utf8fx::{Converter, FrameRenderer, BadgeRenderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = Converter::new()?;

    // Convert text directly
    let result = converter.convert("HELLO WORLD", "mathbold")?;
    println!("{}", result); // 𝐇𝐄𝐋𝐋𝐎 𝐖𝐎𝐑𝐋𝐃

    // Use aliases
    let result = converter.convert("Test", "mb")?;
    println!("{}", result); // 𝐓𝐞𝐬𝐭

    // Add spacing between characters
    let result = converter.convert_with_spacing("HELLO", "mathbold", 1)?;
    println!("{}", result); // 𝐇 𝐄 𝐋 𝐋 𝐎

    // Add custom separators
    let result = converter.convert_with_separator("TITLE", "mathbold", "·", 1)?;
    println!("{}", result); // 𝐓·𝐈·𝐓·𝐋·𝐄

    // Add decorative frames
    let frame_renderer = FrameRenderer::new()?;
    let styled = converter.convert("HEADER", "mathbold")?;
    let result = frame_renderer.apply_frame(&styled, "gradient")?;
    println!("{}", result); // ▓▒░ 𝐇𝐄𝐀𝐃𝐄𝐑 ░▒▓

    // Apply badges
    let badge_renderer = BadgeRenderer::new()?;
    let result = badge_renderer.apply_badge("1", "circle")?;
    println!("{}", result); // ①

    // List available styles
    for style in converter.list_styles() {
        println!("{}: {}", style.id, style.name);
    }

    Ok(())
}
```

### {{sans-serif-bold}}CLI Usage{{/sans-serif-bold}}

```bash
# Convert text
utf8fx convert --style mathbold "HELLO WORLD"

# Add spacing between characters
utf8fx convert --style mathbold --spacing 1 "HEADER"
# Output: 𝐇 𝐄 𝐀 𝐃 𝐄 𝐑

# Process markdown files with templates
utf8fx process input.md -o output.md
```

### {{sans-serif-bold}}Template Syntax{{/sans-serif-bold}}

Add Unicode styling directly in your markdown:

```markdown
# {{mathbold}}TITLE{{/mathbold}}

## Style with Spacing
Use {{script:spacing=2}}elegant spacing{{/script}} for headers.

## Style with Separators
{{mathbold:separator=dot}}T I T L E{{/mathbold}}
{{mathbold:separator=dash}}H E A D E R{{/mathbold}}
{{mathbold:separator=arrow}}F L O W{{/mathbold}}

## Decorative Frames
{{frame:gradient}}Important Note{{/frame}}
{{frame:solid-left}}Action Item{{/frame}}
{{frame:line-bold}}Section Header{{/frame}}

## Composition (Style + Separator + Frame)
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}

## Warnings and Alerts
{{negative-squared:spacing=1}}WARNING{{/negative-squared}}
```

Available separators: `dot` (·), `bullet` (•), `dash` (─), `bolddash` (━), `arrow` (→)

Available frames: `gradient`, `solid-left`, `solid-right`, `solid-both`, `line-light`, `line-bold`, `line-double`, `line-dashed`, `block-top`, `block-bottom`, `arrow-right`, `dot`, `bullet`

## {{mathbold}}Badges{{/mathbold}}

Badges are pre-composed Unicode characters that enclose numbers or letters. Unlike styles (which map every character) or frames (which add decorations), badges are limited to specific charsets:

**Numbers (0-20):**
```markdown
Step {{badge:circle}}1{{/badge}}: Install
Priority {{badge:negative-circle}}1{{/badge}} task
Section {{badge:paren}}3{{/badge}} complete
Item {{badge:period}}5{{/badge}} pending
```

Output:
```
Step ①: Install
Priority ❶ task
Section ⑶ complete
Item 🄅 pending
```

**Letters (a-z):**
```markdown
Option {{badge:paren-letter}}a{{/badge}}: Accept
Option {{badge:paren-letter}}b{{/badge}}: Reject
```

Output:
```
Option ⒜: Accept
Option ⒝: Reject
```

**Available badge types:**
- `circle` - Circled numbers ①②③ (0-20) - aliases: `circled`, `number-circle`
- `negative-circle` - White on black ❶❷❸ (0-20) - aliases: `neg-circle`, `inverse-circle`
- `double-circle` - Double circles ⓵⓶⓷ (1-10) - aliases: `double`, `dbl-circle`
- `paren` - Parenthesized numbers ⑴⑵⑶ (1-20) - aliases: `parenthesized`, `parens`
- `period` - Period-terminated 🄁🄂🄃 (0-20) - aliases: `dot-number`, `period-number`
- `paren-letter` - Parenthesized letters ⒜⒝⒞ (a-z) - aliases: `letter-paren`, `alpha-paren`

**Important:** Badges have limited charset support - attempting to badge unsupported characters (like "99" or uppercase letters) will return an error.

### {{sans-serif-bold}}Visual Examples{{/sans-serif-bold}}

**Before (README.template.md):**
```markdown
# {{mathbold}}utf8fx{{/mathbold}}

{{frame:gradient}}{{mathbold:separator=dot}}FEATURES{{/mathbold}}{{/frame}}

- {{negative-squared}}HIGH{{/negative-squared}} contrast alerts
- {{script:spacing=1}}Elegant{{/script}} headers
```

**After (README.md generated by `utf8fx process`):**
```markdown
# 𝐮𝐭𝐟𝟖𝐟𝐱

▓▒░ 𝐅·𝐄·𝐀·𝐓·𝐔·𝐑·𝐄·𝐒 ░▒▓

- 🅷🅸🅶🅷 contrast alerts
- ℰ 𝓁 ℯ 𝓰 𝒶 𝓃 𝓉 headers
```

**Command:**
```bash
utf8fx process README.template.md -o README.md
```

This README was generated using utf8fx - check `README.template.md` to see the source!

### {{sans-serif-bold}}Installation{{/sans-serif-bold}}

Add to your `Cargo.toml`:

```toml
[dependencies]
utf8fx = "1.0"
```

## {{mathbold}}Project Structure{{/mathbold}}

```
utf8fx/
├── src/
│   ├── lib.rs          # Public API
│   ├── converter.rs    # Core conversion logic
│   ├── frames.rs       # Frame rendering
│   ├── badges.rs       # Badge rendering
│   ├── parser.rs       # Template parser
│   ├── styles.rs       # Style definitions
│   └── error.rs        # Error types
├── data/
│   ├── styles.json     # Character mapping database
│   ├── frames.json     # Frame definitions
│   └── badges.json     # Badge definitions
├── tests/              # Integration tests
├── examples/           # Usage examples
└── docs/               # Documentation
```

## {{mathbold}}Documentation{{/mathbold}}

- [API Guide](docs/API-GUIDE.md) - Complete API reference with examples
- [Architecture](docs/ARCHITECTURE.md) - System design and component architecture
- [Parser Design](docs/parser-design.md) - State machine implementation details
- [Planning Document](docs/PLANNING.md) - Technical design and roadmap
- [Unicode Design Elements](docs/unicode-design-elements.md) - Character reference

## {{mathbold}}Testing{{/mathbold}}

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_convert_mathbold
```

## {{mathbold}}Contributing{{/mathbold}}

Contributions are welcome! Please read our contributing guidelines (coming soon) before submitting PRs.

## {{mathbold}}License{{/mathbold}}

MIT License - see LICENSE file for details

## {{mathbold}}Links{{/mathbold}}

- [GitHub Repository](https://github.com/blackwell-systems/utf8fx)
- [Crates.io](https://crates.io/crates/utf8fx) (coming soon)
- [Documentation](https://docs.rs/utf8fx) (coming soon)
