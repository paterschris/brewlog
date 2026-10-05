const GRAMS_PER_OUNCE: f64 = 28.3495;
const MILLILITERS_PER_CUP: f64 = 236.588;

pub fn ounces_to_grams(ounces: f64) -> f64 {
    ounces * GRAMS_PER_OUNCE
}

/// Water is close enough to 1 g/ml at brewing temperatures that cups convert
/// straight to grams.
pub fn cups_to_grams(cups: f64) -> f64 {
    cups * MILLILITERS_PER_CUP
}

pub fn format_duration(total_seconds: u32) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    if hours > 0 {
        format!("{hours}h {minutes:02}m")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_short_and_long_durations() {
        assert_eq!(format_duration(210), "3:30");
        assert_eq!(format_duration(16 * 3600), "16h 00m");
    }

    #[test]
    fn converts_cups() {
        assert!((cups_to_grams(2.0) - 473.176).abs() < 0.001);
    }

    #[test]
    fn converts_ounces() {
        assert!((ounces_to_grams(12.0) - 340.194).abs() < 0.001);
    }
}
