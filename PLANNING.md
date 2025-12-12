# utf8fx - Project Planning Document

**Version:** 0.1.0
**Status:** Planning Phase
**Last Updated:** 2025-12-12

---

## Project Overview

**utf8fx** is a Unicode text styling tool for markdown and plain text. It transforms text into various Unicode styles (mathematical bold, full-width, negative squared, etc.) through a powerful markdown preprocessing system.

### Core Value Proposition

- **Markdown preprocessors** can process `{{style}}text{{/style}}` templates
- **CLI tool** for batch conversion and file processing
- **Rust library** for embedding in other tools
- **WASM bindings** for browser-based editors and VS Code extensions
- **Platform-agnostic** - works with any static site generator or markdown processor

### Target Users

1. **Technical bloggers** - Style headers and emphasis in blog posts
2. **Documentation writers** - Create distinctive section markers
3. **Markdown tool developers** - Embed utf8fx in their tools
4. **Content creators** - Generate fancy text for social media, READMEs

---

## Technical Architecture

### Core Components

```
utf8fx/
├── src/
│   ├── lib.rs              # Public library interface
│   ├── converter.rs        # Character mapping and conversion logic
│   ├── parser.rs           # Template parser ({{style}}...{{/style}})
│   ├── styles.rs           # Style definitions and loading
│   └── error.rs            # Error types
├── cli/
│   ├── main.rs             # CLI entry point
│   ├── commands.rs         # CLI subcommands
│   └── output.rs           # Output formatting
├── wasm/                   # WASM bindings (Phase 3)
│   ├── lib.rs
│   └── pkg/
├── data/
│   └── styles.json         # Character mapping database
├── examples/               # Usage examples
├── tests/                  # Integration tests
├── benches/                # Benchmarks
└── docs/                   # Documentation
```

### Data Structure

**styles.json** - Single source of truth for all character mappings:

```json
{
  "version": "1.0.0",
  "styles": {
    "mathbold": {
      "id": "mathbold",
      "name": "Mathematical Bold",
      "description": "Bold serif letters for professional emphasis",
      "category": "bold",
      "unicode_block": "Mathematical Alphanumeric Symbols",
      "supports": {
        "uppercase": true,
        "lowercase": true,
        "numbers": true,
        "symbols": false
      },
      "mappings": {
        "A": "𝐀", "B": "𝐁", "C": "𝐂", ...
        "a": "𝐚", "b": "𝐛", "c": "𝐜", ...
        "0": "𝟎", "1": "𝟏", "2": "𝟐", ...
      }
    }
  }
}
```

---

## Feature Roadmap

### Phase 1: Core Library (Week 1-2)

**Goal:** Functional Rust library with character conversion

**Features:**
- [x] Load styles.json at compile time (via `include_str!`)
- [x] Character-by-character conversion for 11 styles
- [x] Style validation and error handling
- [x] Unit tests for all style conversions
- [x] Basic benchmarks

**Deliverables:**
- `Converter` struct with `convert()` method
- `Style` enum for available styles
- Comprehensive test suite
- Documentation with examples

**API Example:**
```rust
use utf8fx::Converter;

let converter = Converter::new()?;
let result = converter.convert("BLACKDOT", "mathbold")?;
assert_eq!(result, "𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓");
```

---

### Phase 2: CLI Tool (Week 2-3)

**Goal:** Production-ready CLI for markdown preprocessing

**Features:**
- [x] `utf8fx convert` - Convert text directly
- [x] `utf8fx process` - Process markdown files with templates
- [x] `utf8fx list` - List available styles
- [x] `--in-place` flag for file modification
- [x] Glob pattern support for batch processing
- [x] Progress bars for large operations
- [x] Colored output and error messages

**CLI Interface:**

```bash
# Convert text
utf8fx convert --style mathbold "BLACKDOT"
utf8fx convert -s negative-squared "WARNING"

# Process single file
utf8fx process post.md -o post-processed.md

# Process multiple files
utf8fx process content/**/*.md --in-place

# List styles
utf8fx list
utf8fx list --category bold
utf8fx list --show-samples

# Preview template
utf8fx preview post.md
```

