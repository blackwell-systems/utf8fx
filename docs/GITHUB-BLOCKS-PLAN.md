# GitHub Blocks Implementation Plan

**Version:** 1.1.0
**Status:** Ready to implement
**Estimated effort:** 2-3 days

---

## Goals

Add GitHub-first compositional blocks that work within Markdown constraints:
- Section headers with dividers
- Blockquote-style callouts with status indicators
- Inline status rows
- All blocks degrade gracefully

## Key Corrections from Initial Plan

### ✅ Status Primitive Already Exists
**Current state:**
```rust
Primitive::Status { level: String, style: String }
```

**Don't add it again.** Instead:
- Keep current implementation
- Level already supports both palette names and hex codes
- Works perfectly for status indicators

### ⚠️ Multiline Blockquote Gotcha
**Problem:**
```markdown
> **Note**
> Line 1
> Line 2
```

Template substitution with `> $content` only quotes the first line.

**Solution:** Add post-processing infrastructure for blockquote transformation.

### 📝 Use Positional Args
**Current syntax:** `$1`, `$2`, `$content`
**Don't change to named args** ($title, $level) - that's a bigger scope change.

---

## Phase 1: Implementation Tasks

### Task 1: Add Golden Tests for Edge Cases (Priority: HIGH)

**File:** `crates/mdfx/tests/whitespace_tests.rs`

Test these scenarios:
```rust
#[test]
fn preserves_blank_lines_around_blocks() {
    let input = r#"
Paragraph 1

{{ui:section title="Section"/}}

Paragraph 2
"#;

    let output = process(input);
    assert!(output.contains("\n\n## Section\n"));
    assert!(output.contains("Section\n![]("));
    assert!(output.contains(".svg)\n\nParagraph 2"));
}

#[test]
fn handles_block_expansion_in_lists() {
    let input = r#"
- Item 1
  {{ui:callout-github type="note"}}Warning{{/ui:callout-github}}
- Item 2
"#;

    let output = process(input);
    // Should preserve list indentation
}

#[test]
fn handles_nested_blockquotes() {
    let input = r#"
> Outer quote
> {{ui:callout-github type="note"}}Inner{{/ui:callout-github}}
"#;

    let output = process(input);
    // Should handle nested > > correctly
}
```

**Why first?** These will catch issues before we ship blocks to users.

---

### Task 2: Add Post-Processing Infrastructure

**File:** `crates/mdfx/src/components.rs`

#### 2.1 Add PostProcess Enum

```rust
/// Post-processing operations for component expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostProcess {
    /// No post-processing
    None,
    /// Prefix every line with "> " for blockquotes
    Blockquote,
}
```

#### 2.2 Update ComponentDef

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDef {
    // ... existing fields ...

    #[serde(default)]
    pub post_process: PostProcess,
}

impl Default for PostProcess {
    fn default() -> Self {
        PostProcess::None
    }
}
```

#### 2.3 Implement Blockquote Transformer

```rust
impl ComponentsRenderer {
    fn expand_template(
        &self,
        component: &str,
        args: &[String],
        content: Option<&str>,
    ) -> Result<String> {
        // ... existing substitution logic ...

        // Apply post-processing
        let comp = self.components.get(component).unwrap();
        let expanded = match &comp.post_process {
            PostProcess::None => expanded,
            PostProcess::Blockquote => self.apply_blockquote(&expanded),
        };

        Ok(expanded)
    }

