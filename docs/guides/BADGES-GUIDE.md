# Badges Guide

Badges transform numbers and letters into stylized Unicode numerals and enclosed characters. Perfect for list numbering, step indicators, and visual markers.

## Basic Syntax

```markdown
{{badge:style:value/}}
```

Where `value` is a number or letter to transform.

---

## All Badge Styles

### circle

Numbers enclosed in circles. Supports 0-20.

| Input | Output |
|-------|--------|
| 0 | ⓪ |
| 1-9 | ① ② ③ ④ ⑤ ⑥ ⑦ ⑧ ⑨ |
| 10-20 | ⑩ ⑪ ⑫ ⑬ ⑭ ⑮ ⑯ ⑰ ⑱ ⑲ ⑳ |

**Aliases:** `circled`, `number-circle`

```markdown
{{badge:circle:1/}} First step
{{badge:circle:2/}} Second step
{{badge:circle:3/}} Third step
```

**Output:**
```
① First step
② Second step
③ Third step
```

---

### negative-circle

White numbers on black circles. High contrast. Supports 0-20.

| Input | Output |
|-------|--------|
| 0 | ⓿ |
| 1-9 | ❶ ❷ ❸ ❹ ❺ ❻ ❼ ❽ ❾ |
| 10-20 | ❿ ⓫ ⓬ ⓭ ⓮ ⓯ ⓰ ⓱ ⓲ ⓳ ⓴ |

**Aliases:** `neg-circle`, `inverse-circle`

```markdown
{{badge:negative-circle:1/}} Critical
{{badge:negative-circle:2/}} Important
{{badge:negative-circle:3/}} Normal
```

**Output:**
```
❶ Critical
❷ Important
❸ Normal
```

---

### double-circle

Numbers in double circles. Supports 1-10 only.

| Input | Output |
|-------|--------|
| 1-10 | ⓵ ⓶ ⓷ ⓸ ⓹ ⓺ ⓻ ⓼ ⓽ ⓾ |

**Aliases:** `double`, `dbl-circle`

```markdown
{{badge:double-circle:1/}} Phase one
{{badge:double-circle:2/}} Phase two
```

**Output:**
```
⓵ Phase one
⓶ Phase two
```

---

### paren

Numbers in parentheses. Supports 1-20.

| Input | Output |
|-------|--------|
| 1-9 | ⑴ ⑵ ⑶ ⑷ ⑸ ⑹ ⑺ ⑻ ⑼ |
| 10-20 | ⑽ ⑾ ⑿ ⒀ ⒁ ⒂ ⒃ ⒄ ⒅ ⒆ ⒇ |

**Aliases:** `parenthesized`, `parens`

```markdown
{{badge:paren:1/}} Introduction
{{badge:paren:2/}} Methods
{{badge:paren:3/}} Results
```

**Output:**
```
⑴ Introduction
⑵ Methods
⑶ Results
```

---

### paren-letter

Lowercase letters in parentheses. Supports a-z.

| Input | Output |
|-------|--------|
| a-i | ⒜ ⒝ ⒞ ⒟ ⒠ ⒡ ⒢ ⒣ ⒤ |
| j-r | ⒥ ⒦ ⒧ ⒨ ⒩ ⒪ ⒫ ⒬ ⒭ |
| s-z | ⒮ ⒯ ⒰ ⒱ ⒲ ⒳ ⒴ ⒵ |

**Aliases:** `letter-paren`, `alpha-paren`

```markdown
{{badge:paren-letter:a/}} Option A
{{badge:paren-letter:b/}} Option B
{{badge:paren-letter:c/}} Option C
```

**Output:**
```
⒜ Option A
⒝ Option B
⒞ Option C
```

---

### period

Numbers with period suffix. Supports 0-20.

| Input | Output |
|-------|--------|
| 0-9 | 🄀 🄁 🄂 🄃 🄄 🄅 🄆 🄇 🄈 🄉 |
| 10-20 | 🄊 🄋 🄌 🄍 🄎 🄏 🄐 🄑 🄒 🄓 🄔 |

**Aliases:** `dot-number`, `period-number`

```markdown
{{badge:period:1/}} Preparation
{{badge:period:2/}} Execution
{{badge:period:3/}} Review
```

**Output:**
```
🄁 Preparation
🄂 Execution
🄃 Review
```

---

## Comparison Chart

| Badge | Range | Example | Best For |
|-------|-------|---------|----------|
| `circle` | 0-20 | ① ② ③ | Standard lists |
| `negative-circle` | 0-20 | ❶ ❷ ❸ | High visibility |
| `double-circle` | 1-10 | ⓵ ⓶ ⓷ | Special emphasis |
| `paren` | 1-20 | ⑴ ⑵ ⑶ | Academic style |
| `paren-letter` | a-z | ⒜ ⒝ ⒞ | Sub-items |
| `period` | 0-20 | 🄁 🄂 🄃 | Formal lists |

