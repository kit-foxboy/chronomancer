//! Dropdown selection field implementing [`FormField`].
//!
//! [`ComboField`] wraps a `cosmic::widget::ComboBox` to provide a typed
//! dropdown selection from a list of options. It is generic over any type
//! that implements the necessary traits for display and comparison.
//!
//! Unlike [`TextField`](super::TextField) and [`NumericField`](super::NumericField),
//! `ComboField` always has a valid selection (it defaults to the first option),
//! so [`validate()`](super::FormField::validate) always returns `true` and
//! [`value()`](super::FormField::value) always returns `Some(T)`.
//!
//! # Type Requirements
//!
//! The type parameter `T` must implement:
//! - `Clone` — for extracting values and combo box state
//! - `Debug` — for development/debugging
//! - `Display` — for rendering option labels in the combo box
//! - `PartialEq` — for comparing the selected value
//! - `'static` — required by cosmic's combo box widget
//!
//! # Examples
//!
//! ```ignore
//! use crate::components::form::{FormField, ComboField};
//! use crate::utils::TimeUnit;
//!
//! let mut field = ComboField::new(
//!     "time-unit",
//!     vec![TimeUnit::Seconds, TimeUnit::Minutes, TimeUnit::Hours, TimeUnit::Days],
//!     TimeUnit::Seconds,
//! ).label("Unit");
//!
//! // Default selection
//! assert_eq!(field.value(), Some(TimeUnit::Seconds));
//!
//! // Change selection
//! field.update_selection(TimeUnit::Minutes);
//! assert_eq!(field.value(), Some(TimeUnit::Minutes));
//!
//! // Always valid
//! assert!(field.validate());
//!
//! // Clear resets to default
//! field.clear();
//! assert_eq!(field.value(), Some(TimeUnit::Seconds));
//! ```

use std::fmt::{Debug, Display};

use cosmic::{
    Element,
    iced::Length::Fill,
    widget::{ComboBox, combo_box},
};

use super::FormField;

/// A dropdown selection field backed by a combo box.
///
/// Wraps `cosmic::widget::ComboBox` with the [`FormField`] interface, providing
/// type-safe selection from a fixed list of options. The field always has a
/// valid selection — there is no "empty" state.
///
/// # Generic Parameters
///
/// - `T` — the option type displayed in the dropdown. Must implement `Clone`,
///   `Debug`, `Display`, and `PartialEq`.
///
/// # Fields
///
/// - `id` — Unique identifier (used for the combo box label if no label is set)
/// - `label` — Display label shown as placeholder text in the combo box
/// - `options` — Combo box state containing the available options
/// - `selected` — The currently selected value
/// - `default` — The default value restored on [`clear()`](FormField::clear)
///
/// # Examples
///
/// ```ignore
/// let mut unit = ComboField::new(
///     "unit",
///     vec![TimeUnit::Seconds, TimeUnit::Minutes],
///     TimeUnit::Seconds,
/// );
/// assert_eq!(unit.value(), Some(TimeUnit::Seconds));
///
/// unit.update_selection(TimeUnit::Minutes);
/// assert_eq!(unit.value(), Some(TimeUnit::Minutes));
/// ```
#[derive(Debug, Clone)]
pub struct ComboField<T>
where
    T: Clone + Debug + Display + PartialEq + 'static,
{
    /// Widget identifier.
    #[allow(dead_code)]
    id: String,

    /// Display label for the combo box placeholder.
    label: String,

    /// Combo box widget state containing the option list.
    options: combo_box::State<T>,

    /// The currently selected value.
    selected: T,

    /// The default value, restored on [`clear()`](FormField::clear).
    default: T,
}

