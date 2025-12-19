//! Badge styling and SVG metrics

/// Badge visual styles
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeStyle {
    /// Flat style with minimal visual elements
    #[default]
    Flat,
    /// Flat style with square corners
    FlatSquare,
    /// Plastic style with subtle gradients and shadows
    Plastic,
    /// Large, bold style popular on GitHub
    ForTheBadge,
    /// Social media style with rounded corners
    Social,
}

impl BadgeStyle {
    /// Parse badge style from string
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "flat" => BadgeStyle::Flat,
            "flat-square" | "flat_square" | "flatsquare" => BadgeStyle::FlatSquare,
            "plastic" => BadgeStyle::Plastic,
            "for-the-badge" | "for_the_badge" | "forthebadge" => BadgeStyle::ForTheBadge,
            "social" => BadgeStyle::Social,
            _ => BadgeStyle::Flat,
        }
    }
}

impl BadgeStyle {
    /// Get the default corner radius for this style
    pub fn default_radius(&self) -> u32 {
        match self {
            BadgeStyle::Flat => 3,
            BadgeStyle::FlatSquare => 0,
            BadgeStyle::Plastic => 3,
            BadgeStyle::ForTheBadge => 3,
            BadgeStyle::Social => 10,
        }
    }

    /// Get the default height for this style
    pub fn default_height(&self) -> u32 {
        match self {
            BadgeStyle::Flat | BadgeStyle::FlatSquare | BadgeStyle::Plastic => 20,
            BadgeStyle::ForTheBadge => 28,
            BadgeStyle::Social => 20,
        }
    }

    /// Whether this style uses gradients
    pub fn has_gradients(&self) -> bool {
        matches!(self, BadgeStyle::Plastic)
    }

    /// Whether this style has shadows
    pub fn has_shadow(&self) -> bool {
        matches!(self, BadgeStyle::Plastic | BadgeStyle::ForTheBadge)
    }
}

/// SVG layout metrics for badge rendering
#[derive(Debug, Clone)]
pub struct SvgMetrics {
    /// Total badge width
    pub width: f32,
    /// Total badge height
    pub height: f32,
    /// Icon section width
    pub icon_width: f32,
    /// Text section width
    pub text_width: f32,
    /// Corner radius
    pub radius: f32,
    /// Text font size
    pub font_size: f32,
    /// Icon size
    pub icon_size: f32,
    /// Left padding for icon
    pub icon_padding_left: f32,
    /// Right padding for icon
    pub icon_padding_right: f32,
    /// Left padding for text
    pub text_padding_left: f32,
    /// Right padding for text
    pub text_padding_right: f32,
}

impl SvgMetrics {
    /// Create metrics from badge style with default values
    pub fn from_style(style: BadgeStyle) -> Self {
        Self {
            width: 100.0, // Default, will be calculated by render
            height: style.default_height() as f32,
            icon_width: 36.0,
            text_width: 64.0,
            radius: style.default_radius() as f32,
            font_size: if style.default_height() > 24 {
                11.0
            } else {
                10.0
            },
            icon_size: 14.0,
            icon_padding_left: 8.0,
            icon_padding_right: 8.0,
            text_padding_left: 12.0,
            text_padding_right: 12.0,
        }
    }

    /// Calculate metrics for a badge with the given parameters
    pub fn calculate(
        text: &str,
        icon_size: f32,
        font_size: f32,
        style: BadgeStyle,
        has_icon: bool,
    ) -> Self {
        // Estimate text width (rough approximation)
        let char_width = font_size * 0.6; // Average character width
        let text_width = text.len() as f32 * char_width;

        let icon_padding = if has_icon { 8.0 } else { 0.0 };
        let text_padding = 12.0;

        let icon_width = if has_icon {
            icon_size + icon_padding * 2.0
        } else {
            0.0
        };
        let text_section_width = text_width + text_padding * 2.0;

        let total_width = icon_width + text_section_width;
        let height = style.default_height() as f32;
        let radius = style.default_radius() as f32;

        Self {
            width: total_width,
            height,
            icon_width,
            text_width: text_section_width,
            radius,
            font_size,
            icon_size,
            icon_padding_left: icon_padding,
            icon_padding_right: icon_padding,
            text_padding_left: text_padding,
            text_padding_right: text_padding,
        }
    }

    /// Get the X coordinate where the text section starts
    pub fn text_x(&self) -> f32 {
        self.icon_width
    }

    /// Get the center Y coordinate for vertical alignment
    pub fn center_y(&self) -> f32 {
        self.height / 2.0
    }

    /// Get the center X coordinate of the icon section
    pub fn icon_center_x(&self) -> f32 {
        self.icon_width / 2.0
    }

    /// Get the center X coordinate of the text section
    pub fn text_center_x(&self) -> f32 {
        self.icon_width + self.text_width / 2.0
    }
}

/// Border styling configuration
#[derive(Debug, Clone)]
pub struct Border {
    /// Border color (hex format)
    pub color: String,
    /// Border width in pixels
    pub width: u32,
}

impl Border {
    /// Create a new border configuration
    pub fn new(color: impl Into<String>, width: u32) -> Self {
        Self {
            color: color.into(),
            width,
        }
    }
}

/// Custom corner radius configuration
#[derive(Debug, Clone)]
pub struct Corners {
    /// Top-left corner radius
    pub top_left: u32,
    /// Top-right corner radius
    pub top_right: u32,
    /// Bottom-right corner radius
    pub bottom_right: u32,
    /// Bottom-left corner radius
    pub bottom_left: u32,
}

