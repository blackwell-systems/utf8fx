# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
  - **Components** (9): Semantic UI elements (`divider`, `swatch`, `tech`, `status`, etc.)
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

## [1.0.0] - 2025-12-13

### Added

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
  - 9 UI components shipped: `divider`, `swatch`, `tech`, `status`, `header`, `callout`, `section`, `callout-github`, `statusitem`
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
