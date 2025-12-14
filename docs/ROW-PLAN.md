# Row Component Design

**Version:** 1.0
**Status:** Proposed

A minimal layout component for inline badge/element arrangement with optional alignment.

---

## Motivation

Most README badge displays are horizontal rows:

```
[Rust] [TypeScript] [Go] [Docker]
```

Currently users write this manually with spaces. A `row` component adds:

1. **Centering** - GitHub requires `<p align="center">` HTML wrapper
2. **Consistent spacing** - No guessing how many spaces between badges
3. **Semantic intent** - Clearer than raw HTML

---

## Design

### Syntax

```markdown
<!-- Basic row (left-aligned, default spacing) -->
{{ui:row}}
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:go/}}
{{/ui}}

<!-- Centered row -->
{{ui:row:align=center}}
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:go/}}
{{/ui}}

<!-- Custom gap (space count between items) -->
{{ui:row:gap=2}}
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:go/}}
{{/ui}}
```

### Parameters

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `align` | enum | `left` | `left`, `center`, `right` |
| `gap` | integer | `1` | Number of spaces between children (0-4) |

### Output by Backend

**Shields (GitHub/GitLab):**

```markdown
<!-- align=left (default) -->
![](badge1) ![](badge2) ![](badge3)

<!-- align=center -->
<p align="center">
![](badge1) ![](badge2) ![](badge3)
</p>
```

**SVG:**
Same as Shields - outputs markdown with `<img>` tags for local SVGs.

**PlainText:**
```
[Rust] [TypeScript] [Go]

          [Rust] [TypeScript] [Go]          (centered, if terminal width known)
```

---

## Implementation

### Option A: Expand Type (Simplest)

Add to `registry.json`:

```json
"row": {
  "type": "expand",
  "self_closing": false,
  "description": "Horizontal row of inline elements",
  "contexts": ["block"],
  "args": [],
  "template": "$content"
}
```

**Pros:** Zero code changes, works today
**Cons:** No alignment, no gap control - just semantic wrapper

### Option B: Native Type with Alignment

```json
"row": {
  "type": "native",
  "self_closing": false,
  "description": "Horizontal row with alignment control",
  "contexts": ["block"],
  "args": [],
  "optional_params": {
    "align": {
      "type": "enum",
      "values": ["left", "center", "right"],
      "default": "left"
    }
  }
}
```

Implementation in `components.rs`:

```rust
"row" => {
    let align = extract_param(args, "align").unwrap_or("left");
    let content = content.unwrap_or("");

    match align {
        "center" => Ok(ComponentOutput::Template(
            format!("<p align=\"center\">\n{}\n</p>", content)
        )),
        "right" => Ok(ComponentOutput::Template(
            format!("<p align=\"right\">\n{}\n</p>", content)
        )),
        _ => Ok(ComponentOutput::Template(content.to_string())),
    }
}
```

**Pros:** Alignment works, minimal complexity
**Cons:** Still passes content through unparsed (no gap control)

### Option C: Native Type with Child Parsing

Full child-aware implementation (like Grid but simpler):

```rust
"row" => {
    let align = extract_param(args, "align").unwrap_or("left");
    let gap = extract_param(args, "gap").unwrap_or(1);

    // Parse children to primitives
    let children = parse_children(content)?;

    // Render each child, join with gap spaces
    let spacer = " ".repeat(gap);
    let rendered = children.iter()
        .map(|c| backend.render(c))
        .collect::<Vec<_>>()
        .join(&spacer);

    // Wrap if aligned
    match align {
        "center" => format!("<p align=\"center\">\n{}\n</p>", rendered),
        "right" => format!("<p align=\"right\">\n{}\n</p>", rendered),
        _ => rendered,
    }
}
```

**Pros:** Full control over spacing
**Cons:** Requires child parsing infrastructure (same complexity as Grid)

---

## Recommendation

**Start with Option B** (native with alignment, no gap parsing).

Rationale:
- Centering is the #1 missing feature for badge rows
- Gap control is nice-to-have; users can add spaces manually
- Avoids child parsing complexity entirely
- Can upgrade to Option C later if needed

---

## Implementation Steps

| Step | Task | Complexity |
|------|------|------------|
| 1 | Add `row` to registry.json | Low |
| 2 | Add native handling in `components.rs` | Low |
| 3 | Test with all backends | Low |
| 4 | Document in COMPONENTS.md | Low |

**Total effort:** Small

---

## Examples

### Tech Stack (Centered)

```markdown
{{ui:row:align=center}}
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:go/}} {{ui:tech:docker/}}
{{/ui}}
```

Output:
```html
<p align="center">
![Rust](https://img.shields.io/badge/-Rust-000000?style=flat-square&logo=rust&logoColor=white) ![TypeScript](https://img.shields.io/badge/-TypeScript-3178C6?style=flat-square&logo=typescript&logoColor=white) ...
</p>
```

### Status Row

```markdown
{{ui:row:align=center}}
{{ui:status:success/}} {{ui:status:warning/}} {{ui:status:error/}}
{{/ui}}
```

### Mixed Content

```markdown
{{ui:row}}
{{ui:swatch:accent/}} **mdfx** {{ui:swatch:accent/}}
{{/ui}}
```

---

## Future: Upgrade Path to Gap Control

If gap control becomes needed, upgrade to Option C:

1. Add `Primitive::Row { children, align, gap }`
2. Implement child parsing in parser
3. Render children with spacer insertion

But this is the same infrastructure Grid needs, so at that point just implement both.

---

## Comparison: Row vs Grid

| Feature | Row | Grid |
|---------|-----|------|
| Layout | Inline horizontal | Table cells |
| Alignment | left/center/right | Per-cell |
| Wrapping | No (single line) | Yes (multi-row) |
| Child parsing | Optional | Required |
| Complexity | Low | High |
| Use case | Badge strips | Tech matrices |

**Row is the 80% solution. Grid is the 20% edge case.**
