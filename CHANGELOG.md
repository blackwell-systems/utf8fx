# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Enhanced Asset Management System

Major improvements to asset generation and tracking (manifest v1.1.0):

**Content-Addressed Filenames**
- Filenames now use SHA-256 content hashing instead of Rust's `DefaultHasher`
- Stable across Rust versions (no CI cache invalidation on toolchain updates)
- True deduplication: identical content always produces identical filenames
- Format: `{type}_{sha256-prefix}.svg` (e.g., `swatch_a3f8e2d1c4b5a6f7.svg`)

> **⚠️ Migration Note:** Asset filenames will change on first re-render after upgrading.
> The old `DefaultHasher`-based names were unstable across Rust versions. The new
> SHA-256-based names are stable forever. Run `mdfx clean` after re-rendering to
> remove old assets, or simply delete your assets directory and regenerate.

**Atomic Manifest Writes**
- Manifests are now written atomically using temp file + rename
- Prevents corruption on crash/interrupt during write
- Use `manifest.write_atomic(&path)` for safe writes

**Incremental Manifest Updates**
- New `manifest.merge(new_assets, keep_paths)` method
- Updates existing manifests without full regeneration
- Preserves provenance tracking across builds

**Provenance Tracking**
- Each asset now tracks:
  - `source_files`: Template files that reference this asset
  - `generated_at`: RFC3339 timestamp of generation
  - `generator_version`: mdfx version that created the asset
- Manifest includes `total_size_bytes` and `generator_version`

**API Additions**
```rust
// Content-addressed filename generation
use mdfx::manifest::content_addressed_filename;
let filename = content_addressed_filename(svg_bytes, "swatch");

// Atomic writes
manifest.write_atomic(&path)?;

// Incremental updates
manifest.merge(new_assets, Some(keep_paths));

// Asset lookup
manifest.get_asset("path/to/asset.svg");
manifest.has_content_hash("abc123...");

// Statistics
let stats = manifest.stats();
println!("Total: {} bytes", stats.total_size_bytes);
```

#### CLI Discovery Commands

The `mdfx list` command now supports listing all resource types for better discoverability:

```bash
mdfx list                    # List styles (default)
mdfx list styles --samples   # List styles with sample output
mdfx list components         # List all UI components with params
mdfx list glyphs             # List named glyphs (grouped by prefix)
mdfx list frames             # List frame styles with previews
mdfx list palette            # List palette colors (grouped by type)
mdfx list glyphs -f star     # Filter results by name pattern
```

Each listing shows relevant details:
- **Components**: Description and available parameters
- **Glyphs**: Grouped by prefix (e.g., `star.*`, `arrow.*`) with character preview
- **Frames**: Aliases, prefix/suffix preview, and description
- **Palette**: Grouped into Semantic, UI/Dark Theme, and General categories

#### Expanded Tech Icon Library (23 new icons)

The mdfx-icons crate now includes 90+ technology icons, up from 70:

**DevOps Extended (10 icons):**
- `pulumi` - Infrastructure as code
- `vagrant` - Development environments
- `helm` - Kubernetes package manager
- `argo` / `argocd` - GitOps continuous delivery
- `consul` - Service mesh and discovery
- `vault` - Secrets management
- `datadog` - Monitoring and analytics
- `sentry` - Error tracking
- `newrelic` - Observability platform

**Testing (4 icons):**
- `vitest` - Vite-native testing framework
- `cypress` - E2E testing
- `selenium` - Browser automation
- `mocha` - JavaScript test framework

**AI/ML (6 icons):**
- `tensorflow` - Machine learning framework
- `pytorch` - Deep learning library
- `huggingface` - AI model hub
- `jupyter` - Interactive notebooks
- `pandas` - Data analysis
- `numpy` - Numerical computing

**Runtime/Package Managers (3 icons):**
- `pnpm` - Fast, disk-efficient package manager
- `bun` - All-in-one JavaScript runtime
- `deno` - Secure JavaScript/TypeScript runtime

#### Full Badge Border (`border_full`)

Tech badges now support `border_full=true` to draw a border around the entire badge perimeter instead of just the icon segment.

**Usage:**
```markdown
{{ui:tech:react:bg=0D0D0D:border=61DAFB:border_full=true/}}
```

**Behavior:**
- `border` alone → Border only on left (icon) segment (original layered approach)
- `border` + `border_full=true` → Border around entire badge edge

The full border is rendered as a separate outline rect on top of all segments, creating a clean perimeter without a dividing line between icon and label areas.

#### Segment Divider (`divider`)

Tech badges now support `divider=true` to draw a vertical line between the icon and label segments.

**Usage:**
```markdown
{{ui:tech:react:border=61DAFB:divider=true/}}                      <!-- Divider only -->
{{ui:tech:react:border=61DAFB:border_full=true:divider=true/}}     <!-- Full border + divider -->
```

**Border Hierarchy:**
| Mode | Perimeter Border | Center Divider |
|------|-----------------|----------------|
| `border=COLOR` | Left segment only | ❌ |
| `border=COLOR:border_full=true` | ✅ Full | ❌ |
| `border=COLOR:divider=true` | Left segment only | ✅ |
| `border=COLOR:border_full=true:divider=true` | ✅ Full | ✅ |
| `style=outline` | ✅ Full | ✅ (automatic) |

#### LSP Extension Installer

One-command setup for VS Code LSP integration:

```bash
mdfx lsp install                # Install VS Code extension automatically
mdfx lsp install --editor vscode  # Explicit editor flag
```

**Features:**
- Generates `package.json` and `extension.js` in `~/.vscode/extensions/mdfx-lsp/`
- Automatically runs `npm install` for dependencies
- Detects VS Code variants (`.vscode`, `.vscode-server`, `.vscode-insiders`)
- Configures extension with correct mdfx binary path
- Works with VS Code Remote and Devcontainers

**New CLI structure:**
```bash
mdfx lsp run      # Start LSP server (for editors)
mdfx lsp install  # Install editor extension
```

**Backward compatibility:** Editor configurations should update from `mdfx lsp` to `mdfx lsp run`.

#### Tech Badge IntelliSense (LSP)

The LSP server now provides full IntelliSense support for tech badges with contextual completions:

**Tech name completions** - After `{{ui:tech:` shows all 90+ available tech names with brand colors:
```markdown
{{ui:tech:|}}  <!-- rust, typescript, docker, kubernetes, etc. -->
```

**Parameter completions** - After the tech name, shows all available parameters:
```markdown
{{ui:tech:rust:|}}  <!-- border, logo_size, corners, chevron, style, etc. -->
```

**Value completions** - After `=`, shows valid values for each parameter:
```markdown
{{ui:tech:rust:logo_size=|}}  <!-- xs, sm, md, lg, xl, xxl -->
{{ui:tech:rust:corners=|}}    <!-- left, right, none, all -->
{{ui:tech:rust:style=|}}      <!-- flat, outline, ghost, plastic... -->
{{ui:tech:rust:bg=|}}         <!-- palette colors: accent, cobalt, plum... -->
```

**Supported parameter value completions:**
- `logo_size`/`icon_size`: xs (10px), sm (12px), md (14px), lg (16px), xl (18px), xxl (20px)
- `corners`: left, right, none, all
- `chevron`: left, right, both
- `style`: flat, flat-square, plastic, for-the-badge, social, outline, ghost
- `border_full`/`divider`: true, false
- `source`: shields
- Color params (`bg`, `bg_left`, `bg_right`, `logo`, `text`, `border`): All palette colors

### Changed

#### TechConfig Struct Refactor (Test Maintainability)

Refactored `Primitive::Tech` from a struct variant to a tuple variant wrapping the new `TechConfig` struct. This enables `..Default::default()` syntax in tests, making them resilient to new field additions.

**Before (fragile - every new field breaks all tests):**
```rust
let tech = Primitive::Tech {
    name: "rust".to_string(),
    bg_color: "000000".to_string(),
    logo_color: "FFFFFF".to_string(),
    style: "flat-square".to_string(),
    label: None,
    border_color: None,
    // ... 14 more fields that must be listed
};
```

**After (resilient to new fields):**
```rust
let tech = Primitive::Tech(TechConfig {
    name: "rust".to_string(),
    ..Default::default()
});

// Or use the helper:
let tech = Primitive::Tech(TechConfig::new("rust"));
```

**Benefits:**
- Adding new Tech fields only requires updating `TechConfig` and its `Default` impl
- Test code reduced by ~140 lines across 9 files
- Helper methods: `TechConfig::new(name)`, `TechConfig::with_colors(name, bg, logo)`

