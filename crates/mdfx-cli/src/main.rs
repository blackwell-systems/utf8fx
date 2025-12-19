use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::Colorize;
use mdfx::manifest::AssetManifest;
use mdfx::renderer::plaintext::PlainTextBackend;
use mdfx::renderer::shields::ShieldsBackend;
use mdfx::renderer::svg::SvgBackend;
use mdfx::{
    available_targets, detect_target_from_path, get_target, BackendType, Converter, Error,
    MdfxConfig, Registry, StyleCategory, Target, TemplateParser,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use std::sync::mpsc::channel;
use std::time::Duration;

#[cfg(feature = "lsp")]
mod lsp;

/// Markdown effects: Unicode text styling and UI components
#[derive(Parser)]
#[command(name = "mdfx")]
#[command(version, about)]
#[command(
    long_about = "Transform markdown with Unicode text effects and UI components through template syntax.\n\nSupports 24 styles including mathbold, fullwidth, script, fraktur, and more.\nUse templates: {{mathbold}}TEXT{{/mathbold}}\n\nFor more info: https://github.com/blackwell-systems/mdfx"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert text to a Unicode style
    ///
    /// Transform plain text into styled Unicode characters using one of 24 available styles.
    /// Supports style aliases (e.g., 'mb' for 'mathbold') and character spacing.
    ///
    /// Examples:
    ///   mdfx convert --style mathbold "Hello World"
    ///   mdfx convert --style mb --spacing 1 "SPACED"
    ///   mdfx convert --style script "Elegant Text"
    ///
    /// Run 'mdfx list' to see all available styles.
    Convert {
        /// The style to use (e.g., mathbold, fullwidth, mb)
        #[arg(short, long)]
        style: String,

        /// Number of spaces between each character (0 = no spacing)
        #[arg(long, default_value = "0")]
        spacing: usize,

        /// The text to convert
        text: String,
    },

    /// List available resources
    ///
    /// Display available styles, components, glyphs, frames, or palette colors.
    /// Without a resource type, lists all 24 Unicode text styles.
    ///
    /// Examples:
    ///   mdfx list                  # List styles (default)
    ///   mdfx list styles --samples
    ///   mdfx list components       # List UI components
    ///   mdfx list glyphs           # List named glyphs
    ///   mdfx list frames           # List frame styles
    ///   mdfx list palette          # List palette colors
    List {
        /// Resource type to list (styles, components, glyphs, frames, palette)
        #[arg(default_value = "styles")]
        resource: String,

        /// Show only styles in a specific category (for styles)
        #[arg(short, long)]
        category: Option<String>,

        /// Show sample output (for styles)
        #[arg(short, long)]
        samples: bool,

        /// Search/filter results by name pattern
        #[arg(short = 'f', long)]
        filter: Option<String>,
    },

    /// Process markdown file with style templates
    ///
    /// Transform markdown files by processing style templates in the format:
    /// {{style}}text{{/style}} or {{style:spacing=N}}text{{/style}}
    ///
    /// Templates are preserved inside code blocks (```) and inline code (`).
    ///
    /// Examples:
    ///   mdfx process input.md -o output.md
    ///   mdfx process -i README.md
    ///   mdfx process --target github input.md
    ///   mdfx process --target local --assets-dir docs/assets input.md
    ///   echo "{{mathbold}}Title{{/mathbold}}" | mdfx process
    ///
    /// Targets:
    ///   github - GitHub README (shields.io badges, default)
    ///   local  - Local docs (SVG files, offline-first)
    ///   npm    - npm package README (like GitHub)
    ///   gitlab - GitLab README (more HTML support)
    ///   pypi   - PyPI package (plain text, ASCII-safe)
    ///   auto   - Auto-detect from output path
    ///
    /// Template syntax:
    ///   {{mathbold}}Bold Text{{/mathbold}}
    ///   {{script:spacing=2}}Spaced Script{{/script}}
    ///   {{ui:swatch:accent/}}
    ///   {{ui:tech:rust/}}
    Process {
        /// Input file (use - or omit for stdin)
        input: Option<PathBuf>,

        /// Output file (use - or omit for stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Modify file in place
        #[arg(short = 'i', long)]
        in_place: bool,

        /// Target platform (github, local, npm, gitlab, pypi, auto)
        #[arg(short, long, default_value = "github")]
        target: String,

        /// Rendering backend override (shields, svg). If not set, uses target's preferred backend.
        #[arg(short, long)]
        backend: Option<String>,

        /// Output directory for SVG assets (only used with svg backend)
        #[arg(long, default_value = "assets/mdfx")]
        assets_dir: String,

        /// Custom palette JSON file for color definitions
        /// Format: {"colorName": "HEXVALUE", ...}
        #[arg(long)]
        palette: Option<PathBuf>,

        /// mdfx configuration file (default: auto-discover .mdfx.json)
        /// Contains partials, palette, and other project settings
        #[arg(long)]
        config: Option<PathBuf>,
    },

    /// Generate shell completions
    ///
    /// Generate tab completion scripts for your shell. Save the output to
    /// your shell's completion directory.
    ///
    /// Examples:
    ///   mdfx completions bash > /etc/bash_completion.d/mdfx
    ///   mdfx completions zsh > ~/.zsh/completions/_mdfx
    ///   mdfx completions fish > ~/.config/fish/completions/mdfx.fish
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Verify asset integrity against manifest
    ///
    /// Check that all assets in manifest.json exist on disk with correct hashes.
    /// Useful for detecting corruption or verifying CI caches.
    ///
    /// Examples:
    ///   mdfx verify --assets-dir assets/mdfx
    ///   mdfx verify  # Uses default assets/mdfx
    Verify {
        /// Assets directory containing manifest.json
        #[arg(long, default_value = "assets/mdfx")]
        assets_dir: String,
    },

    /// Clean unreferenced assets
    ///
    /// Remove asset files that are not referenced in manifest.json or markdown files.
    /// Without --scan, removes files not in manifest.json.
    /// With --scan, removes files not referenced in any markdown file.
    ///
    /// Examples:
    ///   mdfx clean --assets-dir assets/mdfx
    ///   mdfx clean --dry-run  # Show what would be deleted
    ///   mdfx clean --scan "docs/**/*.md"  # Scan markdown files for references
    ///   mdfx clean --scan "*.md" --dry-run  # Preview what would be deleted
    Clean {
        /// Assets directory containing manifest.json
        #[arg(long, default_value = "assets/mdfx")]
        assets_dir: String,

        /// Show what would be deleted without actually deleting
        #[arg(long)]
        dry_run: bool,

        /// Glob pattern for markdown files to scan for asset references
        /// When set, only assets referenced in matching files are kept
        #[arg(long)]
        scan: Option<String>,
    },

    /// Build markdown to multiple targets at once
    ///
    /// Compiles the same source file to multiple platform-specific outputs.
    /// Each target gets its own output file with appropriate rendering.
    ///
    /// Examples:
    ///   mdfx build README.template.md --output-dir dist/
    ///   mdfx build README.template.md --targets github,pypi,npm
    ///   mdfx build README.template.md --all-targets
    Build {
        /// Input markdown file
        input: PathBuf,

        /// Output directory for generated files
        #[arg(short, long, default_value = "dist")]
        output_dir: String,

        /// Comma-separated list of targets (github,local,npm,gitlab,pypi)
        #[arg(short, long)]
        targets: Option<String>,

        /// Build for all available targets
        #[arg(long)]
        all_targets: bool,

        /// Custom palette JSON file for color definitions
        #[arg(long)]
        palette: Option<PathBuf>,
    },

    /// Watch file for changes and rebuild automatically
    ///
    /// Monitor the input file and automatically rebuild the output when changes
    /// are detected. Useful during development for live preview.
    ///
    /// Examples:
    ///   mdfx watch input.md -o output.md
    ///   mdfx watch README.template.md -o README.md --target github
    ///   mdfx watch docs/source.md -o docs/rendered.md --backend svg
    Watch {
        /// Input file to watch
        input: PathBuf,

        /// Output file
        #[arg(short, long)]
        output: PathBuf,

        /// Target platform (github, local, npm, auto)
        #[arg(short, long, default_value = "github")]
        target: String,

        /// Rendering backend override (shields, svg)
        #[arg(short, long)]
        backend: Option<String>,

        /// Output directory for SVG assets (only used with svg backend)
        #[arg(long, default_value = "assets/mdfx")]
        assets_dir: String,

        /// Custom palette JSON file for color definitions
        #[arg(long)]
        palette: Option<PathBuf>,

        /// Debounce delay in milliseconds
        #[arg(long, default_value = "100")]
        debounce: u64,

        /// mdfx configuration file (default: auto-discover .mdfx.json)
        #[arg(long)]
        config: Option<PathBuf>,
    },

    /// Language Server Protocol (LSP) commands
    ///
    /// Provides IDE integration with autocompletion for mdfx template syntax.
    /// Use 'mdfx lsp install' to set up editor extensions automatically.
    ///
    /// Requires: cargo install mdfx-cli --features lsp
    #[cfg(feature = "lsp")]
    #[command(subcommand)]
    Lsp(LspCommands),
}

