//! Power management form component with time input and operation selection.
//!
//! This module provides components for creating power management interfaces, including
//! time-based scheduling for system power operations (suspend, shutdown, reboot, etc.).
//!
//! # Components
//!
//! - [`PowerOperation`] - Enum representing different power management operations
//! - [`PowerForm`] - Form component for entering time duration and selecting time units
//!
//! # Refactored Architecture
//!
//! `PowerForm` now composes [`NumericField`](super::form::NumericField) and
//! [`ComboField`](super::form::ComboField) from the generic form field system
//! rather than duplicating input/validation/clear logic. The form fields handle
//! their own state, validation, and rendering — `PowerForm` orchestrates them
//! and adds the submit button + layout.

use cosmic::{
    Element,
    iced::{Alignment, widget::column},
    theme,
    theme::Button,
    widget::button,
};

use crate::{fl, utils::TimeUnit};

use super::form::{ComboField, FormField, NumericField};

/// System power management operations.
///
/// Represents the different power management actions that can be scheduled
/// or triggered by the application. Each operation has an associated icon,
/// index for UI selection, and localized text.
///
/// # Variants
///
/// - `StayAwake` - Prevent system from sleeping (keep awake mode)
/// - `Suspend` - Suspend system to RAM (sleep mode)
/// - `Shutdown` - Power off the system
/// - `Reboot` - Restart the system
/// - `Logout` - Log out current user session
///
/// # Examples
///
/// ```rust
/// use chronomancer::components::power_form::PowerOperation;
///
/// // Create from radio button index
/// let operation = PowerOperation::from_index(1);
/// assert_eq!(operation, PowerOperation::Suspend);
///
/// // Get operation properties
/// assert_eq!(operation.index(), 1);
/// assert_eq!(operation.icon_name(), "system-suspend-symbolic");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerOperation {
    StayAwake,
    Suspend,
    Shutdown,
    Reboot,
    Logout,
}

impl PowerOperation {
    /// Converts a radio button index to a `PowerOperation`.
    ///
    /// This maps UI selection indices to their corresponding power operations.
    /// Invalid indices default to `Suspend`.
    ///
    /// # Arguments
    ///
    /// - `index` - Zero-based index from radio button selection
    ///
    /// # Returns
    ///
    /// The corresponding `PowerOperation`. Unknown indices return `Suspend` as fallback.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerOperation;
    ///
    /// assert_eq!(PowerOperation::from_index(0), PowerOperation::StayAwake);
    /// assert_eq!(PowerOperation::from_index(1), PowerOperation::Suspend);
    /// assert_eq!(PowerOperation::from_index(2), PowerOperation::Logout);
    /// assert_eq!(PowerOperation::from_index(3), PowerOperation::Reboot);
    /// assert_eq!(PowerOperation::from_index(4), PowerOperation::Shutdown);
    ///
    /// // Invalid index defaults to Suspend
    /// assert_eq!(PowerOperation::from_index(999), PowerOperation::Suspend);
    /// ```
    #[must_use]
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::StayAwake,
            2 => Self::Logout,
            3 => Self::Reboot,
            4 => Self::Shutdown,
            _ => Self::Suspend, // Default fallback (includes index 1)
        }
    }

    /// Gets the radio button index for this operation.
    ///
    /// Returns the zero-based index used for UI selection and radio button positioning.
    ///
    /// # Returns
    ///
    /// The index corresponding to this operation (0-4).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerOperation;
    ///
    /// assert_eq!(PowerOperation::StayAwake.index(), 0);
    /// assert_eq!(PowerOperation::Suspend.index(), 1);
    /// assert_eq!(PowerOperation::Logout.index(), 2);
    /// assert_eq!(PowerOperation::Reboot.index(), 3);
    /// assert_eq!(PowerOperation::Shutdown.index(), 4);
    /// ```
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::StayAwake => 0,
            Self::Suspend => 1,
            Self::Logout => 2,
            Self::Reboot => 3,
            Self::Shutdown => 4,
        }
    }

    /// Gets the system icon name for this operation.
    ///
    /// Returns the freedesktop.org icon name or custom application icon name
    /// that should be displayed for this power operation.
    ///
    /// # Returns
    ///
    /// A static string containing the icon name.
    ///
    /// # Icon Names
    ///
    /// - `StayAwake`: `"io.vulpapps.Chronomancer-stay-awake"` (custom)
    /// - Suspend: `"system-suspend-symbolic"`
    /// - Logout: `"system-log-out-symbolic"`
    /// - Reboot: `"system-reboot-symbolic"`
    /// - Shutdown: `"system-shutdown-symbolic"`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerOperation;
    ///
    /// assert_eq!(
    ///     PowerOperation::StayAwake.icon_name(),
    ///     "io.vulpapps.Chronomancer-stay-awake"
    /// );
    /// assert_eq!(
    ///     PowerOperation::Suspend.icon_name(),
    ///     "system-suspend-symbolic"
    /// );
    /// ```
    #[must_use]
    pub const fn icon_name(self) -> &'static str {
        match self {
            Self::StayAwake => "io.vulpapps.Chronomancer-stay-awake",
            Self::Suspend => "system-suspend-symbolic",
            Self::Logout => "system-log-out-symbolic",
            Self::Reboot => "system-reboot-symbolic",
            Self::Shutdown => "system-shutdown-symbolic",
        }
    }

    /// Gets the localized placeholder text for this operation.
    ///
    /// Returns a localized string suitable for use as placeholder text in
    /// input fields. The text typically prompts the user to enter a time
    /// for the operation.
    ///
    /// # Returns
    ///
    /// Localized placeholder text string. Returns empty string for `StayAwake`
    /// since it doesn't require time input.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::components::power_form::PowerOperation;
    ///
    /// let placeholder = PowerOperation::Suspend.placeholder_text();
    /// // Returns something like "Set time to suspend" (localized)
    ///
    /// // StayAwake doesn't need a placeholder
    /// assert_eq!(PowerOperation::StayAwake.placeholder_text(), "");
    /// ```
    #[must_use]
    pub fn placeholder_text(self) -> String {
        match self {
            Self::StayAwake => String::new(), // No placeholder needed for stay awake
            Self::Suspend => fl!("set-time-label", operation = fl!("operation-suspend")),
            Self::Shutdown => fl!("set-time-label", operation = fl!("operation-shutdown")),
            Self::Reboot => fl!("set-time-label", operation = fl!("operation-reboot")),
            Self::Logout => fl!("set-time-label", operation = fl!("operation-logout")),
        }
    }
}

