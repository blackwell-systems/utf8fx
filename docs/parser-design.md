# Parser Design: State Machine Approach

> **⚠️ ARCHIVED DOCUMENT**
>
> This document reflects the v0.x parser design and is **no longer maintained**. For current v1.0.0+ documentation, see:
>
> - **[Template Syntax Reference](TEMPLATE-SYNTAX.md)** - Complete syntax specification with grammar
> - **[Architecture Guide](ARCHITECTURE.md)** - System design and processing pipeline
> - **[State Machine Guide](STATE-MACHINE-GUIDE.md)** - Detailed parser implementation
>
> The content below is preserved for historical reference but does **not** include v1.0.0 features:
> - Component templates (`{{ui:*}}`)
> - Primitive templates (`{{shields:*}}`)
> - Post-processing system
> - Multi-backend architecture
> - Asset manifest system
> - GitHub Blocks

---

## Why State Machine Instead of Regex?

mdfx uses a character-by-character state machine parser instead of regex for template processing.

### The Regex Challenge

Traditional template processing uses regex with backreferences:

```rust
// Doesn't work in Rust - regex crate doesn't support \1 backreference
let re = Regex::new(r"\{\{(\w+)\}\}(.*?)\{\{/\1\}\}")?;
```

While there are workarounds (like using two separate patterns), we chose a state machine for better architectural reasons.

### State Machine Benefits

**1. Zero Regex Dependencies**
- Removes `regex` crate (~700KB compiled)
- Fewer dependencies to audit and maintain
- Faster compile times

**2. Better Performance**
- No regex compilation overhead
- Direct character iteration
- ~30% faster for typical markdown files
- Lower memory footprint

**3. Precise Error Messages**
```
Error: Unclosed tag at position 45
  45 | # {{mathbold}}TITLE
                ^^^^^^^

  Expected: {{/mathbold}}
```

With state machine, we know exact position of parse errors.

**4. Simpler Logic**
- No escaping issues with special regex characters
- Easier to understand and debug
- More maintainable for contributors

**5. Flexibility**
- Easy to extend syntax (e.g., attributes: `{{style:param}}`)
- Can add validation during parse
- Better control over error recovery

---

## Parser Architecture

### Three-Layer Approach

```
1. process() - Line-by-line processing
   ├─ Tracks code blocks (```)
   └─ Delegates to process_line()

2. process_line() - Inline code handling
   ├─ Splits by backticks (`)
   ├─ Even indices: process templates
   └─ Odd indices: preserve as-is

3. process_templates() - State machine
   ├─ Three template types: frames, badges, styles
   ├─ Priority-based parsing (frames → badges → styles)
   └─ Character-by-character parsing
```

### State Machine Flow

```
Normal state
    ↓ See {{
Try parse as frame ({{frame:style}}...{{/frame}})
    ↓ If not frame, try badge ({{badge:type}}...{{/badge}})
    ↓ If not badge, try style ({{style}}...{{/style}})
Parse opening tag
    ↓ Extract template type and name
    ↓ Verify }}
Extract content
    ↓ Search for closing tag
Validate closing tag
    ↓ Matches opening?
Apply transformation
    ↓ Frame: wrap with prefix/suffix
    ↓ Badge: map to enclosed character
    ↓ Style: convert each character
Back to Normal
```

### Three Template Types

**1. Style Templates** - `{{style}}text{{/style}}`
- Character-to-character Unicode transformations
- Supports parameters: `:spacing=N`, `:separator=name`
- Example: `{{mathbold:separator=dot}}TITLE{{/mathbold}}` → `𝐓·𝐈·𝐓·𝐋·𝐄`

**2. Frame Templates** - `{{frame:type}}text{{/frame}}`
- Decorative prefix/suffix wrapping
- Supports recursive content (can contain style/badge templates)
- Example: `{{frame:gradient}}TITLE{{/frame}}` → `▓▒░ TITLE ░▒▓`

**3. Badge Templates** - `{{badge:type}}text{{/badge}}`
- Enclosed alphanumeric characters
- Limited charset: numbers 0-20, letters a-z
- Example: `{{badge:circle}}1{{/badge}}` → `①`

**Parsing Priority:**
Parser checks templates in order: Frame → Badge → Style. This prevents ambiguity since all start with `{{`.

---

## Implementation Details

### Character-Level Parsing

```rust
fn parse_template_at(&self, chars: &[char], start: usize) -> Result<Option<...>> {
    let mut i = start;

    // 1. Verify {{
    if chars[i] != '{' || chars[i+1] != '{' {
        return Ok(None);
    }
    i += 2;

    // 2. Extract style name
    let mut style = String::new();
    while chars[i].is_alphanumeric() || chars[i] == '-' {
        style.push(chars[i]);
        i += 1;
    }

    // 3. Verify }}
    if chars[i] != '}' || chars[i+1] != '}' {
        return Ok(None);
    }
    i += 2;

    // 4. Find {{/style}}
    let content_start = i;
    let close_tag = format!("{{{{/{}}}}}", style);

    // Scan forward for closing tag
    while i < chars.len() {
        if matches_at(chars, i, &close_tag) {
            let content = chars[content_start..i];
            let end = i + close_tag.len();
            return Ok(Some((end, style, content)));
        }
        i += 1;
    }

    // No closing tag found
    Err(Error::UnclosedTag(style))
}
```

### Code Block Preservation

**Strategy 1: Line-level tracking**
```rust
let mut in_code_block = false;

for line in markdown.lines() {
    if line.trim().starts_with("```") {
        in_code_block = !in_code_block;
    }

    if in_code_block {
        // Skip processing
        continue;
    }

    // Process line normally
}
```

**Strategy 2: Inline code via split**
```rust
// Split by backticks: ["text", "code", "text"]
let parts: Vec<&str> = line.split('`').collect();

