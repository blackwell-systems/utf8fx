# ▓︎▒︎░︎ 𝐌·𝐃·𝐅·𝐗 ░︎▒︎▓︎

<p align="center">
<img alt="" src="assets/mdfx/tech_2fd5b06753b427ff.svg"> <img alt="" src="assets/mdfx/swatch_8010e28a060480ec.svg"> <img alt="" src="assets/mdfx/swatch_9548868f54f0a66e.svg"> <img alt="" src="assets/mdfx/swatch_b4740ff4b229ace7.svg"> <img alt="" src="assets/mdfx/swatch_e4795ff410c7b4fe.svg">
</p>

**Design for markdown.** Transform templates into styled output with Unicode typography, SVG components, and decorative frames.

**Zero runtime dependencies.** Generate self-contained SVG assets that render anywhere—no shields.io, no CDNs, no external requests. Your documentation works offline, forever.

---

## 【︎At a Glance】︎

<!-- Neon showcase: manually curated assets, do not regenerate -->
<p align="center">
<img alt="" src="assets/progress_fc9e4dc664048574.svg"> <img alt="" src="assets/donut_2b901d88c0f805d6.svg"> <img alt="" src="assets/gauge_c0f052769cb69ff6.svg"> <img alt="" src="assets/sparkline_2e6de6cd98cb8040.svg">
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
> ![](assets/mdfx/progress_8c3ecedcea0c2152.svg)
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

## ◆︎ Glyphs ◇︎

500+ named Unicode symbols. Use inline or as separators.

| Category | Examples |
|----------|----------|
| Stars | ★︎ ☆︎ ✦︎ ❇︎ |
| Arrows | →︎ ←︎ ↑︎ ↓︎ |
| Shapes | ■︎ ●︎ ○︎ □︎ |
| Cards | ♠︎ ♥︎ ♦︎ ♣︎ |
| Chess | ♔︎ ♕︎ ♖︎ ♘︎ |
| Music | ♪︎ ♫︎ ♭︎ ♯︎ |

**As separators:**

| Syntax | Output |
|--------|--------|
| `{{mathbold:separator=star}}STARS{{/mathbold}}` | 𝐒★𝐓★𝐀★𝐑★𝐒 |
| `{{mathbold:separator=diamond}}DIAMONDS{{/mathbold}}` | 𝐃◆𝐈◆𝐀◆𝐌◆𝐎◆𝐍◆𝐃◆𝐒 |
| `{{mathbold:separator=card.heart}}LOVE{{/mathbold}}` | 𝐋♥𝐎♥𝐕♥𝐄 |

---

## ◆︎ Components ◇︎

### Progress

![](assets/mdfx/progress_c208a891f10f0738.svg)
![](assets/mdfx/progress_8082d7313dc0eb8d.svg)
![](assets/mdfx/progress_a2efa3ea0d546ca5.svg)

### Sliders

![](assets/mdfx/progress_d2043f5bd9b6e887.svg)
![](assets/mdfx/progress_ae8539f9e4f9c40a.svg)

### Circular

<p align="center">
<img alt="" src="assets/mdfx/donut_c1140c443b55c2f6.svg"> <img alt="" src="assets/mdfx/donut_ded6905336546e5e.svg"> <img alt="" src="assets/mdfx/donut_2d302bb2f57f6e6f.svg"> <img alt="" src="assets/mdfx/donut_ae1d50ff3784744f.svg">
</p>

### Gauges

<p align="center">
<img alt="" src="assets/mdfx/gauge_4cc31488da108560.svg"> <img alt="" src="assets/mdfx/gauge_7af13484a7c1cf2c.svg"> <img alt="" src="assets/mdfx/gauge_baca01a9bb105e80.svg">
</p>

### Sparklines

| Type | Chart |
|------|-------|
| Line | ![](assets/mdfx/sparkline_1452d2434945700b.svg) |
| Bar | ![](assets/mdfx/sparkline_e685c6a06807c837.svg) |
| Area | ![](assets/mdfx/sparkline_117137ab3b25ed96.svg) |

### Waveforms

<p align="center">
<img alt="" src="assets/mdfx/waveform_59cff70e7f4ff062.svg">
</p>

<p align="center">
<img alt="" src="assets/mdfx/waveform_75a58f0eee77441d.svg">
</p>

### Swatches

