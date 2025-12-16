# Donut Chart Gallery

A showcase of creative donut/ring chart designs using the `{{ui:donut}}` component.

---

## Basic Donuts

**Simple 75%:**
{{ui:donut:75/}}

**Half Full:**
{{ui:donut:50/}}

**Almost There:**
{{ui:donut:90/}}

---

## Size Variations

**Tiny (24px):**
{{ui:donut:60:size=24:thickness=3/}}

**Small (32px):**
{{ui:donut:60:size=32:thickness=3/}}

**Default (40px):**
{{ui:donut:60/}}

**Large (60px):**
{{ui:donut:60:size=60:thickness=6/}}

**XL (80px):**
{{ui:donut:60:size=80:thickness=8/}}

---

## Thickness Variations

**Hairline:**
{{ui:donut:70:size=50:thickness=2/}}

**Thin:**
{{ui:donut:70:size=50:thickness=4/}}

**Medium:**
{{ui:donut:70:size=50:thickness=6/}}

**Thick:**
{{ui:donut:70:size=50:thickness=10/}}

**Chunky:**
{{ui:donut:70:size=50:thickness=14/}}

---

## With Labels

**Labeled Progress:**
{{ui:donut:75:size=50:label=true/}}

**Large Labeled:**
{{ui:donut:42:size=70:thickness=6:label=true/}}

**Complete:**
{{ui:donut:100:size=50:label=true:fill=success/}}

---

## Color Palette

**Accent (Pink):**
{{ui:donut:80:fill=accent/}}

**Success (Green):**
{{ui:donut:80:fill=success/}}

**Warning (Yellow):**
{{ui:donut:80:fill=warning/}}

**Error (Red):**
{{ui:donut:80:fill=error/}}

**Info (Blue):**
{{ui:donut:80:fill=info/}}

**Cobalt:**
{{ui:donut:80:fill=cobalt/}}

---

## Custom Track Colors

**Dark Track:**
{{ui:donut:65:track=ink:fill=accent/}}

**Light Track:**
{{ui:donut:65:track=DDDDDD:fill=cobalt/}}

**Subtle Track:**
{{ui:donut:65:track=E8E8E8:fill=success/}}

**Invisible Track:**
{{ui:donut:65:track=FFFFFF:fill=error/}}

---

## Project Status Dashboard

| Metric | Status |
|--------|--------|
| Tests Passing | {{ui:donut:100:size=40:label=true:fill=success/}} |
| Code Coverage | {{ui:donut:87:size=40:label=true:fill=info/}} |
| Build Health | {{ui:donut:95:size=40:label=true:fill=success/}} |
| Documentation | {{ui:donut:64:size=40:label=true:fill=warning/}} |
| Tech Debt | {{ui:donut:23:size=40:label=true:fill=error/}} |

---

## Skill Levels (RPG Style)

| Attribute | Level |
|-----------|-------|
| Strength | {{ui:donut:85:size=36:thickness=5:fill=error/}} |
| Agility | {{ui:donut:72:size=36:thickness=5:fill=success/}} |
| Intelligence | {{ui:donut:94:size=36:thickness=5:fill=cobalt/}} |
| Charisma | {{ui:donut:58:size=36:thickness=5:fill=warning/}} |
| Luck | {{ui:donut:42:size=36:thickness=5:fill=accent/}} |

---

## Battery/Storage Indicators

**Full:**
{{ui:donut:100:size=50:thickness=8:fill=success:label=true/}}

**Good:**
{{ui:donut:75:size=50:thickness=8:fill=success:label=true/}}

**Low:**
{{ui:donut:25:size=50:thickness=8:fill=warning:label=true/}}

**Critical:**
{{ui:donut:10:size=50:thickness=8:fill=error:label=true/}}

---

## Loading States

{{ui:donut:0:size=30/}} {{ui:donut:20:size=30/}} {{ui:donut:40:size=30/}} {{ui:donut:60:size=30/}} {{ui:donut:80:size=30/}} {{ui:donut:100:size=30:fill=success/}}

---

## Compact Inline Usage

Storage: {{ui:donut:73:size=24:thickness=3/}} 73% used | Memory: {{ui:donut:45:size=24:thickness=3:fill=success/}} 45% | CPU: {{ui:donut:92:size=24:thickness=3:fill=error/}} 92%

---

## Dashboard Row

{{ui:row:align=center}}
{{ui:donut:95:size=60:thickness=6:fill=success:label=true/}} {{ui:donut:72:size=60:thickness=6:fill=info:label=true/}} {{ui:donut:48:size=60:thickness=6:fill=warning:label=true/}} {{ui:donut:15:size=60:thickness=6:fill=error:label=true/}}
{{/ui}}

---

## Thin Ring Style (Elegant)

{{ui:donut:80:size=70:thickness=3:fill=accent/}}

{{ui:donut:65:size=70:thickness=3:fill=cobalt/}}

{{ui:donut:50:size=70:thickness=3:fill=success/}}

---

## Chunky Pie Style

{{ui:donut:80:size=60:thickness=20:fill=accent/}}

{{ui:donut:65:size=60:thickness=20:fill=cobalt/}}

{{ui:donut:50:size=60:thickness=20:fill=success/}}

---

## Team Performance

| Team | Sprint Progress |
|------|-----------------|
| Frontend | {{ui:donut:88:size=44:thickness=5:fill=success:label=true/}} |
| Backend | {{ui:donut:76:size=44:thickness=5:fill=info:label=true/}} |
| DevOps | {{ui:donut:92:size=44:thickness=5:fill=success:label=true/}} |
| QA | {{ui:donut:55:size=44:thickness=5:fill=warning:label=true/}} |

---

## Comparison: Donut vs Progress

Same data, different visualization:

| Task | Donut | Bar |
|------|-------|-----|
| API | {{ui:donut:85:size=32:thickness=4/}} | {{ui:progress:85:width=100/}} |
| Tests | {{ui:donut:70:size=32:thickness=4/}} | {{ui:progress:70:width=100/}} |
| Docs | {{ui:donut:45:size=32:thickness=4/}} | {{ui:progress:45:width=100/}} |

---

*Generated with mdfx donut chart component*
