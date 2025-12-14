# Grid Component Implementation Plan

**Version:** Draft 1.0
**Last Updated:** 2025-12-14

This document outlines the implementation plan for the `grid` component in mdfx.

---

## Overview

The grid component enables structured, multi-column layouts for badges and UI elements. It generates markdown tables for Shields/SVG backends and ASCII tables for PlainText.

**Syntax Examples:**

```markdown
<!-- Basic 3-column grid -->
{{ui:grid:cols=3}}
{{ui:tech:rust/}}
{{ui:tech:typescript/}}
{{ui:tech:postgresql/}}
{{ui:tech:docker/}}
{{ui:tech:redis/}}
{{ui:tech:graphql/}}
{{/ui}}

<!-- Grid with custom gap -->
{{ui:grid:cols=4:gap=8}}
{{ui:swatch:accent/}}
{{ui:swatch:success/}}
{{ui:swatch:warning/}}
{{ui:swatch:error/}}
{{/ui}}
```

**Output (Markdown Table):**

| | | |
|:---:|:---:|:---:|
| ![](https://img.shields.io/.../rust) | ![](https://img.shields.io/.../typescript) | ![](https://img.shields.io/.../postgresql) |
| ![](https://img.shields.io/.../docker) | ![](https://img.shields.io/.../redis) | ![](https://img.shields.io/.../graphql) |

---

## Architecture Decisions

### 1. Component Type: `native` (Not `expand`)

The grid component MUST be a `native` component (returns a Primitive) rather than an `expand` component (returns a template string) because:

- **Child Parsing**: Grid needs to parse and collect child elements, then wrap them in table structure
- **Layout Logic**: Number of columns determines row breaks—this requires programmatic control
- **Per-Backend Output**: Table syntax differs between Markdown (`|...|`) and PlainText (ASCII box drawing)

### 2. New Primitive Type: `Primitive::Grid`

```rust
// In crates/mdfx/src/primitive.rs
pub enum Primitive {
    // ... existing variants ...

    /// Grid layout for multiple child elements
    Grid {
        /// Number of columns
        cols: u32,
        /// Gap between cells (in pixels, SVG-only)
        gap: Option<u32>,
        /// Child primitives to render in grid
        children: Vec<Primitive>,
        /// Alignment: "left", "center", "right"
        align: String,
    },
}
```

### 3. Parser Changes

The parser must support:

1. **Collecting Child Elements**: Parse everything between `{{ui:grid:...}}` and `{{/ui}}` as a list of child primitives
2. **Recursive Parsing**: Each child element needs full template parsing support
3. **Error Handling**: Non-primitive children (plain text) should be allowed or error with clear message

---

## Implementation Steps

### Step 1: Add Grid Primitive Variant

**File:** `crates/mdfx/src/primitive.rs`

```rust
pub enum Primitive {
    Swatch { ... },
    Divider { ... },
    Tech { ... },
    Status { ... },

    // NEW
    Grid {
        cols: u32,
        gap: Option<u32>,
        children: Vec<Primitive>,
        align: String,
    },
}
```

**Complexity:** Low
**Tests:** Unit tests for Grid primitive construction

---

### Step 2: Add Component Definition to Registry

**File:** `crates/mdfx/data/registry.json`

```json
{
  "renderables": {
    "components": {
      "grid": {
        "type": "native",
        "self_closing": false,
        "description": "Grid layout for arranging UI elements in columns",
        "contexts": ["block"],
        "args": [],
        "optional_params": {
          "cols": {
            "type": "integer",
            "default": 3,
            "description": "Number of columns (1-12)"
          },
          "gap": {
            "type": "integer",
            "default": 4,
            "description": "Gap between cells in pixels (SVG only)"
          },
          "align": {
            "type": "enum",
            "values": ["left", "center", "right"],
            "default": "center",
            "description": "Cell content alignment"
          }
        }
      }
    }
  }
}
```

**Complexity:** Low
**Tests:** Registry parsing tests

---

### Step 3: Update ComponentsRenderer for Grid

**File:** `crates/mdfx/src/components.rs`

The `expand()` method needs special handling for grid:

```rust
impl ComponentsRenderer {
    pub fn expand(
        &self,
        component: &str,
        args: &[String],
        content: Option<&str>,
    ) -> Result<ComponentOutput> {
        match component {
            "grid" => {
                // Parse grid parameters from args
                let cols = self.parse_param(args, "cols").unwrap_or(3);
                let gap = self.parse_param(args, "gap");
                let align = self.parse_param(args, "align").unwrap_or("center".to_string());

                // Return Grid primitive
                // Children will be populated by parser
                Ok(ComponentOutput::Primitive(Primitive::Grid {
                    cols,
                    gap,
                    children: vec![], // Parser fills this
                    align,
                }))
            }
            // ... existing component handling ...
        }
    }
}
```

**Complexity:** Medium (needs child coordination with parser)
**Tests:** Grid expansion tests

---

### Step 4: Update Parser for Grid Children

**File:** `crates/mdfx/src/parser.rs`

This is the most significant change. The parser needs to:

1. Detect grid opening tag
2. Parse content between `{{ui:grid:...}}` and `{{/ui}}`
3. Recursively parse each child element
4. Collect child primitives into a `Vec<Primitive>`
5. Return combined Grid primitive

```rust
// Pseudocode structure
fn parse_ui_grid(&mut self, params: &str, content: &str) -> Result<String> {
    // 1. Extract grid parameters (cols, gap, align)
    let cols = extract_param(params, "cols").unwrap_or(3);
    let gap = extract_param(params, "gap");
    let align = extract_param(params, "align").unwrap_or("center".to_string());

    // 2. Split content by line/template boundaries
    let child_templates: Vec<&str> = self.extract_child_templates(content)?;

    // 3. Parse each child template to get Primitives
    let mut children: Vec<Primitive> = Vec::new();
    for template in child_templates {
        let (primitive, _) = self.parse_ui_primitive(template)?;
        children.push(primitive);
    }

    // 4. Create Grid primitive
    let grid = Primitive::Grid { cols, gap, children, align };

    // 5. Render via backend
    let asset = self.backend.render(&grid)?;
    Ok(asset.to_markdown())
}
```

**Complexity:** High (recursive parsing, child collection)
**Tests:** Integration tests with nested templates

---

### Step 5: Implement Shields Backend Renderer

**File:** `crates/mdfx/src/renderer/shields.rs`

```rust
impl Renderer for ShieldsBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        match primitive {
            // ... existing ...

            Primitive::Grid { cols, children, align, .. } => {
                let mut rows: Vec<Vec<String>> = Vec::new();
                let mut current_row: Vec<String> = Vec::new();

                for child in children {
                    // Render each child primitive
                    let child_md = self.render(child)?.to_markdown();
                    current_row.push(child_md);

                    if current_row.len() >= *cols as usize {
                        rows.push(current_row);
                        current_row = Vec::new();
                    }
                }

                // Don't forget the last partial row
                if !current_row.is_empty() {
                    rows.push(current_row);
                }

                // Generate markdown table
                let markdown = self.generate_md_table(&rows, *cols, align);
                Ok(RenderedAsset::InlineMarkdown(markdown))
            }
        }
    }

    fn generate_md_table(&self, rows: &[Vec<String>], cols: u32, align: &str) -> String {
        let align_marker = match align {
            "left" => ":---",
            "right" => "---:",
            _ => ":---:", // center
        };

        // Header row (empty cells)
        let header = format!("|{}|", " |".repeat(cols as usize));
        let separator = format!("|{}|", format!("{}|", align_marker).repeat(cols as usize));

        let mut output = format!("{}\n{}\n", header, separator);

        for row in rows {
            // Pad row to full width if partial
            let mut cells = row.clone();
            while cells.len() < cols as usize {
                cells.push(String::new());
            }
            output.push_str(&format!("| {} |\n", cells.join(" | ")));
        }

        output
    }
}
```

**Complexity:** Medium
**Tests:** Table generation tests

---

### Step 6: Implement SVG Backend Renderer

**File:** `crates/mdfx/src/renderer/svg.rs`

The SVG backend has two options:

**Option A: Generate Markdown Table (Same as Shields)**
- Each child is rendered to its own SVG file
- Table contains `<img>` references

**Option B: Generate Single Composite SVG**
- All children positioned in a grid layout within one SVG
- Requires calculating positions based on cols, gap, child dimensions

**Recommendation:** Start with Option A for consistency, add Option B later.

```rust
Primitive::Grid { cols, children, align, gap } => {
    let mut rendered_children: Vec<RenderedAsset> = Vec::new();
    let mut markdown_refs: Vec<String> = Vec::new();

    for child in children {
        let asset = self.render(child)?;
        markdown_refs.push(asset.to_markdown());
        rendered_children.push(asset);
    }

    // Build table structure
    let table_md = self.generate_md_table(&markdown_refs, *cols, align);

    // Return composite asset
    Ok(RenderedAsset::Composite {
        markdown: table_md,
        assets: rendered_children.into_iter()
            .filter_map(|a| match a {
                RenderedAsset::File { .. } => Some(a),
                _ => None,
            })
            .collect(),
    })
}
```

**Note:** This requires a new `RenderedAsset::Composite` variant.

**Complexity:** Medium-High
**Tests:** SVG table rendering tests

---

### Step 7: Implement PlainText Backend Renderer

**File:** `crates/mdfx/src/renderer/plaintext.rs`

```rust
Primitive::Grid { cols, children, align, .. } => {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();

    for child in children {
        let child_text = self.render(child)?.to_markdown();
        current_row.push(child_text);

        if current_row.len() >= *cols as usize {
            rows.push(current_row);
            current_row = Vec::new();
        }
    }

    if !current_row.is_empty() {
        rows.push(current_row);
    }

    // ASCII table
    let output = self.generate_ascii_table(&rows, *cols, align);
    Ok(RenderedAsset::InlineMarkdown(output))
}

fn generate_ascii_table(&self, rows: &[Vec<String>], cols: u32, align: &str) -> String {
    // Calculate max width per column
    let mut col_widths: Vec<usize> = vec![0; cols as usize];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.chars().count());
        }
    }

    // Build table
    let mut output = String::new();
    let border = format!("┌{}┐\n",
        col_widths.iter().map(|w| "─".repeat(*w + 2)).collect::<Vec<_>>().join("┬"));
    output.push_str(&border);

    for (row_idx, row) in rows.iter().enumerate() {
        output.push('│');
        for (i, cell) in row.iter().enumerate() {
            let padded = self.pad_cell(cell, col_widths[i], align);
            output.push_str(&format!(" {} │", padded));
        }
        output.push('\n');

        if row_idx < rows.len() - 1 {
            let mid = format!("├{}┤\n",
                col_widths.iter().map(|w| "─".repeat(*w + 2)).collect::<Vec<_>>().join("┼"));
            output.push_str(&mid);
        }
    }

    let bottom = format!("└{}┘\n",
        col_widths.iter().map(|w| "─".repeat(*w + 2)).collect::<Vec<_>>().join("┴"));
    output.push_str(&bottom);

    output
}
```

**Complexity:** Medium
**Tests:** ASCII table rendering tests

---

### Step 8: Add RenderedAsset::Composite Variant (If Needed)

**File:** `crates/mdfx/src/renderer/mod.rs`

```rust
pub enum RenderedAsset {
    InlineMarkdown(String),
    File { ... },

    // NEW: For grid with multiple child assets
    Composite {
        /// Combined markdown output
        markdown: String,
        /// Child file assets to write
        assets: Vec<RenderedAsset>,
    },
}

impl RenderedAsset {
    pub fn to_markdown(&self) -> String {
        match self {
            RenderedAsset::InlineMarkdown(md) => md.clone(),
            RenderedAsset::File { markdown_ref, .. } => markdown_ref.clone(),
            RenderedAsset::Composite { markdown, .. } => markdown.clone(),
        }
    }
}
```

**Complexity:** Low
**Tests:** RenderedAsset tests

---

## Testing Strategy

### Unit Tests

1. **Primitive construction** - Grid primitive with valid parameters
2. **Registry parsing** - Grid component definition loads correctly
3. **Table generation** - Markdown table output formatting

### Integration Tests

1. **Full pipeline** - `{{ui:grid:cols=3}}...{{/ui}}` → rendered output
2. **Empty grid** - Handle gracefully
3. **Partial rows** - 4 items in 3-column grid
4. **Nested content** - Mixed tech/swatch/status children
5. **All backends** - Shields, SVG, PlainText output

### Edge Cases

- `cols=0` or `cols=100` (validation)
- No children
- One child (single cell)
- Exact multiple of cols (no partial row)

---

## Rollout Plan

| Phase | Description | Files |
|-------|-------------|-------|
| 1 | Add Primitive::Grid | `primitive.rs` |
| 2 | Add registry definition | `registry.json` |
| 3 | Parser child collection | `parser.rs` |
| 4 | Shields backend | `renderer/shields.rs` |
| 5 | PlainText backend | `renderer/plaintext.rs` |
| 6 | SVG backend | `renderer/svg.rs`, `renderer/mod.rs` |
| 7 | Documentation | `COMPONENTS.md`, examples |

---

## Open Questions

1. **Should plain text in grid be allowed?**
   - e.g., `{{ui:grid:cols=2}}Rust{{ui:tech:rust/}}{{/ui}}`
   - Recommendation: No—require explicit elements for now

2. **Should nested grids be allowed?**
   - e.g., `{{ui:grid:cols=2}}{{ui:grid:cols=2}}...{{/ui}}{{/ui}}`
   - Recommendation: No—keep it simple for v1

3. **Vertical alignment in SVG?**
   - When children have different heights
   - Recommendation: Center vertically by default

4. **Should we support `rows` parameter?**
   - e.g., `{{ui:grid:cols=3:rows=2}}` with auto-fill
   - Recommendation: No—let rows be implicit from child count

---

## Estimated Effort

| Component | Complexity | Estimate |
|-----------|------------|----------|
| Primitive enum | Low | Small |
| Registry entry | Low | Small |
| Parser changes | High | Significant |
| Shields renderer | Medium | Moderate |
| PlainText renderer | Medium | Moderate |
| SVG renderer | Medium-High | Moderate-Significant |
| Tests | Medium | Moderate |
| Documentation | Low | Small |

---

## Future Enhancements

After v1 grid ships:

1. **Grid templates** - Predefined layouts (`{{ui:grid:template=tech-stack}}`)
2. **Responsive hints** - Collapse to 1-column in narrow viewports
3. **Section headers** - `{{ui:grid:cols=3:title=Tech Stack}}`
4. **Spacer support** - `{{ui:spacer/}}` in grid cells

---

**Note:** This plan prioritizes simplicity and consistency with existing patterns. Complex features like composite SVG rendering can be added incrementally.
