# mdfx Design Document

**What mdfx is: A Markdown compiler with target-aware rendering**

Version: 2.0 (Design)  
Status: **Foundational Architecture**  
Last Updated: 2025-12-13

---

## Executive Summary

**mdfx is not "a set of visual widgets for GitHub READMEs."**

**mdfx is a compile step for Markdown**: a small styling DSL + design system + asset pipeline, where Markdown with templates is the **source**, and "target-optimized Markdown + optional assets" is the **build artifact**.

More precisely, mdfx is:

1. **A DSL / macro system for Markdown** - `{{ui:...}}` and `{{frame:...}}` are language constructs, not helpers
2. **A renderer-agnostic intermediate representation** - The `Primitive` enum is an IR, backends are code generators
3. **A deterministic asset pipeline** - Hash-based filenames, reproducible builds, `process_with_assets()` is a compiler
4. **A data-driven component registry** - `components.json` is a standard library for the language

**Think of it as: "The Tailwind of Markdown" — a portable, reproducible, target-aware Markdown UI system.**

---

## The Three-Layer Conceptual Model

All mdfx architecture decisions should preserve these three distinct layers:

### Layer 1: Primitives (Semantic Atoms)

```rust
/// The IR (Intermediate Representation)
/// These are semantic, not visual - they describe INTENT, not implementation
#[non_exhaustive]
pub enum Primitive {
    Divider { style: String },
    Swatch { color: String, style: String },
    Tech { logo: String, style: String },
    Status { level: String, style: String },
    // Future: Pill, Grid, Table, etc.
}
```

**Key insight**: A `Swatch` doesn't "know" how to render. It says "I am a color block with this color and style." Backends decide what that means for their target.

### Layer 2: Renderables (Expansion System)

```
Renderables: Things that expand into markdown and may emit assets

Types of renderables:
- Glyphs: literal text (·, →, ⚡)
- Snippets: template strings that expand to markdown
- Components: native (Rust) or expand (template-based)
- Frames: decorative prefix/suffix wrappers
```

**Key insight**: Everything that produces markdown output uses the same resolution pipeline. Frames can embed snippets, snippets can reference components, separators can be swatches.

### Layer 3: Targets/Backends (Code Generation)

```rust
pub trait Renderer {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
}

pub trait Target {
    fn supports_html(&self) -> bool;
    fn supports_svg_embed(&self) -> bool;
    fn preferred_backend(&self) -> BackendType;
}
```

**Key insight**: GitHub gets shields.io URLs, local docs get SVG files, PyPI gets plain text fallbacks. Same source, different targets.

---

## The Compiler Pipeline

```
┌───────────────────────────────────────────────────────────┐
│                INPUT: Source Markdown + DSL               │
│                                                            │
│  # {{ui:header}}PROJECT{{/ui}}                            │
│  {{ui:divider/}}                                           │
│  {{ui:tech:rust/}} {{mathbold:separator=dot}}TEXT{{/..}}  │
└────────────────────────┬──────────────────────────────────┘
                         │
                         ↓ Phase 1: Parse
┌───────────────────────────────────────────────────────────┐
│                    AST (Abstract Syntax Tree)              │
│                                                            │
│  TemplateNode::Component("ui", "header", ["PROJECT"])     │
│  TemplateNode::Component("ui", "divider", [])             │
│  TemplateNode::Component("ui", "tech", ["rust"])          │
└────────────────────────┬──────────────────────────────────┘
                         │
                         ↓ Phase 2: Expand Components
┌───────────────────────────────────────────────────────────┐
│         IR: Primitives + Markdown (Intermediate Rep)       │
│                                                            │
│  Primitive::Divider { style: "flat-square" }              │
│  Primitive::Tech { logo: "rust", style: "flat-square" }   │
│  "𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓" (expanded mathbold)                        │
└────────────────────────┬──────────────────────────────────┘
                         │
                         ↓ Phase 3: Render via Backend
           ┌─────────────┴──────────────┐
           ↓                            ↓
┌──────────────────────┐   ┌────────────────────────┐
│   ShieldsBackend     │   │     SvgBackend         │
│  (GitHub target)     │   │  (Local docs target)   │
└──────────┬───────────┘   └───────────┬────────────┘
           │                           │
           ↓                           ↓
┌──────────────────────┐   ┌────────────────────────┐
│  OUTPUT: Markdown    │   │ OUTPUT: Markdown +     │
│  with shields URLs   │   │ Asset Files            │
│                      │   │                        │
│  ![](https://img...) │   │ ![](assets/swatch.svg) │
│                      │   │ + assets/manifest.json │
└──────────────────────┘   └────────────────────────┘
```

