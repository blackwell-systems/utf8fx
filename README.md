# 【𝐌𝐃𝐅𝐗】

<p align="center">
<img alt="" src="assets/tech_280e4169ab654a30.svg"> <img alt="" src="assets/swatch_8010e28a060480ec.svg"> <img alt="" src="assets/swatch_9548868f54f0a66e.svg"> <img alt="" src="assets/swatch_b4740ff4b229ace7.svg"> <img alt="" src="assets/swatch_e4795ff410c7b4fe.svg">
</p>

**Design for markdown.** Transform templates into styled output with Unicode typography, SVG components, and decorative frames.

---

## 【At a Glance】

<p align="center">
<img alt="" src="assets/progress_8194cde2c1037a78.svg"> <img alt="" src="assets/donut_d37f8d60de67187.svg"> <img alt="" src="assets/gauge_d7c8db8eb599f329.svg"> <img alt="" src="assets/sparkline_6c20f5563ba6eebd.svg">
</p>

Write this:
```markdown
{{mathbold}}CHAPTER ONE{{/mathbold}}
{{ui:progress:75/}}
{{frame:star}}FEATURED{{/frame}}
```

Get this:

> 𝐂𝐇𝐀𝐏𝐓𝐄𝐑 𝐎𝐍𝐄
>
> ![](assets/progress_8c3ecedcea0c2152.svg)
>
> ★ FEATURED ☆

---

## 𝐈𝐧𝐬𝐭𝐚𝐥𝐥

```bash
cargo install mdfx-cli
```

---

## ◆ Typography ◇

24 Unicode text styles. No fonts required.

| | |
|---|---|
| 𝐌𝐀𝐓𝐇𝐁𝐎𝐋𝐃 | 𝔉ℜ𝔄𝔎𝔗𝔘ℜ |
| 𝒮𝒞ℛℐ𝒫𝒯 | 𝔻𝕆𝕌𝔹𝕃𝔼𝕊𝕋ℝ𝕌ℂ𝕂 |
| ⓒⓘⓡⓒⓛⓔⓓ | 𝙼𝙾𝙽𝙾𝚂𝙿𝙰𝙲𝙴 |
| ꜱᴍᴀʟʟᴄᴀᴘꜱ | 🅽🅴🅶 |

**Separators and spacing:**

𝐃·𝐎·𝐓·𝐓·𝐄·𝐃 · 𝒜→ℛ→ℛ→𝒪→𝒲→𝒮 · ꜱ   ᴘ   ᴀ   ᴄ   ᴇ   ᴅ

---

## ◆ Frames ◇

29 decorative borders. Pure Unicode.

| | |
|---|---|
| ▓▒░ GRADIENT ░▒▓ | 【LENTICULAR】 |
| ★ STARRED ☆ | « QUOTED » |
| ▶ TRIANGLES ◀ | ═══ DOUBLE ═══ |
| ☞ POINTED ☜ | 《ANGLED》 |

---

## ◆ Components ◇

### Progress

![](assets/progress_c208a891f10f0738.svg)
![](assets/progress_8082d7313dc0eb8d.svg)
![](assets/progress_a2efa3ea0d546ca5.svg)

### Sliders

![](assets/progress_d2043f5bd9b6e887.svg)
![](assets/progress_ae8539f9e4f9c40a.svg)

### Circular

<p align="center">
<img alt="" src="assets/donut_c1140c443b55c2f6.svg"> <img alt="" src="assets/donut_c7793b4e2437a78e.svg"> <img alt="" src="assets/donut_77ee5c20966d473d.svg"> <img alt="" src="assets/donut_ae1d50ff3784744f.svg">
</p>

### Gauges

<p align="center">
<img alt="" src="assets/gauge_4cc31488da108560.svg"> <img alt="" src="assets/gauge_2ed4a5df5d9ef0af.svg"> <img alt="" src="assets/gauge_baca01a9bb105e80.svg">
</p>

