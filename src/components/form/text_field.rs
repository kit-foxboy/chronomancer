//! Freeform text input field implementing [`FormField`].
//!
//! [`TextField`] captures arbitrary text input with optional placeholder text.
//! It validates that the input is non-empty (after trimming whitespace) and
//! extracts a trimmed `String` on submission.
//!
//! # Examples
//!
//! ```ignore
//! use crate::components::form::{FormField, TextField};
//!
//! let mut field = TextField::new("timer-name")
//!     .placeholder("Enter timer name...");
//!
//! // Accepts any text
//! field.update("My Timer");
//! assert!(field.validate());
//! assert_eq!(field.value(), Some("My Timer".to_string()));
//!
//! // Whitespace-only is invalid
//! field.update("   ");
//! assert!(!field.validate());
//! assert_eq!(field.value(), None);
//!
//! // Clear resets to empty
//! field.clear();
//! assert_eq!(field.value(), None);
//! ```

use cosmic::{Element, iced::Length::Fill, widget::text_input};

use super::FormField;

/// A freeform text input field.
///
/// Accepts any text input without filtering. Validates that the trimmed
/// value is non-empty. Extracts a trimmed `String` on submission.
///
/// # Fields
///
/// - `id` — Unique identifier used as the widget ID for focus management
/// - `placeholder` — Placeholder text shown when the input is empty
/// - `input_value` — Current raw text value
///
/// # Examples
///
/// ```ignore
/// let mut name = TextField::new("name").placeholder("Timer name...");
/// name.update("  My Timer  ");
/// assert_eq!(name.value(), Some("My Timer".to_string())); // Trimmed
/// ```
#[derive(Debug, Clone)]
pub struct TextField {
    /// Widget identifier, used for input element ID.
    #[allow(dead_code)]
    id: String,

    /// Placeholder text displayed when the input is empty.
    placeholder: String,

    /// The current raw input value.
    input_value: String,
}

impl TextField {
    /// Creates a new `TextField` with the given ID.
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
    /// let field = TextField::new("timer-name");
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
    /// let field = TextField::new("name")
    ///     .placeholder("Enter name...");
    /// ```
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Returns a reference to the current raw input value.
    ///
    /// This returns the untrimmed value. For the trimmed, validated value,
    /// use [`value()`](FormField::value) instead.
    #[must_use]
    #[allow(dead_code)]
    pub fn raw_value(&self) -> &str {
        &self.input_value
    }
}

impl FormField for TextField {
    type Value = String;

    /// Renders the text input field as an [`Element`].
    ///
    /// The `on_input` closure maps the raw input `String` to the consumer's
    /// message type, following libcosmic's `TextInput::on_input` pattern.
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

    /// Updates the field with new text input.
    ///
    /// Accepts any text without filtering. The value is stored as-is;
    /// trimming only happens at validation/extraction time.
    ///
    /// # Arguments
    ///
    /// - `input` - The new text value
    fn update(&mut self, input: &str) {
        self.input_value = input.to_string();
    }

    /// Returns `true` if the trimmed input is non-empty.
    fn validate(&self) -> bool {
        !self.input_value.trim().is_empty()
    }

    /// Extracts the trimmed text value if non-empty.
    ///
    /// Returns `Some(String)` with leading/trailing whitespace removed,
    /// or `None` if the trimmed value is empty.
    fn value(&self) -> Option<String> {
        let trimmed = self.input_value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
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
        let field = TextField::new("test");
        assert_eq!(field.id, "test");
        assert_eq!(field.placeholder, "");
        assert_eq!(field.input_value, "");
    }

    #[test]
    fn test_placeholder() {
        let field = TextField::new("test").placeholder("Enter name...");
        assert_eq!(field.placeholder, "Enter name...");
    }

    #[test]
    fn test_update() {
        let mut field = TextField::new("test");
        field.update("Hello World");
        assert_eq!(field.raw_value(), "Hello World");
    }

    #[test]
    fn test_update_accepts_any_text() {
        let mut field = TextField::new("test");
        field.update("Timer #1 — important! @home");
        assert_eq!(field.raw_value(), "Timer #1 — important! @home");
    }

    #[test]
    fn test_update_accepts_empty() {
        let mut field = TextField::new("test");
        field.update("something");
        field.update("");
        assert_eq!(field.raw_value(), "");
    }

    #[test]
    fn test_validate_non_empty() {
        let mut field = TextField::new("test");
        field.update("Hello");
        assert!(field.validate());
    }

    #[test]
    fn test_validate_empty() {
        let field = TextField::new("test");
        assert!(!field.validate());
    }

    #[test]
    fn test_validate_whitespace_only() {
        let mut field = TextField::new("test");
        field.update("   ");
        assert!(!field.validate());
    }

    #[test]
    fn test_value_valid() {
        let mut field = TextField::new("test");
        field.update("My Timer");
        assert_eq!(field.value(), Some("My Timer".to_string()));
    }

    #[test]
    fn test_value_trims_whitespace() {
        let mut field = TextField::new("test");
        field.update("  My Timer  ");
        assert_eq!(field.value(), Some("My Timer".to_string()));
    }

    #[test]
    fn test_value_empty() {
        let field = TextField::new("test");
        assert_eq!(field.value(), None);
    }

    #[test]
    fn test_value_whitespace_only() {
        let mut field = TextField::new("test");
        field.update("   ");
        assert_eq!(field.value(), None);
    }

    #[test]
    fn test_clear() {
        let mut field = TextField::new("test");
        field.update("Some text");
        field.clear();
        assert_eq!(field.raw_value(), "");
        assert_eq!(field.value(), None);
        assert!(!field.validate());
    }

    #[test]
    fn test_builder_chaining() {
        let field = TextField::new("my-field").placeholder("Type here...");
        assert_eq!(field.id, "my-field");
        assert_eq!(field.placeholder, "Type here...");
    }
}
