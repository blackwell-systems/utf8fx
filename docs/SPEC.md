# mdfx Specification

**Stable API Contracts and Versioning Policy**

Version: 1.0.0  
Status: **Normative**  
Last Updated: 2025-12-13

---

## Purpose

This document defines the **stable contracts** that mdfx guarantees to users. It specifies:

1. What **will never break** (guaranteed stable forever)
2. What **can evolve** (following semantic versioning)
3. What **may break** (with major version bumps)
4. How **versions are managed** (semver + LTS policy)

**If it's in this spec, it's a binding commitment.**

---

## Semantic Versioning

mdfx follows [Semantic Versioning 2.0.0](https://semver.org/):

```
MAJOR.MINOR.PATCH

1.2.3
│ │ └─ Patch: Bug fixes, no API changes
│ └─── Minor: New features, backward compatible
└───── Major: Breaking changes
```

### Version Guarantees

**PATCH releases** (e.g., 1.0.0 → 1.0.1):
- Bug fixes only
- No new features
- No behavior changes (except fixing bugs)
- 100% drop-in replacement

**MINOR releases** (e.g., 1.0.0 → 1.1.0):
- New features
- New components, primitives, backends
- Deprecation warnings (not removal)
- Backward compatible with same MAJOR

**MAJOR releases** (e.g., 1.x.x → 2.0.0):
- Breaking changes allowed
- Migration guide required
- Deprecations removed
- 6-month advance notice

---

## Stability Tiers

### Tier 1: STABLE FOREVER

**These will never change in any version.**

#### Template Syntax Core

```markdown
{{name}}              ← Opening tag
{{/name}}             ← Closing tag  
{{name/}}             ← Self-closing tag
{{name:arg}}          ← Positional argument
{{name:key=value}}    ← Named parameter
```

**Guaranteed**:
- `{{` and `}}` delimiters (never changing)
- `/}}` for self-closing (never changing)
- `{{/closer}}` for block closing (never changing)
- `:` as argument separator (never changing)
- `=` for named parameters (never changing)

**Rational**: Changing these would break every template ever written.

#### Character Set

```
Reserved characters in template syntax:
  {  }  /  :  =

Forbidden in parameter values:
  :  /  }

Always allowed:
  Any other Unicode character (including emoji, etc.)
```

**Guaranteed**: These restrictions are permanent.

#### Code Block Preservation

```markdown
Templates inside code blocks are NEVER processed:

```
{{mathbold}}Not processed{{/mathbold}}
```

`{{mathbold}}Not processed{{/mathbold}}`
```

**Guaranteed**: Code blocks are always preserved.

---

### Tier 2: STABLE (Semver Guarantees)

**These follow semver: backward compatible evolution in MINOR, breaks in MAJOR.**

#### Primitive IR (Rust API)

```rust
#[non_exhaustive]
pub enum Primitive {
    Divider {
        style: String,
        // New fields added as Option<T> in MINOR releases
    },
    Swatch {
        color: String,
        style: String,
    },
    Tech {
        logo: String,
        style: String,
    },
    Status {
        level: String,
        style: String,
    },
    // New variants added in MINOR releases
}
```

**Guarantees (v1.x)**:
- Existing variants never change structure (MAJOR required)
- New variants can be added (MINOR)
- New fields added as `Option<T>` with defaults (MINOR)
- `#[non_exhaustive]` protects match exhaustiveness

**Example evolution**:
```rust
// v1.0.0
Divider { style: String }

// v1.1.0 - Added optional theme field
Divider { style: String, theme: Option<String> }

// v2.0.0 - Could restructure (breaking)
Divider { config: DividerConfig }
```

#### Component Schema

```json
{
  "version": "2.0.0",
  "components": {
    "divider": {
      "type": "native",
      "api_version": "1.0.0",
      "self_closing": true,
      "args": [],
      "description": "..."
    }
  }
}
```

**Guarantees**:
- `version` field is schema version (can change in MAJOR)
- `api_version` per component (independent evolution)
- Required fields never removed (MAJOR required)
- New optional fields can be added (MINOR)
- Components can be added (MINOR)
- Components can be renamed/removed (MAJOR)

**Component Versioning**:
```json
{
  "divider": {
    "api_version": "1.0.0",  // Original
    ...
  },
  "divider_v2": {
    "api_version": "2.0.0",  // New version, old still works
    ...
  }
}
```

Components have independent versions. Adding a new version is MINOR, removing an old version is MAJOR.

#### Renderer Trait

```rust
pub trait Renderer {
    fn render(&self, primitive: &Primitive) -> Result<RenderedAsset>;
    
    // New methods added with default implementations in MINOR
    fn render_batch(&self, primitives: &[Primitive]) -> Result<Vec<RenderedAsset>> {
        primitives.iter().map(|p| self.render(p)).collect()
    }
}
```

**Guarantees**:
- Required methods never change signature (MAJOR required)
- New methods added with defaults (MINOR)
- Trait can be split/reorganized (MAJOR)

#### CLI Flags

```bash
# v1.x
mdfx process <INPUT>                       # Stable
mdfx process <INPUT> -o <OUTPUT>           # Stable  
mdfx process <INPUT> --backend <BACKEND>   # Stable

# v1.x → v2.x
mdfx process <INPUT> --target <TARGET>     # Added in MINOR (optional)

# v2.x (could break)
mdfx compile <INPUT> -o <OUTPUT>           # Command rename (MAJOR)
```

**Guarantees**:
- Existing flags never removed (MAJOR required)
- New optional flags can be added (MINOR)
- Flag behavior changes are breaking (MAJOR)
- Command renames are breaking (MAJOR)

---

### Tier 3: UNSTABLE (Experimental)

**These may change at any time, even in PATCH releases.**

#### Experimental Features

Features marked as experimental:
```rust
#[cfg(feature = "experimental-contexts")]
pub mod contexts;
```

```json
{
  "components": {
    "experimental_grid": {
      "experimental": true,
      "stability": "unstable"
    }
  }
}
```

**Warning**: Experimental features:
- May change without notice
- May be removed without deprecation
- Not recommended for production
- Feedback encouraged

#### Internal APIs

```rust
// Public but undocumented internal APIs
#[doc(hidden)]
pub mod internal;
```

**Warning**: These are public for technical reasons (testing, etc.) but should not be used by external code.

---

## Deprecation Policy

### Deprecation Timeline

```
v1.x: Component X announced as deprecated
      → Deprecation warning added to docs
      → Runtime warning when used
      → Still fully functional

v1.y (6+ months later): Deprecation warnings in terminal
      → mdfx process warns on use
      → Still fully functional

v2.0: Deprecated features removed
      → No longer functional
      → Migration guide provided
```

### Deprecation Examples

```rust
#[deprecated(since = "1.1.0", note = "Use `Swatch` instead")]
pub enum ColorBlock { ... }
```

```json
{
  "components": {
    "old_divider": {
      "deprecated": {
        "since": "1.1.0",
        "replacement": "divider",
        "removal": "2.0.0"
      }
    }
  }
}
```

### Deprecation Warnings

```bash
$ mdfx process input.md
Warning: Component 'old_divider' is deprecated since v1.1.0
  Use 'divider' instead
  Will be removed in v2.0.0
  
  input.md:5: {{ui:old_divider/}}
```

---

## Long-Term Support (LTS)

### LTS Policy

```
v1.x: Released 2025-12-13
      Full support: 2025-12-13 → 2027-12-13 (2 years)
      Security fixes: 2027-12-13 → 2028-12-13 (1 year)
      
v2.x: Released ~2026-09-01 (estimated)
      Full support: 2026-09-01 → 2028-09-01 (2 years)
      Security fixes: 2028-09-01 → 2029-09-01 (1 year)
```

**Full support**: Bug fixes, security patches, minor features  
**Security fixes only**: Critical security patches only

### LTS Guarantees

- Templates written for v1.0 will work in all v1.x releases
- v1.x CLI installed today will work in 2 years
- v1.x library API stable for all v1.x releases

---

## Architecture Notes

### Data Registry (Implemented)

The project uses a unified data registry (`registry.json`) as the single source of truth:

- All renderables (styles, frames, badges, components) are defined in one file
- Modules extract only the data they need using intermediate structs
- No redundant JSON files (clean architecture)
- ID and name fields derived from HashMap keys where appropriate

**Benefits**:
- Single source of truth for all renderable data
- No data duplication across files
- Consistent structure for all renderable types
- Easier to maintain and extend

### Context Validation

Context validation ensures renderables are used in appropriate contexts:

```markdown
<!-- Valid: inline separator in inline context -->
{{mathbold:separator=dot}}TEXT{{/mathbold}}

<!-- Error: block renderable in inline context -->
ERROR: Block renderable 'newline' in Inline context
  Suggestion: Use an inline-compatible separator
```

---

## Data Format

### Registry Schema

The `registry.json` file contains all renderable definitions:

```json
{
  "version": "2.0.0",
  "schema_version": "1.0.0",
  "palette": { ... },
  "shield_styles": { ... },
  "renderables": {
    "glyphs": { ... },
    "components": { ... },
    "frames": { ... },
    "styles": { ... },
    "badges": { ... }
  }
}
```

### Forward/Backward Compatibility

**Forward compatibility** (v1.x reading v1.y where y > x):
- Unknown fields are ignored
- New optional fields get defaults
- Works as long as MAJOR version matches

**Backward compatibility** (v1.y reading v1.x where x < y):
- Old files work in new version
- Missing optional fields get defaults
- Always works within same MAJOR

**Cross-MAJOR** (v2.x reading v1.x):
- Automatic migration on read
- Original files not modified
- Warning emitted

---

## Rust API Stability

### Public API

```rust
// STABLE: These are guaranteed stable in v1.x
pub struct TemplateParser { ... }
pub struct Converter { ... }
pub enum Primitive { ... }
pub trait Renderer { ... }

impl TemplateParser {
    pub fn new() -> Result<Self> { ... }           // Stable
    pub fn process(&self, input: &str) -> Result<String> { ... }  // Stable
}
```

**Guarantees**:
- Public types never removed (MAJOR required)
- Public methods never removed (MAJOR required)
- Signatures never change (MAJOR required)
- Behavior changes documented (MINOR or MAJOR)

### Cargo Features

```toml
[dependencies]
mdfx = { version = "1.0", features = ["svg-backend"] }
```

**Guarantees**:
- Default features are stable (never removed)
- Optional features may be experimental
- Feature flags can be added (MINOR)
- Removing features is breaking (MAJOR)

---

## CLI Stability

### Command Structure

```bash
# v1.x STABLE
mdfx process <INPUT>
mdfx convert --style <STYLE> <TEXT>
mdfx list
mdfx frames list
mdfx badges list
mdfx separators
```

**Guarantees**:
- Commands never removed (MAJOR required)
- Positional args never reordered (MAJOR required)
- Required flags never removed (MAJOR required)
- New subcommands can be added (MINOR)

### Output Format

```bash
# Structured output (e.g., JSON) is stable
mdfx list --format json

# Human-readable output may change in MINOR releases
mdfx list  # Format can improve
```

**Guarantees**:
- `--format json` output schema is stable (MAJOR for changes)
- Human-readable output can change freely (MINOR)
- Exit codes are stable (0 = success, non-zero = error)

---

## Error Codes

### Stable Error Categories

```rust
pub enum ErrorCode {
    // Parse errors (1xxx)
    UnclosedTag = 1001,
    MismatchedTags = 1002,
    InvalidSyntax = 1003,
    
    // Resolution errors (2xxx)
    UnknownComponent = 2001,
    UnknownStyle = 2002,
    ContextMismatch = 2003,
    
    // Rendering errors (3xxx)
    BackendError = 3001,
    AssetGenerationError = 3002,
    
    // IO errors (4xxx)
    FileNotFound = 4001,
    PermissionDenied = 4002,
}
```

**Guarantees**:
- Error codes never reused (永久性 unique)
- New codes can be added (MINOR)
- Error messages can improve (MINOR)
- Error code semantics never change (MAJOR required)

### Error Message Format

```
ERROR[1001]: Unclosed tag
  ┌─ input.md:5:1
  │
5 │ {{mathbold}}TEXT
  │ ^^^^^^^^^^^^ expected {{/mathbold}}
  │
  = help: Add closing tag: {{/mathbold}}
```

**Guarantees**:
- Error structure (code, location, message) is stable
- Error message text can improve (MINOR)
- Machine-readable format (`--format json`) is stable (MAJOR for changes)

---

## Asset Manifest Format

### Manifest Schema

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

**Guarantees (v1.x)**:
- Top-level fields never removed
- `assets[]` array structure stable
- New optional fields can be added
- `version` field tracks schema version

---

## Determinism Guarantees

### Reproducible Builds

**Guarantee**: Same input + same config = same output

```bash
# These should produce identical outputs
mdfx process input.md -o dist1/output.md
mdfx process input.md -o dist2/output.md
diff dist1/output.md dist2/output.md  # No difference
```

**Determinism sources**:
- Hash-based asset filenames (content-addressed)
- Stable template expansion order
- No timestamps in output markdown
- No randomness

**Non-deterministic** (intentionally):
- `manifest.json` timestamps (metadata only)
- Build duration
- System-specific paths (all relative)

---

## Performance Guarantees

### No Performance Guarantees

**mdfx makes NO performance commitments.**

Rationale:
- Performance can improve in MINOR releases
- Algorithmic changes may change performance characteristics
- Optimization is ongoing

**What we promise**:
- No exponential complexity (O(n²) max, usually O(n))
- No unbounded memory growth
- Reasonable performance for typical README files (<10MB)

**Benchmarking**:
- Benchmarks exist but not normative
- Performance regressions are bugs but not breaking changes

---

## Security Policy

### Security Updates

**Critical** (immediate):
- Remote code execution
- Arbitrary file writes outside assets dir
- Sensitive data leakage

**High** (1 week):
- Denial of service
- Privilege escalation
- Information disclosure

**Medium** (1 month):
- Input validation bypasses
- Resource exhaustion

**Security fixes**:
- Backported to all supported versions
- May break behavior if necessary (with advisory)
- Documented in SECURITY.md

### Vulnerability Disclosure

See `SECURITY.md` for reporting process.

---

## Testing Guarantees

### Test Suite Stability

**mdfx maintains**:
- Integration tests (stable across MINOR)
- Golden tests (output snapshots)
- Property tests (invariant checking)

**Guarantees**:
- All v1.x releases pass v1.0 test suite
- New tests can be added (MINOR)
- Existing tests never removed (MAJOR required)

---

## Documentation Guarantees

### API Documentation

**Rust docs** (`docs.rs`):
- Public APIs are documented
- Examples are tested (doc tests)
- Updated with every release

**Spec documents** (this file):
- Versioned with releases
- Historical versions available
- Changes documented in changelog

---

## Compatibility Matrix

### Supported Rust Versions

```
mdfx v1.x: Rust 1.70+
mdfx v2.x: Rust 1.75+ (estimated)
```

**MSRV Policy**:
- MSRV can increase in MINOR releases (with 6-month notice)
- MSRV never decreases (no need to)
- Documented in `Cargo.toml` and README

### Supported Platforms

**Tier 1** (guaranteed to work):
- Linux x86_64
- macOS x86_64, ARM64
- Windows x86_64

**Tier 2** (best effort):
- Linux ARM64
- BSDs

**WASM** (experimental):
- wasm32-unknown-unknown (v2.x+)

---

## What This Spec Guarantees

### For Template Authors

- Templates written for v1.0 work in all v1.x releases
- Deprecated features work until MAJOR bump
- Clear error messages when syntax is wrong
- Deterministic output (CI-friendly)

### For Library Users

- Rust API stable within MAJOR version
- Semver-compliant dependencies
- Public APIs never removed without MAJOR bump
- Data format migrations handled automatically

### For Tool Integrators

- CLI flags stable within MAJOR version
- JSON output schema versioned and stable
- Exit codes stable
- Error codes unique and stable

---

## Versioning Examples

### PATCH Release (1.0.0 → 1.0.1)

**Allowed**:
- Bug fixes (parser crashes, incorrect output)
- Documentation fixes
- Internal refactoring (no behavior change)
- Performance improvements (not breaking)

**Not allowed**:
- New features
- Deprecations
- Behavior changes (except bug fixes)

### MINOR Release (1.0.0 → 1.1.0)

**Allowed**:
- New components
- New primitives
- New backends
- New optional parameters (with defaults)
- Deprecation warnings (not removal)
- Performance improvements

**Not allowed**:
- Breaking changes
- Removing features
- Changing required parameters
- Incompatible behavior changes

### MAJOR Release (1.x.x → 2.0.0)

**Allowed**:
- Everything (it's a major bump)
- Breaking changes
- Removing deprecated features
- Restructuring APIs
- Changing defaults

**Required**:
- Migration guide
- 6-month deprecation period (for known breakage)
- Automated migration tool (where possible)

---

## Conclusion

**This spec is a commitment.**

If mdfx violates these guarantees, it's a bug. File an issue, and we'll fix it or update the spec.

**Stability enables trust.**

By clearly defining what's stable and what can change, users can confidently build on mdfx without fear of sudden breakage.

---

**Next**: See [DESIGN.md](DESIGN.md) for the architectural vision that this spec supports.
