/// Basic usage examples for mdfx library
use mdfx::{Converter, TemplateParser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a converter for direct text styling
    let converter = Converter::new()?;

    // Convert text to Unicode styles
    println!("=== Direct Conversion ===");
    let result = converter.convert("HELLO WORLD", "mathbold")?;
    println!("Mathematical Bold: {}", result);

    let result = converter.convert("WARNING", "negative-squared")?;
    println!("Negative Squared: {}", result);

    let result = converter.convert("mdfx v1.0", "fullwidth")?;
    println!("Full-Width: {}", result);

    // Use style aliases
    println!("\n=== Using Aliases ===");
    let result = converter.convert("Test", "mb")?; // mathbold
    println!("Alias 'mb': {}", result);

    let result = converter.convert("Code", "mono")?; // monospace
    println!("Alias 'mono': {}", result);

    // List available styles
    println!("\n=== Available Styles ===");
    for style in converter.list_styles() {
        println!("{}: {}", style.id, style.name);
    }

    // Process markdown templates
    println!("\n=== Template Processing ===");
    let parser = TemplateParser::new()?;

    let markdown = r#"# {{mathbold}}TITLE{{/}}

This is {{italic}}emphasized{{/}} text.

{{swatch:rust:Rust}} {{swatch:python:Python}}

{{frame:star}}Featured Content{{/}}
"#;
    let processed = parser.process(markdown)?;
    println!("{}", processed);

    // Code blocks are preserved (not processed)
    println!("\n=== Code Block Preservation ===");
    let markdown_with_code = r#"Text {{mathbold}}styled{{/}}

```rust
let x = "{{mathbold}}not styled{{/}}";
```

Back to {{mathbold}}styled{{/}} text."#;

    let processed = parser.process(markdown_with_code)?;
    println!("{}", processed);

    Ok(())
}
