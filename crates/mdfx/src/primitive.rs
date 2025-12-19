/// Rendering-neutral primitives for image-based visual elements.
///
/// These primitives represent the SEMANTIC intent of UI components
/// without committing to a specific rendering backend (shields.io, SVG, etc.).
///
/// Each primitive corresponds to a high-level UI concept:
/// - Swatch: Single colored block
/// - Tech: Technology logo badge
/// - Progress: Progress bar with customizable track and fill
/// - Donut: Circular progress/ring chart
/// - Gauge: Semi-circular meter (half-donut)
/// - Sparkline: Mini inline chart for data visualization
/// - Rating: Star/heart rating display with partial fills
/// - Waveform: Audio-style visualization with bars above/below center
///
/// Text-based transformations (frames, styles, badges) remain as direct
/// Unicode rendering and don't use this abstraction.

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Primitive {
    /// Single colored swatch block with optional enhancements
    Swatch {
        color: String,
        style: String,
        /// Opacity (0.0 = transparent, 1.0 = opaque). SVG-only.
        opacity: Option<f32>,
        /// Custom width in pixels (default: 20)
        width: Option<u32>,
        /// Custom height in pixels (default: style-dependent, usually 20)
        height: Option<u32>,
        /// Border color (hex or palette name). SVG-only.
        border_color: Option<String>,
        /// Border width in pixels (default: 0 = no border). SVG-only.
        border_width: Option<u32>,
        /// Text label inside swatch.
        label: Option<String>,
        /// Label text color (hex or palette name). Default: white.
        label_color: Option<String>,
        /// Simple Icons logo name (e.g., "rust", "python", "docker").
        icon: Option<String>,
        /// Icon color (hex or palette name). Default: white.
        icon_color: Option<String>,
        /// Horizontal corner radius in pixels. SVG-only.
        rx: Option<u32>,
        /// Vertical corner radius in pixels (defaults to rx if not set). SVG-only.
        ry: Option<u32>,
        /// Drop shadow: "color:blur:offset_x:offset_y" (e.g., "000000:4:2:2"). SVG-only.
        shadow: Option<String>,
        /// Gradient fill: "direction:color1:color2" (e.g., "horizontal:FF0000:0000FF"). SVG-only.
        gradient: Option<String>,
        /// Border dash pattern: "dash:gap" (e.g., "4:2" for dashed). SVG-only.
        stroke_dash: Option<String>,
        /// Logo size for shields.io ("auto" for adaptive). Shields-only.
        logo_size: Option<String>,
        /// Top border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_top: Option<String>,
        /// Right border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_right: Option<String>,
        /// Bottom border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_bottom: Option<String>,
        /// Left border: "color/width" (e.g., "FF0000/2") or just "color". SVG-only.
        border_left: Option<String>,
    },

    /// Technology badge with logo (uses Simple Icons)
    Tech {
        name: String,
        bg_color: String,
        logo_color: String,
        style: String,
        /// Optional label for two-segment badge
        label: Option<String>,
        /// Border color (hex). SVG-only.
        border_color: Option<String>,
        /// Border width in pixels. SVG-only.
        border_width: Option<u32>,
        /// Corner radius (uniform). SVG-only.
        rx: Option<u32>,
        /// Per-corner radii [top-left, top-right, bottom-right, bottom-left]. SVG-only.
        corners: Option<[u32; 4]>,
        /// Text/label color (hex). SVG-only.
        text_color: Option<String>,
        /// Font family. SVG-only.
        font: Option<String>,
        /// Rendering source: "svg" (default) or "shields" (shields.io URL)
        source: Option<String>,
        /// Chevron/arrow shape: "left", "right", "both". Creates pointed tab-style badges.
        chevron: Option<String>,
        /// Left segment background color (icon area). Defaults to bg_color.
        bg_left: Option<String>,
        /// Right segment background color (label area). Defaults to darkened bg_color.
        bg_right: Option<String>,
        /// Raised icon effect: pixels the icon extends above/below the label section.
        raised: Option<u32>,
    },

    /// Progress bar with customizable track and fill
    Progress {
        /// Percentage complete (0-100)
        percent: u8,
        /// Total width in pixels
        width: u32,
        /// Track (background) height in pixels
        height: u32,
        /// Track (background) color
        track_color: String,
        /// Fill (slider) color
        fill_color: String,
        /// Fill height in pixels (can be less than track height for "floating" effect)
        fill_height: u32,
        /// Corner radius
        rx: u32,
        /// Show percentage label
        show_label: bool,
        /// Label color (if show_label is true)
        label_color: Option<String>,
        /// Border color (optional)
        border_color: Option<String>,
        /// Border width in pixels (default: 0)
        border_width: u32,
        /// Thumb/slider height in pixels (enables slider mode when set)
        thumb_size: Option<u32>,
        /// Thumb width in pixels (defaults to thumb_size if not set)
        thumb_width: Option<u32>,
        /// Thumb color (defaults to fill_color)
        thumb_color: Option<String>,
        /// Thumb shape: "circle", "square", "diamond"
        thumb_shape: String,
    },

    /// Donut/ring chart showing percentage
    Donut {
        /// Percentage complete (0-100)
        percent: u8,
        /// Diameter in pixels
        size: u32,
        /// Ring thickness in pixels
        thickness: u32,
        /// Track (background) color
        track_color: String,
        /// Fill (progress) color
        fill_color: String,
        /// Show percentage label in center
        show_label: bool,
        /// Label color
        label_color: Option<String>,
        /// Thumb size in pixels (enables slider mode when set)
        thumb_size: Option<u32>,
        /// Thumb color (defaults to fill_color)
        thumb_color: Option<String>,
    },

    /// Gauge/half-donut showing percentage as semi-circular meter
    Gauge {
        /// Percentage complete (0-100)
        percent: u8,
        /// Width in pixels (height is approximately half + label space)
        size: u32,
        /// Arc thickness in pixels
        thickness: u32,
        /// Track (background) color
        track_color: String,
        /// Fill (progress) color
        fill_color: String,
        /// Show percentage label below arc
        show_label: bool,
        /// Label color
        label_color: Option<String>,
        /// Thumb size in pixels (enables slider mode when set)
        thumb_size: Option<u32>,
        /// Thumb color (defaults to fill_color)
        thumb_color: Option<String>,
    },

    /// Sparkline - mini inline chart for data visualization
    Sparkline {
        /// Data values (will be normalized to fit height)
        values: Vec<f32>,
        /// Total width in pixels
        width: u32,
        /// Total height in pixels
        height: u32,
        /// Chart type: "line", "bar", "area"
        chart_type: String,
        /// Line/bar/area fill color
        fill_color: String,
        /// Line stroke color (for line/area types)
        stroke_color: Option<String>,
        /// Line stroke width
        stroke_width: u32,
        /// Background/track color (optional)
        track_color: Option<String>,
        /// Show dots at data points (line type only)
        show_dots: bool,
        /// Dot radius (if show_dots is true)
        dot_radius: u32,
    },

    /// Rating display (stars, hearts, etc.) with partial fill support
    Rating {
        /// Rating value (e.g., 3.5 out of 5)
        value: f32,
        /// Maximum rating (default: 5)
        max: u32,
        /// Size of each icon in pixels
        size: u32,
        /// Fill color for filled/partial icons
        fill_color: String,
        /// Color for empty icons
        empty_color: String,
        /// Icon type: "star", "heart", "circle"
        icon: String,
        /// Spacing between icons in pixels
        spacing: u32,
    },

    /// Waveform - audio-style visualization with bars above/below center
    Waveform {
        /// Data values (positive = above center, negative = below)
        values: Vec<f32>,
        /// Total width in pixels
        width: u32,
        /// Total height in pixels
        height: u32,
        /// Color for bars above zero
        positive_color: String,
        /// Color for bars below zero
        negative_color: String,
        /// Width of each bar in pixels
        bar_width: u32,
        /// Spacing between bars in pixels
        spacing: u32,
        /// Background color (optional)
        track_color: Option<String>,
        /// Show center line at zero
        show_center_line: bool,
        /// Center line color
        center_line_color: Option<String>,
    },
}

