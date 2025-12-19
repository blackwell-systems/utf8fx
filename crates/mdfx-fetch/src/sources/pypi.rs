//! PyPI data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// PyPI API response
#[derive(Debug, Deserialize)]
struct PyPIResponse {
    info: PackageInfo,
}

/// PyPI package info
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PackageInfo {
    name: String,
    version: String,
    summary: Option<String>,
    license: Option<String>,
    author: Option<String>,
    home_page: Option<String>,
    requires_python: Option<String>,
}

/// PyPI data source
pub struct PyPISource {
    api_base: String,
}

impl Default for PyPISource {
    fn default() -> Self {
        Self::new()
    }
}

impl PyPISource {
    /// Create a new PyPI source
    pub fn new() -> Self {
        PyPISource {
            api_base: "https://pypi.org/pypi".to_string(),
        }
    }

    /// Fetch package data from PyPI
    fn fetch_package(&self, name: &str) -> Result<PackageInfo> {
        let url = format!("{}/{}/json", self.api_base, name);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let data: PyPIResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse PyPI response: {}", e))
                })?;
                Ok(data.info)
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

impl DataSource for PyPISource {
    fn id(&self) -> &'static str {
        "pypi"
    }

    fn name(&self) -> &'static str {
        "PyPI"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let data = self.fetch_package(query)?;

        match metric {
            "version" => Ok(DataValue::String(data.version)),
            "license" => Ok(DataValue::String(
                data.license.unwrap_or_else(|| "Unknown".to_string()),
            )),
            "summary" => Ok(DataValue::String(
                data.summary.unwrap_or_else(|| "No summary".to_string()),
            )),
            "author" => Ok(DataValue::String(
                data.author.unwrap_or_else(|| "Unknown".to_string()),
            )),
            "python" => Ok(DataValue::String(
                data.requires_python
                    .unwrap_or_else(|| "Any".to_string()),
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
        &["version", "license", "summary", "author", "python"]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "version" => "Version",
            "license" => "License",
            "summary" => "Summary",
            "author" => "Author",
            "python" => "Python",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, _value: &DataValue) -> Option<&str> {
        match metric {
            "version" => Some("3776AB"), // Python blue
            "license" => Some("22C55E"), // Green
            "python" => Some("FFD43B"),  // Python yellow
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_metrics() {
        let source = PyPISource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"version"));
        assert!(metrics.contains(&"license"));
    }

    #[test]
    fn test_metric_label() {
        let source = PyPISource::new();
        assert_eq!(source.metric_label("version"), "Version");
        assert_eq!(source.metric_label("python"), "Python");
    }
}
