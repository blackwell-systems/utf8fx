# Evaluation Contexts

**The Key to Safe Composition**

Version: 2.0 (Design)  
Status: **Specification**  
Last Updated: 2025-12-13

---

## The Problem

Without evaluation contexts, users can inject inappropriate renderables in the wrong places:

```markdown
<!-- This SHOULD error but doesn't in v1.0 -->
{{mathbold:separator=sep.divider}}TEXT{{/mathbold}}

<!-- Output: T[giant divider block]E[giant divider block]X[giant divider block]T -->
<!-- Completely breaks the sentence! -->
```

**The footgun**: A multiline divider block used as an inline separator between characters.

**Why it happens**: v1.0 has no concept of "this renderable is for blocks, not inline."

---

## The Solution: Evaluation Contexts

Every **renderable** has a set of allowed **contexts**.  
Every **expansion site** has a required **context**.  
The compiler **validates compatibility** at expansion time.

### The Three Contexts

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalContext {
    /// Inline: Between characters in styled text
    /// Constraints: Compact, no newlines
    Inline {
        max_graphemes: usize,  // Default: 5
    },
    
    /// Block: Section-level, multiline allowed
    /// Constraints: None
    Block,
    
    /// FrameChrome: Frame prefix/suffix decorations
    /// Constraints: Single line, limited length
    FrameChrome {
        max_length: usize,  // Default: 100 chars
    },
}
```

---

## Context Semantics

### Inline Context

**Where it applies**:
- `separator=` parameter in style templates
- Inline composition (future: `{{inline}}...{{/inline}}`)

**Constraints**:
```rust
fn validate_inline(content: &str, max_graphemes: usize) -> Result<()> {
    // 1. No newlines
    if content.contains('\n') {
        return Err(Error::NewlineInInline);
    }
    
    // 2. Max grapheme count
    let graphemes: Vec<&str> = content.graphemes(true).collect();
    if graphemes.len() > max_graphemes {
        return Err(Error::TooManyGraphemes {
            found: graphemes.len(),
            max: max_graphemes,
        });
    }
    
    Ok(())
}
```

**Examples**:

```markdown
✓ {{mathbold:separator=dot}}TEXT{{/mathbold}}
  → 𝐓·𝐈·𝐓·𝐋·𝐄

✓ {{mathbold:separator=→}}TEXT{{/mathbold}}
  → 𝐓→𝐈→𝐓→𝐋→𝐄

✓ {{mathbold:separator=sep.accent}}TEXT{{/mathbold}}
  → 𝐓🟣𝐈🟣𝐓🟣𝐋🟣𝐄  (swatch is inline-compatible)

✗ {{mathbold:separator=sep.divider}}TEXT{{/mathbold}}
  ERROR: Block renderable 'sep.divider' in Inline context
  
✗ {{mathbold:separator=
}}TEXT{{/mathbold}}
  ERROR: Literal separator contains newline
```

### Block Context

**Where it applies**:
- Top-level markdown content
- Block joiners (future: `{{ui:section_join:separator=X}}`)
- List item joiners

**Constraints**:
```rust
fn validate_block(content: &str) -> Result<()> {
    // No constraints - anything goes
    Ok(())
}
```

**Examples**:

```markdown
✓ {{ui:divider/}}
  → [Full-width gradient bar]

✓ {{ui:callout:warning}}
  Multi-line
  Content here
  {{/ui}}

✓ {{ui:section_join:separator=sep.divider}}
  Item 1
  Item 2
  {{/ui}}
  → Item 1
    [divider]
    Item 2
```

### FrameChrome Context

**Where it applies**:
- Frame `prefix` field
- Frame `suffix` field

**Constraints**:
```rust
fn validate_frame_chrome(content: &str, max_length: usize) -> Result<()> {
    // 1. No newlines (frames must be inline decorations)
    if content.contains('\n') {
        return Err(Error::NewlineInFrameChrome);
    }
    
    // 2. Max length (prevent absurdly long prefixes)
    if content.chars().count() > max_length {
        return Err(Error::ChromeTooLong {
            found: content.chars().count(),
            max: max_length,
        });
    }
    
    Ok(())
}
```

**Examples**:

```json
// ✓ Valid: Compact inline snippet
{
  "frames": {
    "alert": {
      "prefix": "{{snippet:chrome.warning}}",
      "suffix": ""
    }
  },
  "snippets": {
    "chrome.warning": {
      "template": "⚠️ ",
      "contexts": ["frame_chrome"]
    }
  }
}

