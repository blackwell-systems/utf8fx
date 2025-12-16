# Donuts & Gauges Gallery

A showcase of creative donut/ring charts and semi-circular gauges using the `{{ui:donut}}` and `{{ui:gauge}}` components.

---

## Donut Charts

### Basic Donuts

**Simple 75%:**
{{ui:donut:75/}}

**Half Full:**
{{ui:donut:50/}}

**Almost There:**
{{ui:donut:90/}}

---

### Size Variations

**Tiny (24px):**
{{ui:donut:60:size=24:thickness=3/}}

**Small (32px):**
{{ui:donut:75:size=32/}}

**Default (40px):**
{{ui:donut:50/}}

**Large (60px):**
{{ui:donut:65:size=60:thickness=6/}}

**XL (80px):**
{{ui:donut:80:size=80:thickness=8/}}

---

### Thickness Variations

**Hairline:**
{{ui:donut:70:size=50:thickness=1/}}

**Thin:**
{{ui:donut:70:size=50:thickness=2/}}

**Medium:**
{{ui:donut:70:size=50:thickness=5/}}

**Thick:**
{{ui:donut:70:size=50:thickness=10/}}

**Chunky:**
{{ui:donut:70:size=50:thickness=15/}}

---

### With Labels

**Labeled Progress:**
{{ui:donut:75:label=true/}}

**Large Labeled:**
{{ui:donut:85:size=60:thickness=6:label=true/}}

**Complete:**
{{ui:donut:100:fill=success:label=true/}}

---

### Color Palette

**Accent (Pink):**
{{ui:donut:70:fill=accent/}}

**Success (Green):**
{{ui:donut:85:fill=success/}}

**Warning (Yellow):**
{{ui:donut:50:fill=warning/}}

**Error (Red):**
{{ui:donut:25:fill=error/}}

**Info (Blue):**
{{ui:donut:60:fill=info/}}

**Cobalt:**
{{ui:donut:90:fill=cobalt/}}

---

### Custom Track Colors

**Dark Track:**
{{ui:donut:65:track=ink/}}

**Light Track:**
{{ui:donut:75:track=white:fill=cobalt/}}

**Subtle Track:**
{{ui:donut:50:track=ui.panel/}}

**Invisible Track:**
{{ui:donut:80:track=white/}}

---

### Project Status Dashboard

| Metric | Status |
|--------|--------|
| Tests Passing | {{ui:donut:98:fill=success:size=36:thickness=5/}} |
| Code Coverage | {{ui:donut:72:fill=info:size=36:thickness=5/}} |
| Build Health | {{ui:donut:100:fill=success:size=36:thickness=5/}} |
| Documentation | {{ui:donut:45:fill=warning:size=36:thickness=5/}} |
| Tech Debt | {{ui:donut:15:fill=error:size=36:thickness=5/}} |

---

## Gauge Charts

Semi-circular meters perfect for dashboards and speedometer-style displays.

### Basic Gauges

**Simple 75%:**
{{ui:gauge:75/}}

**Half:**
{{ui:gauge:50/}}

**Full:**
{{ui:gauge:100:fill=success/}}

---

### Size Variations

**Small (60px):**
{{ui:gauge:70:size=60:thickness=6/}}

**Default (80px):**
{{ui:gauge:70/}}

**Large (120px):**
{{ui:gauge:70:size=120:thickness=12/}}

**XL (160px):**
{{ui:gauge:70:size=160:thickness=16/}}

---

### With Labels

**Labeled Gauge:**
{{ui:gauge:75:label=true/}}

**Large Labeled:**
{{ui:gauge:85:size=120:thickness=12:label=true/}}

**Complete with Label:**
{{ui:gauge:100:fill=success:label=true/}}

---

### Color Variations

**Accent:**
{{ui:gauge:65:fill=accent/}}

**Success:**
{{ui:gauge:90:fill=success/}}

**Warning:**
{{ui:gauge:45:fill=warning/}}

**Error:**
{{ui:gauge:20:fill=error/}}

**Info:**
{{ui:gauge:70:fill=info/}}

**Cobalt:**
{{ui:gauge:80:fill=cobalt/}}

---

### Speedometer Style

**Speed Low:**
{{ui:gauge:25:fill=success:size=100:thickness=10:label=true/}}

**Speed Medium:**
{{ui:gauge:55:fill=warning:size=100:thickness=10:label=true/}}

**Speed High:**
{{ui:gauge:85:fill=error:size=100:thickness=10:label=true/}}

---

### Dashboard Metrics

| Metric | Gauge |
|--------|-------|
| CPU Usage | {{ui:gauge:73:fill=info:size=60:thickness=6/}} |
| Memory | {{ui:gauge:45:fill=success:size=60:thickness=6/}} |
| Disk | {{ui:gauge:88:fill=warning:size=60:thickness=6/}} |
| Network | {{ui:gauge:35:fill=cobalt:size=60:thickness=6/}} |

---

### Thin Arc Style

