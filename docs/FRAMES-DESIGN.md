# Frames & Boxes Design Document

**Feature:** Box drawing, brackets, and decorative frames for text
**Status:** Design Phase
**Version:** Draft 1.0
**Last Updated:** 2025-12-12

---

## Design Philosophy: Cohesive System Architecture

**Critical Principle:** Every feature must fit into a unified, extensible architecture. We're not bolting on features—we're expanding a coherent system.

### Core System Principles

1. **Single Responsibility** - Each component has one clear purpose:
   - `Converter` - Character-to-character Unicode mapping
   - `Parser` - Template syntax recognition and extraction
   - `FrameRenderer` - Structural elements (boxes, brackets)

2. **Consistent Syntax** - All templates follow the same pattern:
   - Base: `{{type}}content{{/type}}`
   - With params: `{{type:param=value}}content{{/type}}`
   - Composition: `{{type|modifier}}content{{/type}}`

3. **Layered Processing** - Clear pipeline stages:
   ```
   Input → Parser → Converter → FrameRenderer → Output
   ```

4. **Data-Driven** - Configuration over code:
   - `styles.json` - Character mappings
   - `frames.json` - Box drawing characters
   - Adding new styles/frames = data, not code

5. **Composable Operations** - Features combine cleanly:
   - Spacing applies to characters
   - Frames wrap rendered output
   - Order matters: style → space → frame

### Anti-Patterns to Avoid

❌ **Feature sprawl** - Adding every possible option
✅ **Focused features** - Each feature solves a real problem

❌ **Inconsistent syntax** - Different template styles for each feature
✅ **Unified syntax** - All templates use same pattern

❌ **Monolithic functions** - 500-line methods that do everything
✅ **Small, composable functions** - Single responsibility

❌ **Hard-coded values** - Character mappings in code
✅ **Data-driven** - All mappings in JSON files

❌ **Breaking changes** - New features break old templates
✅ **Backward compatibility** - Old templates always work

### How Frames Fit the Architecture

**Frames are structural**, not stylistic:
- **Styles** transform characters: `A` → `𝐀`
- **Frames** add structure: `text` → `[ text ]`
- **They compose**: styled text can be framed

**Parser remains unified:**
- Same state machine
- Same template recognition
- Just routes to different renderers

**Extension point is clear:**
- Want borders? Add to FrameRenderer
- Want colors? Add ColorRenderer
- Want transforms? Add TransformRenderer

All use the same template syntax and processing pipeline.

---

## Overview

Add support for framing text with Unicode box drawing characters, brackets, and decorative elements. This enables creating visual containers, emphasis boxes, warning banners, and artistic frames around text in markdown documents.

### Goals

1. **Visual Impact** - Create distinctive frames and boxes that stand out in text documents
2. **Flexibility** - Support inline brackets and multi-line boxes
3. **Composability** - Allow combining frames with existing text styles
4. **Simplicity** - Maintain intuitive template syntax consistent with current design
5. **Markdown Compatibility** - Preserve compatibility with markdown processors

### Non-Goals

- Complex ASCII art generation
- Dynamic box sizing based on terminal width
- Nested frames (at least initially)
- Color support (beyond existing terminal colors)

---

## Use Cases

### 1. Warning Banners

```markdown
{{box:heavy}}⚠️  WARNING: This action cannot be undone{{/box}}
```