**Template Syntax:**

```markdown
# {{mathbold}}BLACKDOT{{/mathbold}}

This is a {{negative-squared}}WARNING{{/negative-squared}} message.

Code blocks are preserved:
```bash
echo "{{mathbold}}not processed{{/mathbold}}"
```
```

**Deliverables:**
- Fully functional CLI with clap
- Man page and shell completions
- CI/CD for releases (GitHub Actions)
- Binary releases for Linux, macOS, Windows

---

### Phase 3: WASM Bindings (Week 3-4)

**Goal:** Browser-based usage and VS Code extension support

**Features:**
- [x] Compile to WASM with wasm-pack
- [x] JavaScript/TypeScript bindings
- [x] npm package: `utf8fx`
- [x] Web demo/playground
- [x] VS Code extension (future)

**WASM API:**

```javascript
import init, { convert, processTemplate } from 'utf8fx';

await init();

// Convert text
const result = convert("BLACKDOT", "mathbold");
// => "𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓"

// Process template string
const markdown = "# {{mathbold}}Title{{/mathbold}}";
const processed = processTemplate(markdown);
```

**Web Demo Features:**
- Live markdown editor with split preview
- Style selector dropdown
- Copy to clipboard
- Syntax highlighting
- Mobile responsive

**Deliverables:**
- WASM package published to npm
- Hosted web demo at utf8fx.dev
- Integration examples

---

### Phase 4: Ecosystem Integration (Week 4+)

**Goal:** Make utf8fx easy to integrate with existing tools

**Features:**
- [x] Python bindings (PyO3)
- [x] Hugo shortcode templates
- [x] GitHub Action for CI/CD
- [x] Markdown-it plugin
- [x] mdBook preprocessor
- [x] Zola integration guide

**Example Integrations:**

**GitHub Action:**
```yaml
- uses: utf8fx/utf8fx-action@v1
  with:
    files: 'content/**/*.md'
    in-place: true
```

**Makefile (works with any SSG):**
```makefile
preprocess:
	utf8fx process content/**/*.md --in-place

build: preprocess
	hugo build
```

**Hugo Shortcode:**
```html
<!-- layouts/shortcodes/ustyle.html -->
{{ $style := .Get 0 }}
{{ $text := .Inner }}
<!-- Calls utf8fx at build time -->
```

---

## Supported Unicode Styles

### Planned Styles (11 total)

| Style ID | Name | Example | Use Case |
|----------|------|---------|----------|
| `mathbold` | Mathematical Bold | 𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓 | Professional headers |
| `fullwidth` | Full-Width | ＢＬＡＣＫＤＯＴ | Substantial emphasis |
| `negative-squared` | Negative Squared | 🅱🅻🅰🅲🅺🅳🅾🆃 | Maximum contrast |
| `negative-circled` | Negative Circled | 🅑🅛🅐🅒🅚🅓🅞🅣 | Bold, rounded |
| `squared-latin` | Squared Latin | 🄱🄻🄰🄲🄺🄳🄾🅃 | Elegant boxes |
| `small-caps` | Small Caps | ʙʟᴀᴄᴋᴅᴏᴛ | Subtle elegance |
| `monospace` | Monospace | 𝚋𝚕𝚊𝚌𝚔𝚍𝚘𝚝 | Code-like |
| `double-struck` | Double-Struck | 𝔹𝕃𝔸ℂ𝕂𝔻𝕆𝕋 | Outline style |
| `sans-serif-bold` | Sans-Serif Bold | 𝗕𝗟𝗔𝗖𝗞𝗗𝗢𝗧 | Modern, strong |
| `italic` | Italic | 𝐵𝐿𝐴𝐶𝐾𝐷𝑂𝑇 | Flowing emphasis |
| `bold-italic` | Bold Italic | 𝑵𝑶𝑻𝑬 | Strong + flow |

### Style Categories

```rust
pub enum StyleCategory {
    Bold,       // mathbold, sans-serif-bold, fullwidth
    Boxed,      // negative-squared, negative-circled, squared-latin
    Technical,  // monospace, double-struck
    Elegant,    // small-caps, italic, bold-italic
}
```

