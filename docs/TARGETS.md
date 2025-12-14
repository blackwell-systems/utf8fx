# Target Abstraction

**Multi-Surface Rendering Strategy**

Version: 2.0 (Design)  
Status: **Specification**  
Last Updated: 2025-12-13

---

## The Vision

**Same source markdown compiles to different targets with appropriate optimizations.**

```
             ┌──────────────┐
             │  input.md    │
             │  (source)    │
             └──────┬───────┘
                    │
        ┌───────────┴───────────┐
        ↓                       ↓
┌───────────────┐       ┌───────────────┐
│  GitHub       │       │  GitLab       │
│  - shields.io │       │  - Custom CSS │
│  - No HTML    │       │  - HTML OK    │
│  - SVG embed  │       │  - More tags  │
└───────────────┘       └───────────────┘
        ↓                       ↓
┌───────────────┐       ┌───────────────┐
│  PyPI         │       │  npm          │
│  - Plain text │       │  - GitHub-like│
│  - No images  │       │  - Markdown   │
│  - Fallbacks  │       └───────────────┘
└───────────────┘
```

**Key insight**: GitHub, GitLab, PyPI, npm, local docs all have different constraints. One source, multiple optimized outputs.

---

## What is a Target?

A **target** is a rendering destination with specific capabilities and constraints.

### Target Trait

```rust
pub trait Target {
    /// Target identifier (e.g., "github", "gitlab", "pypi")
    fn name(&self) -> &str;
    
    /// Does this target support raw HTML in markdown?
    fn supports_html(&self) -> bool;
    
    /// Does this target support embedded SVG?
    fn supports_svg_embed(&self) -> bool;
    
    /// Does this target support external image URLs?
    fn supports_external_images(&self) -> bool;
    
    /// Max line length (None = unlimited)
    fn max_line_length(&self) -> Option<usize>;
    
    /// Preferred backend for this target
    fn preferred_backend(&self) -> BackendType;
    
    /// Target-specific post-processing
    fn post_process(&self, markdown: &str) -> Result<String> {
        Ok(markdown.to_string())  // Default: no-op
    }
    
    /// Does this target support Unicode styling?
    fn supports_unicode_styling(&self) -> bool {
        true  // Most targets support Unicode
    }
}
```

---

## Shipped Targets

### GitHub Target

```rust
pub struct GitHubTarget;

impl Target for GitHubTarget {
    fn name(&self) -> &str {
        "github"
    }
    
    fn supports_html(&self) -> bool {
        false  // GitHub strips most HTML
    }
    
    fn supports_svg_embed(&self) -> bool {
        true  // Can embed SVG files
    }
    
    fn supports_external_images(&self) -> bool {
        true  // shields.io URLs work
    }
    
    fn max_line_length(&self) -> Option<usize> {
        None  // No hard limit
    }
    
    fn preferred_backend(&self) -> BackendType {
        BackendType::Shields  // Use shields.io by default
    }
}
```

**Characteristics**:
- **Markdown flavor**: GitHub-Flavored Markdown (GFM)
- **HTML support**: Very limited (no `<style>`, no `<script>`)
- **Image support**: External URLs, embedded SVGs
- **Unicode**: Full support
- **Blockquotes**: Special syntax for callouts (`> [!NOTE]`)

**Optimizations**:
- Use shields.io for badges (no local assets needed)
- Use blockquotes for callouts
- Use embedded SVGs for offline docs

### GitLab Target

```rust
pub struct GitLabTarget;

impl Target for GitLabTarget {
    fn name(&self) -> &str {
        "gitlab"
    }
    
    fn supports_html(&self) -> bool {
        true  // GitLab allows more HTML
    }
    
    fn supports_svg_embed(&self) -> bool {
        true
    }
    
    fn supports_external_images(&self) -> bool {
        true
    }
    
    fn max_line_length(&self) -> Option<usize> {
        None
    }
    
    fn preferred_backend(&self) -> BackendType {
        BackendType::Shields  // Or custom GitLab backend
    }
    
    fn post_process(&self, markdown: &str) -> Result<String> {
        // GitLab-specific transformations
        let processed = markdown
            .replace("{{gitlab:issue:", "{{")  // Expand GitLab-specific syntax
            .replace("{{gitlab:mr:", "{{");
        Ok(processed)
    }
}
```