Output:
```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ ⚠️  WARNING: This action cannot be undone ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

### 2. Section Headers

```markdown
{{box:double|mathbold}}INSTALLATION GUIDE{{/box}}
```

Output:
```
╔═══════════════════════╗
║ 𝐈𝐍𝐒𝐓𝐀𝐋𝐋𝐀𝐓𝐈𝐎𝐍 𝐆𝐔𝐈𝐃𝐄 ║
╚═══════════════════════╝
```

### 3. Inline Emphasis

```markdown
The {{bracket:corner}}config.json{{/bracket}} file contains settings.
```

Output:
```
The 「config.json」 file contains settings.
```

### 4. Key-Value Display

```markdown
{{bracket:square}}API_KEY{{/bracket}} = your-key-here
{{bracket:angle}}required{{/bracket}}
```

Output:
```
[ API_KEY ] = your-key-here
⟨ required ⟩
```

### 5. README Badges

```markdown
{{box:rounded}}v1.0.0{{/box}} {{bracket:square}}MIT License{{/bracket}}
```

Output:
```
╭─────────╮ [ MIT License ]
│ v1.0.0  │
╰─────────╯
```

---

## Design Approaches

### Approach 1: Inline Brackets/Decorators

**Complexity:** Low
**Implementation Time:** 1-2 days

#### Syntax

```markdown
{{bracket:style}}text{{/bracket}}
{{decor:style}}text{{/decor}}
```

#### Available Styles

**Brackets:**
- `square` → `[ text ]`
- `angle` → `⟨ text ⟩`
- `curly` → `{ text }`
- `double-angle` → `《 text 》`
- `corner` → `「text」`
- `white-corner` → `『text』`
- `tortoise` → `〔text〕`
- `black-lenticular` → `【text】`

**Decorators:**
- `stars` → `✨ text ✨`
- `arrows` → `→ text ←`
- `dots` → `• text •`
- `diamonds` → `◆ text ◆`
- `triangles` → `▶ text ◀`

#### Technical Implementation

```rust
// New module: src/frames.rs
pub enum BracketStyle {
    Square,
    Angle,
    Curly,
    DoubleAngle,
    Corner,
    WhiteCorner,
    Tortoise,
    BlackLenticular,
}

impl BracketStyle {
    pub fn wrap(&self, text: &str) -> String {
        match self {
            BracketStyle::Square => format!("[ {} ]", text),
            BracketStyle::Angle => format!("⟨ {} ⟩", text),
            BracketStyle::Curly => format!("{{ {} }}", text),
            BracketStyle::DoubleAngle => format!("《 {} 》", text),
            BracketStyle::Corner => format!("「{}」", text),
            BracketStyle::WhiteCorner => format!("『{}』", text),
            BracketStyle::Tortoise => format!("〔{}〕", text),
            BracketStyle::BlackLenticular => format!("【{}】", text),
        }
    }
}
```

#### Pros
- Simple to implement
- Works inline with existing text
- No layout complexity
- Fast processing

#### Cons
- Limited visual impact
- Not true "boxes"
- Single-line only

---

### Approach 2: Single-Line ASCII Boxes

**Complexity:** Medium
**Implementation Time:** 3-5 days

#### Syntax

```markdown
{{box:style}}text{{/box}}
{{box:style:padding=2}}text{{/box}}
```

#### Available Styles

**Light Weight:**
- `box` (light single):
  ```
  ┌─────────┐
  │ text    │
  └─────────┘
  ```

- `rounded`:
  ```
  ╭─────────╮
  │ text    │
  ╰─────────╯
  ```

**Heavy Weight:**
- `heavy` (bold):
  ```
  ┏━━━━━━━━━┓
  ┃ text    ┃
  ┗━━━━━━━━━┛
  ```

- `double`:
  ```
  ╔═════════╗
  ║ text    ║
  ╚═════════╝
  ```

**Block Style:**
- `thick` (solid blocks):
  ```
  █████████████
  █ text      █
  █████████████
  ```

- `shadow`:
  ```
  ┌─────────┐
  │ text    │▓
  └─────────┘▓
   ▓▓▓▓▓▓▓▓▓▓
  ```

#### Parameters

- `padding` - Spaces around text (default: 1)
  - `{{box:heavy:padding=3}}text{{/box}}`
  - Result: `┃   text   ┃`

- `width` - Fixed width (default: auto)
  - `{{box:double:width=20}}text{{/box}}`
  - Pads to exact width

- `align` - Text alignment (default: left)
  - Values: `left`, `center`, `right`
  - `{{box:rounded:align=center}}text{{/box}}`

#### Technical Implementation

```rust
pub struct BoxStyle {
    top_left: char,
    top: char,
    top_right: char,
    left: char,
    right: char,
    bottom_left: char,
    bottom: char,
    bottom_right: char,
}