// ✓ Valid: Inline swatch as prefix
{
  "frames": {
    "color_accent": {
      "prefix": "{{snippet:chrome.accent_block}}",
      "suffix": ""
    }
  },
  "snippets": {
    "chrome.accent_block": {
      "template": "{{ui:swatch:accent/}} ",
      "contexts": ["frame_chrome"]
    }
  }
}

// ✗ Invalid: Block divider in frame chrome
{
  "frames": {
    "broken": {
      "prefix": "{{ui:divider/}}",  // ERROR: divider is block-only
      "suffix": ""
    }
  }
}
```

---

## Renderable Context Annotations

Every renderable declares which contexts it supports:

### Glyphs

```json
{
  "glyphs": {
    "dot": {
      "value": "·",
      "contexts": ["inline", "block"]
    },
    "newline": {
      "value": "\n",
      "contexts": ["block"]  // Only valid in block context
    }
  }
}
```

**Rule**: Glyphs with newlines are block-only.

### Snippets

```json
{
  "snippets": {
    "sep.accent": {
      "template": "{{ui:swatch:accent/}}",
      "contexts": ["inline"],
      "inline_only": true,
      "no_newlines": true
    },
    "sep.divider": {
      "template": "{{ui:divider/}}",
      "contexts": ["block"]
    },
    "chrome.warning": {
      "template": "⚠️ ",
      "contexts": ["inline", "frame_chrome"]
    }
  }
}
```

**Rules**:
- `inline_only: true` → Only `["inline"]` contexts
- `no_newlines: true` → Expanded output validated for newlines
- Multiple contexts allowed if semantically valid

### Components

```json
{
  "components": {
    "divider": {
      "type": "native",
      "contexts": ["block"]
    },
    "swatch": {
      "type": "native",
      "contexts": ["inline", "block"]  // Works both ways
    },
    "tech": {
      "type": "native",
      "contexts": ["inline", "block"]
    },
    "header": {
      "type": "expand",
      "contexts": ["block"]
    }
  }
}
```

**Rule**: Native components declare contexts based on their semantic meaning.

### Frames

```json
{
  "frames": {
    "gradient": {
      "prefix": "▓▒░ ",
      "suffix": " ░▒▓",
      "contexts": ["inline", "block"]
    },
    "alert": {
      "prefix": "{{snippet:chrome.warning}}",
      "suffix": "",
      "contexts": ["block"]
    }
  }
}
```

**Rule**: Frames that wrap multiline content are block-only. Frames that wrap inline spans can be inline or block.

---

## Validation Rules

### Renderable → Context Validation

```rust
fn can_render_in_context(renderable: &Renderable, context: EvalContext) -> bool {
    renderable.contexts().contains(&context)
}
```

**Validation happens**:
1. At resolution time (when looking up `separator=X`)
2. At expansion time (when rendering `{{snippet:X}}`)
3. At frame instantiation time (when `prefix` references a snippet)

### Content → Context Validation

```rust
fn validate_content_for_context(content: &str, context: EvalContext) -> Result<()> {
    match context {
        EvalContext::Inline { max_graphemes } => {
            validate_inline(content, max_graphemes)?;
        }
        EvalContext::Block => {
            // No constraints
        }
        EvalContext::FrameChrome { max_length } => {
            validate_frame_chrome(content, max_length)?;
        }
    }
    Ok(())
}
```

**Validation happens**:
1. After snippet expansion (check expanded content)
2. After primitive rendering (check backend output)
3. After composition (check final result)

---

## Error Messages

### Context Mismatch Error

```
ERROR[2003]: Context mismatch

  {{mathbold:separator=sep.divider}}TEXT{{/mathbold}}
                        ^^^^^^^^^^^

  Block renderable 'sep.divider' cannot be used in Inline context.
  
  Inline contexts require compact separators (max 5 graphemes, no newlines).
  
  Suggestions:
    - Use a glyph: separator=dot, separator=arrow
    - Use an inline snippet: separator=sep.accent
    - Use a literal: separator=→
    
  Available inline separators:
    dot, bullet, arrow, star, sep.accent, sep.success
