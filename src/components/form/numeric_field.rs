//! Positive integer input field implementing [`FormField`].
//!
//! [`NumericField`] captures numeric input, filtering out non-numeric characters
//! and rejecting zero/negative values using [`filters::filter_positive_integer`].
//! It extracts an `i32` on submission.
//!
//! # Examples
//!
//! ```ignore
//! use crate::components::form::{FormField, NumericField};
//!
//! let mut field = NumericField::new("duration")
//!     .placeholder("Duration...");
//!
//! // Accepts valid positive integers
//! field.update("42");
//! assert!(field.validate());
//! assert_eq!(field.value(), Some(42));
//!
//! // Rejects non-numeric input (no change)
//! field.update("abc");
//! assert_eq!(field.value(), Some(42));
//!
//! // Rejects zero
//! field.clear();
//! field.update("0");
//! assert!(!field.validate());
//! assert_eq!(field.value(), None);
//! ```

use cosmic::{Element, iced::Length::Fill, widget::text_input};

use super::FormField;
use crate::utils::filters;

/// A numeric input field that only accepts positive integers.
///
/// Uses [`filters::filter_positive_integer`] to reject invalid input at
/// keystroke time — non-numeric characters, zero, and negative values are
/// silently ignored. The raw value is stored as a `String` (matching how
/// text inputs work), but [`value()`](FormField::value) extracts an `i32`.
///
/// # Fields
///
/// - `id` — Unique identifier used as the widget ID for focus management
/// - `placeholder` — Placeholder text shown when the input is empty
/// - `input_value` — Current raw numeric string value
///
/// # Examples
///
/// ```ignore
/// let mut duration = NumericField::new("duration").placeholder("Enter time...");
/// duration.update("30");
/// assert_eq!(duration.value(), Some(30));
/// ```
#[derive(Debug, Clone)]
pub struct NumericField {
    /// Widget identifier, used for input element ID.
    #[allow(dead_code)]
    id: String,

    /// Placeholder text displayed when the input is empty.
    placeholder: String,

    /// The current raw input value (always a valid positive integer string or empty).
    input_value: String,
}

impl NumericField {
    /// Creates a new `NumericField` with the given ID.
    ///
    /// The ID is used as the widget identifier for focus management and
    /// accessibility. Defaults to no placeholder and an empty value.
    ///
    /// # Arguments
    ///
    /// - `id` - Unique identifier for this field
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let field = NumericField::new("timer-duration");
    /// ```
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            placeholder: String::new(),
            input_value: String::new(),
        }
    }

    /// Sets the placeholder text displayed when the input is empty.
    ///
    /// # Arguments
    ///
    /// - `placeholder` - Text to show as placeholder
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let field = NumericField::new("duration")
    ///     .placeholder("Enter duration...");
    /// ```
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Returns a reference to the current raw input string.
    ///
    /// This is the string representation of the number. For the parsed
    /// integer value, use [`value()`](FormField::value) instead.
    #[must_use]
    #[allow(dead_code)]
    pub fn raw_value(&self) -> &str {
        &self.input_value
    }
}

impl FormField for NumericField {
    type Value = i32;

