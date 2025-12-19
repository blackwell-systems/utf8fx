//! npm registry data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// npm package API response (partial)
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PackageResponse {
    name: String,
    #[serde(rename = "dist-tags")]
    dist_tags: DistTags,
    license: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DistTags {
    latest: String,
    #[serde(default)]
    next: Option<String>,
    #[serde(default)]
    beta: Option<String>,
}

/// npm data source
pub struct NpmSource {
    api_base: String,
}

impl Default for NpmSource {
    fn default() -> Self {
        Self::new()
    }
}

impl NpmSource {
    /// Create a new npm source
    pub fn new() -> Self {
        NpmSource {
            api_base: "https://registry.npmjs.org".to_string(),
        }
    }

    /// Fetch package data from npm registry
    fn fetch_package(&self, name: &str) -> Result<PackageResponse> {
        // Handle scoped packages (@scope/name)
        let encoded_name = name.replace('/', "%2F");
        let url = format!("{}/{}", self.api_base, encoded_name);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: PackageResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse npm response: {}", e))
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
}

impl DataSource for NpmSource {
    fn id(&self) -> &'static str {
        "npm"
    }

    fn name(&self) -> &'static str {
        "npm"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let data = self.fetch_package(query)?;

        match metric {
            "version" => Ok(DataValue::String(data.dist_tags.latest)),
            "next" => Ok(DataValue::String(
                data.dist_tags.next.unwrap_or_else(|| "N/A".to_string()),
            )),
            "beta" => Ok(DataValue::String(
                data.dist_tags.beta.unwrap_or_else(|| "N/A".to_string()),
            )),
            "license" => Ok(DataValue::String(
                data.license.unwrap_or_else(|| "Unknown".to_string()),
            )),
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
        &["version", "next", "beta", "license", "description"]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "version" => "Version",
            "next" => "Next",
            "beta" => "Beta",
            "license" => "License",
            "description" => "Description",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, _value: &DataValue) -> Option<&str> {
        match metric {
            "version" => Some("CB3837"),       // npm red
            "next" | "beta" => Some("EAB308"), // Yellow for pre-release
            "license" => Some("22C55E"),       // Green
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Metric Labels (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("version", "Version")]
    #[case("next", "Next")]
    #[case("beta", "Beta")]
    #[case("license", "License")]
    #[case("description", "Description")]
    #[case("unknown", "Unknown")]
    fn test_metric_label(#[case] metric: &str, #[case] expected: &str) {
        let source = NpmSource::new();
        assert_eq!(source.metric_label(metric), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = NpmSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"version"));
        assert!(metrics.contains(&"license"));
    }
}
