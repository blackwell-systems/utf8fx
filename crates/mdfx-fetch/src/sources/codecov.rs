//! Codecov API data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// Codecov repository API response (partial)
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RepoResponse {
    #[serde(default)]
    totals: Option<Totals>,
    #[serde(default)]
    branch: Option<String>,
    #[serde(default)]
    active: bool,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Totals {
    #[serde(default)]
    coverage: Option<f64>,
    #[serde(default)]
    files: u64,
    #[serde(default)]
    lines: u64,
    #[serde(default)]
    hits: u64,
    #[serde(default)]
    misses: u64,
    #[serde(default)]
    partials: u64,
    #[serde(default)]
    branches: u64,
}

/// Codecov data source
pub struct CodecovSource {
    api_base: String,
}

impl Default for CodecovSource {
    fn default() -> Self {
        Self::new()
    }
}

impl CodecovSource {
    /// Create a new Codecov source
    pub fn new() -> Self {
        CodecovSource {
            api_base: "https://api.codecov.io/api/v2".to_string(),
        }
    }

    /// Fetch repository data from Codecov API
    fn fetch_repo(&self, service: &str, owner: &str, repo: &str) -> Result<RepoResponse> {
        let url = format!("{}/{}/{}/repos/{}", self.api_base, service, owner, repo);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: RepoResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse Codecov response: {}", e))
                })?;
                Ok(body)
            }
            Err(ureq::Error::Status(404, _)) => {
                Err(FetchError::NotFound(format!("{}/{}", owner, repo)))
            }
            Err(ureq::Error::Status(429, resp)) => {
                let retry_after = resp
                    .header("Retry-After")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(60);

                Err(FetchError::RateLimited { retry_after })
            }
            Err(ureq::Error::Status(code, resp)) => {
                let message = resp.into_string().unwrap_or_default();
                Err(FetchError::ApiError {
                    status: code,
                    message,
                })
            }
            Err(e) => Err(FetchError::HttpError(e.to_string())),
        }
    }

    /// Parse query into owner/repo (assumes github service by default)
    fn parse_query(query: &str) -> Result<(&str, &str, &str)> {
        let parts: Vec<&str> = query.split('/').collect();
        match parts.len() {
            // owner/repo - assume github
            2 => Ok(("github", parts[0], parts[1])),
            // service/owner/repo - explicit service
            3 => Ok((parts[0], parts[1], parts[2])),
            _ => Err(FetchError::ParseError(format!(
                "Invalid Codecov query '{}'. Expected format: owner/repo or service/owner/repo",
                query
            ))),
        }
    }
}

