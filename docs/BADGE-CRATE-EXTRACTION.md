# Badge Crate Extraction Plan

Extract badge functionality from mdfx into a standalone crate for independent use.

---

## Crate Naming

**Candidates:**
- `badgery` - memorable, "badge factory" feel
- `escutcheon` - heraldic shield term, likely available

---

## Architecture

Layered extraction for maximum reuse:

```
mdfx-colors (tiny)     →  color utilities, luminance, hex parsing
mdfx-icons (medium)    →  Simple Icons SVG paths, brand colors
badgery (main)         →  tech badges, uses above + optional glyphs
```

```
┌─────────────────────────────────────────────────────────────┐
│                      badgery crate                          │
├─────────────────────────────────────────────────────────────┤
│  pub mod badge      - TechBadge struct                      │
│  pub mod style      - SvgMetrics, badge styles              │
│  pub mod render     - SVG rendering                         │
│  pub mod shapes     - Chevron, rounded rect paths           │
│  pub mod group      - Corner grouping for badge rows        │
│  pub mod glyphs     - Optional: 500+ Unicode glyphs         │
└─────────────────────────────────────────────────────────────┘
        ↓ depends on                ↓ depends on
┌────────────────────────────┐   ┌──────────────────────────┐
│    mdfx-icons crate        │   │   mdfx-colors crate      │
├────────────────────────────┤   ├──────────────────────────┤
│ - get_icon_path()          │   │ - get_luminance()        │
│ - get_brand_color()        │   │ - darken_color()         │
│ - list_icons()             │   │ - get_contrast_color()   │
│ - 20+ Simple Icons         │   │ - hex parsing            │
└────────────────────────────┘   └──────────────────────────┘
```

---

## Phase 1: Extract Color Utilities

**Crate:** `mdfx-colors`
**Size:** ~150 lines
**Complexity:** Low

### Tasks

- [ ] Create `crates/mdfx-colors/Cargo.toml`
- [ ] Extract `get_luminance(hex: &str) -> f32`
- [ ] Extract `get_contrast_color(bg_hex: &str) -> &'static str` (white/black selection)
- [ ] Extract `darken_color(hex: &str, amount: f32) -> String`
- [ ] Extract `parse_hex(s: &str) -> Option<(u8, u8, u8)>`
- [ ] Add tests
- [ ] Publish to crates.io (optional)

### Public API

```rust
pub fn luminance(hex: &str) -> f32;
pub fn contrast_color(bg: &str) -> &'static str;
pub fn darken(hex: &str, amount: f32) -> String;
pub fn parse_hex(s: &str) -> Option<(u8, u8, u8)>;
```

---

## Phase 2: Extract Icon Library

**Crate:** `mdfx-icons`
**Size:** ~700 lines
**Complexity:** Low

### Tasks

- [ ] Create `crates/mdfx-icons/Cargo.toml`
- [ ] Move `get_icon_path(name: &str) -> Option<&'static str>` (20+ Simple Icons)
- [ ] Move `get_brand_color(name: &str) -> Option<&'static str>`
- [ ] Add `list_icons() -> &[&str]` for discovery
- [ ] Add `list_brands() -> &[&str]`
- [ ] Add dependency on `mdfx-colors`
- [ ] Add tests
- [ ] Publish to crates.io (optional)

### Public API

```rust
pub fn icon_path(name: &str) -> Option<&'static str>;
pub fn brand_color(name: &str) -> Option<&'static str>;
pub fn list_icons() -> &'static [&'static str];
pub fn list_brands() -> &'static [&'static str];
```

### Icons Included

- rust, typescript, javascript, python, go, java
- react, vue, svelte, angular
- docker, kubernetes, postgresql, redis, mongodb
- git, github, gitlab
- nodejs, deno, bun
- And more...

---

## Phase 3: Create Badge Crate

**Crate:** `badgery`
**Size:** ~1,200 lines
**Complexity:** Medium

### Directory Structure

```
crates/badgery/
├── Cargo.toml
└── src/
    ├── lib.rs           # Public API, re-exports
    ├── badge.rs         # TechBadge struct
    ├── style.rs         # SvgMetrics, badge styles (flat, plastic, etc.)
    ├── render.rs        # SVG rendering logic
    ├── shapes.rs        # Chevron paths, rounded rect paths
    ├── group.rs         # Corner grouping for badge rows
    └── glyphs.rs        # Optional: glyph registry (500+ chars)
```

### Tasks

- [ ] Create `crates/badgery/Cargo.toml` with features
- [ ] Define `TechBadge` struct (extracted from `Primitive::Tech`):
  ```rust
  pub struct TechBadge {
      pub name: String,
      pub label: Option<String>,
      pub style: BadgeStyle,
      pub bg_color: Option<String>,
      pub logo_color: Option<String>,
      pub text_color: Option<String>,
      pub border: Option<Border>,
      pub corners: Option<Corners>,
      pub logo_size: LogoSize,
      pub chevron: Option<Chevron>,
      pub raised: Option<u32>,
      pub outline: bool,
  }
  ```
- [ ] Move `SvgMetrics` and style definitions
- [ ] Move rendering functions:
  - `render_with_options()`
  - `render_two_segment()`
  - `render_icon_only()`
  - `render_text_only()`
- [ ] Move shape generation:
  - `chevron_path_with_overlap()`
  - `rounded_rect_path()`
- [ ] Move corner grouping logic from `tech_group.rs`
- [ ] Add optional glyphs feature
- [ ] Create builder API
- [ ] Add comprehensive tests
- [ ] Add examples

### Public API

