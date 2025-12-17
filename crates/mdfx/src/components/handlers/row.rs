//! Row layout component handler

use crate::components::{ComponentOutput, PostProcess};
use crate::error::Result;
use std::collections::HashMap;

/// Handle row component expansion
pub fn handle(params: &HashMap<String, String>, content: Option<&str>) -> Result<ComponentOutput> {
    // Extract align parameter (default: center)
    let align = params
        .get("align")
        .cloned()
        .unwrap_or_else(|| "center".to_string());

    // Validate align value
    let align = match align.as_str() {
        "left" | "center" | "right" => align,
        _ => "center".to_string(),
    };

    // Content to be recursively parsed, then post-processed
    let template = content.unwrap_or("").to_string();

    Ok(ComponentOutput::TemplateDelayed {
        template,
        post_process: PostProcess::Row { align },
    })
}

/// Apply row formatting (wrap in HTML with alignment)
///
/// This is called AFTER recursive parsing to transform rendered content:
/// 1. Collapses whitespace/newlines to single spaces
/// 2. Converts markdown images `![alt](url)` to HTML `<img alt="alt" src="url">`
/// 3. Wraps with `<p align="...">...</p>`
///
/// This is necessary because GitHub Flavored Markdown doesn't parse
/// markdown syntax inside HTML blocks.
pub fn apply_row(content: &str, align: &str) -> String {
    // Step 1: Collapse whitespace/newlines to single spaces
    let collapsed: String = content.split_whitespace().collect::<Vec<_>>().join(" ");

    // Step 2: Convert markdown images to HTML img tags
    // Pattern: ![alt](url) or ![](url)
    let mut result = String::new();
    let mut remaining = collapsed.as_str();

    while let Some(start) = remaining.find("![") {
        // Add text before the image
        result.push_str(&remaining[..start]);

        let after_bang = &remaining[start + 2..];

        // Find closing ] for alt text
        if let Some(alt_end) = after_bang.find(']') {
            let alt = &after_bang[..alt_end];
            let after_alt = &after_bang[alt_end + 1..];

            // Expect ( after ]
            if let Some(after_paren) = after_alt.strip_prefix('(') {
                // Find closing )
                if let Some(url_end) = after_paren.find(')') {
                    let url = &after_paren[..url_end];
                    // Convert to HTML img tag
                    result.push_str(&format!(r#"<img alt="{}" src="{}">"#, alt, url));
                    remaining = &after_paren[url_end + 1..];
                    continue;
                }
            }
        }

        // Malformed image syntax, keep as-is
        result.push_str("![");
        remaining = after_bang;
    }

    // Add any remaining text
    result.push_str(remaining);

    // Step 3: Wrap with alignment
    format!(
        r#"<p align="{}">
{}
</p>"#,
        align,
        result.trim()
    )
}
