use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::Colorize;
use mdfx::manifest::AssetManifest;
use mdfx::renderer::plaintext::PlainTextBackend;
use mdfx::renderer::shields::ShieldsBackend;
use mdfx::renderer::svg::SvgBackend;
use mdfx::{
    available_targets, detect_target_from_path, get_target, BackendType, Converter, Error,
    SeparatorsData, StyleCategory, Target, TemplateParser,
};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

/// Markdown effects: Unicode text styling and UI components
#[derive(Parser)]
#[command(name = "mdfx")]
#[command(version, about)]
#[command(
    long_about = "Transform markdown with Unicode text effects and UI components through template syntax.\n\nSupports 19 styles including mathbold, fullwidth, script, fraktur, and more.\nUse templates: {{mathbold}}TEXT{{/mathbold}}\n\nFor more info: https://github.com/blackwell-systems/mdfx"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert text to a Unicode style
    ///
    /// Transform plain text into styled Unicode characters using one of 19 available styles.
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

    /// List available styles
    ///
    /// Display all 19 Unicode styles organized by category: Bold & Impactful,
    /// Boxed, Technical & Code, and Subtle & Elegant. Each style includes
    /// its ID, aliases, and description.
    ///
    /// Examples:
    ///   mdfx list
    ///   mdfx list --samples
    ///   mdfx list --category bold
    List {
        /// Show only styles in a specific category
        #[arg(short, long)]
        category: Option<String>,

        /// Show sample output for each style
        #[arg(short, long)]
        samples: bool,
    },

    /// List available separators
    ///
    /// Display all available separator characters that can be used with the
    /// separator parameter. Includes both named separators (dot, bullet, arrow)
    /// and examples. You can also use any single Unicode character directly.
    ///
    /// Examples:
    ///   mdfx separators
    ///   mdfx separators --examples
    Separators {
        /// Show usage examples for each separator
        #[arg(short, long)]
        examples: bool,
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
    ///   {{ui:divider/}}
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
    /// Remove asset files that are not referenced in manifest.json.
    /// Useful for cleaning up old assets after refactoring.
    ///
    /// Examples:
    ///   mdfx clean --assets-dir assets/mdfx
    ///   mdfx clean --dry-run  # Show what would be deleted
    Clean {
        /// Assets directory containing manifest.json
        #[arg(long, default_value = "assets/mdfx")]
        assets_dir: String,

        /// Show what would be deleted without actually deleting
        #[arg(long)]
        dry_run: bool,
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

        Commands::List { category, samples } => {
            list_styles(&converter, category, samples)?;
        }

        Commands::Separators { examples } => {
            list_separators(examples)?;
        }

        Commands::Process {
            input,
            output,
            in_place,
            target,
            backend,
            assets_dir,
            palette,
        } => {
            process_file(
                input,
                output,
                in_place,
                &target,
                backend.as_deref(),
                &assets_dir,
                palette.as_deref(),
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
        } => {
            clean_assets(&assets_dir, dry_run)?;
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

fn list_separators(show_examples: bool) -> Result<(), Error> {
    let separators_data = SeparatorsData::load()?;

    println!("{}", "Available separators:".bold());
    println!();
    println!(
        "{}",
        "Use with: {{mathbold:separator=NAME}}TEXT{{/mathbold}}".dimmed()
    );
    println!(
        "{}",
        "Or use any single Unicode character directly: {{mathbold:separator=⚡}}TEXT{{/mathbold}}"
            .dimmed()
    );
    println!();

    for sep in &separators_data.separators {
        // Show ID and character
        print!("  {} ", sep.id.green());
        print!("({}) ", sep.char.cyan().bold());

        // Show Unicode code point
        print!("[{}] ", sep.unicode.dimmed());

        // Show description
        println!("- {}", sep.description.dimmed());

        // Show example if requested
        if show_examples {
            let example = format!("A {} B", sep.char);
            println!("    Example: {}", example.yellow());
            println!(
                "    {}",
                format!("{{{{mathbold:separator={}}}}}TEXT{{{{/mathbold}}}}", sep.id).dimmed()
            );
        }
    }

    println!();
    println!("{}", "💡 Tip:".bold());
    println!(
        "  {}",
        "Any single Unicode character works as a separator:".dimmed()
    );
    println!(
        "  {}",
        "{{mathbold:separator=⚡}}LIGHTNING{{/mathbold}}".dimmed()
    );
    println!(
        "  {}",
        "{{mathbold:separator=★}}STARS{{/mathbold}}".dimmed()
    );
    println!(
        "  {}",
        "{{mathbold:separator=|}}PIPES{{/mathbold}}".dimmed()
    );

    Ok(())
}

fn process_file(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    in_place: bool,
    target_name: &str,
    backend_override: Option<&str>,
    assets_dir: &str,
    palette_path: Option<&std::path::Path>,
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
                    "Unknown backend '{}'. Available: shields, svg",
                    backend
                )));
            }
        }
    } else {
        target.preferred_backend()
    };

    // Create the appropriate backend
    let mut parser = match backend_type {
        BackendType::Shields => TemplateParser::with_backend(Box::new(ShieldsBackend::new()?))?,
        BackendType::Svg => TemplateParser::with_backend(Box::new(SvgBackend::new(assets_dir)))?,
        BackendType::PlainText => TemplateParser::with_backend(Box::new(PlainTextBackend::new()))?,
    };

    // Load custom palette if provided
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

        eprintln!(
            "{} Writing {} asset(s) to {}",
            "Info:".cyan(),
            processed_result.assets.len(),
            assets_dir
        );

        // Build manifest for SVG backend
        let mut manifest = if matches!(backend_type, BackendType::Svg) {
            Some(AssetManifest::new("svg", assets_dir))
        } else {
            None
        };

        for asset in &processed_result.assets {
            if let Some(path) = asset.file_path() {
                if let Some(bytes) = asset.file_bytes() {
                    // Write the asset file
                    fs::write(path, bytes).map_err(Error::IoError)?;
                    eprintln!("  {} {}", "Wrote:".green(), path);

                    // Add to manifest if SVG backend
                    if let Some(ref mut m) = manifest {
                        if let mdfx::RenderedAsset::File {
                            relative_path,
                            bytes,
                            primitive,
                            ..
                        } = asset
                        {
                            let asset_type = match primitive {
                                mdfx::Primitive::Swatch { .. } => "swatch",
                                mdfx::Primitive::Divider { .. } => "divider",
                                mdfx::Primitive::Tech { .. } => "tech",
                                mdfx::Primitive::Status { .. } => "status",
                            };
                            m.add_asset(
                                relative_path.clone(),
                                bytes,
                                primitive,
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
            eprintln!("  {} {}", "Wrote:".green(), manifest_path);
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

fn clean_assets(assets_dir: &str, dry_run: bool) -> Result<(), Error> {
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

    // Load manifest
    let manifest = match AssetManifest::load(std::path::Path::new(&manifest_path)) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("{} manifest.json not found", "Error:".red().bold());
            eprintln!("Run with --backend svg to generate a manifest.");
            process::exit(1);
        }
    };

    // Get referenced asset paths
    let referenced: std::collections::HashSet<String> = manifest
        .asset_paths()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    // Find all SVG files in assets directory
    let assets_path = std::path::Path::new(assets_dir);
    if !assets_path.exists() {
        println!("{}", "No assets directory found.".yellow());
        return Ok(());
    }

    let mut deleted_count = 0;
    let mut total_bytes = 0;

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

        // Check if this asset is in the manifest
        if !referenced.contains(&relative_path) {
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
        }
    }

    println!();
    if deleted_count == 0 {
        println!("{}", "✓ No unreferenced assets found.".green());
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
    }

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
            BackendType::Shields => TemplateParser::with_backend(Box::new(ShieldsBackend::new()?))?,
            BackendType::Svg => {
                fs::create_dir_all(&assets_dir).map_err(Error::IoError)?;
                TemplateParser::with_backend(Box::new(SvgBackend::new(&assets_dir)))?
            }
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

        // Write any file-based assets
        if !processed_result.assets.is_empty() {
            for asset in &processed_result.assets {
                if let Some(path) = asset.file_path() {
                    if let Some(bytes) = asset.file_bytes() {
                        fs::write(path, bytes).map_err(Error::IoError)?;
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
