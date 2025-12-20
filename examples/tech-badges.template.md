# {{negative-squared}}TECH{{/negative-squared}} Badge Showcase

{{ui:swatch:000000:width=700:height=6:gradient=horizontal/f41c80/3B82F6/22C55E:rx=3/}}

*Comprehensive demonstration of tech badges, text styles, glyphs, and creative combinations*

---

## {{mathbold:separator=dot}}LOGO SIZES{{/mathbold}}

Scale your tech logos from tiny to prominent:

| Size | Preset | Example |
|------|--------|---------|
| Extra Small | `logo_size=xs` | {{ui:tech:rust:logo_size=xs/}} |
| Small | `logo_size=sm` | {{ui:tech:rust:logo_size=sm/}} |
| Medium (default) | `logo_size=md` | {{ui:tech:rust:logo_size=md/}} |
| Large | `logo_size=lg` | {{ui:tech:rust:logo_size=lg/}} |
| Extra Large | `logo_size=xl` | {{ui:tech:rust:logo_size=xl/}} |
| XXL | `logo_size=xxl` | {{ui:tech:rust:logo_size=xxl/}} |

---

## {{negative-circled}}NEW{{/negative-circled}} Custom Icons

Use any SVG path data for unsupported technologies:

{{ui:row:align=center}}
{{ui:tech:custom:label=Quantum:icon=M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5:bg=7C3AED:logo=white/}}
{{ui:tech:custom:label=Neural:icon=M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8zm0-14a6 6 0 1 0 6 6 6 6 0 0 0-6-6z:bg=EC4899:logo=white/}}
{{ui:tech:custom:label=Fusion:icon=M12 2L4 7v10l8 5 8-5V7l-8-5zm0 2.5L17 8v8l-5 3-5-3V8l5-3.5z:bg=F59E0B:logo=white/}}
{{/ui}}

---

## {{squared-latin}}STYLE{{/squared-latin}} Gallery

### Badge Styles

| Style | Result |
|-------|--------|
| `flat` | {{ui:tech:react:style=flat/}} |
| `flat-square` | {{ui:tech:react:style=flat-square/}} |
| `plastic` | {{ui:tech:react:style=plastic/}} |
| `for-the-badge` | {{ui:tech:react:style=for-the-badge/}} |

### Outline Mode {{glyph:star.filled/}}

Ghost-style badges with transparent backgrounds:

{{ui:row:align=center}}
{{ui:tech:rust:style=outline/}} {{ui:tech:python:style=outline/}} {{ui:tech:go:style=outline/}} {{ui:tech:typescript:style=outline/}}
{{/ui}}

{{ui:row:align=center}}
{{ui:tech:docker:style=outline:label=Container/}} {{ui:tech:kubernetes:style=outline:label=K8s/}} {{ui:tech:terraform:style=outline:label=IaC/}}
{{/ui}}

---

## {{mathbold}}BORDERS{{/mathbold}} & {{script}}Corners{{/script}}

### Border Hierarchy

| Mode | Perimeter | Divider | Description |
|------|-----------|---------|-------------|
| `border=COLOR` | Left only | No | Default - borders icon area |
| `border=COLOR:border_full=true` | Full | No | Clean outline around badge |
| `border=COLOR:divider=true` | Left only | Yes | Separator between segments |
| `border=COLOR:border_full=true:divider=true` | Full | Yes | Full outline + separator |
| `style=outline` | Full | Yes | Outline style (auto border + divider) |

### Accent Borders

{{ui:row:align=center}}
{{ui:tech:rust:border=f41c80:border_width=2/}} {{ui:tech:python:border=3B82F6:border_width=2/}} {{ui:tech:go:border=22C55E:border_width=2/}}
{{/ui}}

### Rounded Corners

{{ui:row:align=center}}
{{ui:tech:react:rx=4/}} {{ui:tech:react:rx=8/}} {{ui:tech:react:rx=12/}} {{ui:tech:react:rx=16/}}
{{/ui}}

### Pill Badges

{{ui:row:align=center}}
{{ui:tech:typescript:label=v5.3:rx=20:border=3178C6:border_width=1/}}
{{ui:tech:nodejs:label=v20 LTS:rx=20:border=339933:border_width=1/}}
{{ui:tech:rust:label=v1.80:rx=20:border=CE412B:border_width=1/}}
{{/ui}}

---

## {{fraktur}}Chevron{{/fraktur}} Badges {{glyph:arrow.right/}}

Directional arrow shapes:

### Right Chevron (→)
{{ui:row:align=center}}
{{ui:tech:git:chevron=right/}}{{ui:tech:github:chevron=right/}}{{ui:tech:gitlab:chevron=right/}}{{ui:tech:bitbucket/}}
{{/ui}}

### Left Chevron (←)
{{ui:row:align=center}}
{{ui:tech:amazonaws/}}{{ui:tech:googlecloud:chevron=left/}}{{ui:tech:microsoftazure:chevron=left/}}{{ui:tech:digitalocean:chevron=left/}}
{{/ui}}

