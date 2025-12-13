# GitHub Blocks Example

This document demonstrates all GitHub-optimized block components in mdfx.
These components are specifically designed to work within GitHub's Markdown constraints
while maintaining clean, semantic markup.

## Installation
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

Install mdfx via cargo:

```bash
cargo install mdfx-cli
```

Or add as a library dependency:

```toml
[dependencies]
mdfx = "1.0"
```

> ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Note**
>
> **Quick Start Tip**
> Process this template file to see the rendered output with shields.io badges.

## Project Status
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Build**: passing · ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Tests**: 217 · ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Coverage**: 94% · ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **License**: MIT

## Features
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

> ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Note**
>
> **What Makes GitHub Blocks Special?**
> These components are designed to work perfectly with GitHub's Markdown renderer.
> No HTML, no CSS, just pure Markdown plus shields.io badges.

Core features of mdfx GitHub blocks:

- **Section Headers**: Automatic dividers for visual separation
- **Blockquote Callouts**: GitHub-compatible callouts with status indicators
- **Status Rows**: Inline status badges for project metadata
- **Composable**: Mix and match blocks in any combination

## Component Gallery
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

### Section Headers

The section component creates visual hierarchy:

## Getting Started
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

Content goes here...

## Advanced Topics
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

More content...

### Blockquote Callouts

Different callout types for different contexts:

> ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Note**
>
> **Information**
> Use info callouts for helpful tips and general information.
> They're perfect for onboarding new users.

> ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Note**
>
> **Success Stories**
> Highlight achievements, completed migrations, or positive outcomes.
> Great for release announcements!

> ![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) **Note**
>
> **Breaking Changes**
> The old API syntax will be deprecated in v2.0.
> Update your code before upgrading.

> ![](https://img.shields.io/badge/-%20-EF4444?style=flat-square) **Note**
>
> **Security Advisory**
> A critical vulnerability was discovered and fixed in v1.0.1.
> Please upgrade immediately.

### Status Indicators

Single status item:

![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Version**: 1.0.0

Multiple items composed into a row:

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Docs**: complete · ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Examples**: 15 · ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **API**: stable

Different status levels:

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **CI**: passing ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Deploy**: done ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Uptime**: 99.9%

## Best Practices
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

> ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Note**
>
> **Composition Guidelines**
> Use section for major headings, callout-github for important notes, and statusitem for project metadata.
> Keep callouts concise (2-4 lines ideal).

### Section Headers

- Use descriptive, action-oriented titles
- Keep titles short (1-3 words)
- Use sentence case, not title case
- Place sections at natural document breaks

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

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Build**: ✓ ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Test**: ✓ ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Deploy**: ✓

## Real-World Examples
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

### README Header

A typical project README might start with:

![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Version**: 2.1.0 · ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **License**: MIT · ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Stars**: 1.2k

> ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Note**
>
> **New Release**
> Version 2.0 brings GitHub blocks, asset manifests, and 3x faster processing!

## Component Reference
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

### section

**Syntax:** Use positional arg for title

**Output:** Markdown header (##) plus visual divider

**Example:** `{{ui:section:Contributing/}}`

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
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

> ![](https://img.shields.io/badge/-%20-EAB308?style=flat-square) **Note**
>
> **Common Issues**
> If components aren't rendering, check syntax uses colons not equals signs.
> Self-closing tags end with slash-double-brace.

### Component Not Rendering

Check that you're using the correct positional syntax with colons.

### Blockquote Not Formatting

Ensure you're using `callout-github` (blockquotes) not `callout` (frames).

## Advanced Usage
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

### Nested Markdown in Callouts

Callouts support lists, bold text, links, and other Markdown:

> ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Note**
>
> **Feature Highlights**
>
> - Unicode text transformation
> - Component-based templates
> - Multi-backend rendering
>
> Check the API guide for complete documentation.

### Complex Status Rows

![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Frontend**: ready ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Backend**: ready ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **DB**: ready · ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Deploy**: staging

## Contributing
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

> ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Note**
>
> **We Welcome Contributions**
> Found a bug? Have a feature idea? PRs are welcome!
> See CONTRIBUTING.md for guidelines.

![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Issues**: open · ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **PRs**: welcome · ![](https://img.shields.io/badge/-%20-22C55E?style=flat-square) **Response**: < 48h

## License
![](https://img.shields.io/badge/-%20-292A2D?style=flat-square)![](https://img.shields.io/badge/-%20-292C34?style=flat-square)![](https://img.shields.io/badge/-%20-F41C80?style=flat-square)![](https://img.shields.io/badge/-%20-282F3C?style=flat-square)

MIT License - See LICENSE for details.

> ![](https://img.shields.io/badge/-%20-3B82F6?style=flat-square) **Note**
>
> **Note**
> This example file is itself a template! Process it with mdfx to see the rendered result.

---

**Generated with mdfx** · [Documentation](../docs/API-GUIDE.md) · [GitHub](https://github.com/blackwell-systems/mdfx)