**Characteristics**:
- **Markdown flavor**: GitLab-Flavored Markdown
- **HTML support**: More permissive than GitHub
- **Image support**: External URLs, embedded SVGs
- **Special features**: Issue/MR references, diagrams

**Optimizations**:
- Can use custom HTML/CSS (within limits)
- GitLab-specific features (Mermaid diagrams, etc.)

### PyPI Target

```rust
pub struct PyPITarget;

impl Target for PyPITarget {
    fn name(&self) -> &str {
        "pypi"
    }
    
    fn supports_html(&self) -> bool {
        false  // PyPI strips HTML in long_description
    }
    
    fn supports_svg_embed(&self) -> bool {
        false  // No SVG embed support
    }
    
    fn supports_external_images(&self) -> bool {
        true  // Can link to external images
    }
    
    fn max_line_length(&self) -> Option<usize> {
        Some(80)  // PyPI recommends 80-char lines
    }
    
    fn preferred_backend(&self) -> BackendType {
        BackendType::PlainText  // Fallback to text
    }
    
    fn supports_unicode_styling(&self) -> bool {
        false  // PyPI often displays in ASCII-only contexts
    }
}
```

**Characteristics**:
- **Markdown flavor**: reStructuredText or limited Markdown
- **HTML support**: None
- **Image support**: External URLs only (no embed)
- **Unicode**: Limited (some clients are ASCII-only)

**Optimizations**:
- Plain text fallbacks for styled text
- External shields.io URLs (if images allowed)
- No fancy Unicode (or provide ASCII alternatives)

### npm Target

```rust
pub struct NpmTarget;

impl Target for NpmTarget {
    fn name(&self) -> &str {
        "npm"
    }
    
    fn supports_html(&self) -> bool {
        false  // npm README doesn't support HTML
    }
    
    fn supports_svg_embed(&self) -> bool {
        true  // Can embed SVGs
    }
    
    fn supports_external_images(&self) -> bool {
        true  // External URLs work
    }
    
    fn max_line_length(&self) -> Option<usize> {
        None
    }
    
    fn preferred_backend(&self) -> BackendType {
        BackendType::Shields  // Same as GitHub
    }
}
```

**Characteristics**:
- **Markdown flavor**: Similar to GitHub
- **HTML support**: Limited
- **Image support**: External URLs, embedded SVGs
- **Unicode**: Full support

**Optimizations**:
- Treat like GitHub (very similar constraints)

### Local Docs Target

```rust
pub struct LocalDocsTarget;

impl Target for LocalDocsTarget {
    fn name(&self) -> &str {
        "local"
    }
    
    fn supports_html(&self) -> bool {
        true  // Local docs can use full HTML
    }
    
    fn supports_svg_embed(&self) -> bool {
        true
    }
    
    fn supports_external_images(&self) -> bool {
        false  // Prefer local assets (offline-first)
    }
    
    fn max_line_length(&self) -> Option<usize> {
        None
    }
    
    fn preferred_backend(&self) -> BackendType {
        BackendType::Svg  // Generate local SVG files
    }
}
```

**Characteristics**:
- **Markdown flavor**: Any (MkDocs, Docusaurus, etc.)
- **HTML support**: Full (can customize CSS)
- **Image support**: Local files preferred (offline-first)
- **Unicode**: Full support

**Optimizations**:
- Use SVG backend (no external dependencies)
- Deterministic asset names (version control friendly)
- Manifest for asset management

---

## Target Selection

### CLI Interface

```bash
# Explicit target selection
mdfx process input.md --target github -o README.md
mdfx process input.md --target gitlab -o README.md
mdfx process input.md --target pypi -o PKG-INFO
mdfx process input.md --target npm -o README.md

# Auto-detect from output path (v2.x)
mdfx process input.md -o README.md           # Auto: github
mdfx process input.md -o docs/index.md       # Auto: local
mdfx process input.md -o PKG-INFO            # Auto: pypi

# List available targets
mdfx targets list
```

### Auto-Detection Rules