impl DataSource for CodecovSource {
    fn id(&self) -> &'static str {
        "codecov"
    }

    fn name(&self) -> &'static str {
        "Codecov"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let (service, owner, repo) = Self::parse_query(query)?;
        let data = self.fetch_repo(service, owner, repo)?;

        match metric {
            "coverage" => {
                let coverage = data
                    .totals
                    .as_ref()
                    .and_then(|t| t.coverage)
                    .unwrap_or(0.0);
                Ok(DataValue::Float(coverage))
            }
            "lines" => {
                let lines = data.totals.as_ref().map(|t| t.lines).unwrap_or(0);
                Ok(DataValue::Number(lines))
            }
            "hits" => {
                let hits = data.totals.as_ref().map(|t| t.hits).unwrap_or(0);
                Ok(DataValue::Number(hits))
            }
            "misses" => {
                let misses = data.totals.as_ref().map(|t| t.misses).unwrap_or(0);
                Ok(DataValue::Number(misses))
            }
            "partials" => {
                let partials = data.totals.as_ref().map(|t| t.partials).unwrap_or(0);
                Ok(DataValue::Number(partials))
            }
            "files" => {
                let files = data.totals.as_ref().map(|t| t.files).unwrap_or(0);
                Ok(DataValue::Number(files))
            }
            "branches" => {
                let branches = data.totals.as_ref().map(|t| t.branches).unwrap_or(0);
                Ok(DataValue::Number(branches))
            }
            "branch" => Ok(DataValue::String(
                data.branch.unwrap_or_else(|| "main".to_string()),
            )),
            "active" => Ok(DataValue::Bool(data.active)),
            "language" => Ok(DataValue::String(
                data.language.unwrap_or_else(|| "Unknown".to_string()),
            )),
            _ => Err(FetchError::UnknownMetric {
                metric: metric.to_string(),
                available: self
                    .available_metrics()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            }),
        }
    }

    fn available_metrics(&self) -> &'static [&'static str] {
        &[
            "coverage",
            "lines",
            "hits",
            "misses",
            "partials",
            "files",
            "branches",
            "branch",
            "active",
            "language",
        ]
    }

    fn default_ttl(&self) -> u64 {
        1800 // 30 minutes - coverage data updates with each push
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "coverage" => "Coverage",
            "lines" => "Lines",
            "hits" => "Hits",
            "misses" => "Misses",
            "partials" => "Partials",
            "files" => "Files",
            "branches" => "Branches",
            "branch" => "Branch",
            "active" => "Active",
            "language" => "Language",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, value: &DataValue) -> Option<&str> {
        match metric {
            "coverage" => {
                // Color based on coverage percentage
                match value.as_float() {
                    Some(p) if p >= 90.0 => Some("22C55E"), // Green - excellent
                    Some(p) if p >= 80.0 => Some("84CC16"), // Lime - good
                    Some(p) if p >= 70.0 => Some("EAB308"), // Yellow - acceptable
                    Some(p) if p >= 50.0 => Some("F97316"), // Orange - needs work
                    _ => Some("EF4444"),                    // Red - poor
                }
            }
            "active" => {
                if let DataValue::Bool(true) = value {
                    Some("22C55E") // Green - active
                } else {
                    Some("6B7280") // Gray - inactive
                }
            }
            "hits" => Some("22C55E"),  // Green
            "misses" => Some("EF4444"), // Red
            "partials" => Some("EAB308"), // Yellow
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Query Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("owner/repo", true, "github", "owner", "repo")]
    #[case("github/owner/repo", true, "github", "owner", "repo")]
    #[case("gitlab/owner/repo", true, "gitlab", "owner", "repo")]
    #[case("bitbucket/owner/repo", true, "bitbucket", "owner", "repo")]
    #[case("invalid", false, "", "", "")]
    #[case("a/b/c/d", false, "", "", "")]
    fn test_parse_query(
        #[case] input: &str,
        #[case] should_succeed: bool,
        #[case] expected_service: &str,
        #[case] expected_owner: &str,
        #[case] expected_repo: &str,
    ) {
        let result = CodecovSource::parse_query(input);
        if should_succeed {
            let (service, owner, repo) = result.unwrap();
            assert_eq!(service, expected_service);
            assert_eq!(owner, expected_owner);
            assert_eq!(repo, expected_repo);
        } else {
            assert!(result.is_err());
        }
    }

    // ========================================================================
    // Coverage Colors (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("coverage", DataValue::Float(95.0), Some("22C55E"))] // excellent
    #[case("coverage", DataValue::Float(85.0), Some("84CC16"))] // good
    #[case("coverage", DataValue::Float(75.0), Some("EAB308"))] // acceptable
    #[case("coverage", DataValue::Float(60.0), Some("F97316"))] // needs work
    #[case("coverage", DataValue::Float(40.0), Some("EF4444"))] // poor
    #[case("active", DataValue::Bool(true), Some("22C55E"))]
    #[case("active", DataValue::Bool(false), Some("6B7280"))]
    #[case("hits", DataValue::Number(1000), Some("22C55E"))]
    #[case("misses", DataValue::Number(100), Some("EF4444"))]
    fn test_metric_colors(
        #[case] metric: &str,
        #[case] value: DataValue,
        #[case] expected: Option<&str>,
    ) {
        let source = CodecovSource::new();
        assert_eq!(source.metric_color(metric, &value), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = CodecovSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"coverage"));
        assert!(metrics.contains(&"lines"));
        assert!(metrics.contains(&"hits"));
    }
}
