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
- [Badge Links](#badge-links)
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

<details>
<summary>Click to expand parameter reference</summary>

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
| `border_full` | boolean | false | Border around entire badge perimeter |
| `divider` | boolean | false | Vertical line between icon and label segments |
| `rx` | number | 0 | Corner radius (uniform or comma-separated per-corner) |
| `corners` | preset | none | Corner preset: `left`, `right`, `none`, `all` |
| `chevron` | enum | none | Arrow shape: `left`, `right`, or `both` |
| `bg_left` | color | bg | Left segment (icon area) background color |
| `bg_right` | color | darkened bg | Right segment (label area) background color |
| `source` | enum | svg | Rendering source: `svg` (local file) or `shields` (shields.io URL) |
| `icon` | string | none | Custom SVG path data for unsupported technologies |
| `logo_size` | preset/number | md | Logo size: `xs` (10px), `sm` (12px), `md` (14px), `lg` (16px), `xl` (18px), or custom px |
| `url` | string | none | URL to link badge to (wraps in markdown link syntax) |

</details>

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

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg=000000/}}` | ![](assets/tech-guide/tech_1237c8aa58e9b89d.svg) |
| `{{ui:tech:docker:bg=accent/}}` | ![](assets/tech-guide/tech_34f82a292fc3ed60.svg) |
| `{{ui:tech:python:bg=1a1a2e/}}` | ![](assets/tech-guide/tech_bf2629c068b2fdad.svg) |

---

## Logo Colors

Logo colors auto-select based on background luminance (white on dark, black on light). Override with `logo`:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:logo=white/}}` | ![](assets/tech-guide/tech_601628c5ff022a49.svg) |
| `{{ui:tech:docker:logo=000000/}}` | ![](assets/tech-guide/tech_c10aebf597ab6c36.svg) |
| `{{ui:tech:go:logo=white/}}` | ![](assets/tech-guide/tech_41ff0df4610702f0.svg) |

---

## Text Customization

### Text Color

Control the label text color with `text_color` (aliases: `text`, `color`):

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:text_color=white/}}` | ![](assets/tech-guide/tech_26e6cfd450055ea4.svg) |
| `{{ui:tech:rust:text=FFFFFF/}}` | ![](assets/tech-guide/tech_26e6cfd450055ea4.svg) |
| `{{ui:tech:rust:color=000000/}}` | ![](assets/tech-guide/tech_baf4bcddc14d2b27.svg) |
| `{{ui:tech:docker:text_color=accent/}}` | ![](assets/tech-guide/tech_3ee2b5649f7a2368.svg) |

Text color also auto-selects based on the right segment luminance if not specified.

### Font Family

Customize the font with `font` (alias: `font_family`):

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:font=monospace/}}` | ![](assets/tech-guide/tech_43f9fd273c8ee379.svg) |
| `{{ui:tech:python:font=Monaco,Consolas,monospace/}}` | ![](assets/tech-guide/tech_1f0a070cb2d460e1.svg) |
| `{{ui:tech:go:font_family=Arial/}}` | ![](assets/tech-guide/tech_faf509df3f7b09aa.svg) |
| `{{ui:tech:docker:font=Georgia,serif/}}` | ![](assets/tech-guide/tech_b43e28e3b3897212.svg) |

### Combined Text Styling

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:text_color=white:font=monospace/}}` | ![](assets/tech-guide/tech_a85573bc3b99b7a1.svg) |
| `{{ui:tech:postgresql:text=FFFFFF:font=Monaco,monospace/}}` | ![](assets/tech-guide/tech_7cbbeb42ea6baba6.svg) |

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

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:label=v1.75.0/}}` | ![](assets/tech-guide/tech_b4741c4c4d129345.svg) |
| `{{ui:tech:nodejs:label=v20 LTS/}}` | ![](assets/tech-guide/tech_5c7840bb8bb06279.svg) |
| `{{ui:tech:python:label=3.12/}}` | ![](assets/tech-guide/tech_e9ff6a5c66251301.svg) |

