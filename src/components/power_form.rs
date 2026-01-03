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

use cosmic::{
    Element,
    iced::{Alignment, Length::Fill, widget::column},
    theme::Button,
    widget::{ComboBox, TextInput, button, combo_box},
};

use crate::{
    fl,
    utils::{Padding, TimeUnit, filters, ui::Gaps},
};

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
/// `PowerForm` provides a complete input interface for specifying time durations,
/// combining a numeric text input field with a combo box for selecting time units
/// (seconds, minutes, hours, days) and a submit button.
///
/// # Fields
///
/// - `input_value` - Current numeric value as a string
/// - `time_unit` - Selected time unit (seconds, minutes, hours, days)
/// - `time_unit_options` - Combo box state for unit selection
/// - `placeholder_text` - Placeholder text shown when input is empty
///
/// # Validation
///
/// The form validates that:
/// - Input is a valid positive integer (> 0)
/// - Non-numeric input is rejected
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
/// assert_eq!(form.input_value, "");
/// assert_eq!(form.time_unit, TimeUnit::Seconds);
///
/// // Validate and handle input
/// form.handle_text_input("30");
/// assert_eq!(form.input_value, "30");
/// assert!(form.validate_input());
///
/// // Clear when done
/// form.clear();
/// assert_eq!(form.input_value, "");
/// ```
#[derive(Debug, Clone)]
pub struct PowerForm {
    /// The current numeric input value as a string.
    pub input_value: String,

    /// The currently selected time unit.
    pub time_unit: TimeUnit,

    /// State for the time unit combo box.
    pub time_unit_options: combo_box::State<TimeUnit>,

    /// Placeholder text displayed in the input field.
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
    /// assert_eq!(form.input_value, "");
    /// ```
    pub fn new(placeholder_text: impl Into<String>) -> Self {
        Self {
            input_value: String::new(),
            time_unit: TimeUnit::Seconds, // Default to seconds
            time_unit_options: combo_box::State::new(vec![
                TimeUnit::Seconds,
                TimeUnit::Minutes,
                TimeUnit::Hours,
                TimeUnit::Days,
            ]),
            placeholder_text: placeholder_text.into(),
        }
    }

    /// Renders the power form as an [`Element`].
    ///
    /// Creates a vertical layout containing:
    /// 1. Text input field for duration
    /// 2. Combo box for time unit selection
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
        let on_submit_clone = on_submit.clone();
        column![
            TextInput::new(&self.placeholder_text, &self.input_value)
                .on_input(on_text_input)
                .on_submit(move |_| on_submit_clone.clone())
                .width(Fill),
            ComboBox::new(
                &self.time_unit_options,
                &fl!("unit-label"),
                Some(&self.time_unit),
                on_time_unit,
            )
            .width(Fill),
            button::text(fl!("set-button-label"))
                .on_press(on_submit)
                .class(Button::Suggested)
        ]
        .align_x(Alignment::Center)
        .spacing(Gaps::s())
        .padding(Padding::horizontal(24))
        .into()
    }

    /// Handles text input changes with numeric validation.
    ///
    /// Uses [`filters::filter_positive_integer`] to validate input.
    /// Only accepts valid positive integers. Rejects:
    /// - Non-numeric characters
    /// - Negative numbers
    /// - Zero
    ///
    /// Empty input is accepted to allow clearing the field.
    ///
    /// # Arguments
    ///
    /// - `new_text` - The new text input value to validate and apply
    ///
    /// # Behavior
    ///
    /// - Valid positive integer: Updates `input_value`
    /// - Empty string: Clears `input_value`
    /// - Invalid input: No change to `input_value`
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
    /// assert_eq!(form.input_value, "15");
    ///
    /// // Invalid input (no change)
    /// form.handle_text_input("abc");
    /// assert_eq!(form.input_value, "15");
    ///
    /// // Clear input
    /// form.handle_text_input("");
    /// assert_eq!(form.input_value, "");
    /// ```
    pub fn handle_text_input(&mut self, new_text: &str) {
        if let Some(filtered) = filters::filter_positive_integer(new_text) {
            self.input_value = filtered;
        }
    }

    /// Validates that the current input is a positive integer.
    ///
    /// Checks whether `input_value` contains a valid positive integer (> 0).
    /// Returns `false` for:
    /// - Empty strings
    /// - Non-numeric values
    /// - Zero
    /// - Negative numbers
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
    /// form.input_value = "10".to_string();
    /// assert!(form.validate_input());
    ///
    /// // Invalid inputs
    /// form.input_value = "0".to_string();
    /// assert!(!form.validate_input());
    ///
    /// form.input_value = "-5".to_string();
    /// assert!(!form.validate_input());
    ///
    /// form.input_value = String::new();
    /// assert!(!form.validate_input());
    /// ```
    pub fn validate_input(&self) -> bool {
        let value = self.input_value.parse::<i32>();
        value.is_ok() && value.unwrap_or_default() > 0
    }

    /// Clears the form and resets to default state.
    ///
    /// Resets:
    /// - `input_value` to empty string
    /// - `time_unit` to `TimeUnit::Seconds`
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
    /// form.input_value = "123".to_string();
    /// form.time_unit = TimeUnit::Hours;
    ///
    /// form.clear();
    ///
    /// assert_eq!(form.input_value, "");
    /// assert_eq!(form.time_unit, TimeUnit::Seconds);
    /// assert_eq!(form.placeholder_text, "Enter time"); // Preserved
    /// ```
    pub fn clear(&mut self) {
        self.input_value.clear();
        self.time_unit = TimeUnit::Seconds;
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
        assert_eq!(form.input_value, "");
        assert_eq!(form.time_unit, TimeUnit::Seconds);
        assert_eq!(form.placeholder_text, "Enter time");
    }

    #[test]
    fn test_handle_text_input_valid() {
        let mut form = PowerForm::new("Enter time");
        form.handle_text_input("15");
        assert_eq!(form.input_value, "15");
    }

    #[test]
    fn test_handle_text_input_invalid() {
        let mut form = PowerForm::new("Enter time");
        form.handle_text_input("potato");
        assert_eq!(form.input_value, ""); // Should remain empty
    }

    #[test]
    fn test_validation_valid_input() {
        let mut form = PowerForm::new("Enter time");
        form.input_value = "10".to_string();
        assert!(form.validate_input());
    }

    #[test]
    fn test_validation_invalid_input() {
        let mut form = PowerForm::new("Enter time");
        form.input_value = "0".to_string();
        assert!(!form.validate_input());

        form.input_value = "-5".to_string();
        assert!(!form.validate_input());

        form.input_value = String::new();
        assert!(!form.validate_input());
    }

    #[test]
    fn test_clear() {
        let mut form = PowerForm::new("Enter time");
        form.input_value = "123".to_string();
        form.time_unit = TimeUnit::Hours;

        form.clear();

        assert_eq!(form.input_value, "");
        assert_eq!(form.time_unit, TimeUnit::Seconds);
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
}