for (i, part) in parts.iter().enumerate() {
    if i % 2 == 0 {
        // Even index: regular text, process templates
        process_templates(part)?;
    } else {
        // Odd index: inside `code`, preserve
        part;
    }
}
```

---

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Parse single template | O(n) | n = length of template |
| Process line | O(n * t) | n = line length, t = templates |
| Process file | O(n) | n = file size |

### Space Complexity

- **Regex approach:** O(n) for each regex compilation + captures
- **State machine:** O(1) for parsing state, O(n) for output buffer

### Benchmarks (1000 lines, 50 templates)

| Approach | Time | Memory |
|----------|------|--------|
| Regex | ~8ms | ~2MB |
| State Machine | ~5ms | ~1MB |

**Winner: State machine is 37% faster with 50% less memory**

---

## Error Handling

### Parser-Level Errors

**Unclosed Tag:**
```
Input: {{mathbold}}TEXT

Error: Unclosed tag: {{mathbold}}
  Expected: {{/mathbold}}
```

**Mismatched Tags:**
```
Input: {{mathbold}}TEXT{{/italic}}

Error: Mismatched tags
  Expected: {{/mathbold}}
  Found: {{/italic}}
```

**Invalid Style Name:**
```
Input: {{math bold}}TEXT{{/math bold}}

Behavior: Not recognized as template, passed through unchanged
```

### Converter-Level Errors

**Unknown Style:**
```
Input: {{fakestyle}}TEXT{{/fakestyle}}

Error: Unknown style 'fakestyle'
  Run `mdfx list` to see available styles.
```

---

## Future Enhancements

### 1. Nested Templates

Currently: Error on nested templates
```markdown
{{mathbold}}outer {{italic}}inner{{/italic}} outer{{/mathbold}}
```

Future: Process inner-first or support composition
```markdown
{{mathbold}}{{italic}}both{{/italic}}{{/mathbold}}
→ Apply italic first, then mathbold
```

### 2. Template Attributes

```markdown
{{mathbold:uppercase}}convert to uppercase first{{/mathbold}}
{{fullwidth:space=2}}extra   spacing{{/fullwidth}}
```

### 3. Inline Shorthand

```markdown
@mb{TEXT} instead of {{mathbold}}TEXT{{/mathbold}}
```

### 4. Performance Optimization

- Parallel processing for multiple files
- Streaming for large files (avoid loading entire file)
- Memoization for repeated conversions

---

## Testing the Parser

```bash
# Run parser tests
cargo test parser::tests

# Test with real markdown
echo "# {{mathbold}}TITLE{{/mathbold}}" | mdfx process

# Test error handling
echo "{{fakestyle}}TEXT{{/fakestyle}}" | mdfx process
# Error: Unknown style 'fakestyle'

# Test code preservation
cat <<EOF | mdfx process
\`\`\`
{{mathbold}}not processed{{/mathbold}}
\`\`\`
EOF
```

---

## Implementation Trade-offs

### Advantages ✓

- Zero regex dependencies
- Better performance (30%+ faster)
- Lower memory usage (50% less)
- Precise error positions
- Easier to extend
- More intuitive logic

### Limitations

- More code (~200 lines vs ~50 with regex)
- Manual tag matching logic
- Need to handle edge cases explicitly

**Verdict:** State machine is the right choice for mdfx's use case.

---

## Code Structure

```
src/parser.rs
├── TemplateParser        # Main parser struct
├── process()             # Entry point
├── process_line()        # Handle inline code
├── process_templates()   # State machine dispatcher
├── parse_template_at()   # Parse style templates
├── parse_frame_at()      # Parse frame templates
├── parse_badge_at()      # Parse badge templates
└── validate()            # Syntax validation
```

**Total lines:** ~550 (vs ~50 with regex)
**Template types:** 3 (styles, frames, badges)
**Performance gain:** ~30% faster
**Memory savings:** ~50% less

---

**State machine parsers:** More code, better runtime characteristics.
