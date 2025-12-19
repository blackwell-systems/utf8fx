//! GitHub API data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// GitHub repository API response (partial)
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RepoResponse {
    stargazers_count: u64,
    forks_count: u64,
    open_issues_count: u64,
    subscribers_count: u64,
    size: u64,
    language: Option<String>,
    archived: bool,
    disabled: bool,
    license: Option<LicenseInfo>,
    default_branch: String,
    topics: Vec<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct LicenseInfo {
    spdx_id: Option<String>,
    name: String,
}

/// GitHub data source
pub struct GitHubSource {
    api_base: String,
}

impl Default for GitHubSource {
    fn default() -> Self {
        Self::new()
    }
}

impl GitHubSource {
    /// Create a new GitHub source
    pub fn new() -> Self {
        GitHubSource {
            api_base: "https://api.github.com".to_string(),
        }
    }

    /// Fetch repository data from GitHub API
    fn fetch_repo(&self, owner: &str, repo: &str) -> Result<RepoResponse> {
        let url = format!("{}/repos/{}/{}", self.api_base, owner, repo);

        let response = ureq::get(&url)
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .set("X-GitHub-Api-Version", "2022-11-28")
            .call();

        match response {
            Ok(resp) => {
                let body: RepoResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse GitHub response: {}", e))
                })?;
                Ok(body)
            }
            Err(ureq::Error::Status(404, _)) => {
                Err(FetchError::NotFound(format!("{}/{}", owner, repo)))
            }
            Err(ureq::Error::Status(403, resp)) => {
                // Check for rate limiting
                let retry_after = resp
                    .header("X-RateLimit-Reset")
                    .and_then(|s| s.parse::<u64>().ok())
                    .map(|reset| {
                        reset.saturating_sub(
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        )
                    })
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

    /// Parse query into owner/repo
    fn parse_query(query: &str) -> Result<(&str, &str)> {
        let parts: Vec<&str> = query.split('/').collect();
        if parts.len() != 2 {
            return Err(FetchError::ParseError(format!(
                "Invalid GitHub query '{}'. Expected format: owner/repo",
                query
            )));
        }
        Ok((parts[0], parts[1]))
    }
}

impl DataSource for GitHubSource {
    fn id(&self) -> &'static str {
        "github"
    }

    fn name(&self) -> &'static str {
        "GitHub"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let (owner, repo) = Self::parse_query(query)?;
        let data = self.fetch_repo(owner, repo)?;

        match metric {
            "stars" => Ok(DataValue::Number(data.stargazers_count)),
            "forks" => Ok(DataValue::Number(data.forks_count)),
            "issues" => Ok(DataValue::Number(data.open_issues_count)),
            "watchers" => Ok(DataValue::Number(data.subscribers_count)),
            "size" => Ok(DataValue::Number(data.size)),
            "language" => Ok(DataValue::String(
                data.language.unwrap_or_else(|| "Unknown".to_string()),
            )),
            "license" => Ok(DataValue::String(
                data.license
                    .and_then(|l| l.spdx_id)
                    .unwrap_or_else(|| "None".to_string()),
            )),
            "archived" => Ok(DataValue::Bool(data.archived)),
            "branch" => Ok(DataValue::String(data.default_branch)),
            "topics" => {
                if data.topics.is_empty() {
                    Ok(DataValue::String("None".to_string()))
                } else {
                    Ok(DataValue::String(data.topics.join(", ")))
                }
            }
            "description" => Ok(DataValue::String(
                data.description
                    .unwrap_or_else(|| "No description".to_string()),
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
            "stars",
            "forks",
            "issues",
            "watchers",
            "size",
            "language",
            "license",
            "archived",
            "branch",
            "topics",
            "description",
        ]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour - GitHub data changes frequently
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "stars" => "Stars",
            "forks" => "Forks",
            "issues" => "Issues",
            "watchers" => "Watchers",
            "size" => "Size",
            "language" => "Language",
            "license" => "License",
            "archived" => "Archived",
            "branch" => "Branch",
            "topics" => "Topics",
            "description" => "Description",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, value: &DataValue) -> Option<&str> {
        match metric {
            "stars" => {
                // Color based on popularity
                match value.as_number() {
                    Some(n) if n >= 10000 => Some("FFD700"), // Gold
                    Some(n) if n >= 1000 => Some("C0C0C0"),  // Silver
                    Some(n) if n >= 100 => Some("CD7F32"),   // Bronze
                    _ => Some("22C55E"),                     // Green
                }
            }
            "forks" | "issues" | "watchers" => Some("3B82F6"), // Blue
            "license" => {
                // Color based on license type
                if let DataValue::String(license) = value {
                    let upper = license.to_uppercase();
                    if upper == "MIT" || upper.contains("APACHE") || upper.contains("BSD") {
                        Some("22C55E") // Green - permissive
                    } else if upper.contains("GPL") {
                        Some("EAB308") // Yellow - copyleft
                    } else if upper == "NONE" {
                        Some("6B7280") // Gray - no license
                    } else {
                        Some("3B82F6") // Blue - other
                    }
                } else {
                    None
                }
            }
            "archived" => {
                if let DataValue::Bool(true) = value {
                    Some("EF4444") // Red - archived
                } else {
                    Some("22C55E") // Green - active
                }
            }
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
    #[case("rust-lang/rust", true, "rust-lang", "rust")]
    #[case("owner/repo", true, "owner", "repo")]
    #[case("invalid", false, "", "")]
    #[case("too/many/parts", false, "", "")]
    fn test_parse_query(
        #[case] input: &str,
        #[case] should_succeed: bool,
        #[case] expected_owner: &str,
        #[case] expected_repo: &str,
    ) {
        let result = GitHubSource::parse_query(input);
        if should_succeed {
            let (owner, repo) = result.unwrap();
            assert_eq!(owner, expected_owner);
            assert_eq!(repo, expected_repo);
        } else {
            assert!(result.is_err());
        }
    }

    // ========================================================================
    // Metric Colors (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("stars", DataValue::Number(50000), Some("FFD700"))]  // gold
    #[case("stars", DataValue::Number(5000), Some("C0C0C0"))]   // silver
    #[case("stars", DataValue::Number(500), Some("CD7F32"))]    // bronze
    #[case("stars", DataValue::Number(50), Some("22C55E"))]     // green
    #[case("license", DataValue::String("MIT".to_string()), Some("22C55E"))]
    #[case("license", DataValue::String("GPL-3.0".to_string()), Some("EAB308"))]
    #[case("license", DataValue::String("NONE".to_string()), Some("6B7280"))]
    #[case("archived", DataValue::Bool(true), Some("EF4444"))]
    #[case("archived", DataValue::Bool(false), Some("22C55E"))]
    fn test_metric_colors(
        #[case] metric: &str,
        #[case] value: DataValue,
        #[case] expected: Option<&str>,
    ) {
        let source = GitHubSource::new();
        assert_eq!(source.metric_color(metric, &value), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = GitHubSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"stars"));
        assert!(metrics.contains(&"license"));
        assert!(metrics.contains(&"forks"));
    }
}
