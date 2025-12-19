//! Data value types for fetched metrics

use serde::{Deserialize, Serialize};
use std::fmt;

/// Value types that can be fetched from data sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataValue {
    /// Numeric value (stars, downloads, etc.)
    Number(u64),
    /// Float value (percentages, ratings)
    Float(f64),
    /// String value (version, license, language)
    String(String),
    /// Boolean value (has_issues, archived)
    Bool(bool),
}

impl DataValue {
    /// Format the value for display in a badge
    pub fn format(&self) -> String {
        match self {
            DataValue::Number(n) => format_number(*n),
            DataValue::Float(f) => format!("{:.1}", f),
            DataValue::String(s) => s.clone(),
            DataValue::Bool(b) => if *b { "yes" } else { "no" }.to_string(),
        }
    }

    /// Get as number, returning None if not a number
    pub fn as_number(&self) -> Option<u64> {
        match self {
            DataValue::Number(n) => Some(*n),
            DataValue::Float(f) => Some(*f as u64),
            _ => None,
        }
    }

    /// Get as string
    pub fn as_string(&self) -> String {
        self.format()
    }

    /// Get as bool, returning None if not a bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DataValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl fmt::Display for DataValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl From<u64> for DataValue {
    fn from(n: u64) -> Self {
        DataValue::Number(n)
    }
}

impl From<i64> for DataValue {
    fn from(n: i64) -> Self {
        DataValue::Number(n as u64)
    }
}

impl From<f64> for DataValue {
    fn from(f: f64) -> Self {
        DataValue::Float(f)
    }
}

impl From<String> for DataValue {
    fn from(s: String) -> Self {
        DataValue::String(s)
    }
}

impl From<&str> for DataValue {
    fn from(s: &str) -> Self {
        DataValue::String(s.to_string())
    }
}

impl From<bool> for DataValue {
    fn from(b: bool) -> Self {
        DataValue::Bool(b)
    }
}

/// Format a large number with K/M/B suffixes
fn format_number(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Number Formatting (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(500, "500")]
    #[case(1_500, "1.5k")]
    #[case(15_000, "15.0k")]
    #[case(1_500_000, "1.5M")]
    #[case(1_500_000_000, "1.5B")]
    fn test_format_number(#[case] input: u64, #[case] expected: &str) {
        assert_eq!(format_number(input), expected);
    }

    // ========================================================================
    // DataValue Display (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(DataValue::Number(42), "42")]
    #[case(DataValue::Number(1500), "1.5k")]
    #[case(DataValue::Float(2.75), "2.8")]
    #[case(DataValue::String("MIT".to_string()), "MIT")]
    #[case(DataValue::Bool(true), "yes")]
    #[case(DataValue::Bool(false), "no")]
    fn test_data_value_display(#[case] value: DataValue, #[case] expected: &str) {
        assert_eq!(value.format(), expected);
    }

    #[test]
    fn test_from_conversions() {
        let n: DataValue = 42u64.into();
        assert_eq!(n, DataValue::Number(42));

        let s: DataValue = "hello".into();
        assert_eq!(s, DataValue::String("hello".to_string()));

        let b: DataValue = true.into();
        assert_eq!(b, DataValue::Bool(true));
    }
}
