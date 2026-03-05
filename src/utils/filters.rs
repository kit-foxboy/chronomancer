//! Text input filters for validating and sanitizing user input.
//!
//! These filters are designed for use with cosmic's `TextInput::on_input` callback
//! pattern, where returning `None` rejects the input and `Some(String)` accepts it
//! (potentially modified).
//!
//! # Design Pattern
//!
//! Filters follow a consistent pattern:
//! 1. Accept raw input as `&str`
//! 2. Apply filtering rules
//! 3. Return `Option<String>` — `None` means "reject this input entirely"
//!
//! # Usage with TextInput
//!
//! ```rust,no_run
//! use chronomancer::utils::filters::filter_positive_integer;
//!
//! // In a cosmic TextInput callback:
//! // TextInput::new("Enter number", &self.value)
//! //     .on_input(|s| {
//! //         filter_positive_integer(&s)
//! //             .map(Message::InputChanged)
//! //             .unwrap_or(Message::InputRejected)
//! //     })
//! ```
//!
//! # Available Filters
//!
//! - [`filter_positive_integer`] - Only digits, must be > 0
//! - [`filter_alphabetic`] - Only alphabetic characters
//! - [`filter_alphanumeric`] - Only alphanumeric characters
//!
//! # Examples
//!
//! ```rust,no_run
//! use chronomancer::utils::filters::{filter_positive_integer, filter_alphabetic, filter_alphanumeric};
//!
//! // Positive integers only
//! assert_eq!(filter_positive_integer("42"), Some("42".to_string()));
//! assert_eq!(filter_positive_integer("0"), None);
//!
//! // Alphabetic only
//! assert_eq!(filter_alphabetic("Hello"), Some("Hello".to_string()));
//! assert_eq!(filter_alphabetic("123"), None);
//!
//! // Alphanumeric only
//! assert_eq!(filter_alphanumeric("Hello123"), Some("Hello123".to_string()));
//! assert_eq!(filter_alphanumeric("!@#"), None);
//! ```

/// Filters input to only allow positive integers (value > 0).
///
/// Empty strings are accepted (returns `Some("")`) to allow clearing input
/// fields. Non-empty input is parsed as `u32` — zero and unparseable values
/// are rejected.
///
/// # Arguments
///
/// - `input` - The text to filter
///
/// # Returns
///
/// - `Some(String)` - Valid positive integer (normalized) or empty string
/// - `None` - Invalid input (non-numeric, zero, or negative)
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::filters::filter_positive_integer;
///
/// assert_eq!(filter_positive_integer("42"), Some("42".to_string()));
/// assert_eq!(filter_positive_integer("007"), Some("7".to_string())); // Normalized
///
/// // Empty string is allowed (for clearing input)
/// assert_eq!(filter_positive_integer(""), Some("".to_string()));
///
/// // Zero is not positive
/// assert_eq!(filter_positive_integer("0"), None);
///
/// // Non-numeric rejected
/// assert_eq!(filter_positive_integer("-5"), None);
/// assert_eq!(filter_positive_integer("abc"), None);
/// assert_eq!(filter_positive_integer("3.14"), None);
/// ```
#[must_use]
pub fn filter_positive_integer(input: &str) -> Option<String> {
    if input.is_empty() {
        Some(String::new())
    } else if let Ok(value) = input.parse::<u32>() {
        if value > 0 {
            Some(value.to_string())
        } else {
            None
        }
    } else {
        None
    }
}

/// Filters input to only allow alphabetic characters.
///
/// # Arguments
///
/// - `input` - The text to filter
///
/// # Returns
///
/// - `Some(String)` - String containing only alphabetic characters
/// - `None` - If result would be empty (when input has no alphabetic chars)
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::filters::filter_alphabetic;
///
/// assert_eq!(filter_alphabetic("Hello"), Some("Hello".to_string()));
/// assert_eq!(filter_alphabetic("Hello123"), Some("Hello".to_string()));
///
/// // Empty string has no alphabetic chars
/// assert_eq!(filter_alphabetic(""), None);
///
/// // All non-alphabetic characters filtered out returns None
/// assert_eq!(filter_alphabetic("123!@#"), None);
/// ```
#[must_use]
#[allow(dead_code)]
pub fn filter_alphabetic(input: &str) -> Option<String> {
    let filtered: String = input.chars().filter(|c| c.is_alphabetic()).collect();
    if filtered.is_empty() {
        None
    } else {
        Some(filtered)
    }
}

/// Filters input to only allow alphanumeric characters.
///
/// # Arguments
///
/// - `input` - The text to filter
///
/// # Returns
///
/// - `Some(String)` - String containing only alphanumeric characters
/// - `None` - If result would be empty (when input has no alphanumeric chars)
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::filters::filter_alphanumeric;
///
/// assert_eq!(filter_alphanumeric("Hello123"), Some("Hello123".to_string()));
/// assert_eq!(filter_alphanumeric("Hello 123!"), Some("Hello123".to_string()));
///
/// // Empty string has no alphanumeric chars
/// assert_eq!(filter_alphanumeric(""), None);
///
/// // All non-alphanumeric characters filtered out returns None
/// assert_eq!(filter_alphanumeric("!@# $%^"), None);
/// ```
#[must_use]
#[allow(dead_code)]
pub fn filter_alphanumeric(input: &str) -> Option<String> {
    let filtered: String = input.chars().filter(|c| c.is_alphanumeric()).collect();
    if filtered.is_empty() {
        None
    } else {
        Some(filtered)
    }
}
