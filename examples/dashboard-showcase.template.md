# Dashboard Visualization Showcase

**A comprehensive demonstration of mdfx visualization components**

{{ui:swatch:000000:width=700:height=6:gradient=horizontal/f41c80/3B82F6/22C55E:rx=3/}}

---

## System Monitoring Dashboard

### Real-Time Metrics

| Metric | Gauge | Current | Trend (7d) |
|--------|-------|---------|------------|
| CPU | {{ui:gauge:45:size=50:thickness=5:fill=success:label=true/}} | 45% | {{ui:sparkline:30,35,42,38,45,40,45:width=100:height=25:fill=success/}} |
| Memory | {{ui:gauge:72:size=50:thickness=5:fill=warning:label=true/}} | 72% | {{ui:sparkline:60,65,68,70,72,71,72:width=100:height=25:fill=warning/}} |
| Disk | {{ui:gauge:88:size=50:thickness=5:fill=error:label=true/}} | 88% | {{ui:sparkline:78,80,82,84,86,87,88:width=100:height=25:fill=error/}} |
| Network | {{ui:gauge:35:size=50:thickness=5:fill=info:label=true/}} | 35% | {{ui:sparkline:25,45,30,55,40,50,35:width=100:height=25:fill=info/}} |

### Service Health

{{ui:row:align=center}}
{{ui:donut:100:size=60:thickness=6:fill=success:label=true/}} API
{{ui:donut:98:size=60:thickness=6:fill=success:label=true/}} Database
{{ui:donut:95:size=60:thickness=6:fill=success:label=true/}} Cache
{{ui:donut:85:size=60:thickness=6:fill=warning:label=true/}} Queue
{{/ui}}

---

## Project Progress Tracker

### Sprint Status

| Feature | Progress | Status |
|---------|----------|--------|
| Authentication | {{ui:progress:100:width=150:fill=success:label=true/}} | Complete |
| User Dashboard | {{ui:progress:85:width=150:fill=info:label=true/}} | In Review |
| API Endpoints | {{ui:progress:65:width=150:fill=accent:label=true/}} | Active |
| Documentation | {{ui:progress:40:width=150:fill=warning:label=true/}} | In Progress |
| Testing | {{ui:progress:25:width=150:fill=slate:label=true/}} | Starting |

### Overall Completion

{{ui:swatch:000000:width=600:height=40:gradient=horizontal/22C55E/3B82F6:rx=6:label=Sprint 73% Complete/}}

### Milestone Timeline

{{frame:gradient}}
{{ui:progress:100:width=100:fill=success:height=8:rx=4/}} Phase 1 → {{ui:progress:100:width=100:fill=success:height=8:rx=4/}} Phase 2 → {{ui:progress:65:width=100:fill=info:height=8:rx=4/}} Phase 3 → {{ui:progress:0:width=100:fill=slate:height=8:rx=4/}} Phase 4
{{/}}

---

## Performance Analytics

### Response Time Distribution

{{ui:sparkline:45,52,48,55,50,47,52,48,55,50,45,48,52,55,48:width=400:height=50:type=area:fill=info/}}

**Average:** 48ms | **P95:** 55ms | **P99:** 62ms

### Request Volume (24h)

{{ui:sparkline:120,145,180,220,280,350,420,380,320,280,240,200:width=400:height=60:type=bar:fill=accent:bar_width=28/}}

### Error Rate Trend

{{ui:waveform:-0.2,0.1,-0.1,0.3,-0.2,0.2,-0.3,0.1,-0.1,0.2,-0.2,0.1:width=400:height=40:positive=success:negative=error/}}

---

## User Satisfaction

### Overall Ratings

{{ui:row:align=center}}
{{ui:rating:4.5/}} **4.5/5** Overall
{{ui:rating:4.8/}} **4.8/5** UX
{{ui:rating:4.2/}} **4.2/5** Performance
{{ui:rating:4.6/}} **4.6/5** Support
{{/ui}}

### Satisfaction Breakdown

| Category | Rating | Score |
|----------|--------|-------|
| Ease of Use | {{ui:rating:5:size=16/}} | Excellent |
| Features | {{ui:rating:4.5:size=16/}} | Great |
| Reliability | {{ui:rating:4:size=16/}} | Good |
| Value | {{ui:rating:4.5:size=16/}} | Great |
| Support | {{ui:rating:5:size=16/}} | Excellent |

