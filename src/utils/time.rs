use crate::fl;
use std::fmt;

/// Time units supported by Chronomancer.
// Internal arithmetic uses seconds. Use `to_seconds_multiplier()` to convert a unit into its multiplier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnit {
    /// Returns the number of whole seconds represented by this time unit.
    #[must_use]
    pub fn to_seconds_multiplier(self) -> i32 {
        match self {
            TimeUnit::Seconds => 1,
            TimeUnit::Minutes => 60,
            TimeUnit::Hours => 3600,
            TimeUnit::Days => 86400,
        }
    }
}

impl fmt::Display for TimeUnit {
    /// Localized human-readable label for the unit.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeUnit::Seconds => write!(f, "{}", fl!("seconds")),
            TimeUnit::Minutes => write!(f, "{}", fl!("minutes")),
            TimeUnit::Hours => write!(f, "{}", fl!("hours")),
            TimeUnit::Days => write!(f, "{}", fl!("days")),
        }
    }
}

/// Formats a duration in seconds into a human-readable string.
#[must_use]
pub fn format_duration(seconds: i32) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!("{} hour{}", hours, if hours == 1 { "" } else { "s" })
    } else {
        format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_hours() {
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(7200), "2 hours");
        assert_eq!(format_duration(10800), "3 hours");
    }

    #[test]
    fn test_format_duration_minutes() {
        assert_eq!(format_duration(60), "1 minute");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(300), "5 minutes");
        assert_eq!(format_duration(1800), "30 minutes");
    }

    #[test]
    fn test_format_duration_rounds_down() {
        // 90 minutes = 1 hour (rounds down)
        assert_eq!(format_duration(5400), "1 hour");
        // 119 minutes = 1 hour
        assert_eq!(format_duration(7140), "1 hour");
    }

    #[test]
    fn test_format_duration_less_than_minute() {
        // Less than a minute shows as 0 minutes
        assert_eq!(format_duration(30), "0 minutes");
        assert_eq!(format_duration(59), "0 minutes");
    }
}
