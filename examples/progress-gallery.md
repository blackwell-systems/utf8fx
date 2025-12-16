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
{{ui:progress:75:width=120:height=16:label=true/}}

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
| Core Engine | {{ui:progress:100:width=120:height=16:fill=success:label=true/}} |
| API | {{ui:progress:85:width=120:height=16:fill=accent:label=true/}} |
| Documentation | {{ui:progress:60:width=120:height=16:fill=warning:label=true/}} |
| Testing | {{ui:progress:40:width=120:height=16:fill=error:label=true/}} |

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

## Slider Mode

When `thumb` is set, the progress bar becomes a slider with a thumb indicator:

**Basic Slider:**
{{ui:progress:50:width=200:thumb=14/}}

**Colored Thumb:**
{{ui:progress:75:width=200:thumb=16:thumb_color=accent/}}

**Square Thumb:**
{{ui:progress:30:width=200:thumb=14:thumb_shape=square/}}

**Diamond Thumb:**
{{ui:progress:60:width=200:thumb=14:thumb_shape=diamond:thumb_color=warning/}}

**Large Thumb:**
{{ui:progress:40:width=200:thumb=20:thumb_color=success/}}

**Minimal Slider (thin track):**
{{ui:progress:65:width=200:height=4:thumb=10:track=CCCCCC:thumb_color=333333/}}

**Thick Track Slider:**
{{ui:progress:50:width=200:height=10:thumb=14/}}

**Barely Larger Thumb:**
{{ui:progress:70:width=200:height=12:thumb=16:thumb_color=cobalt/}}

**iOS-style Slider:**
{{ui:progress:45:width=200:height=8:thumb=20:track=DDDDDD:thumb_color=FFFFFF:border=CCCCCC/}}

---

## Slider Variations

| Setting | Slider |
|---------|--------|
| Volume Low | {{ui:progress:25:width=120:thumb=12/}} |
| Volume Mid | {{ui:progress:50:width=120:thumb=12/}} |
| Volume High | {{ui:progress:85:width=120:thumb=12:thumb_color=error/}} |

---

## Audio Mixer

| Channel | Level |
|---------|-------|
| Master | {{ui:progress:75:width=140:height=6:thumb=14:fill=success:thumb_color=success/}} |
| Bass | {{ui:progress:85:width=140:height=6:thumb=14:fill=error:thumb_color=error/}} |
| Treble | {{ui:progress:60:width=140:height=6:thumb=14:fill=info:thumb_color=info/}} |
| Vocals | {{ui:progress:70:width=140:height=6:thumb=14:fill=warning:thumb_color=warning/}} |

---

## Media Player Controls

**Seek Bar (Dark Theme):**
{{ui:progress:35:width=300:height=4:thumb=12:track=333333:fill=accent:thumb_color=FFFFFF/}}

**Spotify-style (Green):**
{{ui:progress:62:width=300:height=4:thumb=12:track=4D4D4D:fill=1DB954:thumb_color=FFFFFF/}}

**SoundCloud-style (Orange):**
{{ui:progress:45:width=300:height=3:thumb=10:track=333333:fill=FF5500:thumb_color=FF5500/}}

---

## Settings Sliders

**Brightness:**
{{ui:progress:80:width=180:height=8:thumb=16:track=1a1a2e:fill=FFD700:thumb_color=FFD700/}}

**Contrast:**
{{ui:progress:50:width=180:height=8:thumb=16:track=1a1a2e:fill=FFFFFF:thumb_color=FFFFFF/}}

**Saturation:**
{{ui:progress:65:width=180:height=8:thumb=16:track=1a1a2e:fill=FF6B6B:thumb_color=FF6B6B/}}

---

## Creative Thumb Shapes

**Circle (Default):**
{{ui:progress:50:width=160:thumb=16:thumb_color=accent/}}

**Square:**
{{ui:progress:50:width=160:thumb=14:thumb_shape=square:thumb_color=cobalt/}}

**Diamond:**
{{ui:progress:50:width=160:thumb=16:thumb_shape=diamond:thumb_color=success/}}

---

## Material Design Style

**Primary:**
{{ui:progress:60:width=200:height=4:thumb=14:track=E0E0E0:fill=6200EE:thumb_color=6200EE/}}

**Secondary:**
{{ui:progress:40:width=200:height=4:thumb=14:track=E0E0E0:fill=03DAC6:thumb_color=03DAC6/}}

**Error:**
{{ui:progress:75:width=200:height=4:thumb=14:track=E0E0E0:fill=B00020:thumb_color=B00020/}}

---

## Game UI Sliders

**Health Bar:**
{{ui:progress:45:width=200:height=12:thumb=18:track=2D0000:fill=FF0000:thumb_color=FF4444:thumb_shape=diamond/}}

**Mana Bar:**
{{ui:progress:80:width=200:height=12:thumb=18:track=00002D:fill=0066FF:thumb_color=4499FF:thumb_shape=diamond/}}

**XP Bar:**
{{ui:progress:65:width=200:height=12:thumb=18:track=2D2D00:fill=FFD700:thumb_color=FFEE44:thumb_shape=diamond/}}

---

## Temperature Control

**Cold:**
{{ui:progress:20:width=180:height=8:thumb=16:track=EEEEEE:fill=00BFFF:thumb_color=00BFFF/}}

**Comfortable:**
{{ui:progress:50:width=180:height=8:thumb=16:track=EEEEEE:fill=32CD32:thumb_color=32CD32/}}

**Hot:**
{{ui:progress:85:width=180:height=8:thumb=16:track=EEEEEE:fill=FF4500:thumb_color=FF4500/}}

---

## Minimal Dark Mode

{{ui:progress:40:width=220:height=2:thumb=10:track=444444:fill=888888:thumb_color=FFFFFF/}}

{{ui:progress:60:width=220:height=2:thumb=10:track=444444:fill=888888:thumb_color=FFFFFF/}}

{{ui:progress:80:width=220:height=2:thumb=10:track=444444:fill=888888:thumb_color=FFFFFF/}}

---

## Chunky Retro Style

{{ui:progress:50:width=160:height=16:thumb=20:rx=0:track=222222:fill=00FF00:thumb_color=00FF00:thumb_shape=square/}}

{{ui:progress:75:width=160:height=16:thumb=20:rx=0:track=222222:fill=FF00FF:thumb_color=FF00FF:thumb_shape=square/}}

---

*Generated with mdfx progress bar component*