### NPS Score

{{ui:gauge:72:size=100:thickness=12:fill=success:label=true:track=slate/}}

**Net Promoter Score: 72** (Excellent)

---

## Tech Stack

### Primary Technologies (Icon Only)

{{ui:row:align=center}}
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:react/}} {{ui:tech:postgresql/}}
{{/ui}}

### Technologies with Versions (Two-Segment Badges)

{{ui:row:align=center}}
{{ui:tech:rust:label=v1.80/}} {{ui:tech:typescript:label=v5.3/}} {{ui:tech:react:label=v18.2/}} {{ui:tech:nodejs:label=v20 LTS/}}
{{/ui}}

### Infrastructure with Labels

{{ui:row:align=center}}
{{ui:tech:docker:label=Container/}} {{ui:tech:kubernetes:label=K8s/}} {{ui:tech:amazonaws:label=AWS/}} {{ui:tech:terraform:label=IaC/}}
{{/ui}}

### DevOps Tools

{{ui:row:align=center}}
{{ui:tech:github:label=Source/}} {{ui:tech:gitlab:label=CI/CD/}} {{ui:tech:redis:label=Cache/}} {{ui:tech:mongodb:label=NoSQL/}}
{{/ui}}

### Full Stack Banner

{{ui:swatch:000000:width=600:height=50:gradient=horizontal/282f3c/263143:rx=8:border=f41c80:label=Rust + TypeScript + React + PostgreSQL/}}

---

## Activity Waveforms

### Audio Processing

{{ui:waveform:0.8,0.6,0.9,0.4,0.7,0.5,0.8,0.3,0.9,0.6,0.7,0.4,0.8,0.5,0.6:width=500:height=60:positive=accent:negative=info:bar_width=8:spacing=4/}}

### Signal Analysis

{{ui:waveform:-0.5,0.7,-0.3,0.8,-0.6,0.5,-0.4,0.9,-0.2,0.6,-0.7,0.4,-0.5,0.8,-0.3:width=500:height=60:positive=success:negative=error:bar_width=6:spacing=3/}}

### Balanced Data

{{ui:waveform:0.3,-0.4,0.5,-0.3,0.6,-0.5,0.4,-0.6,0.5,-0.4,0.7,-0.3,0.5,-0.5,0.4:width=500:height=50:positive=info:negative=warning:show_center=true/}}

---

## Status Indicators

### Environment Status

{{ui:row:align=center}}
{{ui:swatch:22C55E:width=120:height=32:rx=16:shadow=22C55E/8/0/0:label=PRODUCTION/}}
{{ui:swatch:3B82F6:width=120:height=32:rx=16:shadow=3B82F6/8/0/0:label=STAGING/}}
{{ui:swatch:F59E0B:width=120:height=32:rx=16:shadow=F59E0B/8/0/0:label=DEVELOPMENT/}}
{{/ui}}

### Build Pipeline

| Stage | Status | Duration |
|-------|--------|----------|
| Build | {{ui:swatch:22C55E:width=80:height=24:rx=12:label=PASSED/}} | 2m 15s |
| Test | {{ui:swatch:22C55E:width=80:height=24:rx=12:label=PASSED/}} | 4m 32s |
| Lint | {{ui:swatch:22C55E:width=80:height=24:rx=12:label=PASSED/}} | 45s |
| Deploy | {{ui:swatch:3B82F6:width=80:height=24:rx=12:label=RUNNING/}} | 1m 20s |

### Health Check Timeline

{{ui:swatch:22C55E:width=40:height=20:rx=4/}} {{ui:swatch:22C55E:width=40:height=20:rx=4/}} {{ui:swatch:22C55E:width=40:height=20:rx=4/}} {{ui:swatch:F59E0B:width=40:height=20:rx=4/}} {{ui:swatch:22C55E:width=40:height=20:rx=4/}} {{ui:swatch:22C55E:width=40:height=20:rx=4/}} {{ui:swatch:22C55E:width=40:height=20:rx=4/}} {{ui:swatch:EF4444:width=40:height=20:rx=4/}} {{ui:swatch:22C55E:width=40:height=20:rx=4/}} {{ui:swatch:22C55E:width=40:height=20:rx=4/}}

