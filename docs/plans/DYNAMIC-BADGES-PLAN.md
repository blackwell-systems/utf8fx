# Dynamic Badges Implementation Plan

## Overview

Add dynamic badges that fetch live data from external APIs (GitHub, npm, crates.io, etc.) to compete with shields.io's killer feature.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     mdfx-cli                                │
│   process --offline --refresh --cache-dir                   │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                     mdfx (core)                             │
│   ComponentsRenderer → FetchContext → DataSource            │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                    mdfx-fetch                               │
│   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│   │ DataSource  │  │   Cache     │  │  Fetcher    │        │
│   │   trait     │  │  (disk)     │  │  (HTTP)     │        │
│   └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                             │
│   Sources:                                                  │
│   - GitHubSource (stars, forks, issues, PRs, license)      │
│   - NpmSource (downloads, version)                          │
│   - CratesSource (downloads, version)                       │
│   - PyPiSource (downloads, version)                         │
└─────────────────────────────────────────────────────────────┘
```

## Phase 1: Core Infrastructure (mdfx-fetch crate)

### 1.1 Create mdfx-fetch crate

```rust
// crates/mdfx-fetch/src/lib.rs
pub mod cache;
pub mod error;
pub mod source;
pub mod fetcher;

pub use cache::Cache;
pub use error::{FetchError, Result};
pub use source::{DataSource, DataValue};
pub use fetcher::Fetcher;
```

### 1.2 DataSource Trait

```rust
// crates/mdfx-fetch/src/source.rs
use std::collections::HashMap;

/// Value types that can be fetched
#[derive(Debug, Clone)]
pub enum DataValue {
    Number(u64),
    String(String),
    Bool(bool),
}

/// Trait for data sources (GitHub, npm, etc.)
pub trait DataSource: Send + Sync {
    /// Source identifier (e.g., "github", "npm")
    fn id(&self) -> &str;

    /// Fetch data for a given query
    /// Query format depends on source (e.g., "owner/repo" for GitHub)
    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue>;

    /// List available metrics
    fn available_metrics(&self) -> Vec<&str>;

    /// Default TTL for caching (in seconds)
    fn default_ttl(&self) -> u64 {
        3600 // 1 hour default
    }
}
```

### 1.3 Cache Layer

```rust
// crates/mdfx-fetch/src/cache.rs
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct Cache {
    dir: PathBuf,
}

pub struct CacheEntry {
    pub value: DataValue,
    pub fetched_at: SystemTime,
    pub ttl: Duration,
}

impl Cache {
    pub fn new(dir: PathBuf) -> Self;
    pub fn get(&self, key: &str) -> Option<CacheEntry>;
    pub fn set(&self, key: &str, value: DataValue, ttl: Duration);
    pub fn is_expired(&self, entry: &CacheEntry) -> bool;
    pub fn clear(&self);
}
```

### 1.4 Fetcher (HTTP client)

```rust
// crates/mdfx-fetch/src/fetcher.rs
use ureq; // Lightweight HTTP client

pub struct Fetcher {
    cache: Cache,
    offline: bool,
}

impl Fetcher {
    pub fn new(cache_dir: PathBuf, offline: bool) -> Self;
    pub fn fetch(&self, source: &dyn DataSource, query: &str, metric: &str) -> Result<DataValue>;
}
```

## Phase 2: Data Sources

### 2.1 GitHub Source

API: `https://api.github.com/repos/{owner}/{repo}`

Metrics:
- `stars` - stargazers_count
- `forks` - forks_count
- `issues` - open_issues_count
- `watchers` - subscribers_count
- `license` - license.spdx_id
- `language` - language
- `size` - size (KB)

### 2.2 npm Source

API: `https://registry.npmjs.org/{package}`

Metrics:
- `version` - dist-tags.latest
- `downloads` - (separate endpoint)

### 2.3 Crates.io Source

API: `https://crates.io/api/v1/crates/{name}`

Metrics:
- `version` - crate.max_version
- `downloads` - crate.downloads