/// LSP subcommands
#[cfg(feature = "lsp")]
#[derive(Subcommand)]
enum LspCommands {
    /// Start the LSP server (default)
    ///
    /// The server communicates over stdio using the LSP protocol.
    /// This is typically called by your editor, not manually.
    Run,

    /// Install editor extension for LSP support
    ///
    /// Automatically sets up the mdfx LSP extension for your editor.
    /// Currently supports VS Code.
    ///
    /// Examples:
    ///   mdfx lsp install --editor vscode
    ///   mdfx lsp install  # defaults to vscode
    Install {
        /// Editor to install extension for (vscode)
        #[arg(short, long, default_value = "vscode")]
        editor: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Error> {
    let converter = Converter::new()?;

    match cli.command {
        Commands::Convert {
            style,
            spacing,
            text,
        } => {
            let result = converter.convert_with_spacing(&text, &style, spacing)?;
            println!("{}", result);
        }

        Commands::List {
            resource,
            category,
            samples,
            filter,
        } => {
            let registry = Registry::new()?;
            match resource.as_str() {
                "styles" => list_styles(&converter, category, samples)?,
                "components" => list_components(&registry, filter)?,
                "glyphs" => list_glyphs(&registry, filter)?,
                "frames" => list_frames(&registry, filter)?,
                "palette" => list_palette(&registry, filter)?,
                _ => {
                    return Err(Error::ParseError(format!(
                    "Unknown resource '{}'. Available: styles, components, glyphs, frames, palette",
                    resource
                )))
                }
            }
        }

        Commands::Process {
            input,
            output,
            in_place,
            target,
            backend,
            assets_dir,
            palette,
            config,
        } => {
            process_file(
                input,
                output,
                in_place,
                &target,
                backend.as_deref(),
                &assets_dir,
                palette.as_deref(),
                config.as_deref(),
            )?;
        }

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "mdfx", &mut io::stdout());
        }

        Commands::Verify { assets_dir } => {
            verify_assets(&assets_dir)?;
        }

        Commands::Clean {
            assets_dir,
            dry_run,
            scan,
        } => {
            clean_assets(&assets_dir, dry_run, scan.as_deref())?;
        }

        Commands::Build {
            input,
            output_dir,
            targets,
            all_targets,
            palette,
        } => {
            build_multi_target(
                &input,
                &output_dir,
                targets.as_deref(),
                all_targets,
                palette.as_deref(),
            )?;
        }

        Commands::Watch {
            input,
            output,
            target,
            backend,
            assets_dir,
            palette,
            debounce,
            config,
        } => {
            watch_file(
                input,
                output,
                &target,
                backend.as_deref(),
                &assets_dir,
                palette.as_deref(),
                debounce,
                config.as_deref(),
            )?;
        }

        #[cfg(feature = "lsp")]
        Commands::Lsp(lsp_cmd) => match lsp_cmd {
            LspCommands::Run => {
                // Run the LSP server using tokio runtime
                tokio::runtime::Runtime::new()
                    .expect("Failed to create tokio runtime")
                    .block_on(lsp::run_lsp_server());
            }
            LspCommands::Install { editor } => {
                install_lsp_extension(&editor)?;
            }
        },
    }