impl<T> ComboField<T>
where
    T: Clone + Debug + Display + PartialEq + 'static,
{
    /// Creates a new `ComboField` with the given options and default selection.
    ///
    /// The `default` value is used as the initial selection and is restored
    /// when [`clear()`](FormField::clear) is called.
    ///
    /// # Arguments
    ///
    /// - `id` - Unique identifier for this field
    /// - `options` - List of selectable options
    /// - `default` - The default (and initial) selected value
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let field = ComboField::new(
    ///     "unit",
    ///     vec![TimeUnit::Seconds, TimeUnit::Minutes, TimeUnit::Hours],
    ///     TimeUnit::Seconds,
    /// );
    /// ```
    #[must_use]
    pub fn new(id: impl Into<String>, options: Vec<T>, default: T) -> Self {
        Self {
            id: id.into(),
            label: String::new(),
            options: combo_box::State::new(options),
            selected: default.clone(),
            default,
        }
    }

    /// Sets the display label shown as placeholder text in the combo box.
    ///
    /// If not set, an empty string is used as the placeholder.
    ///
    /// # Arguments
    ///
    /// - `label` - The label text
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let field = ComboField::new("unit", options, default)
    ///     .label("Time Unit");
    /// ```
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Updates the selected value directly.
    ///
    /// This is the typed companion to [`update()`](FormField::update). Use this
    /// when you have a `T` value (e.g., from the combo box's `on_selected`
    /// callback) rather than a raw string.
    ///
    /// # Arguments
    ///
    /// - `value` - The new selected value
    ///
    /// # Examples
    ///
    /// ```ignore
    /// field.update_selection(TimeUnit::Hours);
    /// assert_eq!(field.value(), Some(TimeUnit::Hours));
    /// ```
    pub fn update_selection(&mut self, value: T) {
        self.selected = value;
    }

    /// Returns a reference to the currently selected value.
    ///
    /// Unlike [`value()`](FormField::value) which returns an `Option<T>`,
    /// this always returns a reference since the field always has a selection.
    #[must_use]
    #[allow(dead_code)]
    pub fn selected(&self) -> &T {
        &self.selected
    }

    /// Renders the combo box as an [`Element`], mapping selection events via the closure.
    ///
    /// This is the typed view method — the `on_select` closure receives the
    /// selected `T` value directly, matching `cosmic::widget::ComboBox`'s API.
    ///
    /// Use this instead of [`FormField::view()`] when you want type-safe
    /// selection messages rather than raw `String` messages.
    ///
    /// # Arguments
    ///
    /// - `on_select` - Closure mapping selected `T` to the consumer's message type
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let element = unit_field.view_typed(MyMessage::UnitChanged);
    /// ```
    #[must_use]
    pub fn view_typed<M: Clone + 'static>(
        &self,
        on_select: impl Fn(T) -> M + 'static,
    ) -> Element<'_, M> {
        ComboBox::new(&self.options, &self.label, Some(&self.selected), on_select)
            .width(Fill)
            .into()
    }
}

