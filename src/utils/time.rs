//! Time unit utilities for duration formatting and conversion.
//!
//! This module provides types and functions for working with time durations in
//! Chronomancer. It handles conversion between different time units and formatting
//! durations for display to users.
//!
//! # Examples
//!
//! ## Converting time units to seconds
//!
//! ```rust
//! use chronomancer::utils::TimeUnit;
//!
//! let minutes = TimeUnit::Minutes;
//! let seconds = minutes.to_seconds_multiplier();
//! assert_eq!(seconds, 60);
//!
//! // Calculate total seconds for a duration
//! let duration_value = 5;
//! let total_seconds = duration_value * TimeUnit::Hours.to_seconds_multiplier();
//! assert_eq!(total_seconds, 18000); // 5 hours = 18000 seconds
//! ```
//!
//! ## Formatting durations for display
//!
//! ```rust
//! use chronomancer::utils::time::format_duration;
//!
//! // Displays as hours if >= 1 hour
//! assert_eq!(format_duration(3600), "1 hour");
//! assert_eq!(format_duration(7200), "2 hours");
//!
//! // Displays as minutes if < 1 hour
//! assert_eq!(format_duration(60), "1 minute");
//! assert_eq!(format_duration(300), "5 minutes");
//! ```

use crate::fl;
use std::fmt;

/// Time units supported by Chronomancer.
///
/// Represents the different time units that users can select when specifying
/// durations for timers and power operations. Internally, all durations are
/// stored as seconds, and these units are used for display and input.
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::TimeUnit;
///
/// let unit = TimeUnit::Minutes;
/// assert_eq!(unit.to_seconds_multiplier(), 60);
///
/// // Use with a value to calculate total seconds
/// let value = 30; // 30 minutes
/// let total_seconds = value * unit.to_seconds_multiplier();
/// assert_eq!(total_seconds, 1800);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    /// Seconds (1 second = 1 second)
    Seconds,

    /// Minutes (1 minute = 60 seconds)
    Minutes,

    /// Hours (1 hour = 3600 seconds)
    Hours,

    /// Days (1 day = 86400 seconds)
    Days,
}

impl TimeUnit {
    /// Returns the number of seconds in one unit of this time unit.
    ///
    /// This multiplier is used to convert a duration value to seconds.
    /// For example, to convert 5 minutes to seconds: `5 * TimeUnit::Minutes.to_seconds_multiplier()`
    ///
    /// # Returns
    ///
    /// - `Seconds`: 1
    /// - `Minutes`: 60
    /// - `Hours`: 3600
    /// - `Days`: 86400
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::utils::TimeUnit;
    ///
    /// assert_eq!(TimeUnit::Seconds.to_seconds_multiplier(), 1);
    /// assert_eq!(TimeUnit::Minutes.to_seconds_multiplier(), 60);
    /// assert_eq!(TimeUnit::Hours.to_seconds_multiplier(), 3600);
    /// assert_eq!(TimeUnit::Days.to_seconds_multiplier(), 86400);
    ///
    /// // Convert 3 hours to seconds
    /// let hours = 3;
    /// let seconds = hours * TimeUnit::Hours.to_seconds_multiplier();
    /// assert_eq!(seconds, 10800);
    /// ```
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
    /// Formats the time unit as a localized, human-readable string.
    ///
    /// The display text is localized using the application's current language
    /// settings via the `fl!` macro.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::utils::TimeUnit;
    ///
    /// let unit = TimeUnit::Minutes;
    /// println!("Selected unit: {}", unit); // Prints localized "Minutes"
    /// ```
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
///
/// Converts a duration in seconds to a friendly display format:
/// - Shows hours if the duration is >= 1 hour
/// - Shows minutes if the duration is < 1 hour
/// - Rounds down to whole units (no decimals)
/// - Properly pluralizes (1 hour vs 2 hours)
///
/// # Arguments
///
/// - `seconds` - Duration in seconds to format
///
/// # Returns
///
/// A formatted string like "1 hour", "5 minutes", etc.
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::time::format_duration;
///
/// // Hours (>= 3600 seconds)
/// assert_eq!(format_duration(3600), "1 hour");
/// assert_eq!(format_duration(7200), "2 hours");
///
/// // Minutes (< 3600 seconds)
/// assert_eq!(format_duration(60), "1 minute");
/// assert_eq!(format_duration(120), "2 minutes");
/// assert_eq!(format_duration(1800), "30 minutes");
///
/// // Rounds down to whole units
/// assert_eq!(format_duration(5400), "1 hour");  // 1.5 hours → 1 hour
/// assert_eq!(format_duration(30), "0 minutes"); // < 1 minute → 0 minutes
/// ```
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
