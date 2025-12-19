//! Error types for mdfx-fetch

use thiserror::Error;

/// Result type for fetch operations
pub type Result<T> = std::result::Result<T, FetchError>;

/// Errors that can occur during data fetching
#[derive(Error, Debug)]
pub enum FetchError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(String),

    /// API returned an error response
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    /// Failed to parse API response
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Unknown data source
    #[error("Unknown data source: {0}")]
    UnknownSource(String),

    /// Unknown metric for source
    #[error("Unknown metric '{metric}'. Available: {available:?}")]
    UnknownMetric {
        metric: String,
        available: Vec<String>,
    },

    /// Resource not found (404)
    #[error("Not found: {0}")]
    NotFound(String),

    /// Cache error
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Offline mode with no cached data
    #[error("Offline mode: no cached data for {0}")]
    OfflineNoCache(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON parsing error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl FetchError {
    /// Check if this error is recoverable (can fall back to cache)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            FetchError::HttpError(_)
                | FetchError::RateLimited { .. }
                | FetchError::ApiError {
                    status: 500..=599,
                    ..
                }
        )
    }

    /// Check if this is a rate limit error
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, FetchError::RateLimited { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Error Recoverability (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(FetchError::HttpError("timeout".to_string()), true)]
    #[case(FetchError::RateLimited { retry_after: 60 }, true)]
    #[case(FetchError::ApiError { status: 500, message: "server error".to_string() }, true)]
    #[case(FetchError::ApiError { status: 502, message: "bad gateway".to_string() }, true)]
    #[case(FetchError::NotFound("repo".to_string()), false)]
    #[case(FetchError::UnknownSource("unknown".to_string()), false)]
    #[case(FetchError::ParseError("invalid json".to_string()), false)]
    fn test_is_recoverable(#[case] error: FetchError, #[case] expected: bool) {
        assert_eq!(error.is_recoverable(), expected);
    }
}
