//! Text input filter functions for validation and formatting.
//!
//! This module provides reusable filter functions that can be used to validate
//! and format text input. These are designed to be used with libcosmic's
//! `TextInput` component or any text input handling.
//!
//! # Design Philosophy
//!
//! These are simple, pure functions that:
//! - Take an input string
//! - Return `Some(String)` if valid (possibly filtered/formatted)
//! - Return `None` if invalid
//!
//! The calling code decides what to do with invalid input (ignore it, show error, etc.).
//!
//! # Examples
//!
//! ## Basic usage in a component
//!
//! ```rust
//! use chronomancer::utils::filters;
//!
//! struct MyForm {
//!     count: String,
//! }
//!
//! impl MyForm {
//!     pub fn handle_input(&mut self, new_text: &str) {
//!         if let Some(filtered) = filters::filter_positive_integer(new_text) {
//!             self.count = filtered;
//!         }
//!         // If None, we just ignore the input (keep old value)
//!     }
//! }
//! ```
//!
//! ## Direct usage in message handler
//!
//! ```rust,no_run
//! use chronomancer::utils::filters;
//!
//! # struct App { value: String }
//! # enum Message { TextChanged(String) }
//! # impl App {
//! fn update(&mut self, message: Message) {
//!     match message {
//!         Message::TextChanged(text) => {
//!             if let Some(filtered) = filters::filter_positive_integer(&text) {
//!                 self.value = filtered;
//!             }
//!         }
//!     }
//! }
//! # }
//! ```

/// Filters input to only accept positive integers (> 0).
///
/// This function:
/// - Accepts empty strings (returns `Some("")`)
/// - Parses input as `u32` and rejects 0
/// - Normalizes the input by re-formatting the parsed number
/// - Rejects negative numbers, decimals, and non-numeric input
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
/// // Valid positive integers
/// assert_eq!(filter_positive_integer("42"), Some("42".to_string()));
/// assert_eq!(filter_positive_integer("007"), Some("7".to_string())); // Normalized
///
/// // Empty string is allowed
/// assert_eq!(filter_positive_integer(""), Some("".to_string()));
///
/// // Invalid inputs
/// assert_eq!(filter_positive_integer("0"), None);
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
/// assert_eq!(filter_alphabetic(""), Some("".to_string()));
///
/// // All non-alphabetic removed
/// assert_eq!(filter_alphabetic("123!@#"), Some("".to_string()));
/// ```
#[must_use]
#[allow(dead_code)]
pub fn filter_alphabetic(input: &str) -> Option<String> {
    let filtered: String = input.chars().filter(|c| c.is_alphabetic()).collect();
    Some(filtered)
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
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::filters::filter_alphanumeric;
///
/// assert_eq!(filter_alphanumeric("Hello123"), Some("Hello123".to_string()));
/// assert_eq!(filter_alphanumeric("Hello 123!"), Some("Hello123".to_string()));
/// assert_eq!(filter_alphanumeric("user_name"), Some("username".to_string()));
/// ```
#[must_use]
#[allow(dead_code)]
pub fn filter_alphanumeric(input: &str) -> Option<String> {
    let filtered: String = input.chars().filter(|c| c.is_alphanumeric()).collect();
    Some(filtered)
}
