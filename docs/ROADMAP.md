# mdfx Roadmap

**Version:** 1.0.0
**Last Updated:** 2025-12-13

This document outlines planned features and architectural improvements for mdfx.

---

## ✅ Completed (v1.0.0)

### Core Features
- [x] 19 Unicode text styles with aliases
- [x] 28 decorative frames
- [x] 6 badge types
- [x] UI components (divider, swatch, tech, status, header, callout)
- [x] Multi-backend rendering (ShieldsBackend, SvgBackend)
- [x] Data-driven separator system (12 named + direct Unicode)
- [x] Cargo workspace structure (library + CLI separation)
- [x] 256 comprehensive tests

### Separator System Enhancements
- [x] Validation: trim whitespace, reject template delimiters
- [x] Error messages with "did you mean" suggestions
- [x] Support for any single Unicode character

---

## ✅ Completed (December 2025 - Ahead of Schedule)

### Unified Registry System (Originally planned for v1.1.0-v2.0.0)
- [x] Created unified `registry.json` consolidating 7 data files (1139 lines)
- [x] Implemented `Registry` module with complete typed API (`registry.rs`)
- [x] Renderables: Glyphs (21), Snippets (10), Components (9), Frames (32), Styles (19), Badges (6)
- [x] Alias support for all renderable types
- [x] Parser refactored to use Registry for validation (style, frame, badge, separator)
- [x] Unified resolution: glyphs → snippets → components → literal

### EvalContext System (Originally planned for v1.2.0)
- [x] Implemented `EvalContext` enum: Inline, Block, FrameChrome
- [x] All renderables annotated with allowed contexts in registry.json
- [x] Context promotion rules (Inline → Block/FrameChrome)
- [x] Separator resolution validates inline context with error messages
- [x] 8 new tests for Registry and contexts

### Target Trait (Originally planned for v1.1.0-v1.2.0)
- [x] Implemented `Target` trait abstraction (`targets.rs`)
- [x] Shipped targets: GitHubTarget, LocalDocsTarget, NpmTarget
- [x] `BackendType` enum: Shields, Svg, PlainText (with derived Default)
- [x] `detect_target_from_path()` utility for auto-detection
- [x] 8 new tests for Target trait

---

## 🔧 Next Steps (v2.0.0 Preparation)

### CLI Integration (Required for v2.0)
- [x] Add `--target` flag to CLI (github, local, npm, auto)
- [x] Wire target auto-detection from output path
- [ ] Add multi-target build command (`mdfx build --all-targets`)
- [ ] Add `--strict-contexts` flag for context validation

### Backend/Target Integration
- [ ] Refactor backends to be target-aware
- [ ] Target-specific post-processing in pipeline
- [ ] Fallback strategies for plain text targets

### Data Consolidation ✓
- [x] Consolidated all JSON data files into registry.json
- [x] Removed redundant data files (badges.json, frames.json, styles.json, etc.)
- [x] Updated all modules to use unified registry.json as single source of truth

---

## 🚀 Planned Features

### High Priority (v1.1.0)

#### 1. Enhanced Discoverability Commands

**Problem**: Users don't know what's available without reading docs.

**Solution**: Add `list` and `show` subcommands for all data types.

```bash
# List all components
mdfx components

# Show specific component definition
mdfx components show divider
# Output:
# Component: divider
# Type: Primitive
# Primitive: Divider
# Colors: Uses palette (accent, success, warning, error)
# Example: {{ui:divider/}}

# List palette colors
mdfx palette

# Show specific color
mdfx palette show accent
# Output:
# accent: #F41C80
# Used in: 3 components

# List frames
mdfx frames

# Show specific frame
mdfx frames show gradient
# Output:
# gradient
# Prefix: ▓▒░
# Suffix: ░▒▓
# Example: {{frame:gradient}}TEXT{{/frame}}

# Already implemented:
mdfx list              # Unicode styles
mdfx separators       # Separator characters
```

**Implementation Complexity**: Low
**User Value**: High (reduces docs lookup)
**Files to Change**: `crates/mdfx-cli/src/main.rs`

---

#### 2. SVG Asset Manifest

**Problem**: No metadata about generated assets for CI caching or reproducible builds.

**Solution**: Generate `manifest.json` alongside SVG assets.

```bash
mdfx process --backend svg --assets-dir ./assets input.md
# Generates:
# ./assets/mdfx/manifest.json
```

