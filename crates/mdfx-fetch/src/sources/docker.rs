//! Docker Hub data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// Docker Hub repository API response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RepoResponse {
    name: String,
    namespace: String,
    description: Option<String>,
    pull_count: u64,
    star_count: u64,
    is_official: Option<bool>,
    is_automated: Option<bool>,
    last_updated: Option<String>,
}

/// Docker Hub tags API response
#[derive(Debug, Deserialize)]
struct TagsResponse {
    results: Vec<TagInfo>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct TagInfo {
    name: String,
    full_size: Option<u64>,
    last_updated: Option<String>,
}

/// Docker Hub data source
pub struct DockerSource {
    api_base: String,
}

impl Default for DockerSource {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerSource {
    /// Create a new Docker Hub source
    pub fn new() -> Self {
        DockerSource {
            api_base: "https://hub.docker.com/v2".to_string(),
        }
    }

    /// Parse query into (namespace, repository)
    /// Formats: "library/nginx", "nginx" (implies library/), "user/repo"
    fn parse_query(query: &str) -> (&str, &str) {
        if let Some((ns, repo)) = query.split_once('/') {
            (ns, repo)
        } else {
            // Official images are in "library" namespace
            ("library", query)
        }
    }

    /// Fetch repository data from Docker Hub
    fn fetch_repo(&self, namespace: &str, repo: &str) -> Result<RepoResponse> {
        let url = format!("{}/repositories/{}/{}", self.api_base, namespace, repo);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: RepoResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse Docker Hub response: {}", e))
                })?;
                Ok(body)
            }
            Err(ureq::Error::Status(404, _)) => {
                Err(FetchError::NotFound(format!("{}/{}", namespace, repo)))
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

    /// Fetch latest tag for a repository
    fn fetch_latest_tag(&self, namespace: &str, repo: &str) -> Result<String> {
        let url = format!(
            "{}/repositories/{}/{}/tags?page_size=1&ordering=last_updated",
            self.api_base, namespace, repo
        );

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: TagsResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse Docker Hub tags: {}", e))
                })?;
                Ok(body
                    .results
                    .first()
                    .map(|t| t.name.clone())
                    .unwrap_or_else(|| "latest".to_string()))
            }
            Err(ureq::Error::Status(404, _)) => Ok("latest".to_string()),
            Err(e) => Err(FetchError::HttpError(e.to_string())),
        }
    }

    /// Format pull count with K/M/B suffixes
    fn format_pulls(count: u64) -> String {
        if count >= 1_000_000_000 {
            format!("{:.1}B", count as f64 / 1_000_000_000.0)
        } else if count >= 1_000_000 {
            format!("{:.1}M", count as f64 / 1_000_000.0)
        } else if count >= 1_000 {
            format!("{:.1}K", count as f64 / 1_000.0)
        } else {
            count.to_string()
        }
    }
}

impl DataSource for DockerSource {
    fn id(&self) -> &'static str {
        "docker"
    }

    fn name(&self) -> &'static str {
        "Docker Hub"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let (namespace, repo) = Self::parse_query(query);

        match metric {
            "pulls" => {
                let data = self.fetch_repo(namespace, repo)?;
                Ok(DataValue::String(Self::format_pulls(data.pull_count)))
            }
            "pulls_raw" => {
                let data = self.fetch_repo(namespace, repo)?;
                Ok(DataValue::Number(data.pull_count))
            }
            "stars" => {
                let data = self.fetch_repo(namespace, repo)?;
                Ok(DataValue::Number(data.star_count))
            }
            "tag" => {
                let tag = self.fetch_latest_tag(namespace, repo)?;
                Ok(DataValue::String(tag))
            }
            "description" => {
                let data = self.fetch_repo(namespace, repo)?;
                Ok(DataValue::String(
                    data.description
                        .unwrap_or_else(|| "No description".to_string()),
                ))
            }
            "official" => {
                let data = self.fetch_repo(namespace, repo)?;
                Ok(DataValue::String(
                    if data.is_official.unwrap_or(false) {
                        "Official"
                    } else {
                        "Community"
                    }
                    .to_string(),
                ))
            }
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
            "pulls",
            "pulls_raw",
            "stars",
            "tag",
            "description",
            "official",
        ]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "pulls" | "pulls_raw" => "Pulls",
            "stars" => "Stars",
            "tag" => "Tag",
            "description" => "Description",
            "official" => "Type",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, _value: &DataValue) -> Option<&str> {
        match metric {
            "pulls" | "pulls_raw" => Some("2496ED"), // Docker blue
            "stars" => Some("FFD700"),               // Gold
            "tag" => Some("2496ED"),                 // Docker blue
            "official" => Some("22C55E"),            // Green
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("nginx", ("library", "nginx"))]
    #[case("library/nginx", ("library", "nginx"))]
    #[case("myuser/myrepo", ("myuser", "myrepo"))]
    fn test_parse_query(#[case] query: &str, #[case] expected: (&str, &str)) {
        assert_eq!(DockerSource::parse_query(query), expected);
    }

    #[rstest]
    #[case(500, "500")]
    #[case(1_500, "1.5K")]
    #[case(1_500_000, "1.5M")]
    #[case(1_500_000_000, "1.5B")]
    fn test_format_pulls(#[case] count: u64, #[case] expected: &str) {
        assert_eq!(DockerSource::format_pulls(count), expected);
    }

    #[rstest]
    #[case("pulls", "Pulls")]
    #[case("stars", "Stars")]
    #[case("tag", "Tag")]
    #[case("unknown", "Unknown")]
    fn test_metric_label(#[case] metric: &str, #[case] expected: &str) {
        let source = DockerSource::new();
        assert_eq!(source.metric_label(metric), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = DockerSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"pulls"));
        assert!(metrics.contains(&"stars"));
        assert!(metrics.contains(&"tag"));
    }
}
