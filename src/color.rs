/// Returns the hex color for a given coverage percentage.
/// - < 50%: red (#e05d44)
/// - 50-79%: yellow (#dfb317)
/// - >= 80%: green (#4c1)
pub fn coverage_color(percentage: f64) -> &'static str {
    if percentage < 50.0 {
        "#e05d44"
    } else if percentage < 80.0 {
        "#dfb317"
    } else {
        "#4c1"
    }
}

/// Returns a description of the coverage level.
pub fn coverage_level(percentage: f64) -> &'static str {
    if percentage < 50.0 {
        "low"
    } else if percentage < 80.0 {
        "medium"
    } else {
        "high"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_coverage_is_red() {
        assert_eq!(coverage_color(0.0), "#e05d44");
        assert_eq!(coverage_color(25.0), "#e05d44");
        assert_eq!(coverage_color(49.9), "#e05d44");
    }

    #[test]
    fn test_medium_coverage_is_yellow() {
        assert_eq!(coverage_color(50.0), "#dfb317");
        assert_eq!(coverage_color(65.0), "#dfb317");
        assert_eq!(coverage_color(79.9), "#dfb317");
    }

    #[test]
    fn test_high_coverage_is_green() {
        assert_eq!(coverage_color(80.0), "#4c1");
        assert_eq!(coverage_color(90.0), "#4c1");
        assert_eq!(coverage_color(100.0), "#4c1");
    }
}
