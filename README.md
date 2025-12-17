# 【𝐌𝐃𝐅𝐗】

<p align="center">
<img alt="" src="assets/swatch_8010e28a060480ec.svg"> <img alt="" src="assets/swatch_9548868f54f0a66e.svg"> <img alt="" src="assets/swatch_b4740ff4b229ace7.svg"> <img alt="" src="assets/swatch_e4795ff410c7b4fe.svg">
</p>

**Design for markdown.** Transform templates into styled output with Unicode typography, SVG components, and decorative frames.

**Zero dependencies at runtime.** Generate SVG assets that render on GitHub without shields.io. Your READMEs work offline, forever.

---

## 【At a Glance】

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

29 preconfigured frames with full 531 glyph support. Any symbol becomes a frame.

| Preconfigured | |
|---|---|
| ▓▒░ GRADIENT ░▒▓ | 【LENTICULAR】 |
| ★ STARRED ☆ | « QUOTED » |
| ═══ DOUBLE ═══ | ───  LIGHT  ─── |

| Glyph Frames | |
|---|---|
| ♦ DIAMOND ♦ | ⚡ LIGHTNING ⚡ |
| ☢ RADIOACTIVE ☢ | ♠ SPADES ♠ |

**Syntax:**
```markdown
{{fr:gradient}}TEXT{{//}}             <!-- short form + universal close -->
{{fr:glyph:misc.lightning}}ZAP{{//}}  <!-- any of 531 named glyphs -->
```

`{{//}}` closes any open frame, style, or nested tag.

---

## ◆ Glyphs ◇

531 named Unicode symbols. Access obscure characters by memorable names.

| Category | Examples |
|----------|----------|
| Shapes | ● ○ ◆ ◇ ▲ △ ■ □ |
| Arrows | → ← ↑ ↓ ⇒ ⇐ ⇔ ↔ |
| Checks | ✓ ✔ ✗ ✘ ☐ ☑ ☒ |
| Math | ∞ ≠ ≈ √ ∑ ∫ ± × |
| Blocks | █ ▓ ▒ ░ ▄ ▀ ▌ ▐ |
| Music | ♩ ♪ ♫ ♬ ♭ ♯ |
| Cards | ♠ ♥ ♦ ♣ ♤ ♡ ♢ ♧ |
| Badges | ① ② ③ ❶ ❷ ❸ Ⓐ Ⓑ |

```markdown
{{glyph:check.yes}} Done               <!-- ☑ Done -->
{{glyph:arrow.right}} Next             <!-- → Next -->
{{mathbold:separator=diamond}}TEXT{{//}}  <!-- 𝐓◆𝐄◆𝐗◆𝐓 -->
```

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
<img alt="" src="assets/swatch_d0bde72940f43f46.svg"> <img alt="" src="assets/swatch_9cd99a2098f17346.svg"> <img alt="" src="assets/swatch_91cc34f5d9e2006a.svg"> <img alt="" src="assets/swatch_eb1825990e42203a.svg"> <img alt="" src="assets/swatch_881a50fd59af5e6.svg"> <img alt="" src="assets/swatch_59c524997490f439.svg">
</p>

---

## 𝐔𝐬𝐚𝐠𝐞

```bash
# Generate offline SVG assets (recommended)
mdfx process README.template.md -o README.md --backend svg --assets-dir assets/

# Process with shields.io fallback
mdfx process README.template.md -o README.md

# Multi-target build
mdfx build input.md --all-targets
```

**Backends:**
- `svg` → Local SVG files, commit to repo, no external dependencies
- `shields` → shields.io badges (requires network)
- `plaintext` → Unicode text only

---

## 𝐃𝐨𝐜𝐮𝐦𝐞𝐧𝐭𝐚𝐭𝐢𝐨𝐧

| | |
|---|---|
| [Components](docs/guides/COMPONENTS-GUIDE.md) | UI primitives |
| [Colors](docs/guides/COLORS-GUIDE.md) | Palette system |
| [Progress](docs/guides/PROGRESS-GUIDE.md) | Progress bars |
| [Donut & Gauge](docs/guides/DONUT-GAUGE-GUIDE.md) | Circular charts |
| [Sparklines](docs/guides/SPARKLINE-GUIDE.md) | Inline charts |
| [Text Styles](docs/guides/TEXT-STYLES-GUIDE.md) | 24 styles |
| [Frames](docs/guides/FRAMES-GUIDE.md) | 29 borders |
| [Glyphs](docs/guides/GLYPHS-GUIDE.md) | 531 symbols |
| [CLI](docs/guides/CLI-GUIDE.md) | Commands |

---

▒░ ᴍɪᴛ ʟɪᴄᴇɴꜱᴇ ░▒ · [GitHub](https://github.com/blackwell-systems/mdfx) · [Changelog](CHANGELOG.md)
