# Components Design Document

**Status:** v1.0.0 Shipped
**Last Updated:** 2025-12-16

---

## Overview

mdfx uses a **component-first architecture** where users write semantic `{{ui:*}}` elements that expand into lower-level primitives at parse time. This document explains the three-layer system, expansion model, and how to extend it.

## Architecture Layers

### 1. UI Components (User-Facing)

**What users write:**
```markdown
{{ui:tech:rust/}}
{{ui:swatch:accent/}}
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
```

**Purpose:** Rendering engines for specific output types (shields.io URLs, Unicode frames).

**Characteristics:**
- Verbose parameter syntax
- Explicit closers (`{{/frame}}`) or universal closer (`{{/}}`)
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

When the parser encounters `{{ui:swatch:accent/}}`:

1. **Parse** UI template → extract component name (`swatch`), args (`accent`)
2. **Expand** using `registry.json`:
   ```json
   "swatch": {
     "template": "{{shields:block:color=$1:style=flat-square/}}"
   }
   ```
3. **Substitute** `$1` → `{{shields:block:color=accent:style=flat-square/}}`
4. **Resolve palette** → `accent` becomes `F41C80`
5. **Render** shields URL

### Parser Priority

**Order matters** for expansion to work:

1. **UI** (expand to primitives) → `{{ui:*}}`
2. **Frame** (add prefix/suffix) → `{{frame:*}}` / `{{fr:*}}`
3. **Shields** (generate image URLs) → `{{shields:*}}`
4. **Style** (transform characters) → `{{mathbold}}`

This ensures UI components can use any primitive, and primitives can use styles.

### Example Trace

Input:
```markdown
{{ui:tech:rust/}}
```

