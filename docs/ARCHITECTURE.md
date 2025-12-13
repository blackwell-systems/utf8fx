# utf8fx Architecture

**Version:** 1.0.0
**Last Updated:** 2025-12-12

## Table of Contents

- [System Overview](#system-overview)
- [Component Architecture](#component-architecture)
- [State Machine Design](#state-machine-design)
- [Data Flow](#data-flow)
- [Key Design Decisions](#key-design-decisions)
- [Extension Points](#extension-points)

---

## System Overview

utf8fx is a markdown preprocessor that transforms text using Unicode character mappings. The system consists of four primary components working together in a pipeline architecture.

```mermaid
%%{init: {'theme':'dark'}}%%
graph LR
    A[Input Text] --> B[Template Parser]
    B --> C[Converter]
    B --> H[Badge Renderer]
    B --> F[Frame Renderer]
    C --> F
    H --> F
    F --> D[Styled Output]

    E[styles.json] -.->|Character Mappings| C
    G[frames.json] -.->|Decorative Elements| F
    I[badges.json] -.->|Enclosed Characters| H

    style A fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style B fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style C fill:#2d3748,stroke:#ed8936,stroke-width:2px
    style H fill:#2d3748,stroke:#f56565,stroke-width:2px
    style F fill:#2d3748,stroke:#9f7aea,stroke-width:2px
    style D fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style E fill:#2d3748,stroke:#9f7aea,stroke-width:2px,stroke-dasharray: 5 5
    style G fill:#2d3748,stroke:#9f7aea,stroke-width:2px,stroke-dasharray: 5 5
    style I fill:#2d3748,stroke:#9f7aea,stroke-width:2px,stroke-dasharray: 5 5
```

### Core Principles

1. **Single Responsibility** - Each component has one clear purpose
2. **Allocation-Minimized** - Single-pass processing, no regex backtracking
3. **Data-Driven** - Configuration over code (styles.json)
4. **Strict by Default** - Returns errors for invalid templates (use `--help` for CLI behavior)
5. **Composable** - Features work together cleanly

---

## Component Architecture

```mermaid
%%{init: {'theme':'dark'}}%%
graph TD
    subgraph "Public API Layer"
        A[Converter API]
        B[TemplateParser API]
    end

    subgraph "Core Logic Layer"
        C[Converter]
        D[Parser State Machine]
        E[Styles Manager]
    end

    subgraph "Data Layer"
        F[styles.json]
        G[Style Definitions]
        H[Character Mappings]
    end

    A --> C
    B --> D
    C --> E
    D --> C
    E --> F
    F --> G
    F --> H

    style A fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style B fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style C fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style D fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style E fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style F fill:#2d3748,stroke:#9f7aea,stroke-width:2px
    style G fill:#2d3748,stroke:#9f7aea,stroke-width:2px
    style H fill:#2d3748,stroke:#9f7aea,stroke-width:2px
```

### Component Responsibilities

#### 1. Converter (`src/converter.rs`)

**Purpose:** Character-to-character Unicode mapping with optional separation

**Key Functions:**
- `convert(text, style)` - Transform text using a style
- `convert_with_spacing(text, style, spacing)` - Add space-based character spacing
- `convert_with_separator(text, style, separator, count)` - Add custom separator characters
- `list_styles()` - Query available styles
- `has_style(name)` - Check style existence

**Design:**
- **Unified algorithm**: Internal `convert_with_char_between()` method handles all cases
- Public methods delegate to unified implementation (eliminates duplication)
- Fast path optimization: When count=0, skip separation logic entirely
- Loads `styles.json` once at initialization (lazy_static)
- O(1) style lookup via HashMap
- Preserves whitespace, punctuation, unsupported characters
- Allocates output String for converted text (standard Rust string transformation)

#### 2. TemplateParser (`src/parser.rs`)

**Purpose:** Process markdown with `{{style}}text{{/style}}`, `{{frame:style}}text{{/frame}}`, and `{{badge:type}}text{{/badge}}` templates

**Key Functions:**
- `process(content)` - Parse and transform entire document
- `parse_template_at(chars, pos)` - State machine for style templates
- `parse_frame_at(chars, pos)` - State machine for frame templates
- `parse_badge_at(chars, pos)` - State machine for badge templates

**Design:**
- State machine parser (no regex dependencies)
- **Three template types**: style templates, frame templates, and badge templates
- **Parsing priority**: Frames → Badges → Styles (prevents ambiguity)
- **Recursive processing**: Frame content can contain style/badge templates
- **Parameter parsing**: Supports `:spacing=N` and `:separator=name` parameters (styles only)
- Preserves code blocks (```) and inline code (`)
- **Error handling**: Returns `Error` for unknown styles/frames/badges or unclosed tags
- **Limitation**: Nested templates of the same type not supported (e.g., `{{mb}}{{mb}}X{{/mb}}{{/mb}}`)

#### 3. FrameRenderer (`src/frames.rs`)

**Purpose:** Add decorative prefix/suffix around text (taglines, accents)

**Key Functions:**
- `apply_frame(text, frame_style)` - Wrap text with decorative elements
- `get_frame(name)` - Lookup frame by ID or alias
- `has_frame(name)` - Check frame existence
- `list_frames()` - Query available frames

**Design:**
- Loads `frames.json` once at initialization
- Simple concatenation: `prefix + text + suffix`
- Supports 27 frame styles (gradient, solid, lines, arrows, stars, brackets, etc.)
- Alias support for shorter names (grad = gradient)
- Works with styled content (recursive processing by parser)

#### 4. BadgeRenderer (`src/badges.rs`)

**Purpose:** Enclose numbers and letters with pre-composed Unicode characters

**Key Functions:**
- `apply_badge(text, badge_type)` - Enclose text in badge character
- `get_badge(name)` - Lookup badge by ID or alias
- `has_badge(name)` - Check badge existence
- `list_badges()` - Query available badges

**Design:**
- Loads `badges.json` once at initialization
- Direct character mapping: `text -> badge_character`
- Limited charset support: numbers (0-20), lowercase letters (a-z)
- 6 badge types: circle (①), negative-circle (❶), double-circle (⓵), paren (⑴), period (🄁), paren-letter (⒜)
- Returns `UnsupportedChar` error for characters outside badge's charset
- No recursive processing (badges can't contain other templates)

#### 5. Styles Manager (`src/styles.rs`)

**Purpose:** Load and manage style definitions

**Key Functions:**
- `load_styles()` - Parse styles.json
- `find_style_by_id(id)` - Lookup by primary ID
- `find_style_by_alias(alias)` - Lookup by alias
- `convert_char(ch, mappings)` - Apply character mapping

**Design:**
- Serde-based JSON deserialization
- Category-based organization (Bold, Boxed, Technical, Elegant)
- Alias support for shorter names (mb = mathbold)

---

## State Machine Design

The template parser uses a state machine to process markdown without regex. This enables:
- Precise error messages ("expected closing tag at line X")
- Code block preservation (no accidental transformations)
- O(n) linear time complexity
- No catastrophic backtracking

### Parser State Machine

```mermaid
%%{init: {'theme':'dark'}}%%
stateDiagram-v2
    [*] --> Normal

    Normal --> OpenBrace1: see opening brace
    OpenBrace1 --> OpenBrace2: see second brace
    OpenBrace2 --> StyleName: letter found

    StyleName --> StyleName: letter or hyphen
    StyleName --> ParamColon: colon found
    StyleName --> CloseTag1: closing brace

    ParamColon --> ParamKey: spacing keyword
    ParamKey --> ParamEquals: equals sign
    ParamEquals --> ParamValue: digit found
    ParamValue --> ParamValue: more digits
    ParamValue --> CloseTag1: closing brace

    CloseTag1 --> CloseTag2: second closing brace
    CloseTag2 --> Content: reading content

    Content --> Content: any character
    Content --> EndTag1: opening brace

    EndTag1 --> EndTag2: second opening brace
    EndTag2 --> EndSlash: forward slash
    EndSlash --> EndStyleName: letter found
    EndStyleName --> EndStyleName: letter or hyphen
    EndStyleName --> EndClose1: closing brace
    EndClose1 --> EndClose2: second closing brace
    EndClose2 --> [*]: template complete

    Normal --> Normal: other characters
    OpenBrace1 --> Normal: not second brace
    OpenBrace2 --> Normal: not letter

    style Normal fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style CloseTag2 fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style EndClose2 fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style ParamValue fill:#2d3748,stroke:#ed8936,stroke-width:2px
```

### State Transitions

| State | Input | Next State | Action |
|-------|-------|------------|--------|
| Normal | `{` | OpenBrace1 | Start potential template |
| OpenBrace1 | `{` | OpenBrace2 | Confirm opening |
| OpenBrace2 | alphanumeric | StyleName | Begin style name |
| StyleName | `-` or alphanumeric | StyleName | Accumulate name |
| StyleName | `:` | ParamColon | Start parameter |
| StyleName | `}` | CloseTag1 | End style name |
| ParamColon | `s` | ParamKey | Expect "spacing" |
| ParamValue | digit | ParamValue | Accumulate number |
| ParamValue | `}` | CloseTag1 | End parameter |
| CloseTag1 | `}` | CloseTag2 | Opening tag complete |
| Content | any | Content | Accumulate content |
| Content | `{` | EndTag1 | Start closing tag |
| EndTag1 | `{` | EndTag2 | Confirm closing |
| EndTag2 | `/` | EndSlash | Expect closing tag |
| EndStyleName | match | EndClose1 | Verify tag matches |
| EndClose2 | `}` | Match | Template complete |

### Code Block Handling

The parser tracks markdown context to preserve code blocks:

```mermaid
%%{init: {'theme':'dark'}}%%
stateDiagram-v2
    [*] --> TextMode

    TextMode --> FencedCodeCheck: three backticks
    FencedCodeCheck --> FencedCode: code fence start
    FencedCode --> FencedCode: read lines
    FencedCode --> FencedCodeCheck2: three backticks
    FencedCodeCheck2 --> TextMode: code fence end

    TextMode --> InlineCodeCheck: single backtick
    InlineCodeCheck --> InlineCode: inline code start
    InlineCode --> InlineCode: read chars
    InlineCode --> TextMode: closing backtick

    TextMode --> TemplateProcessing: opening brace
    TemplateProcessing --> TextMode: template done

    style TextMode fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style FencedCode fill:#2d3748,stroke:#ed8936,stroke-width:2px
    style InlineCode fill:#2d3748,stroke:#ed8936,stroke-width:2px
    style TemplateProcessing fill:#2d3748,stroke:#48bb78,stroke-width:2px
```

**Key Feature:** Templates inside code blocks are preserved as-is:

```rust
// This will NOT be transformed:
// `{{mathbold}}CODE{{/mathbold}}`
//
// ```
// let x = {{script}}test{{/script}};
// ```
```

---

## Data Flow

### Direct Conversion Flow

```mermaid
%%{init: {'theme':'dark'}}%%
sequenceDiagram
    participant User
    participant Converter
    participant StyleManager
    participant StylesJSON

    User->>Converter: convert("Hello", "mathbold")
    Converter->>StyleManager: find_style("mathbold")
    StyleManager->>StylesJSON: load mappings
    StylesJSON-->>StyleManager: character map
    StyleManager-->>Converter: Style{mappings}

    loop For each character
        Converter->>StyleManager: convert_char('H', mappings)
        StyleManager-->>Converter: '𝐇'
    end

    Converter-->>User: "𝐇𝐞𝐥𝐥𝐨"

    note over Converter,StyleManager: O(n) time, n = text length
```

### Template Processing Flow

```mermaid
%%{init: {'theme':'dark'}}%%
sequenceDiagram
    participant User
    participant Parser
    participant StateMachine
    participant Converter

    User->>Parser: process("{{mathbold}}Hi{{/mathbold}}")
    Parser->>StateMachine: parse_template_at(0)

    Note over StateMachine: State: Normal → OpenBrace1 → OpenBrace2
    Note over StateMachine: State: StyleName → CloseTag1 → CloseTag2

    StateMachine->>Parser: Template found: style="mathbold", content="Hi"
    Parser->>Converter: convert_with_spacing("Hi", "mathbold", 0)
    Converter-->>Parser: "𝐇𝐢"
    Parser-->>User: "𝐇𝐢"

    note over Parser,StateMachine: O(n) time, n = document length
```

### Error Handling Flow

```mermaid
%%{init: {'theme':'dark'}}%%
graph TD
    A[Template Found] --> B{Valid Style?}
    B -->|Yes| C[Convert Content]
    B -->|No| D[Error: Unknown Style]

    C --> E{Closing Tag Matches?}
    E -->|Yes| F[Return Styled Text]
    E -->|No| G[Error: Mismatched Tags]

    D --> H[Preserve Original]
    G --> H

    style A fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style B fill:#2d3748,stroke:#ed8936,stroke-width:2px
    style C fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style E fill:#2d3748,stroke:#ed8936,stroke-width:2px
    style F fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style D fill:#2d3748,stroke:#e53e3e,stroke-width:2px
    style G fill:#2d3748,stroke:#e53e3e,stroke-width:2px
    style H fill:#2d3748,stroke:#9f7aea,stroke-width:2px
```

### Composition Flow

Example: `{{frame:gradient}}{{mathbold:separator=dash}}TITLE{{/mathbold}}{{/frame}}`

```mermaid
%%{init: {'theme':'dark'}}%%
sequenceDiagram
    participant User
    participant Parser
    participant Converter
    participant FrameRenderer

    User->>Parser: process(template)
    Parser->>Parser: parse_frame_at()
    Note over Parser: Found: frame="gradient"<br/>content="{{mathbold:separator=dash}}TITLE{{/mathbold}}"

    Parser->>Parser: process_templates(content) [RECURSIVE]
    Parser->>Parser: parse_template_at()
    Note over Parser: Found: style="mathbold"<br/>separator="─"<br/>content="TITLE"

    Parser->>Converter: convert_with_separator("TITLE", "mathbold", "─", 1)
    Converter-->>Parser: "𝐓─𝐈─𝐓─𝐋─𝐄"

    Parser->>FrameRenderer: apply_frame("𝐓─𝐈─𝐓─𝐋─𝐄", "gradient")
    FrameRenderer-->>Parser: "▓▒░ 𝐓─𝐈─𝐓─𝐋─𝐄 ░▒▓"

    Parser-->>User: "▓▒░ 𝐓─𝐈─𝐓─𝐋─𝐄 ░▒▓"

    note over Parser: Recursive processing<br/>enables composition
```

---

## Key Design Decisions

### 1. State Machine over Regex

**Decision:** Implement custom state machine parser instead of regex

**Rationale:**
- **Precise error messages**: "Unclosed tag: {{mathbold}} at line 42"
- **No catastrophic backtracking**: O(n) guaranteed performance
- **Context awareness**: Track code blocks, inline code, nesting
- **Zero dependencies**: No regex crate needed

**Trade-off:** More code, but better UX and performance

### 2. Lazy Static Styles Loading

**Decision:** Load styles.json once at first use via lazy_static

**Rationale:**
- **Fast startup**: Don't load unless needed
- **Memory efficiency**: Single shared instance
- **Thread-safe**: lazy_static handles concurrency

```rust
lazy_static! {
    static ref STYLES: StylesData = load_styles().unwrap();
}
```

**Trade-off:** Panic if styles.json invalid (acceptable for v1.0.0)

### 3. Preserve Unsupported Characters

**Decision:** Return original character if no mapping exists

**Rationale:**
- **Predictable behavior**: Users see what they expect
- **Punctuation preserved**: "Hello!" → "𝐇𝐞𝐥𝐥𝐨!"
- **Emoji safe**: "Test 🎉" → "𝐓𝐞𝐬𝐭 🎉"
- **Implementation**: Characters without mappings pass through unchanged

**Note:** While individual characters may be preserved, the conversion process allocates new Strings for the output (standard for Rust string transformations).

### 4. Template Preservation in Code Blocks

**Decision:** Don't process templates inside ``` or ` markers

**Rationale:**
- **Markdown semantics**: Code blocks are literal
- **Documentation safe**: Can show template syntax in docs
- **No escape sequences needed**: Just use code blocks

**Implementation:** Track `in_code_block` and `in_inline_code` flags

### 5. Data-Driven Character Mappings

**Decision:** Store all character mappings in styles.json

**Rationale:**
- **Easy to extend**: Add styles without code changes
- **Auditable**: See all mappings in one file
- **Version controlled**: Track changes to character sets

**Format:**
```json
{
  "id": "mathbold",
  "uppercase": {
    "A": "𝐀",
    "B": "𝐁"
  },
  "lowercase": {
    "a": "𝐚",
    "b": "𝐛"
  }
}
```

---

---

## Data Packaging

### Embedded JSON Files

All character mappings and configurations are **embedded at compile time** using `include_str!()`:

```rust
// src/styles.rs
const STYLES_JSON: &str = include_str!("../data/styles.json");

// src/frames.rs
let data = include_str!("../data/frames.json");

// src/badges.rs
let data = include_str!("../data/badges.json");
```

**This means:**
- No runtime file I/O - data is baked into the binary
- No deployment concerns - everything is self-contained
- Works in any environment (containers, embedded systems, WASM)
- No file path configuration needed
- Data versioned with code in git

**Library users:** Just `use utf8fx::Converter` - data is already embedded
**CLI users:** Single binary, no external files required

### Why Embedded?

1. **Simplicity**: No "where are my data files?" support issues
2. **Performance**: Data parsed once at compile time validation, loaded instantly
3. **Portability**: Binary works anywhere without file dependencies
4. **Security**: No file system access, no path traversal concerns
5. **Versioning**: Data and code versioned together

### Trade-offs

- **Pro**: Zero deployment complexity, guaranteed data availability
- **Con**: Cannot modify mappings without recompiling (intentional - ensures consistency)
- **Con**: Binary size includes ~50KB of JSON data (negligible for most uses)

**For custom mappings:** Edit `data/*.json` and recompile. See "Extension Points" below.

---

## Extension Points

### Adding New Styles

1. **Add to styles.json:**
```json
{
  "id": "new-style",
  "name": "New Style",
  "category": "Technical",
  "description": "Description here",
  "aliases": ["ns", "new"],
  "uppercase": { ... },
  "lowercase": { ... },
  "digits": { ... }
}
```

2. **No code changes needed** - system automatically picks it up

### Adding New Parameters

Current: `{{style:spacing=N}}`

To add new parameters (e.g., `color`, `variant`):

1. **Update parser state machine** (src/parser.rs:~170)
2. **Extend parameter parsing** to handle new syntax
3. **Pass parameters to Converter**
4. **Implement parameter logic**

Example future syntax:
```markdown
{{mathbold:spacing=2:case=upper}}Text{{/mathbold}}
```

### Adding New Frames

Frames are already implemented and extensible:

1. **Add to frames.json:**
```json
{
  "id": "my-frame",
  "name": "My Frame",
  "description": "Custom decorative frame",
  "prefix": "« ",
  "suffix": " »",
  "aliases": ["mf"]
}
```

2. **Use immediately:**
```rust
let result = renderer.apply_frame("text", "my-frame")?;
// Output: « text »
```

See [FRAMES-DESIGN.md](FRAMES-DESIGN.md) for frame design patterns and examples.

**Integration point:** New `FrameRenderer` component

```mermaid
%%{init: {'theme':'dark'}}%%
graph LR
    A[Parser] --> B[Converter]
    B --> C[FrameRenderer]
    C --> D[Output]

    E[frames.json] -.->|Box Chars| C

    style A fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style B fill:#2d3748,stroke:#ed8936,stroke-width:2px
    style C fill:#2d3748,stroke:#9f7aea,stroke-width:2px
    style D fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style E fill:#2d3748,stroke:#9f7aea,stroke-width:2px,stroke-dasharray: 5 5
```

**Template syntax:**
```markdown
{{box:double}}Framed Text{{/box}}
{{mathbold|box:single}}Bold + Frame{{/mathbold}}
```

---

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Style lookup | O(1) | HashMap-based |
| Character conversion | O(n) | n = text length |
| Template parsing | O(n) | n = document length |
| Code block detection | O(n) | Single pass |

### Space Complexity

| Component | Space | Notes |
|-----------|-------|-------|
| styles.json | ~90KB | Loaded once, shared |
| Style HashMap | O(s) | s = number of styles (19) |
| Parse buffer | O(n) | n = document length |
| Output buffer | O(n) | n = output length |

### Optimization Strategies

1. **Lazy loading**: Don't load styles until first use
2. **String interning**: Reuse style name strings
3. **Capacity hints**: Pre-allocate output buffers
4. **Zero-copy paths**: Return original on no-op

---

## Testing Strategy

```mermaid
%%{init: {'theme':'dark'}}%%
graph TD
    A[Unit Tests] --> B[Converter Tests]
    A --> C[Parser Tests]
    A --> D[Styles Tests]

    E[Integration Tests] --> F[End-to-End Conversion]
    E --> G[Template Processing]
    E --> H[CLI Commands]

    I[Doc Tests] --> J[README Examples]
    I --> K[API Examples]

    style A fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style E fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style I fill:#2d3748,stroke:#ed8936,stroke-width:2px
```

**Test Coverage:**
- **49 unit tests**: Component-level functionality
- **4 doc tests**: Documentation examples
- **100% of public API**: Every public function tested

---

## Future Architecture Considerations

### 1. Plugin System

**Goal:** Allow users to add custom styles without forking

**Design:**
```rust
pub trait StyleProvider {
    fn list_styles(&self) -> Vec<StyleInfo>;
    fn convert(&self, text: &str, style: &str) -> Result<String>;
}

impl Converter {
    pub fn register_provider(&mut self, provider: Box<dyn StyleProvider>);
}
```

### 2. WASM Support

**Goal:** Run utf8fx in browsers

**Requirements:**
- No filesystem access (inline styles.json)
- No color output (browser handles styling)
- Smaller binary size (<100KB compressed)

### 3. Streaming API

**Goal:** Process large documents without loading entire file

**Design:**
```rust
pub struct StreamingParser {
    pub fn process_chunk(&mut self, chunk: &str) -> Result<String>;
    pub fn finalize(&mut self) -> Result<String>;
}
```

---

## References

- [Parser Design](parser-design.md) - Detailed parser implementation
- [Frames Design](FRAMES-DESIGN.md) - Future frame feature architecture
- [Planning Document](PLANNING.md) - Product roadmap and milestones
- [Unicode Reference](unicode-design-elements.md) - Character set details

---

**Document Version:** 1.0.0
**Last Updated:** 2025-12-12
**Maintained By:** Dayna Blackwell <blackwellsystems@protonmail.com>