### Status Labels

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:docker:label=Running/}}` | ![](assets/tech-guide/tech_53e24c544590d9f6.svg) |
| `{{ui:tech:postgresql:label=Connected/}}` | ![](assets/tech-guide/tech_a1820d73eb86ed5a.svg) |
| `{{ui:tech:redis:label=Cached/}}` | ![](assets/tech-guide/tech_1fdd23a31cc08678.svg) |

### Glyph Syntax in Labels

You can embed glyph templates directly inside label values. This is more readable in source while producing the same Unicode output.

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:label=★ Rust/}}` | ![](assets/tech-guide/tech_f68fef1b32236aa1.svg) |
| `{{ui:tech:docker:label=✓ Running/}}` | ![](assets/tech-guide/tech_215d099096d962cc.svg) |
| `{{ui:tech:postgresql:label=① Primary/}}` | ![](assets/tech-guide/tech_ae518ad0610181a0.svg) |
| `{{ui:tech:redis:label=♥ Cache/}}` | ![](assets/tech-guide/tech_4f6b6e4c5b220845.svg) |

Use `{{glyph:name/}}` syntax in templates for readable source: `{{ui:tech:rust:label={{glyph:star.filled/}} Rust/}}`

**Notes:**

| Feature | Support |
|---------|---------|
| Direct Unicode (★, ①, ♥) | Works |
| Glyph syntax `{{glyph:name/}}` | Expands to Unicode |
| Block styles `{{mathbold}}...{{/mathbold}}` | Not supported - use pre-transformed Unicode (𝐑𝐔𝐒𝐓) |

---

## Borders & Corners

### Add Borders

Use `border` and `border_width` to add borders:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:border=white/}}` | ![](assets/tech-guide/tech_94d18fa5c257ecc2.svg) |
| `{{ui:tech:rust:border=FFFFFF:border_width=2/}}` | ![](assets/tech-guide/tech_89a9c434dff5847d.svg) |
| `{{ui:tech:docker:border=accent:border_width=3/}}` | ![](assets/tech-guide/tech_ac2abe3dd36d67a1.svg) |

### Border Hierarchy

Tech badges support different border modes for fine-grained control:

| Mode | Perimeter | Divider | Description |
|------|-----------|---------|-------------|
| `border=COLOR` | Left only | No | Default - borders icon area |
| `border=COLOR:border_full=true` | Full | No | Clean outline around badge |
| `border=COLOR:divider=true` | Left only | Yes | Separator between segments |
| `border=COLOR:border_full=true:divider=true` | Full | Yes | Full outline + separator |
| `style=outline` | Full | Yes | Outline style (auto border + divider) |

### Rounded Corners

Use `rx` to add rounded corners:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:rx=3/}}` | ![](assets/tech-guide/tech_4c8bd8a78f6a00df.svg) |
| `{{ui:tech:rust:rx=6/}}` | ![](assets/tech-guide/tech_adb52802fdb7c916.svg) |
| `{{ui:tech:rust:rx=10/}}` | ![](assets/tech-guide/tech_282f1d7873bd30e7.svg) |

### Combined Border & Corners

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:border=white:border_width=2:rx=4/}}` | ![](assets/tech-guide/tech_f9fd032de0c9de6f.svg) |
| `{{ui:tech:docker:border=accent:rx=6/}}` | ![](assets/tech-guide/tech_ffe256817a6fe4c7.svg) |

### Per-Corner Radius

Control individual corners for connected badge groups using `corners` presets or custom `rx` values:

| Preset | Effect | Use Case |
|--------|--------|----------|
| `corners=left` | Rounded left, square right | First badge in group |
| `corners=right` | Square left, rounded right | Last badge in group |
| `corners=none` | All square | Middle badges |
| `corners=all` | All rounded | Standalone (default) |

**Connected badge group:**

```markdown
{{ui:tech:rust:corners=left/}}{{ui:tech:docker:corners=none/}}{{ui:tech:python:corners=right/}}
```

![](assets/tech-guide/tech_17e033ae2a1decb0.svg)![](assets/tech-guide/tech_d979be81598c560f.svg)![](assets/tech-guide/tech_5680ee2214af26ab.svg)

**Custom per-corner** (`rx=tl,tr,br,bl`):

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:rx=8,0,0,8/}}` | ![](assets/tech-guide/tech_470db33614a9f505.svg) |
| `{{ui:tech:rust:rx=0,8,8,0/}}` | ![](assets/tech-guide/tech_28356f248bb4bf5c.svg) |

