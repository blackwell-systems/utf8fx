# Tech Badge Complete Guide

Tech badges display technology logos with brand colors using Simple Icons. This guide covers every parameter and configuration option.

## Table of Contents

- [Basic Syntax](#basic-syntax)
- [All Parameters](#all-parameters)
- [Brand Colors](#brand-colors)
- [Logo Colors](#logo-colors)
- [Text Customization](#text-customization)
- [Custom Labels](#custom-labels)
- [Borders & Corners](#borders--corners)
- [Badge Styles](#badge-styles)
- [Available Technologies](#available-technologies)
- [Complete Examples](#complete-examples)
- [Backend Differences](#backend-differences)
- [Tips & Tricks](#tips--tricks)

---

## Basic Syntax

```markdown
{{ui:tech:NAME/}}
```

Where `NAME` is a Simple Icons technology name (lowercase, no spaces).

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust/}}` | {{ui:tech:rust/}} |
| `{{ui:tech:python/}}` | {{ui:tech:python/}} |
| `{{ui:tech:typescript/}}` | {{ui:tech:typescript/}} |
| `{{ui:tech:docker/}}` | {{ui:tech:docker/}} |
| `{{ui:tech:postgresql/}}` | {{ui:tech:postgresql/}} |
| `{{ui:tech:go/}}` | {{ui:tech:go/}} |

---

## All Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `name` | string | *required* | Technology name (first positional argument) |
| `style` | enum | flat-square | Badge style (flat, flat-square, plastic, for-the-badge, social) |
| `bg` | color | brand color | Background color override |
| `logo` | color | auto | Logo color (auto-selects black/white based on luminance) |
| `label` | string | name | Label text (defaults to technology name) |
| `text_color` | color | auto | Label text color (aliases: `text`, `color`) |
| `font` | string | Verdana | Font family (alias: `font_family`) |
| `border` | color | none | Border color |
| `border_width` | number | none | Border thickness in pixels |
| `rx` | number | 0 | Corner radius in pixels |
| `source` | enum | svg | Rendering source: `svg` (local file) or `shields` (shields.io URL) |

---

## Rendering Source

By default, tech badges render as local SVG files with full customization support. Use `source=shields` to generate shields.io URLs instead (useful when you can't commit asset files).

```markdown
{{ui:tech:rust/}}                    <!-- Default: SVG file -->
{{ui:tech:rust:source=shields/}}     <!-- shields.io URL -->
```

| Source | Output | Features |
|--------|--------|----------|
| `svg` (default) | Local SVG file | Full customization, borders, corners, fonts |
| `shields` | shields.io URL | No local files, limited features |

**Note:** `source=shields` ignores SVG-only features like `border`, `rx`, `text_color`, and `font`.

---

## Brand Colors

Tech badges automatically use brand colors from Simple Icons:

| Technology | Brand Color | Background |
|------------|-------------|------------|
| rust | `#DEA584` | Orange |
| python | `#3776AB` | Blue |
| typescript | `#3178C6` | Blue |
| javascript | `#F7DF1E` | Yellow |
| go | `#00ADD8` | Cyan |
| docker | `#2496ED` | Blue |
| postgresql | `#4169E1` | Royal Blue |
| redis | `#DC382D` | Red |
| react | `#61DAFB` | Cyan |
| vue | `#4FC08D` | Green |
| nodejs | `#339933` | Green |
| github | `#181717` | Black |

### Override Brand Color

Use `bg` to override the brand color:

```markdown
{{ui:tech:rust:bg=000000/}}        <!-- Black background -->
{{ui:tech:docker:bg=accent/}}      <!-- Theme accent color -->
{{ui:tech:python:bg=1a1a2e/}}      <!-- Custom dark blue -->
```

**Rendered:**

{{ui:tech:rust:bg=000000/}} {{ui:tech:docker:bg=accent/}} {{ui:tech:python:bg=1a1a2e/}}

---

## Logo Colors

### Automatic Selection

Logo colors are automatically selected based on background luminance using ITU-R BT.709:

```
luminance = 0.2126*R + 0.7152*G + 0.0722*B
```

| Background Luminance | Logo Color |
|---------------------|------------|
| > 0.5 (light) | Black (`#000000`) |
| ≤ 0.5 (dark) | White (`#FFFFFF`) |

**Examples:**

| Technology | Background | Logo Color |
|------------|------------|------------|
| Rust | Orange (light) | Black |
| Go | Cyan (light) | Black |
| JavaScript | Yellow (light) | Black |
| Docker | Blue (dark) | White |
| PostgreSQL | Blue (dark) | White |
| GitHub | Black (dark) | White |

### Manual Override

Force a specific logo color with `logo`:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:logo=white/}}` | {{ui:tech:rust:logo=white/}} |
| `{{ui:tech:docker:logo=000000/}}` | {{ui:tech:docker:logo=000000/}} |
| `{{ui:tech:go:logo=white/}}` | {{ui:tech:go:logo=white/}} |

---

## Text Customization

### Text Color

Control the label text color with `text_color` (aliases: `text`, `color`):

```markdown
{{ui:tech:rust:text_color=white/}}      <!-- White text -->
{{ui:tech:rust:text=FFFFFF/}}           <!-- Same, using alias -->
{{ui:tech:rust:color=000000/}}          <!-- Black text -->
{{ui:tech:docker:text_color=accent/}}   <!-- Theme color -->
```

**Rendered:**

{{ui:tech:rust:text_color=white/}} {{ui:tech:rust:text=FFFFFF/}} {{ui:tech:rust:color=000000/}} {{ui:tech:docker:text_color=accent/}}

Text color also auto-selects based on the right segment luminance if not specified.

### Font Family

Customize the font with `font` (alias: `font_family`):

```markdown
{{ui:tech:rust:font=monospace/}}
{{ui:tech:python:font=Monaco,Consolas,monospace/}}
{{ui:tech:go:font_family=Arial/}}
{{ui:tech:docker:font=Georgia,serif/}}
```

**Rendered:**

{{ui:tech:rust:font=monospace/}} {{ui:tech:python:font=Monaco,Consolas,monospace/}} {{ui:tech:go:font_family=Arial/}} {{ui:tech:docker:font=Georgia,serif/}}

### Combined Text Styling

```markdown
{{ui:tech:rust:text_color=white:font=monospace/}}
{{ui:tech:postgresql:text=FFFFFF:font=Monaco,monospace/}}
```

**Rendered:**

{{ui:tech:rust:text_color=white:font=monospace/}} {{ui:tech:postgresql:text=FFFFFF:font=Monaco,monospace/}}

---

## Custom Labels

### Override Label Text

Use `label` to customize the displayed text:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:typescript:label=TS/}}` | {{ui:tech:typescript:label=TS/}} |
| `{{ui:tech:javascript:label=JS/}}` | {{ui:tech:javascript:label=JS/}} |
| `{{ui:tech:rust:label=Rust 1.75/}}` | {{ui:tech:rust:label=Rust 1.75/}} |
| `{{ui:tech:python:label=Python 3.12/}}` | {{ui:tech:python:label=Python 3.12/}} |
| `{{ui:tech:docker:label=Container/}}` | {{ui:tech:docker:label=Container/}} |

### Version Badges

```markdown
{{ui:tech:rust:label=v1.75.0/}}
{{ui:tech:nodejs:label=v20 LTS/}}
{{ui:tech:python:label=3.12/}}
```

**Rendered:**

{{ui:tech:rust:label=v1.75.0/}} {{ui:tech:nodejs:label=v20 LTS/}} {{ui:tech:python:label=3.12/}}

### Status Labels

```markdown
{{ui:tech:docker:label=Running/}}
{{ui:tech:postgresql:label=Connected/}}
{{ui:tech:redis:label=Cached/}}
```

**Rendered:**

{{ui:tech:docker:label=Running/}} {{ui:tech:postgresql:label=Connected/}} {{ui:tech:redis:label=Cached/}}

---

## Borders & Corners

### Add Borders

Use `border` and `border_width` to add borders:

```markdown
{{ui:tech:rust:border=white/}}
{{ui:tech:rust:border=FFFFFF:border_width=2/}}
{{ui:tech:docker:border=accent:border_width=3/}}
```

**Rendered:**

{{ui:tech:rust:border=white/}} {{ui:tech:rust:border=FFFFFF:border_width=2/}} {{ui:tech:docker:border=accent:border_width=3/}}

### Rounded Corners

Use `rx` to add rounded corners:

```markdown
{{ui:tech:rust:rx=3/}}          <!-- Slightly rounded -->
{{ui:tech:rust:rx=6/}}          <!-- More rounded -->
{{ui:tech:rust:rx=10/}}         <!-- Very rounded -->
```

**Rendered:**

{{ui:tech:rust:rx=3/}} {{ui:tech:rust:rx=6/}} {{ui:tech:rust:rx=10/}}

### Combined Border & Corners

```markdown
{{ui:tech:rust:border=white:border_width=2:rx=4/}}
{{ui:tech:docker:border=accent:rx=6/}}
```

**Rendered:**

{{ui:tech:rust:border=white:border_width=2:rx=4/}} {{ui:tech:docker:border=accent:rx=6/}}

---

## Badge Styles

The `style` parameter changes the badge appearance:

| Style | Description |
|-------|-------------|
| `flat-square` | Sharp corners (default) |
| `flat` | Slightly rounded corners |
| `plastic` | Glossy 3D effect |
| `for-the-badge` | Taller, prominent style |
| `social` | Pill/capsule shape |

```markdown
{{ui:tech:rust:style=flat-square/}}     <!-- Default -->
{{ui:tech:rust:style=flat/}}
{{ui:tech:rust:style=plastic/}}
{{ui:tech:rust:style=for-the-badge/}}
{{ui:tech:rust:style=social/}}
```

**Rendered:**

{{ui:tech:rust:style=flat-square/}} {{ui:tech:rust:style=flat/}} {{ui:tech:rust:style=plastic/}} {{ui:tech:rust:style=for-the-badge/}} {{ui:tech:rust:style=social/}}

---

## Available Technologies

### Languages

| Name | Icon |
|------|------|
| `rust` | Rust |
| `python` | Python |
| `typescript` | TypeScript |
| `javascript` | JavaScript |
| `go` | Go |
| `java` | Java |
| `csharp` | C# |
| `cpp` | C++ |
| `ruby` | Ruby |
| `php` | PHP |
| `swift` | Swift |
| `kotlin` | Kotlin |

### Frameworks & Libraries

| Name | Icon |
|------|------|
| `react` | React |
| `vue` | Vue.js |
| `angular` | Angular |
| `svelte` | Svelte |
| `nextjs` | Next.js |
| `nuxt` | Nuxt.js |
| `express` | Express |
| `fastapi` | FastAPI |
| `django` | Django |
| `flask` | Flask |

### Databases

| Name | Icon |
|------|------|
| `postgresql` | PostgreSQL |
| `mysql` | MySQL |
| `mongodb` | MongoDB |
| `redis` | Redis |
| `sqlite` | SQLite |
| `elasticsearch` | Elasticsearch |

### DevOps & Cloud

| Name | Icon |
|------|------|
| `docker` | Docker |
| `kubernetes` | Kubernetes |
| `aws` | AWS |
| `googlecloud` | Google Cloud |
| `azure` | Azure |
| `terraform` | Terraform |
| `github` | GitHub |
| `gitlab` | GitLab |
| `jenkins` | Jenkins |
| `circleci` | CircleCI |

### Tools

| Name | Icon |
|------|------|
| `git` | Git |
| `npm` | npm |
| `yarn` | Yarn |
| `pnpm` | pnpm |
| `vscode` | VS Code |
| `vim` | Vim |
| `neovim` | Neovim |
| `linux` | Linux |
| `macos` | macOS |
| `windows` | Windows |

---

## Complete Examples

### Tech Stack Display

```markdown
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}} {{ui:tech:postgresql/}}
```

**Rendered:** {{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}} {{ui:tech:postgresql/}}

### With Versions

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:label=Rust 1.75/}}` | {{ui:tech:rust:label=Rust 1.75/}} |
| `{{ui:tech:python:label=Python 3.12/}}` | {{ui:tech:python:label=Python 3.12/}} |
| `{{ui:tech:nodejs:label=Node 20/}}` | {{ui:tech:nodejs:label=Node 20/}} |

### Monochrome Style

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg=000000:logo=white/}}` | {{ui:tech:rust:bg=000000:logo=white/}} |
| `{{ui:tech:python:bg=000000:logo=white/}}` | {{ui:tech:python:bg=000000:logo=white/}} |
| `{{ui:tech:docker:bg=000000:logo=white/}}` | {{ui:tech:docker:bg=000000:logo=white/}} |

### Custom Branded

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg=1a1a2e:border=F41C80/}}` | {{ui:tech:rust:bg=1a1a2e:border=F41C80/}} |
| `{{ui:tech:docker:bg=1a1a2e:border=00D4FF/}}` | {{ui:tech:docker:bg=1a1a2e:border=00D4FF/}} |

### Status Dashboard

```markdown
| Service | Tech | Status |
|---------|------|--------|
| API | {{ui:tech:rust:label=Running/}} | ✅ |
| Database | {{ui:tech:postgresql:label=Connected/}} | ✅ |
| Cache | {{ui:tech:redis:label=Healthy/}} | ✅ |
| Queue | {{ui:tech:docker:label=3 containers/}} | ⚠️ |
```

**Rendered:**

| Service | Tech | Status |
|---------|------|--------|
| API | {{ui:tech:rust:label=Running/}} | ✅ |
| Database | {{ui:tech:postgresql:label=Connected/}} | ✅ |
| Cache | {{ui:tech:redis:label=Healthy/}} | ✅ |
| Queue | {{ui:tech:docker:label=3 containers/}} | ⚠️ |

### Minimal Dark Theme

```markdown
{{ui:tech:rust:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}}
{{ui:tech:python:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}}
{{ui:tech:go:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}}
```

**Rendered:**

{{ui:tech:rust:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}} {{ui:tech:python:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}} {{ui:tech:go:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}}

### Full Customization

```markdown
{{ui:tech:rust:bg=1a1a2e:logo=DEA584:text_color=FFFFFF:font=Monaco,monospace:border=DEA584:border_width=1:rx=4/}}
```

**Rendered:**

{{ui:tech:rust:bg=1a1a2e:logo=DEA584:text_color=FFFFFF:font=Monaco,monospace:border=DEA584:border_width=1:rx=4/}}

---

## Backend Differences

### SVG Backend (Default)

Full control over all parameters:
- Custom fonts, borders, and corner radius
- Embedded Simple Icons logos
- Exact color control
- Works offline

```bash
mdfx process template.md --assets-dir assets
```

### shields.io Source

Use `source=shields` for individual badges when you can't commit asset files:

```markdown
{{ui:tech:rust:source=shields/}}
```

Or use the legacy shields backend for the entire document:

```bash
mdfx process template.md --backend shields
```

**Note:** shields.io doesn't support custom fonts, borders, or corner radius.

---

## Tips & Tricks

### 1. Use Brand Colors for Consistency

Let the brand colors do the work - they're already optimized for each technology:

```markdown
{{ui:tech:rust/}}  <!-- Orange is Rust's brand -->
{{ui:tech:go/}}    <!-- Cyan is Go's brand -->
```

**Rendered:** {{ui:tech:rust/}} {{ui:tech:go/}}

### 2. Match Logo to Background

When overriding backgrounds, ensure contrast:

```markdown
<!-- Light background → black logo -->
{{ui:tech:docker:bg=FFFFFF:logo=black/}}

<!-- Dark background → white logo -->
{{ui:tech:rust:bg=000000:logo=white/}}
```

**Rendered:** {{ui:tech:docker:bg=FFFFFF:logo=black/}} {{ui:tech:rust:bg=000000:logo=white/}}

### 3. Short Labels for Compact Displays

```markdown
{{ui:tech:typescript:label=TS/}}
{{ui:tech:javascript:label=JS/}}
{{ui:tech:postgresql:label=PG/}}
```

**Rendered:** {{ui:tech:typescript:label=TS/}} {{ui:tech:javascript:label=JS/}} {{ui:tech:postgresql:label=PG/}}

### 4. Version Badges in Tables

```markdown
| Dependency | Version |
|------------|---------|
| {{ui:tech:rust:label=rustc/}} | 1.75.0 |
| {{ui:tech:nodejs:label=node/}} | 20.10.0 |
```

**Rendered:**

| Dependency | Version |
|------------|---------|
| {{ui:tech:rust:label=rustc/}} | 1.75.0 |
| {{ui:tech:nodejs:label=node/}} | 20.10.0 |

### 5. Monochrome for Professional Docs

```markdown
{{ui:tech:rust:bg=333:logo=white:text_color=white/}}
{{ui:tech:python:bg=333:logo=white:text_color=white/}}
```

**Rendered:** {{ui:tech:rust:bg=333:logo=white:text_color=white/}} {{ui:tech:python:bg=333:logo=white:text_color=white/}}

---

## See Also

- [Components Guide](COMPONENTS-GUIDE.md) - All UI components
- [Swatch Guide](SWATCH-GUIDE.md) - Color swatch component
- [CLI Guide](CLI-GUIDE.md) - Command line usage
