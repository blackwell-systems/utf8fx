# mdfx Architecture

**Version:** 1.0.0
**Last Updated:** 2025-12-14

## Overview

mdfx is a **markdown compiler** that transforms template syntax into rich visual output. Unlike simple text processors, mdfx implements a complete compilation pipeline: parsing templates into an AST, semantic analysis through a unified registry, and code generation to multiple rendering backends.

**Think of it as: "The Tailwind of Markdown"** — a portable, reproducible, target-aware Markdown UI system where `registry.json` is the standard library for the language.

## Table of Contents

- [Workspace Structure](#workspace-structure)
- [System Overview](#system-overview)
- [Unified Registry](#unified-registry) 🆕
- [Target System](#target-system) 🆕
- [Three-Layer Architecture](#three-layer-architecture)
- [Component Responsibilities](#component-responsibilities)
- [Expansion Model](#expansion-model)
- [Parser Design](#parser-design)
- [Multi-Backend Rendering](#multi-backend-rendering)
- [Enhanced Swatch Primitives](#enhanced-swatch-primitives) 🆕
- [Separator System](#separator-system)
- [Asset Manifest System](#asset-manifest-system)
- [Data Packaging](#data-packaging)
- [Custom Palette Support](#custom-palette-support) 🆕
- [Performance Characteristics](#performance-characteristics)
- [Key Design Decisions](#key-design-decisions)
- [Extension Points](#extension-points)

---

## Workspace Structure

mdfx uses a Cargo workspace to separate library and CLI concerns:

```
mdfx/
├── Cargo.toml                    # Workspace root
│   └── [workspace.package]       # Shared metadata
├── crates/
│   ├── mdfx/                     # Core library (compiler)
│   │   ├── Cargo.toml           # Package: mdfx
│   │   ├── data/
│   │   │   └── registry.json    # Unified data registry (single source of truth)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── converter.rs     # Character transformation
│   │       ├── parser.rs        # Template parsing
│   │       ├── components.rs    # Component expansion
│   │       ├── primitive.rs     # Rendering-neutral AST
│   │       ├── registry.rs      # Unified registry loader
│   │       ├── targets.rs       # Target platform selection
│   │       ├── renderer/
│   │       │   ├── mod.rs       # Renderer trait
│   │       │   ├── shields.rs   # shields.io backend
│   │       │   └── svg.rs       # Local SVG backend
│   │       └── ...
│   └── mdfx-cli/                # CLI application
│       ├── Cargo.toml           # Package: mdfx-cli
│       └── src/
│           └── main.rs          # Binary: mdfx
```

### Design Rationale

**Library Crate (`mdfx`):**
- Minimal dependencies (8 total)
- No CLI-specific deps (clap, colored excluded)
- Can be embedded in other Rust applications
- Smaller compile times for library users

**CLI Crate (`mdfx-cli`):**
- Thin wrapper around compiler library
- Handles argument parsing (clap)
- Terminal formatting (colored)
- Binary named `mdfx` for UX

### Dependency Analysis

**Library (`mdfx`):**
```toml
serde = "1.0"              # JSON deserialization
serde_json = "1.0"         # Registry loading
thiserror = "2.0"          # Error handling
unicode-segmentation = "1" # Grapheme cluster support
sha2 = "0.10"              # Asset hashing
chrono = "0.4"             # Manifest timestamps
```

**CLI (`mdfx-cli`):**
```toml
mdfx = { path = "../mdfx" }     # Core compiler
clap = "4.4"                     # Argument parsing
clap_complete = "4.4"            # Shell completions
colored = "2.1"                  # Terminal colors
serde_json = "1.0"               # Custom palette loading
```

---

## System Overview

mdfx is a **markdown compiler** that transforms template syntax into styled output through a multi-stage compilation pipeline. The compiler processes Unicode character mappings, decorative frames, and multi-backend rendering through a **unified registry** and **target-aware code generation**.

### Compiler Pipeline

```
Source → Lexer → Parser → Semantic Analysis → Code Generation → Output
  │        │        │            │                   │
  │        │        │            │                   └─ ShieldsBackend / SvgBackend
  │        │        │            └─ Registry resolution, EvalContext filtering
  │        │        └─ Template AST (Primitive enum)
  │        └─ Token stream (template boundaries)
  └─ Markdown with {{templates}}
```

### Three Layers

1. **UI Components** (`{{ui:*}}`) - High-level semantic elements users write
2. **Primitives** (`{{shields:*}}`, `{{frame:*}}` / `{{fr:*}}`) - Rendering engines
3. **Styles** (`{{mathbold}}`) - Character transformations

**Key Innovation:** Components **expand** to primitives at parse time, keeping user-facing syntax concise while maintaining full customization power.

### System Diagram

```mermaid
%%{init: {'theme':'dark'}}%%
graph TB
    subgraph CLI["mdfx-cli (Binary)"]
        CMD[CLI Args] --> MAIN[main.rs]
        MAIN -->|--target| TARGET[Target Selection]
        MAIN -->|--palette| PALETTE[Custom Palette]
    end

    subgraph LIB["mdfx (Compiler Library)"]
        TARGET --> PARSER[TemplateParser]
        PALETTE --> PARSER
        PARSER --> REGISTRY[Registry]
        REGISTRY --> UI[ComponentsRenderer]

        UI -->|ComponentOutput| DECISION{Output Type?}

        DECISION -->|Primitive| BACKEND[Backend<br/>Box&lt;dyn Renderer&gt;]
        DECISION -->|Template| RECURSE[Recursive Parse]

        BACKEND -->|Trait Dispatch| SHIELDS[ShieldsBackend]
        BACKEND -->|Trait Dispatch| SVG[SvgBackend]
        BACKEND -->|Trait Dispatch| PLAIN[PlainTextBackend]

        SHIELDS --> INLINE[InlineMarkdown]
        SVG --> FILES[File Assets]
        PLAIN --> TEXT[ASCII Text]

        RECURSE --> FRAME[Frame Processing]
        RECURSE --> SHIELD[ShieldsRenderer]
        RECURSE --> CONV[Converter]

        FRAME --> OUTPUT[Compiled Output]
        SHIELD --> OUTPUT
        CONV --> OUTPUT
        INLINE --> OUTPUT
        FILES --> OUTPUT
        TEXT --> OUTPUT
    end

    subgraph DATA["Unified Registry"]
        REG[registry.json]
        REG -.->|Palette| UI
        REG -.->|Components| UI
        REG -.->|Styles| CONV
        REG -.->|Frames| FRAME
        REG -.->|Separators| PARSER
        REG -.->|Shield Styles| SHIELD
    end

    style CLI fill:#1a1a1a,stroke:#4299e1,stroke-width:3px
    style LIB fill:#1a1a1a,stroke:#48bb78,stroke-width:3px
    style DATA fill:#1a1a1a,stroke:#9f7aea,stroke-width:2px
    style BACKEND fill:#2d3748,stroke:#f56565,stroke-width:3px
    style UI fill:#2d3748,stroke:#f56565,stroke-width:2px
    style REGISTRY fill:#2d3748,stroke:#f6ad55,stroke-width:2px
```

### Core Principles

1. **Component-First** - Users write semantic `{{ui:*}}`, not verbose primitives
2. **Expansion Over Rendering** - Components expand to primitives (data, not code)
3. **Single Responsibility** - Each renderer has one clear purpose
4. **Allocation-Minimized** - Single-pass processing, streaming where possible
5. **Data-Driven** - Configuration over code (unified registry)
6. **Strict by Default** - Returns errors for invalid templates
7. **Composable** - Nest templates for complex effects
8. **Target-Aware** - Output adapts to deployment platform (GitHub, npm, local)

---

## Unified Registry

**Version:** 1.0.0
**Module:** `src/registry.rs`
**Data:** `data/registry.json`

### Overview

The unified registry consolidates all compiler data into a single JSON file, replacing the previous multi-file approach. This simplifies maintenance, ensures consistency, and enables cross-cutting features like EvalContext filtering.

### Registry Structure

```json
{
  "version": "1.0.0",
  "palette": { "accent": "F41C80", "cobalt": "2B6CB0", ... },
  "styles": { "mathbold": { ... }, "fullwidth": { ... }, ... },
  "separators": { "dot": { "char": "·" }, ... },
  "shield_styles": { "flat-square": { ... }, "flat": { ... }, ... },
  "renderables": {
    "frames": { "gradient": { "pattern": "▓▒░", "mode": "mirror" }, ... },
    "components": { "swatch": { "template": "..." }, ... }
  }
}
```

### Key Benefits

| Before (Multi-file) | After (Unified Registry) |
|---------------------|--------------------------|
| 7 JSON files to maintain | 1 JSON file |
| Inconsistent schema versions | Single version |
| No cross-references | Shared palette, consistent IDs |
| Separate parsing per file | Single parse, type-safe access |

### EvalContext System

Renderables can specify which contexts they're valid in:

```json
{
  "gradient": {
    "prefix": "▓▒░ ",
    "suffix": " ░▒▓",
    "contexts": ["cli", "github", "npm"]
  }
}
```

**Available contexts:**
- `cli` - Command line output
- `github` - GitHub README rendering
- `npm` - npm package documentation
- `local` - Local documentation (file:// or localhost)

**Filtering:** The Registry filters renderables based on the current target's context, ensuring only appropriate elements are available.

### Registry API

```rust
pub struct Registry {
    pub palette: HashMap<String, String>,
    pub styles: HashMap<String, Style>,
    pub separators: HashMap<String, Separator>,
    pub shield_styles: HashMap<String, ShieldStyle>,
    pub frames: HashMap<String, FrameStyle>,
    pub components: HashMap<String, ComponentDef>,
}

impl Registry {
    /// Load the unified registry
    pub fn new() -> Result<Self>;

    /// Resolve a renderable by name with context filtering
    pub fn resolve(&self, name: &str, context: EvalContext) -> Option<ResolvedRenderable>;

    /// Get all renderables valid for a context
    pub fn list_for_context(&self, context: EvalContext) -> Vec<&str>;
}
```

---

## Target System

**Version:** 1.0.0
**Module:** `src/targets.rs`

### Overview

The Target system allows the compiler to adapt output for different deployment platforms. Each target defines rendering preferences, context filtering, and backend selection.

### Available Targets

| Target | Backend | Use Case |
|--------|---------|----------|
| `github` | Shields | GitHub READMEs, wikis |
| `gitlab` | Shields | GitLab READMEs, wikis |
| `npm` | Shields | npm package docs |
| `pypi` | PlainText | PyPI package descriptions (ASCII-safe) |
| `local` | SVG | Offline documentation |
| `auto` | (detected) | Infer from output path |

### Target Trait

```rust
pub trait Target {
    /// Get the rendering backend type
    fn backend_type(&self) -> BackendType;

    /// Get the evaluation context for filtering
    fn eval_context(&self) -> EvalContext;

    /// Get the target name
    fn name(&self) -> &str;
}

pub enum BackendType {
    Shields,
    Svg,
    PlainText,
}
```

### CLI Usage

```bash
# Explicit target selection
mdfx process --target github README.template.md
mdfx process --target local docs/guide.template.md
mdfx process --target npm package-readme.template.md

# Auto-detection (default)
mdfx process README.md              # → github target
mdfx process docs/local/guide.md    # → local target
```

### Auto-Detection Logic

```rust
pub fn detect_target_from_path(path: &Path) -> &'static dyn Target {
    let path_str = path.to_string_lossy().to_lowercase();

    if path_str.contains("readme") || path_str.contains(".github") {
        &GitHubTarget
    } else if path_str.contains("package.json") || path_str.contains("npm") {
        &NpmTarget
    } else if path_str.contains("docs/local") || path_str.contains("offline") {
        &LocalDocsTarget
    } else {
        default_target()  // GitHub
    }
}
```

### Target-Aware Compilation

```rust
// The parser adapts to the target
let target = get_target("github");
let parser = TemplateParser::with_target(target)?;

// Components are filtered by context
// Backends are selected automatically
let output = parser.process(&input)?;
```

---

## Three-Layer Architecture

### Layer 1: UI Components (User-Facing)

**Purpose:** High-level semantic elements optimized for common use cases

**Example:**
```markdown
{{ui:swatch:accent/}}
{{ui:tech:rust/}}
```

**Characteristics:**
- Self-documenting names (`swatch`, `tech`, `row`)
- Self-closing tags (`/}}`) for contentless elements
- Generic closer (`{{/ui}}`) for ergonomics
- Design token integration (palette colors)

**Implementation:**
- Defined in `data/components.json`
- Processed by `ComponentsRenderer` (`src/components.rs`)
- Expand to primitives before rendering

### Layer 2: Primitives (Rendering Engines)

**Purpose:** Low-level rendering for specific output types

**Types:**
1. **Shields** (`{{shields:*}}`) - shields.io badge URLs as Markdown images
2. **Frames** (`{{frame:*}}` or `{{fr:*}}`) - Decorative prefix/suffix (▓▒░ TEXT ░▒▓)
3. **Glyph Frames** (`{{fr:glyph:NAME}}`) - Dynamic Unicode glyph frames

**Example:**
```markdown
{{shields:block:color=F41C80:style=flat-square/}}
{{fr:gradient}}TEXT{{/}}
{{fr:gradient:Inline Text/}}
{{fr:glyph:star*3}}FEATURED{{//}}
```

**Characteristics:**
- Verbose parameter syntax (explicit control)
- Universal closer `{{/}}` or close-all `{{//}}`
- Self-closing frames `{{fr:STYLE:CONTENT/}}`
- Direct mapping to output format
- Available as escape hatch for advanced users

**Implementation:**
- ShieldsRenderer (`src/shields.rs`) - Generate shields.io URLs
- Frame processing via Registry (`src/registry.rs`) - Add decorative borders

### Layer 3: Styles (Character Transformation)

**Purpose:** Unicode character-to-character transformations

**Example:**
```markdown
{{mathbold}}TEXT{{/mathbold}}                      → 𝐓𝐄𝐗𝐓
{{mathbold:separator=dot}}TITLE{{/mathbold}}       → 𝐓·𝐈·𝐓·𝐋·𝐄
{{script:spacing=2}}ELEGANT{{/script}}             → 𝐸  𝓁  𝑒  𝑔  𝒶  𝓃  𝓉
```

**Characteristics:**
- 24 Unicode styles (mathbold, script, fullwidth, etc.)
- Modifiers: spacing, separators
- Character-level mapping
- Composable with other layers

**Implementation:**
- Converter (`src/converter.rs`) - Character transformation
- Styles data (`data/styles.json`) - Unicode mappings

### How Layers Interact

**Input:** `{{ui:swatch:accent/}}`

**Processing:**
```
1. Layer 1 (UI):    Expand "swatch" component
   → {{shields:block:color=F41C80:style=flat-square/}}

2. Layer 2 (Shields): Generate image URL
   → ![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)

3. Output
```

**Key insight:** Expansion happens **once** at UI layer, then rendering flows through primitives/styles naturally. No special-casing needed.

---

## Multi-Backend Rendering Architecture

**Version:** 1.0.0
**Status:** ✅ Fully implemented with ShieldsBackend and SvgBackend

### Overview

UI components (swatch, tech, row, progress) render to **semantic primitives** which are then processed by a pluggable **rendering backend**. This architecture allows the same `{{ui:*}}` templates to generate different output formats without changing user code.

The backend is selected at parser construction time:
```rust
// Shields.io URLs (default)
let parser = TemplateParser::new()?;

// Local SVG files
let parser = TemplateParser::with_backend(Box::new(SvgBackend::new("assets")))?;
```

### Primitive AST

Components that generate visual elements expand to a **Primitive enum** instead of template strings:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// Single colored swatch block with optional enhancements
    Swatch {
        color: String,
        style: String,
        // Optional parameters (many are SVG-only):
        opacity: Option<f32>,         // 0.0-1.0 (SVG-only)
        width: Option<u32>,           // pixels (default: 20)
        height: Option<u32>,          // pixels (default: style-dependent)
        border_color: Option<String>, // hex or palette name (SVG-only)
        border_width: Option<u32>,    // pixels (default: 0, SVG-only)
        label: Option<String>,        // text label inside swatch
        label_color: Option<String>,  // label text color (default: white)
        icon: Option<String>,         // Simple Icons logo name
        icon_color: Option<String>,   // icon color (default: white)
    },

    /// Technology badge with logo
    Tech { name: String, bg_color: String, logo_color: String, style: String },

    /// Status indicator
    Status { level: String, style: String },
}
```

**Why primitives:**
- **Backend-neutral:** Represents *intent* (a tech badge), not implementation (shields.io URL or SVG)
- **Type-safe:** Compiler-verified parameters
- **Testable:** Can assert on primitive generation independent of rendering
- **Serializable:** Primitives can be logged, cached, or transformed
- **Extensible:** Optional fields enable advanced SVG features without breaking shields.io

### Renderer Trait

All backends implement a common interface:

```rust
pub trait Renderer {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderedAsset {
    InlineMarkdown(String),
    File {
        relative_path: String,
        bytes: Vec<u8>,
        markdown_ref: String,
    },
}
```

Helper methods:
- `file_bytes()` → `Option<&[u8]>` - Get file content for writing
- `file_path()` → `Option<&str>` - Get relative path
- `to_markdown()` → `&str` - Get markdown reference
- `is_file_based()` → `bool` - Check if requires file write

### Available Backends

#### ShieldsBackend (Default)

**Status:** ✅ Shipped in v1.0.0

Generates shields.io badge URLs wrapped in Markdown image syntax:
```rust
let backend = ShieldsBackend::new()?;
let primitive = Primitive::Tech { name: "rust", bg_color: "292A2D", logo_color: "FFFFFF", style: "flat-square" };
let rendered = backend.render(&primitive)?;
// Returns: InlineMarkdown("![](https://img.shields.io/badge/...)")
```

**Advantages:**
- Zero asset management (no files to commit)
- Works everywhere (GitHub, GitLab, crates.io docs)
- 2000+ logos via Simple Icons
- Always renders with latest shields.io features

**CLI Usage:**
```bash
mdfx process --backend shields input.md   # default
mdfx process input.md                     # same (shields is default)
```

#### SvgBackend

**Status:** ✅ Shipped in v1.0.0

Generates local SVG files with deterministic hash-based naming:
```rust
let backend = SvgBackend::new("./assets")?;
let primitive = Primitive::Swatch { color: "F41C80", style: "flat-square" };
let rendered = backend.render(&primitive)?;
// Returns: File {
//   relative_path: "assets/mdfx/swatch_541bbacc5bf498fd.svg",
//   bytes: vec![...],  // SVG file content
//   markdown_ref: "![](assets/mdfx/swatch_541bbacc5bf498fd.svg)"
// }
```

**Advantages:**
- Offline docs (works without internet)
- Full control over styling
- No external dependencies (some orgs block shields.io)
- Deterministic builds (same input → same hash → same file)

**CLI Usage:**
```bash
mdfx process --backend svg --assets-dir ./docs/ui input.md
```

**Implementation Details:**
- Hash-based filenames prevent collisions and enable caching
- Filename format: `{type}_{hash}.svg` (e.g., `swatch_a3f8e2b1.svg`)
- Hash computed from primitive parameters (color, style, etc.)
- Assets collected via `process_with_assets()` API
- Supports: Swatch, Tech, Status (solid colors)
- Tech badges use embedded Simple Icons SVG logos

#### PlainTextBackend

**Status:** ✅ Shipped in v1.0.0

Generates ASCII text representations for primitives (used by PyPI target):

```rust
let backend = PlainTextBackend::new();
let primitive = Primitive::Swatch { color: "F41C80", style: "flat-square", ... };
let rendered = backend.render(&primitive)?;
// Returns: InlineMarkdown("[#F41C80]")
```

**Rendering:**
| Primitive | Output |
|-----------|--------|
| `Swatch` | `[#RRGGBB]` |
| `Tech` | `[name]` |
| `Status` | `[OK]`, `[WARN]`, `[ERR]`, or `[INFO]` |
| `Divider` | ` ` (space) |

**Use Cases:**
- PyPI package descriptions (ASCII-only environments)
- CLI output where images aren't supported
- Graceful degradation for text-based renderers

**CLI Usage:**
```bash
mdfx process --backend plaintext input.md
mdfx process --target pypi input.md  # automatically uses plaintext
```

### Rendering Flow

```
{{ui:tech:rust/}}
  ↓ ComponentsRenderer.expand()
Primitive::Tech { name: "rust", bg_color: "292A2D", ... }
  ↓ backend.render() [trait dispatch]
  ├─ ShieldsBackend  → InlineMarkdown("![](https://img.shields.io/...)")
  └─ SvgBackend      → File {
                          relative_path: "assets/mdfx/tech_669db7effe993b2f.svg",
                          bytes: vec![...],
                          markdown_ref: "![](assets/mdfx/tech_669db7effe993b2f.svg)"
                       }
  ↓
Markdown output
```

### Dual-Mode Components

Not all components use primitives. Components fall into two categories:

**1. Native (image rendering):**
- `swatch`, `tech`, `row`
- Return `ComponentOutput::Primitive(Primitive)`
- Rendered by backend trait

**2. Expand (user-defined):**
- User-defined components in registry
- Return `ComponentOutput::Template(String)`
- Recursively parsed (can contain other templates)

Example:
```rust
match renderer.expand("tech", &["rust"], None)? {
    ComponentOutput::Primitive(p) => backend.render(&p)?,  // shields.io or SVG
    ComponentOutput::Template(t) => parser.process_templates(&t)?,  // frames/styles
}
```

### Adding New Backends

To implement a new backend:

1. Create `src/renderer/your_backend.rs`
2. Implement `Renderer` trait:
   ```rust
   pub struct YourBackend { /* ... */ }

   impl Renderer for YourBackend {
       fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
           match primitive {
               Primitive::Swatch { color, style, .. } => { /* generate output */ }
               Primitive::Tech { name, bg_color, logo_color, style } => { /* generate output */ }
               // ...
           }
       }
   }
   ```
3. Update `TemplateParser::new()` to accept backend parameter
4. Add CLI flag validation in `src/bin/main.rs`

### Design Decisions

**Q: Why not always use primitives for everything?**

A: Text effects (frames, styles, badges) compose naturally through template expansion. Converting them to primitives would complicate the parser for no benefit. Primitives are only needed for **cross-backend image rendering**.

**Q: Why not make backend selectable at component level?**

A: Consistency. Mixing shields.io and SVG in the same document creates inconsistent visual appearance. Backend is a **document-level choice**, not per-component.

**Q: Why hash-based SVG filenames?**

A: Determinism + caching. Same primitive parameters → same hash → same filename. This makes builds reproducible and prevents unnecessary file writes (important for CI).

---

## Enhanced Swatch Primitives

**Version:** 1.0.0
**Module:** `src/primitive.rs`, `src/renderer/svg.rs`

### Overview

Swatch primitives now support advanced styling options when using the SVG backend. These options are ignored by the shields.io backend, ensuring backward compatibility.

### Available Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `opacity` | `f32` | `1.0` | Transparency (0.0 = transparent, 1.0 = opaque) |
| `width` | `u32` | `20` | Width in pixels |
| `height` | `u32` | style-dependent | Height in pixels |
| `border_color` | `String` | none | Border color (hex or palette name) |
| `border_width` | `u32` | `0` | Border width in pixels |
| `label` | `String` | none | Text label inside swatch |
| `label_color` | `String` | `"white"` | Label text color |
| `icon` | `String` | none | Simple Icons logo name (e.g., "rust") |
| `icon_color` | `String` | `"white"` | Icon color |

### Template Syntax

```markdown
{{ui:swatch:FF6B35:opacity=0.5/}}
{{ui:swatch:accent:width=40:height=30/}}
{{ui:swatch:cobalt:border=FFFFFF:border_width=2/}}
{{ui:swatch:F41C80:label=v1/}}
```

### SVG Rendering

```rust
// Enhanced swatch options
struct SwatchOptions {
    color: String,
    style: String,
    opacity: f32,
    width: u32,
    height: u32,
    border_color: Option<String>,
    border_width: u32,
    label: Option<String>,
}

fn render_swatch_svg(options: &SwatchOptions) -> String {
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#,
        options.width, options.height
    );

    // Add border if specified
    if options.border_width > 0 {
        svg.push_str(&format!(
            r#"<rect width="100%" height="100%" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            options.color, options.border_color.as_deref().unwrap_or("none"), options.border_width
        ));
    }

    // Add fill with opacity
    svg.push_str(&format!(
        r#"<rect width="100%" height="100%" fill="{}" fill-opacity="{}"/>"#,
        options.color, options.opacity
    ));

    // Add label if specified
    if let Some(label) = &options.label {
        svg.push_str(&format!(
            r#"<text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle" fill="white" font-size="10">{}</text>"#,
            label
        ));
    }

    svg.push_str("</svg>");
    svg
}
```

### Backend Compatibility

| Feature | shields.io | SVG | PlainText |
|---------|------------|-----|-----------|
| Basic color | ✅ | ✅ | ✅ (as `[#hex]`) |
| Style | ✅ | ✅ | ❌ (ignored) |
| Opacity | ❌ (ignored) | ✅ | ❌ (ignored) |
| Custom size | ❌ (ignored) | ✅ | ❌ (ignored) |
| Border | ❌ (ignored) | ✅ | ❌ (ignored) |
| Label | ✅ | ✅ | ❌ (ignored) |
| Label color | ❌ (ignored) | ✅ | ❌ (ignored) |
| Icon | ❌ (ignored) | ✅ | ❌ (ignored) |
| Icon color | ❌ (ignored) | ✅ | ❌ (ignored) |

**Design principle:** Enhanced options gracefully degrade - templates work on all backends, with extra features only visible in SVG output.

---

## Separator System

**Version:** 1.0.0
**Module:** `src/separators.rs`
**Data:** Unified in `registry.json`

### Overview

The separator system allows inserting characters between styled text characters using a data-driven architecture with smart validation.

### Architecture

```rust
// Template syntax
{{mathbold:separator=dot}}TITLE{{/mathbold}}      → 𝐓·𝐈·𝐓·𝐋·𝐄
{{mathbold:separator=⚡}}POWER{{/mathbold}}        → 𝐏⚡𝐎⚡𝐖⚡𝐄⚡𝐑
{{mathbold:separator=👨‍💻}}CODE{{/mathbold}}         → 𝐂👨‍💻𝐎👨‍💻𝐃👨‍💻𝐄
```

### Data Structure

**In registry.json:**
```json
{
  "separators": {
    "dot": {
      "name": "Middle Dot",
      "char": "·",
      "unicode": "U+00B7",
      "description": "Middle dot separator for elegant spacing",
      "example": "𝐓·𝐈·𝐓·𝐋·𝐄"
    }
  }
}
```

### Resolution Logic

**Hybrid approach: Named separators + direct Unicode**

```rust
impl SeparatorsData {
    pub fn resolve(&self, input: &str) -> Result<String, String> {
        let normalized = input.trim();

        // 1. Try named separator lookup
        if let Some(sep) = self.find_separator(normalized) {
            return Ok(sep.char.clone());
        }

        // 2. Validate as direct Unicode character
        let graphemes: Vec<&str> = normalized.graphemes(true).collect();
        if graphemes.len() == 1 {
            // Reject template delimiters
            if grapheme == ":" || grapheme == "/" || grapheme == "}" {
                return Err("reserved for template syntax");
            }
            return Ok(grapheme.to_string());
        }

        // 3. Unknown - suggest alternatives
        Err(self.suggest_separator(normalized))
    }
}
```

### Grapheme Cluster Support

Uses **unicode-segmentation** crate for proper Unicode handling:

- ✅ Simple characters: `·`, `→`, `★`
- ✅ Single emoji: `⭐`, `⚡`, `🔥`
- ✅ Emoji with variation selectors: `👨‍💻` (man technologist - 5 code points, 1 grapheme)
- ✅ Flag emoji: `🇺🇸`, `🇬🇧` (2 regional indicator scalars, 1 grapheme)

**Why graphemes, not chars?**
- `"👨‍💻".chars().count()` = 5 (wrong)
- `"👨‍💻".graphemes(true).count()` = 1 (correct)

### Validation

**Normalization:**
- Trim whitespace: `"  dot  "` → `"dot"`
- Empty input rejected

**Validation:**
- Reject template delimiters (`:`, `/`, `}`)
- Must be exactly 1 grapheme cluster
- Invalid named separators get "did you mean" suggestions

**Error Example:**
```
$ mdfx process "{{mathbold:separator=starr}}TEXT{{/mathbold}}"
Error: Unknown separator 'starr'.
  Did you mean: star?
  Available named separators: dot, bullet, dash, bolddash, arrow, star, diamond, square, circle, pipe, slash, tilde
  Or use any single Unicode character (e.g., separator=⚡)
```

### CLI Command

```bash
# List all separators
mdfx separators

# With examples
mdfx separators --examples
```

Output includes ID, character, Unicode code point, description, and usage example.

---

## Asset Manifest System

**Version:** 1.0.0 (shipped as of v1.0.0)
**Module:** `src/manifest.rs`
**Dependencies:** `sha2`, `chrono`

### Overview

The manifest system tracks all generated SVG assets with SHA-256 hashing, enabling verification, cleanup, and CI caching optimization.

### Architecture

```mermaid
graph LR
    PROC[mdfx process<br/>--backend svg] --> ASSETS[Generate SVG Assets]
    ASSETS --> MANIFEST[Write manifest.json]

    MANIFEST --> VERIFY[mdfx verify]
    MANIFEST --> CLEAN[mdfx clean]

    VERIFY --> CHECK{All valid?}
    CHECK -->|Yes| OK[Exit 0]
    CHECK -->|No| FAIL[Exit 1]

    CLEAN --> ORPHAN[Find Orphaned Files]
    ORPHAN --> DELETE[Delete Unreferenced]
```

### Manifest Structure

```json
{
  "version": "1.0.0",
  "created_at": "2025-12-13T17:31:25Z",
  "backend": "svg",
  "assets_dir": "assets/mdfx",
  "total_assets": 7,
  "assets": [
    {
      "path": "assets/mdfx/swatch_8490176a786b203c.svg",
      "sha256": "2c932535cd177cd4d046122ee3a08338f50835861fb6233f2e527d282ec1ae8c",
      "type": "swatch",
      "primitive": {
        "kind": "Swatch",
        "color": "f41c80",
        "style": "flat-square"
      },
      "size_bytes": 143
    }
  ]
}
```

### Manifest API

**Core Methods:**

```rust
impl AssetManifest {
    /// Create new manifest
    pub fn new(backend: &str, assets_dir: &str) -> Self;

    /// Add asset with automatic SHA-256 hashing
    pub fn add_asset(&mut self, path: String, bytes: &[u8], primitive: &Primitive, asset_type: String);

    /// Write manifest to disk
    pub fn write(&self, manifest_path: &Path) -> Result<()>;

    /// Load existing manifest
    pub fn load(manifest_path: &Path) -> Result<Self>;

    /// Verify all assets exist with correct hashes
    pub fn verify(&self, base_dir: &Path) -> Vec<VerificationResult>;

    /// Get list of all tracked asset paths
    pub fn asset_paths(&self) -> Vec<&str>;
}
```

### Use Cases

#### 1. CI Caching Optimization

```bash
# In CI pipeline
mdfx process --backend svg README.template.md -o README.md

# Check if assets changed
if mdfx verify; then
  echo "Assets unchanged, using cache"
  # Skip expensive operations
else
  echo "Assets changed, rebuilding"
fi
```

#### 2. Integrity Verification

```bash
# Verify assets haven't been corrupted
mdfx verify --assets-dir assets/mdfx

# Output:
#   ✓ assets/mdfx/swatch_8490176a.svg
#   ✓ assets/mdfx/tech_669db7ef.svg
# ✓ All assets verified successfully!
```

#### 3. Cleanup After Refactoring

```bash
# Preview deletions
mdfx clean --dry-run --assets-dir assets/mdfx

# Actually remove orphaned files
mdfx clean --assets-dir assets/mdfx

# Output:
#   Deleting: assets/mdfx/old_badge_abc123.svg
# Deleted: 3 assets (1.2 KB)
```

### Deterministic Builds

The manifest system capitalizes on hash-based deterministic filenames:

**Same input → Same hash → Same filename → Reproducible builds**

Example:
```rust
// Primitive
Primitive::Swatch { color: "F41C80", style: "flat-square" }

// Deterministic hash (using DefaultHasher on primitive fields)
hash = 8490176a786b203c

// Deterministic filename
filename = "swatch_8490176a786b203c.svg"

// Manifest tracks relationship
{
  "path": "assets/mdfx/swatch_8490176a786b203c.svg",
  "sha256": "2c932535cd177cd4...",  // Content hash
  "primitive": { "kind": "Swatch", "color": "f41c80", ... }
}
```

### Future Enhancements

**Smart Caching (v1.1.0):**
```rust
// Before writing
if file_exists && content_hash_matches {
    return cached_asset();  // Skip disk write
}
```

**Incremental Updates:**
- Only regenerate assets with changed primitives
- Reuse existing SVGs when possible
- Track which source files reference which assets

---

## Component Responsibilities

### 1. ComponentsRenderer (`src/components.rs`)

**Purpose:** Expand UI components into either Primitives or Templates

**Key Functions:**
```rust
pub fn new() -> Result<Self>
pub fn expand(&self, component: &str, args: &[String], content: Option<&str>) -> Result<ComponentOutput>
pub fn has(&self, name: &str) -> bool
pub fn list(&self) -> Vec<(&String, &ComponentDef)>
pub fn list_palette(&self) -> Vec<(&String, &String)>
```

**ComponentOutput Enum:**
```rust
pub enum ComponentOutput {
    Primitive(Primitive),  // For image-based components (swatch, tech)
    Template(String),      // For expand-type components (user-defined)
}
```

**Expansion Algorithm:**

**For Native components (swatch, tech, row, progress):**
1. Resolve palette colors from args
2. Construct Primitive enum variant directly
3. Return `ComponentOutput::Primitive(primitive)`

**For Expand components (user-defined):**
1. Load component definition from registry
2. Substitute positional args (`$1`, `$2`, ...) with provided values
3. Substitute content (`$content`) with inner text (if not self-closing)
4. Resolve palette colors (e.g., `accent` → `F41C80`)
5. Return `ComponentOutput::Template(expanded_string)`

**Example (Primitive):**
```rust
let result = renderer.expand("tech", &["rust".to_string()], None)?;
// Returns: ComponentOutput::Primitive(
//   Primitive::Tech {
//     name: "rust",
//     bg_color: "292A2D",
//     logo_color: "FFFFFF",
//     style: "flat-square"
//   }
// )
```

**Example (Template):**
```rust
// Component definition in registry.json:
// "section": { "template": "## $1" }

let result = renderer.expand("section", &["Features".to_string()], None)?;
// Returns: ComponentOutput::Template("## Features")
```

**Design:**
- **Dual-mode expansion:** Returns Primitive OR Template based on component type
- **Palette integration:** Resolves design tokens before rendering
- **Type-safe primitives:** Image components get compiler-verified parameters
- **Template recursion:** Template-based components parsed recursively

**Data:**
- Component definitions in `registry.json` → `renderables.components`
- Palette colors in `registry.json` → `palette`

### 2. ShieldsRenderer (`src/shields.rs`)

**Purpose:** Generate shields.io badge URLs as Markdown image links

**Key Functions:**
```rust
pub fn new() -> Result<Self>
pub fn render_block(&self, color: &str, style: &str) -> Result<String>
pub fn render_twotone(&self, left: &str, right: &str, style: &str) -> Result<String>
pub fn render_bar(&self, colors: &[String], style: &str) -> Result<String>
pub fn render_icon(&self, logo: &str, bg: &str, logo_color: &str, style: &str) -> Result<String>
pub fn resolve_color(&self, color: &str) -> Result<String>
```

**Output Format:**
```markdown
![](https://img.shields.io/badge/-%20-2B6CB0?style=flat-square)
```

**Design:**
- **URL generation only:** Does not fetch images (shields.io renders on GitHub)
- **Simple Icons integration:** Uses `logo` parameter for 2000+ logos
- **Color resolution:** Palette name or 6-digit hex passthrough
- **Self-closing only:** Shields don't have inner content

**Primitives:**
1. **block** - Single solid color block
2. **twotone** - Two-color block (left/right)
3. **bar** - Multiple inline blocks
4. **icon** - Logo chip with background

**Data:**
- Shield styles in `registry.json` → `shield_styles`
- Palette in `registry.json` → `palette`

**Note:** ShieldsRenderer is used internally by `ShieldsBackend` which implements the `Renderer` trait. Direct shields.io template parsing (`{{shields:*}}`) is also supported as an escape hatch.

### 3. Renderer Trait & Backends (`src/renderer/`)

**Purpose:** Backend-agnostic rendering for Primitive types

**Trait Definition:**
```rust
pub trait Renderer {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
}
```

**RenderedAsset:**
```rust
pub enum RenderedAsset {
    InlineMarkdown(String),  // e.g., shields.io URL
    File {
        relative_path: String,  // Path for writing
        bytes: Vec<u8>,         // File content
        markdown_ref: String,   // Markdown reference
    },
}
```

**Implementations:**

**ShieldsBackend** (`src/renderer/shields.rs`):
- Default backend for v1.0.0
- Wraps `ShieldsRenderer` to implement `Renderer` trait
- Maps primitives to shields.io rendering methods:
  - `Primitive::Swatch` → `ShieldsRenderer::render_block()`
  - `Primitive::Tech` → `ShieldsRenderer::render_icon()`
  - `Primitive::Status` → `ShieldsRenderer::render_block()`
- Returns `RenderedAsset::InlineMarkdown`

**SvgBackend** (`src/renderer/svg.rs`):
- Generates local SVG files with deterministic naming
- Hash-based filenames for reproducible builds
- Returns `RenderedAsset::File` with bytes and markdown reference
- Supports all primitive types: Swatch, Tech, Status

**Design:**
- **Separation of concerns:** Shield URL generation (ShieldsRenderer) separate from backend abstraction (ShieldsBackend)
- **Extensibility:** New backends add without touching existing code
- **Type safety:** Renderer trait enforces consistent return type
- **Testability:** Can mock backends for testing parser logic

### 4. Frame Processing (via Registry)

**Purpose:** Add decorative prefix/suffix around text with multiple syntax options

**Syntax Options:**
```markdown
{{frame:gradient}}TEXT{{/frame}}       <!-- Full syntax -->
{{fr:gradient}}TEXT{{/}}               <!-- Shorthand + universal closer -->
{{fr:gradient:Inline/}}                <!-- Self-closing -->
{{fr:a}}{{fr:b}}Nested{{//}}           <!-- Close-all -->
{{fr:glyph:star*3}}TEXT{{/}}           <!-- Glyph frames -->
```

**Example:**
```rust
// Frames are processed via Registry in parser.rs
// Result: "▓▒░ TITLE ░▒▓"
```

**Design:**
- **String concatenation:** `format!("{}{}{}", frame.prefix, text, frame.suffix)`
- **No width calculation:** Frames don't adjust based on content length
- **Recursive content:** Frame content can contain styles and other frames
- **Glyph frames:** Dynamic frames from Unicode glyphs with multiplier/padding
- **27 built-in styles:** gradient, solid, lines, arrows, brackets, alerts, etc.

**Frame Types:**
- Gradient (▓▒░), solid (█▌), lines (═), arrows (→), brackets (【】), star (★☆), glyph (dynamic)

**Data:**
- Frame definitions in `registry.json` → `renderables.frames`

### 5. Converter (`src/converter.rs`)

**Purpose:** Character-to-character Unicode mapping with optional spacing/separators

**Key Functions:**
```rust
pub fn new() -> Result<Self>
pub fn convert(&self, text: &str, style: &str) -> Result<String>
pub fn convert_with_spacing(&self, text: &str, style: &str, spacing: usize) -> Result<String>
pub fn convert_with_separator(&self, text: &str, style: &str, separator: &str, count: usize) -> Result<String>
pub fn has_style(&self, name: &str) -> bool
pub fn list_styles(&self) -> Vec<&Style>
```

**Example:**
```rust
convert("HELLO", "mathbold")
// Returns: "𝐇𝐄𝐋𝐋𝐎"

convert_with_separator("TITLE", "mathbold", "·", 1)
// Returns: "𝐓·𝐈·𝐓·𝐋·𝐄"
```

**Design:**
- **Unified algorithm:** Internal `convert_with_char_between()` handles all cases
- **Streaming:** Uses `chars().peekable()` to avoid Vec allocation
- **Preserves unsupported:** Whitespace, punctuation, unsupported chars pass through
- **O(1) lookup:** HashMap for style resolution
- **24 styles:** mathbold, fullwidth, script, fraktur, monospace, etc.

**Data:**
- Style definitions in `registry.json` → `styles`

---

## Expansion Model

### Why Expansion?

**Problem:** Verbose primitives are powerful but tedious
```markdown
{{shields:icon:logo=rust:bg=292A2D:logoColor=FFFFFF:style=flat-square/}}
```

**Solution:** Components expand to primitives
```markdown
{{ui:tech:rust/}}
```

**Benefit:** Concise authoring + full customization when needed

### Expansion Flow

```
┌─────────────────────────┐
│ {{ui:swatch:accent/}}  │  ← User writes this
└─────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────────────────┐
│ ComponentsRenderer.expand("swatch", ["accent"], None) │
│ → Lookup in registry.json → renderables.components  │
│ → Substitute $1 → accent                             │
│ → Resolve palette refs → accent → F41C80            │
└─────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────────────────────────────────────────┐
│ {{shields:block:color=F41C80:style=flat-square/}}           │
└─────────────────────────────────────────────────────────────┘
           │
           ▼
┌─────────────────────────┐
│ process_templates()      │  ← Recursive parse
│ → Parse frame            │
│ → Parse style            │
│ → Render                 │
└─────────────────────────┘
           │
           ▼
┌─────────────────────┐
│ ▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓ │  ← Final output
└─────────────────────┘
```

### Expansion vs. Direct Rendering

**Expansion (current):**
- Components → Template strings → Recursive parse
- All rendering logic in primitives
- Users can override components via JSON
- No Rust compilation needed

**Direct Rendering (alternative):**
- Components → Call renderer methods directly
- Faster (skips parsing)
- Requires Rust code for each component
- No user customization

**Decision:** Expansion keeps components as **data**, not code. Enables extensibility.

---

## Parser Design

### State Machine Architecture

The parser uses a character-by-character state machine (no regex) for predictable performance and clear error messages.

### Parsing Priority

**Critical for expansion to work:**

1. **UI** (`{{ui:*}}`) - Expand first
2. **Frame** (`{{frame:*}}` / `{{fr:*}}`) - After expansion
3. **Shields** (`{{shields:*}}`) - After frames (from expanded UI)
4. **Style** (`{{mathbold}}`) - Last (innermost)

**Why this order:**
- UI must expand before primitives can parse
- Frames wrap other elements (support nesting with `{{//}}` close-all)
- Shields are leaf nodes
- Styles transform characters (innermost operation)

### Template Parsing

**Three tag types:**

**1. Self-closing** (`/}}`)
```markdown
{{ui:swatch:accent/}}
{{ui:tech:rust/}}
{{shields:block:color=accent:style=flat-square/}}
```

Parser detects `/}}` before `}}`, skips closer search.

**2. Block with generic closer** (`{{/ui}}`)
```markdown
{{ui:row:align=center}}CONTENT{{/ui}}
```

Parser uses stack to match `{{/ui}}` with most recent `ui:*` opener.

**3. Block with specific closer** (`{{/mathbold}}`)
```markdown
{{mathbold}}TEXT{{/mathbold}}
{{frame:gradient}}TEXT{{/frame}}
```

Parser searches for exact closing tag `{{/{tag}}}`.

**4. Universal closer** (`{{/}}`)
```markdown
{{fr:gradient}}TEXT{{/}}
```

Closes the most recent frame without specifying type.

**5. Close-all** (`{{//}}`)
```markdown
{{fr:gradient}}{{fr:star}}{{mathbold}}NESTED{{//}}
```

Closes ALL open tags (frames, styles, UI components) in reverse order.

**6. Self-closing frames** (`{{fr:STYLE:CONTENT/}}`)
```markdown
{{fr:gradient:Title/}}
{{fr:glyph:star*3:Featured/}}
```

Compact syntax for inline frame content.

### Parameter Parsing

**Colon-separated segments:**
```
{{type:arg1:arg2:key=value:key=value}}
```

**Rules:**
- No `=` → Positional arg
- Has `=` → Key-value param
- Order-insensitive (params stored in HashMap)

**Example:**
```markdown
{{ui:tech:rust/}}
→ component="tech", args=["rust"]

{{shields:block:color=accent:style=flat-square/}}
→ type="block", params={color: "accent", style: "flat-square"}
```

### Recursive Processing

Frames and UI components support **nested templates**:

```markdown
{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}
```

**Processing:**
1. Parse frame → extract content
2. Recursively call `process_templates()` on content
3. Apply frame to processed content

**Limitation:** No same-type nesting (e.g., `{{mb}}{{mb}}X{{/mb}}{{/mb}}` fails)

### Code Block Preservation

**Markdown code blocks skipped:**
````markdown
```rust
{{mathbold}}NOT_PROCESSED{{/mathbold}}
```
````

**Inline code skipped:**
```markdown
Use `{{mathbold}}template{{/mathbold}}` syntax
     └─ NOT processed ─┘
```

**Implementation:**
```rust
// Track code block state
let mut in_code_block = false;
if line.trim().starts_with("```") {
    in_code_block = !in_code_block;
}

// Split on backticks for inline code
let parts: Vec<&str> = line.split('`').collect();
for (i, part) in parts.iter().enumerate() {
    if i % 2 == 0 {
        // Process (outside code)
    } else {
        // Skip (inside code)
    }
}
```

---

## Data Packaging

### Unified Registry

All configuration is **embedded at compile time** using `include_str!()`:

```rust
let data = include_str!("../data/registry.json");
let registry: Registry = serde_json::from_str(data)?;
```

**Benefits:**
- No runtime file I/O
- Self-contained binary
- Works in any environment (containers, WASM, embedded systems)
- No deployment concerns
- Single source of truth for all compiler data

**Trade-off:** Users must recompile to change built-in data (runtime custom palette via `--palette` available)

### Registry Contents

| Section | Purpose | Contents |
|---------|---------|----------|
| `palette` | Design tokens | 15+ named colors |
| `styles` | Character mappings | 24 Unicode styles |
| `separators` | Named separator characters | 12 separators |
| `shield_styles` | Badge rendering styles | 5 styles (flat-square, flat, etc.) |
| `renderables.frames` | Prefix/suffix decorations | 27 frames + glyph frames |
| `renderables.components` | UI component definitions | 9 components |

**Total:** ~25KB unified `registry.json`

---

## Custom Palette Support

**Version:** 1.0.0
**Module:** `src/components.rs`, `src/parser.rs`

### Overview

Custom palettes allow projects to define their own named colors without modifying the built-in registry. This enables brand-consistent documentation across repositories.

### CLI Usage

```bash
# Use custom palette
mdfx process --palette ./brand-colors.json README.template.md

# Palette file format
{
  "brand-primary": "FF6B35",
  "brand-secondary": "2B6CB0",
  "brand-accent": "F41C80"
}
```

### Template Usage

```markdown
{{ui:swatch:brand-primary/}}
{{ui:swatch:brand-secondary/}}
{{ui:tech:rust:brand-primary/}}
```

### API

```rust
// Extend palette programmatically
let mut parser = TemplateParser::new()?;
parser.extend_palette(HashMap::from([
    ("brand".to_string(), "FF6B35".to_string()),
]));

// Or via ComponentsRenderer
let mut renderer = ComponentsRenderer::new()?;
renderer.extend_palette(custom_colors);
```

### Resolution Order

1. Custom palette (highest priority)
2. Built-in registry palette
3. Direct hex code (6-digit, e.g., `FF6B35`)

**Error handling:** Unknown color names that don't match palette or hex format return `Error::InvalidColor`

---

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|------------|-------|
| Style lookup | O(1) | HashMap |
| Character conversion | O(n) | n = input length |
| Template parsing | O(n) | Single-pass state machine |
| Component expansion | O(1) | String substitution |
| Recursive processing | O(d*n) | d = nesting depth, n = input length |

### Space Complexity

| Component | Allocation | Notes |
|-----------|------------|-------|
| Converter | Output String | Size ≈ input length * max char width (4 bytes) |
| Parser | Char vector | Converted to Vec for indexing |
| Component expansion | Template String | Small (component templates are short) |

**No regex:** Eliminates backtracking performance issues

### Optimization Notes

**Streaming in Converter:**
```rust
// Avoids Vec<char> allocation
let mut chars = text.chars().peekable();
while let Some(c) = chars.next() {
    result.push(style_obj.convert_char(c));
    if chars.peek().is_some() {
        // Add separator
    }
}
```

**Single-pass parsing:**
- Parser iterates input once
- Recursive calls for nested templates
- No backtracking

**Lazy JSON parsing:**
- JSON parsed once at renderer creation
- Stored in HashMap for O(1) lookup
- Reused across all conversions

---

## Escape Hatch Policy

Users need progressively more control. mdfx provides three tiers:

### Tier 1: UI Components (Recommended)

Portable across all backends, validated, safe:
```markdown
{{ui:swatch:accent/}}
{{ui:tech:rust/}}
{{ui:swatch:success/}}
```

### Tier 2: Primitives (Per-Backend Control)

Backend-specific but still validated:
```markdown
{{shields:block:color=F41C80:style=flat/}}
{{shields:bar:colors=success,warning,error:style=flat-square/}}
```

### Tier 3: Raw (Use At Your Own Risk)

Target-locked, zero validation:
```markdown
{{shields:raw}}https://img.shields.io/custom/badge/foo-bar-blue{{/shields}}
```

**Recommendation:** Use Tier 1 for most cases. Tier 2 for fine-grained control. Avoid Tier 3.

---

## Key Design Decisions

### Decision: Components Expand (Not Render Directly)

**Options:**
- A) Components expand to template strings → recursive parse
- B) Components call renderer methods directly in Rust

**Chose A** because:
- Components are data (JSON), not code
- Users can define custom components without recompiling
- Composability: expanded templates can use any primitive
- Simpler implementation: reuse existing parsing logic

**Trade-off:** Slight performance cost (re-parsing), but negligible for typical use

### Decision: Generic {{/ui}} Closer

**Options:**
- A) Specific closers: `{{ui:row}}...{{/ui:row}}`
- B) Generic closer: `{{ui:row}}...{{/ui}}`

**Chose B** because:
- UI is high-frequency authoring layer (ergonomics matter)
- Reduces visual noise
- Stack-based parsing is simple and reliable

**Trade-off:** Potential mismatch bugs if nesting multiple UI blocks (rare)

### Decision: Self-Closing Tags

**Options:**
- A) All tags require closers: `{{ui:swatch:accent}}{{/ui}}`
- B) Self-closing for contentless: `{{ui:swatch:accent/}}`

**Chose B** because:
- Contentless components are common (swatches, tech badges)
- Reduces verbosity by ~50% for these cases
- Familiar syntax (XML/React JSX)

**Trade-off:** Two tag syntaxes to learn

### Decision: No Pipe Syntax

**Rejected:** `{{mathbold|frame:gradient}}TEXT{{/mathbold}}`

**Reasons:**
- Ambiguous ordering (left-to-right or right-to-left?)
- Parameter passing unclear
- New grammar with edge cases
- Nesting already works and is explicit

**Alternative:** Use explicit nesting (current approach)

### Decision: Palette at Component Layer

**Options:**
- A) Palette in ComponentsRenderer (current)
- B) Palette in ShieldsRenderer
- C) Global palette shared by all renderers

**Chose A** because:
- Components are the primary user-facing API
- Color resolution happens at expansion time
- Shields sees resolved hex (no palette lookup needed)
- Single source of truth for design tokens

### Decision: Primitives as Escape Hatch

**Options:**
- A) Hide primitives entirely (only UI components)
- B) Expose primitives for advanced users (current)

**Chose B** because:
- Power users need direct control
- Debugging: can test primitives in isolation
- Migration path: existing primitive templates keep working

**Documentation strategy:** Feature UI prominently, mention primitives briefly

---

## Extension Points

### Adding New Components

**User workflow:**
1. Create `components.json` in project
2. Define component with template
3. Use `{{ui:mycomponent/}}`

**Future (v0.2):**
- mdfx will load project-local `components.json`
- Merge with built-in components
- User components override built-in

### Adding New Primitives

**Developer workflow:**
1. Create new renderer (e.g., `TableRenderer`)
2. Add data file (e.g., `tables.json`)
3. Add parser method (`parse_table_at()`)
4. Integrate into `process_templates()` priority order
5. Components can now use `{{table:*}}` in templates

### Adding New Styles

**Contributor workflow:**
1. Find Unicode codepoint ranges (e.g., Mathematical Bold Italic)
2. Update `data/styles.json` with mappings
3. Add to appropriate category
4. No code changes needed (data-driven)

### Custom Palette (Per-Project)

**Planned v0.2:**
```json
// my-project/palette.json
{
  "version": "1.0.0",
  "colors": {
    "brand": "FF6B35",
    "accent": "F41C80"
  }
}
```

Then: `{{ui:swatch:brand/}}`

### Native Components (Complex Logic)

**Planned v0.2+:**

For components requiring logic (not just template expansion):
```json
{
  "progress": {
    "type": "native",
    "handler": "progress_bar"
  }
}
```

Rust implements:
```rust
fn progress_bar(args: &[String]) -> Result<String> {
    let value = args[0].parse::<f32>()?;
    // Calculate bar segments
    // Return shields:bar with computed colors
}
```

---

## Testing Strategy

### Unit Tests

**Per-module testing:**
- `src/components.rs` - Expansion logic, palette extension
- `src/shields.rs` - URL generation
- `src/parser.rs` - Template parsing, frames, close-all, self-closing
- `src/converter.rs` - Character transformation
- `src/registry.rs` - Unified registry loading
- `src/targets.rs` - Target selection and detection
- `src/renderer/svg.rs` - SVG generation, enhanced swatches
- `src/manifest.rs` - Asset manifest, SHA-256 verification

**Total:** 280 tests across all modules

### Integration Tests

**Parser integration:**
- UI component parsing
- Composition tests (frame + style + badge)
- Recursive nesting
- Error handling
- Target-aware rendering
- Custom palette integration

### End-to-End Tests

**CLI tests:**
```bash
mdfx process README.template.md > README.md
diff README.md expected_README.md
```

**Target-specific tests:**
```bash
mdfx process --target github README.template.md
mdfx process --target local --backend svg docs/guide.template.md
```

**Custom palette tests:**
```bash
mdfx process --palette brand.json README.template.md
```

---

## References

- **Source:** `crates/mdfx/src/` directory
- **Data:** `crates/mdfx/data/registry.json` (unified)
- **Components Design:** [COMPONENTS.md](COMPONENTS.md)
- **API Guide:** [API-GUIDE.md](API-GUIDE.md)
- **Frames Guide:** [guides/FRAMES-GUIDE.md](guides/FRAMES-GUIDE.md)

---

**Document Status:** Reflects v1.0.0 implementation with markdown compiler architecture, unified registry, target system, enhanced swatch primitives, and custom palette support
