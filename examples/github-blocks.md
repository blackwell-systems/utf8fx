# GitHub Blocks Example

This document demonstrates all GitHub-optimized block components in mdfx.
These components are specifically designed to work within GitHub's Markdown constraints
while maintaining clean, semantic markup.

## Installation

Install mdfx via cargo:

```bash
cargo install mdfx-cli
```

Or add as a library dependency:

```toml
[dependencies]
mdfx = "1.0"
```

{{ui:callout-github:info}}
**Quick Start Tip**
Process this template file to see the rendered output with shields.io badges.
{{/ui}}

## Project Status

{{ui:statusitem:Build:success:passing/}} · {{ui:statusitem:Tests:success:217/}} · {{ui:statusitem:Coverage:success:94%/}} · {{ui:statusitem:License:info:MIT/}}

## Features

{{ui:callout-github:success}}
**What Makes GitHub Blocks Special?**
These components are designed to work perfectly with GitHub's Markdown renderer.
No HTML, no CSS, just pure Markdown plus shields.io badges.
{{/ui}}

Core features of mdfx GitHub blocks:

- **Blockquote Callouts**: GitHub-compatible callouts with status indicators
- **Status Rows**: Inline status badges for project metadata
- **Composable**: Mix and match blocks in any combination

## Component Gallery

### Blockquote Callouts

Different callout types for different contexts:

{{ui:callout-github:info}}
**Information**
Use info callouts for helpful tips and general information.
They're perfect for onboarding new users.
{{/ui}}

{{ui:callout-github:success}}
**Success Stories**
Highlight achievements, completed migrations, or positive outcomes.
Great for release announcements!
{{/ui}}

{{ui:callout-github:warning}}
**Breaking Changes**
The old API syntax will be deprecated in v2.0.
Update your code before upgrading.
{{/ui}}

{{ui:callout-github:error}}
**Security Advisory**
A critical vulnerability was discovered and fixed in v1.0.1.
Please upgrade immediately.
{{/ui}}

### Status Indicators

Single status item:

{{ui:statusitem:Version:info:1.0.0/}}

Multiple items composed into a row:

{{ui:statusitem:Docs:success:complete/}} · {{ui:statusitem:Examples:success:15/}} · {{ui:statusitem:API:info:stable/}}

Different status levels:

{{ui:statusitem:CI:success:passing/}} {{ui:statusitem:Deploy:success:done/}} {{ui:statusitem:Uptime:success:99.9%/}}

## Best Practices

{{ui:callout-github:info}}
**Composition Guidelines**
Use callout-github for important notes and statusitem for project metadata.
Keep callouts concise (2-4 lines ideal).
{{/ui}}

### Callouts

Match callout type to content:

| Type | Use For |
|------|---------|
| `success` | Achievements, completed features |
| `info` | General information, tips |
| `warning` | Breaking changes, deprecations |
| `error` | Critical issues, security notices |

### Status Rows

Keep status rows focused on related metrics. Group similar items together:

{{ui:statusitem:Build:success:✓/}} {{ui:statusitem:Test:success:✓/}} {{ui:statusitem:Deploy:success:✓/}}

## Real-World Examples

### README Header

A typical project README might start with:

{{ui:statusitem:Version:info:2.1.0/}} · {{ui:statusitem:License:info:MIT/}} · {{ui:statusitem:Stars:success:1.2k/}}

{{ui:callout-github:success}}
**New Release**
Version 2.0 brings GitHub blocks, asset manifests, and 3x faster processing!
{{/ui}}

## Component Reference

### callout-github

**Syntax:** Single type argument, multiline content supported

**Types:** `success`, `info`, `warning`, `error`

**Output:** Blockquote with status indicator badge

Callouts support full Markdown inside the content block.

### statusitem

**Syntax:** Three positional args (label, level, text)

**Output:** Status badge plus label plus text

**Example:** `{{ui:statusitem:Build:success:passing/}}`

Compose multiple items manually with ` · ` separator for status rows.

## Troubleshooting

{{ui:callout-github:warning}}
**Common Issues**
If components aren't rendering, check syntax uses colons not equals signs.
Self-closing tags end with slash-double-brace.
{{/ui}}

### Component Not Rendering

Check that you're using the correct positional syntax with colons.

### Blockquote Not Formatting

Ensure you're using `callout-github` (blockquotes) not `callout` (frames).

## Advanced Usage

### Nested Markdown in Callouts

Callouts support lists, bold text, links, and other Markdown:

{{ui:callout-github:info}}
**Feature Highlights**

- Unicode text transformation
- Component-based templates
- Multi-backend rendering

Check the API guide for complete documentation.
{{/ui}}

### Complex Status Rows

{{ui:statusitem:Frontend:success:ready/}} {{ui:statusitem:Backend:success:ready/}} {{ui:statusitem:DB:success:ready/}} · {{ui:statusitem:Deploy:info:staging/}}

## Contributing

{{ui:callout-github:success}}
**We Welcome Contributions**
Found a bug? Have a feature idea? PRs are welcome!
See CONTRIBUTING.md for guidelines.
{{/ui}}

{{ui:statusitem:Issues:info:open/}} · {{ui:statusitem:PRs:success:welcome/}} · {{ui:statusitem:Response:success:< 48h/}}

## License

MIT License - See LICENSE for details.

{{ui:callout-github:info}}
**Note**
This example file is itself a template! Process it with mdfx to see the rendered result.
{{/ui}}

---

**Generated with mdfx** · [Documentation](../docs/API-GUIDE.md) · [GitHub](https://github.com/blackwell-systems/mdfx)
