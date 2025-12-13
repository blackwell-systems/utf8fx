# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-12

### Added

#### Core Features
- **Custom Separators** - Insert Unicode separator characters between styled letters
  - 5 separator types: `dot` (·), `bullet` (•), `dash` (─), `bolddash` (━), `arrow` (→)
  - Template syntax: `{{style:separator=name}}TEXT{{/style}}`
  - API: `Converter::convert_with_separator(text, style, separator, count)`

- **Decorative Frames** - Wrap text with decorative prefix/suffix elements
  - 27 frame styles: gradient, solid (left/right/both), lines (light/bold/double/dashed), blocks, arrows, bullets, brackets (lenticular, angle, guillemet), symbols (star, diamond, finger), arcs
  - Template syntax: `{{frame:style}}TEXT{{/frame}}`
  - New component: `FrameRenderer` with `apply_frame()` method
  - Data-driven via `data/frames.json` (supports aliases like styles)

- **Badge Component** - Enclosed alphanumeric characters for step indicators and labels
  - 6 badge types: `circle` (①), `negative-circle` (❶), `double-circle` (⓵), `paren` (⑴), `period` (🄁), `paren-letter` (⒜)
  - Limited charset: numbers 0-20, lowercase letters a-z
  - Template syntax: `{{badge:type}}TEXT{{/badge}}`
  - New component: `BadgeRenderer` with `apply_badge()` method
  - Data-driven via `data/badges.json` with 200+ character mappings
  - Use cases: step indicators, priority labels, option lists

- **Full Composition** - Combine styles, separators, and frames naturally
  - Recursive parser processes frames containing styled templates
  - Example: `{{frame:gradient}}{{mathbold:separator=dot}}TITLE{{/mathbold}}{{/frame}}`
  - Enables expressive taglines and visual hierarchy

#### Library API
- `Converter::convert_with_separator()` - Convert text with custom separator characters
- `FrameRenderer::new()` - Initialize frame renderer with frames.json
- `FrameRenderer::apply_frame()` - Apply decorative frame around text
- `FrameRenderer::get_frame()` - Lookup frame by ID or alias
- `FrameRenderer::has_frame()` - Check if frame exists
- `FrameRenderer::list_frames()` - Query available frames
- `BadgeRenderer::new()` - Initialize badge renderer with badges.json
- `BadgeRenderer::apply_badge()` - Enclose text in badge character
- `BadgeRenderer::get_badge()` - Lookup badge by ID or alias
- `BadgeRenderer::has_badge()` - Check if badge exists
- `BadgeRenderer::list_badges()` - Query available badges
- Template parser supports `:separator=name` parameter
- Template parser supports `{{frame:style}}...{{/frame}}` syntax
- Template parser supports `{{badge:type}}...{{/badge}}` syntax
- Priority-based parsing: Frames → Badges → Styles (prevents ambiguity)

#### Documentation
- **NEW: docs/API-GUIDE.md** (1,176 lines) - Comprehensive API reference
  - Complete method documentation for all components
  - Code examples for every API call
  - Error handling patterns and recovery strategies
  - Advanced usage: caching, batch processing, composition
  - Performance tips and benchmarks
  - Real-world examples: README generators, CLI banners, status messages

#### Data Files
- `data/frames.json` - 27 frame definitions with prefix/suffix patterns
- `data/badges.json` - Badge character mappings (200+ characters)
  - 15 frame styles with descriptive names
  - Alias support (e.g., `grad` → `gradient`)
  - Version metadata for future compatibility

#### Error Handling
- `Error::UnknownFrame` - Graceful handling of invalid frame names
- `Error::ParseError` - Detailed error messages for invalid separators
- `Error::UnclosedTag` - Precise error messages for unclosed frame templates

#### Testing
- 88 total tests (up from 73 in pre-release)
- 11 tests for separator functionality
- 15 tests for frame functionality
- 3 tests for composition scenarios
- All tests for code block preservation, error handling, edge cases
- Zero clippy warnings

#### Documentation
- Comprehensive README with "Why utf8fx?" section addressing copy/paste alternative
- Visual examples showing before/after transformation
- "Adding Custom Styles" guide for extensibility
- Architecture documentation with data flow diagrams
- Complete frames design documentation
- Updated examples with separator and frame usage

### Changed

#### Architecture Improvements
- **Unified Converter** - Eliminated 98 lines of duplicate code
  - Internal `convert_with_char_between()` method handles all separation cases
  - Public methods (`convert`, `convert_with_spacing`, `convert_with_separator`) delegate to unified implementation
  - Fast path optimization when `count=0` skips separation logic entirely

- **Enhanced Parser** - Extended state machine for new template types
  - Parses `:separator=name` parameter after `:spacing=N`
  - Parses `{{frame:style}}` templates with recursive content processing
  - Maintains existing code block preservation and error handling
  - Returns structured `TemplateData` instead of tuples for cleaner code

- **Component Separation** - Clear single-responsibility design
  - `Converter` → Character transformation
  - `FrameRenderer` → Structural decoration
  - `TemplateParser` → Orchestration and composition
  - Each component has distinct purpose and minimal coupling

### Fixed
- Parser now properly handles multiple parameters (spacing + separator)
- Template nesting depth tracking for proper frame content processing
- Separator parameter parsing validates against known separator names

---

## Pre-release Development

### Phase 2 - Template Parser & CLI (Completed 2025-12-11)
- CLI tool with `convert`, `process`, and `list` commands
- Character-by-character state machine parser (30% faster than regex)
- Template syntax: `{{style}}content{{/style}}`
- Spacing parameter: `{{style:spacing=N}}content{{/style}}`
- Code block preservation (triple backticks and inline backticks)
- 49 comprehensive tests

### Phase 1 - Core Library (Completed 2025-12-10)
- `Converter` struct with style transformation
- 19 Unicode styles across 4 categories (Bold, Boxed, Elegant, Technical)
- Style alias support (e.g., `mb` for `mathbold`)
- Data-driven configuration via `data/styles.json`
- Preserves whitespace, punctuation, and unsupported characters
- Zero-copy operations for performance

---

[1.0.0]: https://github.com/blackwell-systems/utf8fx/releases/tag/v1.0.0