impl BoxStyle {
    pub const LIGHT: Self = BoxStyle {
        top_left: '┌', top: '─', top_right: '┐',
        left: '│', right: '│',
        bottom_left: '└', bottom: '─', bottom_right: '┘',
    };

    pub const DOUBLE: Self = BoxStyle {
        top_left: '╔', top: '═', top_right: '╗',
        left: '║', right: '║',
        bottom_left: '╚', bottom: '═', bottom_right: '╝',
    };

    pub const HEAVY: Self = BoxStyle {
        top_left: '┏', top: '━', top_right: '┓',
        left: '┃', right: '┃',
        bottom_left: '┗', bottom: '━', bottom_right: '┛',
    };

    pub const ROUNDED: Self = BoxStyle {
        top_left: '╭', top: '─', top_right: '╮',
        left: '│', right: '│',
        bottom_left: '╰', bottom: '─', bottom_right: '╯',
    };
}

pub struct BoxOptions {
    pub style: BoxStyle,
    pub padding: usize,
    pub width: Option<usize>,
    pub align: TextAlign,
}

pub fn render_box(text: &str, options: BoxOptions) -> String {
    let padding = " ".repeat(options.padding);
    let content_width = text.chars().count() + (options.padding * 2);
    let total_width = options.width.unwrap_or(content_width);

    let horizontal = options.style.top.to_string().repeat(total_width);

    let aligned_text = match options.align {
        TextAlign::Left => format!("{}{}{}",
            padding, text, " ".repeat(total_width - content_width)),
        TextAlign::Center => {
            let spaces = total_width - content_width;
            let left_pad = spaces / 2;
            let right_pad = spaces - left_pad;
            format!("{}{}{}{}{}",
                " ".repeat(left_pad), padding, text, padding, " ".repeat(right_pad))
        },
        TextAlign::Right => format!("{}{}{}",
            " ".repeat(total_width - content_width), padding, text),
    };

    format!(
        "{}{}{}\n{}{}{}\n{}{}{}",
        options.style.top_left, horizontal, options.style.top_right,
        options.style.left, aligned_text, options.style.right,
        options.style.bottom_left, horizontal, options.style.bottom_right
    )
}
```

#### Pros
- Strong visual impact
- Professional appearance
- Flexible styling options
- Still relatively simple

#### Cons
- Multi-line output changes document layout
- Width calculation needed
- More complex parsing

---

### Approach 3: Multi-Line Content Boxes

**Complexity:** High
**Implementation Time:** 1-2 weeks

#### Syntax

```markdown
{{box:style}}
Line 1
Line 2
Line 3
{{/box}}
```

#### Features

- Automatically wraps multiple lines
- Calculates max line width
- Handles variable-length content
- Optional title bars

#### Example with Title

```markdown
{{box:double:title=Configuration}}
API_KEY=your-key
DATABASE_URL=postgres://...
DEBUG=true
{{/box}}
```

Output:
```
╔═══ Configuration ════════════════╗
║ API_KEY=your-key                 ║
║ DATABASE_URL=postgres://...      ║
║ DEBUG=true                       ║
╚══════════════════════════════════╝
```

#### Technical Challenges

1. **Line tracking** - Need to preserve newlines in template content
2. **Width calculation** - Find longest line for proper padding
3. **UTF-8 width** - Unicode characters have variable display widths
4. **Alignment** - Each line needs individual alignment
5. **Edge cases** - Empty lines, trailing spaces, very long lines

#### Pros
- Maximum flexibility
- Handles complex content
- Professional documentation appearance

#### Cons
- Complex implementation
- Performance considerations for large content
- Potential markdown compatibility issues

---

## Combined Approach (Recommended)

Implement features in phases:

### Phase 1: Inline Brackets (v1.1)
- Simple bracket wrapping
- Quick wins for inline emphasis
- Low complexity, high value
- **Timeline:** 1-2 days

### Phase 2: Single-Line Boxes (v1.2)
- ASCII box rendering for single lines
- Multiple box styles
- Padding and alignment options
- **Timeline:** 3-5 days

### Phase 3: Style + Box Composition (v1.3)
- Combine text styles with boxes
- Syntax: `{{mathbold|box:double}}TEXT{{/mathbold}}`
- Apply character style, then wrap in box
- **Timeline:** 2-3 days

### Phase 4: Multi-Line Boxes (v2.0)
- Full multi-line content support
- Title bars
- Smart width calculation
- **Timeline:** 1-2 weeks

---

## Template Syntax Design

### Current Parser Architecture

The parser uses a state machine to find `{{style}}...{{/style}}` patterns:

```rust
fn parse_template_at(&self, chars: &[char], start: usize)
    -> Result<Option<(usize, String, usize, String)>>
{
    // Returns: (end_pos, style, spacing, content)
    // Currently handles: {{style:spacing=N}}content{{/style}}
}
```

### Proposed Extensions

#### 1. Bracket Templates

```markdown
{{bracket:style}}content{{/bracket}}
```

Parser recognizes `bracket:` prefix, passes to bracket renderer instead of converter.

#### 2. Box Templates

```markdown
{{box:style}}content{{/box}}
{{box:style:padding=N}}content{{/box}}
{{box:style:width=N:align=center}}content{{/box}}
```

Parser extracts:
- Base type: `box`
- Style: `heavy`, `double`, `rounded`, etc.
- Parameters: `padding`, `width`, `align`

#### 3. Combined Templates

```markdown
{{mathbold|box:double}}TEXT{{/mathbold}}
```

Parser strategy:
1. Parse outer template: `mathbold`
2. Detect pipe `|` separator
3. Parse modifier: `box:double`
4. Apply style conversion first
5. Apply box wrapper second

---

## Parser Modifications

### Current Architecture

```rust
// src/parser.rs
fn process_templates(&self, text: &str) -> Result<String> {
    // Finds {{style}}...{{/style}}
    // Calls: converter.convert_with_spacing(content, style, spacing)
}
```

### Proposed Architecture

```rust
// src/parser.rs
enum TemplateType {
    Style { name: String, spacing: usize },
    Bracket { style: String },
    Box { style: String, params: BoxParams },
    Combined { style: String, modifier: Modifier },
}

