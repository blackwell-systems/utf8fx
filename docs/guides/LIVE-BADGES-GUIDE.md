# Live Badges Guide

Live badges fetch data from external APIs to display real-time metrics like GitHub stars, npm versions, and crate downloads.

## Syntax

All live data badges use the `live:` namespace:

```markdown
{{ui:live:source:query:metric/}}
```

Where:
- `source` - Data source: `github`, `npm`, `crates`, or `pypi`
- `query` - Source-specific query (repo, package name, etc.)
- `metric` - Metric to fetch (optional, defaults vary by source)

## Requirements

Live badges require the `fetch` feature to be enabled:

```bash
cargo build --release --features fetch
```

Or when running:

```bash
cargo run --release --features fetch -- process input.template.md -o output.md
```

## Available Sources

### GitHub

Fetch live repository metrics from GitHub.

**Syntax:**
```markdown
{{ui:live:github:owner/repo:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `stars` | Star count (default) | `{{ui:live:github:rust-lang/rust:stars/}}` |
| `forks` | Fork count | `{{ui:live:github:facebook/react:forks/}}` |
| `issues` | Open issue count | `{{ui:live:github:microsoft/vscode:issues/}}` |
| `watchers` | Watcher count | `{{ui:live:github:torvalds/linux:watchers/}}` |
| `license` | SPDX license identifier | `{{ui:live:github:rust-lang/rust:license/}}` |
| `language` | Primary language | `{{ui:live:github:rust-lang/rust:language/}}` |

**Examples:**
```markdown
{{ui:live:github:rust-lang/rust/}}           <!-- stars (default) -->
{{ui:live:github:rust-lang/rust:forks/}}     <!-- forks -->
{{ui:live:github:rust-lang/rust:license/}}   <!-- license -->
```

### npm

Fetch package information from the npm registry.

**Syntax:**
```markdown
{{ui:live:npm:package-name:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `version` | Latest stable version (default) | `{{ui:live:npm:react:version/}}` |
| `license` | Package license | `{{ui:live:npm:typescript:license/}}` |
| `next` | Latest @next tag | `{{ui:live:npm:react:next/}}` |
| `beta` | Latest @beta tag | `{{ui:live:npm:vue:beta/}}` |

**Examples:**
```markdown
{{ui:live:npm:react/}}                <!-- version (default) -->
{{ui:live:npm:typescript:license/}}   <!-- license -->
{{ui:live:npm:react:next/}}           <!-- next version -->
```

### crates.io (Rust)

Fetch crate information from crates.io.

**Syntax:**
```markdown
{{ui:live:crates:crate-name:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `version` | Latest version (default) | `{{ui:live:crates:serde:version/}}` |
| `downloads` | Total download count | `{{ui:live:crates:tokio:downloads/}}` |
| `description` | Crate description | `{{ui:live:crates:clap:description/}}` |

**Examples:**
```markdown
{{ui:live:crates:serde/}}              <!-- version (default) -->
{{ui:live:crates:tokio:downloads/}}    <!-- downloads -->
```

### PyPI (Python)

Fetch package information from PyPI.

**Syntax:**
```markdown
{{ui:live:pypi:package-name:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `version` | Latest version (default) | `{{ui:live:pypi:requests:version/}}` |
| `license` | Package license | `{{ui:live:pypi:flask:license/}}` |
| `author` | Package author | `{{ui:live:pypi:django:author/}}` |
| `python` | Required Python version | `{{ui:live:pypi:numpy:python/}}` |
| `summary` | Package summary | `{{ui:live:pypi:pytest:summary/}}` |

**Examples:**
```markdown
{{ui:live:pypi:requests/}}           <!-- version (default) -->
{{ui:live:pypi:numpy:python/}}       <!-- Python requirement -->
{{ui:live:pypi:flask:license/}}      <!-- license -->
```

## Styling Options

Live badges support the same styling options as other components:

```markdown
<!-- Custom background color -->
{{ui:live:github:rust-lang/rust:stars:bg=1a1a2e/}}

<!-- Custom text color -->
{{ui:live:npm:react:version:text=white/}}

<!-- Badge style -->
{{ui:live:crates:serde:version:style=pill/}}

<!-- Custom icon -->
{{ui:live:pypi:requests:version:icon=python/}}

<!-- Width override -->
{{ui:live:github:rust-lang/rust:stars:width=200/}}
```

## CLI Options

### Offline Mode

Run in offline mode to only use cached data:

```bash
mdfx process --offline input.template.md -o output.md
```

In offline mode:
- No network requests are made
- Only cached data is used
- Errors if no cache exists for a badge

### Force Refresh

Force fetching fresh data, ignoring cache:

```bash
mdfx process --refresh input.template.md -o output.md
```

### Custom Cache Directory

Specify a custom cache directory:

```bash
mdfx process --cache-dir /tmp/mdfx-cache input.template.md -o output.md
```

Default cache directory is `.mdfx-cache` in the current working directory.

## Caching Behavior

- Data is cached on disk as JSON files
- Default TTL (Time To Live) is 1 hour
- Cache keys are based on source, query, and metric
- Stale cache is used as fallback on network errors
- Cache is organized by source ID

**Cache structure:**
```
.mdfx-cache/
├── github/
│   ├── rust-lang_rust_stars.json
│   └── facebook_react_forks.json
├── npm/
│   ├── react_version.json
│   └── typescript_license.json
├── crates/
│   └── serde_version.json
└── pypi/
    └── requests_version.json
```

## Error Handling

Live badges handle errors gracefully:

1. **Network errors**: Fall back to stale cache if available
2. **API errors**: Display error message in badge
3. **Rate limiting**: Use cached data, warn user
4. **Invalid query**: Display error in badge

## Rate Limits

Be aware of API rate limits:

| Source | Rate Limit | Notes |
|--------|------------|-------|
| GitHub | 60 req/hour | Unauthenticated |
| npm | No limit | Be respectful |
| crates.io | No limit | Has user-agent requirement |
| PyPI | No limit | Be respectful |

## Examples

### Project Status Dashboard

```markdown
# My Awesome Project

| Metric | Value |
|--------|-------|
| Stars | {{ui:live:github:myorg/myrepo:stars/}} |
| Version | {{ui:live:npm:my-package:version/}} |
| Downloads | {{ui:live:crates:my-crate:downloads/}} |
| License | {{ui:live:github:myorg/myrepo:license/}} |
```

### Multi-Language Project

```markdown
## Package Versions

- **npm**: {{ui:live:npm:my-package:version/}}
- **crates.io**: {{ui:live:crates:my-crate:version/}}
- **PyPI**: {{ui:live:pypi:my-package:version/}}
```

### GitHub Project Stats

```markdown
{{ui:live:github:rust-lang/rust:stars/}}
{{ui:live:github:rust-lang/rust:forks/}}
{{ui:live:github:rust-lang/rust:issues/}}
{{ui:live:github:rust-lang/rust:license/}}
{{ui:live:github:rust-lang/rust:language/}}
```