---

## Chevron Badges

Create tab-style badges with pointed arrow shapes using the `chevron` parameter:

| Value | Syntax | Result |
|-------|--------|--------|
| Right arrow | `{{ui:tech:rust:chevron=right/}}` | ![](assets/tech-guide/tech_4c5f16d733e3653.svg) |
| Left arrow | `{{ui:tech:rust:chevron=left/}}` | ![](assets/tech-guide/tech_adc8b3d2d9c0299d.svg) |
| Both arrows | `{{ui:tech:rust:chevron=both/}}` | ![](assets/tech-guide/tech_beee4d4eef0fc1fd.svg) |

### Chained Pipeline

Chain chevrons for a connected tab-bar effect:

![](assets/tech-guide/tech_4c5f16d733e3653.svg)![](assets/tech-guide/tech_3e14e5521023bef8.svg)![](assets/tech-guide/tech_5a747462b1e82e92.svg)![](assets/tech-guide/tech_fa7ca15a0812d6cb.svg)

```markdown
{{ui:tech:rust:chevron=right/}}{{ui:tech:typescript:chevron=both/}}{{ui:tech:docker:chevron=both/}}{{ui:tech:postgresql:chevron=left/}}
```

---

## Independent Segment Colors

Control the left (icon) and right (label) segment colors with `bg_left` and `bg_right`. By default, the right segment is 15% darker than the left.

### Custom Segment Colors

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg_left=DEA584:bg_right=B8856E/}}` | ![](assets/tech-guide/tech_77cb0965d56f00bd.svg) |
| `{{ui:tech:python:bg_left=3776AB:bg_right=FFD43B/}}` | ![](assets/tech-guide/tech_24fa14755759825f.svg) |

### With Chevrons

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:chevron=right:bg_left=DEA584:bg_right=B8856E/}}` | ![](assets/tech-guide/tech_fe00a167f32d408f.svg) |
| `{{ui:tech:typescript:chevron=left:bg_left=3178C6:bg_right=2967A9/}}` | ![](assets/tech-guide/tech_299ba9ffc141a779.svg) |

---

## Outline Style

