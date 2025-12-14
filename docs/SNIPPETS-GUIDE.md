# Snippets Guide

Snippets are reusable template fragments that expand to commonly-used mdfx patterns. They provide shortcuts for frequent operations.

## Basic Syntax

```markdown
{{snippet:name/}}
```

Snippets are self-closing and expand to their template content.

---

## All Snippets

### Separator Snippets

Quick access to colored separator swatches.

| Snippet | Expands To | Description |
|---------|------------|-------------|
| `sep.accent` | `{{ui:swatch:accent/}}` | Accent color separator |
| `sep.success` | `{{ui:swatch:success/}}` | Success color separator |
| `sep.divider` | `{{ui:divider/}}` | Full-width gradient divider |

**Examples:**
```markdown
Item A {{snippet:sep.accent/}} Item B {{snippet:sep.accent/}} Item C

{{snippet:sep.divider/}}
```

**Use case:** Quick inline separators between items without typing full swatch syntax.

---

### Chrome Snippets

Emoji and visual prefixes for status messages.

| Snippet | Expands To | Description |
|---------|------------|-------------|
| `chrome.warning` | ⚠️  | Warning emoji |
| `chrome.info` | ℹ️  | Info emoji |
| `chrome.success` | ✅  | Success emoji |
| `chrome.error` | ❌  | Error emoji |

**Examples:**
```markdown
{{snippet:chrome.warning/}} Proceed with caution
{{snippet:chrome.info/}} More details available
{{snippet:chrome.success/}} Operation complete
{{snippet:chrome.error/}} Something went wrong
```

**Output:**
```
⚠️  Proceed with caution
ℹ️  More details available
✅  Operation complete
❌  Something went wrong
```

---

### Visual Chrome Snippets

Decorative elements for frame prefixes and visual accents.

| Snippet | Expands To | Description |
|---------|------------|-------------|
| `chrome.accent_block` | `{{ui:swatch:accent/}} ` | Accent swatch with trailing space |
| `chrome.gradient` | `▓▒░ ` | Gradient block characters |

**Examples:**
```markdown
{{snippet:chrome.accent_block/}}Important announcement

{{snippet:chrome.gradient/}}Header text
```

---

## Snippet Categories

### Separators (sep.*)

For inline content separation:

```markdown
Tech stack: Rust {{snippet:sep.accent/}} TypeScript {{snippet:sep.accent/}} Go

---

{{snippet:sep.divider/}}

New section starts here
```

---

### Chrome (chrome.*)

For status indicators and visual prefixes:

```markdown
{{snippet:chrome.success/}} All tests passed
{{snippet:chrome.warning/}} 3 deprecation warnings
{{snippet:chrome.error/}} Build failed
{{snippet:chrome.info/}} See documentation for details
```

---

## Practical Examples

### Status Dashboard

```markdown
## Build Status

{{snippet:chrome.success/}} **Frontend**: Build successful
{{snippet:chrome.success/}} **Backend**: All tests passing
{{snippet:chrome.warning/}} **Docs**: 2 broken links
{{snippet:chrome.error/}} **Deploy**: Connection timeout
```

---

### Navigation Bar

```markdown
Home {{snippet:sep.accent/}} Products {{snippet:sep.accent/}} About {{snippet:sep.accent/}} Contact
```

---

### Section Divider

```markdown
## Features

Feature list here...

{{snippet:sep.divider/}}

## Installation

Installation steps here...
```

---

### Alert Messages

```markdown
{{snippet:chrome.info/}} **Note:** This feature requires v2.0 or higher.

{{snippet:chrome.warning/}} **Caution:** This action cannot be undone.

{{snippet:chrome.error/}} **Error:** Invalid configuration detected.
```

---

### Tag Line

```markdown
{{snippet:chrome.accent_block/}}**New Feature** - Dark mode support
```

---

## When to Use Snippets vs Components

| Use Case | Snippet | Component |
|----------|---------|-----------|
| Quick inline separator | `{{snippet:sep.accent/}}` | - |
| Status emoji | `{{snippet:chrome.success/}}` | - |
| Colored block | - | `{{ui:swatch:color/}}` |
| Custom parameters | - | `{{ui:swatch:color:width=100/}}` |
| Full divider | `{{snippet:sep.divider/}}` | `{{ui:divider/}}` |

**Rule of thumb:** Use snippets for quick, common patterns. Use components when you need customization.

---

## Context Compatibility

| Snippet | Inline | Block | Frame Chrome |
|---------|--------|-------|--------------|
| sep.accent | ✓ | - | - |
| sep.success | ✓ | - | - |
| sep.divider | - | ✓ | - |
| chrome.warning | ✓ | - | ✓ |
| chrome.info | ✓ | - | ✓ |
| chrome.success | ✓ | - | ✓ |
| chrome.error | ✓ | - | ✓ |
| chrome.accent_block | - | - | ✓ |
| chrome.gradient | ✓ | - | ✓ |

---

## Tips

1. **Snippets are shortcuts** - They expand to full mdfx syntax
2. **No parameters** - Snippets don't accept customization; use components for that
3. **Inline by default** - Most snippets work inline; only `sep.divider` is block-only
4. **Consistent styling** - Snippets use theme colors from the palette
5. **Composable** - Combine snippets with other mdfx elements freely