impl<T> FormField for ComboField<T>
where
    T: Clone + Debug + Display + PartialEq + 'static,
{
    type Value = T;

    /// Renders the combo box as an [`Element`].
    ///
    /// **Note:** For combo boxes, the `on_input` closure receives the
    /// `Display` representation of the selected item as a `String`. In most
    /// cases, prefer [`view_typed()`](ComboField::view_typed) which provides
    /// the actual `T` value directly.
    ///
    /// This implementation exists to satisfy the [`FormField`] trait contract
    /// for uniform field handling.
    ///
    /// # Arguments
    ///
    /// - `on_input` - Closure mapping the display string to the consumer's message type
    fn view<M: Clone + 'static>(&self, on_input: impl Fn(String) -> M + 'static) -> Element<'_, M> {
        ComboBox::new(
            &self.options,
            &self.label,
            Some(&self.selected),
            move |value: T| on_input(value.to_string()),
        )
        .width(Fill)
        .into()
    }

    /// No-op for combo fields.
    ///
    /// Combo box selections are handled through [`update_selection()`](ComboField::update_selection)
    /// with the typed value, not through raw string input. This method exists
    /// to satisfy the [`FormField`] trait contract but does not modify state.
    fn update(&mut self, _input: &str) {
        // Combo boxes don't receive raw string input — selections are handled
        // via update_selection() with the typed T value.
    }

    /// Always returns `true` — a combo field always has a valid selection.
    fn validate(&self) -> bool {
        true
    }

    /// Returns the currently selected value.
    ///
    /// Always returns `Some(T)` since a combo field always has a selection.
    fn value(&self) -> Option<T> {
        Some(self.selected.clone())
    }

    /// Resets the selection to the default value provided at construction.
    fn clear(&mut self) {
        self.selected = self.default.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestOption {
        Alpha,
        Beta,
        Gamma,
    }

    impl Display for TestOption {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestOption::Alpha => write!(f, "Alpha"),
                TestOption::Beta => write!(f, "Beta"),
                TestOption::Gamma => write!(f, "Gamma"),
            }
        }
    }

    fn test_options() -> Vec<TestOption> {
        vec![TestOption::Alpha, TestOption::Beta, TestOption::Gamma]
    }

    #[test]
    fn test_new_defaults() {
        let field = ComboField::new("test", test_options(), TestOption::Alpha);
        assert_eq!(field.selected, TestOption::Alpha);
        assert_eq!(field.default, TestOption::Alpha);
    }

    #[test]
    fn test_new_with_non_first_default() {
        let field = ComboField::new("test", test_options(), TestOption::Beta);
        assert_eq!(field.selected, TestOption::Beta);
        assert_eq!(field.default, TestOption::Beta);
    }

    #[test]
    fn test_label() {
        let field = ComboField::new("test", test_options(), TestOption::Alpha).label("Pick one");
        assert_eq!(field.label, "Pick one");
    }

    #[test]
    fn test_update_selection() {
        let mut field = ComboField::new("test", test_options(), TestOption::Alpha);
        field.update_selection(TestOption::Gamma);
        assert_eq!(field.selected, TestOption::Gamma);
    }

    #[test]
    fn test_selected() {
        let field = ComboField::new("test", test_options(), TestOption::Beta);
        assert_eq!(*field.selected(), TestOption::Beta);
    }

    #[test]
    fn test_validate_always_true() {
        let field = ComboField::new("test", test_options(), TestOption::Alpha);
        assert!(field.validate());
    }

    #[test]
    fn test_value() {
        let mut field = ComboField::new("test", test_options(), TestOption::Alpha);
        assert_eq!(field.value(), Some(TestOption::Alpha));

        field.update_selection(TestOption::Gamma);
        assert_eq!(field.value(), Some(TestOption::Gamma));
    }

    #[test]
    fn test_clear_resets_to_default() {
        let mut field = ComboField::new("test", test_options(), TestOption::Alpha);
        field.update_selection(TestOption::Gamma);
        field.clear();
        assert_eq!(field.selected, TestOption::Alpha);
        assert_eq!(field.value(), Some(TestOption::Alpha));
    }

    #[test]
    fn test_clear_with_non_first_default() {
        let mut field = ComboField::new("test", test_options(), TestOption::Beta);
        field.update_selection(TestOption::Alpha);
        field.clear();
        assert_eq!(field.selected, TestOption::Beta);
    }

    #[test]
    fn test_update_raw_string_is_noop() {
        let mut field = ComboField::new("test", test_options(), TestOption::Alpha);
        field.update("Beta");
        // Raw string update does nothing for combo fields
        assert_eq!(field.selected, TestOption::Alpha);
    }

    #[test]
    fn test_builder_chaining() {
        let field =
            ComboField::new("my-combo", test_options(), TestOption::Alpha).label("Choose option");
        assert_eq!(field.id, "my-combo");
        assert_eq!(field.label, "Choose option");
        assert_eq!(field.selected, TestOption::Alpha);
    }
}