fn parse_template_at(&self, chars: &[char], start: usize)
    -> Result<Option<(usize, TemplateType, String)>>
{
    // Parse template and determine type
    // Return type + content
}

fn process_template(&self, template: TemplateType, content: &str)
    -> Result<String>
{
    match template {
        TemplateType::Style { name, spacing } => {
            self.converter.convert_with_spacing(content, &name, spacing)
        }
        TemplateType::Bracket { style } => {
            self.frames.wrap_bracket(content, &style)
        }
        TemplateType::Box { style, params } => {
            self.frames.render_box(content, &style, params)
        }
        TemplateType::Combined { style, modifier } => {
            let styled = self.converter.convert(content, &style)?;
            self.apply_modifier(&styled, modifier)
        }
    }
}
```

### New Module: frames.rs

```rust
// src/frames.rs
pub struct FrameRenderer {
    bracket_styles: HashMap<String, BracketStyle>,
    box_styles: HashMap<String, BoxStyle>,
}

impl FrameRenderer {
    pub fn new() -> Self { ... }

    pub fn wrap_bracket(&self, text: &str, style: &str) -> Result<String> { ... }

    pub fn render_box(&self, text: &str, style: &str, params: BoxParams)
        -> Result<String> { ... }

    pub fn list_bracket_styles(&self) -> Vec<&str> { ... }

    pub fn list_box_styles(&self) -> Vec<&str> { ... }
}
```

---

## CLI Integration

### New Commands

```bash
# List available frame styles
utf8fx list-frames
utf8fx list-frames --category brackets
utf8fx list-frames --category boxes