/// Form component for time duration input with unit selection.
///
/// `PowerForm` composes a [`NumericField`] for duration input and a
/// [`ComboField<TimeUnit>`] for unit selection, adding a submit button
/// and placeholder text management. The composed fields handle their own
/// state, validation, filtering, and rendering.
///
/// # Fields
///
/// - `duration` - Numeric input field for the time value
/// - `time_unit` - Combo box field for selecting the time unit
/// - `placeholder_text` - Placeholder text shown when duration input is empty
///
/// # Validation
///
/// The form validates that:
/// - Duration is a valid positive integer (> 0) — enforced by `NumericField`
/// - Non-numeric input is rejected at keystroke time
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::components::power_form::PowerForm;
/// use chronomancer::utils::TimeUnit;
///
/// let mut form = PowerForm::new("Enter duration");
///
/// // Initially empty
/// assert_eq!(form.input_value(), "");
/// assert_eq!(form.selected_time_unit(), &TimeUnit::Seconds);
///
/// // Validate and handle input
/// form.handle_text_input("30");
/// assert_eq!(form.input_value(), "30");
/// assert!(form.validate_input());
///
/// // Clear when done
/// form.clear();
/// assert_eq!(form.input_value(), "");
/// ```
#[derive(Debug, Clone)]
pub struct PowerForm {
    /// Numeric input field for the duration value.
    duration: NumericField,

    /// Combo box field for time unit selection.
    time_unit: ComboField<TimeUnit>,

    /// Placeholder text displayed in the duration input field.
    pub placeholder_text: String,
}

