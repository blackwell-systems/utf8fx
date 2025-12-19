//! Parity tests comparing badgefx output with original mdfx svg/tech.rs renderer
//!
//! These tests ensure that badgefx produces identical output to the original mdfx
//! tech badge renderer, maintaining pixel-perfect parity.
//!
//! Test cases are derived from the Neon Tech Badge Showcase examples.

use badgefx::{badge, BadgeStyle, Chevron};

/// Helper to generate a badge using mdfx's tech renderer with all options
fn mdfx_render_full(
    name: &str,
    label: Option<&str>,
    bg_color: &str,
    logo_color: &str,
    style: &str,
    border_color: Option<&str>,
    border_width: Option<u32>,
    text_color: Option<&str>,
    chevron: Option<&str>,
    bg_left: Option<&str>,
    bg_right: Option<&str>,
) -> String {
    mdfx::renderer::svg::tech::render_with_options(
        name,
        label,
        bg_color,
        logo_color,
        style,
        border_color,
        border_width,
        false, // border_full
        None,  // rx
        None,  // corners
        text_color,
        None, // font
        chevron,
        bg_left,
        bg_right,
        None, // raised
        None, // logo_size
    )
}

/// Simple helper for basic badges
fn mdfx_render_tech(
    name: &str,
    label: Option<&str>,
    bg_color: &str,
    logo_color: &str,
    style: &str,
) -> String {
    mdfx_render_full(
        name, label, bg_color, logo_color, style, None, None, None, None, None, None,
    )
}

// =============================================================================
// BASIC BADGE TESTS
// =============================================================================

#[test]
fn test_parity_basic_rust_badge() {
    let name = "rust";
    let label = "rust";
    let bg_color = "DEA584";
    let logo_color = "000000";

    let mdfx_svg = mdfx_render_tech(name, Some(label), bg_color, logo_color, "flat");
    let badgefx_svg = badge(name)
        .label(label)
        .bg_color("#DEA584")
        .logo_color("#000000")
        .style(BadgeStyle::Flat)
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Rust badge should match");
}

#[test]
fn test_parity_typescript_badge() {
    let name = "typescript";
    let label = "typescript";
    let bg_color = "3178C6";
    let logo_color = "FFFFFF";

    let mdfx_svg = mdfx_render_tech(name, Some(label), bg_color, logo_color, "flat");
    let badgefx_svg = badge(name)
        .label(label)
        .bg_color("#3178C6")
        .logo_color("#FFFFFF")
        .style(BadgeStyle::Flat)
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "TypeScript badge should match");
}

#[test]
fn test_parity_icon_only() {
    let name = "docker";
    let bg_color = "2496ED";
    let logo_color = "FFFFFF";

    let mdfx_svg = mdfx_render_tech(name, None, bg_color, logo_color, "flat");
    let badgefx_svg = badge(name)
        .label("")
        .bg_color("#2496ED")
        .logo_color("#FFFFFF")
        .style(BadgeStyle::Flat)
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Docker icon-only badge should match");
}

#[test]
fn test_parity_text_only() {
    let name = "unknown-tech";
    let label = "unknown-tech";
    let bg_color = "555555";
    let logo_color = "FFFFFF";

    let mdfx_svg = mdfx_render_tech(name, Some(label), bg_color, logo_color, "flat");
    let badgefx_svg = badge(name)
        .label(label)
        .bg_color("#555555")
        .logo_color("#FFFFFF")
        .style(BadgeStyle::Flat)
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Unknown tech text-only badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: CYBER STACK (bg + bg_right + border)
// =============================================================================

#[test]
fn test_neon_cyber_stack_rust() {
    // {{ui:tech:rust:bg=0D0D0D:bg_right=1a0a0a:border=FF6B6B:border_width=1/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "0D0D0D",
        "FFFFFF",
        "flat",
        Some("FF6B6B"),
        Some(1),
        None,
        None,
        None,
        Some("1a0a0a"),
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#0D0D0D")
        .bg_right("#1a0a0a")
        .border("#FF6B6B", 1)
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Cyber Stack Rust badge should match");
}

#[test]
fn test_neon_cyber_stack_typescript() {
    // {{ui:tech:typescript:bg=0D0D0D:bg_right=0a0a1a:border=00D9FF:border_width=1/}}
    let mdfx_svg = mdfx_render_full(
        "typescript",
        Some("typescript"),
        "0D0D0D",
        "FFFFFF",
        "flat",
        Some("00D9FF"),
        Some(1),
        None,
        None,
        None,
        Some("0a0a1a"),
    );

    let badgefx_svg = badge("typescript")
        .label("typescript")
        .bg_color("#0D0D0D")
        .bg_right("#0a0a1a")
        .border("#00D9FF", 1)
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Cyber Stack TypeScript badge should match"
    );
}