```rust
fn detect_target_from_path(path: &Path) -> Option<&'static str> {
    match path.file_name()?.to_str()? {
        "README.md" => Some("github"),  // Assume GitHub by default
        "PKG-INFO" => Some("pypi"),
        "setup.py" | "pyproject.toml" => Some("pypi"),
        "package.json" => Some("npm"),
        _ => {
            // Check parent directory names
            if path.ancestors().any(|p| p.ends_with("docs")) {
                Some("local")
            } else {
                None  // Unknown, use default
            }
        }
    }
}
```

### Configuration File

```json
// mdfx.json
{
  "version": "1.0.0",
  "target": "github",
  "backend": "shields",
  "assets_dir": "assets/mdfx",
  
  "targets": {
    "github": {
      "backend": "shields",
      "post_process": ["optimize_images"]
    },
    "local": {
      "backend": "svg",
      "assets_dir": "docs/assets"
    }
  }
}
```

```bash
# Use config file
mdfx process input.md --config mdfx.json
```

---

## Target-Specific Rendering

### Backend Selection

Each target has a **preferred backend**, but users can override:

```bash
# Use target's preferred backend
mdfx process input.md --target github -o README.md
# → Uses ShieldsBackend (GitHub's preferred)

# Override backend
mdfx process input.md --target github --backend svg -o README.md
# → Uses SvgBackend (local assets even on GitHub)
```

### Primitive Rendering Differs by Target

```rust
impl Renderer for MultiTargetBackend {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset> {
        match (primitive, self.target.name()) {
            (Primitive::Divider { .. }, "github") => {
                // GitHub: Use shields.io bar
                self.shields_backend.render(primitive)
            }
            (Primitive::Divider { .. }, "local") => {
                // Local: Generate SVG
                self.svg_backend.render(primitive)
            }
            (Primitive::Divider { .. }, "pypi") => {
                // PyPI: Fallback to ASCII art
                Ok(RenderedAsset::InlineMarkdown(
                    "─────────────────────────────".to_string()
                ))
            }
            _ => self.default_backend.render(primitive),
        }
    }
}
```

### Post-Processing by Target

```rust
impl Target for GitHubTarget {
    fn post_process(&self, markdown: &str) -> Result<String> {
        let mut output = markdown.to_string();
        
        // GitHub-specific: Convert callouts to GitHub alert syntax
        output = convert_callouts_to_github_alerts(&output);
        
        // GitHub-specific: Optimize image URLs
        output = optimize_image_urls(&output);
        
        Ok(output)
    }
}

fn convert_callouts_to_github_alerts(markdown: &str) -> String {
    // Convert mdfx callouts to GitHub's native alert syntax
    markdown
        .replace("> 🟢 **Note**", "> [!NOTE]")
        .replace("> ⚠️  **Warning**", "> [!WARNING]")
        .replace("> 🔴 **Error**", "> [!CAUTION]")
}
```

---

## Fallback Strategies

### Image Fallbacks

```rust
pub struct FallbackStrategy {
    /// Try external URLs first, then local assets
    pub prefer_external: bool,
    
    /// If images not supported, use alt text
    pub alt_text_fallback: bool,
    
    /// If Unicode not supported, use ASCII
    pub ascii_fallback: bool,
}

impl Target for PyPITarget {
    fn render_primitive(&self, primitive: &Primitive) -> Result<String> {
        match primitive {
            Primitive::Swatch { color, .. } => {
                if !self.supports_external_images() {
                    // Fallback to colored text representation
                    Ok(format!("[COLOR: #{}]", color))
                } else {
                    // Use shields.io URL
                    Ok(format!("![](https://img.shields.io/badge/-%20-{})", color))
                }
            }
            Primitive::Tech { logo, .. } => {
                if !self.supports_external_images() {
                    // Fallback to text
                    Ok(format!("[{}]", logo))
                } else {
                    Ok(shields_url(logo))
                }
            }
            _ => Err(Error::UnsupportedPrimitive),
        }
    }
}
```

### Unicode Fallbacks

```rust
fn render_styled_text(text: &str, style: &str, target: &dyn Target) -> String {
    if target.supports_unicode_styling() {
        // Normal Unicode transformation
        apply_unicode_style(text, style)
    } else {
        // ASCII fallback
        match style {
            "mathbold" => format!("**{}**", text),  // Markdown bold
            "script" => format!("*{}*", text),      // Markdown italic
            _ => text.to_string(),                  // Plain text
        }
    }
}
```

