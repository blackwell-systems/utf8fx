# Version & License Badge Guide

Semantic badges for version numbers and software licenses with automatic color-coding.

## Table of Contents

- [Version Badges](#version-badges)
  - [Basic Syntax](#basic-syntax)
  - [Auto-Detection](#auto-detection)
  - [Status Override](#status-override)
  - [Version Parameters](#version-parameters)
  - [Custom Styling](#custom-styling)
- [License Badges](#license-badges)
  - [License Syntax](#license-syntax)
  - [License Categories](#license-categories)
  - [License Parameters](#license-parameters)
  - [Common Licenses](#common-licenses)
- [Combining with Tech Badges](#combining-with-tech-badges)
- [Tips & Tricks](#tips--tricks)

---

## Version Badges

### Basic Syntax

```markdown
{{ui:version:VERSION/}}
```

The version component automatically detects stability from the version string and applies semantic coloring.

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0/}}` | ![](assets/version-license-guide/swatch_b5903e6df93de08a.svg) |
| `{{ui:version:2.5.3/}}` | ![](assets/version-license-guide/swatch_e73d6eb731346322.svg) |
| `{{ui:version:10.0.0/}}` | ![](assets/version-license-guide/swatch_c050bdbfa66fd35a.svg) |

---

### Auto-Detection

Version strings are automatically parsed to determine status:

#### Stable Versions (Green)

Released production versions (1.x.x and higher):

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0/}}` | ![](assets/version-license-guide/swatch_b5903e6df93de08a.svg) |
| `{{ui:version:3.2.1/}}` | ![](assets/version-license-guide/swatch_29f24d9cb5c465e6.svg) |
| `{{ui:version:12.0.0/}}` | ![](assets/version-license-guide/swatch_cf1da21a9e2df1cc.svg) |

#### Beta Versions (Yellow)

Pre-release testing versions (0.x.x or -beta/-rc suffix):

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:0.9.0/}}` | ![](assets/version-license-guide/swatch_5d4b1bd6a66fbd84.svg) |
| `{{ui:version:2.0.0-beta/}}` | ![](assets/version-license-guide/swatch_9acccc5c652cccc7.svg) |
| `{{ui:version:1.5.0-beta.2/}}` | ![](assets/version-license-guide/swatch_cf68c743712ab6bc.svg) |
| `{{ui:version:3.0.0-rc.1/}}` | ![](assets/version-license-guide/swatch_05ff18b1f7e52759.svg) |
| `{{ui:version:2.0.0-preview/}}` | ![](assets/version-license-guide/swatch_46eda57bd94ecbdb.svg) |

#### Alpha Versions (Orange)

Early development versions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-alpha/}}` | ![](assets/version-license-guide/swatch_cfa17153a02f3d97.svg) |
| `{{ui:version:2.0.0-alpha.3/}}` | ![](assets/version-license-guide/swatch_7b1a809527fe1f13.svg) |

#### Development Versions (Purple)

Unstable development builds:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-dev/}}` | ![](assets/version-license-guide/swatch_57142a90f2fcbbb3.svg) |
| `{{ui:version:2.0.0-snapshot/}}` | ![](assets/version-license-guide/swatch_9aac1c5b025104e0.svg) |
| `{{ui:version:3.0.0-nightly/}}` | ![](assets/version-license-guide/swatch_227e34b9124b0f22.svg) |

#### Deprecated Versions (Red)

End-of-life or unsupported versions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0-deprecated/}}` | ![](assets/version-license-guide/swatch_cbedb484a8a8d855.svg) |
| `{{ui:version:0.5.0-eol/}}` | ![](assets/version-license-guide/swatch_e6c77cb498fb19bd.svg) |

---

### Status Override

Override auto-detection with the `status` parameter:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:status=stable/}}` | ![](assets/version-license-guide/swatch_b5903e6df93de08a.svg) |
| `{{ui:version:1.0.0:status=beta/}}` | ![](assets/version-license-guide/swatch_41161b7cd5883f9a.svg) |
| `{{ui:version:1.0.0:status=alpha/}}` | ![](assets/version-license-guide/swatch_b893506546ea2c6e.svg) |
| `{{ui:version:1.0.0:status=dev/}}` | ![](assets/version-license-guide/swatch_492057cb996cb78e.svg) |
| `{{ui:version:1.0.0:status=deprecated/}}` | ![](assets/version-license-guide/swatch_f929bac19030b184.svg) |

---

### Version Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `version` | string | *required* | Version string (first positional argument) |
| `status` | enum | auto | Override: stable, beta, alpha, dev, deprecated |
| `bg` | color | auto | Custom background color |
| `text` | color | auto | Custom text color |
| `prefix` | string | "v" | Version prefix (use "" to disable) |
| `style` | enum | flat-square | Badge style |

---

### Custom Styling

#### Without Prefix

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:prefix=/}}` | ![](assets/version-license-guide/swatch_fd35f14f01731225.svg) |
| `{{ui:version:2.5.0:prefix=/}}` | ![](assets/version-license-guide/swatch_10cf270530d35e71.svg) |

#### Custom Colors

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:bg=cobalt/}}` | ![](assets/version-license-guide/swatch_3b18a92c6c73492c.svg) |
| `{{ui:version:2.0.0:bg=plum/}}` | ![](assets/version-license-guide/swatch_817027d3d5769d55.svg) |
| `{{ui:version:3.0.0:bg=accent/}}` | ![](assets/version-license-guide/swatch_3291a46b616028da.svg) |

#### Badge Styles

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:style=flat/}}` | ![](assets/version-license-guide/swatch_a295dcc06519763d.svg) |
| `{{ui:version:1.0.0:style=plastic/}}` | ![](assets/version-license-guide/swatch_cc76a4c888a4babb.svg) |
| `{{ui:version:1.0.0:style=for-the-badge/}}` | ![](assets/version-license-guide/swatch_d551cf1f41ecb2fd.svg) |

---

## License Badges

### License Syntax

```markdown
{{ui:license:LICENSE/}}
```

The license component categorizes licenses and applies semantic coloring.

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT/}}` | ![](assets/version-license-guide/swatch_786cabc60dc7e642.svg) |
| `{{ui:license:Apache-2.0/}}` | ![](assets/version-license-guide/swatch_e952000834310ef7.svg) |
| `{{ui:license:GPL-3.0/}}` | ![](assets/version-license-guide/swatch_ee28fc71dbacec07.svg) |

---

### License Categories

#### Permissive Licenses (Green)

Open-source friendly, minimal restrictions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT/}}` | ![](assets/version-license-guide/swatch_786cabc60dc7e642.svg) |
| `{{ui:license:Apache-2.0/}}` | ![](assets/version-license-guide/swatch_e952000834310ef7.svg) |
| `{{ui:license:BSD-3-Clause/}}` | ![](assets/version-license-guide/swatch_d0a378d8ea916e02.svg) |
| `{{ui:license:BSD-2-Clause/}}` | ![](assets/version-license-guide/swatch_fff70b4f7b675b06.svg) |
| `{{ui:license:ISC/}}` | ![](assets/version-license-guide/swatch_41b2c943f98b19aa.svg) |

#### Weak Copyleft (Blue)

File-level copyleft requirements:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:LGPL-3.0/}}` | ![](assets/version-license-guide/swatch_01b1b9c2138a54b7.svg) |
| `{{ui:license:LGPL-2.1/}}` | ![](assets/version-license-guide/swatch_871c20e9a2b63c99.svg) |
| `{{ui:license:MPL-2.0/}}` | ![](assets/version-license-guide/swatch_a67fd56bd32027c7.svg) |
| `{{ui:license:EPL-2.0/}}` | ![](assets/version-license-guide/swatch_27e016de60e384bf.svg) |

#### Copyleft (Yellow)

Strong copyleft requirements:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:GPL-3.0/}}` | ![](assets/version-license-guide/swatch_ee28fc71dbacec07.svg) |
| `{{ui:license:GPL-2.0/}}` | ![](assets/version-license-guide/swatch_45ace2cc3a42af34.svg) |
| `{{ui:license:AGPL-3.0/}}` | ![](assets/version-license-guide/swatch_8f0eb3bc91a53810.svg) |