    /// Renders the numeric input field as an [`Element`].
    ///
    /// The `on_input` closure maps the raw input `String` to the consumer's
    /// message type. Note that the string passed to `on_input` is the raw
    /// keystroke value *before* filtering — the consumer should route it
    /// through [`update()`](FormField::update) which applies the filter.
    ///
    /// # Arguments
    ///
    /// - `on_input` - Closure mapping raw input to the consumer's message type
    fn view<M: Clone + 'static>(&self, on_input: impl Fn(String) -> M + 'static) -> Element<'_, M> {
        text_input(&self.placeholder, &self.input_value)
            .on_input(on_input)
            .width(Fill)
            .into()
    }

    /// Updates the field with new input, applying positive integer filtering.
    ///
    /// Uses [`filters::filter_positive_integer`] to validate. Only updates
    /// the stored value if the input passes the filter. Invalid input
    /// (non-numeric, zero, negative) is silently ignored — the previous
    /// value is preserved.
    ///
    /// Empty input is accepted to allow clearing the field via backspace.
    ///
    /// # Arguments
    ///
    /// - `input` - Raw input string from the widget callback
    fn update(&mut self, input: &str) {
        if let Some(filtered) = filters::filter_positive_integer(input) {
            self.input_value = filtered;
        }
    }

    /// Returns `true` if the input contains a valid positive integer (> 0).
    fn validate(&self) -> bool {
        self.input_value.parse::<i32>().is_ok_and(|v| v > 0)
    }

    /// Extracts the integer value if valid.
    ///
    /// Returns `Some(i32)` when the field contains a parseable positive
    /// integer, or `None` if the field is empty or invalid.
    fn value(&self) -> Option<i32> {
        let parsed = self.input_value.parse::<i32>().ok()?;
        if parsed > 0 { Some(parsed) } else { None }
    }

    /// Clears the input value to an empty string.
    fn clear(&mut self) {
        self.input_value.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_defaults() {
        let field = NumericField::new("test");
        assert_eq!(field.id, "test");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_value, "");
    }

    #[test]
    fn test_placeholder() {
        let field = NumericField::new("test").placeholder("Duration...");
        assert_eq!(field.placeholder, "Duration...");
    }

    #[test]
    fn test_update_valid() {
        let mut field = NumericField::new("test");
        field.update("42");
        assert_eq!(field.raw_value(), "42");
    }

    #[test]
    fn test_update_rejects_non_numeric() {
        let mut field = NumericField::new("test");
        field.update("15");
        field.update("abc");
        assert_eq!(field.raw_value(), "15"); // Unchanged
    }

    #[test]
    fn test_update_rejects_zero() {
        let mut field = NumericField::new("test");
        field.update("0");
        assert_eq!(field.raw_value(), ""); // Rejected, stays empty
    }

    #[test]
    fn test_update_rejects_negative() {
        let mut field = NumericField::new("test");
        field.update("-5");
        assert_eq!(field.raw_value(), ""); // Rejected
    }

    #[test]
    fn test_update_accepts_empty() {
        let mut field = NumericField::new("test");
        field.update("15");
        field.update("");
        assert_eq!(field.raw_value(), ""); // Cleared via backspace
    }

    #[test]
    fn test_update_normalizes_leading_zeros() {
        let mut field = NumericField::new("test");
        field.update("007");
        assert_eq!(field.raw_value(), "7"); // Normalized by filter
    }

    #[test]
    fn test_validate_valid() {
        let mut field = NumericField::new("test");
        field.update("10");
        assert!(field.validate());
    }

    #[test]
    fn test_validate_empty() {
        let field = NumericField::new("test");
        assert!(!field.validate());
    }

    #[test]
    fn test_validate_after_clear() {
        let mut field = NumericField::new("test");
        field.update("10");
        field.update("");
        assert!(!field.validate());
    }

    #[test]
    fn test_value_valid() {
        let mut field = NumericField::new("test");
        field.update("30");
        assert_eq!(field.value(), Some(30));
    }

    #[test]
    fn test_value_empty() {
        let field = NumericField::new("test");
        assert_eq!(field.value(), None);
    }

    #[test]
    fn test_value_large_number() {
        let mut field = NumericField::new("test");
        field.update("86400");
        assert_eq!(field.value(), Some(86400));
    }

    #[test]
    fn test_clear() {
        let mut field = NumericField::new("test");
        field.update("42");
        field.clear();
        assert_eq!(field.raw_value(), "");
        assert_eq!(field.value(), None);
        assert!(!field.validate());
    }

    #[test]
    fn test_builder_chaining() {
        let field = NumericField::new("my-field").placeholder("Enter number...");
        assert_eq!(field.id, "my-field");
        assert_eq!(field.placeholder, "Enter number...");
    }

    #[test]
    fn test_update_rejects_decimal() {
        let mut field = NumericField::new("test");
        field.update("3.14");
        assert_eq!(field.raw_value(), ""); // Rejected
    }

    #[test]
    fn test_update_rejects_special_chars() {
        let mut field = NumericField::new("test");
        field.update("12abc34");
        assert_eq!(field.raw_value(), ""); // Rejected
    }
}
