//! GitHub Actions API data source

use crate::error::{FetchError, Result};
use crate::sources::DataSource;
use crate::value::DataValue;
use serde::Deserialize;

/// GitHub Actions workflow runs response
#[derive(Debug, Deserialize)]
struct WorkflowRunsResponse {
    workflow_runs: Vec<WorkflowRun>,
}

/// Individual workflow run
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WorkflowRun {
    id: u64,
    name: Option<String>,
    status: String,
    conclusion: Option<String>,
    run_number: u64,
    event: String,
    #[serde(default)]
    head_branch: Option<String>,
}

/// GitHub Actions data source
pub struct ActionsSource {
    api_base: String,
}

impl Default for ActionsSource {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionsSource {
    /// Create a new GitHub Actions source
    pub fn new() -> Self {
        ActionsSource {
            api_base: "https://api.github.com".to_string(),
        }
    }

    /// Fetch latest workflow run from GitHub Actions API
    fn fetch_latest_run(
        &self,
        owner: &str,
        repo: &str,
        workflow: Option<&str>,
        branch: Option<&str>,
    ) -> Result<WorkflowRun> {
        let mut url = format!("{}/repos/{}/{}/actions/runs", self.api_base, owner, repo);

        // Build query params
        let mut params = vec!["per_page=1"];
        let branch_param;
        if let Some(b) = branch {
            branch_param = format!("branch={}", b);
            params.push(&branch_param);
        }

        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = ureq::get(&url)
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", "mdfx-fetch/1.0")
            .set("X-GitHub-Api-Version", "2022-11-28")
            .call();

        match response {
            Ok(resp) => {
                let body: WorkflowRunsResponse = resp.into_json().map_err(|e| {
                    FetchError::ParseError(format!("Failed to parse Actions response: {}", e))
                })?;

                // Filter by workflow name if specified
                let runs: Vec<_> = if let Some(wf) = workflow {
                    body.workflow_runs
                        .into_iter()
                        .filter(|r| {
                            r.name
                                .as_ref()
                                .map(|n| n.to_lowercase().contains(&wf.to_lowercase()))
                                .unwrap_or(false)
                        })
                        .collect()
                } else {
                    body.workflow_runs
                };

                runs.into_iter().next().ok_or_else(|| {
                    FetchError::NotFound(format!("No workflow runs found for {}/{}", owner, repo))
                })
            }
            Err(ureq::Error::Status(404, _)) => {
                Err(FetchError::NotFound(format!("{}/{}", owner, repo)))
            }
            Err(ureq::Error::Status(403, resp)) => {
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

    /// Parse query into owner/repo and optional workflow/branch
    /// Format: owner/repo or owner/repo/workflow or owner/repo/workflow@branch
    fn parse_query(query: &str) -> Result<(&str, &str, Option<&str>, Option<&str>)> {
        // Check for branch specification
        let (query, branch) = if let Some(at_pos) = query.find('@') {
            (&query[..at_pos], Some(&query[at_pos + 1..]))
        } else {
            (query, None)
        };

        let parts: Vec<&str> = query.split('/').collect();
        match parts.len() {
            2 => Ok((parts[0], parts[1], None, branch)),
            3 => Ok((parts[0], parts[1], Some(parts[2]), branch)),
            _ => Err(FetchError::ParseError(format!(
                "Invalid Actions query '{}'. Expected: owner/repo, owner/repo/workflow, or owner/repo/workflow@branch",
                query
            ))),
        }
    }
}

impl DataSource for ActionsSource {
    fn id(&self) -> &'static str {
        "actions"
    }

    fn name(&self) -> &'static str {
        "GitHub Actions"
    }

    fn fetch(&self, query: &str, metric: &str) -> Result<DataValue> {
        let (owner, repo, workflow, branch) = Self::parse_query(query)?;
        let run = self.fetch_latest_run(owner, repo, workflow, branch)?;

        match metric {
            "status" => Ok(DataValue::String(run.status)),
            "conclusion" => Ok(DataValue::String(
                run.conclusion.unwrap_or_else(|| "pending".to_string()),
            )),
            "run_number" => Ok(DataValue::Number(run.run_number)),
            "workflow" => Ok(DataValue::String(
                run.name.unwrap_or_else(|| "Unknown".to_string()),
            )),
            "event" => Ok(DataValue::String(run.event)),
            "branch" => Ok(DataValue::String(
                run.head_branch.unwrap_or_else(|| "Unknown".to_string()),
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
            "status",
            "conclusion",
            "run_number",
            "workflow",
            "event",
            "branch",
        ]
    }

    fn default_ttl(&self) -> u64 {
        300 // 5 minutes - workflow status changes frequently
    }

    fn metric_label(&self, metric: &str) -> &'static str {
        match metric {
            "status" => "Status",
            "conclusion" => "Build",
            "run_number" => "Run",
            "workflow" => "Workflow",
            "event" => "Event",
            "branch" => "Branch",
            _ => "Unknown",
        }
    }

    fn metric_color(&self, metric: &str, value: &DataValue) -> Option<&str> {
        match metric {
            "conclusion" => {
                if let DataValue::String(conclusion) = value {
                    match conclusion.as_str() {
                        "success" => Some("22C55E"),            // Green
                        "failure" => Some("EF4444"),            // Red
                        "cancelled" => Some("6B7280"),          // Gray
                        "skipped" => Some("6B7280"),            // Gray
                        "timed_out" => Some("F97316"),          // Orange
                        "action_required" => Some("EAB308"),    // Yellow
                        "pending" | "neutral" => Some("3B82F6"), // Blue
                        _ => Some("6B7280"),                    // Gray - unknown
                    }
                } else {
                    None
                }
            }
            "status" => {
                if let DataValue::String(status) = value {
                    match status.as_str() {
                        "completed" => Some("22C55E"),   // Green
                        "in_progress" => Some("3B82F6"), // Blue
                        "queued" => Some("EAB308"),      // Yellow
                        "waiting" => Some("EAB308"),     // Yellow
                        _ => Some("6B7280"),             // Gray
                    }
                } else {
                    None
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
    #[case("owner/repo", true, "owner", "repo", None, None)]
    #[case("owner/repo/ci", true, "owner", "repo", Some("ci"), None)]
    #[case("owner/repo@main", true, "owner", "repo", None, Some("main"))]
    #[case("owner/repo/ci@main", true, "owner", "repo", Some("ci"), Some("main"))]
    #[case("invalid", false, "", "", None, None)]
    #[case("a/b/c/d", false, "", "", None, None)]
    fn test_parse_query(
        #[case] input: &str,
        #[case] should_succeed: bool,
        #[case] expected_owner: &str,
        #[case] expected_repo: &str,
        #[case] expected_workflow: Option<&str>,
        #[case] expected_branch: Option<&str>,
    ) {
        let result = ActionsSource::parse_query(input);
        if should_succeed {
            let (owner, repo, workflow, branch) = result.unwrap();
            assert_eq!(owner, expected_owner);
            assert_eq!(repo, expected_repo);
            assert_eq!(workflow, expected_workflow);
            assert_eq!(branch, expected_branch);
        } else {
            assert!(result.is_err());
        }
    }

    // ========================================================================
    // Conclusion Colors (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("conclusion", DataValue::String("success".to_string()), Some("22C55E"))]
    #[case("conclusion", DataValue::String("failure".to_string()), Some("EF4444"))]
    #[case("conclusion", DataValue::String("cancelled".to_string()), Some("6B7280"))]
    #[case("conclusion", DataValue::String("pending".to_string()), Some("3B82F6"))]
    #[case("status", DataValue::String("completed".to_string()), Some("22C55E"))]
    #[case("status", DataValue::String("in_progress".to_string()), Some("3B82F6"))]
    #[case("status", DataValue::String("queued".to_string()), Some("EAB308"))]
    fn test_metric_colors(
        #[case] metric: &str,
        #[case] value: DataValue,
        #[case] expected: Option<&str>,
    ) {
        let source = ActionsSource::new();
        assert_eq!(source.metric_color(metric, &value), expected);
    }

    #[test]
    fn test_available_metrics() {
        let source = ActionsSource::new();
        let metrics = source.available_metrics();
        assert!(metrics.contains(&"status"));
        assert!(metrics.contains(&"conclusion"));
        assert!(metrics.contains(&"run_number"));
    }
}
