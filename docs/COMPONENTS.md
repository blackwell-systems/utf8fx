# Components Design Document

**Status:** v1.0.0 Shipped
**Last Updated:** 2025-12-14

---

## Overview

mdfx uses a **component-first architecture** where users write semantic `{{ui:*}}` elements that expand into lower-level primitives at parse time. This document explains the three-layer system, expansion model, and how to extend it.

## Architecture Layers

### 1. UI Components (User-Facing)

**What users write:**
```markdown
{{ui:header}}TITLE{{/ui}}
{{ui:divider/}}
{{ui:tech:rust/}}
```

**Purpose:** High-level semantic elements optimized for common use cases.

**Characteristics:**
- Concise syntax
- Self-documenting names
- Design token integration
- Generic `{{/ui}}` closer

### 2. Primitives (Implementation)

**What components expand to:**
```markdown
{{shields:block:color=accent:style=flat-square/}}
{{frame:gradient}}...{{/frame}}
{{badge:circle}}1{{/badge}}
```

**Purpose:** Rendering engines for specific output types (shields.io URLs, Unicode frames, badge characters).

**Characteristics:**
- Verbose parameter syntax
- Explicit closers (`{{/frame}}`, `{{/badge}}`)
- Direct control over rendering
- Available as escape hatch

### 3. Styles (Character Transformation)

**What both can use:**
```markdown
{{mathbold}}TEXT{{/mathbold}}
{{script:separator=dot}}ELEGANT{{/script}}
```

**Purpose:** Unicode character transformations (𝐀𝐁𝐂, 𝒜ℬ𝒞, ᴀʙᴄ).

**Characteristics:**
- Character-level mapping
- Separator and spacing modifiers
- Composable with other layers

## Expansion Model

### How It Works

When the parser encounters `{{ui:header}}TITLE{{/ui}}`:

1. **Parse** UI template → extract component name (`header`), content (`TITLE`)
2. **Expand** using `registry.json`:
   ```json
   "header": {
     "template": "{{frame:gradient}}{{mathbold:separator=dot}}$content{{/mathbold}}{{/frame}}"
   }
   ```
3. **Substitute** `$content` → `{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}`
4. **Recursively process** the expanded template (frame → style → render)

### Parser Priority

**Order matters** for expansion to work:

1. **UI** (expand to primitives) → `{{ui:*}}`
2. **Frame** (add prefix/suffix) → `{{frame:*}}`
3. **Badge** (enclose characters) → `{{badge:*}}`
4. **Shields** (generate image URLs) → `{{shields:*}}`
5. **Style** (transform characters) → `{{mathbold}}`

This ensures UI components can use any primitive, and primitives can use styles.

### Example Trace

Input:
```markdown
{{ui:header}}PROJECT{{/ui}}
```

Expansion steps:
```
1. Parse: ui:header with content="PROJECT"
2. Expand: {{frame:gradient}}{{mathbold:separator=dot}}PROJECT{{/mathbold}}{{/frame}}
3. Parse frame: frame_style="gradient", content="{{mathbold:separator=dot}}PROJECT{{/mathbold}}"
4. Parse style: style="mathbold", separator="dot", content="PROJECT"
5. Transform: PROJECT → 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓
6. Apply frame: ▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓 ░▒▓
7. Output
```

## Component Structure

### Registry Schema (Unified)

Components are defined in `registry.json` under `renderables.components`:

```json
{
  "version": "1.0.0",
  "renderables": {
    "components": {
      "component_name": {
        "type": "expand",
        "self_closing": true|false,
        "description": "Human-readable description",
        "args": ["arg1", "arg2"],
        "template": "{{primitive:param=$1}}$content{{/primitive}}",
        "contexts": ["inline", "block"]
      }
    }
  }
}
```

### Fields

**type** (`"expand"`)
- Currently only `"expand"` is implemented
- Future: `"native"` for Rust-implemented logic (e.g., progress bars)

**self_closing** (`boolean`)
- `true` → `{{ui:component/}}` (no closing tag)
- `false` → `{{ui:component}}content{{/ui}}` (requires `$content` in template)

**description** (`string`)
- Human-readable explanation
- Used in `mdfx components list` output

**args** (`string[]`, optional)
- Positional argument names (documentation only)
- Actual args parsed from `:arg1:arg2` syntax

**template** (`string`)
- Expansion target using primitive templates
- Variables:
  - `$1`, `$2`, ... → positional args
  - `$content` → inner content (non-self-closing only)

### Shipped Components (9 total)