# Convert with brackets
utf8fx convert --bracket corner "config.json"
# Output: 「config.json」

# Convert with box
utf8fx convert --box double --padding 2 "WARNING"
# Output:
# ╔═══════════╗
# ║  WARNING  ║
# ╚═══════════╝

# Combined style + frame
utf8fx convert --style mathbold --box heavy "TITLE"
# Output:
# ┏━━━━━━━━━┓
# ┃ 𝐓𝐈𝐓𝐋𝐄  ┃
# ┗━━━━━━━━━┛
```

### CLI Structure Changes

```rust
// src/bin/main.rs
#[derive(Subcommand)]
enum Commands {
    Convert {
        #[arg(short, long)]
        style: Option<String>,

        #[arg(short, long)]
        spacing: usize,

        #[arg(long)]
        bracket: Option<String>,

        #[arg(long)]
        r#box: Option<String>,

        #[arg(long)]
        padding: Option<usize>,

        text: String,
    },

    ListFrames {
        #[arg(short, long)]
        category: Option<String>,
    },

    // ... existing commands
}
```

---

## Data Structure

### frames.json

Similar to `styles.json`, create `data/frames.json`:

```json
{
  "version": "1.0.0",
  "brackets": {
    "square": {
      "id": "square",
      "name": "Square Brackets",
      "left": "[",
      "right": "]",
      "spacing": true
    },
    "corner": {
      "id": "corner",
      "name": "Corner Brackets",
      "left": "「",
      "right": "」",
      "spacing": false
    }
  },
  "boxes": {
    "light": {
      "id": "light",
      "name": "Light Box",
      "top_left": "┌",
      "top": "─",
      "top_right": "┐",
      "left": "│",
      "right": "│",
      "bottom_left": "└",
      "bottom": "─",
      "bottom_right": "┘"
    },
    "double": {
      "id": "double",
      "name": "Double Line Box",
      "top_left": "╔",
      "top": "═",
      "top_right": "╗",
      "left": "║",
      "right": "║",
      "bottom_left": "╚",
      "bottom": "═",
      "bottom_right": "╝"
    }
  }
}
```

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_bracket() {
        let frames = FrameRenderer::new();
        let result = frames.wrap_bracket("text", "square").unwrap();
        assert_eq!(result, "[ text ]");
    }

    #[test]
    fn test_light_box() {
        let frames = FrameRenderer::new();
        let params = BoxParams {
            padding: 1,
            width: None,
            align: TextAlign::Left,
        };
        let result = frames.render_box("test", "light", params).unwrap();
        let expected = "┌──────┐\n│ test │\n└──────┘";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_combined_style_and_box() {
        let parser = TemplateParser::new().unwrap();
        let input = "{{mathbold|box:double}}TEST{{/mathbold}}";
        let result = parser.process(input).unwrap();
        assert!(result.contains("╔"));
        assert!(result.contains("𝐓𝐄𝐒𝐓"));
    }
}
```

### Integration Tests

```rust
#[test]
fn test_bracket_template_in_markdown() {
    let parser = TemplateParser::new().unwrap();
    let input = "See {{bracket:corner}}config.json{{/bracket}} for settings.";
    let result = parser.process(input).unwrap();
    assert_eq!(result, "See 「config.json」 for settings.");
}

#[test]
fn test_box_with_spacing() {
    let parser = TemplateParser::new().unwrap();
    let input = "{{mathbold:spacing=1|box:heavy}}TITLE{{/mathbold}}";
    let result = parser.process(input).unwrap();
    // Verify box contains spaced bold text
}
```

---

## Performance Considerations

### Single-Line Operations

- **Brackets:** O(1) - Simple string concatenation
- **Boxes:** O(n) where n = text length
- **Impact:** Negligible for typical use cases

### Multi-Line Operations