Expansion steps:
```
1. Parse: ui:tech with args=["rust"]
2. Expand: {{shields:icon:logo=rust:bg=ui.bg:logoColor=white:style=flat-square/}}
3. Resolve palette: ui.bg → 292A2D
4. Generate shields URL: https://img.shields.io/badge/...
5. Output: ![](https://img.shields.io/badge/...)
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

**type** (`"expand"` or `"native"`)
- `"expand"` - Template substitution components (user-defined partials)
- `"native"` - Rust-implemented logic (swatch, tech, row, progress)

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

### Shipped Components (6 total)

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

#### row
```json
{
  "type": "native",
  "self_closing": false,
  "description": "Horizontal row of badges with alignment control",
  "contexts": ["block"],
  "optional_params": {
    "align": {
      "type": "enum",
      "values": ["left", "center", "right"],
      "default": "center"
    }
  }
}
```

**Usage:** `{{ui:row}}{{ui:tech:rust/}} {{ui:tech:go/}}{{/ui}}`

**With alignment:** `{{ui:row:align=left}}...{{/ui}}`

**Output:**
```html
<p align="center">
<img alt="" src="https://img.shields.io/...rust..."> <img alt="" src="https://img.shields.io/...go...">
</p>
```

**How it works:**
1. Content is recursively parsed (tech badges render to `![](url)`)
2. Delayed post-processing converts `![alt](url)` → `<img alt="alt" src="url">`
3. Wraps in `<p align="...">` for GitHub compatibility

**Why HTML output?** GitHub Flavored Markdown doesn't parse markdown syntax inside HTML blocks, so we must emit `<img>` tags directly for alignment to work.

#### progress
```json
{
  "type": "native",
  "self_closing": true,
  "description": "Progress bar with customizable track and fill",
  "contexts": ["inline", "block"],
  "args": ["percent"],
  "optional_params": {
    "width": { "type": "number", "default": "100" },
    "height": { "type": "number", "default": "10" },
    "fill_height": { "type": "number", "default": "same as height" },
    "track": { "type": "color", "default": "slate" },
    "fill": { "type": "color", "default": "accent" },
    "rx": { "type": "number", "default": "3" },
    "label": { "type": "boolean", "default": "false" },
    "label_color": { "type": "color", "default": "white" },
    "border": { "type": "color", "default": "none" },
    "border_width": { "type": "number", "default": "1" },
    "thumb": { "type": "number", "default": "none" },
    "thumb_color": { "type": "color", "default": "same as fill" },
    "thumb_shape": { "type": "string", "default": "circle" }
  }
}
```

**Basic usage:** `{{ui:progress:75/}}`

**Customized:** `{{ui:progress:50:width=200:height=16:fill=success/}}`

**Floating fill effect:** `{{ui:progress:80:height=12:fill_height=8/}}`

**With label:** `{{ui:progress:65:width=150:label=true/}}`

**With border:** `{{ui:progress:70:width=150:border=accent/}}`

**Slider mode (thumb):** `{{ui:progress:50:width=200:thumb=14/}}`

**Slider with shapes:** `{{ui:progress:60:thumb=14:thumb_shape=diamond:thumb_color=warning/}}`

**How it works:**
1. Generates SVG with two overlapping rectangles (track + fill)
2. Fill width calculated as `(width * percent / 100)`
3. If `fill_height < height`, fill is vertically centered
4. Label shows percentage text centered on the bar
5. When `thumb` is set, renders slider mode: thin track with positioned thumb

**Slider mode:**
- `thumb=14` enables slider mode with 14px thumb
- `thumb_width`: independent width for oval/pill shapes (defaults to thumb)
- `thumb_shape`: circle (default), square, or diamond
- `thumb_color`: defaults to fill color

**Backends:**
- **SVG:** Full rendering with all features
- **Shields.io:** Fallback badge showing "XX%"
- **Plaintext:** ASCII bar `[=====>    ] 50%`

#### donut
```json
{
  "type": "native",
  "self_closing": true,
  "description": "Circular progress ring/donut chart",
  "contexts": ["inline", "block"],
  "args": ["percent"],
  "optional_params": {
    "size": { "type": "number", "default": "40" },
    "thickness": { "type": "number", "default": "4" },
    "track": { "type": "color", "default": "slate" },
    "fill": { "type": "color", "default": "accent" },
    "label": { "type": "boolean", "default": "false" },
    "label_color": { "type": "color", "default": "white" },
    "thumb": { "type": "number", "default": "none" },
    "thumb_color": { "type": "color", "default": "same as fill" }
  }
}
```

**Basic usage:** `{{ui:donut:75/}}`

**Customized:** `{{ui:donut:50:size=60:thickness=8:fill=success/}}`

**With label:** `{{ui:donut:85:label=true/}}`

**Slider mode:** `{{ui:donut:75:thumb=12:thumb_color=accent/}}`

**How it works:**
1. Generates SVG with circular arc using stroke-dasharray
2. Fill percentage determines arc length (clockwise from top)
3. Optional thumb positioned at fill endpoint using trigonometry

#### gauge
```json
{
  "type": "native",
  "self_closing": true,
  "description": "Semi-circular gauge/meter for dashboards",
  "contexts": ["inline", "block"],
  "args": ["percent"],
  "optional_params": {
    "size": { "type": "number", "default": "80" },
    "thickness": { "type": "number", "default": "8" },
    "track": { "type": "color", "default": "slate" },
    "fill": { "type": "color", "default": "accent" },
    "label": { "type": "boolean", "default": "false" },
    "label_color": { "type": "color", "default": "white" },
    "thumb": { "type": "number", "default": "none" },
    "thumb_color": { "type": "color", "default": "same as fill" }
  }
}
```

**Basic usage:** `{{ui:gauge:75/}}`

**Speedometer style:** `{{ui:gauge:50:size=100:thickness=10:fill=warning/}}`

**With label:** `{{ui:gauge:85:label=true/}}`

**Slider mode:** `{{ui:gauge:50:thumb=16:thumb_color=warning/}}`

**How it works:**
1. Generates SVG with semi-circular arc (180° span)
2. Arc goes from left to right (9 o'clock to 3 o'clock)
3. Fill percentage determines arc length
4. Optional thumb positioned at fill endpoint

#### sparkline
```json
{
  "type": "native",
  "self_closing": true,
  "description": "Mini inline chart for data visualization",
  "contexts": ["inline", "block"],
  "args": ["values (comma-separated)"],
  "optional_params": {
    "width": { "type": "number", "default": "100" },
    "height": { "type": "number", "default": "20" },
    "type": { "type": "enum", "values": ["line", "bar", "area"], "default": "line" },
    "fill": { "type": "color", "default": "accent" },
    "stroke": { "type": "color", "default": "same as fill" },
    "stroke_width": { "type": "number", "default": "2" },
    "track": { "type": "color", "default": "none" },
    "dots": { "type": "boolean", "default": "false" },
    "dot_radius": { "type": "number", "default": "2" }
  }
}
```

**Basic usage:** `{{ui:sparkline:1,3,2,6,4,8,5,7/}}`

**Bar chart:** `{{ui:sparkline:1,3,2,6,4,8,5,7:type=bar/}}`

**Area chart:** `{{ui:sparkline:1,3,2,6,4,8,5,7:type=area/}}`

**With dots:** `{{ui:sparkline:1,3,2,6,4,8,5,7:dots=true/}}`

**Custom size:** `{{ui:sparkline:1,3,2,6,4,8,5,7:width=200:height=40/}}`

**Custom colors:** `{{ui:sparkline:1,3,2,6,4,8,5,7:fill=success:stroke=warning/}}`

**How it works:**
1. Values are normalized to fit within the height (auto-scaling)
2. For **line** type: renders polyline with optional dots at data points
3. For **bar** type: renders vertical rectangles for each value
4. For **area** type: renders filled polygon under the line
5. All chart types support track color for background

**Backends:**
- **SVG:** Full rendering with all features (line, bar, area charts)
- **Shields.io:** Fallback badge showing data range (min..max)
- **Plaintext:** Unicode sparkline using block characters (▁▂▃▄▅▆▇█)

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
{{ui:row:align=center}}badges{{/ui}}
```

