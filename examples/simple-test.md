# Simple Multi-Backend Test

## Basic Components

{{ui:divider/}}

### Swatches
{{ui:swatch:accent/}} Accent color
{{ui:swatch:success/}} Success green

### Status Indicators
{{ui:status:success/}} All good
{{ui:status:warning/}} Needs attention
{{ui:status:error/}} Critical issue

### Tech Badges
{{ui:tech:rust/}} {{ui:tech:typescript/}} {{ui:tech:postgresql/}}

{{ui:divider/}}

## In Tables

| Service | Stack | Status |
|---------|-------|--------|
| API | {{ui:tech:rust/}} | {{ui:status:success/}} |
| Frontend | {{ui:tech:typescript/}} | {{ui:status:success/}} |
| Database | {{ui:tech:postgresql/}} | {{ui:status:warning/}} |

{{ui:divider/}}

## With Frames

{{frame:gradient}}
### {{mathbold}}Framed Content{{/mathbold}}
{{ui:swatch:accent/}} This is inside a gradient frame
{{ui:status:success/}} Everything works
{{/frame}}

{{ui:divider/}}

Done!
