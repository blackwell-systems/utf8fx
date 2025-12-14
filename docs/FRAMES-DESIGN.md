# Frame System

**Status:** v1.0.0 Shipped
**Last Updated:** 2025-12-14

---

## Overview

Frames add **decorative prefix/suffix** around text without changing the characters themselves. They're simple string concatenation: `prefix + content + suffix`.

## Template Syntax

```markdown
{{frame:style}}content{{/frame}}
```

**Example:**
```markdown
{{frame:gradient}}TITLE{{/frame}}
```

**Output:**
```
▓▒░ TITLE ░▒▓
```

---

## Available Frame Styles (27)

All frames use simple string concatenation: `prefix + content + suffix`

**Categories:**
- **Gradient:** 3 styles (gradient, gradient-light, gradient-reverse)
- **Solid:** 3 styles (solid-left, solid-right, solid-both)
- **Lines:** 4 styles (line-light, line-bold, line-double, line-dashed)
- **Blocks:** 2 styles (block-top, block-bottom)
- **Arrows/Symbols:** 4 styles (arrow-right, dot, bullet, star)
- **Brackets:** 5 styles (lenticular, angle, guillemet, guillemet-single, heavy-quote)
- **Special:** 6 styles (diamond, triangle-right, finger, fisheye, asterism, arc-top, arc-bottom)

---

## Data Structure

Frames are defined in the unified `registry.json`:

```json
{
  "frames": {
    "gradient": {
      "id": "gradient",
      "name": "Gradient",
      "description": "Gradient blocks from bold to light",
      "prefix": "▓▒░ ",
      "suffix": " ░▒▓",
      "aliases": ["grad", "gradient-full"]
    },
    "solid-left": {
      "id": "solid-left",
      "name": "Solid Left",
      "description": "Solid block on the left",
      "prefix": "█▌",
      "suffix": "",
      "aliases": ["solidleft", "left"]
    }
  }
}
```

**Key points:**
- Data embedded at compile time via `include_str!()`
- Each frame has `prefix` and `suffix` (both can be empty string)
- Alias support for shorter names
- No width calculation - frames are applied as-is

---

## Composition

Frames can nest with other templates:

```markdown
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
```

**Output:**
```
▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓
```

Parser processes in priority order:
1. UI templates (expand to primitives)
2. Frame templates (outer)
3. Badge templates (middle)
4. Style templates (inner)

mdfx composes features through **nesting**, not a separate DSL. Pipe syntax like `{{mathbold|frame:double}}` is not supported - keep it simple.

---

## Use Cases

**Section headers:**
```markdown
{{frame:line-bold}}INSTALLATION{{/frame}}
```

**Warnings:**
```markdown
{{frame:solid-left}}{{negative-squared}}WARNING{{/negative-squared}}{{/frame}}
```

**Branded titles:**
```markdown
{{frame:gradient}}{{mathbold:separator=dot}}PROJECT{{/mathbold}}{{/frame}}
```

---

## Limitations

- **Single-line only** - No multiline content support
- **No width calculation** - Frames applied as-is
- **No box borders** - Just prefix/suffix, not full rectangles
- **Unicode width variance** - Suffix may not align with wide Unicode characters
- **Best for monospace** - Designed for fixed-width display

These are intentional design constraints. Frames are simple prefix/suffix decoration.

---

## Implementation

Frames are trivial string concatenation:

```rust
pub fn apply_frame(&self, text: &str, frame_style: &str) -> Result<String> {
    let frame = self.get_frame(frame_style)?;
    Ok(format!("{}{}{}", frame.prefix, text, frame.suffix))
}
```

No width calculation, no wrapping, no complexity.

---

## References

- **Implementation:** `crates/mdfx/src/frames.rs`
- **Data:** `crates/mdfx/data/registry.json` (frames section)
- **Tests:** `crates/mdfx/src/parser.rs` (frame template tests)
