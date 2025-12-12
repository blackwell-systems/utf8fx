# utf8fx

**Unicode text effects for markdown and beyond**

Transform text into various Unicode styles (mathematical bold, full-width, negative squared, and more) through a powerful markdown preprocessing system.

## Features

- Convert text to 19 different Unicode styles
- Style aliases for shorter names (e.g., `mb` for `mathbold`)
- Preserves whitespace, punctuation, and unsupported characters
- Zero-copy operations for maximum performance
- Comprehensive error handling
- Rust library with planned CLI and WASM bindings

## Available Styles

### Bold & Emphasis
| Style | Example | Use Case |
|-------|---------|----------|
| `mathbold` | 𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓 | Professional headers |
| `fullwidth` | ＢＬＡＣＫＤＯＴ | Substantial emphasis |
| `sans-serif-bold` | 𝗕𝗟𝗔𝗖𝗞𝗗𝗢𝗧 | Modern, strong |
| `sans-serif-bold-italic` | 𝘽𝙇𝘼𝘾𝙆𝘿𝙊𝙏 | Maximum emphasis |

### Boxed Styles
| Style | Example | Use Case |
|-------|---------|----------|
| `negative-squared` | 🅱🅻🅰🅲🅺🅳🅾🆃 | Maximum contrast |
| `negative-circled` | 🅑🅛🅐🅒🅚🅓🅞🅣 | Bold, rounded |
| `squared-latin` | 🄱🄻🄰🄲🄺🄳🄾🅃 | Elegant boxes |
| `circled-latin` | Ⓑⓛⓐⓒⓚⓓⓞⓣ | Playful circles |

### Elegant & Script
| Style | Example | Use Case |
|-------|---------|----------|
| `script` | 𝐵𝐿𝒜𝒞𝒦𝒟𝒪𝒯 | Elegant cursive |
| `bold-script` | 𝓑𝓛𝓐𝓒𝓚𝓓𝓞𝓣 | Heavy cursive |
| `fraktur` | 𝔅𝔏𝔄ℭ𝔎𝔇𝔒𝔗 | Gothic/blackletter |
| `bold-fraktur` | 𝕭𝕷𝕬𝕮𝕶𝕯𝕺𝕿 | Heavy Gothic |
| `italic` | 𝐵𝐿𝐴𝐶𝐾𝐷𝑂𝑇 | Flowing emphasis |
| `bold-italic` | 𝑩𝑳𝑨𝑪𝑲𝑫𝑶𝑻 | Strong + flow |
| `small-caps` | ʙʟᴀᴄᴋᴅᴏᴛ | Subtle elegance |

### Technical
| Style | Example | Use Case |
|-------|---------|----------|
| `monospace` | 𝚋𝚕𝚊𝚌𝚔𝚍𝚘𝚝 | Code-like |
| `double-struck` | 𝔹𝕃𝔸ℂ𝕂𝔻𝕆𝕋 | Outline style |
| `sans-serif` | 𝖡𝖫𝖠𝖢𝖪𝖣𝖮𝖳 | Clean, modern |
| `sans-serif-italic` | 𝘉𝘓𝘈𝘊𝘒𝘋𝘖𝘛 | Modern slant |

## Quick Start

### Library Usage

```rust
use utf8fx::Converter;

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

    // List available styles
    for style in converter.list_styles() {
        println!("{}: {}", style.id, style.name);
    }

    Ok(())
}
```

### CLI Usage

```bash
# Convert text
utf8fx convert --style mathbold "HELLO WORLD"

# Add spacing between characters
utf8fx convert --style mathbold --spacing 1 "HEADER"
# Output: 𝐇 𝐄 𝐀 𝐃 𝐄 𝐑

# Process markdown files with templates
utf8fx process input.md -o output.md
```

### Template Syntax

Add Unicode styling directly in your markdown:

```markdown
# {{mathbold}}TITLE{{/mathbold}}

Use {{script:spacing=2}}elegant spacing{{/script}} for headers.

{{negative-squared:spacing=1}}WARNING{{/negative-squared}}
```

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
utf8fx = "0.1"
```

## Project Structure

```
utf8fx/
├── src/
│   ├── lib.rs          # Public API
│   ├── converter.rs    # Core conversion logic
│   ├── styles.rs       # Style definitions
│   └── error.rs        # Error types
├── data/
│   └── styles.json     # Character mapping database
├── tests/              # Integration tests
├── examples/           # Usage examples
└── docs/               # Documentation
```

## Documentation

- [Planning Document](PLANNING.md) - Technical design and roadmap
- [Unicode Design Elements](unicode-design-elements.md) - Character reference
- [API Documentation](https://docs.rs/utf8fx) - Full API docs (coming soon)

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_convert_mathbold
```

## Contributing

Contributions are welcome! Please read our contributing guidelines (coming soon) before submitting PRs.

## License

MIT License - see LICENSE file for details

## Links

- [GitHub Repository](https://github.com/utf8fx/utf8fx-rs)
- [Crates.io](https://crates.io/crates/utf8fx) (coming soon)
- [Documentation](https://docs.rs/utf8fx) (coming soon)