- **Width calculation:** O(n*m) where n = lines, m = avg line length
- **Rendering:** O(n) for line count
- **Impact:** Could be significant for large blocks

### Optimization Strategies

1. **Lazy rendering** - Only calculate box size when needed
2. **Width caching** - Cache calculated widths
3. **Unicode width tables** - Pre-compute common character widths
4. **Streaming** - Process line-by-line for large content

---

## Documentation Requirements

### README Updates

Add section: "Frames & Boxes"

```markdown
### Frames & Boxes

#### Inline Brackets

Add brackets around text:
- `{{bracket:square}}text{{/bracket}}` → [ text ]
- `{{bracket:corner}}text{{/bracket}}` → 「text」

#### ASCII Boxes

Create framed boxes:
- `{{box:double}}WARNING{{/box}}` → ╔═══════╗ style box
- `{{box:heavy:padding=2}}TITLE{{/box}}` → Heavy box with padding

#### Combined Styles

Combine text styles with frames:
- `{{mathbold|box:rounded}}HEADER{{/mathbold}}`
```

### New Documentation File

Create `docs/FRAMES.md` with:
- Complete list of bracket styles
- Complete list of box styles
- Parameter reference
- Visual examples
- Use case gallery

---

## Open Questions

1. **Should boxes be single-line or multi-line by default?**
   - Single-line is simpler and covers 80% of use cases
   - Multi-line adds significant complexity

2. **How to handle very long text?**
   - Word wrapping?
   - Truncation?
   - Error?

3. **Should we support custom box styles?**
   - User-defined characters in config file?
   - Or just ship with predefined set?

4. **Terminal width awareness?**
   - Should boxes adapt to terminal width?
   - Or always use content width?

5. **Nested frames?**
   - `{{box:double}}{{bracket:square}}text{{/bracket}}{{/box}}`
   - Do we need to support this?

6. **Escape sequences?**
   - How to render literal `{{box}}` in output?
   - Current parser doesn't handle escaping

---

## Timeline Estimate

### Phase 1: Inline Brackets
- **Implementation:** 1-2 days
- **Testing:** 1 day
- **Documentation:** 0.5 days
- **Total:** 2-3 days

### Phase 2: Single-Line Boxes
- **Implementation:** 3-4 days
- **Testing:** 1-2 days
- **Documentation:** 1 day
- **Total:** 5-7 days

### Phase 3: Combined Styles
- **Implementation:** 2 days
- **Testing:** 1 day
- **Documentation:** 0.5 days
- **Total:** 3-4 days

### Phase 4: Multi-Line Boxes (Future)
- **Design:** 2-3 days
- **Implementation:** 5-7 days
- **Testing:** 2-3 days
- **Documentation:** 1-2 days
- **Total:** 10-15 days

---

## Success Criteria

### Phase 1 Complete When:
- [ ] Bracket templates work in markdown
- [ ] At least 6 bracket styles supported
- [ ] CLI `--bracket` flag functional
- [ ] Tests passing
- [ ] Documentation updated

### Phase 2 Complete When:
- [ ] Single-line boxes render correctly
- [ ] At least 4 box styles supported
- [ ] Padding parameter works
- [ ] CLI `--box` flag functional
- [ ] Tests passing
- [ ] Visual examples in docs

### Phase 3 Complete When:
- [ ] Can combine any style with any box
- [ ] Syntax: `{{style|box:type}}text{{/style}}`
- [ ] Tests for all combinations
- [ ] Performance acceptable (<10ms per box)

---

## References

### Unicode Resources

- Box Drawing: U+2500–U+257F
- Block Elements: U+2580–U+259F
- Geometric Shapes: U+25A0–U+25FF
- Miscellaneous Symbols: U+2600–U+26FF

### Similar Tools

- **boxes** (Unix tool) - ASCII art box generator
- **figlet** - ASCII art text generator
- **toilet** - FIGlet clone with Unicode support

### Prior Art

Study how these handle box rendering:
- `boxes` command line tool
- Rust `tui` crate box widgets
- Python `rich` library panels