---

## Volume & Sliders

### Audio Mixer

| Channel | Level |
|---------|-------|
| Master | {{ui:progress:85:width=200:height=12:thumb=16:thumb_color=white:fill=accent/}} |
| Vocals | {{ui:progress:70:width=200:height=12:thumb=16:thumb_color=white:fill=info/}} |
| Bass | {{ui:progress:65:width=200:height=12:thumb=16:thumb_color=white:fill=success/}} |
| Drums | {{ui:progress:75:width=200:height=12:thumb=16:thumb_color=white:fill=warning/}} |

### Brightness Control

{{ui:progress:60:width=300:height=16:thumb=20:thumb_shape=circle:fill=warning:track=ink:rx=8/}}

---

## Data Grids

### Repository Stats

| Repository | Stars | Activity | Health |
|------------|-------|----------|--------|
| core-api | {{ui:swatch:292a2d:width=60:height=24:rx=4:label=2.4k/}} | {{ui:sparkline:45,52,48,55,50,60,58:width=60:height=20:fill=success/}} | {{ui:donut:95:size=24:thickness=3:fill=success/}} |
| web-app | {{ui:swatch:292a2d:width=60:height=24:rx=4:label=1.8k/}} | {{ui:sparkline:30,35,42,38,45,48,52:width=60:height=20:fill=info/}} | {{ui:donut:88:size=24:thickness=3:fill=success/}} |
| mobile | {{ui:swatch:292a2d:width=60:height=24:rx=4:label=890/}} | {{ui:sparkline:20,25,22,28,30,35,32:width=60:height=20:fill=accent/}} | {{ui:donut:72:size=24:thickness=3:fill=warning/}} |
| cli-tools | {{ui:swatch:292a2d:width=60:height=24:rx=4:label=456/}} | {{ui:sparkline:10,15,12,18,20,22,25:width=60:height=20:fill=slate/}} | {{ui:donut:100:size=24:thickness=3:fill=success/}} |

---

## Creative Decorations

### Section Dividers

{{ui:swatch:000000:width=600:height=4:gradient=horizontal/f41c80/transparent/}}

{{ui:swatch:000000:width=600:height=2:gradient=horizontal/transparent/3B82F6/transparent/}}

{{ui:swatch:6b7280:width=400:height=2:stroke_dash=8/4/}}

### Floating Orbs

{{ui:row:align=center}}
{{ui:swatch:f41c80:width=50:height=50:rx=25:shadow=f41c80/15/0/0:opacity=0.9/}}
{{ui:swatch:3B82F6:width=40:height=40:rx=20:shadow=3B82F6/12/0/0:opacity=0.8/}}
{{ui:swatch:22C55E:width=30:height=30:rx=15:shadow=22C55E/10/0/0:opacity=0.7/}}
{{ui:swatch:F59E0B:width=25:height=25:rx=12:shadow=F59E0B/8/0/0:opacity=0.6/}}
{{/ui}}

### Gradient Cards

{{ui:swatch:000000:width=200:height=100:gradient=diagonal/f41c80/3B82F6:rx=12:shadow=f41c80/10/4/4/}}
{{ui:swatch:000000:width=200:height=100:gradient=diagonal/22C55E/3B82F6:rx=12:shadow=22C55E/10/4/4/}}
{{ui:swatch:000000:width=200:height=100:gradient=diagonal/F59E0B/EF4444:rx=12:shadow=F59E0B/10/4/4/}}

---

## Summary Stats

{{frame:star}}
{{ui:row:align=center}}
{{ui:donut:100:size=80:thickness=8:fill=success:label=true/}} Uptime
{{ui:gauge:96:size=80:thickness=8:fill=info:label=true/}} Rating
{{ui:donut:73:size=80:thickness=8:fill=accent:label=true/}} Sprint
{{/ui}}
{{/}}

---

{{ui:swatch:000000:width=700:height=4:gradient=horizontal/22C55E/3B82F6/f41c80:rx=2/}}

*Generated with mdfx - Markdown Effects for Beautiful Documentation*