#### Public Domain (Cyan)

No restrictions:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:CC0/}}` | ![](assets/version-license-guide/swatch_79332b3955a66d15.svg) |
| `{{ui:license:Unlicense/}}` | ![](assets/version-license-guide/swatch_9095adfff894e5ba.svg) |

#### Proprietary (Gray)

Closed source:

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:Proprietary/}}` | ![](assets/version-license-guide/swatch_5758d3e0b9270255.svg) |
| `{{ui:license:Commercial/}}` | ![](assets/version-license-guide/swatch_88b39d90e396f395.svg) |

---

### License Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `license` | string | *required* | License identifier (first positional argument) |
| `label` | string | auto | Custom label (default: formatted license name) |
| `bg` | color | auto | Custom background color |
| `text` | color | auto | Custom text color |
| `style` | enum | flat-square | Badge style |

---

### Common Licenses

Quick reference for popular licenses:

| License | Category | Description |
|---------|----------|-------------|
| MIT | Permissive | Simple, do anything with attribution |
| Apache-2.0 | Permissive | Patent protection, attribution required |
| BSD-3-Clause | Permissive | Attribution, no endorsement |
| GPL-3.0 | Copyleft | Derivatives must be GPL |
| LGPL-3.0 | Weak Copyleft | Library linking exception |
| MPL-2.0 | Weak Copyleft | File-level copyleft |
| AGPL-3.0 | Copyleft | Network use triggers copyleft |
| CC0 | Public Domain | No restrictions whatsoever |

