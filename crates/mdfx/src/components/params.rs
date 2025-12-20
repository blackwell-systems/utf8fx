//! Shared parameter definitions for components
//!
//! This module defines parameter metadata used by both the component handlers
//! and the LSP server for autocompletion.

/// Parameter metadata for LSP completions
pub struct ParamInfo {
    /// Parameter name (as used in templates)
    pub name: &'static str,
    /// Human-readable description
    pub description: &'static str,
    /// Example usage
    pub example: &'static str,
    /// Valid values (if enumerated)
    pub values: Option<&'static [(&'static str, &'static str)]>,
}

/// Tech badge parameters
pub static TECH_PARAMS: &[ParamInfo] = &[
    // Basic
    ParamInfo {
        name: "label",
        description: "Custom label text",
        example: "label=My Label",
        values: None,
    },
    ParamInfo {
        name: "bg",
        description: "Background color (both segments)",
        example: "bg=1a1a1a",
        values: None,
    },
    ParamInfo {
        name: "bg_left",
        description: "Left (icon) segment background",
        example: "bg_left=DEA584",
        values: None,
    },
    ParamInfo {
        name: "bg_right",
        description: "Right (label) segment background",
        example: "bg_right=B8856E",
        values: None,
    },
    ParamInfo {
        name: "logo",
        description: "Icon/logo color",
        example: "logo=FFFFFF",
        values: None,
    },
    ParamInfo {
        name: "text",
        description: "Label text color",
        example: "text=000000",
        values: None,
    },
    ParamInfo {
        name: "text_color",
        description: "Label text color (alias)",
        example: "text_color=FFFFFF",
        values: None,
    },
    ParamInfo {
        name: "color",
        description: "Label text color (alias)",
        example: "color=000000",
        values: None,
    },
    ParamInfo {
        name: "font",
        description: "Custom font family",
        example: "font=Monaco,monospace",
        values: None,
    },
    ParamInfo {
        name: "font_family",
        description: "Custom font family (alias)",
        example: "font_family=Arial",
        values: None,
    },
    // Sizing
    ParamInfo {
        name: "logo_size",
        description: "Icon size (xs/sm/md/lg/xl/xxl or pixels)",
        example: "logo_size=lg",
        values: Some(&[
            ("xs", "10px - Extra small"),
            ("sm", "12px - Small"),
            ("md", "14px - Medium (default)"),
            ("lg", "16px - Large"),
            ("xl", "18px - Extra large"),
            ("xxl", "20px - Extra extra large"),
        ]),
    },
    ParamInfo {
        name: "icon_size",
        description: "Icon size (alias for logo_size)",
        example: "icon_size=16",
        values: Some(&[
            ("xs", "10px - Extra small"),
            ("sm", "12px - Small"),
            ("md", "14px - Medium (default)"),
            ("lg", "16px - Large"),
            ("xl", "18px - Extra large"),
            ("xxl", "20px - Extra extra large"),
        ]),
    },
    ParamInfo {
        name: "height",
        description: "Badge height in pixels",
        example: "height=24",
        values: None,
    },
    ParamInfo {
        name: "raised",
        description: "Raised icon effect (pixels)",
        example: "raised=4",
        values: None,
    },
    // Corners & Shape
    ParamInfo {
        name: "rx",
        description: "Uniform corner radius",
        example: "rx=6",
        values: None,
    },
    ParamInfo {
        name: "corners",
        description: "Corner preset (left/right/none/all)",
        example: "corners=left",
        values: Some(&[
            ("left", "Rounded left, square right"),
            ("right", "Square left, rounded right"),
            ("none", "All square corners"),
            ("all", "All rounded corners"),
        ]),
    },
    ParamInfo {
        name: "top_left",
        description: "Top-left corner radius",
        example: "top_left=8",
        values: None,
    },
    ParamInfo {
        name: "top_right",
        description: "Top-right corner radius",
        example: "top_right=8",
        values: None,
    },
    ParamInfo {
        name: "bottom_left",
        description: "Bottom-left corner radius",
        example: "bottom_left=8",
        values: None,
    },
    ParamInfo {
        name: "bottom_right",
        description: "Bottom-right corner radius",
        example: "bottom_right=8",
        values: None,
    },
    ParamInfo {
        name: "chevron",
        description: "Arrow shape (left/right/both)",
        example: "chevron=right",
        values: Some(&[
            ("left", "Left-pointing arrow ←"),
            ("right", "Right-pointing arrow →"),
            ("both", "Both arrows ← →"),
        ]),
    },
    // Borders
    ParamInfo {
        name: "border",
        description: "Border color",
        example: "border=61DAFB",
        values: None,
    },
    ParamInfo {
        name: "border_width",
        description: "Border thickness",
        example: "border_width=2",
        values: None,
    },
    ParamInfo {
        name: "border_full",
        description: "Border around entire badge",
        example: "border_full=true",
        values: Some(&[("true", "Enable"), ("false", "Disable (default)")]),
    },
    ParamInfo {
        name: "divider",
        description: "Center divider line",
        example: "divider=true",
        values: Some(&[("true", "Enable"), ("false", "Disable (default)")]),
    },
    // Style
    ParamInfo {
        name: "style",
        description: "Badge style",
        example: "style=flat",
        values: Some(&[
            ("flat", "Rounded corners (rx=3)"),
            ("flat-square", "Sharp corners (default)"),
            ("plastic", "Shiny gradient overlay"),
            ("for-the-badge", "Tall blocks (height=28)"),
            ("social", "Very rounded (rx=10)"),
            ("outline", "Border-only with transparent fill"),
            ("ghost", "Alias for outline"),
        ]),
    },
    // Advanced
    ParamInfo {
        name: "icon",
        description: "Custom SVG path data",
        example: "icon=M12 2L2 7...",
        values: None,
    },
    ParamInfo {
        name: "source",
        description: "Render source (shields for shields.io)",
        example: "source=shields",
        values: Some(&[("shields", "Use shields.io URL instead of SVG")]),
    },
    ParamInfo {
        name: "url",
        description: "Make badge a clickable link",
        example: "url=https://example.com",
        values: None,
    },
];