    Ok(())
}

fn list_styles(
    converter: &Converter,
    category: Option<String>,
    show_samples: bool,
) -> Result<(), Error> {
    println!("{}", "Available styles:".bold());
    println!();

    let styles = converter.list_styles();

    // Group by category
    let categories = [
        (StyleCategory::Bold, "Bold & Impactful"),
        (StyleCategory::Boxed, "Boxed"),
        (StyleCategory::Technical, "Technical & Code"),
        (StyleCategory::Elegant, "Subtle & Elegant"),
    ];

    for (cat, label) in categories {
        // Filter by category if specified
        if let Some(ref filter) = category {
            let cat_str = format!("{:?}", cat).to_lowercase();
            if cat_str != filter.to_lowercase() {
                continue;
            }
        }

        let cat_styles: Vec<_> = styles.iter().filter(|s| s.category == cat).collect();

        if cat_styles.is_empty() {
            continue;
        }

        println!("{}", label.yellow().bold());

        for style in cat_styles {
            print!("  {}", style.id.green());

            // Show aliases
            if !style.aliases.is_empty() {
                print!(" ({})", style.aliases.join(", ").dimmed());
            }

            // Show description
            println!(" - {}", style.description.dimmed());

            // Show sample if requested
            if show_samples {
                let sample = converter.convert("ABC123", &style.id)?;
                println!("    Sample: {}", sample.cyan());
            }
        }

        println!();
    }

    Ok(())
}

fn list_components(registry: &Registry, filter: Option<String>) -> Result<(), Error> {
    println!("{}", "Available UI components:".bold());
    println!();

    let components = registry.components();
    let mut entries: Vec<_> = components.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    // Apply filter if provided
    let entries: Vec<_> = if let Some(ref pattern) = filter {
        let pattern = pattern.to_lowercase();
        entries
            .into_iter()
            .filter(|(name, _)| name.to_lowercase().contains(&pattern))
            .collect()
    } else {
        entries
    };

    for (name, comp) in entries {
        print!("  {}", name.green());
        if let Some(ref desc) = comp.description {
            println!(" - {}", desc.dimmed());
        } else {
            println!();
        }
        // Show optional params if any
        if let Some(ref params) = comp.optional_params {
            let param_names: Vec<_> = params.keys().collect();
            if !param_names.is_empty() {
                let mut sorted_names: Vec<_> = param_names.iter().map(|s| s.as_str()).collect();
                sorted_names.sort();
                println!("    Params: {}", sorted_names.join(", ").cyan());
            }
        }
    }

    println!();
    println!(
        "Total: {} components",
        components.len().to_string().yellow()
    );

    Ok(())
}

fn list_glyphs(registry: &Registry, filter: Option<String>) -> Result<(), Error> {
    println!("{}", "Available named glyphs:".bold());
    println!();

    let glyphs = registry.glyphs();
    let mut entries: Vec<_> = glyphs.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    // Apply filter if provided
    let entries: Vec<_> = if let Some(ref pattern) = filter {
        let pattern = pattern.to_lowercase();
        entries
            .into_iter()
            .filter(|(name, _)| name.to_lowercase().contains(&pattern))
            .collect()
    } else {
        entries
    };

    // Group by prefix (e.g., block.*, shade.*, etc.)
    let mut groups: std::collections::BTreeMap<String, Vec<(&String, &String)>> =
        std::collections::BTreeMap::new();

    for (name, char) in &entries {
        let prefix = name.split('.').next().unwrap_or("other").to_string();
        groups.entry(prefix).or_default().push((name, char));
    }

    for (prefix, glyphs) in groups {
        println!("{}", format!("{}.*", prefix).yellow().bold());
        for (name, char) in glyphs {
            println!("  {} → {}", name.green(), char.cyan());
        }
        println!();
    }

    println!("Total: {} glyphs", entries.len().to_string().yellow());
    println!();
    println!(
        "{}",
        "Usage: {{glyph:name/}} or {{fr:glyph:name:text/}}".dimmed()
    );

    Ok(())
}

fn list_frames(registry: &Registry, filter: Option<String>) -> Result<(), Error> {
    println!("{}", "Available frame styles:".bold());
    println!();

    let frames = registry.frames();
    let mut entries: Vec<_> = frames.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    // Apply filter if provided
    let entries: Vec<_> = if let Some(ref pattern) = filter {
        let pattern = pattern.to_lowercase();
        entries
            .into_iter()
            .filter(|(name, _)| name.to_lowercase().contains(&pattern))
            .collect()
    } else {
        entries
    };

    for (name, frame) in entries {
        print!("  {}", name.green());
        if !frame.aliases.is_empty() {
            print!(" ({})", frame.aliases.join(", ").dimmed());
        }

        // Show prefix/suffix preview
        let preview = format!("{}...{}", frame.prefix, frame.suffix);
        println!(" → {}", preview.cyan());

        if let Some(ref desc) = frame.description {
            println!("    {}", desc.dimmed());
        }
    }

    println!();
    println!("Total: {} frames", frames.len().to_string().yellow());
    println!();
    println!(
        "{}",
        "Usage: {{fr:name}}text{{/}} or {{fr:name:inline text/}}".dimmed()
    );

    Ok(())
}