#### mdfx Tech Renderer Migration to badgefx

The mdfx tech badge renderer has been migrated to use the `badgefx` crate:

- **Single source of truth** - All tech badge rendering now goes through badgefx
- **Reduced code duplication** - 775 lines of inline SVG rendering replaced by thin wrapper
- **30 parity tests** - Comprehensive tests verify identical output to original renderer
- **Breaking change** - `render_with_options` now has an additional `raised: Option<u32>` parameter

The `tech.rs` module is now a lightweight wrapper that converts mdfx parameters to badgefx's `BadgeBuilder` API:

```rust
// Before: 850+ lines of inline SVG rendering
// After: ~150 lines delegating to badgefx
pub fn render_with_options(
    name: &str,
    label: Option<&str>,
    // ... 14 other parameters ...
    raised: Option<u32>,  // NEW
) -> String {
    BadgeBuilder::new(name)
        .label(label.unwrap_or(""))
        .bg_color(bg_color)
        // ... configure badge ...
        .render()
}
```

### Added

#### Raised Badge Feature

Tech badges now support the `raised` parameter for elevated icon styling:

```
{{ui:tech:rust:raised=4/}}
```

The raised badge has the icon section extending above and below the skinnier label section:

```
┌─────────┬────────────┐
│         │            │
│  ICON   │   label    │  <- label section is vertically centered
│         │            │
└─────────┴────────────┘
```

**Features:**
- Icon section height = label height + (raised × 2)
- Single background color for visual conformity
- Available via `raised=N` parameter (N pixels above/below)
- Also accessible in badgefx via `.raised(pixels)` builder method

#### badgefx Crate - Standalone Badge Rendering

Extracted badge rendering into a standalone crate for independent use:

```rust
use badgefx::{badge, BadgeStyle};

// Simple badge with defaults
let svg = badge("rust").render();

// Customized badge
let svg = badge("typescript")
    .label("TypeScript v5.0")
    .style(BadgeStyle::FlatSquare)
    .bg_color("#3178C6")
    .logo_size_lg()
    .render();
```

**Features:**
- **Builder pattern API** - Fluent interface for badge customization
- **5 badge styles** - Flat, FlatSquare, Plastic, ForTheBadge, Social
- **Logo size presets** - Xs (10px), Sm (12px), Md (14px), Lg (16px), Xl (18px), Xxl (20px), Custom
- **Independent segment colors** - `bg_left` and `bg_right` for two-tone badges
- **Chevron shapes** - Left, right, or both-sided arrow badges
- **Custom borders** - Color and width configuration
- **Custom corners** - Per-corner radius control
- **Custom icons** - Override built-in icons with SVG path data
- **Typography** - Custom fonts and text colors
- **Outline mode** - Border-only badges with transparent fill
- **Raised icons** - Icon positioned above/below the label baseline
- **Group rendering** - Automatic corner presets for seamless badge groups

**Modules:**
- `badge.rs` - Core `TechBadge` struct and `BadgeBuilder`
- `render.rs` - SVG generation with all rendering modes
- `style.rs` - `BadgeStyle`, `SvgMetrics`, `Border`, `Corners`, `Chevron`
- `shapes.rs` - SVG path generation utilities
- `group.rs` - Badge group layout and corner auto-configuration
- `glyphs.rs` - Optional Unicode glyph support (feature-gated)

#### mdfx-icons Crate - Icon Library

Lightweight crate for Simple Icons SVG paths and brand colors:

```rust
use mdfx_icons::{icon_path, brand_color, list_icons};

let rust_path = icon_path("rust").unwrap();
let rust_color = brand_color("rust"); // "DEA584"

for icon in list_icons() {
    println!("{}: #{}", icon, brand_color(icon).unwrap_or("unknown"));
}
```

