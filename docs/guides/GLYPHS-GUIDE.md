# Glyphs Guide

Named Unicode characters for use in templates, separators, and frames.

## Quick Navigation

| Category | Sections |
|----------|----------|
| **Basics** | [Separators](#separators) · [Block Elements](#block-elements) · [Shades](#shades) · [Quadrants](#quadrants) |
| **Drawing** | [Box Drawing](#box-drawing) · [Braille Patterns](#braille-patterns) |
| **Numbers** | [Number Badges](#number-badges) · [Roman Numerals](#roman-numerals) · [Fractions](#fractions) |
| **Shapes** | [Squares](#shapes) · [Circles](#shapes) · [Triangles](#shapes) · [Diamonds](#shapes) |
| **Symbols** | [Checkboxes](#checkboxes) · [Arrows](#arrows) · [Stars](#stars) |
| **Games** | [Dice](#dice) · [Card Suits](#card-suits) |
| **Music** | [Music](#music) |
| **Math** | [Math Symbols](#math-symbols) · [Superscript & Subscript](#superscript--subscript) · [Greek Letters](#greek-letters) |
| **Other** | [Currency](#currency) · [Miscellaneous](#miscellaneous-symbols) |

---

## Syntax

```markdown
{{glyph:name/}}
```

Or as a separator in style templates:

```markdown
{{mathbold:separator=dot}}HELLO{{/mathbold}}
```

---

## Available Glyphs

### Separators

Short names for common inline characters:

| Name | Character | Usage |
|------|-----------|-------|
| `dot` | · | `separator=dot` |
| `bullet` | • | `separator=bullet` |
| `dash` | ─ | `separator=dash` |
| `bolddash` | ━ | `separator=bolddash` |
| `arrow` | → | `separator=arrow` |
| `star` | ★ | `separator=star` |
| `diamond` | ◆ | `separator=diamond` |
| `pipe` | \| | `separator=pipe` |
| `tilde` | ~ | `separator=tilde` |

### Block Elements

Numbers represent eighths (1 = 1/8, 4 = 1/2, 7 = 7/8):

| Name | Character | Description |
|------|-----------|-------------|
| `block.full` | █ | Full block |
| `block.upper.1` | ▔ | Upper 1/8 |
| `block.upper.4` | ▀ | Upper half |
| `block.lower.1` | ▁ | Lower 1/8 |
| `block.lower.2` | ▂ | Lower 1/4 |
| `block.lower.3` | ▃ | Lower 3/8 |
| `block.lower.4` | ▄ | Lower half |
| `block.lower.5` | ▅ | Lower 5/8 |
| `block.lower.6` | ▆ | Lower 3/4 |
| `block.lower.7` | ▇ | Lower 7/8 |
| `block.left.1` | ▏ | Left 1/8 |
| `block.left.2` | ▎ | Left 1/4 |
| `block.left.3` | ▍ | Left 3/8 |
| `block.left.4` | ▌ | Left half |
| `block.left.5` | ▋ | Left 5/8 |
| `block.left.6` | ▊ | Left 3/4 |
| `block.left.7` | ▉ | Left 7/8 |
| `block.right.1` | ▕ | Right 1/8 |
| `block.right.4` | ▐ | Right half |

### Shades

| Name | Character | Description |
|------|-----------|-------------|
| `shade.light` | ░ | Light shade |
| `shade.medium` | ▒ | Medium shade |
| `shade.dark` | ▓ | Dark shade |

### Quadrants

Grid positions: 1=top-left, 2=top-right, 3=bottom-left, 4=bottom-right

```
1 | 2
-----
3 | 4
```

| Name | Character | Filled positions |
|------|-----------|------------------|
| `quad.1` | ▘ | Top-left |
| `quad.2` | ▝ | Top-right |
| `quad.3` | ▖ | Bottom-left |
| `quad.4` | ▗ | Bottom-right |
| `quad.1-4` | ▚ | Diagonal (TL + BR) |
| `quad.2-3` | ▞ | Diagonal (TR + BL) |
| `quad.1-3-4` | ▙ | All except TR |
| `quad.1-2-3` | ▛ | All except BR |
| `quad.1-2-4` | ▜ | All except BL |
| `quad.2-3-4` | ▟ | All except TL |

### Braille Patterns

Bar graph elements (fills from bottom-left, then bottom-right):

| Name | Character | Description |
|------|-----------|-------------|
| `braille.empty` | ⠀ | Empty (no dots) |
| `braille.bar.1` | ⡀ | 1/8 filled |
| `braille.bar.2` | ⡄ | 2/8 filled |
| `braille.bar.3` | ⡆ | 3/8 filled |
| `braille.bar.4` | ⡇ | 4/8 (left column) |
| `braille.bar.5` | ⣇ | 5/8 filled |
| `braille.bar.6` | ⣧ | 6/8 filled |
| `braille.bar.7` | ⣷ | 7/8 filled |
| `braille.bar.8` | ⣿ | Full (all dots) |
| `braille.full` | ⣿ | All dots filled |
| `braille.left` | ⡇ | Left column |
| `braille.right` | ⢸ | Right column |

### Box Drawing

Elements: `h` (horizontal), `v` (vertical), `tl/tr/bl/br` (corners), `cross`, `t-up/t-down/t-left/t-right` (T-junctions)

**Light** (`box.light.*`):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `box.light.h` | ─ | `box.light.v` | │ |
| `box.light.tl` | ┌ | `box.light.tr` | ┐ |
| `box.light.bl` | └ | `box.light.br` | ┘ |
| `box.light.cross` | ┼ | `box.light.t-down` | ┬ |
| `box.light.t-up` | ┴ | `box.light.t-right` | ├ |
| `box.light.t-left` | ┤ | | |

**Heavy** (`box.heavy.*`):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `box.heavy.h` | ━ | `box.heavy.v` | ┃ |
| `box.heavy.tl` | ┏ | `box.heavy.tr` | ┓ |
| `box.heavy.bl` | ┗ | `box.heavy.br` | ┛ |
| `box.heavy.cross` | ╋ | `box.heavy.t-down` | ┳ |
| `box.heavy.t-up` | ┻ | `box.heavy.t-right` | ┣ |
| `box.heavy.t-left` | ┫ | | |

**Double** (`box.double.*`):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `box.double.h` | ═ | `box.double.v` | ║ |
| `box.double.tl` | ╔ | `box.double.tr` | ╗ |
| `box.double.bl` | ╚ | `box.double.br` | ╝ |
| `box.double.cross` | ╬ | `box.double.t-down` | ╦ |
| `box.double.t-up` | ╩ | `box.double.t-right` | ╠ |
| `box.double.t-left` | ╣ | | |

**Round corners** (`box.round.*`):

| Name | Character | Description |
|------|-----------|-------------|
| `box.round.tl` | ╭ | Rounded top-left |
| `box.round.tr` | ╮ | Rounded top-right |
| `box.round.bl` | ╰ | Rounded bottom-left |
| `box.round.br` | ╯ | Rounded bottom-right |

### Number Badges

Circled numbers (0-20):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `circle.0` | ⓪ | `circle.10` | ⑩ |
| `circle.1` | ① | `circle.11` | ⑪ |
| `circle.2` | ② | `circle.12` | ⑫ |
| `circle.3` | ③ | ... | ... |

Negative circled (white on black, 0-20):

| Name | Character | Name | Character |
|------|-----------|------|-----------|
| `neg-circle.1` | ❶ | `neg-circle.10` | ❿ |
| `neg-circle.2` | ❷ | ... | ... |

Double-circled (1-10): `dbl-circle.1` → ⓵

Parenthesized numbers (1-20): `paren.1` → ⑴

Parenthesized letters (a-z): `paren.a` → ⒜

Period numbers (0-20): `period.1` → ⒈

### Shapes

**Squares** (`square.*`):

| Name | Char | Name | Char |
|------|------|------|------|
| `square.filled` | ■ | `square.empty` | □ |
| `square.rounded` | ▢ | `square.dotted` | ▣ |
| `square.h-lines` | ▤ | `square.v-lines` | ▥ |
| `square.grid` | ▦ | `square.cross` | ▩ |

**Circles** (`circle.*`):

| Name | Char | Name | Char |
|------|------|------|------|
| `circle.filled` | ● | `circle.empty` | ○ |
| `circle.target` | ◎ | `circle.dotted` | ◌ |
| `circle.half-left` | ◐ | `circle.half-right` | ◑ |
| `circle.half-top` | ◓ | `circle.half-bottom` | ◒ |

**Triangles** (`tri.*`):

| Direction | Filled | Empty | Small |
|-----------|--------|-------|-------|
| up | ▲ | △ | ▴ |
| down | ▼ | ▽ | ▾ |
| left | ◀ | ◁ | ◂ |
| right | ▶ | ▷ | ▸ |

**Diamonds** (`diamond.*`): ◆ ◇ ◈ ◊

### Checkboxes

| Name | Char | Description |
|------|------|-------------|
| `check.empty` | ☐ | Empty box |
| `check.yes` | ☑ | Checked |
| `check.no` | ☒ | X'd out |
| `check.mark` | ✓ | Checkmark |
| `check.heavy` | ✔ | Heavy check |
| `check.x` | ✗ | X mark |
| `check.x.heavy` | ✘ | Heavy X |

### Arrows

**Basic** (`arrow.*`):

| Name | Char | Name | Char |
|------|------|------|------|
| `arrow.left` | ← | `arrow.right` | → |
| `arrow.up` | ↑ | `arrow.down` | ↓ |
| `arrow.left-right` | ↔ | `arrow.up-down` | ↕ |
| `arrow.nw` | ↖ | `arrow.ne` | ↗ |
| `arrow.sw` | ↙ | `arrow.se` | ↘ |

**Double** (`arrow.double-*`): ⇐ ⇑ ⇒ ⇓ ⇔ ⇕

**Dashed** (`arrow.dashed-*`): ⇠ ⇡ ⇢ ⇣

### Dice

| Name | Char | Name | Char |
|------|------|------|------|
| `die.1` | ⚀ | `die.4` | ⚃ |
| `die.2` | ⚁ | `die.5` | ⚄ |
| `die.3` | ⚂ | `die.6` | ⚅ |

### Card Suits

| Name | Char | Name | Char |
|------|------|------|------|
| `card.spade` | ♠ | `card.spade.empty` | ♤ |
| `card.heart` | ♥ | `card.heart.empty` | ♡ |
| `card.diamond` | ♦ | `card.diamond.empty` | ♢ |
| `card.club` | ♣ | `card.club.empty` | ♧ |

### Music

| Name | Char | Description |
|------|------|-------------|
| `music.quarter` | ♩ | Quarter note |
| `music.eighth` | ♪ | Eighth note |
| `music.beamed` | ♫ | Beamed eighth notes |
| `music.beamed-16` | ♬ | Beamed sixteenth notes |
| `music.flat` | ♭ | Flat |
| `music.natural` | ♮ | Natural |
| `music.sharp` | ♯ | Sharp |

### Math Symbols

**Operators** (`math.*`):

| Name | Char | Name | Char |
|------|------|------|------|
| `math.plus-minus` | ± | `math.times` | × |
| `math.divide` | ÷ | `math.sqrt` | √ |
| `math.sum` | ∑ | `math.product` | ∏ |
| `math.integral` | ∫ | `math.partial` | ∂ |
| `math.delta` | ∆ | `math.nabla` | ∇ |

**Relations**:

| Name | Char | Name | Char |
|------|------|------|------|
| `math.approx` | ≈ | `math.not-equal` | ≠ |
| `math.lte` | ≤ | `math.gte` | ≥ |
| `math.infinity` | ∞ | | |

**Set theory**:

| Name | Char | Name | Char |
|------|------|------|------|
| `math.element-of` | ∈ | `math.not-element` | ∉ |
| `math.subset` | ⊂ | `math.superset` | ⊃ |
| `math.union` | ∪ | `math.intersect` | ∩ |
| `math.empty-set` | ∅ | | |

**Logic**:

| Name | Char | Name | Char |
|------|------|------|------|
| `math.forall` | ∀ | `math.exists` | ∃ |
| `math.not` | ¬ | `math.and` | ∧ |
| `math.or` | ∨ | `math.xor` | ⊕ |
| `math.therefore` | ∴ | `math.because` | ∵ |

### Superscript & Subscript

**Superscript** (`sup.*`):

| Name | Char | Name | Char | Name | Char |
|------|------|------|------|------|------|
| `sup.0` | ⁰ | `sup.1` | ¹ | `sup.2` | ² |
| `sup.3` | ³ | `sup.4` | ⁴ | `sup.5` | ⁵ |
| `sup.6` | ⁶ | `sup.7` | ⁷ | `sup.8` | ⁸ |
| `sup.9` | ⁹ | `sup.n` | ⁿ | | |

Also: `sup.+` ⁺, `sup.-` ⁻, `sup.=` ⁼, `sup.(` ⁽, `sup.)` ⁾

**Subscript** (`sub.*`):

| Name | Char | Name | Char | Name | Char |
|------|------|------|------|------|------|
| `sub.0` | ₀ | `sub.1` | ₁ | `sub.2` | ₂ |
| `sub.3` | ₃ | `sub.4` | ₄ | `sub.5` | ₅ |
| `sub.6` | ₆ | `sub.7` | ₇ | `sub.8` | ₈ |
| `sub.9` | ₉ | | | | |

Also: `sub.+` ₊, `sub.-` ₋, `sub.=` ₌, `sub.(` ₍, `sub.)` ₎

### Roman Numerals

| Name | Char | Name | Char | Name | Char |
|------|------|------|------|------|------|
| `roman.1` | Ⅰ | `roman.2` | Ⅱ | `roman.3` | Ⅲ |
| `roman.4` | Ⅳ | `roman.5` | Ⅴ | `roman.6` | Ⅵ |
| `roman.7` | Ⅶ | `roman.8` | Ⅷ | `roman.9` | Ⅸ |
| `roman.10` | Ⅹ | `roman.11` | Ⅺ | `roman.12` | Ⅻ |
| `roman.50` | Ⅼ | `roman.100` | Ⅽ | `roman.500` | Ⅾ |
| `roman.1000` | Ⅿ | | | | |

### Stars

| Name | Char | Name | Char |
|------|------|------|------|
| `star.filled` | ★ | `star.empty` | ☆ |
| `star.4` | ✦ | `star.4.empty` | ✧ |
| `star.circle` | ✪ | `star.shadow` | ✫ |
| `star.5.empty` | ✭ | `star.outlined` | ✮ |
| `star.pinwheel` | ✯ | `star.heavy` | ✰ |
| `star.6` | ✡ | `star.8` | ✴ |
| `star.sparkle` | ❇ | | |

### Currency

| Name | Char | Name | Char |
|------|------|------|------|
| `currency.dollar` | $ | `currency.cent` | ¢ |
| `currency.pound` | £ | `currency.yen` | ¥ |
| `currency.euro` | € | `currency.won` | ₩ |
| `currency.rupee` | ₹ | `currency.ruble` | ₽ |
| `currency.bitcoin` | ₿ | `currency.generic` | ¤ |

### Greek Letters

**Lowercase** (`greek.*`):

| Name | Char | Name | Char | Name | Char |
|------|------|------|------|------|------|
| `greek.alpha` | α | `greek.beta` | β | `greek.gamma` | γ |
| `greek.delta` | δ | `greek.epsilon` | ε | `greek.zeta` | ζ |
| `greek.eta` | η | `greek.theta` | θ | `greek.iota` | ι |
| `greek.kappa` | κ | `greek.lambda` | λ | `greek.mu` | μ |
| `greek.nu` | ν | `greek.xi` | ξ | `greek.omicron` | ο |
| `greek.pi` | π | `greek.rho` | ρ | `greek.sigma` | σ |
| `greek.tau` | τ | `greek.upsilon` | υ | `greek.phi` | φ |
| `greek.chi` | χ | `greek.psi` | ψ | `greek.omega` | ω |

**Uppercase** (`greek.Alpha`, `greek.Beta`, etc.): Α Β Γ Δ Ε Ζ Η Θ Ι Κ Λ Μ Ν Ξ Ο Π Ρ Σ Τ Υ Φ Χ Ψ Ω

### Fractions

| Name | Char | Name | Char | Name | Char |
|------|------|------|------|------|------|
| `frac.1-2` | ½ | `frac.1-3` | ⅓ | `frac.2-3` | ⅔ |
| `frac.1-4` | ¼ | `frac.3-4` | ¾ | `frac.1-5` | ⅕ |
| `frac.2-5` | ⅖ | `frac.3-5` | ⅗ | `frac.4-5` | ⅘ |
| `frac.1-6` | ⅙ | `frac.5-6` | ⅚ | `frac.1-7` | ⅐ |
| `frac.1-8` | ⅛ | `frac.3-8` | ⅜ | `frac.5-8` | ⅝ |
| `frac.7-8` | ⅞ | `frac.1-9` | ⅑ | `frac.1-10` | ⅒ |

### Miscellaneous Symbols

**Warning/Safety** (`misc.*`):

| Name | Char | Name | Char |
|------|------|------|------|
| `misc.warning` | ⚠ | `misc.radioactive` | ☢ |
| `misc.biohazard` | ☣ | `misc.skull` | ☠ |
| `misc.medical` | ⚕ | `misc.recycle` | ♻ |

**Weather/Nature**:

| Name | Char | Name | Char |
|------|------|------|------|
| `misc.sun` | ☀ | `misc.cloud` | ☁ |
| `misc.umbrella` | ☂ | `misc.snowman` | ☃ |
| `misc.comet` | ☄ | `misc.lightning` | ⚡ |

**Objects/Tools**:

| Name | Char | Name | Char |
|------|------|------|------|
| `misc.anchor` | ⚓ | `misc.gear` | ⚙ |
| `misc.crossed-swords` | ⚔ | `misc.scales` | ⚖ |
| `misc.phone` | ☎ | `misc.mail` | ✉ |
| `misc.scissors` | ✂ | `misc.pencil` | ✏ |
| `misc.flag` | ⚑ | `misc.flag.empty` | ⚐ |

**Symbols**:

| Name | Char | Name | Char |
|------|------|------|------|
| `misc.peace` | ☮ | `misc.yinyang` | ☯ |
| `misc.atom` | ⚛ | `misc.fleur-de-lis` | ⚜ |
| `misc.infinity` | ♾ | `misc.wheelchair` | ♿ |
| `misc.smiley` | ☺ | `misc.frown` | ☹ |
| `misc.hot` | ♨ | | |

---

## Examples

### Gradient border

```markdown
{{glyph:shade.dark/}}{{glyph:shade.medium/}}{{glyph:shade.light/}} Title {{glyph:shade.light/}}{{glyph:shade.medium/}}{{glyph:shade.dark/}}
```

Output: `▓▒░ Title ░▒▓`

### Progress bar

```markdown
{{glyph:block.full/}}{{glyph:block.full/}}{{glyph:block.full/}}{{glyph:block.left.4/}}{{glyph:block.left.1/}}
```

Output: `███▌▏`

### Styled text with separator

```markdown
{{mathbold:separator=star}}HELLO{{/mathbold}}
```

Output: `𝐇★𝐄★𝐋★𝐋★𝐎`

### Braille bar chart

```markdown
{{glyph:braille.bar.2/}}{{glyph:braille.bar.5/}}{{glyph:braille.bar.8/}}{{glyph:braille.bar.6/}}{{glyph:braille.bar.3/}}
```

Output: `⡄⣇⣿⣧⡆`

### Box drawing frame

```markdown
{{glyph:box.round.tl/}}{{glyph:box.light.h/}}{{glyph:box.light.h/}}{{glyph:box.round.tr/}}
{{glyph:box.light.v/}} Hi {{glyph:box.light.v/}}
{{glyph:box.round.bl/}}{{glyph:box.light.h/}}{{glyph:box.light.h/}}{{glyph:box.round.br/}}
```

Output:
```
╭──╮
│ Hi │
╰──╯
```