```rust
// Core types
pub struct TechBadge { ... }
pub struct BadgeBuilder { ... }
pub enum BadgeStyle { Flat, FlatSquare, Plastic, ForTheBadge, Social }
pub enum LogoSize { Xs, Sm, Md, Lg, Xl, Xxl }

// Rendering
pub fn render(badge: &TechBadge) -> String;
pub fn render_to_file(badge: &TechBadge, path: &Path) -> io::Result<()>;

// Builder pattern
pub fn badge(name: &str) -> BadgeBuilder;

// Example usage:
let svg = badgery::badge("rust")
    .label("v1.80")
    .style(BadgeStyle::FlatSquare)
    .raised(4)
    .render();
```

### Cargo.toml Features

```toml
[features]
default = []
glyphs = []  # Include 500+ Unicode glyph mappings

[dependencies]
mdfx-colors = { version = "1.0", path = "../mdfx-colors" }
mdfx-icons = { version = "1.0", path = "../mdfx-icons" }
```

---

## Phase 4: Wire Back to mdfx

**Size:** ~200 lines changes
**Complexity:** Medium

### Tasks

- [ ] Add `badgery` as dependency in `mdfx/Cargo.toml`
- [ ] Update `Primitive::Tech` to use `badgery::TechBadge` internally or convert
- [ ] Update tech handler to construct `TechBadge`
- [ ] Update SVG renderer to call `badgery::render()`
- [ ] Ensure backward compatibility for existing mdfx users
- [ ] Update tests
- [ ] Update documentation

### Integration Points

1. **Component Dispatch** - route "tech" to handler that builds `TechBadge`
2. **Primitive Rendering** - dispatch `Tech` variant to `badgery::render()`
3. **Color Resolution** - pass palette colors through to badge builder
4. **Glyph Expansion** - mdfx expands `{{glyph:...}}` before passing label to badgery

---

## Glyphs: Include or Separate?

### Recommendation: Include as Optional Feature

**Size:** ~500+ character mappings
**Implementation:** HashMap or match statement

### Rationale

- Glyphs enhance badges: `★ Rust`, `✓ Passing`, `⚡ Fast`
- Self-contained: users get badges + decorative elements in one crate
- Optional: `features = ["glyphs"]` keeps core small
- Already integrated: `{{glyph:star.filled/}}` in labels works

### Glyph Categories

| Category | Examples | Count |
|----------|----------|-------|
| Symbols | ★ ✓ ✗ ⚡ ♥ | ~50 |
| Arrows | → ← ↑ ↓ ⇒ | ~30 |
| Math | ± × ÷ ∞ ≈ | ~40 |
| Greek | α β γ δ Ω | ~48 |
| Box Drawing | ─ │ ┌ ┐ └ | ~40 |
| Blocks | ▀ ▄ █ ░ ▒ | ~20 |
| Emoji/Misc | ☀ ☁ ☂ ⚙ | ~100+ |
| Text Styles | 𝐀-𝐙 (bold), 𝔄-ℨ (fraktur) | ~200+ |
| **Total** | | **500+** |

### Glyph API

```rust
#[cfg(feature = "glyphs")]
pub mod glyphs {
    pub fn get(name: &str) -> Option<&'static str>;
    pub fn list() -> impl Iterator<Item = (&'static str, &'static str)>;
    pub fn categories() -> &'static [&'static str];
    pub fn by_category(cat: &str) -> impl Iterator<Item = (&'static str, &'static str)>;
}
```

---

## File Mapping

| Source (mdfx) | Destination | Lines |
|---------------|-------------|-------|
| `primitive.rs` (Tech variant) | `badgery/src/badge.rs` | 38 |
| `handlers/tech.rs` | `badgery/src/badge.rs` (builder) | 147 |
| `handlers/tech_group.rs` | `badgery/src/group.rs` | 157 |
| `renderer/svg/tech.rs` | `badgery/src/render.rs` + `shapes.rs` | 934 |
| `renderer/svg/tech.rs` (colors) | `mdfx-colors/src/lib.rs` | 150 |
| `renderer/svg/tech.rs` (icons) | `mdfx-icons/src/lib.rs` | 700 |
| `registry.json` (glyphs) | `badgery/src/glyphs.rs` | 200 |
| **Total** | | **~2,300** |

---

## Estimated Effort

| Phase | Lines | Sessions |
|-------|-------|----------|
| 1. Colors | 150 | 0.5 |
| 2. Icons | 700 | 1 |
| 3. Badge crate | 1,200 | 2 |
| 4. Wire back | 200 | 1 |
| **Total** | ~2,250 | **4-5 sessions** |

---

## Success Criteria

- [ ] `badgery` compiles and passes tests independently
- [ ] Can render badges without mdfx dependency
- [ ] mdfx still works with badges via badgery
- [ ] All existing tech badge tests pass
- [ ] Builder API is ergonomic
- [ ] Documentation with examples
- [ ] Optional: published to crates.io

---

## Example Usage (Final API)

```rust
use badgery::{badge, BadgeStyle};

// Simple badge
let svg = badge("rust").render();

// Full customization
let svg = badge("typescript")
    .label("TypeScript")
    .style(BadgeStyle::FlatSquare)
    .bg("#3178C6")
    .text_color("#FFFFFF")
    .border("#FFFFFF", 2)
    .raised(4)
    .logo_size_lg()
    .render();

// Save to file
badge("docker")
    .label("Container")
    .render_to_file("docker-badge.svg")?;

// With glyphs (requires "glyphs" feature)
#[cfg(feature = "glyphs")]
let label = format!("{} Rust", badgery::glyphs::get("star.filled").unwrap());
```
