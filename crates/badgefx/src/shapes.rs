//! SVG path generation for badge shapes

use crate::style::ChevronDirection;

/// Arrow depth constant for chevron badges
pub const CHEVRON_ARROW_DEPTH: f32 = 10.0;

/// Generate SVG path for a rectangle with per-corner radii
/// Order: [top-left, top-right, bottom-right, bottom-left]
/// Corner radii are clamped to avoid path artifacts (max half of width/height)
pub fn rounded_rect_path(x: f32, y: f32, w: f32, h: f32, corners: [u32; 4]) -> String {
    // Clamp corner radii to prevent path artifacts when radius >= dimension
    let max_h_radius = h / 2.0;
    let max_w_radius = w / 2.0;
    let [tl, tr, br, bl] = corners.map(|c| {
        let r = c as f32;
        r.min(max_h_radius).min(max_w_radius)
    });

    let mut path = String::new();

    // Start at top-left (after corner if rounded)
    path.push_str(&format!("M{} {}", x + tl, y));

    // Top edge to top-right
    path.push_str(&format!("H{}", x + w - tr));

    // Top-right corner
    if tr > 0.0 {
        path.push_str(&format!("Q{} {} {} {}", x + w, y, x + w, y + tr));
    } else {
        path.push_str(&format!("L{} {}", x + w, y));
    }

    // Right edge to bottom-right
    path.push_str(&format!("V{}", y + h - br));

    // Bottom-right corner
    if br > 0.0 {
        path.push_str(&format!("Q{} {} {} {}", x + w, y + h, x + w - br, y + h));
    } else {
        path.push_str(&format!("L{} {}", x + w, y + h));
    }

    // Bottom edge to bottom-left
    path.push_str(&format!("H{}", x + bl));

    // Bottom-left corner
    if bl > 0.0 {
        path.push_str(&format!("Q{} {} {} {}", x, y + h, x, y + h - bl));
    } else {
        path.push_str(&format!("L{} {}", x, y + h));
    }

    // Left edge to top-left
    path.push_str(&format!("V{}", y + tl));

    // Top-left corner
    if tl > 0.0 {
        path.push_str(&format!("Q{} {} {} {}", x, y, x + tl, y));
    } else {
        path.push_str(&format!("L{} {}", x, y));
    }

    path.push('Z');
    path
}

/// Generate SVG path for a chevron/arrow shaped badge
pub fn chevron_path_with_overlap(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    direction: ChevronDirection,
    arrow_depth: f32,
) -> (String, bool, bool) {
    let center_y = h / 2.0;
    let center = y + center_y;
    let bottom = y + h;

    match direction {
        ChevronDirection::Left => {
            let arrow_tip = x - arrow_depth;
            let path = format!(
                "M{arrow_tip} {center}L{x} {y}H{right}V{bottom}H{x}L{arrow_tip} {center}Z",
                arrow_tip = arrow_tip,
                center = center,
                x = x,
                y = y,
                right = x + w,
                bottom = bottom
            );
            (path, true, false)
        }
        ChevronDirection::Right => {
            let arrow_tip = x + w + arrow_depth;
            let path = format!(
                "M{x} {y}H{right}L{arrow_tip} {center}L{right} {bottom}H{x}V{y}Z",
                x = x,
                y = y,
                right = x + w,
                arrow_tip = arrow_tip,
                center = center,
                bottom = bottom
            );
            (path, false, true)
        }
        ChevronDirection::Both => {
            let left_tip = x - arrow_depth;
            let right_tip = x + w + arrow_depth;
            let path = format!(
                "M{left_tip} {center}L{x} {y}H{right}L{right_tip} {center}L{right} {bottom}H{x}L{left_tip} {center}Z",
                left_tip = left_tip,
                center = center,
                x = x,
                y = y,
                right = x + w,
                right_tip = right_tip,
                bottom = bottom
            );
            (path, true, true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::ChevronDirection;

    #[test]
    fn test_rounded_rect_path() {
        let path = rounded_rect_path(0.0, 0.0, 100.0, 20.0, [3, 3, 3, 3]);
        assert!(path.contains("M3 0"));
        assert!(path.contains("H97"));
    }

    #[test]
    fn test_simple_rect_path() {
        let path = rounded_rect_path(0.0, 0.0, 100.0, 20.0, [0, 0, 0, 0]);
        assert!(path.contains("M0 0"));
        assert!(path.contains("H100"));
    }

    #[test]
    fn test_different_corner_radii() {
        let path = rounded_rect_path(0.0, 0.0, 100.0, 20.0, [5, 10, 15, 20]);
        assert!(path.contains("M5 0")); // Start after top-left radius
        assert!(path.contains("H90")); // Top edge to before top-right radius
    }

    #[test]
    fn test_chevron_paths() {
        let (path, has_left, has_right) =
            chevron_path_with_overlap(0.0, 0.0, 100.0, 20.0, ChevronDirection::Left, 10.0);
        assert!(has_left);
        assert!(!has_right);
        assert!(path.contains("M-10 10")); // Arrow tip

        let (path, has_left, has_right) =
            chevron_path_with_overlap(0.0, 0.0, 100.0, 20.0, ChevronDirection::Right, 10.0);
        assert!(!has_left);
        assert!(has_right);
        assert!(path.contains("L110 10")); // Arrow tip

        let (_path, has_left, has_right) =
            chevron_path_with_overlap(0.0, 0.0, 100.0, 20.0, ChevronDirection::Both, 10.0);
        assert!(has_left);
        assert!(has_right);
    }
}
