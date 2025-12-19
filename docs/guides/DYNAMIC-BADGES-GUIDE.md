# Dynamic Badges Guide

Dynamic badges fetch live data from external APIs to display real-time metrics like GitHub stars, npm versions, and crate downloads.

## Requirements

Dynamic badges require the `fetch` feature to be enabled:

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
{{ui:github:owner/repo:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `stars` | Star count (default) | `{{ui:github:rust-lang/rust:stars/}}` |
| `forks` | Fork count | `{{ui:github:facebook/react:forks/}}` |
| `issues` | Open issue count | `{{ui:github:microsoft/vscode:issues/}}` |
| `watchers` | Watcher count | `{{ui:github:torvalds/linux:watchers/}}` |
| `license` | SPDX license identifier | `{{ui:github:rust-lang/rust:license/}}` |
| `language` | Primary language | `{{ui:github:rust-lang/rust:language/}}` |

**Examples:**
```markdown
{{ui:github:rust-lang/rust/}}           <!-- stars (default) -->
{{ui:github:rust-lang/rust:forks/}}     <!-- forks -->
{{ui:github:rust-lang/rust:license/}}   <!-- license -->
```

### npm

Fetch package information from the npm registry.

**Syntax:**
```markdown
{{ui:npm:package-name:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `version` | Latest stable version (default) | `{{ui:npm:react:version/}}` |
| `license` | Package license | `{{ui:npm:typescript:license/}}` |
| `next` | Latest @next tag | `{{ui:npm:react:next/}}` |
| `beta` | Latest @beta tag | `{{ui:npm:vue:beta/}}` |

**Examples:**
```markdown
{{ui:npm:react/}}                <!-- version (default) -->
{{ui:npm:typescript:license/}}   <!-- license -->
{{ui:npm:react:next/}}           <!-- next version -->
```

### crates.io (Rust)

Fetch crate information from crates.io.

**Syntax:**
```markdown
{{ui:crates:crate-name:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `version` | Latest version (default) | `{{ui:crates:serde:version/}}` |
| `downloads` | Total download count | `{{ui:crates:tokio:downloads/}}` |
| `description` | Crate description | `{{ui:crates:clap:description/}}` |

**Examples:**
```markdown
{{ui:crates:serde/}}              <!-- version (default) -->
{{ui:crates:tokio:downloads/}}    <!-- downloads -->
```

### PyPI (Python)

Fetch package information from PyPI.

**Syntax:**
```markdown
{{ui:pypi:package-name:metric/}}
```

**Metrics:**
| Metric | Description | Example |
|--------|-------------|---------|
| `version` | Latest version (default) | `{{ui:pypi:requests:version/}}` |
| `license` | Package license | `{{ui:pypi:flask:license/}}` |
| `author` | Package author | `{{ui:pypi:django:author/}}` |
| `python` | Required Python version | `{{ui:pypi:numpy:python/}}` |
| `summary` | Package summary | `{{ui:pypi:pytest:summary/}}` |

**Examples:**
```markdown
{{ui:pypi:requests/}}           <!-- version (default) -->
{{ui:pypi:numpy:python/}}       <!-- Python requirement -->
{{ui:pypi:flask:license/}}      <!-- license -->
```

## Styling Options

Dynamic badges support the same styling options as other components:

```markdown
<!-- Custom background color -->
{{ui:github:rust-lang/rust:bg=1a1a2e/}}

<!-- Custom text color -->
{{ui:npm:react:text=white/}}

<!-- Badge style -->
{{ui:crates:serde:style=pill/}}

<!-- Custom icon -->
{{ui:pypi:requests:icon=python/}}

<!-- Width override -->
{{ui:github:rust-lang/rust:width=200/}}
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

Dynamic badges handle errors gracefully:

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
| Stars | {{ui:github:myorg/myrepo:stars/}} |
| Version | {{ui:npm:my-package:version/}} |
| Downloads | {{ui:crates:my-crate:downloads/}} |
| License | {{ui:github:myorg/myrepo:license/}} |
```

### Multi-Language Project

```markdown
## Package Versions

- **npm**: {{ui:npm:my-package:version/}}
- **crates.io**: {{ui:crates:my-crate:version/}}
- **PyPI**: {{ui:pypi:my-package:version/}}
```

### GitHub Project Stats

```markdown
{{ui:github:rust-lang/rust:stars/}}
{{ui:github:rust-lang/rust:forks/}}
{{ui:github:rust-lang/rust:issues/}}
{{ui:github:rust-lang/rust:license/}}
{{ui:github:rust-lang/rust:language/}}
```
