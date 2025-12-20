//! Packagist (PHP/Composer) data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// Packagist package API response
#[derive(Debug, Deserialize)]
struct PackageWrapper {
    package: PackageResponse,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PackageResponse {
    name: String,
    description: Option<String>,
    downloads: DownloadStats,
    favers: i64,
    versions: std::collections::HashMap<String, VersionInfo>,
}

#[derive(Debug, Deserialize)]
struct DownloadStats {
    total: i64,
    monthly: i64,
    daily: i64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct VersionInfo {
    version: String,
    license: Option<Vec<String>>,
    require: Option<std::collections::HashMap<String, String>>,
}

/// Packagist data source
pub struct PackagistSource {
    api_base: String,
}

impl Default for PackagistSource {
    fn default() -> Self {
        Self::new()
    }
}

impl PackagistSource {
    /// Create a new Packagist source
    pub fn new() -> Self {
        PackagistSource {
            api_base: "https://repo.packagist.org/packages".to_string(),
        }
    }

    /// Fetch package data from Packagist
    fn fetch_package(&self, name: &str) -> Result<PackageResponse> {
        let url = format!("{}/{}.json", self.api_base, name);

        let response = ureq::get(&url)
            .set("Accept", "application/json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .call();

        match response {
            Ok(resp) => {
                let body: PackageWrapper = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse Packagist response: {}", e))
                })?;
                Ok(body.package)
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

    /// Get the latest stable version (ignoring dev/alpha/beta/RC)
    fn get_latest_version(package: &PackageResponse) -> String {
        let mut versions: Vec<&str> = package
            .versions
            .keys()
            .filter(|v| {
                !v.contains("dev")
                    && !v.contains("alpha")
                    && !v.contains("beta")
                    && !v.contains("RC")
                    && !v.starts_with("dev-")
            })
            .map(|s| s.as_str())
            .collect();

        // Sort semver-ish (simple string sort works for most cases)
        versions.sort_by(|a, b| b.cmp(a));
        versions.first().copied().unwrap_or("unknown").to_string()
    }

    /// Get license from latest version
    fn get_license(package: &PackageResponse) -> String {
        let latest = Self::get_latest_version(package);
        package
            .versions
            .get(&latest)
            .and_then(|v| v.license.as_ref())
            .and_then(|l| l.first())
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }

    /// Get PHP version requirement from latest version
    fn get_php_version(package: &PackageResponse) -> String {
        let latest = Self::get_latest_version(package);
        package
            .versions
            .get(&latest)
            .and_then(|v| v.require.as_ref())
            .and_then(|r| r.get("php"))
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string())
    }

    /// Format download count with K/M suffixes
    fn format_downloads(count: i64) -> String {
        if count >= 1_000_000 {
            format!("{:.1}M", count as f64 / 1_000_000.0)
        } else if count >= 1_000 {
            format!("{:.1}K", count as f64 / 1_000.0)
        } else {
            count.to_string()
        }
    }
}

impl DataSource for PackagistSource {
    fn id(&self) -> &'static str {
        "packagist"
    }

    fn name(&self) -> &'static str {
        "Packagist"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let data = self.fetch_package(query)?;

        match metric {
            "version" => Ok(DataValue::String(Self::get_latest_version(&data))),
            "downloads" => Ok(DataValue::String(Self::format_downloads(
                data.downloads.total,
            ))),
            "downloads_raw" => Ok(data.downloads.total.into()),
            "monthly" => Ok(DataValue::String(Self::format_downloads(
                data.downloads.monthly,
            ))),
            "daily" => Ok(DataValue::String(Self::format_downloads(
                data.downloads.daily,
            ))),
            "stars" | "favers" => Ok(data.favers.into()),
            "license" => Ok(DataValue::String(Self::get_license(&data))),
            "php" => Ok(DataValue::String(Self::get_php_version(&data))),
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
            "version",
            "downloads",
            "downloads_raw",
            "monthly",
            "daily",
            "stars",
            "license",
            "php",
            "description",
        ]
    }

    fn default_ttl(&self) -> u64 {
        3600 // 1 hour
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "version" => "Version",
            "downloads" | "downloads_raw" => "Downloads",
            "monthly" => "Monthly",
            "daily" => "Daily",
            "stars" | "favers" => "Stars",
            "license" => "License",
            "php" => "PHP",
            "description" => "Description",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, _value: &DataValue) -> Option<&str> {
        match metric {
            "version" => Some("F28D1A"),                     // Packagist orange
            "downloads" | "downloads_raw" => Some("8892BF"), // PHP purple-ish
            "monthly" | "daily" => Some("8892BF"),
            "stars" | "favers" => Some("FFD700"), // Gold
            "license" => Some("22C55E"),          // Green
            "php" => Some("777BB4"),              // PHP purple
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
    fn test_format_downloads(#[case] count: i64, #[case] expected: &str) {
        assert_eq!(PackagistSource::format_downloads(count), expected);
    }

    #[rstest]
    #[case("version", "Version")]
    #[case("downloads", "Downloads")]
    #[case("stars", "Stars")]
    #[case("license", "License")]
    #[case("php", "PHP")]
    #[case("unknown", "Unknown")]
    fn test_metric_label(#[case] metric: &str, #[case] expected: &str) {
        let source = PackagistSource::new();
        assert_eq!(source.metric_label(metric), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = PackagistSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"version"));
        assert!(metrics.contains(&"downloads"));
        assert!(metrics.contains(&"license"));
    }
}
