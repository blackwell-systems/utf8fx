# utf8fx

**Unicode text effects for markdown and beyond**

Transform text into various Unicode styles (mathematical bold, full-width, negative squared, and more) through a powerful markdown preprocessing system.

## Features

- Convert text to 11 different Unicode styles
- Style aliases for shorter names (e.g., `mb` for `mathbold`)
- Preserves whitespace, punctuation, and unsupported characters
- Zero-copy operations for maximum performance
- Comprehensive error handling
- Rust library with planned CLI and WASM bindings

## Available Styles

| Style | Example | Use Case |
|-------|---------|----------|
| `mathbold` | 𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓 | Professional headers |
| `fullwidth` | ＢＬＡＣＫＤＯＴ | Substantial emphasis |
| `negative-squared` | 🅱🅻🅰🅲🅺🅳🅾🆃 | Maximum contrast |
| `negative-circled` | 🅑🅛🅐🅒🅚🅓🅞🅣 | Bold, rounded |
| `squared-latin` | 🄱🄻🄰🄲🄺🄳🄾🅃 | Elegant boxes |
| `small-caps` | ʙʟᴀᴄᴋᴅᴏᴛ | Subtle elegance |
| `monospace` | 𝚋𝚕𝚊𝚌𝚔𝚍𝚘𝚝 | Code-like |
| `double-struck` | 𝔹𝕃𝔸ℂ𝕂𝔻𝕆𝕋 | Outline style |
| `sans-serif-bold` | 𝗕𝗟𝗔𝗖𝗞𝗗𝗢𝗧 | Modern, strong |
| `italic` | 𝐵𝐿𝐴𝐶𝐾𝐷𝑂𝑇 | Flowing emphasis |
| `bold-italic` | 𝑩𝑳𝑨𝑪𝑲𝑫𝑶𝑻 | Strong + flow |

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

    // List available styles
    for style in converter.list_styles() {
        println!("{}: {}", style.id, style.name);
    }

    Ok(())
}
```

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
utf8fx = "0.1"
```

## Development Status

**Phase 1: Core Library** - ✓ Complete
- Character mappings for 11 styles
- Comprehensive test suite
- Full documentation

**Phase 2: CLI Tool** - In Progress
- Command-line interface for text conversion
- Markdown file processing
- Template syntax support

**Phase 3: WASM Bindings** - Planned
- Browser-based usage
- npm package
- Web demo

**Phase 4: Ecosystem** - Planned
- Python bindings
- Hugo/Jekyll integrations
- VS Code extension

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

## Acknowledgments

- Unicode Consortium for character specifications
- Mathematical Alphanumeric Symbols (U+1D400–U+1D7FF)
- Enclosed Alphanumerics (U+2460–U+24FF, U+1F100–U+1F1FF)
- Halfwidth and Fullwidth Forms (U+FF00–U+FFEF)

## Links

- [GitHub Repository](https://github.com/utf8fx/utf8fx-rs)
- [Crates.io](https://crates.io/crates/utf8fx) (coming soon)
- [Documentation](https://docs.rs/utf8fx) (coming soon)
