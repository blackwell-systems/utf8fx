# Progress Bar Gallery

A showcase of creative progress bar designs using the `{{ui:progress}}` component.

---

## Basic Styles

**Simple Progress:**
{{ui:progress:75/}}

**Wide Bar:**
{{ui:progress:60:width=200/}}

**Tall Bar:**
{{ui:progress:80:width=150:height=20/}}

---

## Color Variations

**Success (Green):**
{{ui:progress:100:fill=success:width=150/}}

**Warning (Yellow):**
{{ui:progress:65:fill=warning:width=150/}}

**Error (Red):**
{{ui:progress:25:fill=error:width=150/}}

**Accent (Pink):**
{{ui:progress:90:fill=accent:width=150/}}

**Cobalt (Blue):**
{{ui:progress:50:fill=cobalt:width=150/}}

---

## Track Color Variations

**Dark Track:**
{{ui:progress:70:track=ink:fill=success:width=150/}}

**Light Track:**
{{ui:progress:45:track=CCCCCC:fill=accent:width=150/}}

**Contrasting:**
{{ui:progress:85:track=slate:fill=warning:width=150/}}

---

## Floating Fill Effect

When `fill_height` is less than `height`, the fill "floats" inside the track:

**Subtle Float:**
{{ui:progress:60:height=12:fill_height=8:width=150/}}

**Prominent Float:**
{{ui:progress:75:height=16:fill_height=8:width=150/}}

**Thin Float:**
{{ui:progress:80:height=14:fill_height=4:width=150/}}

---

## With Labels

**Basic Label:**
{{ui:progress:75:width=120:label=true/}}

**Tall with Label:**
{{ui:progress:50:width=150:height=20:label=true/}}

**Colored Label:**
{{ui:progress:90:width=150:height=18:label=true:label_color=000000/}}

---

## Corner Radius Variations

**Sharp Corners (rx=0):**
{{ui:progress:65:rx=0:width=150/}}

**Slightly Rounded (rx=2):**
{{ui:progress:65:rx=2:width=150/}}

**Very Rounded (rx=8):**
{{ui:progress:65:rx=8:height=16:width=150/}}

**Pill Shape:**
{{ui:progress:65:rx=10:height=20:width=150/}}

---

## With Borders

**Accent Border:**
{{ui:progress:70:width=150:border=accent/}}

**Dark Border:**
{{ui:progress:55:width=150:border=ink:border_width=2/}}

**Contrasting Border:**
{{ui:progress:80:width=150:track=slate:fill=success:border=white/}}

---

## Skill Bars (Common Use Case)

| Skill | Level |
|-------|-------|
| Rust | {{ui:progress:95:width=100:fill=DEA584/}} |
| Python | {{ui:progress:85:width=100:fill=3776AB/}} |
| JavaScript | {{ui:progress:80:width=100:fill=F7DF1E/}} |
| Go | {{ui:progress:70:width=100:fill=00ADD8/}} |
| TypeScript | {{ui:progress:75:width=100:fill=3178C6/}} |

---

## Project Status

| Component | Completion |
|-----------|------------|
| Core Engine | {{ui:progress:100:width=120:fill=success:label=true/}} |
| API | {{ui:progress:85:width=120:fill=accent:label=true/}} |
| Documentation | {{ui:progress:60:width=120:fill=warning:label=true/}} |
| Testing | {{ui:progress:40:width=120:fill=error:label=true/}} |

---

## Creative Combinations

**Neon Style:**
{{ui:progress:75:width=200:height=8:fill_height=4:track=111111:fill=00FF88:border=00FF88/}}

**Minimal:**
{{ui:progress:60:width=200:height=4:rx=2:track=EEEEEE:fill=333333/}}

**Corporate Blue:**
{{ui:progress:80:width=180:height=14:track=E8EEF4:fill=cobalt:border=cobalt:rx=7/}}

**Sunset Gradient Effect (using multiple bars):**
{{ui:progress:100:width=50:height=12:fill=FF6B6B:rx=0/}}{{ui:progress:100:width=50:height=12:fill=FFE66D:rx=0/}}{{ui:progress:100:width=50:height=12:fill=4ECDC4:rx=0/}}

---

## Loading States

**Empty:**
{{ui:progress:0:width=150/}}

**Starting:**
{{ui:progress:10:width=150/}}

**Quarter:**
{{ui:progress:25:width=150/}}

**Half:**
{{ui:progress:50:width=150/}}

**Three-quarters:**
{{ui:progress:75:width=150/}}

**Complete:**
{{ui:progress:100:width=150:fill=success/}}

---

*Generated with mdfx progress bar component*
