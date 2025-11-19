use crate::fl;
use std::fmt;

/// Time units supported by Chronomancer.
///
/// Internal arithmetic uses seconds. Use `to_seconds_multiplier()` to convert a unit into its multiplier.
///
/// Variants:
/// - `Seconds`
/// - `Minutes`
/// - `Hours`
/// - `Days`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnit {
    /// Returns the number of whole seconds represented by this time unit.
    ///
    /// Takes `self` by value per Clippy's convention for `to_*` methods on `Copy` types.
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
