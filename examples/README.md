# mdfx Examples

Example templates and rendered outputs demonstrating mdfx capabilities.

## Gallery

| Example | Description |
|---------|-------------|
| [neon-tech-showcase](neon-tech-showcase.md) | Tech badges with custom styling, icons, and logo sizes |
| [tech-badges](tech-badges.md) | Technology badge styles and options |
| [dashboard-showcase](dashboard-showcase.md) | Dashboard layouts and status indicators |
| [donuts-and-gauges-gallery](donuts-and-gauges-gallery.md) | Donut charts and gauge visualizations |
| [progress-gallery](progress-gallery.md) | Progress bars and indicators |

## Documentation

- [Main README](../README.md)
- [Tech Badge Guide](../docs/guides/TECH-GUIDE.md)
- [API Guide](../docs/API-GUIDE.md)
- [Components Design](../docs/COMPONENTS.md)
- [Architecture](../docs/ARCHITECTURE.md)

## Processing Templates

```bash
cd examples
cargo run --release --bin mdfx -- process <template>.template.md -o <output>.md --assets-dir assets
```