**Elegant Thin:**
{{ui:gauge:65:size=100:thickness=4/}}

**Hairline:**
{{ui:gauge:80:size=100:thickness=2/}}

---

### Thick Arc Style

**Chunky:**
{{ui:gauge:55:size=100:thickness=20/}}

**Bold:**
{{ui:gauge:75:size=100:thickness=16/}}

---

## Slider Mode (with Thumb)

Both donut and gauge components support a `thumb` parameter that adds a circular indicator at the fill position, creating a slider-like appearance.

### Donut Sliders

**Basic Donut Slider:**
{{ui:donut:75:size=60:thickness=6:thumb=14/}}

**Large with Custom Thumb:**
{{ui:donut:50:size=80:thickness=8:thumb=18:thumb_color=accent/}}

**Thin Track Slider:**
{{ui:donut:65:size=70:thickness=3:thumb=12/}}

**Multiple Values:**
{{ui:donut:25:size=50:thickness=5:thumb=12:fill=error/}} {{ui:donut:50:size=50:thickness=5:thumb=12:fill=warning/}} {{ui:donut:75:size=50:thickness=5:thumb=12:fill=success/}}

---

### Gauge Sliders

**Basic Gauge Slider:**
{{ui:gauge:60:size=100:thickness=8:thumb=16/}}

**Large with Custom Thumb:**
{{ui:gauge:75:size=120:thickness=10:thumb=20:thumb_color=accent/}}

**Thin Arc Slider:**
{{ui:gauge:40:size=100:thickness=4:thumb=14/}}

**Volume Control Style:**
{{ui:gauge:30:size=80:thickness=6:thumb=14:fill=info/}} {{ui:gauge:60:size=80:thickness=6:thumb=14:fill=info/}} {{ui:gauge:90:size=80:thickness=6:thumb=14:fill=info/}}

---

### Custom Thumb Colors

Demonstrating `thumb_color` to create contrasting thumb indicators:

**Donut with Accent Thumb:**
{{ui:donut:65:size=60:thickness=5:fill=info:thumb=14:thumb_color=accent/}}

**Donut with White Thumb:**
{{ui:donut:80:size=60:thickness=5:fill=success:thumb=14:thumb_color=white/}}

**Gauge with Warning Thumb:**
{{ui:gauge:50:size=100:thickness=8:fill=cobalt:thumb=16:thumb_color=warning/}}

**Gauge with Error Thumb:**
{{ui:gauge:75:size=100:thickness=8:fill=success:thumb=16:thumb_color=error/}}

**Contrasting Palette:**
{{ui:donut:40:size=50:thickness=4:fill=info:thumb=12:thumb_color=error/}} {{ui:donut:60:size=50:thickness=4:fill=warning:thumb=12:thumb_color=cobalt/}} {{ui:donut:80:size=50:thickness=4:fill=success:thumb=12:thumb_color=accent/}}

---

### Neon Sliders

**Neon Donut:**
{{ui:donut:70:size=70:thickness=6:fill=00FF41:track=0D0D0D:thumb=16:thumb_color=00FF41/}}

**Neon Gauge:**
{{ui:gauge:65:size=100:thickness=8:fill=FF00FF:track=0D0D0D:thumb=18:thumb_color=FF00FF/}}

---

## Comparison: Donut vs Gauge vs Progress

Same data, different visualizations:

| Task | Donut | Gauge | Bar |
|------|-------|-------|-----|
| API | {{ui:donut:85:size=36:thickness=5:fill=success/}} | {{ui:gauge:85:size=60:thickness=6:fill=success/}} | {{ui:progress:85:width=80:fill=success/}} |
| Tests | {{ui:donut:72:size=36:thickness=5:fill=info/}} | {{ui:gauge:72:size=60:thickness=6:fill=info/}} | {{ui:progress:72:width=80:fill=info/}} |
| Docs | {{ui:donut:45:size=36:thickness=5:fill=warning/}} | {{ui:gauge:45:size=60:thickness=6:fill=warning/}} | {{ui:progress:45:width=80:fill=warning/}} |

---

## Neon Gauges

**Neon Green:**
{{ui:gauge:75:fill=00FF41:track=0D0D0D:size=100:thickness=8/}}

**Neon Pink:**
{{ui:gauge:60:fill=FF00FF:track=0D0D0D:size=100:thickness=8/}}

**Neon Cyan:**
{{ui:gauge:85:fill=00FFFF:track=0D0D0D:size=100:thickness=8/}}

**Neon Orange:**
{{ui:gauge:50:fill=FF6600:track=0D0D0D:size=100:thickness=8/}}

---

## Loading Gauges

{{ui:gauge:0:size=50:thickness=4/}} {{ui:gauge:20:size=50:thickness=4/}} {{ui:gauge:40:size=50:thickness=4/}} {{ui:gauge:60:size=50:thickness=4/}} {{ui:gauge:80:size=50:thickness=4/}} {{ui:gauge:100:fill=success:size=50:thickness=4/}}

---

*Generated with mdfx donut and gauge chart components*