#[test]
fn test_neon_cyber_stack_docker() {
    // {{ui:tech:docker:bg=0D0D0D:bg_right=0a1a1a:border=00FF88:border_width=1/}}
    let mdfx_svg = mdfx_render_full(
        "docker",
        Some("docker"),
        "0D0D0D",
        "FFFFFF",
        "flat",
        Some("00FF88"),
        Some(1),
        None,
        None,
        None,
        Some("0a1a1a"),
    );

    let badgefx_svg = badge("docker")
        .label("docker")
        .bg_color("#0D0D0D")
        .bg_right("#0a1a1a")
        .border("#00FF88", 1)
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Cyber Stack Docker badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: GHOST PROTOCOL (outline style)
// =============================================================================

#[test]
fn test_neon_ghost_protocol_rust() {
    // {{ui:tech:rust:style=outline/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "DEA584", // brand color used as outline color
        "DEA584",
        "outline",
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#DEA584")
        .outline()
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Ghost Protocol Rust badge should match"
    );
}

#[test]
fn test_neon_ghost_protocol_python() {
    // {{ui:tech:python:style=outline/}}
    let mdfx_svg = mdfx_render_full(
        "python",
        Some("python"),
        "3776AB",
        "3776AB",
        "outline",
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let badgefx_svg = badge("python")
        .label("python")
        .bg_color("#3776AB")
        .outline()
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Ghost Protocol Python badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: HOLOGRAPHIC GRADIENTS (bg_left + bg_right)
// =============================================================================

#[test]
fn test_neon_holographic_rust() {
    // {{ui:tech:rust:bg_left=FF6B6B:bg_right=4ECDC4/}}
    // When bg not specified, uses brand color (DEA584 for rust -> black logo)
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "DEA584", // Brand color
        "000000", // Auto-calculated from brand color
        "flat",
        None,
        None,
        None,
        None,
        Some("FF6B6B"),
        Some("4ECDC4"),
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#DEA584")
        .logo_color("#000000")
        .bg_left("#FF6B6B")
        .bg_right("#4ECDC4")
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Holographic Rust badge should match");
}

#[test]
fn test_neon_holographic_python() {
    // {{ui:tech:python:bg_left=3776AB:bg_right=FFD43B/}}
    let mdfx_svg = mdfx_render_full(
        "python",
        Some("python"),
        "3776AB",
        "FFFFFF",
        "flat",
        None,
        None,
        None,
        None,
        Some("3776AB"),
        Some("FFD43B"),
    );

    let badgefx_svg = badge("python")
        .label("python")
        .bg_color("#3776AB")
        .bg_left("#3776AB")
        .bg_right("#FFD43B")
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Holographic Python badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: TERMINAL GREEN (outline + custom colors)
// =============================================================================

#[test]
fn test_neon_terminal_green_go() {
    // {{ui:tech:go:style=outline:border=00FF00:text_color=00FF00/}}
    let mdfx_svg = mdfx_render_full(
        "go",
        Some("go"),
        "00ADD8", // brand color
        "00ADD8",
        "outline",
        Some("00FF00"),
        None,
        Some("00FF00"),
        None,
        None,
        None,
    );

    let badgefx_svg = badge("go")
        .label("go")
        .bg_color("#00ADD8")
        .outline()
        .border("#00FF00", 2)
        .text_color("#00FF00")
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Terminal Green Go badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: SYNTHWAVE DREAMS
// =============================================================================

#[test]
fn test_neon_synthwave_rust() {
    // {{ui:tech:rust:bg=1a1a2e:bg_right=16213e:border=FF00FF:border_width=1/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "1a1a2e",
        "FFFFFF",
        "flat",
        Some("FF00FF"),
        Some(1),
        None,
        None,
        None,
        Some("16213e"),
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#1a1a2e")
        .bg_right("#16213e")
        .border("#FF00FF", 1)
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Synthwave Rust badge should match");
}

// =============================================================================
// NEON TECH SHOWCASE: CHEVRON FLOW
// =============================================================================

#[test]
fn test_neon_chevron_rust_right() {
    // {{ui:tech:rust:chevron=right:bg=1a1a2e:bg_right=2a2a3e/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "1a1a2e",
        "FFFFFF",
        "flat",
        None,
        None,
        None,
        Some("right"),
        None,
        Some("2a2a3e"),
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#1a1a2e")
        .bg_right("#2a2a3e")
        .chevron(Chevron::right(10.0))
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Chevron Right Rust badge should match"
    );
}

#[test]
fn test_neon_chevron_typescript_both() {
    // {{ui:tech:typescript:chevron=both:bg=2a2a3e:bg_right=3a3a4e/}}
    let mdfx_svg = mdfx_render_full(
        "typescript",
        Some("typescript"),
        "2a2a3e",
        "FFFFFF",
        "flat",
        None,
        None,
        None,
        Some("both"),
        None,
        Some("3a3a4e"),
    );

    let badgefx_svg = badge("typescript")
        .label("typescript")
        .bg_color("#2a2a3e")
        .bg_right("#3a3a4e")
        .chevron(Chevron::both(10.0))
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Chevron Both TypeScript badge should match"
    );
}

#[test]
fn test_neon_chevron_kubernetes_left() {
    // {{ui:tech:kubernetes:chevron=left:bg=4a4a5e:bg_right=5a5a6e/}}
    let mdfx_svg = mdfx_render_full(
        "kubernetes",
        Some("kubernetes"),
        "4a4a5e",
        "FFFFFF",
        "flat",
        None,
        None,
        None,
        Some("left"),
        None,
        Some("5a5a6e"),
    );

    let badgefx_svg = badge("kubernetes")
        .label("kubernetes")
        .bg_color("#4a4a5e")
        .bg_right("#5a5a6e")
        .chevron(Chevron::left(10.0))
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Chevron Left Kubernetes badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: ICE & FIRE
// =============================================================================

#[test]
fn test_neon_ice_fire_typescript() {
    // {{ui:tech:typescript:bg_left=00D9FF:bg_right=0099CC/}}
    // When bg not specified, uses brand color (3178C6 for typescript -> white logo)
    let mdfx_svg = mdfx_render_full(
        "typescript",
        Some("typescript"),
        "3178C6", // Brand color
        "FFFFFF", // Auto-calculated from brand color
        "flat",
        None,
        None,
        None,
        None,
        Some("00D9FF"),
        Some("0099CC"),
    );

    let badgefx_svg = badge("typescript")
        .label("typescript")
        .bg_color("#3178C6")
        .logo_color("#FFFFFF")
        .bg_left("#00D9FF")
        .bg_right("#0099CC")
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Ice & Fire TypeScript badge should match"
    );
}

#[test]
fn test_neon_ice_fire_rust() {
    // {{ui:tech:rust:bg_left=FF4500:bg_right=DC143C/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "FF4500",
        "FFFFFF",
        "flat",
        None,
        None,
        None,
        None,
        Some("FF4500"),
        Some("DC143C"),
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#FF4500")
        .bg_left("#FF4500")
        .bg_right("#DC143C")
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Ice & Fire Rust badge should match");
}

// =============================================================================
// NEON TECH SHOWCASE: MATRIX CODE
// =============================================================================

#[test]
fn test_neon_matrix_go() {
    // {{ui:tech:go:bg=000000:bg_right=001100:border=00FF00:border_width=1:text_color=00FF00/}}
    let mdfx_svg = mdfx_render_full(
        "go",
        Some("go"),
        "000000",
        "00FF00", // Using green logo for matrix theme
        "flat",
        Some("00FF00"),
        Some(1),
        Some("00FF00"),
        None,
        None,
        Some("001100"),
    );

    let badgefx_svg = badge("go")
        .label("go")
        .bg_color("#000000")
        .bg_right("#001100")
        .logo_color("#00FF00")
        .border("#00FF00", 1)
        .text_color("#00FF00")
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Matrix Code Go badge should match");
}

// =============================================================================
// NEON TECH SHOWCASE: VAPOR WAVE
// =============================================================================

#[test]
fn test_neon_vapor_wave_react() {
    // {{ui:tech:react:bg_left=FF71CE:bg_right=01CDFE/}}
    // When bg is not specified, uses brand color (61DAFB for react)
    // Logo color is calculated from brand color (61DAFB is light -> black logo)
    let mdfx_svg = mdfx_render_full(
        "react",
        Some("react"),
        "61DAFB", // Brand color, not bg_left
        "000000", // Auto-calculated from brand color (light -> black)
        "flat",
        None,
        None,
        None,
        None,
        Some("FF71CE"),
        Some("01CDFE"),
    );

    let badgefx_svg = badge("react")
        .label("react")
        .bg_color("#61DAFB")
        .logo_color("#000000")
        .bg_left("#FF71CE")
        .bg_right("#01CDFE")
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Vapor Wave React badge should match");
}

// =============================================================================
// NEON TECH SHOWCASE: STEALTH MODE
// =============================================================================

#[test]
fn test_neon_stealth_kubernetes() {
    // {{ui:tech:kubernetes:bg=080808:bg_right=0f0f0f:border=333333:border_width=1/}}
    let mdfx_svg = mdfx_render_full(
        "kubernetes",
        Some("kubernetes"),
        "080808",
        "FFFFFF",
        "flat",
        Some("333333"),
        Some(1),
        None,
        None,
        None,
        Some("0f0f0f"),
    );

    let badgefx_svg = badge("kubernetes")
        .label("kubernetes")
        .bg_color("#080808")
        .bg_right("#0f0f0f")
        .border("#333333", 1)
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Stealth Mode Kubernetes badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: OUTLINE RAINBOW
// =============================================================================

#[test]
fn test_neon_outline_rainbow_rust() {
    // {{ui:tech:rust:style=outline:border=FF0000/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "DEA584",
        "DEA584",
        "outline",
        Some("FF0000"),
        None,
        None,
        None,
        None,
        None,
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#DEA584")
        .outline()
        .border("#FF0000", 2)
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Outline Rainbow Rust badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: CYBERPUNK 2077
// =============================================================================

#[test]
fn test_neon_cyberpunk_rust() {
    // {{ui:tech:rust:bg=FCE300:bg_right=00F0FF:text_color=000000/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "FCE300",
        "000000", // dark logo on bright bg
        "flat",
        None,
        None,
        Some("000000"),
        None,
        None,
        Some("00F0FF"),
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#FCE300")
        .bg_right("#00F0FF")
        .logo_color("#000000")
        .text_color("#000000")
        .render();

    assert_eq!(mdfx_svg, badgefx_svg, "Cyberpunk Rust badge should match");
}

// =============================================================================
// NEON TECH SHOWCASE: MIDNIGHT OIL
// =============================================================================

#[test]
fn test_neon_midnight_rust() {
    // {{ui:tech:rust:style=outline:border=DEA584:border_width=3/}}
    let mdfx_svg = mdfx_render_full(
        "rust",
        Some("rust"),
        "DEA584",
        "DEA584",
        "outline",
        Some("DEA584"),
        Some(3),
        None,
        None,
        None,
        None,
    );

    let badgefx_svg = badge("rust")
        .label("rust")
        .bg_color("#DEA584")
        .outline()
        .border("#DEA584", 3)
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Midnight Oil Rust badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: AURORA BOREALIS
// =============================================================================

#[test]
fn test_neon_aurora_postgresql() {
    // {{ui:tech:postgresql:bg_left=00FF87:bg_right=60EFFF/}}
    let mdfx_svg = mdfx_render_full(
        "postgresql",
        Some("postgresql"),
        "00FF87",
        "000000", // dark logo on bright bg
        "flat",
        None,
        None,
        None,
        None,
        Some("00FF87"),
        Some("60EFFF"),
    );

    let badgefx_svg = badge("postgresql")
        .label("postgresql")
        .bg_color("#00FF87")
        .logo_color("#000000")
        .bg_left("#00FF87")
        .bg_right("#60EFFF")
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Aurora Borealis PostgreSQL badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: ELECTRIC DREAMS
// =============================================================================

#[test]
fn test_neon_electric_javascript() {
    // {{ui:tech:javascript:bg=0D0D0D:bg_right=1a1a0a:border=F7DF1E:border_width=2/}}
    let mdfx_svg = mdfx_render_full(
        "javascript",
        Some("javascript"),
        "0D0D0D",
        "FFFFFF",
        "flat",
        Some("F7DF1E"),
        Some(2),
        None,
        None,
        None,
        Some("1a1a0a"),
    );

    let badgefx_svg = badge("javascript")
        .label("javascript")
        .bg_color("#0D0D0D")
        .bg_right("#1a1a0a")
        .border("#F7DF1E", 2)
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Electric Dreams JavaScript badge should match"
    );
}

// =============================================================================
// NEON TECH SHOWCASE: INFRASTRUCTURE CHEVRONS
// =============================================================================

#[test]
fn test_neon_infra_docker_chevron() {
    // {{ui:tech:docker:chevron=right:bg_left=2496ED:bg_right=1E7DC9/}}
    // Docker brand color is 2496ED, logo color is white on dark blue
    let mdfx_svg = mdfx_render_full(
        "docker",
        Some("docker"),
        "2496ED",
        "FFFFFF", // White logo on dark blue
        "flat",
        None,
        None,
        None,
        Some("right"),
        Some("2496ED"),
        Some("1E7DC9"),
    );

    let badgefx_svg = badge("docker")
        .label("docker")
        .bg_color("#2496ED")
        .logo_color("#FFFFFF") // Explicitly match mdfx
        .bg_left("#2496ED")
        .bg_right("#1E7DC9")
        .chevron(Chevron::right(10.0))
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Infrastructure Docker chevron badge should match"
    );
}

#[test]
fn test_neon_infra_terraform_chevron() {
    // {{ui:tech:terraform:chevron=left:bg_left=844FBA:bg_right=6D3F9A/}}
    let mdfx_svg = mdfx_render_full(
        "terraform",
        Some("terraform"),
        "844FBA",
        "FFFFFF",
        "flat",
        None,
        None,
        None,
        Some("left"),
        Some("844FBA"),
        Some("6D3F9A"),
    );

    let badgefx_svg = badge("terraform")
        .label("terraform")
        .bg_color("#844FBA")
        .bg_left("#844FBA")
        .bg_right("#6D3F9A")
        .chevron(Chevron::left(10.0))
        .render();

    assert_eq!(
        mdfx_svg, badgefx_svg,
        "Infrastructure Terraform chevron badge should match"
    );
}

// =============================================================================
// UTILITY TESTS
// =============================================================================

#[test]
fn test_darken_color_parity() {
    let colors = ["DEA584", "3178C6", "3776AB", "2496ED", "FFFFFF", "000000"];

    for color in colors {
        let darkened = mdfx_colors::darken(&format!("#{}", color), 0.15);
        assert!(
            !darkened.is_empty(),
            "Darkened color should not be empty for {}",
            color
        );
    }
}

#[test]
fn test_contrast_color_parity() {
    let test_cases = [
        ("FFFFFF", "000000"),
        ("000000", "FFFFFF"),
        ("3178C6", "FFFFFF"),
        ("F7DF1E", "000000"),
        ("DEA584", "000000"),
    ];

    for (bg, expected) in test_cases {
        let result = mdfx_colors::contrast_color(&format!("#{}", bg));
        let result = result.trim_start_matches('#');
        assert_eq!(
            result, expected,
            "Contrast color for {} should be {}",
            bg, expected
        );
    }
}