---

## Target Capabilities Matrix

| Feature | GitHub | GitLab | PyPI | npm | Local |
|---------|--------|--------|------|-----|-------|
| **HTML** | ✗ | ✓ | ✗ | ✗ | ✓ |
| **SVG Embed** | ✓ | ✓ | ✗ | ✓ | ✓ |
| **External Images** | ✓ | ✓ | ✓ | ✓ | ✗ |
| **Unicode Styling** | ✓ | ✓ | ~ | ✓ | ✓ |
| **Blockquotes** | ✓ | ✓ | ✗ | ✓ | ✓ |
| **Tables** | ✓ | ✓ | ~ | ✓ | ✓ |
| **Task Lists** | ✓ | ✓ | ✗ | ✓ | ✓ |
| **Emoji** | ✓ | ✓ | ~ | ✓ | ✓ |
| **Mermaid** | ✓ | ✓ | ✗ | ✗ | ~ |

Legend:
- ✓ = Supported
- ✗ = Not supported
- ~ = Partial support

---

## Target-Specific Escapes

### Tier 3 Escapes (Use at Your Own Risk)

```markdown
<!-- GitHub-specific HTML (may break on other targets) -->
{{target:github:html}}
<details>
<summary>Click to expand</summary>

This uses GitHub's <details> support.

</details>
{{/target}}

<!-- GitLab-specific diagram -->
{{target:gitlab:mermaid}}
graph TD
    A[Start] --> B[End]
{{/target}}

<!-- PyPI-specific reStructuredText -->
{{target:pypi:rst}}
.. note::
   This is a reStructuredText note.
{{/target}}
```

**Behavior**:
- Content only rendered when target matches
- Ignored (removed) for other targets
- Zero validation (user responsibility)

**Warning**: These are **explicitly unsupported**. They bypass all safety guarantees.

---

## Multi-Target Builds

### Building for Multiple Targets

```bash
# Build for all targets
mdfx build --all-targets

# Outputs:
# - dist/github/README.md
# - dist/gitlab/README.md
# - dist/pypi/PKG-INFO.md
# - dist/npm/README.md
# - dist/local/docs/index.md

# Build for specific targets
mdfx build --targets github,local
```

### Multi-Target Configuration

```json
// mdfx.json
{
  "version": "1.0.0",
  "builds": [
    {
      "target": "github",
      "input": "README.template.md",
      "output": "README.md",
      "backend": "shields"
    },
    {
      "target": "local",
      "input": "README.template.md",
      "output": "docs/index.md",
      "backend": "svg",
      "assets_dir": "docs/assets"
    },
    {
      "target": "pypi",
      "input": "README.template.md",
      "output": "PKG-INFO.md",
      "backend": "plaintext"
    }
  ]
}
```

```bash
mdfx build --config mdfx.json
```

---

## Target Detection Heuristics

### Git Repository Detection

```rust
fn detect_target_from_git_remote() -> Option<&'static str> {
    let output = std::process::Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .output()
        .ok()?;
    
    let remote = String::from_utf8(output.stdout).ok()?;
    
    if remote.contains("github.com") {
        Some("github")
    } else if remote.contains("gitlab.com") {
        Some("gitlab")
    } else {
        None
    }
}
```

### Package Manager Detection

```rust
fn detect_target_from_package_manager(cwd: &Path) -> Option<&'static str> {
    if cwd.join("package.json").exists() {
        Some("npm")
    } else if cwd.join("pyproject.toml").exists() || cwd.join("setup.py").exists() {
        Some("pypi")
    } else if cwd.join("Cargo.toml").exists() {
        Some("crates-io")  // Future
    } else {
        None
    }
}
```

---

## Implementation Phases

### Phase 1: v1.0 - GitHub Only ✅ COMPLETE

**Status**: Shipped December 2025

- ✅ Single target: GitHub (implicit)
- ✅ Shields.io + SVG backends
- No target abstraction yet

### Phase 2: v1.1 - Add Target Trait ✅ COMPLETE (ahead of schedule)

**Status**: Implemented December 2025