**API:**
- `icon_path(name)` - Get SVG path data for 24x24 viewBox icons
- `brand_color(name)` - Get official hex color (without #)
- `list_icons()` - List all 18+ supported technologies

**Supported Icons:**
rust, typescript, javascript, python, react, docker, kubernetes, postgresql, go, nodejs, vue, svelte, terraform, aws, redis, mongodb, github, gitlab

#### mdfx-colors Crate - Color Utilities

Lightweight color manipulation for badge generation:

```rust
use mdfx_colors::{luminance, contrast_color, darken, parse_hex};

// Check if background is light or dark
let lum = luminance("#DEA584"); // ~0.55

// Get ideal text color for contrast
let text = contrast_color("#DEA584"); // "#000000" (black)

// Darken a color for borders
let border = darken("#DEA584", 0.15); // "#BC8C70"

// Parse hex colors
let (r, g, b) = parse_hex("#FF5500").unwrap();
```

**API:**
- `luminance(hex)` - ITU-R BT.709 relative luminance (0.0-1.0)
- `contrast_color(hex)` - Returns "#000000" or "#FFFFFF" for best contrast
- `darken(hex, amount)` - Reduce brightness by percentage (0.0-1.0)
- `parse_hex(hex)` - Parse 3 or 6 digit hex to RGB tuple

#### Raised Icon Badges

Icons can now be visually raised above or below the label baseline:

```markdown
{{ui:tech:rust:raised=2/}}
```

The `raised` parameter shifts the icon position vertically while keeping text centered.

#### Glyph Syntax in Labels

Use inline glyph syntax directly in badge labels:

```markdown
{{ui:tech:rust:label=Rust {{glyph:star}}/}}
```

Glyphs expand within labels for creative badge text.

#### Tech Badge Showcase Examples

New example files demonstrating tech badge capabilities:

- **neon-tech-showcase.md** - Full showcase with 71 SVG assets demonstrating all badge features
- **neon-tech-showcase-refactorv2.md** - Parity verification example using badgefx
- **tech-badges.md** - Creative combinations with text styles and glyphs

### Fixed

#### badgefx Parity Audit - 5 Critical Fixes

Comprehensive function-by-function audit of badgefx vs original mdfx tech renderer, ensuring byte-for-byte SVG output parity:

1. **estimate_text_width calculation**
   - Original: `(text.len() * 7).max(20)` - byte length × 7, minimum 20
   - badgefx had: `(text.chars().count() * 6.5) as u32` - char count × 6.5
   - For "rust": Original=28px, badgefx was 26px (2 pixel width difference per badge)

2. **darken_color return format**
   - Original returns: `XXXXXX` (no hash prefix)
   - mdfx_colors::darken returns: `#XXXXXX` (with hash prefix)
   - Caused double-hash bug: `fill="##XXXXXX"` in SVG output

3. **render_text_only text fill**
   - Original: hardcoded `fill="white"`
   - badgefx had: calculated text_color based on background

4. **render_icon_only background**
   - Original: single `<rect>` element with uniform corner radius
   - badgefx had: two overlapping `<rect>` elements

5. **render_text_only background**
   - Same issue as render_icon_only (two rects instead of one)

All fixes verified: 33 badgefx tests pass, 71 showcase assets produce identical hashes.

#### Path-Based Rendering Inner Border Fix

Fixed inconsistent behavior where path-based rendering (used when `rx` or `corners` parameters are specified) would incorrectly show an inner border at the segment boundary, while rect-based rendering did not.

**Before:** Using `rx=3` or custom `corners` caused a visible dividing line between icon and label segments (because the stroke was applied to all edges of the left path, including the inner edge).

**After:** Path-based rendering now uses the same 3-layer technique as rect-based rendering:
1. Full-width background path with border
2. Right segment overlay hides inner border portion
3. Corner patch restores rounded corners on right side

This ensures consistent behavior - the inner border only appears when explicitly requested via `divider=true`.

#### Outline/Ghost Style

Border-only badges with transparent fill for a sleek outline appearance:

```markdown
{{ui:tech:rust:style=outline/}}      <!-- Rust-colored border and icon -->
{{ui:tech:typescript:style=ghost/}}  <!-- "ghost" is an alias for outline -->
```

The outline style uses the brand color for:
- Border stroke (customizable with `border=` parameter)
- Icon color
- Label text color

Parameters:
- `style=outline` or `style=ghost` - Enable outline mode
- `border=COLOR` - Custom border color (defaults to brand color)
- `border_width=N` - Border thickness in pixels (default: 2)

#### Tech Group Component

Automatically apply corner presets for seamless badge groups:

```markdown
{{ui:tech-group}}
{{ui:tech:rust/}}
{{ui:tech:typescript/}}
{{ui:tech:docker/}}
{{/ui}}
```

The tech-group component automatically applies:
- `corners=left` to the first badge (rounded left, square right)
- `corners=none` to middle badges (all square)
- `corners=right` to the last badge (square left, rounded right)

This creates a seamless "pill" group when badges are placed side-by-side.

#### Chevron Badges

Tab-style badges with pointed arrow shapes:

```markdown
{{ui:tech:rust:chevron=right/}}      <!-- right arrow → -->
{{ui:tech:typescript:chevron=both/}} <!-- ← both arrows → -->
{{ui:tech:postgresql:chevron=left/}} <!-- ← left arrow -->
```

Chevron badges now render with proper two-color scheme (icon + label segments).

#### Independent Segment Colors

Control left (icon) and right (label) segment colors independently:

```markdown
{{ui:tech:rust:bg_left=DEA584:bg_right=B8856E/}}
```

Parameters:
- `bg_left` - Left segment (icon area) background color
- `bg_right` - Right segment (label area) background color

#### Custom SVG Icons

Provide custom SVG path data for technologies not in Simple Icons:

```markdown
{{ui:tech:mytech:icon=M12 2L2 7l10 5 10-5-10-5z:bg=4A90D9:label=Custom/}}
```

Parameters:
- `icon` - SVG path data (`d` attribute from a 24x24 viewBox SVG)

When using `icon`, the technology name is used only for the label text, not icon lookup.

#### Logo Size Presets

Control icon size with presets or custom pixel values:

```markdown
{{ui:tech:rust:logo_size=xs/}}   <!-- 10px - extra small -->
{{ui:tech:rust:logo_size=sm/}}   <!-- 12px - small -->
{{ui:tech:rust:logo_size=md/}}   <!-- 14px - medium (default) -->
{{ui:tech:rust:logo_size=lg/}}   <!-- 16px - large -->
{{ui:tech:rust:logo_size=xl/}}   <!-- 18px - extra large -->
{{ui:tech:rust:logo_size=20/}}   <!-- custom pixel size -->
```

Parameters:
- `logo_size` (alias: `icon_size`) - Size preset or pixel value

The badge width automatically adjusts based on the logo size.

## [1.0.0] - 2025-12-17

### Changed

#### SVG Backend Now Default

All targets now default to SVG backend for full-fidelity rendering:

- **GitHub, GitLab, npm** - Changed from shields.io to SVG
- **Full features everywhere** - Borders, corners, custom fonts work on all targets
- **Offline-first** - No external dependencies, works without internet
- **Tech badge source option** - Use `source=shields` for shields.io URLs on individual badges

**Before (shields.io default):**
```bash
mdfx process input.md --target github -o README.md
# Progress bars rendered as "75%-75%" text badges
```

**After (SVG default):**
```bash
mdfx process input.md --target github -o README.md --assets-dir assets
# Progress bars rendered as actual visual bars
```

**Per-badge shields.io option:**
```markdown
{{ui:tech:rust/}}                    <!-- SVG file (default) -->
{{ui:tech:rust:source=shields/}}     <!-- shields.io URL -->
```

#### Hybrid Backend Removed

The hybrid backend has been removed. Use `source=shields` on individual tech badges instead.

### Added

#### Incremental Asset Generation

- **Skip existing assets** - SVG backend now checks if files exist before writing
- **Hash-based filenames** - Same component parameters produce identical filenames, so existing files have correct content
- **Progress reporting** - CLI shows "N written, M unchanged" instead of listing every file
- **Faster rebuilds** - Repeated `mdfx process` runs skip all unchanged assets

**Before:**
```
Info: Writing 41 asset(s) to assets/
  Wrote: assets/swatch_abc123.svg
  ... (writes all files every time)
```

**After:**
```
Info: Assets: 41 unchanged (assets/)           # Second run
Info: Assets: 2 written, 39 unchanged (assets/)  # Only new assets
```

#### Intelligent Tech Badge Colors

- **Auto logo color** - Logo color automatically selects black or white based on background luminance
- **ITU-R BT.709 luminance** - Uses standard formula: `0.2126*R + 0.7152*G + 0.0722*B`
- **Light backgrounds** - Rust (orange), Go (cyan) get black logos for contrast
- **Dark backgrounds** - PostgreSQL (blue), Docker (blue) get white logos
- **Manual override** - Use `logo=000000` or `logo=FFFFFF` to override

**Usage:**
```markdown
{{ui:tech:rust/}}        <!-- Orange bg → black logo (automatic) -->
{{ui:tech:postgresql/}}  <!-- Blue bg → white logo (automatic) -->
{{ui:tech:go:logo=white/}}  <!-- Override: force white logo -->
```

#### Tech Badge Text Customization

- **Text color** - `text_color`, `text`, or `color` parameter for label color
- **Font family** - `font` or `font_family` parameter for custom fonts
- **Intelligent defaults** - Text color auto-selects based on right segment luminance

**Usage:**
```markdown
{{ui:tech:rust:text_color=white/}}           <!-- White text -->
{{ui:tech:python:font=Monaco,monospace/}}    <!-- Custom font -->
{{ui:tech:go:text=000000:font=Arial/}}       <!-- Both customized -->
```

#### Gauge Component

- **Semi-circular gauge meter** - Half-donut style visualization for dashboards
- **Syntax:** `{{ui:gauge:percent/}}` with optional parameters
- **Parameters:**
  - `size` - Width in pixels (default: 80)
  - `thickness` - Arc thickness in pixels (default: 8)
  - `track` - Track (background) color (default: slate)
  - `fill` - Fill (progress) color (default: accent)
  - `label` - Show percentage label (default: false)
  - `label_color` - Label text color (default: white)
- **SVG arc rendering** - Uses stroke-dasharray for smooth semi-circular arcs
- **All renderer backends** - SVG, shields.io fallback, plaintext fallback
- **Gallery examples** - Comprehensive gallery with size, color, and style variations

**Usage:**
```markdown
{{ui:gauge:75/}}
{{ui:gauge:50:size=120:thickness=12:fill=success/}}
{{ui:gauge:85:label=true/}}
```

**Use cases:** CPU/memory meters, speedometers, dashboard widgets, loading indicators

#### Thumb Support for Donut and Gauge

- **Slider mode** - Both donut and gauge components now support `thumb` parameter
- **Visual indicator** - Circular thumb positioned at fill endpoint
- **Customizable color** - Use `thumb_color` to override the default fill color
- **Consistent with progress bar** - Same slider pattern as `{{ui:progress:75:thumb=12/}}`

**Usage:**
```markdown
{{ui:donut:75:thumb=12/}}
{{ui:gauge:50:thumb=14:thumb_color=accent/}}
```

#### Custom Thumb Width for Progress Bar

- **Oval/pill-shaped thumbs** - Progress bar sliders now support `thumb_width` parameter
- **Independent width control** - Set thumb width separately from height for non-circular shapes
- **Ellipse rendering** - Circle shape becomes ellipse when width differs from height

**Usage:**
```markdown
{{ui:progress:50:thumb=12:thumb_width=20/}}
{{ui:progress:75:thumb=10:thumb_width=24:thumb_color=accent/}}
```

#### Enhanced Asset Cleanup

- **Markdown scanning** - `mdfx clean --scan` now scans markdown files to find actually referenced assets
- **Automatic cleanup** - Removes orphaned assets not referenced in any markdown file
- **Manifest update** - Automatically updates manifest.json after cleaning
- **Dry-run preview** - Use `--dry-run` to see what would be deleted before committing

**Usage:**
```bash
mdfx clean --assets-dir examples/assets --scan "examples/*-rendered.md" --dry-run
mdfx clean --assets-dir docs/assets --scan "docs/**/*.md"
```

#### Template Partials

- **User-defined reusable templates** - Define partials in `.mdfx.json` config file
- **Content substitution** - Use `$1` or `$content` placeholders for dynamic content
- **Project-wide consistency** - Share styles across all markdown files in a project
- **Auto-discovery** - CLI automatically finds `.mdfx.json` in current or parent directories

**Configuration (`.mdfx.json`):**
```json
{
  "partials": {
    "hero": {
      "template": "{{frame:gradient}}{{mathbold}}$1{{/mathbold}}{{/frame}}",
      "description": "Hero header with gradient frame"
    },
    "techstack": {
      "template": "{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}}"
    },
    "warning-box": {
      "template": "{{frame:solid-left}}⚠️ $content{{/frame}}"
    }
  },
  "palette": {
    "brand": "FF5500"
  }
}
```

**Usage in markdown:**
```markdown
{{partial:hero}}MY TITLE{{/partial}}
{{partial:techstack/}}
{{partial:warning-box}}Careful here{{/partial}}
```

**CLI:**
```bash
# Auto-discover .mdfx.json
mdfx process input.md -o output.md

# Explicit config path
mdfx process input.md --config project.mdfx.json -o output.md
```

#### Enhanced Swatch Primitives (SVG-only)

- **Flip any frame** - Swap prefix and suffix of any frame with `/reverse` modifier
- **Alias support** - `/rev` also works
- **Works with all frames** - Predefined frames, glyph frames, and frame combos

**Usage:**
```markdown
{{fr:gradient/reverse}}Title{{/}}          → ░▒▓ Title ▓▒░
{{fr:star/rev}}VIP{{/}}                    → ☆ VIP ★
{{fr:diamond/reverse}}Premium{{/}}         → ◇ Premium ◆
```

#### Frame Count Multiplier (`*N`)

- **Repeat predefined frames** - Use `*N` to repeat frame patterns N times
- **Works with all predefined frames** - star, gradient, diamond, etc.
- **Combines with other modifiers** - Works with separator, spacing, and reverse
- **Max count 20** - Capped to prevent abuse

**Usage:**
```markdown
{{fr:star*3}}Title{{/}}                    → ★★★ Title ☆☆☆
{{fr:gradient*2}}X{{/}}                    → ▓▒░▓▒░ X ░▒▓░▒▓
{{fr:diamond*5}}Premium{{/}}               → ◆◆◆◆◆ Premium ◇◇◇◇◇
{{fr:star*3/separator=·}}Title{{/}}        → ★·★·★ Title ☆·☆·☆
{{fr:star*2/reverse}}VIP{{/}}              → ☆☆ VIP ★★
```

#### Hybrid Backend (`--backend hybrid`)

- **Auto-selection** - Automatically chooses between shields.io and SVG based on feature usage
- **Best of both worlds** - Uses shields.io for simple badges (fast, no files), SVG only when needed
- **Smart detection** - Triggers SVG for: gradients, shadows, rx/ry, stroke_dash, per-side borders
- **Seamless mixing** - Same document can have both shields.io URLs and SVG files

**Usage:**
```bash
mdfx process template.md --backend hybrid --assets-dir assets
```

**When shields.io is used:**
```markdown
{{ui:swatch:accent/}}                    <!-- Simple color → shields.io -->
{{ui:swatch:accent:label=Badge/}}        <!-- With label → shields.io -->
{{ui:swatch:000000:icon=rust/}}          <!-- With icon → shields.io -->
```

**When SVG is used:**
```markdown
{{ui:swatch:accent:gradient=horizontal/FF0000/0000FF/}}  <!-- Gradient → SVG -->
{{ui:swatch:accent:shadow=000000/4/2/2/}}                <!-- Shadow → SVG -->
{{ui:swatch:accent:rx=10/}}                              <!-- Custom corners → SVG -->
{{ui:swatch:accent:border_bottom=F41C80/3/}}             <!-- Per-side border → SVG -->
```

#### Per-Side Border Control (SVG Backend)

- **Independent borders** - Control each side separately with `border_top`, `border_right`, `border_bottom`, `border_left`
- **Format** - "color/width" (e.g., `FF0000/3`) or just "color" (defaults to width 2)
- **CSS-like styling** - Create underlines, accent bars, multi-colored frames

**Usage:**
```markdown
{{ui:swatch:333333:width=100:height=40:border_top=3B82F6/3/}}      <!-- Top border only -->
{{ui:swatch:333333:width=100:height=40:border_bottom=22C55E/3/}}   <!-- Underline effect -->
{{ui:swatch:333333:width=100:height=40:border_left=F41C80/4:border_right=F41C80/4/}}  <!-- Side accents -->

<!-- All sides different colors -->
{{ui:swatch:1a1a1a:width=120:height=60:border_top=EF4444/3:border_right=F59E0B/3:border_bottom=22C55E/3:border_left=3B82F6/3/}}
```

#### Frame Namespace Shorthand (`{{fr:*}}`)

- **Shorter syntax** - Use `{{fr:}}` instead of `{{frame:}}`
- **Full feature parity** - Works with all frames, glyph frames, nesting
- **Less typing** - Saves 4 characters per frame tag

**Usage:**
```markdown
{{fr:gradient}}Title{{/}}                   <!-- Same as {{frame:gradient}} -->
{{fr:glyph:star*3}}Text{{/}}                <!-- Glyph frames work too -->
{{fr:gradient}}{{fr:star}}Nested{{/}}{{/}}  <!-- Nesting supported -->
```

#### Short Close Tag (`{{/}}`) and Close-All (`{{//}}`)

- **Universal closer** - Use `{{/}}` instead of `{{/frame}}` or `{{/ui}}`
- **Close-all** - Use `{{//}}` to close all open tags at once (frames, styles, UI components)
- **Cleaner syntax** - Reduces verbosity for deeply nested structures
- **Backward compatible** - Full tag names (`{{/frame}}`, `{{/ui}}`) still work
- **LIFO order** - Tags are closed in reverse order of opening

**Usage:**
```markdown
{{fr:gradient}}Title{{/}}                     <!-- Same as {{/frame}} -->
{{fr:gradient}}{{fr:star}}Nested{{//}}        <!-- Close-all: frames -->
{{fr:gradient}}{{mathbold}}Title{{//}}        <!-- Close-all: frame + style -->
{{fr:a}}{{fr:b}}{{fr:c}}Deep{{//}}            <!-- Works at any depth -->
{{ui:header}}Content{{/}}                     <!-- Same as {{/ui}} -->
```

#### Glyph Frame Shorthand (`{{frame:glyph:NAME}}`)

- **Dynamic frames** - Use any registered glyph as frame decoration
- **Multiplier** - Repeat glyphs with `*N` syntax (e.g., `glyph:star*3` → `★★★`)
- **Padding control** - Use `/pad=VALUE` to control spacing:
  - Numeric = spaces (`/pad=3` → 3 spaces)
  - String = literal (`/pad=·` → custom character)
  - Zero = tight (`/pad=0` → no spacing)
- **Max count** - Capped at 20 to prevent abuse

**Usage:**
```markdown
{{frame:glyph:star}}Title{{/}}                <!-- ★ Title ★ -->
{{frame:glyph:star*3}}Title{{/}}              <!-- ★★★ Title ★★★ -->
{{frame:glyph:star*3/pad=0}}Title{{/}}        <!-- ★★★Title★★★ -->
{{frame:glyph:diamond*2/pad=·}}Gem{{/}}       <!-- ◆◆·Gem·◆◆ -->
```

#### Self-Closing Frames (`{{fr:STYLE:CONTENT/}}`)

- **Inline syntax** - Compact form for short frame content
- **Less typing** - `{{fr:gradient:Title/}}` instead of `{{fr:gradient}}Title{{/}}`
- **Glyph support** - Works with glyph frames: `{{fr:glyph:star*3:VIP/}}`
- **Last colon splits** - Content is everything after the last `:` (handles glyph modifiers correctly)

**Usage:**
```markdown
{{fr:gradient:Title/}}                 <!-- ▓▒░ Title ░▒▓ -->
{{fr:star:VIP/}}                       <!-- ★ VIP ☆ -->
{{fr:glyph:diamond*2:Gem/}}            <!-- ◆◆ Gem ◆◆ -->
{{fr:glyph:star*3/pad=0:Tight/}}       <!-- ★★★Tight★★★ -->
```

### Changed

- **Frames consolidated** - Removed redundant `FrameRenderer` module, frames now handled entirely by `Registry`
- **Badge templates removed** - `{{badge:*}}` templates were never implemented; use glyphs (`{{glyph:circle.1}}` → ①) or styles (`{{circled-latin}}`) instead

#### Keyboard Keys (`{{kbd:...}}`)

- **Native HTML kbd tags** - GitHub renders `<kbd>` tags with keyboard styling
- **Compound key support** - `{{kbd:Ctrl+C/}}` expands to `<kbd>Ctrl</kbd>+<kbd>C</kbd>`
- **Top-level template** - No `ui:` prefix needed for cleaner syntax
- **Unicode support** - Works with Mac symbols: `{{kbd:⌘+C/}}`

**Examples:**
```markdown
Press {{kbd:Enter/}} to continue
Copy with {{kbd:Ctrl+C/}} or {{kbd:⌘+C/}}
Open command palette: {{kbd:Ctrl+Shift+P/}}
```

#### New Text Styles (4 additions, 23 total)

- **subscript** - Small lowered text for chemistry and math: H₂O, x₁ (aliases: `sub`)
- **superscript** - Small raised text for exponents and ordinals: x², 1ˢᵗ (aliases: `sup`, `super`)
- **parenthesized** - Letters in parentheses: ⒜⒝⒞ (aliases: `paren`, `parens`)
- **inverted** - Upside-down text for fun: ʇxǝʇ uʍop ǝpᴉsd∩ (aliases: `upsidedown`, `flip`, `flipped`)

**Examples:**
```markdown
{{subscript}}H2O{{/subscript}}        → H₂O
{{superscript}}x2{{/superscript}}     → x²
{{parenthesized}}abc{{/parenthesized}} → ⒜⒝⒞
{{inverted}}Hello{{/inverted}}        → Hǝllo
```

#### Watch Mode (`mdfx watch`)

- **Live rebuilding** - Monitor input file and automatically rebuild on changes
- **Debounce support** - Configurable delay to avoid rapid rebuilds (default: 100ms)
- **Full feature parity** - Supports all process options: `--target`, `--backend`, `--assets-dir`, `--palette`

**Usage:**
```bash
mdfx watch input.md -o output.md
mdfx watch README.template.md -o README.md --target github
mdfx watch docs/source.md -o docs/rendered.md --backend svg --debounce 200
```

#### LSP Server (Language Server Protocol)

- **Universal IDE support** - Autocompletion in any editor supporting LSP
- **Optional feature** - Enable with `cargo install mdfx-cli --features lsp` to avoid dependency bloat
- **Comprehensive completions**:
  - 493 glyphs with hierarchical namespacing
  - 19 text styles with aliases
  - 32 frames
  - 15 palette colors
  - 5 shield styles
  - 8 UI components
- **Hover documentation** - View glyph info and style descriptions
- **Context-aware** - Detects `{{glyph:`, `{{frame:`, `style=`, etc.

**Usage:**
```bash
cargo install mdfx-cli --features lsp
mdfx lsp  # Starts LSP server on stdio
```

#### VS Code Extension

- **Full extension package** - Ready for VS Code Marketplace publishing
- **LSP client integration** - Connects to mdfx language server
- **TextMate syntax highlighting** - Injection grammar for markdown files
  - Template delimiters (`{{` and `}}`)
  - Tag names (glyph, frame, styles, components)
  - Parameters and values
  - Self-closing syntax (`/}}`)
- **Configuration options** - Enable/disable, custom server path

**Files:** `editors/vscode/`

#### New Glyphs (104 additions, 493 total)

- **currency.\*** (10): dollar, euro, pound, yen, bitcoin, cent, rupee, won, franc, lira
- **greek.\*** (48): Full Greek alphabet (α-ω, Α-Ω) - `greek.alpha` through `greek.omega`
- **frac.\*** (16): Common fractions - `frac.half`, `frac.third`, `frac.quarter`, etc.
- **misc.\*** (30): Warning symbols, weather, zodiac, and utility characters

#### PlainTextBackend

- **New rendering backend** for PyPI and ASCII-only contexts
- Renders primitives as plain ASCII text:
  - Swatches: `[#RRGGBB]` or `[#RRGGBB label]`
  - Tech badges: `[rust]`, `[python]`
  - Status indicators: `[OK]`, `[WARN]`, `[ERR]`, `[INFO]`
- Handles both semantic names and resolved hex colors for status
- 9 unit tests for plain text rendering

#### Multi-Target Build Command

- **`mdfx build`** - Compile markdown to multiple targets at once
- **`--all-targets`** flag - Build for all 5 available targets
- **`--targets`** flag - Selective builds (e.g., `--targets github,pypi,npm`)
- **Per-target assets** - SVG assets organized by target (`dist/assets/local/`)
- **Custom palette support** - `--palette` flag for build command

**Usage:**
```bash
mdfx build README.template.md --all-targets           # All 5 targets
mdfx build README.template.md --targets github,pypi   # Selective
mdfx build README.template.md -o dist/                # Custom output dir
```

**Output:**
```
dist/readme_github.md
dist/readme_pypi.md
dist/readme_local.md + dist/assets/local/*.svg
dist/readme_gitlab.md
dist/readme_npm.md
```

#### GitLabTarget

- **New target** for GitLab-Flavored Markdown
- More permissive HTML support than GitHub
- Embedded SVG support
- **Post-processing** - Converts callouts to GitLab alert syntax
- Shields.io backend (default)

#### PyPITarget

- **New target** for PyPI package descriptions
- Plain text fallbacks for maximum compatibility
- ASCII-safe rendering (no Unicode styling by default)
- 80-character line limit recommendation
- **Post-processing** - Converts Unicode to ASCII equivalents:
  - Arrows: `→` → `->`, `←` → `<-`
  - Box drawing: `─` → `-`, `│` → `|`
  - Gradient chars: `▓` → `#`, `▒` → `=`, `░` → `-`
  - Status emoji: `🟢` → `[OK]`, `🟡` → `[WARN]`, `🔴` → `[ERR]`

#### Target Pipeline Integration

- **`target.preferred_backend()`** now properly selects backend (Shields/SVG/PlainText)
- **`target.post_process()`** called on final output for target-specific transformations
- Full backend selection wired into CLI process pipeline

### Changed

- **Test count** - Increased from 266 to 276 tests
- **PyPITarget** - Now uses PlainTextBackend instead of falling back to Shields

#### Enhanced Swatch Primitives

- **Opacity control** - `{{ui:swatch:accent:opacity=0.5/}}` for transparent swatches (SVG-only)
- **Custom dimensions** - `{{ui:swatch:accent:width=40:height=30/}}` for non-standard sizes (SVG-only)
- **Border support** - `{{ui:swatch:accent:border=white:border_width=2/}}` for outlined swatches (SVG-only)
- **Labels** - `{{ui:swatch:accent:label=v1/}}` for text overlay on swatches (Shields.io + SVG)
- **Label color** - `{{ui:swatch:white:label=X:label_color=000000/}}` for custom text color (SVG-only)
- **Icon support** - `{{ui:swatch:F41C80:icon=rust/}}` for Simple Icons logos (All backends)
- **Icon color** - `{{ui:swatch:accent:icon=docker:icon_color=white/}}` for custom icon color (All backends)

**Usage:**
```markdown
{{ui:swatch:F41C80:opacity=0.7/}}
{{ui:swatch:accent:width=50:height=25:border=FFFFFF/}}
{{ui:swatch:cobalt:label=API/}}
{{ui:swatch:FFFFFF:label=X:label_color=000000/}}
{{ui:swatch:22C55E:icon=rust/}}
{{ui:swatch:accent:icon=docker:icon_color=white/}}
```

**Backend support:**
| Option | Shields.io | SVG | PlainText |
|--------|------------|-----|-----------|
| opacity, width, height, border | ❌ | ✅ | ❌ |
| label | ✅ | ✅ | ✅ |
| label_color | ❌ | ✅ | ❌ |
| icon, icon_color | ✅ (real logos) | ⚠️ (text fallback) | ✅ |

#### Custom Palette Support

- **`--palette` CLI flag** - Load custom color definitions from JSON file
- **Override or extend** - Custom colors override built-in palette or add new names
- **Reusable branding** - Define brand colors once, use throughout documents

**Usage:**
```bash
# Create palette file (palette.json):
{
  "brand": "FF5500",
  "primary": "2B6CB0",
  "secondary": "48BB78"
}

# Use with mdfx:
mdfx process --palette palette.json input.md
```

Then in your markdown:
```markdown
{{ui:swatch:brand/}}
{{ui:swatch:primary/}}
```

#### CLI Target Flag

- **`--target` flag** for multi-platform rendering:
  - `github` (default) - shields.io badges for GitHub READMEs
  - `local` - SVG files for offline documentation
  - `npm` - shields.io badges for npm packages
  - `auto` - auto-detect from output path
- **Optional `--backend` flag** - targets now provide preferred backends automatically
  - GitHubTarget → Shields backend
  - LocalDocsTarget → SVG backend
  - NpmTarget → Shields backend
- **Auto-detection** - `--target auto` detects from output path (e.g., `docs/` → local, `npm/` → npm)

**Usage:**
```bash
mdfx process --target github input.md       # Default (shields.io)
mdfx process --target local input.md        # SVG files
mdfx process --target auto -o docs/README.md  # Auto-detect
mdfx process --target github --backend svg input.md  # Override backend
```

### Removed

- **Divider component** - Removed `{{ui:divider/}}` component entirely. It was just 4 colored swatches without distinct value. Users can achieve similar effects with multiple `{{ui:swatch:color/}}` components if needed.
- **Section divider** - The `{{ui:section:Title/}}` component now expands to just `## Title` without the divider line

### Changed

#### Data Consolidation

- **Removed redundant JSON files** - Deleted 7 separate data files in favor of single `registry.json`:
  - Removed: `badges.json`, `frames.json`, `styles.json`, `palette.json`, `separators.json`, `shields.json`, `components.json`
- **Updated all modules to use registry.json directly**:
  - `shields.rs` - Extracts `palette` and `shield_styles` from registry root
  - `components.rs` - Extracts `palette` and `renderables.components`
  - `separators.rs` - Extracts `renderables.glyphs` and converts to separator format
  - `frames.rs` - Extracts `renderables.frames` with ID/name derived from keys
  - `badges.rs` - Uses existing `renderables.badges`
  - `styles.rs` - Uses existing `renderables.styles`
- **Cleaner frame definitions** - Frame entries in registry.json no longer require redundant `id` and `name` fields; these are now derived from the HashMap key
- **Updated documentation** - Removed migrate tool references; updated architecture docs to reflect single data source

### Added

#### Unified Registry System

- **Single source of truth** - Consolidated 7 separate JSON data files into unified `registry.json` (1139 lines)
  - Replaces: `styles.json`, `frames.json`, `badges.json`, `palette.json`, `components.json`, `shields.json`, `separators.json`
  - Enables IntelliSense tooling with single schema (#1 priority)
- **Registry module** - New `registry.rs` with complete typed API
  - `Registry::new()` - Load and validate unified registry
  - `resolve()` - Unified resolution: glyphs → snippets → components → literal
  - Lookup methods: `glyph()`, `snippet()`, `component()`, `frame()`, `style()`, `badge()`, `shield_style()`
  - Color resolution: `resolve_color()` with palette support
  - Alias support for all renderable types
- **EvalContext system** - Context-aware validation for safe composition
  - Three contexts: `Inline`, `Block`, `FrameChrome`
  - Context promotion rules (Inline can promote to Block/FrameChrome)
  - All renderables annotated with allowed contexts
  - Runtime validation prevents invalid compositions
- **Renderables taxonomy**:
  - **Glyphs** (21): Unicode character mappings (`dot`, `bullet`, `arrow`, etc.)
  - **Snippets** (10): Template expansion shortcuts (`sep.accent`, `header.bold`, etc.)
  - **Components** (8): Semantic UI elements (`swatch`, `tech`, `status`, `header`, etc.)
  - **Frames** (32): Decorative wrappers (`gradient`, `solid-left`, `dashed`, etc.)
  - **Styles** (19): Character transformations (`mathbold`, `italic`, `monospace`, etc.)
  - **Badges** (6): Character modifiers (`circle`, `paren`, `period`, etc.)
- **Shield styles** - 5 visual styles with default marking (`flat`, `flat-square`, `for-the-badge`, `plastic`, `social`)

#### Target Trait (Multi-Surface Rendering)

- **Target abstraction** - Trait for rendering destination capabilities
  - `supports_html()`, `supports_svg_embed()`, `supports_external_images()`
  - `max_line_length()`, `preferred_backend()`, `supports_unicode_styling()`
  - `post_process()` - Platform-specific markdown transformations
- **BackendType enum** - `Shields`, `Svg`, `PlainText` with derived Default
- **Shipped targets**:
  - **GitHubTarget** - shields.io badges, no HTML, 100 char line limit
  - **LocalDocsTarget** - SVG files, offline-first, unlimited line length
  - **NpmTarget** - Similar constraints to GitHub
- **Target detection** - `detect_target_from_path()` for automatic target selection
- **Future targets** (v2.0): `GitLabTarget`, `PyPITarget`

#### Data-Driven Separator System

- **12 named separators** - Predefined separators with documentation: `dot`, `bullet`, `dash`, `bolddash`, `arrow`, `star`, `diamond`, `square`, `circle`, `pipe`, `slash`, `tilde`
- **Direct Unicode separator support** - Use any single grapheme cluster as separator: `{{mathbold:separator=⚡}}TEXT{{/mathbold}}`
- **Grapheme cluster support** - Properly handles emoji with variation selectors (👨‍💻), flag emoji (🇺🇸), and composed characters
- **Validation & normalization** - Automatic whitespace trimming, template delimiter rejection (`:`, `/`, `}`), empty input prevention
- **Enhanced error messages** - "Did you mean" suggestions for typos, lists all available separators
- **New module: `separators.rs`** - Loads `data/separators.json` with lazy_static
- **New CLI command: `mdfx separators`** - List all available separators with `--examples` flag
- **New dependency: `unicode-segmentation`** - Proper grapheme cluster counting

#### Asset Manifest System

- **AssetManifest** - Tracks all generated SVG assets with metadata
- **Automatic manifest generation** - `manifest.json` written alongside assets when using `--backend svg`
- **SHA-256 hashing** - Content-based verification for each asset
- **Primitive tracking** - Full primitive parameters preserved in manifest for reproducibility
- **RFC 3339 timestamps** - Build time tracking
- **New module: `manifest.rs`** - Complete manifest API with verification support
- **New dependencies:** `sha2` (hashing), `chrono` (timestamps)

**Manifest Format:**
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
      "sha256": "2c932535cd177cd4...",
      "type": "swatch",
      "primitive": { "kind": "Swatch", "color": "f41c80", "style": "flat-square" },
      "size_bytes": 143
    }
  ]
}
```

#### Documentation

- **SECURITY.md** - Vulnerability reporting process, security best practices, coordinated disclosure policy
- **CONTRIBUTING.md** - Development setup, code style, commit conventions, PR process, project structure guide
- **CODE_OF_CONDUCT.md** - Contributor Covenant 2.1
- **ROADMAP.md** - v1.1-v1.3 feature planning with priority matrix
- **Comprehensive documentation hub** - Restructured docs/README.md with organized navigation similar to dotclaude
- **examples/github-blocks.md** - Complete gallery of GitHub block components with rendered output

### Changed

- **Parser refactored to unified resolution** - All validation now flows through Registry
  - Style validation via `registry.style()` instead of `converter.has_style()`
  - Frame validation via `registry.frame()` instead of `frame_renderer.has_frame()`
  - Badge validation via `registry.badge()` instead of `badge_renderer.has_badge()`
  - Separator resolution via `registry.glyph()` with EvalContext checking
  - Removed lazy_static SEPARATORS dependency from parser
  - Improved error messages with available glyph suggestions
- **BackendType enum** - Now uses `#[derive(Default)]` with `#[default]` attribute (clippy optimization)
- **Project renamed from utf8fx to mdfx** - The new name better reflects the tool's focus on markdown enhancement. "mdfx" (markdown effects) is more descriptive than "utf8fx" for a tool specifically designed to transform markdown with Unicode styling and UI components.
- **Dependency count increased** - From 4 to 7: added unicode-segmentation, sha2, chrono
- **RenderedAsset::File variant** - Now includes `primitive` field for manifest tracking
- **Documentation theme** - Changed from pink (#f41c80) to blue (#4a9eff) to match Blackwell Systems branding
- **Test count** - Increased from 189 to 256 tests (21 new tests for GitHub blocks and whitespace handling, 20 new tests for badge style control, 8 new tests for Target trait, 8 new tests for Registry)
- **PostProcess enum** - Now derives `Default` (clippy optimization)

### Fixed

- **Separator resolution** - Now data-driven via separators.json instead of hardcoded match statement
- **Test coverage** - Fixed obsolete test in renderer/mod.rs for new RenderedAsset structure
- **Trailing newline preservation** - Parser now correctly preserves trailing newlines after components
- **Whitespace handling** - Fixed edge cases in component expansion (empty lines, indentation, list contexts)

### Initial Release Foundation

#### Badge Style Control

- **5 visual styles** for all primitive components (divider, swatch, tech, status):
  - **`flat`** - Rounded corners (rx=3) for friendly appearance
  - **`flat-square`** - Sharp corners (rx=0), default, modern look
  - **`for-the-badge`** - Tall blocks (height=28) for prominence
  - **`plastic`** - Shiny gradient overlay for retro 3D effect
  - **`social`** - Very rounded (rx=10) for social media style
- **Syntax:** `{{ui:swatch:COLOR:style=STYLE/}}` - Optional style parameter
- **Default behavior:** Omitting `style=` uses `flat-square` (backward compatible)
- **Cross-backend support:** Works with both shields.io URLs and local SVG generation
- **SVG metrics system:** Style-aware rendering (rx, height, gradient overlays)
- **Deterministic hashing:** Different styles produce unique filenames
- **Mix and match:** Compose blocks with different styles ("Minecraft bricks")
- **20 new tests:** 11 shields.io style passthrough, 8 SVG rendering, 1 alias update

**Usage:**
```markdown
{{ui:swatch:F41C80:style=flat/}}              ← Rounded
{{ui:swatch:F41C80:style=for-the-badge/}}     ← Tall
{{ui:divider:style=plastic/}}                 ← Shiny divider
```

**Design flexibility:** Create visual variety by mixing styles in compositions:
```markdown
{{ui:swatch:FF0000:style=flat/}}{{ui:swatch:FF0000:style=flat-square/}}{{ui:swatch:FF0000:style=for-the-badge/}}
```

**Rationale:** Users need different "brick types" for creative design compositions. Like Minecraft blocks, having variety in shape (rounded, sharp, tall) and texture (shiny, flat) enables richer visual storytelling in READMEs.

#### GitHub Blocks

- **Three new components** optimized for GitHub READMEs:
  - **`section`** - Section header with automatic divider (`{{ui:section:Title/}}`)
  - **`callout-github`** - Blockquote-style callouts with status indicators (`{{ui:callout-github:TYPE}}CONTENT{{/ui}}`)
  - **`statusitem`** - Inline status badges (`{{ui:statusitem:Label:Level:Text/}}`)
- **Blockquote post-processor** - New `PostProcess` enum with `Blockquote` variant
- **Multiline blockquote support** - Automatically prefixes every line with `"> "` for GitHub compatibility
- **Empty line handling** - Empty lines in callouts rendered as `">"` (no trailing space)
- **Composable status rows** - Manually compose multiple status items with ` · ` separator
- **Four callout types** - `success` (green), `info` (blue), `warning` (yellow), `error` (red)
- **Example gallery** - `examples/github-blocks.md` demonstrating all GitHub block components
- **Golden tests** - 11 comprehensive integration tests for GitHub blocks
- **Whitespace preservation tests** - 10 tests ensuring document structure integrity

**Usage:**
```markdown
{{ui:section:Features/}}

{{ui:callout-github:warning}}
Breaking changes in v2.0!
{{/ui}}

{{ui:statusitem:Build:success:passing/}} · {{ui:statusitem:Tests:success:237/}}
```

**Rationale:** GitHub's Markdown renderer has strict constraints (no custom HTML/CSS). These components work within those constraints by using blockquotes and shields.io badges, ensuring READMEs look professional while remaining portable.

#### Component-First Architecture

**Major architectural shift to semantic UI components:**

- **ComponentsRenderer** - New primary API for high-level semantic elements
  - 8 UI components shipped: `divider`, `swatch`, `tech`, `status`, `header`, `callout`, `section`, `callout-github`, `statusitem`
  - Expansion model: components expand to primitives (data-driven, not code)
  - Design token integration: named colors resolve from palette.json
  - Template syntax: `{{ui:component/}}` (self-closing) or `{{ui:component}}content{{/ui}}` (block)
  - Generic `{{/ui}}` closer for ergonomics
  - API: `ComponentsRenderer::expand(name, args, content)`

- **ShieldsRenderer** - Generate shields.io badge URLs as Markdown images
  - 4 primitives: `block` (single color), `twotone` (split), `bar` (multiple), `icon` (logo)
  - Template syntax: `{{shields:type:color=...:style=.../}}`
  - Integration with Simple Icons (2000+ logos)
  - Color resolution: palette name or 6-digit hex
  - API: `render_block()`, `render_twotone()`, `render_bar()`, `render_icon()`

**Design Token System:**

- **palette.json** - 15 named colors for consistent branding
  - Theme colors: `accent`, `success`, `warning`, `error`, `info`
  - UI colors: `ui.bg`, `ui.surface`, `ui.panel` (dark theme)
  - Utility colors: `slate`, `white`, `black`, `ink`, `cobalt`, `plum`
  - Dot notation support for namespacing (`ui.bg`)

- **components.json** - Component definitions (expand-based)
  - Each component specifies `template` string with `$1`, `$content` substitution
  - `self_closing` flag determines syntax
  - `args` list documents parameters
  - Future support for `native` type (Rust-implemented logic)

**Multi-Backend Rendering Architecture:**

- **Primitive enum** - Backend-neutral representation of visual elements
  - `Swatch`, `Divider`, `Tech`, `Status` primitives
  - Semantic intent (tech badge) not implementation (shields URL)
  - Type-safe parameters enforced at compile time

- **Renderer trait** - Pluggable backend system
  - Common interface: `fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>`
  - Allows multiple output formats (shields.io URLs, local SVG files, etc.)
  - Backend selection via CLI flag: `--backend shields|svg`

- **ComponentOutput enum** - Dual expansion mode
  - `Primitive` for image-based components (divider, swatch, tech, status)
  - `Template` for text-effect components (header, callout, section, callout-github, statusitem)
  - Parser automatically routes to correct rendering path

- **ShieldsBackend** - Default renderer (shipped in v1.0.0)
  - Implements `Renderer` trait for shields.io
  - Maps primitives to ShieldsRenderer methods
  - Returns inline Markdown: `InlineMarkdown("![](https://...)")`

- **SvgBackend** - Local SVG file generation (shipped in v1.0.0)
  - Generates deterministic hash-based filenames
  - Returns file assets: `File { relative_path, bytes, markdown_ref, primitive }`
  - Supports all primitives: Swatch, Divider, Tech, Status
  - CLI usage: `mdfx process --backend svg --assets-dir assets/mdfx input.md`

#### New Components

**Visual Elements:**

- `divider` - 4-color themed bar for section separation
  - Self-closing: `{{ui:divider/}}`
  - Expands to `{{shields:bar}}` with theme colors

- `swatch` - Single colored block
  - Usage: `{{ui:swatch:accent/}}`
  - Supports palette colors or direct hex

- `status` - Colored status indicator
  - Usage: `{{ui:status:success/}}`
  - Common levels: success (green), warning (yellow), error (red), info (blue)

**Tech Stack:**

- `tech` - Technology logo badge using Simple Icons
  - Usage: `{{ui:tech:rust/}}`
  - 2000+ logos available (rust, python, postgresql, docker, kubernetes, etc.)
  - Renders as shields.io badge with logo

**Content Blocks:**

- `header` - Gradient-framed bold header with dot separators
  - Usage: `{{ui:header}}TITLE{{/ui}}`
  - Expands to `{{frame:gradient}}{{mathbold:separator=dot}}$content{{/mathbold}}{{/frame}}`
  - Output: ▓▒░ 𝐓·𝐈·𝐓·𝐋·𝐄 ░▒▓

- `callout` - Framed message with colored indicator
  - Usage: `{{ui:callout:warning}}Message{{/ui}}`
  - Expands to `{{frame:solid-left}}{{shields:block:color=$1}}} $content{{/frame}}`
  - Use cases: warnings, important notes, alerts

#### Template Syntax Extensions

**Self-closing tags:**
```markdown
{{ui:divider/}}
{{ui:tech:rust/}}
{{ui:swatch:accent/}}
```

**Block tags with generic closer:**
```markdown
{{ui:header}}TITLE{{/ui}}
{{ui:callout:warning}}Message{{/ui}}
```

**Primitives (escape hatch):**
```markdown
{{shields:block:color=accent:style=flat-square/}}
{{frame:gradient}}TEXT{{/frame}}
{{badge:circle}}1{{/badge}}
```

#### Library API Additions

**ComponentsRenderer:**
- `ComponentsRenderer::new()` - Load components.json and palette.json
- `expand(component, args, content)` - Expand component to primitive template
- `has(name)` - Check if component exists
- `list()` - Query all components
- `list_palette()` - Query all palette colors
- `get(name)` - Get component definition

**ShieldsRenderer:**
- `ShieldsRenderer::new()` - Load shields.json
- `render_block(color, style)` - Single colored block
- `render_twotone(left, right, style)` - Two-color block
- `render_bar(colors, style)` - Multiple inline blocks
- `render_icon(logo, bg, logo_color, style)` - Logo badge
- `resolve_color(color)` - Resolve palette name to hex
- `has_style(name)` - Check if shield style exists
- `list_styles()` - Query available styles
- `list_palette()` - Query palette colors

#### Parser Enhancements

**Priority-based parsing (updated):**
1. UI components (`{{ui:*}}`) - Expand first
2. Frame templates (`{{frame:*}}`)
3. Badge templates (`{{badge:*}}`)
4. Shields templates (`{{shields:*}}`)
5. Style templates (`{{mathbold}}`)

**New parsing features:**
- Self-closing tag detection (`/}}` ending)
- Generic closer matching (`{{/ui}}` for any `ui:*` block)
- Stack-based tag matching for `{{/ui}}`
- Colon-separated parameter parsing (`:arg1:arg2:key=value`)
- Positional args vs key-value params

#### Data Files

**New:**
- `data/components.json` - 6 component definitions (~1KB)
- `data/palette.json` - 15 design token colors (<1KB)
- `data/shields.json` - Shield styles and palette (~1KB)

**Existing:**
- `data/frames.json` - 27 frame styles (~3KB)
- `data/badges.json` - 6 badge types (~2KB)
- `data/styles.json` - 19 character styles (~15KB)

**Total embedded data:** ~22KB

#### Error Handling

**New errors:**
- `Error::UnknownShieldType` - Invalid shield primitive type
- `Error::UnknownShieldStyle` - Invalid shield style
- `Error::InvalidColor` - Invalid color (not palette name or hex)
- `Error::MissingShieldParam` - Missing required parameter
- Component expansion errors (unknown component)

#### Testing

**152 tests passing** (up from 113):
- 14 ComponentsRenderer tests (expansion, palette resolution)
- 14 ShieldsRenderer tests (all 4 primitives, color resolution, styles)
- 12 UI template parser tests (self-closing, block, args, composition)
- 2 shields primitive tests (escape hatch usage)
- All existing tests still passing (frames, badges, styles, composition)

#### Documentation

**New documents:**
- `docs/COMPONENTS.md` (627 lines) - Component system design
  - Expansion model explained
  - Component structure and schema
  - Design tokens guide
  - Creating custom components
  - Troubleshooting

**Major rewrites:**
- `README.template.md` (487 lines) - Component-first examples
- `docs/ARCHITECTURE.md` (820 lines) - 5-component architecture
- `docs/API-GUIDE.md` (1,753 lines) - ComponentsRenderer and ShieldsRenderer APIs added
- `examples/README.md` (568 lines) - UI component examples
- `docs/PLANNING.md` (554 lines) - v1.0.0 status and roadmap

**Total new documentation:** 4,809 lines

### Changed

#### Architecture

**From 4 to 5 components:**
- Old: Converter, FrameRenderer, BadgeRenderer, TemplateParser
- New: ComponentsRenderer, ShieldsRenderer, Converter, FrameRenderer, BadgeRenderer, TemplateParser

**Three-layer model:**
1. **UI Components** - What users write (`{{ui:*}}`)
2. **Primitives** - Rendering engines (`{{shields:*}}`, `{{frame:*}}`, `{{badge:*}}`)
3. **Styles** - Character transformation (`{{mathbold}}`)

**Expansion over direct rendering:**
- Components expand to template strings (not direct rendering calls)
- Enables data-driven component definitions
- Users can define custom components in JSON (no recompilation)
- Recursive parsing handles expanded templates naturally

**Parser priority order:**
- Old: Frame → Badge → Style
- New: UI → Frame → Badge → Shields → Style
- Critical for expansion model to work correctly

#### Documentation Strategy

**Component-first approach:**
- README features `{{ui:*}}` prominently
- Primitives mentioned as "Advanced Features" or "Escape Hatch"
- API docs start with ComponentsRenderer (not Converter)
- Examples show UI components first, primitives later

**Honesty about capabilities:**
- Removed "zero-copy" claims (we allocate Strings)
- Changed "fail-safe" to "strict by default"
- Removed unverified benchmark numbers
- Added "allocation-minimized" (accurate)
- Explicit about data packaging (`include_str!()`)

#### README Structure

**New header:**
```markdown
# {{ui:header}}MDFX{{/ui}}
```

**New quick start:**
```markdown
{{ui:header}}PROJECT NAME{{/ui}}
{{ui:divider/}}
{{ui:tech:rust/}} {{ui:tech:python/}}
{{ui:status:success/}} All systems operational
```

**Sections reordered:**
1. Quick Start (UI components)
2. Motivation
3. UI Components (NEW - primary API)
4. Text Styles (moved down)
5. Advanced Features (frames, badges, primitives)

#### CLI Output

**No changes to CLI** - all changes are library/API level

### Fixed

**Documentation accuracy:**
- Removed misleading "zero-copy" and "fail-safe" claims
- Fixed nesting depth tracking claims (we don't track same-type nesting)
- Corrected data packaging explanation (embedded, not filesystem)
- Updated performance claims (complexity analysis, not fake benchmarks)

**Code optimization:**
- Removed `Vec<char>` allocation in separator mode
- Changed to `chars().peekable()` for streaming
- Maintains same functionality with better memory characteristics

**Documentation clarity:**
- FRAMES-DESIGN.md rewritten (1,152 → 426 lines, -63%)
- Hard split between "What's Implemented" vs "What's Planned"
- Removed all timeline estimates and "Phase" sections
- Removed confusing "Approach 1/2/3" brainstorms

### Technical Details

**Parsing flow example:**
```
Input: {{ui:header}}PROJECT{{/ui}}

1. Parse UI template → component="header", content="PROJECT"
2. Expand via ComponentsRenderer:
   → {{frame:gradient}}{{mathbold:separator=dot}}PROJECT{{/mathbold}}{{/frame}}
3. Recursive parse expanded template:
   → Parse frame template
   → Parse style template with separator
4. Render:
   → Apply mathbold: PROJECT → 𝐏𝐑𝐎𝐉𝐄𝐂𝐓
   → Add separators: 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓
   → Apply frame: ▓▒░ 𝐏·𝐑·𝐎·𝐉·𝐄·𝐂·𝐓 ░▒▓
5. Output
```

**Performance characteristics:**
- Component expansion: O(1) string substitution
- Recursive parsing: O(d*n) where d=depth, n=length
- Same overall complexity as before (expansion cost negligible)
- Streaming in converter (no Vec allocation)

**Memory:**
- Embedded JSON: ~22KB (up from ~20KB)
- Component expansion allocates small strings (<1KB typically)
- Shields URLs are ~100-200 bytes each

### Breaking Changes

**None** - v1.0.0 is the first versioned release.

**If migrating from earlier development versions:**
- No breaking changes to existing syntax (`{{mathbold}}`, `{{frame:*}}`, `{{badge:*}}` all work)
- New `{{ui:*}}` namespace added (recommended for new usage)
- ShieldsRenderer added (primitives, not typically used directly)

### Notes

**Component-first philosophy:**
- This release shifts mdfx from "Unicode styles with frames and badges" to "semantic components that expand to primitives"
- The old APIs remain and work perfectly - they're just positioned as advanced/escape hatch
- Most users should start with `{{ui:*}}` components for concise, semantic markup

**Extensibility:**
- v1.0.0 ships with 6 built-in components
- Components are JSON-defined (not Rust code)
- v1.1.0 will support user-provided components (no recompilation)
- Native components (Rust-implemented logic) planned for v1.1.0+

**Design tokens:**
- Palette allows consistent branding
- 15 colors cover most use cases
- Users can reference by name (`accent`) or direct hex
- v1.1.0 will support project-local palette.json

**Documentation quality:**
- 4,809 lines of new/rewritten documentation
- All docs now feature UI components prominently
- Comprehensive API reference (1,753 lines)
- Real-world examples and integration guides

**Next steps:**
- Publish to crates.io
- GitHub release with binaries
- Community feedback
- v1.1.0 planning (user extensibility)

---

## Pre-release Development

### 2025-12-12 - Component System Implementation
- Designed three-layer architecture (UI → Primitives → Styles)
- Implemented ComponentsRenderer with expansion model
- Created palette.json with 15 design tokens
- Implemented ShieldsRenderer (4 primitives)
- Added self-closing tag syntax
- Added generic `{{/ui}}` closer
- Updated parser priority order
- 152 tests passing
- Major documentation rewrite (4,809 lines)

### 2025-12-11 - Badge Component & Composition
- Implemented BadgeRenderer (6 badge types)
- Added badge template parsing
- Composition system (frames + badges + styles)
- Updated recursive parsing
- 113 tests passing

### 2025-12-10 - Frame Component & Separators
- Implemented FrameRenderer (27 frame styles)
- Added separator support (5 types)
- Frame template parsing
- Unified converter architecture
- 88 tests passing

### 2025-12-09 - Template Parser & CLI
- Character-by-character state machine parser
- CLI with convert, process, list commands
- Code block preservation
- Template syntax: `{{style}}content{{/style}}`
- Spacing parameter support

### 2025-12-08 - Core Library
- Converter struct with 19 Unicode styles
- Style alias support
- Data-driven configuration (styles.json)
- Zero-copy operations
- 49 tests passing

---

[1.0.0]: https://github.com/blackwell-systems/mdfx/releases/tag/v1.0.0
