use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process;
use utf8fx::{Converter, Error, StyleCategory};

/// Unicode text effects for markdown and beyond
#[derive(Parser)]
#[command(name = "utf8fx")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert text to a Unicode style
    Convert {
        /// The style to use (e.g., mathbold, fullwidth, mb)
        #[arg(short, long)]
        style: String,

        /// The text to convert
        text: String,
    },

    /// List available styles
    List {
        /// Show only styles in a specific category
        #[arg(short, long)]
        category: Option<String>,

        /// Show sample output for each style
        #[arg(short, long)]
        samples: bool,
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
        Commands::Convert { style, text } => {
            let result = converter.convert(&text, &style)?;
            println!("{}", result);
        }

        Commands::List { category, samples } => {
            list_styles(&converter, category, samples)?;
        }
    }

    Ok(())
}

fn list_styles(converter: &Converter, category: Option<String>, show_samples: bool) -> Result<(), Error> {
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
