use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use colored::Colorize;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use utf8fx::{Converter, Error, StyleCategory, TemplateParser};
use utf8fx::renderer::shields::ShieldsBackend;
use utf8fx::renderer::svg::SvgBackend;

/// Unicode text effects for markdown and beyond
#[derive(Parser)]
#[command(name = "utf8fx")]
#[command(version, about)]
#[command(
    long_about = "Transform text into various Unicode styles through template syntax or direct conversion.\n\nSupports 19 styles including mathbold, fullwidth, script, fraktur, and more.\nUse templates in markdown: {{mathbold}}TEXT{{/mathbold}}\n\nFor more info: https://github.com/blackwell-systems/utf8fx"
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
    ///   utf8fx convert --style mathbold "Hello World"
    ///   utf8fx convert --style mb --spacing 1 "SPACED"
    ///   utf8fx convert --style script "Elegant Text"
    ///
    /// Run 'utf8fx list' to see all available styles.
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
    ///   utf8fx list
    ///   utf8fx list --samples
    ///   utf8fx list --category bold
    List {
        /// Show only styles in a specific category
        #[arg(short, long)]
        category: Option<String>,

        /// Show sample output for each style
        #[arg(short, long)]
        samples: bool,
    },

    /// Process markdown file with style templates
    ///
    /// Transform markdown files by processing style templates in the format:
    /// {{style}}text{{/style}} or {{style:spacing=N}}text{{/style}}
    ///
    /// Templates are preserved inside code blocks (```) and inline code (`).
    ///
    /// Examples:
    ///   utf8fx process input.md -o output.md
    ///   utf8fx process -i README.md
    ///   utf8fx process --backend shields input.md
    ///   echo "{{mathbold}}Title{{/mathbold}}" | utf8fx process
    ///   cat doc.md | utf8fx process > styled.md
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

        /// Rendering backend for UI components (shields = shields.io URLs, svg = local SVG files)
        #[arg(short, long, default_value = "shields")]
        backend: String,

        /// Output directory for SVG assets (only used with --backend svg)
        #[arg(long, default_value = "assets/utf8fx")]
        assets_dir: String,
    },

    /// Generate shell completions
    ///
    /// Generate tab completion scripts for your shell. Save the output to
    /// your shell's completion directory.
    ///
    /// Examples:
    ///   utf8fx completions bash > /etc/bash_completion.d/utf8fx
    ///   utf8fx completions zsh > ~/.zsh/completions/_utf8fx
    ///   utf8fx completions fish > ~/.config/fish/completions/utf8fx.fish
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
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

        Commands::Process {
            input,
            output,
            in_place,
            backend,
            assets_dir,
        } => {
            process_file(input, output, in_place, &backend, &assets_dir)?;
        }

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "utf8fx", &mut io::stdout());
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

fn process_file(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    in_place: bool,
    backend: &str,
    assets_dir: &str,
) -> Result<(), Error> {
    // Create the appropriate backend
    let parser = match backend {
        "shields" => {
            // Default: shields.io URLs (no files)
            TemplateParser::with_backend(Box::new(ShieldsBackend::new()?))?
        }
        "svg" => {
            // SVG backend: generates local files
            TemplateParser::with_backend(Box::new(SvgBackend::new(assets_dir)))?
        }
        _ => {
            return Err(Error::ParseError(format!(
                "Unknown backend '{}'. Available: shields, svg",
                backend
            )));
        }
    };

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

        for asset in &processed_result.assets {
            if let Some(path) = asset.file_path() {
                if let Some(bytes) = asset.file_bytes() {
                    // Write the asset file
                    fs::write(path, bytes).map_err(Error::IoError)?;
                    eprintln!("  {} {}", "Wrote:".green(), path);
                }
            }
        }
    }

    let processed = processed_result.markdown;

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
