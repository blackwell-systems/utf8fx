# utf8fx Examples

This directory contains demonstration files showing utf8fx capabilities.

## Files

### demo-input.md
Markdown file with template syntax showing all 11 Unicode styles.

### demo-output.md
The processed result with Unicode characters applied.

## Comparison

**Before (demo-input.md):**
```markdown
# {{mathbold}}UNICODE STYLING DEMO{{/mathbold}}
```

**After (demo-output.md):**
```markdown
# 𝐔𝐍𝐈𝐂𝐎𝐃𝐄 𝐒𝐓𝐘𝐋𝐈𝐍𝐆 𝐃𝐄𝐌𝐎
```

## Try It Yourself

```bash
# View original
cat examples/demo-input.md

# Process and view result
utf8fx process examples/demo-input.md

# Compare with saved output
diff <(utf8fx process examples/demo-input.md) examples/demo-output.md
```

## Template Syntax

```markdown
{{style}}content{{/style}}
```

**Available styles:**
- `mathbold` (alias: `mb`)
- `fullwidth` (alias: `fw`)
- `negative-squared` (alias: `neg-sq`)
- `negative-circled` (alias: `neg-circle`)
- `squared-latin` (alias: `sq-latin`)
- `circled-latin` (alias: `circled`, `circle`)
- `small-caps` (alias: `sc`)
- `monospace` (alias: `mono`)
- `double-struck` (alias: `ds`)
- `sans-serif` (alias: `ss`, `sans`)
- `sans-serif-bold` (alias: `ssb`)
- `sans-serif-italic` (alias: `ssi`, `sans-italic`)
- `sans-serif-bold-italic` (alias: `ssbi`, `sans-bold-italic`)
- `italic` (alias: `it`)
- `bold-italic` (alias: `bi`)
- `script` (alias: `scr`, `cursive`, `calligraphic`)
- `bold-script` (alias: `bscr`, `bold-cursive`)
- `fraktur` (alias: `fr`, `gothic`, `blackletter`)
- `bold-fraktur` (alias: `bfr`, `bold-gothic`)

## Integration Examples

### Hugo Blog

```bash
# Process all posts before building
utf8fx process content/posts/*.md --in-place
hugo build
```

### Jekyll

```bash
# Process posts
utf8fx process _posts/*.md --in-place
jekyll build
```

### MkDocs

```bash
# Process docs
utf8fx process docs/**/*.md --in-place
mkdocs build
```

### As a Build Step

```makefile
.PHONY: preprocess build

preprocess:
	utf8fx process content/**/*.md --in-place

build: preprocess
	hugo build
```