Border-only badges with transparent fill. Uses brand color for border, icon, and text.

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:style=outline/}}` | ![](assets/tech-guide/tech_24f5e6e760ca333b.svg) |
| `{{ui:tech:typescript:style=outline/}}` | ![](assets/tech-guide/tech_a9004ad74dd1ce01.svg) |
| `{{ui:tech:python:style=outline/}}` | ![](assets/tech-guide/tech_14e6697439dde7ef.svg) |
| `{{ui:tech:docker:style=outline:border_width=3/}}` | ![](assets/tech-guide/tech_a5ef42e473be143d.svg) |

| Parameter | Default | Description |
|-----------|---------|-------------|
| `style` | - | `outline` or `ghost` (alias) |
| `border` | brand color | Custom border color |
| `border_width` | 2 | Border thickness in pixels |

---

## Custom Icons

For technologies not in Simple Icons, provide custom SVG path data via the `icon` parameter (from a 24x24 viewBox).

![](assets/tech-guide/tech_137ce9fa97d45e1d.svg) ![](assets/tech-guide/tech_17c9bee7760f8225.svg)

```markdown
{{ui:tech:mydb:icon=M12 2C6.48 2 2 4.02 2 6.5v11C2 19.98 6.48 22 12 22s10-2.02 10-4.5v-11C22 4.02 17.52 2 12 2:bg=336791:label=MyDB/}}
{{ui:tech:cloud:icon=M19.35 10.04A7.49 7.49 0 0012 4C9.11 4 6.6 5.64 5.35 8.04A5.994 5.994 0 000 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96:bg=2196F3:label=Cloud/}}
```

Extract the `d` attribute from any SVG `<path>` element. Sources: [Heroicons](https://heroicons.com), [Feather](https://feathericons.com).

---

## Logo Size

Control the icon size within badges using the `logo_size` parameter (alias: `icon_size`).

| Preset | Size | Syntax | Result |
|--------|------|--------|--------|
| `xs` | 10px | `{{ui:tech:rust:logo_size=xs/}}` | ![](assets/tech-guide/tech_a6dea7b9e22bd9ef.svg) |
| `sm` | 12px | `{{ui:tech:rust:logo_size=sm/}}` | ![](assets/tech-guide/tech_559f6e1d5f9063ef.svg) |
| `md` | 14px | `{{ui:tech:rust:logo_size=md/}}` | ![](assets/tech-guide/tech_2cd2b404c5fdc9a6.svg) |
| `lg` | 16px | `{{ui:tech:rust:logo_size=lg/}}` | ![](assets/tech-guide/tech_bc1d19c7323981de.svg) |
| `xl` | 18px | `{{ui:tech:rust:logo_size=xl/}}` | ![](assets/tech-guide/tech_b4e69040a819f346.svg) |
| `xxl` | 20px | `{{ui:tech:rust:logo_size=xxl/}}` | ![](assets/tech-guide/tech_3b53bb1ab7701071.svg) |

Custom pixel values also supported: `logo_size=20` for 20px.

---

## Badge Styles

The `style` parameter changes the badge appearance:

| Style | Syntax | Rendered |
|-------|--------|----------|
| `flat-square` | `{{ui:tech:rust:style=flat-square/}}` | ![](assets/tech-guide/tech_4d3dc36ab190463c.svg) |
| `flat` | `{{ui:tech:rust:style=flat/}}` | ![](assets/tech-guide/tech_2af318b39ee8334b.svg) |
| `plastic` | `{{ui:tech:rust:style=plastic/}}` | ![](assets/tech-guide/tech_c733da3ff65e48b7.svg) |
| `for-the-badge` | `{{ui:tech:rust:style=for-the-badge/}}` | ![](assets/tech-guide/tech_3eac9722cea4ae70.svg) |
| `social` | `{{ui:tech:rust:style=social/}}` | ![](assets/tech-guide/tech_bda69193c17addc0.svg) |
| `outline` | `{{ui:tech:rust:style=outline/}}` | ![](assets/tech-guide/tech_24f5e6e760ca333b.svg) |
| `ghost` | Alias for `outline` | — |

---

## Badge Links

Make badges clickable with the `url` parameter. The badge image is wrapped in a markdown link.

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:url=https://rust-lang.org/}}` | [![](assets/tech-guide/tech_4d3dc36ab190463c.svg)](https://rust-lang.org/) |
| `{{ui:tech:docker:url=https://docker.com/}}` | [![](assets/tech-guide/tech_c10aebf597ab6c36.svg)](https://docker.com/) |
| `{{ui:tech:python:url=https://python.org/}}` | [![](assets/tech-guide/tech_612cae4b1bd8fe91.svg)](https://python.org/) |

### Use Cases

**Project homepage links in README:**

```markdown
{{ui:tech:rust:url=https://github.com/your/rust-project/}}
{{ui:tech:typescript:url=https://github.com/your/ts-project/}}
```

**Documentation links:**

```markdown
{{ui:tech:docker:url=https://docs.docker.com/}}
{{ui:tech:kubernetes:url=https://kubernetes.io/docs/}}
```

**Download/install pages:**

```markdown
{{ui:tech:rust:label=Install:url=https://rustup.rs/}}
{{ui:tech:nodejs:label=Download:url=https://nodejs.org/}}
```

### Output Format

The `url` parameter wraps the badge in markdown link syntax:

```markdown
<!-- Input -->
{{ui:tech:rust:url=https://rust-lang.org/}}

<!-- SVG output -->
[![](assets/tech_xxx.svg)](https://rust-lang.org/)

<!-- shields.io output (source=shields) -->
[![](https://img.shields.io/...)](https://rust-lang.org/)
```

---

## Raised Icon Badge

The `raised` parameter creates badges where the icon section extends above and below the label section. The value is the number of pixels to extend on each side.

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:label=Rust:raised=4/}}` | ![](assets/tech-guide/tech_f895c7a30580a635.svg) |
| `{{ui:tech:docker:label=Container:raised=6/}}` | ![](assets/tech-guide/tech_5f9dfb9503acf669.svg) |
| `{{ui:tech:postgresql:label=Database:raised=6:logo_size=lg/}}` | ![](assets/tech-guide/tech_c620466fae625ee5.svg) |

---

## Tech Groups

The `tech-group` component creates seamless badge groups by auto-applying corner presets (first=left, middle=none, last=right).

**Supported badge types:** `tech`, `version`, `license`

![](assets/tech-guide/tech_4b359f770f12d88f.svg)![](assets/tech-guide/tech_d507b8eab2a8d265.svg)![](assets/tech-guide/tech_5dc98f9c998100d2.svg)

```markdown
{{ui:tech-group}}
{{ui:tech:rust/}}{{ui:tech:typescript/}}{{ui:tech:docker/}}
{{/ui}}
```

### Mixed Badge Types

Combine version, tech, and license badges in one seamless pill:

```markdown
{{ui:tech-group:bg=1a1a2e:border=333}}
{{ui:version:1.2.0/}}{{ui:tech:rust/}}{{ui:tech:docker/}}{{ui:license:MIT/}}
{{/ui}}
```

### More Examples

![](assets/tech-guide/tech_ec4dda8d6c53178a.svg)![](assets/tech-guide/tech_e65b3824eab484b7.svg) | ![](assets/tech-guide/tech_8856d74e9e5f5137.svg)![](assets/tech-guide/tech_b65657c7069e8137.svg)![](assets/tech-guide/tech_35b10ae14aef1ea4.svg)

### Style Inheritance

Style the group once and all child badges inherit those styles. Individual badges can override specific params.

**All params are inheritable** - any param set on the group passes to children (unless overridden).

```markdown
<!-- Dark theme group - all badges inherit bg and border -->
{{ui:tech-group:bg=1a1a2e:border=333}}
{{ui:tech:rust/}}{{ui:tech:go/}}{{ui:tech:python/}}
{{/ui}}

<!-- Neon border group -->
{{ui:tech-group:border=00ff00:border_width=2}}
{{ui:tech:docker/}}{{ui:tech:kubernetes/}}
{{/ui}}

<!-- Override on specific badge -->
{{ui:tech-group:bg=1a1a2e}}
{{ui:tech:rust/}}{{ui:tech:go:bg=00ADD8/}}{{ui:tech:python/}}
{{/ui}}
```

In the last example, `rust` and `python` inherit `bg=1a1a2e`, but `go` uses its own `bg=00ADD8`.

---

## Complete Examples

### Tech Stack Showcase

Display your project's tech stack with brand colors:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust/}}` | ![](assets/tech-guide/tech_4d3dc36ab190463c.svg) |
| `{{ui:tech:typescript/}}` | ![](assets/tech-guide/tech_9fa73146667d5e15.svg) |
| `{{ui:tech:docker/}}` | ![](assets/tech-guide/tech_c10aebf597ab6c36.svg) |
| `{{ui:tech:postgresql/}}` | ![](assets/tech-guide/tech_c5447bd8d58564dc.svg) |
| `{{ui:tech:redis/}}` | ![](assets/tech-guide/tech_b21a0f458da62fc6.svg) |

---

### Neon Cyberpunk Theme

Bright logos on dark backgrounds with matching borders:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:react:bg=0D0D0D:logo=61DAFB:border=61DAFB:border_width=1:rx=3/}}` | ![](assets/tech-guide/tech_7afa33bf244b19ed.svg) |
| `{{ui:tech:typescript:bg=0D0D0D:logo=3178C6:border=3178C6:border_width=1:rx=3/}}` | ![](assets/tech-guide/tech_9d1e7d272d4a13a1.svg) |
| `{{ui:tech:nodejs:bg=0D0D0D:logo=339933:border=339933:border_width=1:rx=3/}}` | ![](assets/tech-guide/tech_6eca811359f744ed.svg) |
| `{{ui:tech:mongodb:bg=0D0D0D:logo=47A248:border=47A248:border_width=1:rx=3/}}` | ![](assets/tech-guide/tech_876eb0e45096fd40.svg) |

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

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}` | ![](assets/tech-guide/tech_25ea36b6520cb9e0.svg) |
| `{{ui:tech:typescript:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}` | ![](assets/tech-guide/tech_86f49608d0cd1455.svg) |
| `{{ui:tech:docker:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}` | ![](assets/tech-guide/tech_917e8b9d013e7635.svg) |
| `{{ui:tech:postgresql:bg=18181b:logo=a1a1aa:text_color=a1a1aa/}}` | ![](assets/tech-guide/tech_fcf39fcb9607b4c.svg) |