**Manifest Format**:
```json
{
  "version": "1.0",
  "created_at": "2025-12-13T10:30:00Z",
  "backend": "svg",
  "assets_dir": "./assets/mdfx",
  "total_assets": 12,
  "assets": [
    {
      "path": "assets/mdfx/swatch_541bbacc5bf498fd.svg",
      "sha256": "a3f8e2b1...",
      "type": "swatch",
      "primitive": {
        "color": "F41C80",
        "style": "flat-square"
      },
      "size_bytes": 234
    }
  ]
}
```

**Use Cases**:
- CI caching: Skip processing if manifest unchanged
- Asset verification: Detect corruption
- Dependency tracking: Know what files are generated
- Static site generators: Preload asset list

**Implementation Complexity**: Medium
**User Value**: High (enables CI optimization)
**Files to Change**: `crates/mdfx/src/renderer/svg.rs`, `crates/mdfx/src/parser.rs`

---

#### 3. Asset Caching (Smart Writes)

**Problem**: Large docs regenerate all SVGs even if unchanged.

**Solution**: Skip writes if file exists and content matches.

```rust
// In SvgBackend::render()
let target_path = format!("{}/{}", self.assets_dir, filename);

// Check if file exists and content matches
if Path::new(&target_path).exists() {
    let existing = fs::read(&target_path)?;
    if existing == svg_bytes {
        // File unchanged, skip write
        return Ok(RenderedAsset::File {
            relative_path,
            bytes: existing,
            markdown_ref,
            cached: true,  // NEW FIELD
        });
    }
}

// File doesn't exist or changed - write it
fs::write(&target_path, &svg_bytes)?;
```

**CLI Output**:
```bash
mdfx process --backend svg input.md
# Generating SVG assets...
#   ✓ swatch_541bbacc.svg (234 bytes)
#   ⚡ tech_669db7ef.svg (cached)
#   ✓ status_a3f8e2b1.svg (189 bytes)
# Generated 2 assets, cached 1
```

**Implementation Complexity**: Low
**User Value**: High (CI speed improvement)
**Files to Change**: `crates/mdfx/src/renderer/svg.rs`

---

### Medium Priority (v1.2.0)

#### 4. Inline SVG Mode (No Files)

**Problem**: Some contexts don't support external SVG files.

**Solution**: Add `svg-inline` backend that embeds SVGs as data URIs or raw `<svg>`.

```bash
mdfx process --backend svg-inline input.md
```

**Output Options**:
1. **Data URI** (for HTML/Markdown):
   ```markdown
   ![](data:image/svg+xml;utf8,%3Csvg%20xmlns%3D...)
   ```

2. **Raw SVG** (for HTML):
   ```html
   <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20">...</svg>
   ```

**Pros**:
- Single-file output (great for email, Notion, etc.)
- No asset management needed
- Works offline

**Cons**:
- Larger markdown files
- GitHub has inconsistent data URI support
- Harder to cache

**Implementation Complexity**: Medium
**User Value**: Medium (niche use case)
**Files to Create**: `crates/mdfx/src/renderer/svg_inline.rs`

---

#### 5. Template Formatter (`mdfx fmt`)

**Problem**: Templates have inconsistent formatting in team codebases.

**Solution**: Add formatter that normalizes templates.

```bash
mdfx fmt README.template.md        # Format file
mdfx fmt --check README.template.md  # Check only (CI)
```

**Formatting Rules**:
1. **Parameter Order**: Alphabetical
   ```markdown
   <!-- Before -->
   {{ui:tech:style=flat:logo=rust/}}

   <!-- After -->
   {{ui:tech:logo=rust:style=flat/}}
   ```

2. **Whitespace**: Remove extra spaces
   ```markdown
   <!-- Before -->
   {{  mathbold  :  separator=dot  }}TEXT{{  /mathbold  }}

   <!-- After -->
   {{mathbold:separator=dot}}TEXT{{/mathbold}}
   ```

3. **Palette Canonicalization**: Resolve hex to named colors
   ```markdown
   <!-- Before -->
   {{ui:swatch:F41C80/}}

   <!-- After -->
   {{ui:swatch:accent/}}  # If F41C80 matches palette
   ```

4. **Consistent Quotes** (if we add them):
   ```markdown
   {{ui:tech:name='rust'/}}  → {{ui:tech:name="rust"/}}
   ```

**Implementation Complexity**: High
**User Value**: High (team consistency)
**Files to Create**: `crates/mdfx/src/formatter.rs`, `crates/mdfx-cli/src/fmt.rs`

---

#### 6. Strict Mode

**Problem**: Teams want to enforce "pure mdfx" and catch errors early.

**Solution**: Add `--strict` flag that fails on warnings.

```bash
mdfx process --strict --backend svg input.md
```