<p align="center">
<img alt="" src="assets/mdfx/swatch_8010e28a060480ec.svg"> <img alt="" src="assets/mdfx/swatch_9548868f54f0a66e.svg"> <img alt="" src="assets/mdfx/swatch_e4795ff410c7b4fe.svg"> <img alt="" src="assets/mdfx/swatch_e666c671e27adcb2.svg"> <img alt="" src="assets/mdfx/swatch_b4740ff4b229ace7.svg"> <img alt="" src="assets/mdfx/swatch_518ded146f6f965a.svg"> <img alt="" src="assets/mdfx/swatch_c056f66b5750e2ba.svg"> <img alt="" src="assets/mdfx/swatch_a9a177f7358a610c.svg">
</p>

**Pixel art** with tiny swatches:

| | | | | | | | |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_c6061e305cb910cc.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|

| | | | | | | |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|
|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_687e4110c4781eb0.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|![](assets/mdfx/swatch_496efc41e8049b7b.svg)|

### Ratings

<p align="center">
<img alt="" src="assets/mdfx/rating_2485a6dc7b105ee1.svg"> <img alt="" src="assets/mdfx/rating_1d06bfc94d8aa7fd.svg"> <img alt="" src="assets/mdfx/rating_40cc3e40be961aee.svg">
</p>

```
{{ui:rating:4.5/}}
{{ui:rating:3.5:icon=heart:fill=error/}}
{{ui:rating:4:icon=circle:fill=info/}}
```

### Tech Badges

Brand-colored badges with Simple Icons. Full customization, no external requests.

<p align="center">
<img alt="" src="assets/mdfx/tech_2fd5b06753b427ff.svg"> <img alt="" src="assets/mdfx/tech_f3bce2794fe6ae29.svg"> <img alt="" src="assets/mdfx/tech_6c08a587dad3b4d3.svg"> <img alt="" src="assets/mdfx/tech_5ac81af210cd3ffe.svg"> <img alt="" src="assets/mdfx/tech_7e8636feb8c1ff0f.svg"> <img alt="" src="assets/mdfx/tech_429de164d996317.svg">
</p>

**Connected badge groups** with per-corner control:

<p align="center">
<img alt="" src="assets/mdfx/tech_36f3fa820a7891b6.svg"><img alt="" src="assets/mdfx/tech_324a7898bc5d0754.svg"><img alt="" src="assets/mdfx/tech_dfbae44840561af1.svg"><img alt="" src="assets/mdfx/tech_2854f3d49806da17.svg">
</p>

```
{{ui:tech:rust:corners=left/}}{{ui:tech:typescript:corners=none/}}{{ui:tech:docker:corners=none/}}{{ui:tech:postgresql:corners=right/}}
```

**Custom styling** — borders, colors, themes:

<p align="center">
<img alt="" src="assets/mdfx/tech_2e907b2455376b1c.svg"> <img alt="" src="assets/mdfx/tech_af89e52a035e33ac.svg"> <img alt="" src="assets/mdfx/tech_ed8e938db212caa2.svg"> <img alt="" src="assets/mdfx/tech_80b543eed3b221de.svg">
</p>

```
{{ui:tech:rust:bg=1a1a2e:border=DEA584:border_width=2/}}
{{ui:tech:python:bg=1a1a2e:border=3776AB:border_width=2/}}
```

**Chevron badges** — pointed tab-style badges that overlap:

<p align="center">
<img alt="" src="assets/mdfx/tech_1a60b131886928dd.svg"><img alt="" src="assets/mdfx/tech_d8962ace30e5e4c3.svg"><img alt="" src="assets/mdfx/tech_f89ed453eca99754.svg"><img alt="" src="assets/mdfx/tech_ec20ec94318a26a2.svg">
</p>

```
{{ui:tech:rust:chevron=first/}}{{ui:tech:typescript:chevron=middle/}}{{ui:tech:docker:chevron=middle/}}{{ui:tech:postgresql:chevron=last/}}
```

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
| [Ratings](docs/guides/RATING-GUIDE.md) | Stars & hearts |
| [Text Styles](docs/guides/TEXT-STYLES-GUIDE.md) | 24 styles |
| [Frames](docs/guides/FRAMES-GUIDE.md) | 29 borders |
| [Glyphs](docs/guides/GLYPHS-GUIDE.md) | 500+ symbols |
| [Tech Badges](docs/guides/TECH-GUIDE.md) | Brand logos |
| [CLI](docs/guides/CLI-GUIDE.md) | Commands |

---

▒︎░︎ ᴍɪᴛ ʟɪᴄᴇɴꜱᴇ ░︎▒︎ · [GitHub](https://github.com/blackwell-systems/mdfx) · [Changelog](CHANGELOG.md)