    /// Apply blockquote formatting (prefix every line with "> ")
    fn apply_blockquote(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    ">".to_string()  // Empty blockquote line
                } else {
                    format!("> {}", line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

---

### Task 3: Add GitHub Block Components

**File:** `crates/mdfx/data/components.json`

#### 3.1 Section Header

```json
{
  "section": {
    "type": "expand",
    "self_closing": true,
    "description": "Section header with divider (GitHub-optimized)",
    "args": ["title", "level"],
    "template": "$2 $1\n{{ui:divider/}}",
    "defaults": {
      "level": "##"
    }
  }
}
```

**Usage:**
```markdown
{{ui:section title="Installation"/}}
→ ## Installation
  ![](assets/mdfx/divider_abc123.svg)

{{ui:section title="Advanced" level="###"/}}
→ ### Advanced
  ![](assets/mdfx/divider_abc123.svg)
```

**Implementation note:** Need to handle default arg in expand_template:
```rust
// If arg missing, use default from component definition
let arg = if i < args.len() {
    &args[i]
} else if let Some(default) = comp.defaults.get(&format!("{}", i + 1)) {
    default
} else {
    ""
};
```

#### 3.2 GitHub Callout (Blockquote Style)

```json
{
  "callout-github": {
    "type": "expand",
    "self_closing": false,
    "description": "GitHub-style blockquote callout with status indicator",
    "args": ["type", "title"],
    "template": "{{ui:status:$1/}} **$2**\n$content",
    "post_process": "blockquote",
    "defaults": {
      "title": "Note"
    }
  }
}
```

**Usage:**
```markdown
{{ui:callout-github type="warning" title="Breaking Change"}}
This API will be removed in v2.0.
Use the new syntax instead.
{{/ui:callout-github}}
```

**Output:**
```markdown
> ![](assets/mdfx/status_warning.svg) **Breaking Change**
> This API will be removed in v2.0.
> Use the new syntax instead.
```

**Type mappings:**
- `note` → info level (blue)
- `tip` → success level (green)
- `warning` → warning level (yellow)
- `danger` → error level (red)
- `info` → info level (blue)

#### 3.3 Status Item (Inline Composition)

```json
{
  "statusitem": {
    "type": "expand",
    "self_closing": true,
    "description": "Inline status indicator with label and text",
    "args": ["label", "level", "text"],
    "template": "{{ui:status:$2/}} **$1**: $3"
  }
}
```

**Usage:**
```markdown
{{ui:statusitem label="Build" level="success" text="passing"/}}
→ ![](assets/mdfx/status_success.svg) **Build**: passing

{{ui:statusitem label="Tests" level="success" text="189"/}}
→ ![](assets/mdfx/status_success.svg) **Tests**: 189
```

**Manual status row composition:**
```markdown
{{ui:statusitem label="Build" level="success" text="passing"/}} · {{ui:statusitem label="Tests" level="success" text="189"/}} · {{ui:statusitem label="Coverage" level="info" text="94%"/}}
```

---

### Task 4: Keep Existing Callout (Frame-Based)

**Don't remove the current callout component** - it's useful for non-GitHub targets.

Rename semantically:
- Current: `callout` (uses frames) → keep as-is
- New: `callout-github` (uses blockquotes) → for GitHub READMEs

Later, add target profiles:
```rust
// v1.2 feature
mdfx process --target github README.template.md
// Uses callout-github variant

mdfx process --target docusaurus README.template.md
// Uses MDX admonition syntax
```

---

### Task 5: Create Example Gallery

**File:** `examples/github-blocks.md`

```markdown
# GitHub Blocks Gallery

This file demonstrates all GitHub-optimized block components.

{{ui:section title="Installation"/}}

Install mdfx via cargo:

\`\`\`bash
cargo install mdfx
\`\`\`

{{ui:section title="Quick Start" level="###"/}}

Process a template file:

\`\`\`bash
mdfx process README.template.md --backend svg
\`\`\`

{{ui:section title="Features"/}}

{{ui:callout-github type="note" title="Why mdfx?"}}
mdfx generates deterministic SVG assets with content-based hashing.
This enables CI caching and reproducible builds.
{{/ui:callout-github}}

{{ui:callout-github type="tip"}}
Use named separators for discoverability: \`separator=dot\`
Or use any Unicode character directly: \`separator=⚡\`
{{/ui:callout-github}}

{{ui:callout-github type="warning" title="Breaking Change in v2.0"}}
The \`--shields\` backend flag will be renamed to \`--backend shields\`.
Update your scripts accordingly.
{{/ui:callout-github}}

{{ui:section title="Project Status"/}}

{{ui:statusitem label="Build" level="success" text="passing"/}} · {{ui:statusitem label="Tests" level="success" text="189"/}} · {{ui:statusitem label="Coverage" level="info" text="94%"/}} · {{ui:statusitem label="License" level="info" text="MIT"/}}

{{ui:section title="Contributing"/}}

We welcome contributions! See CONTRIBUTING.md for guidelines.

{{ui:callout-github type="info" title="Getting Help"}}
- GitHub Issues: Bug reports and feature requests
- GitHub Discussions: Questions and community support
- Security: blackwellsystems@protonmail.com (private disclosure)
{{/ui:callout-github}}
```

**Process it:**
```bash
mdfx process examples/github-blocks.md --backend svg -o examples/github-blocks.rendered.md
```

Then commit both template and rendered output for visual comparison.

---

### Task 6: Documentation Updates

#### 6.1 Add GitHub Blocks Section to README.md

After the "Quick Start" section, add:

```markdown
## GitHub Blocks

mdfx includes special components optimized for GitHub READMEs:

### Section Headers

\`\`\`markdown
{{ui:section title="Installation"/}}
\`\`\`

Generates a heading with a visual divider.

### Callouts

\`\`\`markdown
{{ui:callout-github type="warning" title="Breaking Change"}}
This API will be removed in v2.0.
{{/ui:callout-github}}
\`\`\`

Generates GitHub-compatible blockquote callouts with status indicators.

**Types:** `note`, `tip`, `warning`, `danger`, `info`

### Status Rows

\`\`\`markdown
{{ui:statusitem label="Build" level="success" text="passing"/}} ·
{{ui:statusitem label="Tests" level="success" text="189"/}}
\`\`\`

Creates inline status indicators for project metadata.

See [examples/github-blocks.md](examples/github-blocks.md) for a complete gallery.
```

#### 6.2 Update API-GUIDE.md

Add new section after ComponentsRenderer API:

```markdown
## GitHub Blocks

### section

**Type:** Self-closing expand component
**Args:** `[title, level]`
**Defaults:** `level="##"`

Generates a section header with divider.

**Usage:**
\`\`\`rust
let result = renderer.expand("section", &["Installation".to_string()], None)?;
\`\`\`

### callout-github

**Type:** Block expand component with post-processing
**Args:** `[type, title]`
**Defaults:** `title="Note"`
**Post-process:** `blockquote`

Generates GitHub blockquote callout.

**Types:** `note`, `tip`, `warning`, `danger`, `info`

**Usage:**
\`\`\`rust
let result = renderer.expand(
    "callout-github",
    &["warning".to_string(), "Breaking Change".to_string()],
    Some("API removed in v2.0")
)?;
\`\`\`

### statusitem

**Type:** Self-closing expand component
**Args:** `[label, level, text]`

Inline status indicator.

**Usage:**
\`\`\`rust
let result = renderer.expand(
    "statusitem",
    &["Build".to_string(), "success".to_string(), "passing".to_string()],
    None
)?;
\`\`\`
```

---

## Testing Strategy

### Unit Tests

**File:** `crates/mdfx/tests/components_github_blocks.rs`

```rust
#[test]
fn test_section_with_defaults() {
    let renderer = ComponentsRenderer::new().unwrap();
    let result = renderer.expand("section", &["Installation".to_string()], None).unwrap();

    assert!(result.contains("## Installation"));
    assert!(result.contains("{{ui:divider/}}"));
}

#[test]
fn test_section_custom_level() {
    let renderer = ComponentsRenderer::new().unwrap();
    let result = renderer.expand(
        "section",
        &["Advanced".to_string(), "###".to_string()],
        None
    ).unwrap();

    assert!(result.contains("### Advanced"));
}

#[test]
fn test_callout_github_blockquote_formatting() {
    let renderer = ComponentsRenderer::new().unwrap();
    let content = "Line 1\nLine 2\nLine 3";

    let result = renderer.expand(
        "callout-github",
        &["warning".to_string()],
        Some(content)
    ).unwrap();

    // Every line should start with "> "
    for line in result.lines() {
        assert!(line.starts_with(">"));
    }
}

#[test]
fn test_statusitem_composition() {
    let renderer = ComponentsRenderer::new().unwrap();
    let result = renderer.expand(
        "statusitem",
        &["Build".to_string(), "success".to_string(), "passing".to_string()],
        None
    ).unwrap();

    assert!(result.contains("{{ui:status:success/}}"));
    assert!(result.contains("**Build**"));
    assert!(result.contains("passing"));
}
```

### Integration Tests

Process `examples/github-blocks.md` and verify:
- Section headers render with dividers
- Callouts have `> ` prefix on every line
- Status items render inline
- Nested templates expand correctly

---

## Phase 2: Advanced Layouts (Future)

**Not in v1.1 scope**, but good to plan:

### Grid Component (Table Generation)

```markdown
{{ui:grid cols="Feature,What it does,Status"}}
Deterministic assets | Hash-based SVG filenames | {{ui:status success/}}
Backends | shields or svg | {{ui:status success/}}
{{/ui:grid}}
```

**Implementation approach:**
- Parse rows from body (split by newlines)
- Split cells by `|` (handle escaping)
- Build Markdown table
- Process nested templates in cells

**Complexity:** Medium (no new parser mode needed)

### StatusRow Component (Auto-joining)

```markdown
{{ui:statusrow}}
{{ui:statusitem label="Build" level="success" text="passing"/}}
{{ui:statusitem label="Tests" level="success" text="189"/}}
{{/ui:statusrow}}
```

**Implementation:**
- Collect statusitem expansions
- Join with ` · ` separator
- Return single line

**Complexity:** Low (just collection + joining)

### Pill Primitive (Rich Badges)

```rust
Primitive::Pill {
    label: String,
    message: String,
    color: String,
    label_color: Option<String>,
    style: String,
}
```

**Shields URL:**
```
https://img.shields.io/badge/{label}-{message}-{color}?style={style}
```

**Use case:**
```markdown
{{ui:pill label="version" message="1.0.0" color="blue"/}}
```

**Complexity:** Low (new primitive + renderer)

---

## Success Criteria

v1.1.0 is ready to ship when:

✅ Golden tests pass (whitespace, lists, nesting)
✅ Blockquote post-processor works for multiline content
✅ Section, callout-github, statusitem components added
✅ examples/github-blocks.md processes without errors
✅ Visual inspection of rendered output looks good on GitHub
✅ Documentation updated (README, API-GUIDE)
✅ All existing tests still pass

---

## Risk Mitigation

### Risk: Blockquote post-processing breaks nested templates

**Mitigation:** Apply post-processing *after* all template substitutions are complete.

Order of operations:
1. Substitute positional args (`$1`, `$2`)
2. Substitute content (`$content`)
3. Resolve palette references
4. **Then** apply post-processing (blockquote, etc.)

### Risk: Defaults system adds complexity

**Mitigation:** Keep defaults simple in v1.1:
- Only support positional arg defaults
- No conditional logic
- Just "if arg missing, use default value"

### Risk: GitHub rendering differences across contexts

**Mitigation:**
- Test in actual GitHub README (not just local preview)
- Test in Issues, Discussions, Wiki (blockquote support varies slightly)
- Document known limitations

---

## Next Steps

1. **Review this plan** - Any corrections or adjustments?
2. **Start with Task 1** - Add golden tests first (TDD approach)
3. **Implement incrementally** - One task at a time, test after each
4. **Visual validation** - Create a test repo, push examples/github-blocks.md, verify rendering

Ready to start implementation?