fn list_palette(registry: &Registry, filter: Option<String>) -> Result<(), Error> {
    println!("{}", "Available palette colors:".bold());
    println!();

    let palette = registry.palette();
    let mut entries: Vec<_> = palette.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    // Apply filter if provided
    let entries: Vec<_> = if let Some(ref pattern) = filter {
        let pattern = pattern.to_lowercase();
        entries
            .into_iter()
            .filter(|(name, _)| name.to_lowercase().contains(&pattern))
            .collect()
    } else {
        entries
    };

    // Group by prefix (e.g., dark*, ui.*, etc.)
    let mut semantic: Vec<_> = Vec::new();
    let mut ui: Vec<_> = Vec::new();
    let mut other: Vec<_> = Vec::new();

    for (name, hex) in &entries {
        if ["success", "warning", "error", "info", "accent"].contains(&name.as_str()) {
            semantic.push((name, hex));
        } else if name.starts_with("ui.") || name.starts_with("dark") {
            ui.push((name, hex));
        } else {
            other.push((name, hex));
        }
    }

    if !semantic.is_empty() {
        println!("{}", "Semantic".yellow().bold());
        for (name, hex) in semantic {
            println!("  {} → #{}", name.green(), hex.cyan());
        }
        println!();
    }

    if !ui.is_empty() {
        println!("{}", "UI / Dark Theme".yellow().bold());
        for (name, hex) in ui {
            println!("  {} → #{}", name.green(), hex.cyan());
        }
        println!();
    }

    if !other.is_empty() {
        println!("{}", "General".yellow().bold());
        for (name, hex) in other {
            println!("  {} → #{}", name.green(), hex.cyan());
        }
        println!();
    }

    println!("Total: {} colors", entries.len().to_string().yellow());
    println!();
    println!(
        "{}",
        "Usage: {{ui:swatch:name/}} or color params like bg=name".dimmed()
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn process_file(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    in_place: bool,
    target_name: &str,
    backend_override: Option<&str>,
    assets_dir: &str,
    palette_path: Option<&std::path::Path>,
    config_path: Option<&std::path::Path>,
) -> Result<(), Error> {
    // Resolve target (with auto-detection support)
    let target: Box<dyn Target> = if target_name == "auto" {
        // Auto-detect from output path
        let detected = output
            .as_ref()
            .and_then(|p| detect_target_from_path(p))
            .or_else(|| input.as_ref().and_then(|p| detect_target_from_path(p)));

        if let Some(name) = detected {
            eprintln!("{} Auto-detected target: {}", "Info:".cyan(), name.green());
            get_target(name).unwrap()
        } else {
            eprintln!(
                "{} Could not auto-detect target, using github",
                "Info:".cyan()
            );
            get_target("github").unwrap()
        }
    } else {
        get_target(target_name).ok_or_else(|| {
            Error::ParseError(format!(
                "Unknown target '{}'. Available: {}",
                target_name,
                available_targets().join(", ")
            ))
        })?
    };

    // Determine backend: explicit override > target's preferred backend
    let backend_type = if let Some(backend) = backend_override {
        match backend {
            "shields" => BackendType::Shields,
            "svg" => BackendType::Svg,
            _ => {
                return Err(Error::ParseError(format!(
                    "Unknown backend '{}'. Available: svg, shields",
                    backend
                )));
            }
        }
    } else {
        target.preferred_backend()
    };

    // Create the appropriate backend
    let mut parser = match backend_type {
        BackendType::Svg => TemplateParser::with_backend(Box::new(SvgBackend::new(assets_dir)))?,
        BackendType::Shields => TemplateParser::with_backend(Box::new(ShieldsBackend::new()?))?,
        BackendType::PlainText => TemplateParser::with_backend(Box::new(PlainTextBackend::new()))?,
    };

    // Load config file (explicit path or auto-discover)
    let config = if let Some(config_file) = config_path {
        Some(MdfxConfig::load(config_file)?)
    } else {
        MdfxConfig::discover()
    };

    if let Some(ref cfg) = config {
        let partial_count = cfg.partials.len();
        let palette_count = cfg.palette.len();

        if partial_count > 0 || palette_count > 0 {
            eprintln!(
                "{} Loaded config: {} partial(s), {} color(s)",
                "Info:".cyan(),
                partial_count,
                palette_count
            );
        }

        parser.load_config(cfg);
    }

    // Load custom palette if provided (overrides config palette)
    if let Some(palette_file) = palette_path {
        let palette_content = fs::read_to_string(palette_file).map_err(Error::IoError)?;
        let custom_palette: std::collections::HashMap<String, String> =
            serde_json::from_str(&palette_content).map_err(|e| {
                Error::ParseError(format!(
                    "Failed to parse palette file '{}': {}",
                    palette_file.display(),
                    e
                ))
            })?;
        eprintln!(
            "{} Loaded {} custom color(s) from {}",
            "Info:".cyan(),
            custom_palette.len(),
            palette_file.display()
        );
        parser.extend_palette(custom_palette);
    }

    // Read input
    let content = if let Some(ref path) = input {
        if path.to_str() == Some("-") {
            // Read from stdin
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .map_err(Error::IoError)?;
            buffer
        } else {
            // Read from file
            fs::read_to_string(path).map_err(Error::IoError)?
        }
    } else {
        // No input specified, read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(Error::IoError)?;
        buffer
    };

    // Process content with asset collection
    let processed_result = parser.process_with_assets(&content)?;

    // Write any file-based assets to disk
    if !processed_result.assets.is_empty() {
        // Ensure assets directory exists
        fs::create_dir_all(assets_dir).map_err(Error::IoError)?;

        // Build manifest for SVG backend
        let mut manifest = if matches!(backend_type, BackendType::Svg) {
            Some(AssetManifest::new("svg", assets_dir))
        } else {
            None
        };

        let mut written = 0;
        let mut skipped = 0;

        for asset in &processed_result.assets {
            if let Some(path) = asset.file_path() {
                if let Some(bytes) = asset.file_bytes() {
                    // Skip if file already exists (hash-based names mean same content)
                    let path_ref = std::path::Path::new(path);
                    if path_ref.exists() {
                        skipped += 1;
                    } else {
                        // Write the asset file
                        fs::write(path, bytes).map_err(Error::IoError)?;
                        written += 1;
                    }

                    // Add to manifest if SVG backend
                    if let Some(ref mut m) = manifest {
                        if let mdfx::RenderedAsset::File {
                            relative_path,
                            bytes,
                            primitive,
                            ..
                        } = asset
                        {
                            let asset_type = match primitive.as_ref() {
                                mdfx::Primitive::Swatch { .. } => "swatch",
                                mdfx::Primitive::Tech(_) => "tech",
                                mdfx::Primitive::Progress { .. } => "progress",
                                mdfx::Primitive::Donut { .. } => "donut",
                                mdfx::Primitive::Gauge { .. } => "gauge",
                                mdfx::Primitive::Sparkline { .. } => "sparkline",
                                mdfx::Primitive::Rating { .. } => "rating",
                                mdfx::Primitive::Waveform { .. } => "waveform",
                            };
                            m.add_asset(
                                relative_path.clone(),
                                bytes,
                                primitive.as_ref(),
                                asset_type.to_string(),
                            );
                        }
                    }
                }
            }
        }

        // Write manifest.json for SVG backend
        if let Some(manifest) = manifest {
            let manifest_path = format!("{}/manifest.json", assets_dir);
            manifest.write(std::path::Path::new(&manifest_path))?;
        }

        // Report asset generation results
        if written > 0 || skipped > 0 {
            let mut parts = Vec::new();
            if written > 0 {
                parts.push(format!("{} written", written));
            }
            if skipped > 0 {
                parts.push(format!("{} unchanged", skipped));
            }
            eprintln!(
                "{} Assets: {} ({})",
                "Info:".cyan(),
                parts.join(", "),
                assets_dir
            );
        }
    }

    // Apply target-specific post-processing
    let processed = target.post_process(&processed_result.markdown)?;

    // Write output
    if in_place {
        // In-place requires input file
        if let Some(ref path) = input {
            if path.to_str() == Some("-") {
                return Err(Error::IoError(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Cannot use --in-place with stdin",
                )));
            }
            fs::write(path, processed).map_err(Error::IoError)?;
            eprintln!("{} {}", "Processed:".green(), path.display());
        } else {
            return Err(Error::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot use --in-place without input file",
            )));
        }
    } else if let Some(ref path) = output {
        if path.to_str() == Some("-") {
            // Write to stdout
            print!("{}", processed);
        } else {
            // Write to file
            fs::write(path, processed).map_err(Error::IoError)?;
            eprintln!("{} {}", "Wrote:".green(), path.display());
        }
    } else {
        // No output specified, write to stdout
        print!("{}", processed);
    }

    Ok(())
}