### Generic Closers

**Only for UI components:** `{{/ui}}` closes any `ui:*` block.

**Other templates use specific closers:**
- `{{mathbold}}...{{/mathbold}}`
- `{{frame:gradient}}...{{/frame}}` or `{{fr:gradient}}...{{/}}`

**Rationale:** UI is the high-level authoring layer, so ergonomics matter. Primitives are explicit escape hatches.

### Argument Parsing

**Segments separated by `:`**

**Without `=`** → Positional arg
```markdown
{{ui:tech:rust/}}           → args = ["rust"]
{{ui:swatch:accent/}}       → args = ["accent"]
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
- Verb or noun, not adjective: `header` (✓), `colorful` (✗)
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
fn test_expand_swatch() {
    let renderer = ComponentsRenderer::new().unwrap();
    let result = renderer.expand("swatch", &["accent"], None).unwrap();

    assert!(result.contains("{{shields:block"));
    assert!(result.contains("F41C80"));  // accent color resolved
}
```

**Parser integration tests:**
```rust
#[test]
fn test_ui_swatch() {
    let parser = TemplateParser::new().unwrap();
    let input = "{{ui:swatch:accent/}}";
    let result = parser.process(input).unwrap();

    assert!(result.contains("![]("));  // Shields rendered to Markdown
    assert!(result.contains("img.shields.io"));
}
```

**End-to-end tests:**
```bash
echo "{{ui:swatch:accent/}}" | mdfx process -
# Verify: ![](https://img.shields.io/badge/...)
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

**Document Status:** Reflects v1.0.0 shipped implementation with unified registry, enhanced swatches, custom palette support, target system, donut/gauge components, slider thumb support, thumb_width for progress bars, and sparkline charts