---

## Implementation Details

### Core Data Structures

```rust
// src/lib.rs
pub struct Converter {
    styles: HashMap<String, Style>,
}

// src/styles.rs
pub struct Style {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: StyleCategory,
    pub supports: StyleSupport,
    pub mappings: HashMap<char, char>,
}

pub struct StyleSupport {
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

pub enum StyleCategory {
    Bold,
    Boxed,
    Technical,
    Elegant,
}

// src/converter.rs
impl Converter {
    pub fn new() -> Result<Self, Error>;
    pub fn convert(&self, text: &str, style: &str) -> Result<String, Error>;
    pub fn list_styles(&self) -> Vec<&Style>;
    pub fn get_style(&self, id: &str) -> Option<&Style>;
}

// src/parser.rs
pub struct TemplateParser {
    converter: Converter,
}

impl TemplateParser {
    pub fn new(converter: Converter) -> Self;
    pub fn process(&self, markdown: &str) -> Result<String, Error>;
    pub fn process_file(&self, path: &Path) -> Result<String, Error>;
}

// src/error.rs
pub enum Error {
    UnknownStyle(String),
    InvalidTemplate(String),
    IoError(std::io::Error),
    ParseError(String),
}
```

### Template Parser Logic

**Requirements:**
1. Find `{{style}}text{{/style}}` patterns
2. Skip code blocks (` ``` `, ` `` `, `` ` ``)
3. Skip inline code (`` `code` ``)
4. Handle nested styles (error or process inner-first?)
5. Preserve markdown structure

**Implementation Strategy:**

```rust
pub fn process(&self, markdown: &str) -> Result<String, Error> {
    // 1. Parse markdown into AST or line-by-line
    // 2. Track state: in_code_block, in_inline_code
    // 3. Use regex to find {{style}}...{{/style}} when not in code
    // 4. Convert matched text using converter
    // 5. Replace in original string
    // 6. Return processed markdown
}
```

**Regex Pattern:**
```rust
let re = Regex::new(r"\{\{(\w+(?:-\w+)*)\}\}(.*?)\{\{/\1\}\}")?;
```

**Edge Cases:**
- Unclosed tags: `{{mathbold}}text` (error)
- Mismatched tags: `{{mathbold}}text{{/italic}}` (error)
- Empty content: `{{mathbold}}{{/mathbold}}` (return empty)
- Unknown style: `{{fakestyle}}text{{/fakestyle}}` (error)

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathbold_uppercase() {
        let converter = Converter::new().unwrap();
        assert_eq!(
            converter.convert("ABC", "mathbold").unwrap(),
            "𝐀𝐁𝐂"
        );
    }

    #[test]
    fn test_unknown_style() {
        let converter = Converter::new().unwrap();
        assert!(converter.convert("ABC", "fakestyle").is_err());
    }

    #[test]
    fn test_template_processing() {
        let parser = TemplateParser::new(Converter::new().unwrap());
        let input = "# {{mathbold}}TITLE{{/mathbold}}";
        let expected = "# 𝐓𝐈𝐓𝐋𝐄";
        assert_eq!(parser.process(input).unwrap(), expected);
    }

    #[test]
    fn test_skip_code_blocks() {
        let parser = TemplateParser::new(Converter::new().unwrap());
        let input = "```\n{{mathbold}}CODE{{/mathbold}}\n```";
        assert_eq!(parser.process(input).unwrap(), input);
    }
}
```

### Integration Tests

```rust
// tests/cli_tests.rs
#[test]
fn test_convert_command() {
    let output = Command::cargo_bin("utf8fx")
        .arg("convert")
        .arg("--style")
        .arg("mathbold")
        .arg("TEST")
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "𝐓𝐄𝐒𝐓\n");
}
```

### Benchmark Tests

```rust
// benches/conversion_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_convert(c: &mut Criterion) {
    let converter = Converter::new().unwrap();

    c.bench_function("convert_short", |b| {
        b.iter(|| converter.convert(black_box("HELLO"), "mathbold"))
    });

    c.bench_function("convert_long", |b| {
        let text = "Lorem ipsum dolor sit amet".repeat(100);
        b.iter(|| converter.convert(black_box(&text), "mathbold"))
    });
}