impl Corners {
    /// Create uniform corner radii
    pub fn uniform(radius: u32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }

    /// Create corners with different horizontal and vertical radii
    pub fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self {
            top_left: horizontal,
            top_right: vertical,
            bottom_right: horizontal,
            bottom_left: vertical,
        }
    }

    /// Create completely custom corners
    pub fn custom(top_left: u32, top_right: u32, bottom_right: u32, bottom_left: u32) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

/// Chevron/arrow styling configuration
#[derive(Debug, Clone)]
pub struct Chevron {
    /// Direction of the chevron
    pub direction: ChevronDirection,
    /// Arrow depth in pixels
    pub depth: f32,
}

/// Chevron direction options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChevronDirection {
    /// Left-pointing arrow
    Left,
    /// Right-pointing arrow
    Right,
    /// Arrows on both sides
    Both,
}

impl ChevronDirection {
    /// Check if chevron has a left-pointing arrow
    pub fn has_left(&self) -> bool {
        matches!(self, ChevronDirection::Left | ChevronDirection::Both)
    }

    /// Check if chevron has a right-pointing arrow
    pub fn has_right(&self) -> bool {
        matches!(self, ChevronDirection::Right | ChevronDirection::Both)
    }
}

impl Chevron {
    /// Create a left-pointing chevron
    pub fn left(depth: f32) -> Self {
        Self {
            direction: ChevronDirection::Left,
            depth,
        }
    }

    /// Create a right-pointing chevron
    pub fn right(depth: f32) -> Self {
        Self {
            direction: ChevronDirection::Right,
            depth,
        }
    }

    /// Create chevrons on both sides
    pub fn both(depth: f32) -> Self {
        Self {
            direction: ChevronDirection::Both,
            depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // ========================================================================
    // Badge Style Properties (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(BadgeStyle::Flat, 3, 20, false, false)]
    #[case(BadgeStyle::FlatSquare, 0, 20, false, false)]
    #[case(BadgeStyle::Plastic, 3, 20, true, true)]
    #[case(BadgeStyle::ForTheBadge, 3, 28, false, true)]
    #[case(BadgeStyle::Social, 10, 20, false, false)]
    fn test_badge_style_properties(
        #[case] style: BadgeStyle,
        #[case] radius: u32,
        #[case] height: u32,
        #[case] gradients: bool,
        #[case] shadow: bool,
    ) {
        assert_eq!(style.default_radius(), radius);
        assert_eq!(style.default_height(), height);
        assert_eq!(style.has_gradients(), gradients);
        assert_eq!(style.has_shadow(), shadow);
    }

    // ========================================================================
    // Style Parsing (Parameterized)
    // ========================================================================

    #[rstest]
    #[case("flat", BadgeStyle::Flat)]
    #[case("flat-square", BadgeStyle::FlatSquare)]
    #[case("flat_square", BadgeStyle::FlatSquare)]
    #[case("plastic", BadgeStyle::Plastic)]
    #[case("for-the-badge", BadgeStyle::ForTheBadge)]
    #[case("social", BadgeStyle::Social)]
    #[case("unknown", BadgeStyle::Flat)] // fallback
    fn test_style_parsing(#[case] input: &str, #[case] expected: BadgeStyle) {
        assert_eq!(BadgeStyle::parse(input), expected);
    }

    #[test]
    fn test_svg_metrics() {
        let metrics = SvgMetrics::calculate("Rust", 14.0, 11.0, BadgeStyle::Flat, true);

        assert!(metrics.width > 0.0);
        assert_eq!(metrics.height, 20.0);
        assert!(metrics.icon_width > 0.0);
        assert!(metrics.text_width > 0.0);
        assert_eq!(metrics.radius, 3.0);
    }

    #[test]
    fn test_border() {
        let border = Border::new("#FF0000", 2);
        assert_eq!(border.color, "#FF0000");
        assert_eq!(border.width, 2);
    }

    // ========================================================================
    // Corners (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(5, [5, 5, 5, 5])] // uniform
    #[case(0, [0, 0, 0, 0])]
    #[case(10, [10, 10, 10, 10])]
    fn test_corners_uniform(#[case] radius: u32, #[case] expected: [u32; 4]) {
        let corners = Corners::uniform(radius);
        assert_eq!(
            [corners.top_left, corners.top_right, corners.bottom_right, corners.bottom_left],
            expected
        );
    }

    #[test]
    fn test_corners_custom() {
        let custom = Corners::custom(1, 2, 3, 4);
        assert_eq!(custom.top_left, 1);
        assert_eq!(custom.top_right, 2);
        assert_eq!(custom.bottom_right, 3);
        assert_eq!(custom.bottom_left, 4);
    }

    // ========================================================================
    // Chevron (Parameterized)
    // ========================================================================

    #[rstest]
    #[case(ChevronDirection::Left, true, false)]
    #[case(ChevronDirection::Right, false, true)]
    #[case(ChevronDirection::Both, true, true)]
    fn test_chevron_direction(
        #[case] direction: ChevronDirection,
        #[case] has_left: bool,
        #[case] has_right: bool,
    ) {
        assert_eq!(direction.has_left(), has_left);
        assert_eq!(direction.has_right(), has_right);
    }

    #[test]
    fn test_chevron_constructors() {
        let left = Chevron::left(10.0);
        assert_eq!(left.direction, ChevronDirection::Left);
        assert_eq!(left.depth, 10.0);

        let both = Chevron::both(8.0);
        assert_eq!(both.direction, ChevronDirection::Both);
        assert_eq!(both.depth, 8.0);
    }
}