---

### Gradient Border Effect

Simulate gradients with colored borders on matching dark backgrounds:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg=1a0a0a:logo=DEA584:border=DEA584:border_width=2:rx=6/}}` | ![](assets/tech-guide/tech_b98d8fd6327e60cc.svg) |
| `{{ui:tech:python:bg=0a0a1a:logo=3776AB:border=3776AB:border_width=2:rx=6/}}` | ![](assets/tech-guide/tech_454e55bd5f6b01b6.svg) |
| `{{ui:tech:go:bg=0a1a1a:logo=00ADD8:border=00ADD8:border_width=2:rx=6/}}` | ![](assets/tech-guide/tech_ff06050996c4906c.svg) |
| `{{ui:tech:ruby:bg=1a0a0a:logo=CC342D:border=CC342D:border_width=2:rx=6/}}` | ![](assets/tech-guide/tech_408300884aadd55e.svg) |

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

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg=FFF5EE:logo=000000:text_color=000000/}}` | ![](assets/tech-guide/tech_e49485130e8d0fb0.svg) |
| `{{ui:tech:docker:bg=E6F3FF:logo=000000:text_color=000000/}}` | ![](assets/tech-guide/tech_64cf72de9fe2d3c4.svg) |
| `{{ui:tech:nodejs:bg=E6FFE6:logo=000000:text_color=000000/}}` | ![](assets/tech-guide/tech_da6031514b345f33.svg) |