---

## Practical Examples

### Step-by-Step Instructions

```markdown
{{badge:circle:1/}} Clone the repository
{{badge:circle:2/}} Install dependencies
{{badge:circle:3/}} Run the development server
{{badge:circle:4/}} Open localhost:3000
```

### Priority Levels

```markdown
{{badge:negative-circle:1/}} **Critical** - Fix immediately
{{badge:negative-circle:2/}} **High** - Address this sprint
{{badge:negative-circle:3/}} **Medium** - Schedule for next sprint
```

### Nested List Items

```markdown
{{badge:circle:1/}} Main feature
  {{badge:paren-letter:a/}} Sub-feature one
  {{badge:paren-letter:b/}} Sub-feature two
  {{badge:paren-letter:c/}} Sub-feature three
{{badge:circle:2/}} Secondary feature
```

### Project Phases

```markdown
{{badge:double-circle:1/}} Discovery
{{badge:double-circle:2/}} Design
{{badge:double-circle:3/}} Development
{{badge:double-circle:4/}} Testing
{{badge:double-circle:5/}} Deployment
```

### Academic Citations

```markdown
{{badge:paren:1/}} Smith et al., 2023
{{badge:paren:2/}} Johnson & Lee, 2022
{{badge:paren:3/}} Davis, 2021
```

### Rating Scale

```markdown
Performance rating:
{{badge:negative-circle:5/}} Excellent
{{badge:negative-circle:4/}} Good
{{badge:negative-circle:3/}} Satisfactory
{{badge:negative-circle:2/}} Needs improvement
{{badge:negative-circle:1/}} Unsatisfactory
```

---

## Combining with Other Elements

### With Frames

```markdown
{{frame:arrow}}{{badge:circle:1/}} Navigate to settings{{/frame}}
{{frame:arrow}}{{badge:circle:2/}} Click "Advanced"{{/frame}}
{{frame:arrow}}{{badge:circle:3/}} Enable feature{{/frame}}
```

### With Status Indicators

```markdown
{{badge:circle:1/}} {{ui:status:success/}} Setup complete
{{badge:circle:2/}} {{ui:status:success/}} Dependencies installed
{{badge:circle:3/}} {{ui:status:warning/}} Configuration needed
{{badge:circle:4/}} {{ui:status:error/}} Build failed
```

### In Tables

```markdown
| Step | Task | Status |
|------|------|--------|
| {{badge:circle:1/}} | Initialize | Done |
| {{badge:circle:2/}} | Configure | Done |
| {{badge:circle:3/}} | Test | Pending |
```

---

## Full Character Reference

### Circled Numbers (circle)
```
⓪ ① ② ③ ④ ⑤ ⑥ ⑦ ⑧ ⑨ ⑩ ⑪ ⑫ ⑬ ⑭ ⑮ ⑯ ⑰ ⑱ ⑲ ⑳
```

### Negative Circled Numbers (negative-circle)
```
⓿ ❶ ❷ ❸ ❹ ❺ ❻ ❼ ❽ ❾ ❿ ⓫ ⓬ ⓭ ⓮ ⓯ ⓰ ⓱ ⓲ ⓳ ⓴
```

### Double Circled Numbers (double-circle)
```
⓵ ⓶ ⓷ ⓸ ⓹ ⓺ ⓻ ⓼ ⓽ ⓾
```

### Parenthesized Numbers (paren)
```
⑴ ⑵ ⑶ ⑷ ⑸ ⑹ ⑺ ⑻ ⑼ ⑽ ⑾ ⑿ ⒀ ⒁ ⒂ ⒃ ⒄ ⒅ ⒆ ⒇
```

### Parenthesized Letters (paren-letter)
```
⒜ ⒝ ⒞ ⒟ ⒠ ⒡ ⒢ ⒣ ⒤ ⒥ ⒦ ⒧ ⒨ ⒩ ⒪ ⒫ ⒬ ⒭ ⒮ ⒯ ⒰ ⒱ ⒲ ⒳ ⒴ ⒵
```

### Period Numbers (period)
```
🄀 🄁 🄂 🄃 🄄 🄅 🄆 🄇 🄈 🄉 🄊 🄋 🄌 🄍 🄎 🄏 🄐 🄑 🄒 🄓 🄔
```

---

## Tips

1. **Consistency** - Pick one badge style per list/section
2. **Visibility** - Use `negative-circle` for dark backgrounds
3. **Range limits** - Check supported range before using higher numbers
4. **Accessibility** - Screen readers may not interpret badges correctly
5. **Fallbacks** - Some fonts may not render all badge characters