### Pipeline Flow
{{ui:row:align=center}}
{{ui:tech:git:label=Code:chevron=right/}}{{ui:tech:githubactions:label=CI:chevron=right/}}{{ui:tech:docker:label=Build:chevron=right/}}{{ui:tech:kubernetes:label=Deploy/}}
{{/ui}}

---

## {{mathbold:separator=star.filled}}TWO SEGMENT{{/mathbold}} Colors

### Independent Segment Colors

{{ui:row:align=center}}
{{ui:tech:rust:label=Backend:bg_left=1a1a1a:bg_right=CE412B/}}
{{ui:tech:typescript:label=Frontend:bg_left=1a1a1a:bg_right=3178C6/}}
{{ui:tech:postgresql:label=Database:bg_left=1a1a1a:bg_right=4169E1/}}
{{/ui}}

### Gradient Feel

{{ui:row:align=center}}
{{ui:tech:react:label=UI:bg_left=20232a:bg_right=61DAFB:logo=61DAFB/}}
{{ui:tech:vue:label=Framework:bg_left=35495e:bg_right=4FC08D:logo=4FC08D/}}
{{ui:tech:svelte:label=Compiler:bg_left=2a1506:bg_right=FF3E00:logo=FF3E00/}}
{{/ui}}

---

## {{negative-squared}}TEXT{{/negative-squared}} + {{circled-latin}}TECH{{/circled-latin}} Combos

### {{frame:gradient}}{{mathbold:separator=dot}}PRIMARY STACK{{/mathbold}}{{/frame}}

{{ui:row:align=center}}
{{ui:tech:rust:logo_size=lg/}} {{ui:tech:typescript:logo_size=lg/}} {{ui:tech:react:logo_size=lg/}} {{ui:tech:postgresql:logo_size=lg/}}
{{/ui}}

### {{frame:solid-left}}{{fraktur}}Backend Services{{/fraktur}}{{/frame}}

{{ui:row:align=center}}
{{ui:tech:rust:label=API:border=CE412B:border_width=2:rx=6/}}
{{ui:tech:redis:label=Cache:border=DC382D:border_width=2:rx=6/}}
{{ui:tech:rabbitmq:label=Queue:border=FF6600:border_width=2:rx=6/}}
{{/ui}}

### {{frame:star}}{{script}}Frontend Magic{{/script}}{{/frame}}

{{ui:row:align=center}}
{{ui:tech:react:style=outline:logo_size=lg/}}
{{ui:tech:tailwindcss:style=outline:logo_size=lg/}}
{{ui:tech:vite:style=outline:logo_size=lg/}}
{{/ui}}

---

## {{glyph:circle.1/}} Status Labels with Text Styles

### {{negative-squared}}PROD{{/negative-squared}} Production Stack

{{ui:row:align=center}}
{{ui:tech:rust:label=v1.80:bg=22C55E:logo=white/}}
{{ui:tech:postgresql:label=v16:bg=22C55E:logo=white/}}
{{ui:tech:redis:label=v7.2:bg=22C55E:logo=white/}}
{{/ui}}

### {{squared-latin}}BETA{{/squared-latin}} Experimental

{{ui:row:align=center}}
{{ui:tech:deno:label=Experimental:border=F59E0B:border_width=2:rx=8/}}
{{ui:tech:bun:label=Testing:border=F59E0B:border_width=2:rx=8/}}
{{/ui}}

---

## {{frame:gradient}}{{mathbold}}RAISED{{/mathbold}}{{/frame}} Icon Badges

Icons that extend above and below the label section:

### Basic Raised

{{ui:row:align=center}}
{{ui:tech:rust:label=Rust:raised=4:bg=DEA584/}}
{{ui:tech:typescript:label=TypeScript:raised=4:bg=3178C6/}}
{{ui:tech:python:label=Python:raised=4:bg=3776AB/}}
{{/ui}}

### Raised with Large Icons

{{ui:row:align=center}}
{{ui:tech:docker:label=Container:raised=6:logo_size=lg:bg=2496ED/}}
{{ui:tech:kubernetes:label=K8s:raised=6:logo_size=lg:bg=326CE5/}}
{{ui:tech:postgresql:label=DB:raised=6:logo_size=lg:bg=4169E1/}}
{{/ui}}

---

## {{negative-squared}}ALL{{/negative-squared}} Parameters Demo

One badge, every parameter:

{{ui:tech:rust:label=Rust v1.80:style=flat-square:bg=1a1a1a:logo=CE412B:text_color=CE412B:border=CE412B:border_width=2:rx=10:logo_size=lg/}}

```
{{ui:tech:rust:
  label=Rust v1.80:
  style=flat-square:
  bg=1a1a1a:
  logo=CE412B:
  text_color=CE412B:
  border=CE412B:
  border_width=2:
  rx=10:
  logo_size=lg/}}
```

---

{{ui:swatch:000000:width=700:height=4:gradient=horizontal/22C55E/3B82F6/f41c80:rx=2/}}

{{glyph:star.filled/}} *Generated with mdfx — where markdown becomes art* {{glyph:star.filled/}}