impl PowerForm {
    /// Creates a new `PowerForm` with the given placeholder text.
    ///
    /// Initializes the form with:
    /// - Empty input value
    /// - Default time unit (Seconds)
    /// - All time unit options available
    /// - Custom placeholder text
    ///
    /// # Arguments
    ///
    /// - `placeholder_text` - Text to show when input is empty (accepts `String`, `&str`, etc.)
    ///
    /// # Returns
    ///
    /// A new `PowerForm` instance ready for use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerForm;
    ///
    /// let form = PowerForm::new("Enter time");
    /// assert_eq!(form.placeholder_text, "Enter time");
    /// assert_eq!(form.input_value(), "");
    /// ```
    pub fn new(placeholder_text: impl Into<String>) -> Self {
        let placeholder: String = placeholder_text.into();
        Self {
            duration: NumericField::new("power-duration").placeholder(&placeholder),
            time_unit: ComboField::new(
                "power-time-unit",
                vec![
                    TimeUnit::Seconds,
                    TimeUnit::Minutes,
                    TimeUnit::Hours,
                    TimeUnit::Days,
                ],
                TimeUnit::Seconds,
            )
            .label(fl!("unit-label")),
            placeholder_text: placeholder,
        }
    }

    /// Returns a reference to the current raw input value string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerForm;
    ///
    /// let form = PowerForm::new("Enter time");
    /// assert_eq!(form.input_value(), "");
    /// ```
    #[must_use]
    #[allow(dead_code)]
    pub fn input_value(&self) -> &str {
        self.duration.raw_value()
    }

    /// Returns a reference to the currently selected time unit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerForm;
    /// use chronomancer::utils::TimeUnit;
    ///
    /// let form = PowerForm::new("Enter time");
    /// assert_eq!(form.selected_time_unit(), &TimeUnit::Seconds);
    /// ```
    #[must_use]
    #[allow(dead_code)]
    pub fn selected_time_unit(&self) -> &TimeUnit {
        self.time_unit.selected()
    }

    /// Sets the selected time unit.
    ///
    /// # Arguments
    ///
    /// - `unit` - The new time unit to select
    pub fn set_time_unit(&mut self, unit: TimeUnit) {
        self.time_unit.update_selection(unit);
    }

    /// Computes the total duration in seconds from the input value and time unit.
    ///
    /// Returns `None` if the input is empty or invalid.
    ///
    /// # Returns
    ///
    /// `Some(i32)` — total seconds, or `None` if the duration field is invalid.
    #[must_use]
    pub fn duration_seconds(&self) -> Option<i32> {
        let value = self.duration.value()?;
        let unit = self.time_unit.value()?;
        Some(value * unit.to_seconds_multiplier())
    }

    /// Renders the power form as an [`Element`].
    ///
    /// Creates a vertical layout containing:
    /// 1. Text input field for duration (rendered by `NumericField`)
    /// 2. Combo box for time unit selection (rendered by `ComboField`)
    /// 3. Submit button
    ///
    /// # Arguments
    ///
    /// - `on_text_input` - Handler called when text input changes
    /// - `on_time_unit` - Handler called when time unit selection changes
    /// - `on_submit` - Message sent when submit button is pressed or Enter is pressed
    ///
    /// # Returns
    ///
    /// An [`Element`] containing the complete form UI.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::components::power_form::PowerForm;
    /// use chronomancer::utils::TimeUnit;
    /// use cosmic::Element;
    ///
    /// #[derive(Clone, Debug)]
    /// enum Message {
    ///     TextChanged(String),
    ///     UnitChanged(TimeUnit),
    ///     Submit,
    /// }
    ///
    /// fn view(form: &PowerForm) -> Element<'_, Message> {
    ///     form.view(
    ///         Message::TextChanged,
    ///         Message::UnitChanged,
    ///         Message::Submit,
    ///     )
    /// }
    /// ```
    pub fn view<Message>(
        &self,
        on_text_input: impl Fn(String) -> Message + 'static,
        on_time_unit: impl Fn(TimeUnit) -> Message + 'static,
        on_submit: Message,
    ) -> Element<'_, Message>
    where
        Message: Clone + 'static,
    {
        let spacing = theme::active().cosmic().spacing;
        let on_submit_clone = on_submit.clone();

        // Duration field with submit-on-enter
        let duration_input = self.duration.view(move |s| {
            // We need to route through on_text_input, but also need on_submit for Enter.
            // Since FormField::view only takes on_input, we handle Enter at the page level.
            on_text_input(s)
        });

        // Time unit combo box using the typed view method
        let unit_combo = self.time_unit.view_typed(on_time_unit);

        // Submit button
        let submit_btn = button::text(fl!("set-button-label"))
            .on_press(on_submit)
            .class(Button::Suggested);

        // Keep the same layout as before: vertical stack
        let _ = on_submit_clone; // Available for future Enter-to-submit support
        column![duration_input, unit_combo, submit_btn]
            .align_x(Alignment::Center)
            .spacing(spacing.space_s)
            .padding([0, spacing.space_l, 0, spacing.space_l])
            .into()
    }