---

## The Unified Renderables Registry

**Problem**: We currently have fragmented systems (separators.json, components.json, frames.json, palette.json) that can't reference each other cleanly.

**Solution**: One unified registry where everything that produces markdown output follows the same resolution rules.

### Proposed Schema v2.0

```json
{
  "version": "2.0.0",
  "renderables": {
    
    "glyphs": {
      "dot": {
        "value": "·",
        "description": "Middle dot",
        "contexts": ["inline", "block"]
      },
      "arrow": {
        "value": "→",
        "contexts": ["inline"]
      }
    },

    "snippets": {
      "sep.accent": {
        "template": "{{ui:swatch:accent/}}",
        "description": "Accent color swatch separator",
        "inline_only": true,
        "no_newlines": true,
        "contexts": ["inline"]
      },
      "sep.gradient_bar": {
        "template": "{{ui:divider/}}",
        "description": "Full-width gradient divider",
        "contexts": ["block"]
      },
      "chrome.warning": {
        "template": "⚠️ ",
        "contexts": ["inline", "frame_chrome"]
      }
    },

    "components": {
      "divider": {
        "type": "native",
        "self_closing": true,
        "contexts": ["block"],
        "args": [],
        "description": "Visual divider bar"
      },
      "swatch": {
        "type": "native",
        "self_closing": true,
        "contexts": ["inline", "block"],
        "args": ["color"],
        "description": "Single color block"
      }
    },

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
      },
      "color_accent": {
        "prefix": "{{snippet:chrome.accent_block}}",
        "suffix": "",
        "contexts": ["block"]
      }
    }
  }
}
```

### Resolution Algorithm

```rust
fn resolve_renderable(name: &str, context: EvalContext) -> Result<Renderable> {
    // 1. Check glyphs registry
    if let Some(glyph) = registry.glyphs.get(name) {
        return validate_context(glyph, context)?;
    }

    // 2. Check snippets registry
    if let Some(snippet) = registry.snippets.get(name) {
        return validate_context(snippet, context)?;
    }

    // 3. Check components registry
    if let Some(component) = registry.components.get(name) {
        return validate_context(component, context)?;
    }

    // 4. Treat as literal grapheme cluster
    validate_literal_grapheme(name, context)
}
```

**Key insight**: Separators, frames, components all use the same resolution pipeline. The distinction is **context** (inline vs block), not type.

---

## Evaluation Contexts: The Key to Safety

**Problem**: Without contexts, users can inject multiline markdown in an inline join and explode formatting.

**Solution**: Every renderable has a context, every expansion site has a context, validation enforces compatibility.

### Context Types

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalContext {
    /// Inline: between characters in styled text
    /// Constraints: max 5 graphemes, no newlines
    Inline {
        max_graphemes: usize,  // Default: 5
    },
    
    /// Block: section-level, multiline allowed
    /// Constraints: none
    Block,
    
    /// FrameChrome: frame prefix/suffix
    /// Constraints: max 100 chars, no newlines
    FrameChrome {
        max_length: usize,  // Default: 100
    },
}
```

### Context Validation Examples

```markdown
<!-- ✓ Valid: inline glyph in inline context -->
{{mathbold:separator=dot}}TEXT{{/mathbold}}

