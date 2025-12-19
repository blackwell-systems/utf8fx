//! crates.io data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// crates.io API response wrapper
#[derive(Debug, Deserialize)]
struct CrateResponseWrapper {
    #[serde(rename = "crate")]
    crate_data: CrateResponse,
}

/// crates.io crate data
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CrateResponse {
    name: String,
    max_version: String,
    max_stable_version: Option<String>,
    downloads: u64,
    recent_downloads: Option<u64>,
    description: Option<String>,
    documentation: Option<String>,
    repository: Option<String>,
}

/// crates.io data source
pub struct CratesSource {
    api_base: String,
}

impl Default for CratesSource {
    fn default() -> Self {
        Self::new()
    }
}

impl CratesSource {
    /// Create a new crates.io source
    pub fn new() -> Self {
        CratesSource {
            api_base: "https://crates.io/api/v1".to_string(),
        }
    }

    /// Fetch crate data from crates.io
    fn fetch_crate(&self, name: &str) -> Result<CrateResponse> {
        let url = format!("{}/crates/{}", self.api_base, name);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0 (https://github.com/blackwell-systems/mdfx)")
            .call();

        match response {
            Ok(resp) => {
                let wrapper: CrateResponseWrapper = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse crates.io response: {}", e))
                })?;
                Ok(wrapper.crate_data)
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

impl DataSource for CratesSource {
    fn id(&self) -> &'static str {
        "crates"
    }

    fn name(&self) -> &'static str {
        "crates.io"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let data = self.fetch_crate(query)?;

        match metric {
            "version" => Ok(DataValue::String(data.max_version)),
            "stable" => Ok(DataValue::String(
                data.max_stable_version
                    .unwrap_or_else(|| data.max_version.clone()),
            )),
            "downloads" => Ok(DataValue::Number(data.downloads)),
            "recent" => Ok(DataValue::Number(data.recent_downloads.unwrap_or(0))),
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
        &["version", "stable", "downloads", "recent", "description"]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "version" => "Version",
            "stable" => "Stable",
            "downloads" => "Downloads",
            "recent" => "Recent",
            "description" => "Description",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, value: &DataValue) -> Option<&str> {
        match metric {
            "version" | "stable" => Some("DEA584"), // Rust orange
            "downloads" | "recent" => {
                // Color based on download count
                match value.as_number() {
                    Some(n) if n >= 1_000_000 => Some("FFD700"), // Gold
                    Some(n) if n >= 100_000 => Some("C0C0C0"),   // Silver
                    Some(n) if n >= 10_000 => Some("CD7F32"),    // Bronze
                    _ => Some("22C55E"),                         // Green
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_metrics() {
        let source = CratesSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"version"));
        assert!(metrics.contains(&"downloads"));
    }

    #[test]
    fn test_metric_colors() {
        let source = CratesSource::new();

        // High downloads = gold
        let color = source.metric_color("downloads", &DataValue::Number(2_000_000));
        assert_eq!(color, Some("FFD700"));
    }
}