    /// Handles text input changes with numeric validation.
    ///
    /// Delegates to [`NumericField::update`] which uses
    /// [`filters::filter_positive_integer`](crate::utils::filters::filter_positive_integer)
    /// to validate input.
    ///
    /// # Arguments
    ///
    /// - `new_text` - The new text input value to validate and apply
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerForm;
    ///
    /// let mut form = PowerForm::new("Enter time");
    ///
    /// // Valid input
    /// form.handle_text_input("15");
    /// assert_eq!(form.input_value(), "15");
    ///
    /// // Invalid input (no change)
    /// form.handle_text_input("abc");
    /// assert_eq!(form.input_value(), "15");
    ///
    /// // Clear input
    /// form.handle_text_input("");
    /// assert_eq!(form.input_value(), "");
    /// ```
    pub fn handle_text_input(&mut self, new_text: &str) {
        self.duration.update(new_text);
    }

    /// Validates that the current input is a positive integer.
    ///
    /// Delegates to [`NumericField::validate`].
    ///
    /// # Returns
    ///
    /// `true` if input is a positive integer, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerForm;
    ///
    /// let mut form = PowerForm::new("Enter time");
    ///
    /// // Valid input
    /// form.handle_text_input("10");
    /// assert!(form.validate_input());
    ///
    /// // Empty is invalid
    /// form.handle_text_input("");
    /// assert!(!form.validate_input());
    /// ```
    pub fn validate_input(&self) -> bool {
        self.duration.validate()
    }