#### divider
```json
{
  "type": "expand",
  "self_closing": true,
  "template": "{{shields:bar:colors=ui.bg,ui.surface,accent,ui.panel:style=flat-square/}}"
}
```

**Usage:** `{{ui:divider/}}`

**Output:** 4 inline colored blocks forming a visual separator

#### swatch
```json
{
  "type": "expand",
  "self_closing": true,
  "args": ["color"],
  "template": "{{shields:block:color=$1:style=flat-square/}}"
}
```

**Usage:** `{{ui:swatch:accent/}}`

**Output:** Single colored block (color resolved from palette or passed as hex)

#### tech
```json
{
  "type": "expand",
  "self_closing": true,
  "args": ["logo"],
  "template": "{{shields:icon:logo=$1:bg=ui.bg:logoColor=white:style=flat-square/}}"
}
```

**Usage:** `{{ui:tech:rust/}}`

**Output:** shields.io badge with Simple Icons logo

#### status
```json
{
  "type": "expand",
  "self_closing": true,
  "args": ["level"],
  "template": "{{shields:block:color=$1:style=flat-square/}}"
}
```

**Usage:** `{{ui:status:success/}}`

**Output:** Colored block (success → green, warning → yellow, error → red)

#### header
```json
{
  "type": "expand",
  "self_closing": false,
  "template": "{{frame:gradient}}{{mathbold:separator=dot}}$content{{/mathbold}}{{/frame}}"
}
```

**Usage:** `{{ui:header}}TITLE{{/ui}}`

**Output:** `▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓`

#### callout
```json
{
  "type": "expand",
  "self_closing": false,
  "args": ["level"],
  "template": "{{frame:solid-left}}{{shields:block:color=$1:style=flat-square/}} $content{{/frame}}"
}
```

**Usage:** `{{ui:callout:warning}}Message{{/ui}}`

**Output:** `█▌ 🟡 Message`

#### section
```json
{
  "type": "expand",
  "self_closing": true,
  "args": ["title"],
  "template": "## $1\n{{ui:divider/}}"
}
```

**Usage:** `{{ui:section:Features/}}`

**Output:** Markdown header (`##`) followed by divider badge row

#### callout-github
```json
{
  "type": "expand",
  "self_closing": false,
  "args": ["type"],
  "template": "{{ui:status:$1/}} **Note**\n$content",
  "post_process": "blockquote"
}
```

**Usage:** `{{ui:callout-github:warning}}Breaking changes{{/ui}}`

**Output:** Blockquote with colored status indicator (uses `post_process: blockquote` to wrap all lines with `> `)

**Types:** `success` (green), `info` (blue), `warning` (yellow), `error` (red)

#### statusitem
```json
{
  "type": "expand",
  "self_closing": true,
  "args": ["label", "level", "text"],
  "template": "{{ui:status:$2/}} **$1**: $3"
}
```

**Usage:** `{{ui:statusitem:Build:success:passing/}}`

**Output:** `![](badge) **Build**: passing`

**Composing rows:** `{{ui:statusitem:Build:success:✓/}} · {{ui:statusitem:Tests:success:276/}}`

## Design Tokens

### Palette in Registry

Colors are defined in `registry.json` under `palette`:

```json
{
  "palette": {
    "accent": "F41C80",
    "success": "22C55E",
    "ui.bg": "292A2D"
  }
}
```

Custom palettes can be loaded via the `--palette` CLI flag (see API-GUIDE.md).

### Color Resolution

**In components:**
```markdown
{{ui:swatch:accent/}}
```

**Expansion flow:**
1. Component expands to: `{{shields:block:color=accent:style=flat-square/}}`
2. Parser resolves `accent` from palette: `accent` → `F41C80`
3. Shields renderer receives resolved hex: `F41C80`

**Fallback:** If token not found in palette, pass through as-is (allows direct hex).

### Shipped Tokens

| Token | Hex | Purpose |
|-------|-----|---------|
| `accent` | F41C80 | Primary brand color |
| `slate` | 6B7280 | Neutral gray |
| `ui.bg` | 292A2D | Dark background layer |
| `ui.surface` | 292C34 | Elevated surface |
| `ui.panel` | 282F3C | Panel background |
| `ui.raised` | 263143 | Raised element |
| `success` | 22C55E | Success/positive state |
| `warning` | EAB308 | Warning/caution state |
| `error` | EF4444 | Error/danger state |
| `info` | 3B82F6 | Info/neutral state |
| `white` | FFFFFF | Pure white |
| `black` | 000000 | Pure black |
| `ink` | 111111 | Near-black text |
| `cobalt` | 2B6CB0 | Blue accent |
| `plum` | 6B46C1 | Purple accent |

