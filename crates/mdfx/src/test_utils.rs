//! Test utilities and macros for mdfx
//!
//! Provides helper macros to reduce boilerplate in tests.
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::test_utils::*;
//!
//! // Simple input -> expected test
//! test_process!("{{mathbold}}HELLO{{/mathbold}}" => "𝐇𝐄𝐋𝐋𝐎");
//!
//! // Test that processing fails
//! test_process_err!("{{unknown}}text{{/unknown}}");
//!
//! // Test with custom name
//! test_process!(test_bold_text, "{{mathbold}}X{{/mathbold}}" => "𝐗");
//! ```

/// Test that processing input produces expected output.
///
/// # Variants
///
/// - `test_process!(input => expected)` - anonymous test assertion
/// - `test_process!(name, input => expected)` - named test function
///
/// # Example
///
/// ```rust,ignore
/// test_process!("{{mathbold}}A{{/mathbold}}" => "𝐀");
/// ```
#[macro_export]
macro_rules! test_process {
    // Anonymous assertion (use inside a test function)
    ($input:expr => $expected:expr) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        assert_eq!(result, $expected, "Input: {}", $input);
    }};

    // Named test function
    ($name:ident, $input:expr => $expected:expr) => {
        #[test]
        fn $name() {
            let parser = $crate::TemplateParser::new().expect("Failed to create parser");
            let result = parser.process($input).expect("Processing failed");
            assert_eq!(result, $expected, "Input: {}", $input);
        }
    };
}

/// Test that processing input fails with an error.
///
/// # Example
///
/// ```rust,ignore
/// test_process_err!("{{unknown}}text{{/unknown}}");
/// ```
#[macro_export]
macro_rules! test_process_err {
    // Anonymous assertion
    ($input:expr) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input);
        assert!(result.is_err(), "Expected error for input: {}", $input);
    }};

    // Named test function
    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            let parser = $crate::TemplateParser::new().expect("Failed to create parser");
            let result = parser.process($input);
            assert!(result.is_err(), "Expected error for input: {}", $input);
        }
    };
}

/// Test that input is preserved unchanged (useful for code blocks, etc.)
///
/// # Example
///
/// ```rust,ignore
/// test_process_unchanged!("```\n{{mathbold}}CODE{{/mathbold}}\n```");
/// ```
#[macro_export]
macro_rules! test_process_unchanged {
    ($input:expr) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        let result = parser.process($input).expect("Processing failed");
        assert_eq!(result, $input, "Expected unchanged output for: {}", $input);
    }};

    ($name:ident, $input:expr) => {
        #[test]
        fn $name() {
            let parser = $crate::TemplateParser::new().expect("Failed to create parser");
            let result = parser.process($input).expect("Processing failed");
            assert_eq!(result, $input, "Expected unchanged output for: {}", $input);
        }
    };
}

/// Test multiple input/output pairs in a single test.
///
/// # Example
///
/// ```rust,ignore
/// test_process_cases!(
///     "{{mathbold}}A{{/mathbold}}" => "𝐀",
///     "{{mathbold}}B{{/mathbold}}" => "𝐁",
///     "{{mathbold}}C{{/mathbold}}" => "𝐂",
/// );
/// ```
#[macro_export]
macro_rules! test_process_cases {
    ($($input:expr => $expected:expr),+ $(,)?) => {{
        let parser = $crate::TemplateParser::new().expect("Failed to create parser");
        $(
            let result = parser.process($input).expect(&format!("Processing failed for: {}", $input));
            assert_eq!(result, $expected, "Input: {}", $input);
        )+
    }};
}

/// Test converter output for a style.
///
/// # Example
///
/// ```rust,ignore
/// test_convert!("mathbold", "ABC" => "𝐀𝐁𝐂");
/// ```
#[macro_export]
macro_rules! test_convert {
    ($style:expr, $input:expr => $expected:expr) => {{
        let converter = $crate::Converter::new().expect("Failed to create converter");
        let result = converter.convert($input, $style).expect("Conversion failed");
        assert_eq!(result, $expected, "Style: {}, Input: {}", $style, $input);
    }};
}

/// Test that a style conversion fails.
#[macro_export]
macro_rules! test_convert_err {
    ($style:expr, $input:expr) => {{
        let converter = $crate::Converter::new().expect("Failed to create converter");
        let result = converter.convert($input, $style);
        assert!(result.is_err(), "Expected error for style: {}", $style);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_basic() {
        test_process!("{{mathbold}}A{{/mathbold}}" => "𝐀");
    }

    #[test]
    fn test_macro_unchanged() {
        test_process_unchanged!("```\n{{mathbold}}CODE{{/mathbold}}\n```");
    }

    #[test]
    fn test_macro_error() {
        // Unclosed tag causes an error
        test_process_err!("{{mathbold}}text without closing tag");
    }

    #[test]
    fn test_macro_cases() {
        test_process_cases!(
            "{{mathbold}}A{{/mathbold}}" => "𝐀",
            "{{mathbold}}B{{/mathbold}}" => "𝐁",
            "plain text" => "plain text",
        );
    }

    #[test]
    fn test_convert_macro() {
        test_convert!("mathbold", "ABC" => "𝐀𝐁𝐂");
    }
}