- ✅ Added `Target` trait to codebase (`targets.rs`)
- ✅ Implemented `GitHubTarget` (wraps current behavior)
- ✅ Added `BackendType` enum with derived Default
- ✅ 8 tests for Target trait

**Remaining**:
- ⏳ Refactor backends to be fully target-aware

**Migration**: None required (GitHub is default)

### Phase 3: v1.2 - Add More Targets ✅ COMPLETE

**Status**: Implemented December 2025

- ✅ Implemented `LocalDocsTarget` (SVG files, offline-first)
- ✅ Implemented `NpmTarget` (similar to GitHub)
- ✅ Implemented `detect_target_from_path()` utility
- ✅ Added `--target` flag to CLI (github, local, npm, auto)
- ✅ Wired target auto-detection into CLI
- ✅ Added `--palette` flag for custom palettes

**Migration**: None required (GitHub still default)

### Phase 4: v2.0 - Full Multi-Target ✅ COMPLETE

**Status**: Implemented December 2025

- ✅ Implement `GitLabTarget` (more HTML support, Mermaid diagrams)
- ✅ Implement `PyPITarget` (plain text fallbacks, ASCII-safe)
- ⏳ Target-specific post-processing
- ⏳ Fallback strategies
- ⏳ Target-specific escapes

**Migration**: Users can explicitly set target in config

---

## Target Registry

### Extensibility

Users can define custom targets:

```json
// mdfx.json
{
  "version": "2.0.0",
  "custom_targets": {
    "my_docs_platform": {
      "supports_html": true,
      "supports_svg_embed": true,
      "supports_external_images": false,
      "preferred_backend": "svg",
      "post_process": [
        "convert_headings_to_custom_syntax"
      ]
    }
  }
}
```

### Plugin System (v3.x)

```rust
// External crate: mdfx-target-confluence
pub struct ConfluenceTarget;

impl Target for ConfluenceTarget {
    fn name(&self) -> &str { "confluence" }
    // ... implement trait
}

// Register with mdfx
#[mdfx::target]
pub fn register() -> Box<dyn Target> {
    Box::new(ConfluenceTarget)
}
```

---

## Testing Targets

### Cross-Target Testing

```rust
#[cfg(test)]
mod target_tests {
    use super::*;

    #[test]
    fn test_github_rejects_html() {
        let target = GitHubTarget;
        assert!(!target.supports_html());
    }

    #[test]
    fn test_gitlab_allows_html() {
        let target = GitLabTarget;
        assert!(target.supports_html());
    }

    #[test]
    fn test_pypi_prefers_plaintext() {
        let target = PyPITarget;
        assert_eq!(target.preferred_backend(), BackendType::PlainText);
    }

    #[test]
    fn test_target_specific_rendering() {
        let primitive = Primitive::Divider { style: "flat-square".into() };
        
        let github_output = render_for_target(&primitive, &GitHubTarget)?;
        let pypi_output = render_for_target(&primitive, &PyPITarget)?;
        
        // GitHub uses shields.io
        assert!(github_output.contains("img.shields.io"));
        
        // PyPI uses ASCII fallback
        assert!(pypi_output.contains("─────"));
    }
}
```

### Golden Tests per Target

```
tests/
├── golden/
│   ├── github/
│   │   ├── input.md
│   │   └── expected_output.md
│   ├── gitlab/
│   │   ├── input.md
│   │   └── expected_output.md
│   ├── pypi/
│   │   ├── input.md
│   │   └── expected_output.md
│   └── npm/
│       ├── input.md
│       └── expected_output.md
```

```bash
# Run golden tests for all targets
cargo test --test golden -- --target all

# Run for specific target
cargo test --test golden -- --target github
```

---

## Summary

**Targets enable multi-surface rendering.**

**Key concepts**:
- **Target trait**: Defines capabilities and constraints
- **Preferred backend**: Each target chooses best backend
- **Post-processing**: Target-specific transformations
- **Fallbacks**: Graceful degradation when features unavailable

**Shipped targets** (all implemented):
- GitHub (default, shields.io badges)
- Local (SVG files, offline-first)
- npm (like GitHub)
- GitLab (more HTML support)
- PyPI (plain text, ASCII-safe)

**The goal**: Write once (source), compile for many (targets).

---

**Next**: See [DESIGN.md](DESIGN.md) for the overall architectural vision.