**Strict Checks**:
1. **Unknown Palette Keys**: `{{ui:swatch:unknowncolor/}}` → ERROR
2. **Invalid Component Args**: `{{ui:tech:invalidarg/}}` → ERROR
3. **Escape Hatch Usage**: `{{shields:block:...}}` → WARNING (unless `--allow-primitives`)
4. **Deprecated Syntax**: Old template formats → WARNING
5. **Missing Assets**: Referenced assets don't exist → ERROR

**Exit Codes**:
- `0` - Success
- `1` - Errors found
- `2` - Warnings found (only in --strict)

**Implementation Complexity**: Medium
**User Value**: Medium (team enforcement)
**Files to Change**: `crates/mdfx/src/parser.rs`, `crates/mdfx-cli/src/main.rs`

---

### Low Priority (v1.3.0+)

#### 7. New Primitives

##### A. Spacer Primitive

**Use Case**: Layout control in table cells or between elements.

```markdown
{{spacer:width=50:height=20/}}
```

**Output** (SVG):
```svg
<svg xmlns="http://www.w3.org/2000/svg" width="50" height="20">
  <rect width="50" height="20" fill="transparent"/>
</svg>
```

**Implementation**: New primitive type, straightforward SVG generation.

##### B. Rule Primitive

**Use Case**: Horizontal/vertical lines separate from divider blocks.

```markdown
{{rule:width=100:thickness=2:color=slate/}}
{{rule:type=vertical:height=50/}}
```

##### C. BadgeGroup Primitive

**Use Case**: Render multiple badges with consistent spacing.

```markdown
{{badgegroup}}
{{ui:tech:rust/}}
{{ui:tech:typescript/}}
{{ui:tech:postgresql/}}
{{/badgegroup}}
```

**Output**: Inline badges with defined spacing (e.g., 4px gap).

**Implementation Complexity**: Medium per primitive
**User Value**: Low to Medium (nice-to-have)

---

#### 8. Grapheme Cluster Support (Separators)

**Problem**: Some emojis use multiple code points (emoji + variation selector).

**Current**: `separator=👨‍💻` fails (2+ chars)
**Desired**: `separator=👨‍💻` works (1 grapheme cluster)

**Solution**: Use `unicode-segmentation` crate.

```rust
use unicode_segmentation::UnicodeSegmentation;

let graphemes: Vec<&str> = input.graphemes(true).collect();
if graphemes.len() == 1 {
    // Valid separator
}
```

**Implementation Complexity**: Low (add dependency)
**User Value**: Low (edge case)
**Files to Change**: `Cargo.toml`, `crates/mdfx/src/separators.rs`

---

## 📊 Priority Matrix

| Feature | Value | Complexity | Priority | Version |
|---------|-------|------------|----------|---------|
| Discoverability commands | High | Low | **High** | v1.1.0 |
| Asset manifest | High | Medium | **High** | v1.1.0 |
| Asset caching | High | Low | **High** | v1.1.0 |
| Inline SVG mode | Medium | Medium | Medium | v1.2.0 |
| Template formatter | High | High | Medium | v1.2.0 |
| Strict mode | Medium | Medium | Medium | v1.2.0 |
| New primitives | Low-Medium | Medium | Low | v1.3.0+ |
| Grapheme clusters | Low | Low | Low | v1.3.0+ |

---

## 🛠️ Implementation Guidelines

### Adding New Commands

1. **Define in `Commands` enum** (`crates/mdfx-cli/src/main.rs`)
2. **Add handler in `run()` function**
3. **Implement logic in separate function**
4. **Add tests in `crates/mdfx-cli/src/main.rs`**
5. **Update documentation**

### Adding New Primitives

1. **Add variant to `Primitive` enum** (`crates/mdfx/src/primitive.rs`)
2. **Implement rendering in backends** (`crates/mdfx/src/renderer/*.rs`)
3. **Add parser support** (`crates/mdfx/src/parser.rs`)
4. **Create data file if needed** (`crates/mdfx/data/*.json`)
5. **Add comprehensive tests**
6. **Document in examples**

### Adding New Backends

1. **Create new file** (`crates/mdfx/src/renderer/your_backend.rs`)
2. **Implement `Renderer` trait**
3. **Add CLI flag** (`--backend your-backend`)
4. **Update documentation**
5. **Add integration tests**

---

## 📝 Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed contribution guidelines.

For feature requests or discussions, open an issue at:
https://github.com/blackwell-systems/mdfx/issues

---

**Note**: This roadmap is subject to change based on user feedback and community priorities.
