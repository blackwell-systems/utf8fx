# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:

**blackwellsystems@protonmail.com**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information (as much as you can provide):

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

This information will help us triage your report more quickly.

## Preferred Languages

We prefer all communications to be in English.

## Disclosure Policy

We follow the principle of [Coordinated Vulnerability Disclosure](https://vuls.cert.org/confluence/display/CVD).

Once a security issue is reported:

1. **Confirmation**: We'll acknowledge receipt within 48 hours
2. **Investigation**: We'll investigate and determine severity (typically 1-7 days)
3. **Fix Development**: We'll develop and test a fix
4. **Coordinated Disclosure**:
   - We'll notify you when the fix is ready
   - We'll agree on a disclosure timeline (typically 90 days from initial report)
   - We'll release the fix and publicly disclose the issue
5. **Credit**: We'll credit you in the security advisory (unless you prefer to remain anonymous)

## Security Update Process

When a security vulnerability is fixed:

1. A new patch version is released (e.g., 1.0.1 → 1.0.2)
2. A GitHub Security Advisory is published
3. The CHANGELOG is updated with security fix details
4. Users are notified via GitHub release notes

## Known Security Considerations

### Template Processing

mdfx processes untrusted markdown templates. While the library is designed to be safe:

- **No code execution**: Templates do not execute arbitrary code
- **No file access**: Processing is memory-only (SVG backend writes are explicit)
- **Resource limits**: Parser has no inherent resource limits (callers should implement timeouts)

### Dependency Security

We regularly audit dependencies for known vulnerabilities using:
- `cargo audit` in CI/CD
- Dependabot security updates
- Manual review of critical dependencies

### SVG Backend

The SVG backend generates SVG files based on user input:

- **No script injection**: SVG output contains no `<script>` tags
- **Sanitized input**: Colors and styles are validated
- **Path traversal protection**: Output paths are relative and validated

## Security Best Practices

When using mdfx in your application:

### 1. Validate Input Sources

If processing user-supplied templates:

```rust
// Set reasonable limits
const MAX_TEMPLATE_SIZE: usize = 1024 * 1024; // 1MB

if template.len() > MAX_TEMPLATE_SIZE {
    return Err("Template too large");
}
```

### 2. Implement Timeouts

For long-running processing:

```rust
use std::time::Duration;
use tokio::time::timeout;

let result = timeout(
    Duration::from_secs(5),
    async { parser.process(template) }
).await?;
```

### 3. Isolate Processing

Process untrusted templates in isolated contexts:

- Use separate threads/processes for user content
- Apply resource limits (memory, CPU)
- Run in sandboxed environments when possible

### 4. Validate Output Paths

When using SVG backend with user-controlled paths:

```rust
use std::path::Path;

fn validate_output_path(path: &str) -> Result<PathBuf, Error> {
    let path = Path::new(path).canonicalize()?;

    // Ensure within allowed directory
    if !path.starts_with("/allowed/output/dir") {
        return Err("Invalid output path");
    }

    Ok(path)
}
```

## Scope

### In Scope

- mdfx library code (`crates/mdfx/src/`)
- mdfx-cli binary code (`crates/mdfx-cli/src/`)
- Template parser vulnerabilities
- SVG generation vulnerabilities
- Dependency vulnerabilities

### Out of Scope

- Vulnerabilities in user code that uses mdfx
- Issues in shields.io (external service)
- Vulnerabilities in documentation/examples (non-production code)
- Social engineering attacks

## Comments on This Policy

If you have suggestions on how this process could be improved, please submit a pull request or open an issue.

---

**Last Updated**: 2025-12-13