<!-- ✓ Valid: inline snippet (swatch) in inline context -->
{{mathbold:separator=sep.accent}}TEXT{{/mathbold}}

<!-- ✗ Invalid: block snippet in inline context -->
{{mathbold:separator=sep.gradient_bar}}TEXT{{/mathbold}}
ERROR: Block renderable 'sep.gradient_bar' cannot be used in Inline context

<!-- ✓ Valid: block snippet in frame chrome -->
{{frame:divider_framed}}CONTENT{{/frame}}
where divider_framed.prefix = "{{snippet:sep.gradient_bar}}\n"

<!-- ✗ Invalid: literal newline in inline context -->
{{mathbold:separator=
}}TEXT{{/mathbold}}
ERROR: Literal separator contains newline, not allowed in Inline context
```

---

## Target Abstraction: Multi-Surface Rendering

**Vision**: Same source markdown compiles to different targets with appropriate optimizations.

### Target Trait

```rust
pub trait Target {
    /// Target identifier
    fn name(&self) -> &str;
    
    /// Does this target support raw HTML in markdown?
    fn supports_html(&self) -> bool;
    
    /// Does this target support embedded SVG?
    fn supports_svg_embed(&self) -> bool;
    
    /// Max line length (None = unlimited)
    fn max_line_length(&self) -> Option<usize>;
    
    /// Preferred backend for this target
    fn preferred_backend(&self) -> BackendType;
    
    /// Target-specific post-processing
    fn post_process(&self, markdown: &str) -> String {
        markdown.to_string()  // Default: no-op
    }
}
```

### Shipped Targets (v1.x - v2.x)

```rust
pub struct GitHubTarget;
impl Target for GitHubTarget {
    fn name(&self) -> &str { "github" }
    fn supports_html(&self) -> bool { false }  // GitHub strips most HTML
    fn supports_svg_embed(&self) -> bool { true }
    fn preferred_backend(&self) -> BackendType { BackendType::Shields }
}

pub struct LocalDocsTarget;
impl Target for LocalDocsTarget {
    fn name(&self) -> &str { "local" }
    fn supports_html(&self) -> bool { true }
    fn supports_svg_embed(&self) -> bool { true }
    fn preferred_backend(&self) -> BackendType { BackendType::Svg }
}

// Future targets:
// - GitLabTarget (supports more HTML)
// - PyPITarget (plain text fallback)
// - NpmTarget (similar to GitHub)
```

### CLI Integration

```bash
# v1.x: Implicit GitHub target
mdfx process input.md -o output.md

# v2.x: Explicit target selection
mdfx process input.md --target github -o output.md
mdfx process input.md --target gitlab -o output.md
mdfx process input.md --target pypi -o output.md

# v2.x: Auto-detect from output context
mdfx process input.md -o README.md           # Auto: github
mdfx process input.md -o docs/index.md       # Auto: local
mdfx process input.md -o PKG-INFO            # Auto: pypi
```

---

## Escape Hatch Policy

Users need progressively more control. We provide three tiers:

### Tier 1: Primitives (Recommended)

**Portable across all backends, validated, safe**

```markdown
{{shields:block:color=F41C80:style=flat/}}
{{shields:bar:colors=success,warning,error:style=flat-square/}}
```

### Tier 2: Backend-Specific (Per-Backend Escape Hatch)

**Backend-locked, but still validated**

```markdown
<!-- ShieldsBackend: raw URL -->
{{shields:raw}}https://img.shields.io/custom/badge/foo-bar-blue{{/shields}}

<!-- SvgBackend: embed custom SVG -->
{{svg:embed}}path/to/custom.svg{{/svg}}
```

### Tier 3: Target-Specific (Use At Your Own Risk)

**Target-locked, zero validation, user responsibility**

```markdown
<!-- GitHub-specific HTML (may break on other targets) -->
{{target:github:html}}
<details>
<summary>Click to expand</summary>
This only works on GitHub.
</details>
{{/target}}

