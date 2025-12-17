# ▓︎▒︎░︎ 𝐌·𝐃·𝐅·𝐗 ░︎▒︎▓︎

<p align="center">
<img alt="" src="assets/tech_abfb5c5be62dde98.svg"> <img alt="" src="assets/swatch_8010e28a060480ec.svg"> <img alt="" src="assets/swatch_9548868f54f0a66e.svg"> <img alt="" src="assets/swatch_b4740ff4b229ace7.svg"> <img alt="" src="assets/swatch_e4795ff410c7b4fe.svg">
</p>

**Design for markdown.** Transform templates into styled output with Unicode typography, SVG components, and decorative frames.

**Zero runtime dependencies.** Generate self-contained SVG assets that render anywhere—no shields.io, no CDNs, no external requests. Your documentation works offline, forever.

---

## 【︎At a Glance】︎

<p align="center">
<img alt="" src="assets/progress_fc9e4dc664048574.svg">&nbsp;&nbsp;&nbsp;&nbsp;<img alt="" src="assets/donut_2b901d88c0f805d6.svg">&nbsp;&nbsp;&nbsp;&nbsp;<img alt="" src="assets/gauge_c0f052769cb69ff6.svg">&nbsp;&nbsp;&nbsp;&nbsp;<img alt="" src="assets/sparkline_2e6de6cd98cb8040.svg">
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
> ★︎ FEATURED ☆︎

---

## 𝐈𝐧𝐬𝐭𝐚𝐥𝐥

```bash
cargo install mdfx-cli
```

---

## ◆︎ Typography ◇︎

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

## ◆︎ Frames ◇︎

29 decorative borders. Pure Unicode.

| | |
|---|---|
| ▓︎▒︎░︎ GRADIENT ░︎▒︎▓︎ | 【︎LENTICULAR】︎ |
| ★︎ STARRED ☆︎ | «︎ QUOTED »︎ |
| ▶︎ TRIANGLES ◀︎ | ═︎═︎═︎ DOUBLE ═︎═︎═︎ |
| ☞︎ POINTED ☜︎ | 《︎ANGLED》︎ |

---

## ◆︎ Components ◇︎

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

### Waveforms

![](assets/waveform_7d21d7d64a5780d4.svg)

### Swatches

<p align="center">
<img alt="" src="assets/swatch_8010e28a060480ec.svg"> <img alt="" src="assets/swatch_9548868f54f0a66e.svg"> <img alt="" src="assets/swatch_e4795ff410c7b4fe.svg"> <img alt="" src="assets/swatch_e666c671e27adcb2.svg"> <img alt="" src="assets/swatch_b4740ff4b229ace7.svg"> <img alt="" src="assets/swatch_518ded146f6f965a.svg"> <img alt="" src="assets/swatch_c056f66b5750e2ba.svg"> <img alt="" src="assets/swatch_a9a177f7358a610c.svg">
</p>

### Tech

<p align="center">
<img alt="" src="assets/tech_abfb5c5be62dde98.svg"> <img alt="" src="assets/tech_2fc6eda4a7dfa4d.svg"> <img alt="" src="assets/tech_22f5c369904f37b7.svg"> <img alt="" src="assets/tech_e36c30bc2d511501.svg"> <img alt="" src="assets/tech_7540c9e4510f2af2.svg"> <img alt="" src="assets/tech_bc742128c87c5680.svg">
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

## 𝐃𝐨𝐜𝐮𝐦𝐞𝐧𝐭𝐚𝐭𝐢𝐨𝐧

| | |
|---|---|
| [Components](docs/guides/COMPONENTS-GUIDE.md) | UI primitives |
| [Progress](docs/guides/PROGRESS-GUIDE.md) | Progress bars |
| [Donut & Gauge](docs/guides/DONUT-GAUGE-GUIDE.md) | Circular charts |
| [Sparklines](docs/guides/SPARKLINE-GUIDE.md) | Inline charts |
| [Waveforms](docs/guides/WAVEFORM-GUIDE.md) | Audio viz |
| [Text Styles](docs/guides/TEXT-STYLES-GUIDE.md) | 24 styles |
| [Frames](docs/guides/FRAMES-GUIDE.md) | 29 borders |
| [Glyphs](docs/guides/GLYPHS-GUIDE.md) | 500+ symbols |
| [CLI](docs/guides/CLI-GUIDE.md) | Commands |

---

▒︎░︎ ᴍɪᴛ ʟɪᴄᴇɴꜱᴇ ░︎▒︎ · [GitHub](https://github.com/blackwell-systems/mdfx) · [Changelog](CHANGELOG.md)