### 2.4 PyPI Source

API: `https://pypi.org/pypi/{package}/json`

Metrics:
- `version` - info.version
- `license` - info.license

## Phase 3: Component Integration

### 3.1 New Component Syntax

```markdown
<!-- GitHub metrics -->
{{ui:github:owner/repo:stars/}}
{{ui:github:owner/repo:forks/}}
{{ui:github:owner/repo:license/}}

<!-- npm metrics -->
{{ui:npm:package-name:version/}}
{{ui:npm:package-name:downloads/}}

<!-- Crates.io metrics -->
{{ui:crates:crate-name:version/}}
{{ui:crates:crate-name:downloads/}}

<!-- Custom styling -->
{{ui:github:rust-lang/rust:stars:bg=1a1a2e:border=FFD700/}}
```

### 3.2 FetchContext

Add to ComponentsRenderer:

```rust
pub struct FetchContext {
    fetcher: Option<Fetcher>,
    sources: HashMap<String, Box<dyn DataSource>>,
}

impl ComponentsRenderer {
    pub fn with_fetch(fetcher: Fetcher) -> Self;

    fn expand_dynamic(&self, source: &str, args: &[String], params: &HashMap<String, String>) -> Result<ComponentOutput>;
}
```

### 3.3 Component Handlers

```rust
// handlers/github.rs
pub fn handle(args: &[String], params: &HashMap<String, String>, fetcher: &Fetcher) -> Result<ComponentOutput> {
    // args[0] = "owner/repo"
    // params.get("metric") or args[1] = "stars"
    let value = fetcher.fetch(&GitHubSource, &args[0], metric)?;

    // Format value as badge
    let label = format_metric_label(metric);
    Ok(ComponentOutput::Primitive(Primitive::Swatch {
        color: determine_color(metric, &value),
        label: Some(format!("{}: {}", label, format_value(&value))),
        ...
    }))
}
```

## Phase 4: CLI Integration

### 4.1 New CLI Flags

```rust
// main.rs Commands::Process
#[arg(long)]
offline: bool,  // Don't fetch, use cache only

#[arg(long)]
refresh: bool,  // Ignore cache, fetch fresh data

#[arg(long, default_value = ".mdfx-cache")]
cache_dir: String,  // Cache directory
```

### 4.2 Config File Support

```json
// .mdfx.json
{
    "fetch": {
        "offline": false,
        "cache_dir": ".mdfx-cache",
        "ttl": {
            "github": 3600,
            "npm": 86400
        }
    }
}
```

## Phase 5: Error Handling & Fallbacks

### 5.1 Graceful Degradation

```rust
// When fetch fails:
1. Check cache (even if expired)
2. Show "N/A" badge with warning color
3. Log warning to stderr
4. Continue processing (don't fail entire document)
```

### 5.2 Rate Limiting

- GitHub: 60 req/hour (unauthenticated)
- npm: No limit
- crates.io: No limit but be nice

Add request throttling to prevent hitting rate limits.

## Implementation Order

1. **Week 1**: mdfx-fetch crate skeleton
   - Create crate with error types
   - Implement Cache (file-based JSON)
   - Add ureq dependency for HTTP

2. **Week 2**: GitHub source
   - Implement GitHubSource
   - Add github component handler
   - Test with real API

3. **Week 3**: Integration
   - Wire up FetchContext to ComponentsRenderer
   - Add CLI flags
   - Handle offline mode

4. **Week 4**: Additional sources + polish
   - npm, crates.io, pypi sources
   - Documentation
   - Error messages & fallbacks

## Dependencies

```toml
# crates/mdfx-fetch/Cargo.toml
[dependencies]
ureq = { version = "2.9", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
```

## Testing Strategy

1. Unit tests with mock HTTP responses
2. Integration tests with real API (gated behind feature flag)
3. Cache tests with temp directories
4. Offline mode tests

## Security Considerations

1. No auth tokens in code (environment variables only)
2. Validate API responses before parsing
3. Sanitize user input in queries
4. HTTPS only for all requests