<!-- GitLab-specific directives -->
{{target:gitlab:include}}snippets/common.md{{/target}}
```

**Warning**: Tier 3 escapes are **explicitly unsupported**. They bypass all validation and may break when switching targets.

---

## Stability Contracts and Versioning

### What is STABLE (guaranteed not to break)

**Template Syntax (forever)**:
- Double-brace delimiters: `{{ }}`
- Self-closing: `{{name/}}`
- Block: `{{name}}content{{/closer}}`
- Colon args: `{{ui:component:arg1:arg2}}`
- Named params: `{{style:key=value}}`

**Primitive IR (v1.x)**:
- `Primitive` enum is `#[non_exhaustive]` (can add variants)
- Existing variants won't change structure
- New fields added as `Option<T>` or with defaults

**Component Schema (versioned)**:
```json
{
  "version": "2.0.0",  // Schema version
  "components": {
    "divider": {
      "api_version": "1.0.0",  // Component-specific version
      ...
    }
  }
}
```

### What CAN EVOLVE (semver minor)

- New component names
- New primitive types
- New backends
- New targets
- Optional parameters (with defaults)

### What BREAKS (semver major)

- Template syntax changes
- Primitive enum restructuring
- Required parameter changes
- Component renames/removals
- Backend trait signature changes

### LTS Policy

- **v1.x**: Supported for 2 years after v2.0 release
- **Security fixes**: Continue for v1.x after 2-year support window
- **Migration guides**: Provided for all major version bumps

---

## Deterministic Builds and Reproducibility

**Goal**: Same input + same config = same output + same asset filenames

### Requirements for Determinism

1. **Hash-based asset filenames** (already implemented)
   ```rust
   fn asset_filename(primitive: &Primitive) -> String {
       let hash = deterministic_hash(primitive);
       format!("{}_{:x}.svg", primitive.kind(), hash)
   }
   ```

2. **Stable template expansion order**
   - Components expand in parse order (left-to-right, top-to-bottom)
   - Snippets expand depth-first

3. **No timestamp dependencies in output**
   - Manifest timestamps are metadata only
   - Markdown output contains no timestamps

4. **No filesystem path dependencies**
   - All paths relative to assets directory
   - Works across different machines/CI environments

### Verification

```bash
# Build twice, compare outputs
mdfx process input.md -o dist1/output.md
mdfx process input.md -o dist2/output.md
diff -r dist1/ dist2/  # Should be identical
```

---

## The Reality Check Test

> "Given the same inputs and config, it produces the same output markdown and the same asset filenames, and the output is correct for multiple targets."

**v1.0 status**: ✅ for GitHub + shields/svg backends  
**v2.0 goal**: ✅ for GitHub/GitLab/PyPI/npm + all backends

---

## Implementation Phases

### Phase 0: v1.0.0 (December 2025) ✅ COMPLETE

**Shipped**:
- ✅ Template parser with 19 Unicode styles
- ✅ Component system (9 components)
- ✅ Multi-backend rendering (shields.io, SVG)
- ✅ Badge style control
- ✅ Asset manifest system
- ✅ 237 passing tests

**Position as**: "Markdown styling tool with component system"

### Phase 1: v1.1.0 - Unified Registry ✅ COMPLETE (ahead of schedule)

**Implemented** (December 2025):
- ✅ Created unified `registry.json` consolidating ALL 7 data files (1139 lines)
- ✅ Implemented `Registry` module with complete typed API
- ✅ Extended separator resolution to use glyphs from registry
- ✅ Added snippets (10) for template expansion
- ✅ Parser refactored to use Registry for validation
- ✅ Maintained backward compatibility with v1.0 templates

**Migration**: None required, purely additive