fn verify_assets(assets_dir: &str) -> Result<(), Error> {
    let manifest_path = format!("{}/manifest.json", assets_dir);

    println!("{}", "Verifying assets...".bold());
    println!();

    // Load manifest
    let manifest = match AssetManifest::load(std::path::Path::new(&manifest_path)) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("{} manifest.json not found", "Error:".red().bold());
            eprintln!("Run with --backend svg to generate a manifest.");
            process::exit(1);
        }
    };

    println!(
        "Manifest: {} ({})",
        manifest_path.dimmed(),
        manifest.created_at.dimmed()
    );
    println!("Backend: {}", manifest.backend.cyan());
    println!("Total assets: {}", manifest.total_assets);
    println!();

    // Verify each asset
    let results = manifest.verify(std::path::Path::new("."));

    let mut valid_count = 0;
    let mut missing_count = 0;
    let mut mismatch_count = 0;
    let mut error_count = 0;

    for result in results {
        match result {
            mdfx::VerificationResult::Valid { path } => {
                println!("  {} {}", "✓".green(), path.dimmed());
                valid_count += 1;
            }
            mdfx::VerificationResult::Missing { path } => {
                println!("  {} {} {}", "✗".red(), path, "(missing)".red());
                missing_count += 1;
            }
            mdfx::VerificationResult::HashMismatch {
                path,
                expected,
                actual,
            } => {
                println!("  {} {} {}", "✗".red(), path, "(hash mismatch)".red());
                println!("    Expected: {}", expected.dimmed());
                println!("    Actual:   {}", actual.dimmed());
                mismatch_count += 1;
            }
            mdfx::VerificationResult::ReadError { path, error } => {
                println!(
                    "  {} {} {} {}",
                    "✗".red(),
                    path,
                    "(read error:".red(),
                    format!("{})", error).red()
                );
                error_count += 1;
            }
        }
    }

    println!();
    println!("{}", "Summary:".bold());
    println!("  Valid: {}", valid_count.to_string().green());

    if missing_count > 0 {
        println!("  Missing: {}", missing_count.to_string().red());
    }
    if mismatch_count > 0 {
        println!("  Hash mismatches: {}", mismatch_count.to_string().red());
    }
    if error_count > 0 {
        println!("  Errors: {}", error_count.to_string().red());
    }

    // Exit with error if any problems found
    if missing_count > 0 || mismatch_count > 0 || error_count > 0 {
        process::exit(1);
    }

    println!();
    println!("{}", "✓ All assets verified successfully!".green().bold());

    Ok(())
}

