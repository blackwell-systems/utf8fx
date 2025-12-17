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

| Syntax | Description |
|--------|-------------|
| `{{ui:tech:rust/}}` | Rust badge with orange brand color |
| `{{ui:tech:python/}}` | Python badge with blue brand color |
| `{{ui:tech:typescript/}}` | TypeScript badge with blue brand color |
| `{{ui:tech:docker/}}` | Docker badge with blue brand color |
| `{{ui:tech:postgresql/}}` | PostgreSQL badge with blue brand color |
| `{{ui:tech:go/}}` | Go badge with cyan brand color |

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

```markdown
{{ui:tech:rust:logo=white/}}           <!-- Force white logo -->
{{ui:tech:rust:logo=FFFFFF/}}          <!-- Same, hex format -->
{{ui:tech:docker:logo=000000/}}        <!-- Force black logo -->
{{ui:tech:postgresql:logo=black/}}     <!-- Force black logo -->
```

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

Text color also auto-selects based on the right segment luminance if not specified.

### Font Family

Customize the font with `font` (alias: `font_family`):

```markdown
{{ui:tech:rust:font=monospace/}}
{{ui:tech:python:font=Monaco,Consolas,monospace/}}
{{ui:tech:go:font_family=Arial/}}
{{ui:tech:docker:font=Georgia,serif/}}
```

### Combined Text Styling

```markdown
{{ui:tech:rust:text_color=white:font=monospace/}}
{{ui:tech:postgresql:text=FFFFFF:font=Monaco,monospace/}}
```

---

## Custom Labels

### Override Label Text

Use `label` to customize the displayed text:

```markdown
{{ui:tech:typescript:label=TS/}}           <!-- Short label -->
{{ui:tech:javascript:label=JS/}}           <!-- Short label -->
{{ui:tech:rust:label=Rust 1.75/}}          <!-- Version label -->
{{ui:tech:python:label=Python 3.12/}}      <!-- Version label -->
{{ui:tech:docker:label=Container/}}        <!-- Descriptive label -->
```

### Version Badges

```markdown
{{ui:tech:rust:label=v1.75.0/}}
{{ui:tech:nodejs:label=v20 LTS/}}
{{ui:tech:python:label=3.12/}}
```

### Status Labels

```markdown
{{ui:tech:docker:label=Running/}}
{{ui:tech:postgresql:label=Connected/}}
{{ui:tech:redis:label=Cached/}}
```

---

## Borders & Corners

### Add Borders

Use `border` and `border_width` to add borders:

```markdown
{{ui:tech:rust:border=white/}}
{{ui:tech:rust:border=FFFFFF:border_width=2/}}
{{ui:tech:docker:border=accent:border_width=3/}}
```

### Rounded Corners

Use `rx` to add rounded corners:

```markdown
{{ui:tech:rust:rx=3/}}          <!-- Slightly rounded -->
{{ui:tech:rust:rx=6/}}          <!-- More rounded -->
{{ui:tech:rust:rx=10/}}         <!-- Very rounded -->
```

### Combined Border & Corners

```markdown
{{ui:tech:rust:border=white:border_width=2:rx=4/}}
{{ui:tech:docker:border=accent:rx=6/}}
```

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

### With Versions

```markdown
{{ui:tech:rust:label=Rust 1.75/}}
{{ui:tech:python:label=Python 3.12/}}
{{ui:tech:nodejs:label=Node 20/}}
```

### Monochrome Style

```markdown
{{ui:tech:rust:bg=000000:logo=white/}}
{{ui:tech:python:bg=000000:logo=white/}}
{{ui:tech:docker:bg=000000:logo=white/}}
```

### Custom Branded

```markdown
{{ui:tech:rust:bg=1a1a2e:text_color=F41C80:border=F41C80/}}
{{ui:tech:docker:bg=1a1a2e:text_color=00D4FF:border=00D4FF/}}
```

### Status Dashboard

```markdown
| Service | Tech | Status |
|---------|------|--------|
| API | {{ui:tech:rust:label=Running/}} | ✅ |
| Database | {{ui:tech:postgresql:label=Connected/}} | ✅ |
| Cache | {{ui:tech:redis:label=Healthy/}} | ✅ |
| Queue | {{ui:tech:docker:label=3 containers/}} | ⚠️ |
```

### Minimal Dark Theme

```markdown
{{ui:tech:rust:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}}
{{ui:tech:python:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}}
{{ui:tech:go:bg=292A2D:logo=FFFFFF:text_color=CCCCCC/}}
```

### Full Customization

```markdown
{{ui:tech:rust:bg=1a1a2e:logo=DEA584:text_color=FFFFFF:font=Monaco,monospace:border=DEA584:border_width=1:rx=4/}}
```

---

## Backend Differences

### Shields Backend

Uses shields.io badges. Basic styling only:
- Color and style work
- Custom fonts and borders not supported

```bash
mdfx process template.md --target github
```

### SVG Backend

Full control over all parameters:
- Custom fonts
- Border and corner radius
- Exact color control

```bash
mdfx process template.md --backend svg --assets-dir assets
```

---

## Tips & Tricks

### 1. Use Brand Colors for Consistency

Let the brand colors do the work - they're already optimized for each technology:

```markdown
{{ui:tech:rust/}}  <!-- Orange is Rust's brand -->
{{ui:tech:go/}}    <!-- Cyan is Go's brand -->
```

### 2. Match Logo to Background

When overriding backgrounds, ensure contrast:

```markdown
<!-- Light background → black logo -->
{{ui:tech:docker:bg=FFFFFF:logo=black/}}

<!-- Dark background → white logo -->
{{ui:tech:rust:bg=000000:logo=white/}}
```

### 3. Short Labels for Compact Displays

```markdown
{{ui:tech:typescript:label=TS/}}
{{ui:tech:javascript:label=JS/}}
{{ui:tech:postgresql:label=PG/}}
```

### 4. Version Badges in Tables

```markdown
| Dependency | Version |
|------------|---------|
| {{ui:tech:rust:label=rustc/}} | 1.75.0 |
| {{ui:tech:nodejs:label=node/}} | 20.10.0 |
```

### 5. Monochrome for Professional Docs

```markdown
{{ui:tech:rust:bg=333:logo=white:text_color=white/}}
{{ui:tech:python:bg=333:logo=white:text_color=white/}}
```

---

## See Also

- [Components Guide](COMPONENTS-GUIDE.md) - All UI components
- [Swatch Guide](SWATCH-GUIDE.md) - Color swatch component
- [CLI Guide](CLI-GUIDE.md) - Command line usage