### Phase 2: v1.2.0 - Context System ✅ COMPLETE (ahead of schedule)

**Implemented** (December 2025):
- ✅ Added `contexts` field to all renderables in registry.json
- ✅ Implemented `EvalContext` enum (Inline, Block, FrameChrome)
- ✅ Implemented context promotion rules
- ✅ Validate context compatibility for separators
- ✅ Error messages with available glyph suggestions

**Migration**: Existing templates work; context validation enforced for separators

### Phase 3: v2.0.0 - Full Compiler Model ⏳ PARTIALLY COMPLETE

**Completed**:
- ✅ Consolidated all data files into unified registry
- ✅ Implemented Target trait abstraction
- ✅ Shipped targets: GitHubTarget, LocalDocsTarget, NpmTarget
- ✅ BackendType enum with derived Default

**Remaining**:
- ⏳ Wire `--target` flag into CLI
- ⏳ Multi-target compilation command
- ⏳ Target-aware backend selection
- ⏳ Remove deprecated v1.x APIs (separators.json still loaded by old code)

**Migration**: Provide `mdfx migrate` tool to auto-convert v1 → v2

---

## Design Principles (The Rules)

These principles guide all architectural decisions:

### 1. Context-Aware, Always

**Every renderable has a context. Every expansion site has a context. Validate compatibility.**

Bad:
```rust
fn resolve(name: &str) -> Result<String>
```

Good:
```rust
fn resolve(name: &str, context: EvalContext) -> Result<String>
```

### 2. Primitives Are Semantic, Not Visual

**A primitive describes INTENT (what), backends decide IMPLEMENTATION (how).**

Bad:
```rust
enum Primitive {
    ShieldsUrl(String),  // Visual implementation leaked
}
```

Good:
```rust
enum Primitive {
    Swatch { color: String, style: String },  // Semantic intent
}
```

### 3. Same Species, Different Contexts

**Don't special-case blocks, separators, frames. They're all renderables with different contexts.**

Bad:
```rust
fn resolve_separator(name: &str) -> Result<String>
fn resolve_block(name: &str) -> Result<Block>
fn resolve_frame(name: &str) -> Result<Frame>
```

Good:
```rust
fn resolve_renderable(name: &str, context: EvalContext) -> Result<Renderable>
```

### 4. Backends Generate Code, Don't Hardcode

**The Primitive IR is target-agnostic. Backends compile it for specific targets.**

Bad:
```rust
impl Primitive {
    fn render(&self) -> String {
        // Hardcoded to shields.io
    }
}
```

Good:
```rust
trait Renderer {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
}
// ShieldsBackend, SvgBackend, etc. implement Renderer
```

### 5. Data-Driven, Not Code-Driven

**New components, snippets, frames should be JSON additions, not Rust code.**

Bad: Add new component by writing Rust code

Good: Add new component by editing `components.json`

Exception: "Native" components with complex logic (like `divider` with themed colors)

### 6. Explicit Over Implicit

**Don't guess. Make the user specify target, backend, assets directory.**

Bad:
```bash
mdfx process input.md  # Where do assets go? What target?
```

Good:
```bash
mdfx process input.md --target github --backend svg --assets-dir assets/
```

v1.x compromise: Sensible defaults with explicit flags available

### 7. Fail Early, Fail Clearly

**Validate at compile-time. Provide actionable error messages.**

Bad:
```
Error: Invalid separator
```

Good:
```
Error: Block renderable 'sep.gradient_bar' cannot be used in Inline context

  {{mathbold:separator=sep.gradient_bar}}TEXT{{/mathbold}}
                        ^^^^^^^^^^^^^^^^

  Inline contexts require compact separators (max 5 graphemes, no newlines).
  
  Suggestions:
    - Use a glyph: separator=dot, separator=arrow
    - Use an inline snippet: separator=sep.accent
    - Use a literal: separator=→
```

---

## File Organization (Proposed v2.0)