fn clean_assets(assets_dir: &str, dry_run: bool, scan_pattern: Option<&str>) -> Result<(), Error> {
    let manifest_path = format!("{}/manifest.json", assets_dir);

    println!(
        "{}",
        if dry_run {
            "Dry run: showing what would be deleted...".bold()
        } else {
            "Cleaning unreferenced assets...".bold()
        }
    );
    println!();

    // Determine which assets are referenced
    let referenced: std::collections::HashSet<String> = if let Some(pattern) = scan_pattern {
        // Scan markdown files for asset references
        println!("{} Scanning markdown files: {}", "Info:".cyan(), pattern);
        scan_markdown_for_assets(pattern, assets_dir)?
    } else {
        // Load manifest
        let manifest = match AssetManifest::load(std::path::Path::new(&manifest_path)) {
            Ok(m) => m,
            Err(_) => {
                eprintln!("{} manifest.json not found", "Error:".red().bold());
                eprintln!("Run with --backend svg to generate a manifest.");
                process::exit(1);
            }
        };

        // Get referenced asset paths from manifest
        manifest
            .asset_paths()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    };

    println!(
        "{} Found {} referenced assets",
        "Info:".cyan(),
        referenced.len()
    );
    println!();

    // Find all SVG files in assets directory
    let assets_path = std::path::Path::new(assets_dir);
    if !assets_path.exists() {
        println!("{}", "No assets directory found.".yellow());
        return Ok(());
    }

    let mut deleted_count = 0;
    let mut total_bytes = 0;
    let mut kept_count = 0;

    for entry in fs::read_dir(assets_path).map_err(Error::IoError)? {
        let entry = entry.map_err(Error::IoError)?;
        let path = entry.path();

        // Skip manifest.json itself
        if path.file_name().and_then(|n| n.to_str()) == Some("manifest.json") {
            continue;
        }

        // Only process .svg files
        if path.extension().and_then(|e| e.to_str()) != Some("svg") {
            continue;
        }

        let relative_path = path.to_str().unwrap().to_string();

        // Also check just the filename (some refs may use different paths)
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();

        // Check if this asset is referenced (by full path or filename)
        let is_referenced =
            referenced.contains(&relative_path) || referenced.iter().any(|r| r.ends_with(filename));

        if !is_referenced {
            let metadata = fs::metadata(&path).map_err(Error::IoError)?;
            let size = metadata.len();

            println!(
                "  {} {}",
                if dry_run {
                    "Would delete:".yellow()
                } else {
                    "Deleting:".red()
                },
                relative_path
            );

            if !dry_run {
                fs::remove_file(&path).map_err(Error::IoError)?;
            }

            deleted_count += 1;
            total_bytes += size;
        } else {
            kept_count += 1;
        }
    }

    println!();
    if deleted_count == 0 {
        println!("{}", "✓ No unreferenced assets found.".green());
        println!("  {} assets kept", kept_count);
    } else {
        let size_kb = total_bytes as f64 / 1024.0;
        println!(
            "{} {} assets ({:.1} KB)",
            if dry_run {
                "Would delete:".yellow()
            } else {
                "Deleted:".green()
            },
            deleted_count,
            size_kb
        );
        println!("  {} assets kept", kept_count);
    }

    // Update manifest if we're in scan mode and not dry-run
    if scan_pattern.is_some() && !dry_run && deleted_count > 0 {
        update_manifest_after_clean(assets_dir, &referenced)?;
    }

    Ok(())
}