```

### Content Validation Error

```
ERROR[2004]: Invalid content for context

  {{mathbold:separator=
  }}TEXT{{/mathbold}}

  Literal separator contains newline, not allowed in Inline context.
  
  Inline separators must be compact (no newlines, max 5 graphemes).
  
  Did you mean:
    separator=newline  (for block contexts)
    separator=dot      (for inline contexts)
```

### Frame Chrome Error

```
ERROR[2005]: Invalid frame chrome

  Frame 'broken' prefix contains block renderable:
  
    "prefix": "{{ui:divider/}}"
  
  Frame chrome must be inline decorations (no newlines, max 100 chars).
  
  Component 'divider' only supports contexts: [Block]
  
  Suggestions:
    - Use an inline swatch: {{ui:swatch:accent/}}
    - Use a glyph: ⚠️ or →
    - Use an inline snippet: {{snippet:chrome.accent_block}}
```

---

## Implementation Strategy

### Phase 1: Add Context Annotations ✅ COMPLETE (ahead of schedule)

**Status**: Implemented December 2025

- ✅ Added `contexts` field to all renderables in `registry.json`
- ✅ All glyphs, snippets, components, frames, styles, badges annotated
- ✅ EvalContext enum implemented (Inline, Block, FrameChrome)

**Example**:
```json
{
  "glyphs": {
    "dot": {
      "value": "·",
      "contexts": ["inline", "block", "frame_chrome"]
    }
  },
  "components": {
    "divider": {
      "type": "native",
      "contexts": ["block"]
    }
  }
}
```

### Phase 2: Validate Contexts ✅ COMPLETE (ahead of schedule)

**Status**: Implemented December 2025

- ✅ Implemented `EvalContext::can_promote_to()` for context promotion
- ✅ Validation logic implemented in `Registry::resolve()`
- ✅ Separator resolution validates inline context
- ✅ Error messages with available glyph suggestions

**Current behavior** (separators):
```bash
$ mdfx process input.md
# Unknown multi-grapheme separator → ERROR with suggestions
# Known glyph in wrong context → ERROR with context explanation
```

**Remaining**:
- ⏳ Add `--strict-contexts` flag to CLI
- ⏳ Extend validation to all expansion sites (frames, components)
- ⏳ Add warnings mode (currently hard errors)

### Phase 3: Enforce by Default (v2.0.0) ⏳ NOT STARTED

**Status**: Planned Q3 2026

- ⏳ Context validation enabled by default for all sites
- ⏳ `--allow-context-mismatches` flag to disable (escape hatch)
- ⏳ Hard errors on mismatch

**Migration**:
```bash
# v1.x behavior in v2.x
$ mdfx process input.md --allow-context-mismatches

# v2.x default (strict)
$ mdfx process input.md
```

---

## Context Inference

### Automatic Context Detection

The compiler infers context from usage site:

```markdown
<!-- Inline context (between characters) -->
{{mathbold:separator=X}}TEXT{{/mathbold}}
          ↑ EvalContext::Inline

<!-- Block context (top-level) -->
{{ui:divider/}}
↑ EvalContext::Block