```
crates/mdfx/
├── src/
│   ├── lib.rs                  # Public API
│   ├── compiler/               # NEW: Compiler abstraction
│   │   ├── mod.rs
│   │   ├── pipeline.rs         # Phase 1-4 orchestration
│   │   └── context.rs          # EvalContext system
│   ├── ir/                     # NEW: Intermediate representation
│   │   ├── mod.rs
│   │   ├── primitive.rs        # Primitive enum (moved)
│   │   └── renderable.rs       # Renderable trait
│   ├── parser/                 # Phase 1: Parse
│   │   ├── mod.rs
│   │   └── template.rs
│   ├── expander/               # Phase 2: Expand
│   │   ├── mod.rs
│   │   ├── components.rs
│   │   └── registry.rs         # Unified registry
│   ├── backends/               # Phase 3: Render
│   │   ├── mod.rs
│   │   ├── shields.rs
│   │   └── svg.rs
│   ├── targets/                # NEW: Target abstraction
│   │   ├── mod.rs
│   │   ├── github.rs
│   │   ├── gitlab.rs
│   │   └── pypi.rs
│   └── ...
└── data/
    ├── registry.json           # NEW: Unified (v2.0)
    ├── components.json         # DEPRECATED (v2.0)
    ├── separators.json         # DEPRECATED (v2.0)
    ├── frames.json             # DEPRECATED (v2.0)
    └── palette.json            # MIGRATED into registry.json
```

---

## What Makes mdfx Different

### Not "Yet Another Markdown Renderer"

Most markdown tools:
- Parse markdown → Render to HTML/PDF
- Focus on output formats

mdfx:
- Parse markdown → Transform markdown → Output markdown
- Focus on source augmentation

### Not "Badge Generator"

Badge tools (shields.io):
- Generate badge URLs
- One-off, no composition

mdfx:
- Composes badges into larger systems
- Deterministic, reproducible, version-controlled

### Not "README Template Engine"

Template engines:
- String substitution
- No semantic understanding

mdfx:
- Semantic primitives
- Context-aware validation
- Type-safe composition

### The Closest Analog: Tailwind CSS

**Tailwind for HTML**:
- Utility classes → Compiled CSS
- Design tokens (colors, spacing)
- Build step, deterministic output

**mdfx for Markdown**:
- Template syntax → Compiled markdown
- Design tokens (palette, components)
- Build step, deterministic output

---

## Success Criteria

**v1.0 Success**: Used in 100+ GitHub projects for README styling

**v2.0 Success**: Used in multi-repo organizations as standard README compiler

**v3.0 Success**: Integrated into documentation platforms (Docusaurus, MkDocs, etc.)

---

## Open Questions for Community Discussion

1. **Should mdfx be opinionated about design?**
   - Ship with default palette and components?
   - Or require users to define everything?

2. **How aggressive should context validation be?**
   - Hard errors or warnings?
   - Lint mode vs strict mode?

3. **Should mdfx support plugins?**
   - User-defined backends in separate crates?
   - User-defined primitives?

4. **How should versioning work for data files?**
   - Each data file has its own version?
   - Or single schema version for all?

5. **Should mdfx have a "preview" mode?**
   - Generate HTML preview without committing?
   - Watch mode for live updates?

---

## Conclusion

**mdfx is a compiler, not a widget collection.**

By embracing this identity early, we can:
- Make better architectural decisions
- Attract the right contributors
- Set correct user expectations
- Build a sustainable, extensible system

**The north star**: Semantic primitives + deterministic compilation + graceful degradation = A design system that travels well across targets.

**Ship v1.0, then refactor toward this design.**

---

**Next Steps**:
1. Review this design doc with stakeholders
2. Create detailed specs for Phase 1 (Unified Registry)
3. Write migration guide (v1 → v2)
4. Update README to position as compiler/design system

---

*This is the first public face of mdfx. This is what we're building.*
