# Contributing to mdfx

Thank you for your interest in contributing to mdfx! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Testing](#testing)
- [Code Style](#code-style)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to blackwellsystems@protonmail.com.

## Getting Started

### Areas for Contribution

We welcome contributions in several areas:

**🐛 Bug Fixes**
- Fix parsing edge cases
- Resolve rendering issues
- Address platform-specific bugs

**✨ New Features**
- Add new Unicode styles
- Implement new UI components
- Add rendering backends
- Improve CLI ergonomics

**📚 Documentation**
- Improve API documentation
- Add more examples
- Fix typos and clarify explanations
- Add tutorials

**🧪 Testing**
- Add test coverage
- Create integration tests
- Test cross-platform behavior

**🎨 Design**
- Design new frame styles
- Create component templates
- Improve color palettes

## How to Contribute

### Reporting Bugs

Before creating a bug report:
1. Check the [issue tracker](https://github.com/blackwell-systems/mdfx/issues)
2. Search for similar issues
3. Verify the bug exists in the latest version

**Good bug reports include:**
- Clear, descriptive title
- Steps to reproduce
- Expected vs actual behavior
- mdfx version (`mdfx --version`)
- Operating system
- Minimal code example

**Example:**

```markdown
**Bug**: Frame processing fails on multi-line content

**Steps to Reproduce**:
1. Create file with `{{frame:gradient}}\nLine 1\nLine 2\n{{/frame}}`
2. Run `mdfx process input.md`

**Expected**: Frame wraps all lines
**Actual**: Error: "Unclosed tag: {{frame}}"

**Environment**:
- mdfx version: 1.0.0
- OS: Ubuntu 22.04
- Rust: 1.75.0
```

### Suggesting Features

Feature requests should include:
- **Use case**: Why is this feature needed?
- **Proposed solution**: How should it work?
- **Alternatives considered**: What other approaches did you consider?
- **Examples**: Show what the API would look like

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A code editor (VS Code with rust-analyzer recommended)

### Setup Instructions

1. **Fork the repository**
   ```bash
   # Visit https://github.com/blackwell-systems/mdfx
   # Click "Fork" button
   ```

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/mdfx.git
   cd mdfx
   ```

3. **Add upstream remote**
   ```bash
   git remote add upstream https://github.com/blackwell-systems/mdfx.git
   ```

4. **Install dependencies**
   ```bash
   cargo build
   ```

5. **Run tests**
   ```bash
   cargo test --workspace
   ```

### Project Structure

```
mdfx/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── mdfx/                     # Core library
│   │   ├── Cargo.toml           # Library dependencies (4 total)
│   │   ├── data/                # JSON configuration
│   │   │   └── registry.json    # Unified registry (components, palette, styles, frames)
│   │   └── src/
│   │       ├── lib.rs           # Public API
│   │       ├── converter.rs     # Character transformations
│   │       ├── parser.rs        # Template parser
│   │       ├── components.rs    # Component expansion
│   │       ├── primitive.rs     # Primitive AST
│   │       ├── registry.rs      # Unified registry
│   │       ├── shields.rs       # Shields renderer
│   │       ├── styles.rs        # Style renderer
│   │       ├── targets.rs       # Target backends (GitHub, local)
│   │       ├── error.rs         # Error types
│   │       └── manifest.rs      # Asset manifest
│   └── mdfx-cli/                # CLI application
│       ├── Cargo.toml           # CLI dependencies
│       └── src/main.rs          # CLI implementation
├── docs/                         # Documentation
├── examples/                     # Usage examples
└── assets/                       # Generated assets
```

## Testing

### Running Tests

```bash
# All tests
cargo test --workspace

# Library only
cargo test -p mdfx

# CLI only
cargo test -p mdfx-cli

# With output
cargo test -- --nocapture

# Specific test
cargo test test_frame_multiline
```

### Writing Tests

Tests go in the same file as the code they test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathbold_conversion() {
        let converter = Converter::new().unwrap();
        let result = converter.convert("ABC", "mathbold").unwrap();
        assert_eq!(result, "𝐀𝐁𝐂");
    }
}
```

### Test Coverage

We aim for:
- **80%+ coverage** for core library code
- **All primitives** have unit tests
- **Multi-line constructs** have integration tests
- **Error cases** are tested

### Integration Tests

For end-to-end testing:

```rust
#[test]
fn test_full_template_processing() {
    let parser = TemplateParser::new().unwrap();
    let input = "# {{ui:header}}TEST{{/ui}}";
    let output = parser.process(input).unwrap();
    assert!(output.contains("▓▒░"));
    assert!(output.contains("𝐓"));
}
```

## Code Style

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` and fix all warnings
- Use meaningful variable names
- Add doc comments for public APIs

### Documentation Comments

```rust
/// Converts text to a Unicode style with optional spacing.
///
/// # Arguments
///
/// * `text` - The text to convert
/// * `style` - Style name (e.g., "mathbold", "script")
/// * `spacing` - Number of spaces between characters
///
/// # Returns
///
/// Converted text with spacing applied
///
/// # Errors
///
/// Returns `Error::StyleNotFound` if style doesn't exist.
///
/// # Example
///
/// ```
/// use mdfx::Converter;
///
/// let converter = Converter::new()?;
/// let result = converter.convert_with_spacing("ABC", "mathbold", 1)?;
/// assert_eq!(result, "𝐀 𝐁 𝐂");
/// ```
pub fn convert_with_spacing(&self, text: &str, style: &str, spacing: usize) -> Result<String>
```

### Error Handling

Use `thiserror` for error types:

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Style not found: {0}")]
    StyleNotFound(String),

    #[error("Invalid template syntax: {0}")]
    InvalidSyntax(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, not CSS)
- `refactor`: Code restructuring
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Build/tooling changes

### Examples

```bash
feat(parser): add support for nested frames

Allows frames to be nested within other frames by tracking
frame depth during parsing.

Closes #123
```

```bash
fix(converter): handle empty string input

Previously crashed on empty input. Now returns empty string.
```

```bash
docs(api-guide): add examples for SVG backend

Added three examples showing SVG backend usage with different
asset directory configurations.
```

### Scope Guidelines

- `parser` - Template parsing
- `converter` - Character conversion
- `components` - UI components
- `renderer` - Backend rendering
- `cli` - CLI tool
- `deps` - Dependencies
- `ci` - CI/CD changes

## Pull Request Process

### Before Submitting

1. **Create a branch**
   ```bash
   git checkout -b feat/add-new-style
   ```

2. **Make changes**
   - Write code
   - Add tests
   - Update documentation

3. **Test thoroughly**
   ```bash
   cargo test --workspace
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

4. **Commit changes**
   ```bash
   git add .
   git commit -m "feat: add new Unicode style"
   ```

5. **Push to your fork**
   ```bash
   git push origin feat/add-new-style
   ```

### Creating the PR

1. Go to your fork on GitHub
2. Click "New Pull Request"
3. Select your branch
4. Fill out the PR template:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation
- [ ] Refactor

## Testing
- [ ] All tests pass
- [ ] Added new tests
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings
```

### Review Process

1. **Automated Checks**: CI runs tests and linting
2. **Code Review**: Maintainer reviews code
3. **Feedback**: Address any requested changes
4. **Approval**: Maintainer approves PR
5. **Merge**: PR is merged to main

### After Merge

1. **Delete branch**
   ```bash
   git branch -d feat/add-new-style
   git push origin --delete feat/add-new-style
   ```

2. **Update main**
   ```bash
   git checkout main
   git pull upstream main
   ```

## Release Process

Releases are handled by maintainers:

1. **Version Bump**: Update version in `Cargo.toml`
2. **Changelog**: Update `CHANGELOG.md`
3. **Tag**: Create git tag (`v1.0.1`)
4. **Publish**: `cargo publish` to crates.io
5. **GitHub Release**: Create release on GitHub

## Questions?

- **General questions**: Open a [Discussion](https://github.com/blackwell-systems/mdfx/discussions)
- **Bug reports**: Open an [Issue](https://github.com/blackwell-systems/mdfx/issues)
- **Security**: Email blackwellsystems@protonmail.com

---

Thank you for contributing to mdfx! 🎉
