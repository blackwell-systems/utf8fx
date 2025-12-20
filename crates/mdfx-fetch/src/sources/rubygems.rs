//! RubyGems data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// RubyGems gem API response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GemResponse {
    name: String,
    version: String,
    downloads: i64,
    version_downloads: i64,
    authors: Option<String>,
    info: Option<String>,
    licenses: Option<Vec<String>>,
    project_uri: Option<String>,
    source_code_uri: Option<String>,
    homepage_uri: Option<String>,
    ruby_version: Option<String>,
}

/// RubyGems data source
pub struct RubyGemsSource {
    api_base: String,
}

impl Default for RubyGemsSource {
    fn default() -> Self {
        Self::new()
    }
}

impl RubyGemsSource {
    /// Create a new RubyGems source
    pub fn new() -> Self {
        RubyGemsSource {
            api_base: "https://rubygems.org/api/v1/gems".to_string(),
        }
    }

    /// Fetch gem data from RubyGems
    fn fetch_gem(&self, name: &str) -> Result<GemResponse> {
        let url = format!("{}/{}.json", self.api_base, name);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: GemResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse RubyGems response: {}", e))
                })?;
                Ok(body)
            }
            Err(ureq::Error::Status(404, _)) => Err(FetchError::NotFound(name.to_string())),
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

    /// Format download count with K/M suffixes
    fn format_downloads(count: i64) -> String {
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

impl DataSource for RubyGemsSource {
    fn id(&self) -> &'static str {
        "rubygems"
    }

    fn name(&self) -> &'static str {
        "RubyGems"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let data = self.fetch_gem(query)?;

        match metric {
            "version" => Ok(DataValue::String(data.version)),
            "downloads" => Ok(DataValue::String(Self::format_downloads(data.downloads))),
            "downloads_raw" => Ok(data.downloads.into()),
            "version_downloads" => Ok(DataValue::String(Self::format_downloads(
                data.version_downloads,
            ))),
            "license" => Ok(DataValue::String(
                data.licenses
                    .and_then(|l| l.first().cloned())
                    .unwrap_or_else(|| "Unknown".to_string()),
            )),
            "authors" => Ok(DataValue::String(
                data.authors.unwrap_or_else(|| "Unknown".to_string()),
            )),
            "info" | "description" => Ok(DataValue::String(
                data.info.unwrap_or_else(|| "No description".to_string()),
            )),
            "ruby" => Ok(DataValue::String(
                data.ruby_version.unwrap_or_else(|| "Unknown".to_string()),
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
            "version",
            "downloads",
            "downloads_raw",
            "version_downloads",
            "license",
            "authors",
            "info",
            "ruby",
        ]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "version" => "Version",
            "downloads" | "downloads_raw" => "Downloads",
            "version_downloads" => "Version DLs",
            "license" => "License",
            "authors" => "Authors",
            "info" | "description" => "Info",
            "ruby" => "Ruby",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, _value: &DataValue) -> Option<&str> {
        match metric {
            "version" => Some("CC342D"),                     // Ruby red
            "downloads" | "downloads_raw" => Some("CC342D"), // Ruby red
            "version_downloads" => Some("CC342D"),
            "license" => Some("22C55E"), // Green
            "ruby" => Some("CC342D"),    // Ruby red
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(500, "500")]
    #[case(1_500, "1.5K")]
    #[case(1_500_000, "1.5M")]
    #[case(1_500_000_000, "1.5B")]
    fn test_format_downloads(#[case] count: i64, #[case] expected: &str) {
        assert_eq!(RubyGemsSource::format_downloads(count), expected);
    }

    #[rstest]
    #[case("version", "Version")]
    #[case("downloads", "Downloads")]
    #[case("license", "License")]
    #[case("ruby", "Ruby")]
    #[case("unknown", "Unknown")]
    fn test_metric_label(#[case] metric: &str, #[case] expected: &str) {
        let source = RubyGemsSource::new();
        assert_eq!(source.metric_label(metric), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = RubyGemsSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"version"));
        assert!(metrics.contains(&"downloads"));
        assert!(metrics.contains(&"license"));
    }
}
