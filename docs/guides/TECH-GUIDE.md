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
- [Chevron Badges](#chevron-badges)
- [Independent Segment Colors](#independent-segment-colors)
- [Outline Style](#outline-style)
- [Custom Icons](#custom-icons)
- [Logo Size](#logo-size)
- [Badge Styles](#badge-styles)
- [Tech Groups](#tech-groups)
- [Complete Examples](#complete-examples)
- [Backend Differences](#backend-differences)
- [Tips & Tricks](#tips--tricks)
- [Available Technologies](#available-technologies)

---

## Basic Syntax

```markdown
{{ui:tech:NAME/}}
```

Where `NAME` is a Simple Icons technology name (lowercase, no spaces).

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust/}}` | ![](assets/tech-guide/tech_4d3dc36ab190463c.svg) |
| `{{ui:tech:python/}}` | ![](assets/tech-guide/tech_612cae4b1bd8fe91.svg) |
| `{{ui:tech:typescript/}}` | ![](assets/tech-guide/tech_9fa73146667d5e15.svg) |
| `{{ui:tech:docker/}}` | ![](assets/tech-guide/tech_c10aebf597ab6c36.svg) |
| `{{ui:tech:postgresql/}}` | ![](assets/tech-guide/tech_c5447bd8d58564dc.svg) |
| `{{ui:tech:go/}}` | ![](assets/tech-guide/tech_710a7adb9ff00951.svg) |

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
| `rx` | number | 0 | Corner radius (uniform or comma-separated per-corner) |
| `corners` | preset | none | Corner preset: `left`, `right`, `none`, `all` |
| `chevron` | enum | none | Arrow shape: `left`, `right`, or `both` |
| `bg_left` | color | bg | Left segment (icon area) background color |
| `bg_right` | color | darkened bg | Right segment (label area) background color |
| `source` | enum | svg | Rendering source: `svg` (local file) or `shields` (shields.io URL) |
| `icon` | string | none | Custom SVG path data for unsupported technologies |
| `logo_size` | preset/number | md | Logo size: `xs` (10px), `sm` (12px), `md` (14px), `lg` (16px), `xl` (18px), or custom px |

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

Tech badges automatically use brand colors from Simple Icons. Override with `bg`:

```markdown
{{ui:tech:rust:bg=000000/}}        <!-- Black background -->
{{ui:tech:docker:bg=accent/}}      <!-- Theme accent color -->
{{ui:tech:python:bg=1a1a2e/}}      <!-- Custom dark blue -->
```

**Rendered:**

![](assets/tech-guide/tech_1237c8aa58e9b89d.svg) ![](assets/tech-guide/tech_34f82a292fc3ed60.svg) ![](assets/tech-guide/tech_bf2629c068b2fdad.svg)

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
| `{{ui:tech:rust:logo=white/}}` | ![](assets/tech-guide/tech_601628c5ff022a49.svg) |
| `{{ui:tech:docker:logo=000000/}}` | ![](assets/tech-guide/tech_c10aebf597ab6c36.svg) |
| `{{ui:tech:go:logo=white/}}` | ![](assets/tech-guide/tech_41ff0df4610702f0.svg) |

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

![](assets/tech-guide/tech_26e6cfd450055ea4.svg) ![](assets/tech-guide/tech_26e6cfd450055ea4.svg) ![](assets/tech-guide/tech_baf4bcddc14d2b27.svg) ![](assets/tech-guide/tech_3ee2b5649f7a2368.svg)

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

![](assets/tech-guide/tech_43f9fd273c8ee379.svg) ![](assets/tech-guide/tech_1f0a070cb2d460e1.svg) ![](assets/tech-guide/tech_faf509df3f7b09aa.svg) ![](assets/tech-guide/tech_b43e28e3b3897212.svg)

### Combined Text Styling

```markdown
{{ui:tech:rust:text_color=white:font=monospace/}}
{{ui:tech:postgresql:text=FFFFFF:font=Monaco,monospace/}}
```

**Rendered:**

![](assets/tech-guide/tech_a85573bc3b99b7a1.svg) ![](assets/tech-guide/tech_7cbbeb42ea6baba6.svg)

---

## Custom Labels

### Override Label Text

Use `label` to customize the displayed text:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:typescript:label=TS/}}` | ![](assets/tech-guide/tech_2831da29465584bd.svg) |
| `{{ui:tech:javascript:label=JS/}}` | ![](assets/tech-guide/tech_342f9caaad33cd0e.svg) |
| `{{ui:tech:rust:label=Rust 1.75/}}` | ![](assets/tech-guide/tech_4bbc82163b6c5f5c.svg) |
| `{{ui:tech:python:label=Python 3.12/}}` | ![](assets/tech-guide/tech_dbb238688c778ef2.svg) |
| `{{ui:tech:docker:label=Container/}}` | ![](assets/tech-guide/tech_8d8e623f5a96f56e.svg) |

### Version Badges

```markdown
{{ui:tech:rust:label=v1.75.0/}}
{{ui:tech:nodejs:label=v20 LTS/}}
{{ui:tech:python:label=3.12/}}
```

**Rendered:**

![](assets/tech-guide/tech_b4741c4c4d129345.svg) ![](assets/tech-guide/tech_5c7840bb8bb06279.svg) ![](assets/tech-guide/tech_e9ff6a5c66251301.svg)

### Status Labels

```markdown
{{ui:tech:docker:label=Running/}}
{{ui:tech:postgresql:label=Connected/}}
{{ui:tech:redis:label=Cached/}}
```

**Rendered:**

![](assets/tech-guide/tech_53e24c544590d9f6.svg) ![](assets/tech-guide/tech_a1820d73eb86ed5a.svg) ![](assets/tech-guide/tech_1fdd23a31cc08678.svg)

### Glyph Syntax in Labels

You can embed glyph templates directly inside label values:

```markdown
{{ui:tech:rust:label={{glyph:star.filled/}} Rust/}}
{{ui:tech:docker:label={{glyph:check.yes/}} Running/}}
{{ui:tech:postgresql:label={{glyph:circle.1/}} Primary/}}
```

This is more readable in source while producing the same Unicode output.

**Important Notes:**

- **Direct Unicode works:** Characters like ★, ①, ♥ render correctly
- **Glyph syntax works:** `{{glyph:name/}}` templates expand to Unicode in labels
- **Text styles don't work inline:** Block-style templates like `{{mathbold}}TEXT{{/mathbold}}` aren't supported - use pre-transformed Unicode (𝐑𝐔𝐒𝐓) instead

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

![](assets/tech-guide/tech_94d18fa5c257ecc2.svg) ![](assets/tech-guide/tech_89a9c434dff5847d.svg) ![](assets/tech-guide/tech_ac2abe3dd36d67a1.svg)

### Rounded Corners

Use `rx` to add rounded corners:

```markdown
{{ui:tech:rust:rx=3/}}          <!-- Slightly rounded -->
{{ui:tech:rust:rx=6/}}          <!-- More rounded -->
{{ui:tech:rust:rx=10/}}         <!-- Very rounded -->
```

**Rendered:**

![](assets/tech-guide/tech_4c8bd8a78f6a00df.svg) ![](assets/tech-guide/tech_adb52802fdb7c916.svg) ![](assets/tech-guide/tech_282f1d7873bd30e7.svg)

### Combined Border & Corners

```markdown
{{ui:tech:rust:border=white:border_width=2:rx=4/}}
{{ui:tech:docker:border=accent:rx=6/}}
```

**Rendered:**

![](assets/tech-guide/tech_f9fd032de0c9de6f.svg) ![](assets/tech-guide/tech_ffe256817a6fe4c7.svg)

### Per-Corner Radius

Control individual corners for connected badge groups using `corners` presets or custom `rx` values:

**Presets:**
| Preset | Effect | Use Case |
|--------|--------|----------|
| `corners=left` | Rounded left, square right | First badge in group |
| `corners=right` | Square left, rounded right | Last badge in group |
| `corners=none` | All square | Middle badges |
| `corners=all` | All rounded | Standalone (default) |

```markdown
{{ui:tech:rust:corners=left/}}{{ui:tech:docker:corners=none/}}{{ui:tech:python:corners=right/}}
```

**Rendered (connected badge group):**

![](assets/tech-guide/tech_17e033ae2a1decb0.svg)![](assets/tech-guide/tech_d979be81598c560f.svg)![](assets/tech-guide/tech_5680ee2214af26ab.svg)

**Custom per-corner:** Use `rx=tl,tr,br,bl` (top-left, top-right, bottom-right, bottom-left):

```markdown
{{ui:tech:rust:rx=8,0,0,8/}}     <!-- Same as corners=left with rx=8 -->
{{ui:tech:rust:rx=0,8,8,0/}}     <!-- Same as corners=right with rx=8 -->
```

**Rendered:**

![](assets/tech-guide/tech_470db33614a9f505.svg) ![](assets/tech-guide/tech_28356f248bb4bf5c.svg)

---

## Chevron Badges

Create tab-style badges with pointed arrow shapes using the `chevron` parameter:

| Value | Description | Shape |
|-------|-------------|-------|
| `chevron=left` | Left-pointing arrow | ◁ badge |
| `chevron=right` | Right-pointing arrow | badge ▷ |
| `chevron=both` | Both arrows | ◁ badge ▷ |

### Basic Chevron Examples

```markdown
{{ui:tech:rust:chevron=right/}}      <!-- First in chain -->
{{ui:tech:typescript:chevron=both/}} <!-- Middle badges -->
{{ui:tech:docker:chevron=both/}}     <!-- Middle badges -->
{{ui:tech:postgresql:chevron=left/}} <!-- Last in chain -->
```

Chain chevrons together for a connected tab-bar effect:

```markdown
{{ui:tech:rust:chevron=right/}}{{ui:tech:typescript:chevron=both/}}{{ui:tech:docker:chevron=both/}}{{ui:tech:postgresql:chevron=left/}}
```

### Chevron with Custom Colors

Combine chevrons with custom background colors:

```markdown
{{ui:tech:rust:chevron=right:bg=1a1a2e/}}
{{ui:tech:typescript:chevron=both:bg=2a2a3e/}}
{{ui:tech:postgresql:chevron=left:bg=3a3a4e/}}
```

---

## Independent Segment Colors

Control the left (icon) and right (label) segment colors independently using `bg_left` and `bg_right`:

### Default Behavior

By default, tech badges use:
- **Left segment**: Brand color (or `bg` override)
- **Right segment**: Darkened brand color (15% darker)

### Custom Segment Colors

Override individual segments:

```markdown
{{ui:tech:rust:bg_left=DEA584:bg_right=B8856E/}}
{{ui:tech:typescript:bg_left=3178C6:bg_right=2967A9/}}
{{ui:tech:docker:bg_left=2496ED:bg_right=1E7DC9/}}
```

### Creative Color Combinations

Use contrasting colors for visual impact:

```markdown
{{ui:tech:rust:bg_left=FF6B6B:bg_right=4ECDC4/}}
{{ui:tech:python:bg_left=3776AB:bg_right=FFD43B/}}
```

### With Chevrons

Combine segment colors with chevron shapes:

```markdown
{{ui:tech:rust:chevron=right:bg_left=DEA584:bg_right=B8856E/}}
{{ui:tech:typescript:chevron=both:bg_left=3178C6:bg_right=2967A9/}}
```

---

## Outline Style

Border-only badges with transparent fill, perfect for clean, minimal designs:

```markdown
{{ui:tech:rust:style=outline/}}
{{ui:tech:typescript:style=ghost/}}   <!-- "ghost" is an alias -->
```

The outline style uses the brand color for:
- **Border stroke** - Creates the outline
- **Icon color** - Logo matches the brand color
- **Text color** - Label text matches the brand color

### Outline Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `border` | brand color | Custom border color |
| `border_width` | 2 | Border thickness in pixels |
| `text_color` | brand color | Custom text color |

### Outline Examples

```markdown
{{ui:tech:rust:style=outline/}}
{{ui:tech:typescript:style=outline:border_width=3/}}
{{ui:tech:python:style=outline:border=FF0000/}}
```

Outline badges work great in dark mode or on colored backgrounds where you don't want solid fills.

---

## Custom Icons

For technologies not included in Simple Icons, you can provide custom SVG path data using the `icon` parameter. The path data should be from a 24x24 viewBox SVG.

### Syntax

```markdown
{{ui:tech:mytech:icon=M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5:bg=4A90D9/}}
```

### Finding Icon Paths

1. Get an SVG icon from a source like [Heroicons](https://heroicons.com), [Feather](https://feathericons.com), or design your own
2. Open the SVG file and extract the `d` attribute from the `<path>` element
3. The icon should be designed for a 24x24 viewBox

### Examples

```markdown
<!-- Custom database icon -->
{{ui:tech:mydb:icon=M12 2C6.48 2 2 4.02 2 6.5v11C2 19.98 6.48 22 12 22s10-2.02 10-4.5v-11C22 4.02 17.52 2 12 2:bg=336791:label=MyDB/}}

<!-- Custom cloud icon -->
{{ui:tech:cloud:icon=M19.35 10.04A7.49 7.49 0 0012 4C9.11 4 6.6 5.64 5.35 8.04A5.994 5.994 0 000 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96:bg=2196F3/}}
```

**Note:** When using `icon`, the technology name is only used for the label, not for icon lookup.

---

## Logo Size

Control the icon size within badges using the `logo_size` parameter (alias: `icon_size`).

### Size Presets

| Preset | Size | Use Case |
|--------|------|----------|
| `xs` | 10px | Compact badges, inline text |
| `sm` | 12px | Small badges |
| `md` | 14px | Default size |
| `lg` | 16px | Emphasized badges |
| `xl` | 18px | Large, prominent badges |
| `xxl` | 20px | Extra large badges |

### Syntax

```markdown
{{ui:tech:rust:logo_size=xs/}}    <!-- Extra small -->
{{ui:tech:rust:logo_size=sm/}}    <!-- Small -->
{{ui:tech:rust:logo_size=md/}}    <!-- Medium (default) -->
{{ui:tech:rust:logo_size=lg/}}    <!-- Large -->
{{ui:tech:rust:logo_size=xl/}}    <!-- Extra large -->
{{ui:tech:rust:logo_size=20/}}    <!-- Custom: 20px -->
```

### Examples

Create a size progression:

```markdown
{{ui:tech:rust:logo_size=xs/}} {{ui:tech:rust:logo_size=sm/}} {{ui:tech:rust:logo_size=md/}} {{ui:tech:rust:logo_size=lg/}} {{ui:tech:rust:logo_size=xl/}}
```

Use larger icons for hero sections:

```markdown
{{ui:tech:rust:logo_size=xl:label=Rust Programming/}}
```

**Note:** The badge width automatically adjusts based on the logo size.

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
| `outline` | Border-only with transparent fill |
| `ghost` | Alias for outline |

```markdown
{{ui:tech:rust:style=flat-square/}}     <!-- Default -->
{{ui:tech:rust:style=flat/}}
{{ui:tech:rust:style=plastic/}}
{{ui:tech:rust:style=for-the-badge/}}
{{ui:tech:rust:style=social/}}
{{ui:tech:rust:style=outline/}}
```

**Rendered:**

![](assets/tech-guide/tech_4d3dc36ab190463c.svg) ![](assets/tech-guide/tech_2af318b39ee8334b.svg) ![](assets/tech-guide/tech_c733da3ff65e48b7.svg) ![](assets/tech-guide/tech_3eac9722cea4ae70.svg) ![](assets/tech-guide/tech_bda69193c17addc0.svg)

---

## Raised Icon Badge

The `raised` parameter creates badges where the icon section extends above and below the label section, creating a distinctive 3D tab effect:

```
    ┌────┐
════│icon│════════
    └────┘
```

### Syntax

```markdown
{{ui:tech:rust:label=Rust:raised=4/}}
{{ui:tech:docker:label=Container:raised=6/}}
```

The `raised` value is the number of pixels the icon section extends on each side (top and bottom).

### Use Cases

- **Emphasized badges** - Draw attention to important technologies
- **Hero sections** - Create prominent badge displays
- **Status indicators** - Make key technologies stand out

```markdown
{{ui:tech:rust:label=Primary:raised=4:border=f41c80:border_width=2/}}
{{ui:tech:postgresql:label=Database:raised=6:logo_size=lg/}}
```

---

## Tech Groups

The `tech-group` component automatically applies corner presets for seamless badge groups:

```markdown
{{ui:tech-group}}
{{ui:tech:rust/}}
{{ui:tech:typescript/}}
{{ui:tech:docker/}}
{{/ui}}
```

### How It Works

The component analyzes badges inside and applies:
- **First badge**: `corners=left` (rounded left, square right)
- **Middle badges**: `corners=none` (all square corners)
- **Last badge**: `corners=right` (square left, rounded right)

This creates a seamless "pill" group where badges connect visually.

### Tech Group Examples

**Two technologies:**
```markdown
{{ui:tech-group}}
{{ui:tech:react/}}{{ui:tech:typescript/}}
{{/ui}}
```

**Stack visualization:**
```markdown
{{ui:tech-group}}
{{ui:tech:docker/}}{{ui:tech:kubernetes/}}{{ui:tech:terraform/}}
{{/ui}}
```

### Combining with Row

For centered badge groups, wrap in a row:

```markdown
{{ui:row}}
{{ui:tech-group}}
{{ui:tech:rust/}}{{ui:tech:go/}}{{ui:tech:python/}}
{{/ui}}
{{/ui}}
```

**Note:** Single badges in a tech-group are not modified (all corners remain rounded).

---

## Complete Examples

### Tech Stack Showcase

Display your project's tech stack with brand colors:

```markdown
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:docker/}} {{ui:tech:postgresql/}} {{ui:tech:redis/}}
```

**Rendered:** ![](assets/tech-guide/tech_4d3dc36ab190463c.svg) ![](assets/tech-guide/tech_9fa73146667d5e15.svg) ![](assets/tech-guide/tech_c10aebf597ab6c36.svg) ![](assets/tech-guide/tech_c5447bd8d58564dc.svg) ![](assets/tech-guide/tech_b21a0f458da62fc6.svg)

---

### Neon Cyberpunk Theme

Bright logos on dark backgrounds with matching borders:

```markdown
{{ui:tech:react:bg=0D0D0D:logo=61DAFB:border=61DAFB:border_width=1:rx=3/}}
{{ui:tech:typescript:bg=0D0D0D:logo=3178C6:border=3178C6:border_width=1:rx=3/}}
{{ui:tech:nodejs:bg=0D0D0D:logo=339933:border=339933:border_width=1:rx=3/}}
{{ui:tech:mongodb:bg=0D0D0D:logo=47A248:border=47A248:border_width=1:rx=3/}}
```

**Rendered:**

![](assets/tech-guide/tech_7afa33bf244b19ed.svg) ![](assets/tech-guide/tech_9d1e7d272d4a13a1.svg) ![](assets/tech-guide/tech_6eca811359f744ed.svg) ![](assets/tech-guide/tech_876eb0e45096fd40.svg)

---

### Architecture Layers

Visualize your system architecture:

| Layer | Technologies |
|-------|-------------|
| **Frontend** | ![](assets/tech-guide/tech_e5903984b9196dbf.svg) ![](assets/tech-guide/tech_ef9f089a9a7b6fc5.svg) ![](assets/tech-guide/tech_f425834d91556b6b.svg) |
| **Backend** | ![](assets/tech-guide/tech_620b33b3a6ee192e.svg) ![](assets/tech-guide/tech_b536c8f6f5323486.svg) |
| **Data** | ![](assets/tech-guide/tech_12086181a8203feb.svg) ![](assets/tech-guide/tech_4cd264c979a63f40.svg) |
| **Infrastructure** | ![](assets/tech-guide/tech_c10aebf597ab6c36.svg) ![](assets/tech-guide/tech_64052a715031e06b.svg) ![](assets/tech-guide/tech_cad86b5848b423e5.svg) |

---

### Version Requirements

Show minimum version requirements for your project:

| Dependency | Required | Status |
|------------|----------|--------|
| ![](assets/tech-guide/tech_dcf122d8ba1567c6.svg) | ≥ 1.75.0 | ![](assets/tech-guide/tech_cb90055fa600ffea.svg) |
| ![](assets/tech-guide/tech_c3cf2473b757a130.svg) | ≥ 18.0.0 | ![](assets/tech-guide/tech_b027e58fcabf6a5f.svg) |
| ![](assets/tech-guide/tech_c10aebf597ab6c36.svg) | ≥ 24.0 | ![](assets/tech-guide/tech_a57ab5036ba15cb5.svg) |

---

### Team Roles

Define team responsibilities with tech stacks:

**Frontend Team:**
![](assets/tech-guide/tech_d1bb60f931b78c66.svg) ![](assets/tech-guide/tech_163074ea544e10ba.svg) ![](assets/tech-guide/tech_79a4e916e6c90e79.svg)

**Backend Team:**
![](assets/tech-guide/tech_1211b33f6a972f17.svg) ![](assets/tech-guide/tech_8b8d34b0b62efa24.svg) ![](assets/tech-guide/tech_c2cad5fcf07c01c4.svg)

**DevOps Team:**
![](assets/tech-guide/tech_7bd5f56cb65e9372.svg) ![](assets/tech-guide/tech_841315634c7979ea.svg) ![](assets/tech-guide/tech_23693ec450749563.svg)

---

### Sleek Monochrome

Professional look with consistent dark styling:

```markdown
{{ui:tech:rust:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}
{{ui:tech:typescript:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}
{{ui:tech:docker:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}
{{ui:tech:postgresql:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}
```

**Rendered:**

![](assets/tech-guide/tech_25ea36b6520cb9e0.svg) ![](assets/tech-guide/tech_86f49608d0cd1455.svg) ![](assets/tech-guide/tech_917e8b9d013e7635.svg) ![](assets/tech-guide/tech_fcf39fcb9607b4c.svg)

---

### Gradient Border Effect

Simulate gradients with colored borders on matching dark backgrounds:

```markdown
{{ui:tech:rust:bg=1a0a0a:logo=DEA584:border=DEA584:border_width=2:rx=6/}}
{{ui:tech:python:bg=0a0a1a:logo=3776AB:border=3776AB:border_width=2:rx=6/}}
{{ui:tech:go:bg=0a1a1a:logo=00ADD8:border=00ADD8:border_width=2:rx=6/}}
{{ui:tech:ruby:bg=1a0a0a:logo=CC342D:border=CC342D:border_width=2:rx=6/}}
```

**Rendered:**

![](assets/tech-guide/tech_b98d8fd6327e60cc.svg) ![](assets/tech-guide/tech_454e55bd5f6b01b6.svg) ![](assets/tech-guide/tech_ff06050996c4906c.svg) ![](assets/tech-guide/tech_408300884aadd55e.svg)

---

### Compact Abbreviations

Space-efficient badges for dense layouts:

```markdown
{{ui:tech:typescript:label=TS/}} {{ui:tech:javascript:label=JS/}} {{ui:tech:postgresql:label=PG/}} {{ui:tech:kubernetes:label=K8s/}} {{ui:tech:elasticsearch:label=ES/}}
```

**Rendered:** ![](assets/tech-guide/tech_2831da29465584bd.svg) ![](assets/tech-guide/tech_342f9caaad33cd0e.svg) ![](assets/tech-guide/tech_e4e09f595a9c8901.svg) ![](assets/tech-guide/tech_64052a715031e06b.svg) ![](assets/tech-guide/tech_74573365dbfffc1f.svg)

---

### Inverted Colors

Light backgrounds with dark logos for light-themed docs:

```markdown
{{ui:tech:rust:bg=FFF5EE:logo=000000:text_color=000000/}}
{{ui:tech:docker:bg=E6F3FF:logo=000000:text_color=000000/}}
{{ui:tech:nodejs:bg=E6FFE6:logo=000000:text_color=000000/}}
```

**Rendered:**

![](assets/tech-guide/tech_e49485130e8d0fb0.svg) ![](assets/tech-guide/tech_64cf72de9fe2d3c4.svg) ![](assets/tech-guide/tech_da6031514b345f33.svg)

---

### For The Badge Style

Large, prominent badges for headers:

```markdown
{{ui:tech:rust:style=for-the-badge/}} {{ui:tech:typescript:style=for-the-badge/}} {{ui:tech:docker:style=for-the-badge/}}
```

**Rendered:**

![](assets/tech-guide/tech_3eac9722cea4ae70.svg) ![](assets/tech-guide/tech_9976657f202f84ab.svg) ![](assets/tech-guide/tech_b0c5402df31934d.svg)

---

### Full Customization Showcase

Every parameter in use:

```markdown
{{ui:tech:rust:bg=1a1a2e:logo=DEA584:label=Rust 1.80:text_color=FFFFFF:font=JetBrains Mono,monospace:border=DEA584:border_width=2:rx=8/}}
```

**Rendered:**

![](assets/tech-guide/tech_5509811bace5455e.svg)

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

**Rendered:** ![](assets/tech-guide/tech_4d3dc36ab190463c.svg) ![](assets/tech-guide/tech_710a7adb9ff00951.svg)

### 2. Match Logo to Background

When overriding backgrounds, ensure contrast:

```markdown
<!-- Light background → black logo -->
{{ui:tech:docker:bg=FFFFFF:logo=black/}}

<!-- Dark background → white logo -->
{{ui:tech:rust:bg=000000:logo=white/}}
```

**Rendered:** ![](assets/tech-guide/tech_a24a5597c1f62934.svg) ![](assets/tech-guide/tech_1237c8aa58e9b89d.svg)

### 3. Short Labels for Compact Displays

```markdown
{{ui:tech:typescript:label=TS/}}
{{ui:tech:javascript:label=JS/}}
{{ui:tech:postgresql:label=PG/}}
```

**Rendered:** ![](assets/tech-guide/tech_2831da29465584bd.svg) ![](assets/tech-guide/tech_342f9caaad33cd0e.svg) ![](assets/tech-guide/tech_e4e09f595a9c8901.svg)

### 4. Monochrome for Professional Docs

```markdown
{{ui:tech:rust:bg=333:logo=white:text_color=white/}}
{{ui:tech:python:bg=333:logo=white:text_color=white/}}
```

**Rendered:** ![](assets/tech-guide/tech_6194845c38af3b48.svg) ![](assets/tech-guide/tech_b59a52b8bd8e85da.svg)

---

## Available Technologies

<details>
<summary>Click to expand full list of supported technologies</summary>

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

</details>

---

## See Also

- [Components Guide](COMPONENTS-GUIDE.md) - All UI components
- [Swatch Guide](SWATCH-GUIDE.md) - Color swatch component
- [CLI Guide](CLI-GUIDE.md) - Command line usage