/// Scan markdown files for asset references and return set of referenced paths
fn scan_markdown_for_assets(
    pattern: &str,
    assets_dir: &str,
) -> Result<std::collections::HashSet<String>, Error> {
    use regex::Regex;

    let mut referenced = std::collections::HashSet::new();

    // Pattern to match image references: ![...](path) or <img src="path">
    let img_regex = Regex::new(r#"!\[[^\]]*\]\(([^)]+)\)|<img[^>]+src=["']([^"']+)["']"#)
        .map_err(|e| Error::ParseError(format!("Invalid regex: {}", e)))?;

    // Get assets directory basename for matching
    let assets_basename = std::path::Path::new(assets_dir)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("assets");

    // Use glob to find matching files
    let glob_pattern = glob::glob(pattern)
        .map_err(|e| Error::ParseError(format!("Invalid glob pattern '{}': {}", pattern, e)))?;

    let mut files_scanned = 0;

    for entry in glob_pattern {
        let path = entry.map_err(|e| Error::ParseError(format!("Glob error: {}", e)))?;

        // Read markdown file
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue, // Skip files we can't read
        };

        files_scanned += 1;

        // Find all image references
        for cap in img_regex.captures_iter(&content) {
            let src = cap.get(1).or_else(|| cap.get(2));
            if let Some(m) = src {
                let asset_path = m.as_str();

                // Check if this looks like one of our assets
                if asset_path.contains(assets_basename) && asset_path.ends_with(".svg") {
                    // Normalize the path
                    let normalized = asset_path
                        .trim_start_matches("./")
                        .trim_start_matches("../");
                    referenced.insert(normalized.to_string());

                    // Also add just the filename
                    if let Some(filename) = std::path::Path::new(asset_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                    {
                        referenced.insert(filename.to_string());
                    }
                }
            }
        }
    }

    println!("  Scanned {} markdown file(s)", files_scanned);

    Ok(referenced)
}

/// Update manifest.json after cleaning to only include kept assets
fn update_manifest_after_clean(
    assets_dir: &str,
    referenced: &std::collections::HashSet<String>,
) -> Result<(), Error> {
    let manifest_path = format!("{}/manifest.json", assets_dir);

    // Load existing manifest
    let manifest = match AssetManifest::load(std::path::Path::new(&manifest_path)) {
        Ok(m) => m,
        Err(_) => return Ok(()), // No manifest to update
    };

    // Filter assets to only those still referenced
    let kept_assets: Vec<_> = manifest
        .assets
        .into_iter()
        .filter(|a| {
            let filename = std::path::Path::new(&a.path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            referenced.contains(&a.path) || referenced.iter().any(|r| r.ends_with(filename))
        })
        .collect();

    // Create updated manifest
    let total_size: usize = kept_assets.iter().map(|a| a.size_bytes).sum();
    let updated = AssetManifest {
        version: manifest.version,
        created_at: chrono::Utc::now().to_rfc3339(),
        backend: manifest.backend,
        assets_dir: manifest.assets_dir,
        total_assets: kept_assets.len(),
        total_size_bytes: total_size,
        generator_version: Some(env!("CARGO_PKG_VERSION").to_string()),
        assets: kept_assets,
    };

    // Write updated manifest
    updated.write(std::path::Path::new(&manifest_path))?;
    println!(
        "  {} Updated manifest.json ({} assets)",
        "Info:".cyan(),
        updated.total_assets
    );

    Ok(())
}

fn build_multi_target(
    input: &std::path::Path,
    output_dir: &str,
    targets: Option<&str>,
    all_targets: bool,
    palette_path: Option<&std::path::Path>,
) -> Result<(), Error> {
    // Determine which targets to build
    let target_names: Vec<&str> = if all_targets {
        available_targets()
    } else if let Some(t) = targets {
        t.split(',').map(|s| s.trim()).collect()
    } else {
        // Default to common targets
        vec!["github", "pypi", "npm"]
    };

    // Validate targets
    for name in &target_names {
        if get_target(name).is_none() {
            return Err(Error::ParseError(format!(
                "Unknown target '{}'. Available: {}",
                name,
                available_targets().join(", ")
            )));
        }
    }

    // Read input file
    let content = fs::read_to_string(input).map_err(Error::IoError)?;

    // Load custom palette if provided
    let custom_palette: Option<std::collections::HashMap<String, String>> =
        if let Some(palette_file) = palette_path {
            let palette_content = fs::read_to_string(palette_file).map_err(Error::IoError)?;
            let palette: std::collections::HashMap<String, String> =
                serde_json::from_str(&palette_content).map_err(|e| {
                    Error::ParseError(format!(
                        "Failed to parse palette file '{}': {}",
                        palette_file.display(),
                        e
                    ))
                })?;
            eprintln!(
                "{} Loaded {} custom color(s) from {}",
                "Info:".cyan(),
                palette.len(),
                palette_file.display()
            );
            Some(palette)
        } else {
            None
        };

    // Create output directory
    fs::create_dir_all(output_dir).map_err(Error::IoError)?;

    // Get the input filename stem
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    println!("{}", "Building for multiple targets...".bold());
    println!();

    let mut success_count = 0;

    for target_name in &target_names {
        let target = get_target(target_name).unwrap();

        print!("  {} {} ", "Building:".cyan(), target_name);

        // Create backend based on target's preference
        let backend_type = target.preferred_backend();
        let assets_dir = format!("{}/assets/{}", output_dir, target_name);

        let mut parser = match backend_type {
            BackendType::Svg => {
                fs::create_dir_all(&assets_dir).map_err(Error::IoError)?;
                TemplateParser::with_backend(Box::new(SvgBackend::new(&assets_dir)))?
            }
            BackendType::Shields => TemplateParser::with_backend(Box::new(ShieldsBackend::new()?))?,
            BackendType::PlainText => {
                TemplateParser::with_backend(Box::new(PlainTextBackend::new()))?
            }
        };

        // Apply custom palette
        if let Some(ref palette) = custom_palette {
            parser.extend_palette(palette.clone());
        }

        // Process content
        let processed_result = parser.process_with_assets(&content)?;

        // Write any file-based assets (skip existing)
        if !processed_result.assets.is_empty() {
            for asset in &processed_result.assets {
                if let Some(path) = asset.file_path() {
                    if let Some(bytes) = asset.file_bytes() {
                        let path_ref = std::path::Path::new(path);
                        if !path_ref.exists() {
                            fs::write(path, bytes).map_err(Error::IoError)?;
                        }
                    }
                }
            }
        }

        // Apply target-specific post-processing
        let processed = target.post_process(&processed_result.markdown)?;

        // Write output file
        let output_path = format!("{}/{}_{}.md", output_dir, stem, target_name);
        fs::write(&output_path, &processed).map_err(Error::IoError)?;

        println!("{} {}", "→".green(), output_path);
        success_count += 1;
    }

    println!();
    println!(
        "{} Built {} target(s) successfully!",
        "✓".green().bold(),
        success_count
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn watch_file(
    input: PathBuf,
    output: PathBuf,
    target_name: &str,
    backend_override: Option<&str>,
    assets_dir: &str,
    palette_path: Option<&std::path::Path>,
    debounce_ms: u64,
    config_path: Option<&std::path::Path>,
) -> Result<(), Error> {
    // Validate input file exists
    if !input.exists() {
        return Err(Error::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input file not found: {}", input.display()),
        )));
    }

    println!("{}", "Watch mode".bold().cyan());
    println!("  Input:  {}", input.display().to_string().green());
    println!("  Output: {}", output.display().to_string().green());
    println!("  Target: {}", target_name.yellow());
    if let Some(backend) = backend_override {
        println!("  Backend: {}", backend.yellow());
    }
    println!();
    println!("{}", "Press Ctrl+C to stop watching".dimmed());
    println!();

    // Initial build
    println!("{} Initial build...", "[watch]".cyan());
    match process_file(
        Some(input.clone()),
        Some(output.clone()),
        false,
        target_name,
        backend_override,
        assets_dir,
        palette_path,
        config_path,
    ) {
        Ok(()) => println!("{} Build complete", "[watch]".green()),
        Err(e) => eprintln!("{} Build failed: {}", "[watch]".red(), e),
    }

    // Set up file watcher
    let (tx, rx) = channel();

    let config = Config::default().with_poll_interval(Duration::from_millis(debounce_ms));

    let mut watcher: RecommendedWatcher =
        Watcher::new(tx, config).map_err(|e| Error::ParseError(format!("Watch error: {}", e)))?;

    // Watch the input file's parent directory
    let watch_path = input.parent().unwrap_or(&input);
    watcher
        .watch(watch_path, RecursiveMode::NonRecursive)
        .map_err(|e| Error::ParseError(format!("Watch error: {}", e)))?;

    let input_filename = input.file_name();

    // Event loop
    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                // Check if the event is for our input file
                let is_our_file = event.paths.iter().any(|p| p.file_name() == input_filename);

                if is_our_file && event.kind.is_modify() {
                    println!();
                    println!("{} File changed, rebuilding...", "[watch]".cyan());

                    match process_file(
                        Some(input.clone()),
                        Some(output.clone()),
                        false,
                        target_name,
                        backend_override,
                        assets_dir,
                        palette_path,
                        config_path,
                    ) {
                        Ok(()) => println!("{} Build complete", "[watch]".green()),
                        Err(e) => eprintln!("{} Build failed: {}", "[watch]".red(), e),
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("{} Watch error: {}", "[watch]".red(), e);
            }
            Err(e) => {
                eprintln!("{} Channel error: {}", "[watch]".red(), e);
                break;
            }
        }
    }

    Ok(())
}

#[cfg(feature = "lsp")]
fn install_lsp_extension(editor: &str) -> Result<(), Error> {
    match editor.to_lowercase().as_str() {
        "vscode" | "code" => install_vscode_extension(),
        _ => Err(Error::ParseError(format!(
            "Unsupported editor '{}'. Currently supported: vscode",
            editor
        ))),
    }
}

#[cfg(feature = "lsp")]
fn install_vscode_extension() -> Result<(), Error> {
    use std::process::Command;

    println!("{}", "Installing mdfx LSP extension for VS Code...".bold());
    println!();

    // Find mdfx binary path
    let mdfx_path = std::env::current_exe()
        .map_err(Error::IoError)?
        .to_string_lossy()
        .to_string();

    // Determine VS Code extensions directory
    let extensions_dir = get_vscode_extensions_dir()?;
    let extension_path = extensions_dir.join("mdfx-lsp");

    println!("  {} {}", "Extension path:".cyan(), extension_path.display());
    println!("  {} {}", "mdfx binary:".cyan(), mdfx_path);
    println!();

    // Create extension directory
    fs::create_dir_all(&extension_path).map_err(Error::IoError)?;

    // Write package.json
    let package_json = generate_package_json(&mdfx_path);
    let package_path = extension_path.join("package.json");
    fs::write(&package_path, &package_json).map_err(Error::IoError)?;
    println!("  {} package.json", "Created:".green());

    // Write extension.js
    let extension_js = generate_extension_js(&mdfx_path);
    let extension_js_path = extension_path.join("extension.js");
    fs::write(&extension_js_path, &extension_js).map_err(Error::IoError)?;
    println!("  {} extension.js", "Created:".green());

    println!();
    println!("{} Installing npm dependencies...", "Info:".cyan());

    // Run npm install
    let npm_result = Command::new("npm")
        .arg("install")
        .arg("--production")
        .current_dir(&extension_path)
        .output();

    match npm_result {
        Ok(output) => {
            if output.status.success() {
                println!("  {} npm install", "Success:".green());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("  {} npm install failed: {}", "Warning:".yellow(), stderr);
                eprintln!();
                eprintln!("  You may need to run manually:");
                eprintln!("    cd {} && npm install", extension_path.display());
            }
        }
        Err(e) => {
            eprintln!("  {} Could not run npm: {}", "Warning:".yellow(), e);
            eprintln!();
            eprintln!("  Please install dependencies manually:");
            eprintln!("    cd {} && npm install", extension_path.display());
        }
    }

    println!();
    println!("{}", "✓ VS Code extension installed!".green().bold());
    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Reload VS Code (Cmd+Shift+P → 'Developer: Reload Window')");
    println!("  2. Open a .md file and type {{ui:tech: to see completions");
    println!();
    println!(
        "{}",
        "The extension will activate automatically for markdown files.".dimmed()
    );

    Ok(())
}

#[cfg(feature = "lsp")]
fn get_vscode_extensions_dir() -> Result<PathBuf, Error> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| {
            Error::ParseError("Could not determine home directory".to_string())
        })?;

    let home_path = PathBuf::from(home);

    // Try different VS Code extension locations
    let candidates = [
        home_path.join(".vscode/extensions"),           // Standard VS Code
        home_path.join(".vscode-server/extensions"),    // VS Code Remote
        home_path.join(".vscode-insiders/extensions"),  // VS Code Insiders
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    // Default to standard location (will be created)
    Ok(candidates[0].clone())
}

