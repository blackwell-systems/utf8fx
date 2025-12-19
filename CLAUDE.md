# Claude Development Guide for mdfx

This document provides guidelines for AI assistants working with the mdfx project.

## Project Structure

```
mdfx/
├── crates/
│   ├── mdfx/           # Main library crate
│   ├── mdfx-cli/       # CLI binary
│   ├── badgefx/        # Badge rendering library (tech badges)
│   ├── mdfx-colors/    # Color utilities (contrast, luminance, darken)
│   └── mdfx-icons/     # Icon paths and brand colors
├── examples/
│   ├── assets/         # Generated SVG assets (tracked in git)
│   ├── *.template.md   # Source templates with mdfx syntax
│   └── *.md            # Rendered output markdown
└── CLAUDE.md           # This file
```

## Asset Generation Workflow

### Rendering Examples

To render a template file to its output markdown with SVG assets:

```bash
cargo run --release -- process -b svg --assets-dir examples/assets -o examples/OUTPUT.md examples/INPUT.template.md
```

**Key Points:**
- Assets are output to `examples/assets/`
- Assets are named with content hashes (e.g., `tech_abc123.svg`)
- The `--assets-dir` flag controls where SVGs are written
- Assets with unchanged content/hash are NOT regenerated

### Asset Path References

**IMPORTANT:** Asset paths in rendered markdown must be relative to the markdown file's location:

- If `examples/tech-badges.md` references an asset, use: `assets/foo.svg`
- Do NOT use: `examples/assets/foo.svg` (this breaks GitHub rendering)

After rendering, fix paths if needed:
```bash
sed -i 's|examples/assets/|assets/|g' examples/OUTPUT.md
```

### Forcing Asset Regeneration

Assets are cached by hash. To force regeneration (e.g., after fixing rendering bugs):

```bash
# Delete specific asset(s)
rm examples/assets/tech_HASH.svg

# Re-render - deleted assets will be regenerated
cargo run --release -- process -b svg --assets-dir examples/assets -o examples/OUTPUT.md examples/INPUT.template.md
```

### Verifying Existing Assets

Before making changes, preserve existing asset checksums:
```bash
md5sum examples/assets/tech_*.svg > /tmp/assets_before.md5
```

After changes, verify no unintended modifications:
```bash
# Check if any existing assets changed
while read line; do
    old_md5=$(echo "$line" | awk '{print $1}')
    file=$(echo "$line" | awk '{print $2}')
    if [ -f "$file" ]; then
        new_md5=$(md5sum "$file" | awk '{print $1}')
        if [ "$old_md5" != "$new_md5" ]; then
            echo "MODIFIED: $file"
        fi
    fi
done < /tmp/assets_before.md5
```

## Badge Rendering (badgefx)

### Architecture

- `badgefx/src/badge.rs` - TechBadge struct and BadgeBuilder
- `badgefx/src/render.rs` - SVG rendering functions
- `badgefx/src/shapes.rs` - SVG path generation
- `badgefx/src/style.rs` - Badge styles and metrics

### Common Issues

1. **Path Artifacts on Pill Badges**: Corner radii must be clamped to `min(radius, height/2, width/2)` to prevent self-intersecting paths.

2. **Seams Between Segments**: Adjacent rectangles need 1px overlap to prevent anti-aliasing artifacts.

3. **Asset Not Updating**: Delete the specific asset file to force regeneration.

## Git Workflow

### What to Commit

- All source code changes
- Rendered markdown files (`examples/*.md`)
- Generated SVG assets (`examples/assets/*.svg`)
- Manifest files (`examples/assets/manifest.json`)

### Asset Directory

The `examples/assets/` directory is tracked in git (NOT ignored). This ensures:
- Assets are available on GitHub for README rendering
- Consistent asset hashes across builds

## Testing

```bash
# Run all tests
cargo test --release

# Run specific crate tests
cd crates/badgefx && cargo test --release
```

## Common Commands

```bash
# Build release
cargo build --release

# Render tech-badges showcase
cargo run --release -- process -b svg --assets-dir examples/assets -o examples/tech-badges.md examples/tech-badges.template.md

# Fix asset paths for GitHub
sed -i 's|examples/assets/|assets/|g' examples/tech-badges.md

# Check for path artifacts in SVGs
grep -l "Q0 20 0 0\|Q0 0 20 0" examples/assets/*.svg
```