criterion_group!(benches, benchmark_convert);
criterion_main!(benches);
```

---

## Performance Targets

### Benchmarks

| Operation | Target | Notes |
|-----------|--------|-------|
| Load styles.json | < 1ms | Compile-time embed |
| Convert short text (10 chars) | < 10μs | Character mapping |
| Convert long text (1000 chars) | < 100μs | Linear with input |
| Process markdown file (10KB) | < 5ms | Parse + convert |
| Process 1000 files | < 2s | Batch processing |

### Memory Usage

- **Styles data:** ~200KB in memory (all 11 styles)
- **Per-conversion overhead:** Minimal (no allocations in hot path)
- **CLI binary size:** < 5MB (statically linked)
- **WASM bundle size:** < 500KB (with wasm-opt)

---

## CLI User Experience

### Help Text

```
utf8fx 0.1.0
Unicode text effects for markdown

USAGE:
    utf8fx <SUBCOMMAND>

SUBCOMMANDS:
    convert     Convert text to a Unicode style
    process     Process markdown files with style templates
    list        List available styles
    preview     Preview processed markdown
    help        Print this message or the help of the given subcommand(s)

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

### Example Usage

```bash
# Quick conversion
$ utf8fx convert -s mathbold "BLACKDOT"
𝐁𝐋𝐀𝐂𝐊𝐃𝐎𝐓

# Process with template
$ cat post.md
# {{mathbold}}Title{{/mathbold}}
Content here

$ utf8fx process post.md
# 𝐓𝐢𝐭𝐥𝐞
Content here

# Batch process (with progress bar)
$ utf8fx process content/**/*.md --in-place
Processing: [████████████████████] 234/234 files (1.2s)
✓ Processed 234 files

# List styles
$ utf8fx list
Available styles (11):

Bold & Impactful:
  mathbold          Mathematical Bold (𝐀𝐁𝐂)
  sans-serif-bold   Sans-Serif Bold (𝗔𝗕𝗖)
  fullwidth         Full-Width (ＡＢＣ)

Boxed:
  negative-squared  Negative Squared (🅰🅱🅲)
  negative-circled  Negative Circled (🅐🅑🅒)
  squared-latin     Squared Latin (🄰🄱🄲)

... (more styles)
```

---

## Error Handling

### Error Types

```rust
pub enum Error {
    // Style errors
    UnknownStyle(String),           // "Style 'fakestyle' not found"
    StyleNotSupported(String),      // "Style doesn't support numbers"

    // Template errors
    UnclosedTag(String),            // "Unclosed tag: {{mathbold}}"
    MismatchedTags(String, String), // "Expected {{/mathbold}}, found {{/italic}}"
    InvalidStyleName(String),       // "Style name contains invalid chars"

    // IO errors
    FileNotFound(PathBuf),
    PermissionDenied(PathBuf),
    IoError(std::io::Error),

    // Parse errors
    InvalidJson(String),
    InvalidUtf8(String),
}
```

### Error Messages

**Good error messages with context:**

```
Error: Unknown style 'mathbod'
  Did you mean: mathbold?

  Available styles:
    - mathbold
    - fullwidth
    - negative-squared

  Run `utf8fx list` to see all styles.
```

```
Error: Unclosed tag at line 5
  5 | # {{mathbold}}TITLE
                 ^^^^^^^

  Expected: {{/mathbold}}
```

---

## Documentation Plan

### README.md

- Quick start guide
- Installation instructions
- Basic usage examples
- Link to full documentation

### docs/

- `installation.md` - Installation methods
- `cli-reference.md` - Complete CLI documentation
- `library-usage.md` - Rust API examples
- `styles.md` - Style gallery with samples
- `templates.md` - Template syntax guide
- `integrations.md` - SSG integration guides
- `wasm.md` - WASM/JavaScript usage
- `contributing.md` - Contribution guidelines

### API Documentation

