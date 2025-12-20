//! NuGet (.NET) data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// NuGet registration index response
#[derive(Debug, Deserialize)]
struct RegistrationIndex {
    items: Vec<RegistrationPage>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RegistrationPage {
    items: Option<Vec<RegistrationLeaf>>,
    #[serde(rename = "@id")]
    id: Option<String>,
    lower: Option<String>,
    upper: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RegistrationLeaf {
    #[serde(rename = "catalogEntry")]
    catalog_entry: CatalogEntry,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CatalogEntry {
    version: String,
    description: Option<String>,
    authors: Option<String>,
    #[serde(rename = "licenseExpression")]
    license_expression: Option<String>,
    #[serde(rename = "projectUrl")]
    project_url: Option<String>,
    tags: Option<Vec<String>>,
}

/// NuGet package stats from search API
#[derive(Debug, Deserialize)]
struct SearchResponse {
    data: Vec<SearchPackage>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct SearchPackage {
    id: String,
    version: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    #[serde(rename = "totalDownloads")]
    total_downloads: Option<i64>,
    #[serde(rename = "licenseUrl")]
    license_url: Option<String>,
}

/// NuGet data source
pub struct NuGetSource {
    registration_base: String,
    search_base: String,
}

impl Default for NuGetSource {
    fn default() -> Self {
        Self::new()
    }
}

impl NuGetSource {
    /// Create a new NuGet source
    pub fn new() -> Self {
        NuGetSource {
            registration_base: "https://api.nuget.org/v3/registration5-semver1".to_string(),
            search_base: "https://azuresearch-usnc.nuget.org/query".to_string(),
        }
    }

    /// Fetch package data from NuGet registration API
    fn fetch_registration(&self, name: &str) -> Result<RegistrationIndex> {
        let url = format!(
            "{}/{}/index.json",
            self.registration_base,
            name.to_lowercase()
        );

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: RegistrationIndex = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse NuGet response: {}", e))
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

    /// Fetch package data from NuGet search API (for download counts)
    fn fetch_search(&self, name: &str) -> Result<SearchPackage> {
        let url = format!("{}?q=packageid:{}&take=1", self.search_base, name);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: SearchResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse NuGet search: {}", e))
                })?;
                body.data
                    .into_iter()
                    .next()
                    .ok_or_else(|| FetchError::NotFound(name.to_string()))
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

    /// Get latest version from registration data
    fn get_latest_version(reg: &RegistrationIndex) -> String {
        reg.items
            .last()
            .and_then(|page| page.items.as_ref())
            .and_then(|items| items.last())
            .map(|leaf| leaf.catalog_entry.version.clone())
            .or_else(|| reg.items.last().and_then(|page| page.upper.clone()))
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Get latest catalog entry
    fn get_latest_entry(reg: &RegistrationIndex) -> Option<&CatalogEntry> {
        reg.items
            .last()
            .and_then(|page| page.items.as_ref())
            .and_then(|items| items.last())
            .map(|leaf| &leaf.catalog_entry)
    }

    /// Format download count with K/M/B suffixes
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

impl DataSource for NuGetSource {
    fn id(&self) -> &'static str {
        "nuget"
    }

    fn name(&self) -> &'static str {
        "NuGet"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        match metric {
            "version" => {
                let reg = self.fetch_registration(query)?;
                Ok(DataValue::String(Self::get_latest_version(&reg)))
            }
            "downloads" => {
                let search = self.fetch_search(query)?;
                let count = search.total_downloads.unwrap_or(0);
                Ok(DataValue::String(Self::format_downloads(count)))
            }
            "downloads_raw" => {
                let search = self.fetch_search(query)?;
                Ok(search.total_downloads.unwrap_or(0).into())
            }
            "description" => {
                let reg = self.fetch_registration(query)?;
                let desc = Self::get_latest_entry(&reg)
                    .and_then(|e| e.description.clone())
                    .unwrap_or_else(|| "No description".to_string());
                Ok(DataValue::String(desc))
            }
            "authors" => {
                let reg = self.fetch_registration(query)?;
                let authors = Self::get_latest_entry(&reg)
                    .and_then(|e| e.authors.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                Ok(DataValue::String(authors))
            }
            "license" => {
                let reg = self.fetch_registration(query)?;
                let license = Self::get_latest_entry(&reg)
                    .and_then(|e| e.license_expression.clone())
                    .unwrap_or_else(|| "Unknown".to_string());
                Ok(DataValue::String(license))
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
            "version",
            "downloads",
            "downloads_raw",
            "description",
            "authors",
            "license",
        ]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "version" => "Version",
            "downloads" | "downloads_raw" => "Downloads",
            "description" => "Description",
            "authors" => "Authors",
            "license" => "License",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, _value: &DataValue) -> Option<&str> {
        match metric {
            "version" => Some("004880"),                     // NuGet blue
            "downloads" | "downloads_raw" => Some("004880"), // NuGet blue
            "license" => Some("22C55E"),                     // Green
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
        assert_eq!(NuGetSource::format_downloads(count), expected);
    }

    #[rstest]
    #[case("version", "Version")]
    #[case("downloads", "Downloads")]
    #[case("license", "License")]
    #[case("unknown", "Unknown")]
    fn test_metric_label(#[case] metric: &str, #[case] expected: &str) {
        let source = NuGetSource::new();
        assert_eq!(source.metric_label(metric), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = NuGetSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"version"));
        assert!(metrics.contains(&"downloads"));
        assert!(metrics.contains(&"license"));
    }
}