#[cfg(feature = "lsp")]
fn generate_package_json(mdfx_path: &str) -> String {
    format!(
        r#"{{
  "name": "mdfx-lsp",
  "displayName": "mdfx Language Server",
  "description": "LSP support for mdfx markdown template syntax",
  "version": "{}",
  "publisher": "mdfx",
  "engines": {{
    "vscode": "^1.75.0"
  }},
  "categories": [
    "Programming Languages",
    "Linters"
  ],
  "activationEvents": [
    "onLanguage:markdown"
  ],
  "main": "./extension.js",
  "contributes": {{
    "configuration": {{
      "type": "object",
      "title": "mdfx",
      "properties": {{
        "mdfx.path": {{
          "type": "string",
          "default": "{}",
          "description": "Path to mdfx executable"
        }},
        "mdfx.trace.server": {{
          "type": "string",
          "enum": ["off", "messages", "verbose"],
          "default": "off",
          "description": "Traces the communication between VS Code and the mdfx language server"
        }}
      }}
    }}
  }},
  "dependencies": {{
    "vscode-languageclient": "^9.0.1"
  }}
}}
"#,
        env!("CARGO_PKG_VERSION"),
        mdfx_path.replace('\\', "\\\\").replace('"', "\\\"")
    )
}

#[cfg(feature = "lsp")]
fn generate_extension_js(mdfx_path: &str) -> String {
    format!(
        r#"const vscode = require('vscode');
const {{ LanguageClient, TransportKind }} = require('vscode-languageclient/node');

let client;

function activate(context) {{
    const config = vscode.workspace.getConfiguration('mdfx');
    const mdfxPath = config.get('path', '{}');

    const serverOptions = {{
        command: mdfxPath,
        args: ['lsp', 'run'],
        transport: TransportKind.stdio
    }};

    const clientOptions = {{
        documentSelector: [
            {{ scheme: 'file', language: 'markdown' }},
            {{ scheme: 'untitled', language: 'markdown' }}
        ],
        synchronize: {{
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.md')
        }}
    }};

    client = new LanguageClient(
        'mdfx',
        'mdfx Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
    console.log('mdfx LSP client started');
}}

function deactivate() {{
    if (client) {{
        return client.stop();
    }}
}}

module.exports = {{ activate, deactivate }};
"#,
        mdfx_path.replace('\\', "\\\\").replace('\'', "\\'")
    )
}