    /// Clears the form and resets to default state.
    ///
    /// Resets:
    /// - Duration input to empty string
    /// - Time unit to `TimeUnit::Seconds`
    ///
    /// The placeholder text is preserved.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use chronomancer::components::power_form::PowerForm;
    /// use chronomancer::utils::TimeUnit;
    ///
    /// let mut form = PowerForm::new("Enter time");
    /// form.handle_text_input("123");
    /// form.set_time_unit(TimeUnit::Hours);
    ///
    /// form.clear();
    ///
    /// assert_eq!(form.input_value(), "");
    /// assert_eq!(form.selected_time_unit(), &TimeUnit::Seconds);
    /// assert_eq!(form.placeholder_text, "Enter time"); // Preserved
    /// ```
    pub fn clear(&mut self) {
        self.duration.clear();
        self.time_unit.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    enum TestMessage {
        TextChanged(String),
        TimeUnitChanged(TimeUnit),
        Submit,
    }

    #[test]
    fn test_power_form_creation() {
        let form = PowerForm::new("Enter time");
        assert_eq!(form.input_value(), "");
        assert_eq!(form.selected_time_unit(), &TimeUnit::Seconds);
        assert_eq!(form.placeholder_text, "Enter time");
    }

    #[test]
    fn test_handle_text_input_valid() {
        let mut form = PowerForm::new("Enter time");
        form.handle_text_input("15");
        assert_eq!(form.input_value(), "15");
    }

    #[test]
    fn test_handle_text_input_invalid() {
        let mut form = PowerForm::new("Enter time");
        form.handle_text_input("potato");
        assert_eq!(form.input_value(), ""); // Should remain empty
    }

    #[test]
    fn test_validation_valid_input() {
        let mut form = PowerForm::new("Enter time");
        form.handle_text_input("10");
        assert!(form.validate_input());
    }

    #[test]
    fn test_validation_invalid_input() {
        let mut form = PowerForm::new("Enter time");
        assert!(!form.validate_input()); // empty

        form.handle_text_input("0");
        assert!(!form.validate_input()); // zero rejected by filter, still empty
    }

    #[test]
    fn test_clear() {
        let mut form = PowerForm::new("Enter time");
        form.handle_text_input("123");
        form.set_time_unit(TimeUnit::Hours);

        form.clear();

        assert_eq!(form.input_value(), "");
        assert_eq!(form.selected_time_unit(), &TimeUnit::Seconds);
    }

    #[test]
    fn test_view_compiles() {
        let form = PowerForm::new("Enter time");

        // Just verify that the view method compiles and returns an Element
        let _element = form.view(
            TestMessage::TextChanged,
            TestMessage::TimeUnitChanged,
            TestMessage::Submit,
        );
    }

    #[test]
    fn test_power_operation_from_index() {
        assert_eq!(PowerOperation::from_index(0), PowerOperation::StayAwake);
        assert_eq!(PowerOperation::from_index(1), PowerOperation::Suspend);
        assert_eq!(PowerOperation::from_index(2), PowerOperation::Logout);
        assert_eq!(PowerOperation::from_index(3), PowerOperation::Reboot);
        assert_eq!(PowerOperation::from_index(4), PowerOperation::Shutdown);
        // Test fallback for invalid index
        assert_eq!(PowerOperation::from_index(999), PowerOperation::Suspend);
    }

    #[test]
    fn test_power_operation_placeholder_text() {
        use crate::fl;

        // StayAwake has no placeholder
        assert_eq!(PowerOperation::StayAwake.placeholder_text(), "");

        // Others should have localized text (just verify they're not empty)
        assert!(!PowerOperation::Suspend.placeholder_text().is_empty());
        assert!(!PowerOperation::Shutdown.placeholder_text().is_empty());
        assert!(!PowerOperation::Reboot.placeholder_text().is_empty());
        assert!(!PowerOperation::Logout.placeholder_text().is_empty());

        // Verify they contain the operation name
        assert!(
            PowerOperation::Suspend
                .placeholder_text()
                .contains(&fl!("operation-suspend"))
        );
        assert!(
            PowerOperation::Shutdown
                .placeholder_text()
                .contains(&fl!("operation-shutdown"))
        );
        assert!(
            PowerOperation::Reboot
                .placeholder_text()
                .contains(&fl!("operation-reboot"))
        );
        assert!(
            PowerOperation::Logout
                .placeholder_text()
                .contains(&fl!("operation-logout"))
        );
    }

    #[test]
    fn test_power_operation_index() {
        assert_eq!(PowerOperation::StayAwake.index(), 0);
        assert_eq!(PowerOperation::Suspend.index(), 1);
        assert_eq!(PowerOperation::Logout.index(), 2);
        assert_eq!(PowerOperation::Reboot.index(), 3);
        assert_eq!(PowerOperation::Shutdown.index(), 4);
    }

    #[test]
    fn test_power_operation_icon_name() {
        assert_eq!(
            PowerOperation::StayAwake.icon_name(),
            "io.vulpapps.Chronomancer-stay-awake"
        );
        assert_eq!(
            PowerOperation::Suspend.icon_name(),
            "system-suspend-symbolic"
        );
        assert_eq!(
            PowerOperation::Logout.icon_name(),
            "system-log-out-symbolic"
        );
        assert_eq!(PowerOperation::Reboot.icon_name(), "system-reboot-symbolic");
        assert_eq!(
            PowerOperation::Shutdown.icon_name(),
            "system-shutdown-symbolic"
        );
    }

    #[test]
    fn test_set_time_unit() {
        let mut form = PowerForm::new("Enter time");
        form.set_time_unit(TimeUnit::Minutes);
        assert_eq!(form.selected_time_unit(), &TimeUnit::Minutes);

        form.set_time_unit(TimeUnit::Hours);
        assert_eq!(form.selected_time_unit(), &TimeUnit::Hours);
    }

    #[test]
    fn test_duration_seconds() {
        let mut form = PowerForm::new("Enter time");

        // No input — None
        assert_eq!(form.duration_seconds(), None);

        // 30 seconds
        form.handle_text_input("30");
        assert_eq!(form.duration_seconds(), Some(30));

        // 5 minutes
        form.set_time_unit(TimeUnit::Minutes);
        form.handle_text_input("5");
        assert_eq!(form.duration_seconds(), Some(300));

        // 2 hours
        form.set_time_unit(TimeUnit::Hours);
        form.handle_text_input("2");
        assert_eq!(form.duration_seconds(), Some(7200));

        // 1 day
        form.set_time_unit(TimeUnit::Days);
        form.handle_text_input("1");
        assert_eq!(form.duration_seconds(), Some(86400));
    }

    #[test]
    fn test_clear_preserves_placeholder() {
        let mut form = PowerForm::new("My placeholder");
        form.handle_text_input("42");
        form.set_time_unit(TimeUnit::Days);
        form.clear();
        assert_eq!(form.placeholder_text, "My placeholder");
    }
}