<!-- Frame chrome context (in frame definition) -->
{
  "frames": {
    "alert": {
      "prefix": "{{snippet:chrome.warning}}",
                ↑ EvalContext::FrameChrome
```

**No explicit annotation required** - context is derived from usage.

---

## Advanced: Context Promotion

**Problem**: A renderable that works in Inline should also work in Block (Inline is more restrictive).

**Solution**: Context promotion rules.

### Promotion Rules

```
Inline → Block    ✓ Always allowed
Inline → FrameChrome    ✓ Allowed if meets length constraints

Block → Inline    ✗ Never allowed
Block → FrameChrome    ✗ Never allowed

FrameChrome → Inline    ✓ Always allowed (chrome is already compact)
FrameChrome → Block    ✓ Always allowed
```

**Implementation**:
```rust
fn can_promote(from: EvalContext, to: EvalContext) -> bool {
    match (from, to) {
        (EvalContext::Inline, EvalContext::Block) => true,
        (EvalContext::Inline, EvalContext::FrameChrome) => true,
        (EvalContext::FrameChrome, _) => true,
        (EvalContext::Block, EvalContext::Inline) => false,
        (EvalContext::Block, EvalContext::FrameChrome) => false,
        (same1, same2) if same1 == same2 => true,
        _ => false,
    }
}
```

### Usage Example

```json
{
  "snippets": {
    "sep.dot": {
      "template": "·",
      "contexts": ["inline"]  // Declared as inline-only
    }
  }
}
```

```markdown
<!-- ✓ Used in inline context (exact match) -->
{{mathbold:separator=sep.dot}}TEXT{{/mathbold}}

<!-- ✓ Used in block context (promoted) -->
{{ui:section_join:separator=sep.dot}}
Item 1
Item 2
{{/ui}}
```

**Rationale**: Anything safe for inline is safe for block. The reverse is not true.

---

## Testing Contexts

### Context Validation Tests

```rust
#[cfg(test)]
mod context_tests {
    use super::*;

    #[test]
    fn test_inline_separator_accepts_glyph() {
        let registry = Registry::load()?;
        let resolved = registry.resolve("dot", EvalContext::Inline { max_graphemes: 5 })?;
        assert_eq!(resolved, "·");
    }

    #[test]
    fn test_inline_separator_rejects_block_snippet() {
        let registry = Registry::load()?;
        let result = registry.resolve("sep.divider", EvalContext::Inline { max_graphemes: 5 });
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().code(),
            ErrorCode::ContextMismatch
        );
    }

    #[test]
    fn test_inline_separator_accepts_inline_snippet() {
        let registry = Registry::load()?;
        let resolved = registry.resolve("sep.accent", EvalContext::Inline { max_graphemes: 5 })?;
        
        // Should expand to swatch primitive
        assert!(resolved.contains("swatch"));
    }

    #[test]
    fn test_frame_chrome_rejects_newlines() {
        let registry = Registry::load()?;
        let result = registry.resolve_frame_prefix(
            "{{snippet:multiline}}",
            EvalContext::FrameChrome { max_length: 100 }
        );
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().code(),
            ErrorCode::NewlineInFrameChrome
        );
    }
}
```

### Golden Tests

```markdown
<!-- test_inline_context_validation.md -->
# Inline Context Validation Tests

## Should succeed
{{mathbold:separator=dot}}TEXT{{/mathbold}}
{{mathbold:separator=→}}TEXT{{/mathbold}}
{{mathbold:separator=sep.accent}}TEXT{{/mathbold}}

## Should fail
{{mathbold:separator=sep.divider}}TEXT{{/mathbold}}
<!-- ERROR[2003]: Context mismatch -->

{{mathbold:separator=
}}TEXT{{/mathbold}}
<!-- ERROR[2004]: Newline in inline context -->
```

---

## Context-Aware Autocomplete (Future: LSP)

**Vision**: Editor extensions use context to filter suggestions.

```typescript
// VS Code language server
function provideCompletions(position: Position, context: CompletionContext) {
  const evalContext = inferContextAt(position);  // → EvalContext::Inline
  
  const registry = loadRegistry();
  const suggestions = registry.renderables
    .filter(r => r.contexts.includes(evalContext))
    .map(r => ({
      label: r.name,
      kind: CompletionItemKind.Value,
      detail: r.description,
    }));
  
  return suggestions;
}
```

**User experience**:
```markdown
{{mathbold:separator=█}}
                    ↑ Autocomplete triggers
                    
Suggestions:
  dot          (·)
  arrow        (→)
  sep.accent   (color swatch)
  
  [sep.divider not shown - block-only]
```

---

## Summary

**Contexts are the key to safe composition.**

Without contexts:
- Users inject inappropriate renderables
- Formatting breaks in subtle ways
- Hard to debug

With contexts:
- Compiler validates usage
- Clear error messages
- Safe composition guaranteed

**The three contexts**:
- **Inline**: Compact, between characters
- **Block**: Section-level, multiline
- **FrameChrome**: Inline decorations

**Implementation phases**:
- v1.1: Add context annotations (warnings)
- v1.2: Validate contexts (opt-in)
- v2.0: Enforce contexts (default)

---

**Next**: See [TARGETS.md](TARGETS.md) for multi-target rendering strategy.