impl Primitive {
    /// Get the default shield style
    pub fn default_style() -> &'static str {
        "flat-square"
    }

    /// Create a simple swatch with just color and style (all other options None)
    pub fn simple_swatch(color: impl Into<String>, style: impl Into<String>) -> Self {
        Primitive::Swatch {
            color: color.into(),
            style: style.into(),
            opacity: None,
            width: None,
            height: None,
            border_color: None,
            border_width: None,
            label: None,
            label_color: None,
            icon: None,
            icon_color: None,
            rx: None,
            ry: None,
            shadow: None,
            gradient: None,
            stroke_dash: None,
            logo_size: None,
            border_top: None,
            border_right: None,
            border_bottom: None,
            border_left: None,
        }
    }

    /// Create a simple progress bar with defaults
    pub fn simple_progress(
        percent: u8,
        track_color: impl Into<String>,
        fill_color: impl Into<String>,
    ) -> Self {
        Primitive::Progress {
            percent: percent.min(100),
            width: 100,
            height: 10,
            track_color: track_color.into(),
            fill_color: fill_color.into(),
            fill_height: 10,
            rx: 3,
            show_label: false,
            label_color: None,
            border_color: None,
            border_width: 0,
            thumb_size: None,
            thumb_width: None,
            thumb_color: None,
            thumb_shape: "circle".to_string(),
        }
    }

    /// Create a simple donut with defaults
    pub fn simple_donut(
        percent: u8,
        track_color: impl Into<String>,
        fill_color: impl Into<String>,
    ) -> Self {
        Primitive::Donut {
            percent: percent.min(100),
            size: 40,
            thickness: 4,
            track_color: track_color.into(),
            fill_color: fill_color.into(),
            show_label: false,
            label_color: None,
            thumb_size: None,
            thumb_color: None,
        }
    }

    /// Create a simple gauge with defaults
    pub fn simple_gauge(
        percent: u8,
        track_color: impl Into<String>,
        fill_color: impl Into<String>,
    ) -> Self {
        Primitive::Gauge {
            percent: percent.min(100),
            size: 80,
            thickness: 8,
            track_color: track_color.into(),
            fill_color: fill_color.into(),
            show_label: false,
            label_color: None,
            thumb_size: None,
            thumb_color: None,
        }
    }

    /// Create a simple sparkline with defaults
    pub fn simple_sparkline(values: Vec<f32>, fill_color: impl Into<String>) -> Self {
        Primitive::Sparkline {
            values,
            width: 100,
            height: 20,
            chart_type: "line".to_string(),
            fill_color: fill_color.into(),
            stroke_color: None,
            stroke_width: 2,
            track_color: None,
            show_dots: false,
            dot_radius: 2,
        }
    }

    /// Create a simple rating with defaults
    pub fn simple_rating(value: f32, fill_color: impl Into<String>) -> Self {
        Primitive::Rating {
            value,
            max: 5,
            size: 20,
            fill_color: fill_color.into(),
            empty_color: "6B7280".to_string(), // slate
            icon: "star".to_string(),
            spacing: 2,
        }
    }

    /// Create a simple waveform with defaults
    pub fn simple_waveform(
        values: Vec<f32>,
        positive_color: impl Into<String>,
        negative_color: impl Into<String>,
    ) -> Self {
        Primitive::Waveform {
            values,
            width: 100,
            height: 40,
            positive_color: positive_color.into(),
            negative_color: negative_color.into(),
            bar_width: 3,
            spacing: 1,
            track_color: None,
            show_center_line: false,
            center_line_color: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_swatch() {
        let swatch = Primitive::simple_swatch("ff6b35", "flat-square");
        if let Primitive::Swatch { color, style, .. } = swatch {
            assert_eq!(color, "ff6b35");
            assert_eq!(style, "flat-square");
        } else {
            panic!("Expected Swatch primitive");
        }
    }

    #[test]
    fn test_primitive_swatch_with_options() {
        let swatch = Primitive::Swatch {
            color: "F41C80".to_string(),
            style: "flat".to_string(),
            opacity: Some(0.5),
            width: Some(40),
            height: Some(30),
            border_color: Some("FFFFFF".to_string()),
            border_width: Some(2),
            label: Some("v1".to_string()),
            label_color: Some("000000".to_string()),
            icon: Some("rust".to_string()),
            icon_color: Some("FFFFFF".to_string()),
            rx: Some(5),
            ry: Some(10),
            shadow: Some("000000:4:2:2".to_string()),
            gradient: Some("horizontal:FF0000:0000FF".to_string()),
            stroke_dash: Some("4:2".to_string()),
            logo_size: Some("auto".to_string()),
            border_top: Some("FF0000/2".to_string()),
            border_right: None,
            border_bottom: Some("0000FF/3".to_string()),
            border_left: None,
        };
        if let Primitive::Swatch {
            opacity,
            width,
            label,
            rx,
            shadow,
            gradient,
            stroke_dash,
            logo_size,
            ..
        } = swatch
        {
            assert_eq!(opacity, Some(0.5));
            assert_eq!(width, Some(40));
            assert_eq!(label, Some("v1".to_string()));
            assert_eq!(rx, Some(5));
            assert_eq!(shadow, Some("000000:4:2:2".to_string()));
            assert_eq!(gradient, Some("horizontal:FF0000:0000FF".to_string()));
            assert_eq!(stroke_dash, Some("4:2".to_string()));
            assert_eq!(logo_size, Some("auto".to_string()));
        } else {
            panic!("Expected Swatch primitive");
        }
    }

    #[test]
    fn test_primitive_tech() {
        let tech = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "ffffff".to_string(),
            style: "flat-square".to_string(),
            label: None,
            border_color: None,
            border_width: None,
            rx: None,
            corners: None,
            text_color: None,
            font: None,
            source: None,
            chevron: None,
            bg_left: None,
            bg_right: None,
            raised: None,
        };

        if let Primitive::Tech { name, .. } = tech {
            assert_eq!(name, "rust");
        } else {
            panic!("Expected Tech primitive");
        }
    }

    #[test]
    fn test_primitive_tech_with_label() {
        let tech = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "ffffff".to_string(),
            style: "flat-square".to_string(),
            label: Some("v1.80".to_string()),
            border_color: None,
            border_width: None,
            rx: None,
            corners: None,
            text_color: None,
            font: None,
            source: None,
            chevron: None,
            bg_left: None,
            bg_right: None,
            raised: None,
        };

        if let Primitive::Tech { name, label, .. } = tech {
            assert_eq!(name, "rust");
            assert_eq!(label, Some("v1.80".to_string()));
        } else {
            panic!("Expected Tech primitive");
        }
    }

    #[test]
    fn test_primitive_tech_with_border() {
        let tech = Primitive::Tech {
            name: "rust".to_string(),
            bg_color: "000000".to_string(),
            logo_color: "ffffff".to_string(),
            style: "flat-square".to_string(),
            label: Some("v1.80".to_string()),
            border_color: Some("F41C80".to_string()),
            border_width: Some(2),
            rx: Some(8),
            corners: None,
            text_color: None,
            font: None,
            source: None,
            chevron: None,
            bg_left: None,
            bg_right: None,
            raised: None,
        };

        if let Primitive::Tech {
            name,
            border_color,
            border_width,
            rx,
            ..
        } = tech
        {
            assert_eq!(name, "rust");
            assert_eq!(border_color, Some("F41C80".to_string()));
            assert_eq!(border_width, Some(2));
            assert_eq!(rx, Some(8));
        } else {
            panic!("Expected Tech primitive");
        }
    }

    #[test]
    fn test_primitive_progress() {
        let progress = Primitive::simple_progress(75, "gray", "pink");

        if let Primitive::Progress {
            percent,
            width,
            height,
            fill_height,
            ..
        } = progress
        {
            assert_eq!(percent, 75);
            assert_eq!(width, 100);
            assert_eq!(height, 10);
            assert_eq!(fill_height, 10);
        } else {
            panic!("Expected Progress primitive");
        }
    }

    #[test]
    fn test_primitive_progress_clamped() {
        let progress = Primitive::simple_progress(150, "gray", "pink");

        if let Primitive::Progress { percent, .. } = progress {
            assert_eq!(percent, 100); // Clamped to 100
        } else {
            panic!("Expected Progress primitive");
        }
    }
}