---

### For The Badge Style

Large, prominent badges for headers:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:style=for-the-badge/}}` | ![](assets/tech-guide/tech_3eac9722cea4ae70.svg) |
| `{{ui:tech:typescript:style=for-the-badge/}}` | ![](assets/tech-guide/tech_9976657f202f84ab.svg) |
| `{{ui:tech:docker:style=for-the-badge/}}` | ![](assets/tech-guide/tech_b0c5402df31934d.svg) |

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

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust/}}` | ![](assets/tech-guide/tech_4d3dc36ab190463c.svg) |
| `{{ui:tech:go/}}` | ![](assets/tech-guide/tech_710a7adb9ff00951.svg) |

### 2. Match Logo to Background

When overriding backgrounds, ensure contrast:

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:docker:bg=FFFFFF:logo=black/}}` | ![](assets/tech-guide/tech_a24a5597c1f62934.svg) |
| `{{ui:tech:rust:bg=000000:logo=white/}}` | ![](assets/tech-guide/tech_1237c8aa58e9b89d.svg) |

### 3. Short Labels for Compact Displays

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:typescript:label=TS/}}` | ![](assets/tech-guide/tech_2831da29465584bd.svg) |
| `{{ui:tech:javascript:label=JS/}}` | ![](assets/tech-guide/tech_342f9caaad33cd0e.svg) |
| `{{ui:tech:postgresql:label=PG/}}` | ![](assets/tech-guide/tech_e4e09f595a9c8901.svg) |

### 4. Monochrome for Professional Docs

| Syntax | Rendered |
|--------|----------|
| `{{ui:tech:rust:bg=333:logo=white:text_color=white/}}` | ![](assets/tech-guide/tech_6194845c38af3b48.svg) |
| `{{ui:tech:python:bg=333:logo=white:text_color=white/}}` | ![](assets/tech-guide/tech_b59a52b8bd8e85da.svg) |

---

## Available Technologies

<details>
<summary>Click to expand full list of 90+ supported technologies</summary>

### Languages

| Name | Aliases | Description |
|------|---------|-------------|
| `rust` | | Rust |
| `python` | | Python |
| `typescript` | | TypeScript |
| `javascript` | | JavaScript |
| `go` | `golang` | Go |
| `java` | `openjdk` | Java |
| `c` | | C |
| `cpp` | `cplusplus`, `c++` | C++ |
| `ruby` | | Ruby |
| `php` | | PHP |
| `swift` | | Swift |
| `kotlin` | | Kotlin |
| `scala` | | Scala |
| `elixir` | | Elixir |
| `dart` | | Dart |
| `zig` | | Zig |

### Frontend Frameworks