### Dot Notation

Tokens support `.` for namespacing:
```json
{
  "ui.bg": "292A2D",
  "ui.surface": "292C34",
  "ui.panel": "282F3C"
}
```

This groups related colors without requiring nested objects.

## Template Syntax

### Self-Closing Tags

**Format:** `{{ui:component:arg1:arg2/}}`

**Parser behavior:**
- Detects `/}}` ending
- No closing tag required
- `content` parameter is `None`

**Example:**
```markdown
{{ui:divider/}}
{{ui:tech:rust/}}
{{ui:swatch:accent/}}
```

### Block Tags

**Format:** `{{ui:component:arg1:arg2}}content{{/ui}}`

**Parser behavior:**
- Requires generic `{{/ui}}` closer
- Stack-based matching (closes most recent `ui:*` block)
- `content` parameter is inner text

**Example:**
```markdown
{{ui:header}}TITLE{{/ui}}
{{ui:callout:warning}}Message{{/ui}}
```

### Generic Closers

**Only for UI components:** `{{/ui}}` closes any `ui:*` block.

**Other templates use specific closers:**
- `{{mathbold}}...{{/mathbold}}`
- `{{frame:gradient}}...{{/frame}}`
- `{{badge:circle}}...{{/badge}}`

**Rationale:** UI is the high-level authoring layer, so ergonomics matter. Primitives are explicit escape hatches.

### Argument Parsing

**Segments separated by `:`**

**Without `=`** → Positional arg
```markdown
{{ui:tech:rust/}}           → args = ["rust"]
{{ui:callout:warning}}      → args = ["warning"]
{{ui:multi:a:b:c/}}         → args = ["a", "b", "c"]
```

**With `=`** → Key-value param
```markdown
{{shields:block:color=accent:style=flat-square/}}
→ params = {color: "accent", style: "flat-square"}
```

**Commas allowed in args:**
```markdown
{{shields:bar:colors=success,warning,error:style=flat-square/}}
→ params = {colors: "success,warning,error", style: "flat-square"}
→ Split on `,` in renderer: ["success", "warning", "error"]
```

## Extending Components

### Adding a New Component

1. **Edit `data/registry.json`** under `renderables.components`:
```json
{
  "mycomponent": {
    "type": "expand",
    "self_closing": true,
    "description": "My custom component",
    "args": ["color"],
    "template": "{{shields:block:color=$1:style=for-the-badge/}}",
    "contexts": ["inline", "block"]
  }
}
```

2. **Use in templates**
```markdown
{{ui:mycomponent:cobalt/}}
```

3. **Recompile** - components are embedded at compile time

### Creating Project-Specific Components

Custom components are defined in `registry.json` under the `components` key. To add project-specific components, fork the registry or contribute upstream.

### Design Guidelines

**When to create a component:**
- Pattern used in 3+ places
- Complex primitive composition
- Semantic meaning (not just styling)

**When NOT to create a component:**
- One-off custom effect (use primitives directly)
- Requires runtime logic (wait for native components in v0.2+)

**Naming conventions:**
- Lowercase, hyphen-separated: `tech-stack`, `status-badge`
- Verb or noun, not adjective: `divider` (✓), `colorful` (✗)
- Self-explanatory: `header` (✓), `h1` (✗)

## Implementation Details

### ComponentsRenderer Structure

```rust
pub struct ComponentsRenderer {
    palette: HashMap<String, String>,
    components: HashMap<String, ComponentDef>,
}

pub struct ComponentDef {
    pub component_type: String,  // "expand" or "native"
    pub self_closing: bool,
    pub description: String,
    pub args: Vec<String>,
    pub template: String,
}

impl ComponentsRenderer {
    pub fn new() -> Result<Self>;
    pub fn expand(&self, component: &str, args: &[String], content: Option<&str>) -> Result<String>;
    pub fn has(&self, name: &str) -> bool;
    pub fn list(&self) -> Vec<(&String, &ComponentDef)>;
}
```

### Expansion Algorithm

```rust
fn expand(&self, component: &str, args: &[String], content: Option<&str>) -> Result<String> {
    // 1. Get component definition
    let comp = self.components.get(component)?;

    // 2. Start with template
    let mut expanded = comp.template.clone();

    // 3. Substitute args: $1, $2, ...
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("${}", i + 1);
        let resolved_arg = self.resolve_color(arg);  // Try palette lookup
        expanded = expanded.replace(&placeholder, &resolved_arg);
    }

    // 4. Substitute content: $content
    if let Some(content_str) = content {
        expanded = expanded.replace("$content", content_str);
    }

    // 5. Resolve any remaining palette refs in template
    expanded = self.resolve_palette_refs(&expanded);

    Ok(expanded)
}
```