```rust
/// Converts text to a specified Unicode style.
///
/// # Examples
///
/// ```
/// use utf8fx::Converter;
///
/// let converter = Converter::new()?;
/// let result = converter.convert("HELLO", "mathbold")?;
/// assert_eq!(result, "𝐇𝐄𝐋𝐋𝐎");
/// ```
///
/// # Errors
///
/// Returns `Error::UnknownStyle` if the style doesn't exist.
pub fn convert(&self, text: &str, style: &str) -> Result<String, Error>
```

---

## Release Strategy

### Version Numbering

Following semantic versioning (SemVer):

- `0.1.0` - Initial release (Phase 1: Library)
- `0.2.0` - CLI tool added (Phase 2)
- `0.3.0` - WASM bindings (Phase 3)
- `1.0.0` - Stable API, production-ready

### Distribution Channels

**Rust:**
- crates.io: `utf8fx`
- docs.rs: Automatic documentation

**Binary releases:**
- GitHub Releases (Linux, macOS, Windows)
- Homebrew tap: `brew install utf8fx/tap/utf8fx`
- Cargo install: `cargo install utf8fx`

**WASM/JavaScript:**
- npm: `utf8fx`
- unpkg CDN: `https://unpkg.com/utf8fx`

**Python (future):**
- PyPI: `utf8fx`

---

## Open Questions

### Technical Decisions

1. **Template nesting:** Allow `{{mathbold}}{{italic}}text{{/italic}}{{/mathbold}}`?
   - **Decision:** Error on nested tags (simpler, clearer)

2. **Unknown characters:** What to do with chars not in mapping?
   - **Decision:** Pass through unchanged (e.g., emoji, punctuation)

3. **Code block detection:** Parse full markdown AST or simple regex?
   - **Decision:** Start with regex (fast), upgrade if needed

4. **Style aliasing:** Allow `mb` as alias for `mathbold`?
   - **Decision:** Yes, add `aliases` field to style definitions

5. **Custom styles:** Allow users to define their own mappings?
   - **Decision:** Future feature (v2.0)

### Naming Conventions

- **Style IDs:** kebab-case (`negative-squared`)
- **Rust types:** PascalCase (`NegativeSquared`)
- **CLI flags:** kebab-case (`--in-place`)
- **JSON keys:** snake_case (`unicode_block`)

---

## Dependencies

### Core Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
regex = "1.10"
thiserror = "1.0"

[dev-dependencies]
criterion = "0.5"
tempfile = "3.8"
```

### CLI Dependencies

```toml
[dependencies]
clap = { version = "4.4", features = ["derive", "cargo"] }
indicatif = "0.17"
colored = "2.1"
glob = "0.3"
```

### WASM Dependencies

```toml
[dependencies]
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
```

---

## Success Metrics

### Phase 1 (Library)
- [ ] All 11 styles implemented
- [ ] 100% test coverage for conversions
- [ ] Comprehensive documentation
- [ ] Published to crates.io

### Phase 2 (CLI)
- [ ] All CLI commands functional
- [ ] Binary releases for 3 platforms
- [ ] 50+ GitHub stars
- [ ] 5+ crates.io downloads/day

### Phase 3 (WASM)
- [ ] WASM package published to npm
- [ ] Live web demo deployed
- [ ] 100+ npm downloads/week

### Phase 4 (Ecosystem)
- [ ] 3+ integration guides published
- [ ] Featured on at least one Rust blog/newsletter
- [ ] Used in at least 5 real-world projects

---

## Timeline

| Phase | Duration | Deliverables |
|-------|----------|--------------|
| Phase 1 | Week 1-2 | Core library + tests |
| Phase 2 | Week 2-3 | CLI tool + releases |
| Phase 3 | Week 3-4 | WASM + web demo |
| Phase 4 | Week 4+ | Integrations + ecosystem |

**Total estimated time:** 4-6 weeks to v1.0.0

---

## Next Steps

1. **Generate complete `styles.json`** with all 11 style mappings
2. **Initialize Rust project:** `cargo init --lib`
3. **Implement core `Converter`** struct
4. **Write comprehensive tests**
5. **Build CLI tool** with clap
6. **Create web demo** with WASM
7. **Write documentation** and examples
8. **Publish to crates.io** and npm

---

**Let's build this!** 🚀