| Name | Aliases | Description |
|------|---------|-------------|
| `react` | | React |
| `vue` | `vuejs`, `vue.js` | Vue.js |
| `angular` | | Angular |
| `svelte` | | Svelte |
| `nextjs` | `next.js` | Next.js |
| `nuxt` | `nuxtjs` | Nuxt |
| `astro` | | Astro |
| `vite` | | Vite |
| `tailwindcss` | `tailwind` | Tailwind CSS |
| `sass` | | Sass |

### Backend Frameworks

| Name | Aliases | Description |
|------|---------|-------------|
| `nodejs` | `node.js` | Node.js |
| `express` | | Express |
| `fastapi` | | FastAPI |
| `django` | | Django |
| `flask` | | Flask |
| `rails` | `rubyonrails` | Ruby on Rails |
| `spring` | | Spring |
| `dotnet` | `.net` | .NET |
| `nestjs` | | NestJS |

### Databases

| Name | Aliases | Description |
|------|---------|-------------|
| `postgresql` | | PostgreSQL |
| `mysql` | | MySQL |
| `mongodb` | | MongoDB |
| `redis` | | Redis |
| `sqlite` | | SQLite |
| `elasticsearch` | | Elasticsearch |
| `neo4j` | | Neo4j |
| `supabase` | | Supabase |
| `firebase` | | Firebase |

### Cloud Providers

| Name | Aliases | Description |
|------|---------|-------------|
| `amazonaws` | `aws` | Amazon Web Services |
| `googlecloud` | `gcp` | Google Cloud |
| `vercel` | | Vercel |
| `netlify` | | Netlify |
| `cloudflare` | | Cloudflare |
| `digitalocean` | | DigitalOcean |

### DevOps & Infrastructure

| Name | Aliases | Description |
|------|---------|-------------|
| `docker` | | Docker |
| `kubernetes` | | Kubernetes |
| `terraform` | | Terraform |
| `nginx` | | Nginx |
| `ansible` | | Ansible |
| `jenkins` | | Jenkins |
| `circleci` | | CircleCI |
| `githubactions` | | GitHub Actions |
| `prometheus` | | Prometheus |
| `grafana` | | Grafana |

### DevOps Extended

| Name | Aliases | Description |
|------|---------|-------------|
| `pulumi` | | Pulumi (IaC) |
| `vagrant` | | Vagrant |
| `helm` | | Helm |
| `argo` | `argocd` | Argo CD |
| `consul` | | HashiCorp Consul |
| `vault` | | HashiCorp Vault |
| `datadog` | | Datadog |
| `sentry` | | Sentry |
| `newrelic` | | New Relic |

### Testing

| Name | Aliases | Description |
|------|---------|-------------|
| `vitest` | | Vitest |
| `cypress` | | Cypress |
| `selenium` | | Selenium |
| `mocha` | | Mocha |

### AI/ML

| Name | Aliases | Description |
|------|---------|-------------|
| `tensorflow` | | TensorFlow |
| `pytorch` | | PyTorch |
| `huggingface` | | Hugging Face |
| `jupyter` | | Jupyter |
| `pandas` | | pandas |
| `numpy` | | NumPy |

### Runtime/Package Managers

| Name | Aliases | Description |
|------|---------|-------------|
| `pnpm` | | pnpm |
| `bun` | | Bun |
| `deno` | | Deno |

### Tools

| Name | Aliases | Description |
|------|---------|-------------|
| `git` | | Git |
| `github` | | GitHub |
| `gitlab` | | GitLab |
| `bitbucket` | | Bitbucket |
| `npm` | | npm |
| `yarn` | | Yarn |
| `webpack` | | Webpack |
| `eslint` | | ESLint |
| `prettier` | | Prettier |
| `jest` | | Jest |

### Platforms

| Name | Aliases | Description |
|------|---------|-------------|
| `linux` | | Linux |
| `apple` | | Apple/macOS |
| `android` | | Android |
| `figma` | | Figma |
| `discord` | | Discord |

</details>

---

## See Also

- [Components Guide](COMPONENTS-GUIDE.md) - All UI components
- [Swatch Guide](SWATCH-GUIDE.md) - Color swatch component
- [CLI Guide](CLI-GUIDE.md) - Command line usage
