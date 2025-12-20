//! Shared SVG rendering utilities

use crate::primitive::ThumbConfig;

/// Build stroke attribute string for SVG elements.
///
/// Returns a string like ` stroke="#COLOR" stroke-width="WIDTH"` if color is Some
/// and width > 0, otherwise returns an empty string.
///
/// # Examples
/// ```ignore
/// let attr = build_stroke_attr(Some("FF0000"), 2);
/// assert_eq!(attr, " stroke=\"#FF0000\" stroke-width=\"2\"");
///
/// let attr = build_stroke_attr(None, 2);
/// assert_eq!(attr, "");
/// ```
pub fn build_stroke_attr(color: Option<&str>, width: u32) -> String {
    match color {
        Some(c) if width > 0 => format!(" stroke=\"#{}\" stroke-width=\"{}\"", c, width),
        _ => String::new(),
    }
}

/// Calculate fill and gap dash lengths for stroke-dasharray.
///
/// Used by donut and gauge renderers for percentage-based arc fills.
/// Returns (fill_length, gap_length) for use in stroke-dasharray attribute.
pub fn arc_dash_lengths(arc_length: f32, percent: u8) -> (f32, f32) {
    let fill = arc_length * (percent as f32 / 100.0);
    (fill, arc_length - fill)
}

/// Calculate a point on a circle given center, radius, and angle.
///
/// Returns (x, y) coordinates for the point at the given angle.
/// Angle is in degrees (0° = right, 90° = down, etc.).
pub fn point_on_circle(center_x: f32, center_y: f32, radius: f32, angle_deg: f32) -> (f32, f32) {
    let angle_rad = angle_deg * std::f32::consts::PI / 180.0;
    (
        center_x + radius * angle_rad.cos(),
        center_y + radius * angle_rad.sin(),
    )
}

/// Calculate thumb padding needed to ensure thumb doesn't clip outside the SVG.
///
/// When a thumb indicator extends beyond the track, extra padding is needed.
/// Returns the padding to add on each side.
pub fn thumb_padding(thumb: Option<&ThumbConfig>, thickness: u32) -> u32 {
    thumb
        .map(|t| (t.size / 2).saturating_sub(thickness / 2))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Some("FF0000"), 2, " stroke=\"#FF0000\" stroke-width=\"2\"")]
    #[case(None, 2, "")]
    #[case(Some("FF0000"), 0, "")]
    #[case(None, 0, "")]
    fn test_build_stroke_attr(
        #[case] color: Option<&str>,
        #[case] width: u32,
        #[case] expected: &str,
    ) {
        assert_eq!(build_stroke_attr(color, width), expected);
    }

    #[rstest]
    #[case(100.0, 50, 50.0, 50.0)]
    #[case(100.0, 0, 0.0, 100.0)]
    #[case(100.0, 100, 100.0, 0.0)]
    #[case(200.0, 25, 50.0, 150.0)]
    fn test_arc_dash_lengths(
        #[case] arc_length: f32,
        #[case] percent: u8,
        #[case] expected_fill: f32,
        #[case] expected_gap: f32,
    ) {
        let (fill, gap) = arc_dash_lengths(arc_length, percent);
        assert!((fill - expected_fill).abs() < 0.01);
        assert!((gap - expected_gap).abs() < 0.01);
    }

    #[rstest]
    #[case(50.0, 50.0, 25.0, 0.0, 75.0, 50.0)]   // 0° = right
    #[case(50.0, 50.0, 25.0, 90.0, 50.0, 75.0)]  // 90° = down
    #[case(50.0, 50.0, 25.0, 180.0, 25.0, 50.0)] // 180° = left
    #[case(50.0, 50.0, 25.0, 270.0, 50.0, 25.0)] // 270° = up
    fn test_point_on_circle(
        #[case] cx: f32,
        #[case] cy: f32,
        #[case] radius: f32,
        #[case] angle: f32,
        #[case] expected_x: f32,
        #[case] expected_y: f32,
    ) {
        let (x, y) = point_on_circle(cx, cy, radius, angle);
        assert!((x - expected_x).abs() < 0.01);
        assert!((y - expected_y).abs() < 0.01);
    }

    #[rstest]
    #[case(None, 10, 0)]          // No thumb -> 0 padding
    #[case(Some(20), 10, 5)]      // Thumb larger than track
    #[case(Some(8), 20, 0)]       // Thumb smaller than track (saturating)
    #[case(Some(16), 16, 0)]      // Thumb equals track
    fn test_thumb_padding(
        #[case] thumb_size: Option<u32>,
        #[case] thickness: u32,
        #[case] expected: u32,
    ) {
        let thumb = thumb_size.map(|size| ThumbConfig {
            size,
            ..Default::default()
        });
        assert_eq!(thumb_padding(thumb.as_ref(), thickness), expected);
    }
}
