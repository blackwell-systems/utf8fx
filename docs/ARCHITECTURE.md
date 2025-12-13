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

utf8fx is a markdown preprocessor that transforms text using Unicode character mappings. The system consists of three primary components working together in a pipeline architecture.

```mermaid
%%{init: {'theme':'dark'}}%%
graph LR
    A[Input Text] --> B[Template Parser]
    B --> C[Converter]
    C --> D[Styled Output]

    E[styles.json] -.->|Character Mappings| C

    style A fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style B fill:#2d3748,stroke:#48bb78,stroke-width:2px
    style C fill:#2d3748,stroke:#ed8936,stroke-width:2px
    style D fill:#2d3748,stroke:#4299e1,stroke-width:2px
    style E fill:#2d3748,stroke:#9f7aea,stroke-width:2px,stroke-dasharray: 5 5
```

### Core Principles

1. **Single Responsibility** - Each component has one clear purpose
2. **Zero-Copy Operations** - Minimize memory allocations
3. **Data-Driven** - Configuration over code (styles.json)
4. **Fail-Safe** - Preserve original text on errors
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

**Purpose:** Character-to-character Unicode mapping

**Key Functions:**
- `convert(text, style)` - Transform text using a style
- `convert_with_spacing(text, style, spacing)` - Add character spacing
- `list_styles()` - Query available styles
- `has_style(name)` - Check style existence

**Design:**
- Loads `styles.json` once at initialization (lazy_static)
- O(1) style lookup via HashMap
- Preserves whitespace, punctuation, unsupported characters
- Zero allocations for unsupported characters

#### 2. TemplateParser (`src/parser.rs`)

**Purpose:** Process markdown with `{{style}}text{{/style}}` templates

**Key Functions:**
- `process(content)` - Parse and transform entire document
- `parse_template_at(chars, pos)` - State machine for single template

**Design:**
- State machine parser (no regex dependencies)
- Preserves code blocks (```) and inline code (`)
- Tracks nesting depth for proper template matching
- Fail-safe: invalid templates preserved as-is

#### 3. Styles Manager (`src/styles.rs`)

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

    Normal --> OpenBrace1: char == '{'
    OpenBrace1 --> OpenBrace2: char == '{'
    OpenBrace2 --> StyleName: alphanumeric

    StyleName --> StyleName: alphanumeric or '-'
    StyleName --> ParamColon: char == ':'
    StyleName --> CloseTag1: char == '}'

    ParamColon --> ParamKey: char == 's' (spacing)
    ParamKey --> ParamEquals: char == '='
    ParamEquals --> ParamValue: digit
    ParamValue --> ParamValue: digit
    ParamValue --> CloseTag1: char == '}'

    CloseTag1 --> CloseTag2: char == '}'
    CloseTag2 --> Content: any char

    Content --> Content: any char
    Content --> EndTag1: char == '{'

    EndTag1 --> EndTag2: char == '{'
    EndTag2 --> EndSlash: char == '/'
    EndSlash --> EndStyleName: alphanumeric
    EndStyleName --> EndStyleName: alphanumeric or '-'
    EndStyleName --> EndClose1: char == '}'
    EndClose1 --> EndClose2: char == '}'
    EndClose2 --> [*]: Match!

    Normal --> Normal: any other char
    OpenBrace1 --> Normal: char != '{'
    OpenBrace2 --> Normal: not alphanumeric

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

    TextMode --> FencedCodeCheck: line starts with ```
    FencedCodeCheck --> FencedCode: count == 3
    FencedCode --> FencedCode: any line
    FencedCode --> FencedCodeCheck2: line starts with ```
    FencedCodeCheck2 --> TextMode: count == 3

    TextMode --> InlineCodeCheck: char == '`'
    InlineCodeCheck --> InlineCode: single backtick
    InlineCode --> InlineCode: any char
    InlineCode --> TextMode: char == '`'

    TextMode --> TemplateProcessing: char == '{'
    TemplateProcessing --> TextMode: template complete

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

### 3. Zero-Copy for Unsupported Characters

**Decision:** Return original character if no mapping exists

**Rationale:**
- **Predictable behavior**: Users see what they expect
- **Punctuation preserved**: "Hello!" → "𝐇𝐞𝐥𝐥𝐨!"
- **Emoji safe**: "Test 🎉" → "𝐓𝐞𝐬𝐭 🎉"

**Trade-off:** None - strictly better UX

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

### Adding Frame Support (Planned)

See [FRAMES-DESIGN.md](FRAMES-DESIGN.md) for detailed architecture.

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