### Sparklines

| Type | Chart |
|------|-------|
| Line | ![](assets/sparkline_1452d2434945700b.svg) |
| Bar | ![](assets/sparkline_e685c6a06807c837.svg) |
| Area | ![](assets/sparkline_117137ab3b25ed96.svg) |

### Swatches

<p align="center">
<img alt="" src="assets/swatch_8010e28a060480ec.svg"> <img alt="" src="assets/swatch_9548868f54f0a66e.svg"> <img alt="" src="assets/swatch_e4795ff410c7b4fe.svg"> <img alt="" src="assets/swatch_e666c671e27adcb2.svg"> <img alt="" src="assets/swatch_b4740ff4b229ace7.svg"> <img alt="" src="assets/swatch_518ded146f6f965a.svg"> <img alt="" src="assets/swatch_c056f66b5750e2ba.svg"> <img alt="" src="assets/swatch_5ae9a07e7148661a.svg">
</p>

### Tech

<p align="center">
<img alt="" src="assets/tech_280e4169ab654a30.svg"> <img alt="" src="assets/tech_e0a66117821b4ab4.svg"> <img alt="" src="assets/tech_9403fca3232fbd53.svg"> <img alt="" src="assets/tech_b3176b9aa0bc8dac.svg"> <img alt="" src="assets/tech_2cb657d0c91a4b65.svg"> <img alt="" src="assets/tech_f44b382c3419bb2a.svg">
</p>

---

## 𝐔𝐬𝐚𝐠𝐞

```bash
# Process a template
mdfx process README.template.md -o README.md

# SVG backend for local docs
mdfx process input.md -o output.md --backend svg --assets-dir assets/

# Multi-target build
mdfx build input.md --all-targets
```

**Targets:**
- `github` / `gitlab` → shields.io badges
- `local` → SVG files
- `pypi` → Unicode plaintext

---

## 𝐏𝐚𝐥𝐞𝐭𝐭𝐞

| Token | Hex | |
|-------|-----|---|
| accent | F41C80 | ![](assets/swatch_8010e28a060480ec.svg) |
| success | 22C55E | ![](assets/swatch_9548868f54f0a66e.svg) |
| warning | EAB308 | ![](assets/swatch_e4795ff410c7b4fe.svg) |
| error | EF4444 | ![](assets/swatch_e666c671e27adcb2.svg) |
| info | 3B82F6 | ![](assets/swatch_b4740ff4b229ace7.svg) |
| cobalt | 2B6CB0 | ![](assets/swatch_518ded146f6f965a.svg) |
| plum | 6B46C1 | ![](assets/swatch_c056f66b5750e2ba.svg) |

Custom: `mdfx process --palette brand.json input.md`

---

## 𝐃𝐨𝐜𝐮𝐦𝐞𝐧𝐭𝐚𝐭𝐢𝐨𝐧

| | |
|---|---|
| [Components](docs/guides/COMPONENTS-GUIDE.md) | UI primitives |
| [Progress](docs/guides/PROGRESS-GUIDE.md) | Progress bars |
| [Donut & Gauge](docs/guides/DONUT-GAUGE-GUIDE.md) | Circular charts |
| [Sparklines](docs/guides/SPARKLINE-GUIDE.md) | Inline charts |
| [Text Styles](docs/guides/TEXT-STYLES-GUIDE.md) | 24 styles |
| [Frames](docs/guides/FRAMES-GUIDE.md) | 29 borders |
| [Glyphs](docs/guides/GLYPHS-GUIDE.md) | 500+ symbols |
| [CLI](docs/guides/CLI-GUIDE.md) | Commands |

---

▒░ ᴍɪᴛ ʟɪᴄᴇɴꜱᴇ ░▒ · [GitHub](https://github.com/blackwell-systems/mdfx) · [Changelog](CHANGELOG.md)