### Parser Integration

```rust
// In process_templates():
if let Some(ui_data) = self.parse_ui_at(&chars, i)? {
    // Expand component
    let expanded = self.components_renderer.expand(
        &ui_data.component_name,
        &ui_data.args,
        ui_data.content.as_deref(),
    )?;

    // Recursively process expanded template
    let processed = self.process_templates(&expanded)?;
    result.push_str(&processed);

    i = ui_data.end_pos;
    continue;
}
```

**Key insight:** Expansion happens **before** rendering primitives, allowing components to use any lower-level feature.

## Recent Enhancements (v1.0.0)

### Enhanced Swatch Primitives

Swatches now support advanced SVG-only options:

```markdown
{{ui:swatch:FF6B35:opacity=0.5/}}
{{ui:swatch:accent:width=40:height=30/}}
{{ui:swatch:cobalt:border=FFFFFF:border_width=2/}}
{{ui:swatch:F41C80:label=v1:label_color=000000/}}
{{ui:swatch:DEA584:icon=rust:icon_color=FFFFFF/}}
```

Options: `opacity`, `width`, `height`, `border`, `border_width`, `label`, `label_color`, `icon`, `icon_color`

### Custom Palette Support

Load custom color definitions at runtime:

```bash
mdfx process --palette brand-colors.json README.template.md
```

Palette file format:
```json
{
  "brand-primary": "FF6B35",
  "brand-secondary": "2B6CB0"
}
```

### Target System

Compile for different platforms:

```bash
mdfx process --target github README.md    # shields.io backend
mdfx process --target local docs/guide.md  # SVG backend
```

## Future Enhancements

### User-Provided Components

**Planned:**
- Read `./mdfx.json` config from working directory
- Project-specific component overrides

### Native Components

**Planned:**

**Progress bars:**
```markdown
{{ui:progress:75/}}                  → ███▒▒ 75%
```

**Tables:**
```markdown
{{ui:table}}
| Name | Value |
| A    | 1     |
{{/ui}}
```

## Testing Strategy

**Component expansion tests:**
```rust
#[test]
fn test_expand_divider() {
    let renderer = ComponentsRenderer::new().unwrap();
    let result = renderer.expand("divider", &[], None).unwrap();

    assert!(result.contains("{{shields:bar"));
    assert!(result.contains("292a2d"));  // ui.bg resolved
}
```

**Parser integration tests:**
```rust
#[test]
fn test_ui_divider() {
    let parser = TemplateParser::new().unwrap();
    let input = "{{ui:divider/}}";
    let result = parser.process(input).unwrap();

    assert!(result.contains("![]("));  // Shields rendered to Markdown
    assert!(result.contains("img.shields.io"));
}
```

**End-to-end tests:**
```bash
echo "{{ui:header}}TEST{{/ui}}" | mdfx process -
# Verify: ▓▒░ 𝐓·𝐄·𝐒·𝐓 ░▒▓
```

## Troubleshooting

### Component Not Found

**Error:** `Unknown component 'mycomp'`

**Causes:**
- Typo in component name
- Component not defined in `registry.json`
- Using wrong namespace (e.g., `{{frame:mycomp}}` instead of `{{ui:mycomp}}`)

**Fix:** Check `mdfx components list` for available components.

### Template Expansion Infinite Loop

**Symptom:** Hang or stack overflow

**Cause:** Component template references itself:
```json
{
  "bad": {
    "template": "{{ui:bad/}}"  ← Recursion!
  }
}
```

**Fix:** Component templates must expand to primitives or styles, not other UI components.

### Color Not Resolving

**Symptom:** Literal string "accent" appears in output instead of `F41C80`

**Cause:** Color token not defined in palette.

**Fix:** Add the color to `registry.json` under `palette`, or inject via `--palette custom.json` at runtime.

## References

- **Implementation:** `crates/mdfx/src/components.rs` (ComponentsRenderer)
- **Data:** `crates/mdfx/data/registry.json` (unified)
- **Parser:** `crates/mdfx/src/parser.rs` (parse_ui_at, expansion logic)
- **Tests:** `crates/mdfx/src/components.rs` (tests module), `crates/mdfx/src/parser.rs` (UI tests)

---

**Document Status:** Reflects v1.0.0 shipped implementation with unified registry, enhanced swatches, custom palette support, and target system