/// Live badge sources and their metrics
pub static LIVE_SOURCES: &[(&str, &str, &[(&str, &str)])] = &[
    (
        "github",
        "GitHub repository metrics",
        &[
            ("stars", "Repository star count"),
            ("forks", "Fork count"),
            ("issues", "Open issue count"),
            ("watchers", "Watcher count"),
            ("size", "Repository size"),
            ("language", "Primary programming language"),
            ("license", "SPDX license identifier"),
            ("archived", "Is repository archived"),
            ("branch", "Default branch name"),
            ("topics", "Repository topics"),
            ("description", "Repository description"),
        ],
    ),
    (
        "npm",
        "npm package metrics",
        &[
            ("version", "Latest stable version"),
            ("license", "Package license"),
            ("next", "Latest @next tag version"),
            ("beta", "Latest @beta tag version"),
            ("description", "Package description"),
        ],
    ),
    (
        "crates",
        "crates.io package metrics",
        &[
            ("version", "Latest version"),
            ("downloads", "Total download count"),
            ("description", "Crate description"),
        ],
    ),
    (
        "pypi",
        "PyPI package metrics",
        &[
            ("version", "Latest version"),
            ("license", "Package license"),
            ("author", "Package author"),
            ("python", "Required Python version"),
            ("summary", "Package summary"),
        ],
    ),
    (
        "codecov",
        "Codecov coverage metrics",
        &[
            ("coverage", "Coverage percentage"),
            ("lines", "Total lines tracked"),
            ("hits", "Lines with coverage"),
            ("misses", "Lines without coverage"),
            ("files", "Number of files tracked"),
            ("branches", "Branch coverage count"),
        ],
    ),
    (
        "actions",
        "GitHub Actions workflow status",
        &[
            ("status", "Workflow run status (completed, in_progress, queued)"),
            ("conclusion", "Workflow conclusion (success, failure, cancelled)"),
            ("run_number", "Workflow run number"),
            ("workflow", "Workflow name"),
            ("event", "Trigger event"),
            ("branch", "Branch name"),
        ],
    ),
    (
        "docker",
        "Docker Hub image metrics",
        &[
            ("pulls", "Total pull count"),
            ("pulls_raw", "Unformatted pull count"),
            ("stars", "Star count"),
            ("tag", "Latest tag"),
            ("description", "Image description"),
            ("official", "Official or Community"),
        ],
    ),
    (
        "packagist",
        "Packagist (PHP) package metrics",
        &[
            ("version", "Latest stable version"),
            ("downloads", "Total download count"),
            ("downloads_raw", "Unformatted download count"),
            ("monthly", "Monthly downloads"),
            ("daily", "Daily downloads"),
            ("stars", "Star/faver count"),
            ("license", "Package license"),
            ("php", "Required PHP version"),
            ("description", "Package description"),
        ],
    ),
    (
        "rubygems",
        "RubyGems package metrics",
        &[
            ("version", "Latest version"),
            ("downloads", "Total download count"),
            ("downloads_raw", "Unformatted download count"),
            ("version_downloads", "Downloads for latest version"),
            ("license", "Gem license"),
            ("authors", "Gem authors"),
            ("info", "Gem info"),
            ("ruby", "Required Ruby version"),
        ],
    ),
    (
        "nuget",
        "NuGet (.NET) package metrics",
        &[
            ("version", "Latest version"),
            ("downloads", "Total download count"),
            ("downloads_raw", "Unformatted download count"),
            ("description", "Package description"),
            ("authors", "Package authors"),
            ("license", "Package license"),
        ],
    ),
];

/// Get valid live sources
pub fn valid_live_sources() -> impl Iterator<Item = &'static str> {
    LIVE_SOURCES.iter().map(|(name, _, _)| *name)
}

/// Get metrics for a live source
pub fn metrics_for_source(source: &str) -> Option<&'static [(&'static str, &'static str)]> {
    LIVE_SOURCES
        .iter()
        .find(|(name, _, _)| *name == source)
        .map(|(_, _, metrics)| *metrics)
}

/// Check if a metric is valid for a source
pub fn is_valid_metric(source: &str, metric: &str) -> bool {
    metrics_for_source(source)
        .map(|metrics| metrics.iter().any(|(name, _)| *name == metric))
        .unwrap_or(false)
}

/// Get all valid tech badge parameter names
pub fn valid_tech_param_names() -> impl Iterator<Item = &'static str> {
    TECH_PARAMS.iter().map(|p| p.name)
}

/// Check if a tech badge parameter name is valid
pub fn is_valid_tech_param(name: &str) -> bool {
    TECH_PARAMS.iter().any(|p| p.name == name)
}

/// Get unknown tech badge parameters from a list
/// Returns parameter names that are not recognized
pub fn unknown_tech_params<'a>(params: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
    params.filter(|name| !is_valid_tech_param(name)).collect()
}