---

### Custom Labels

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT:label=MIT License/}}` | ![](assets/version-license-guide/swatch_742f30635622e632.svg) |
| `{{ui:license:Apache-2.0:label=Apache/}}` | ![](assets/version-license-guide/swatch_8b5ccf4c765de518.svg) |
| `{{ui:license:GPL-3.0:label=GPLv3/}}` | ![](assets/version-license-guide/swatch_1a1b7b375ed4f32a.svg) |

### Custom Colors

| Syntax | Rendered |
|--------|----------|
| `{{ui:license:MIT:bg=cobalt/}}` | ![](assets/version-license-guide/swatch_e7e91268834a3ebe.svg) |
| `{{ui:license:Apache-2.0:bg=plum/}}` | ![](assets/version-license-guide/swatch_af8fbd563deb78fe.svg) |
| `{{ui:license:GPL-3.0:bg=accent/}}` | ![](assets/version-license-guide/swatch_43f1a9a1995737b3.svg) |

---

## Combining with Tech Badges

Create comprehensive project headers with tech, version, and license badges:

### Project Header Example

```markdown
{{ui:tech:rust/}} {{ui:version:1.0.0/}} {{ui:license:MIT/}}
```

![](assets/version-license-guide/tech_9b07f32e2323dccd.svg) ![](assets/version-license-guide/swatch_b5903e6df93de08a.svg) ![](assets/version-license-guide/swatch_786cabc60dc7e642.svg)

### Full Stack Example

```markdown
{{ui:tech:typescript/}} {{ui:tech:react/}} {{ui:version:2.5.0-beta/}} {{ui:license:Apache-2.0/}}
```

![](assets/version-license-guide/tech_b30721c0a0394c2e.svg) ![](assets/version-license-guide/tech_fa93a6b13b34f67b.svg) ![](assets/version-license-guide/swatch_40b0441ea2d03487.svg) ![](assets/version-license-guide/swatch_e952000834310ef7.svg)

### Deprecated Project

```markdown
{{ui:tech:python/}} {{ui:version:0.5.0:status=deprecated/}} {{ui:license:GPL-3.0/}}
```

![](assets/version-license-guide/tech_c5b0cf28158ee95f.svg) ![](assets/version-license-guide/swatch_55a14df18ee1628a.svg) ![](assets/version-license-guide/swatch_ee28fc71dbacec07.svg)

---

## Tips & Tricks

### 1. Use Auto-Detection for Clean Source

Let mdfx detect the version status automatically:

```markdown
<!-- Clean source, smart colors -->
{{ui:version:1.0.0/}}       <!-- Green - stable -->
{{ui:version:0.9.0/}}       <!-- Yellow - 0.x is beta -->
{{ui:version:2.0.0-rc.1/}}  <!-- Yellow - prerelease -->
```

### 2. Version + URL for Releases

Combine with the `url` parameter for clickable release links:

```markdown
<!-- Links to GitHub release -->
{{ui:tech:rust:url=https://github.com/org/repo/}}
```

### 3. Consistent Badge Styles

Use the same style across all badges for visual consistency:

| Consistent Flat | Consistent For-The-Badge |
|-----------------|-------------------------|
| ![](assets/version-license-guide/tech_837edd35922729ee.svg) ![](assets/version-license-guide/swatch_a295dcc06519763d.svg) ![](assets/version-license-guide/swatch_49e5d538189e98d3.svg) | ![](assets/version-license-guide/tech_cd9cdaecb20390d2.svg) ![](assets/version-license-guide/swatch_d551cf1f41ecb2fd.svg) ![](assets/version-license-guide/swatch_92bc676149ce35ad.svg) |

### 4. Custom Colors for Branding

Override default colors to match your project theme:

| Syntax | Rendered |
|--------|----------|
| `{{ui:version:1.0.0:bg=1a1a2e:text=FFFFFF/}}` | ![](assets/version-license-guide/swatch_b75fc7e1e39c6a9c.svg) |
| `{{ui:license:MIT:bg=1a1a2e:text=FFFFFF/}}` | ![](assets/version-license-guide/swatch_2ced75446266bc56.svg) |

---

## See Also

- [Tech Badges](TECH-GUIDE.md) - Technology logo badges
- [Swatches](SWATCH-GUIDE.md) - Color block component
- [Components](COMPONENTS-GUIDE.md) - All UI components
- [Colors](COLORS-GUIDE.md) - Palette reference
