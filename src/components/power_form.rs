use cosmic::{
    Element,
    iced::{Alignment, Length::Fill, widget::column},
    theme::Button,
    widget::{ComboBox, TextInput, button, combo_box},
};

use crate::{
    fl,
    utils::{Padding, TimeUnit, ui::Gaps},
};

/// Enum representing the different power operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerOperation {
    StayAwake,
    Suspend,
    Shutdown,
    Reboot,
    Logout,
}

impl PowerOperation {
    /// Convert from radio button index to `PowerOperation`
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

    /// Get the button index for this operation
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

    /// Get the icon name for this operation
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

    /// Get the localized placeholder text for this operation
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

/// Struct representing a power form component
#[derive(Debug, Clone)]
pub struct PowerForm {
    pub input_value: String,
    pub time_unit: TimeUnit,
    pub time_unit_options: combo_box::State<TimeUnit>,
    pub placeholder_text: String,
}

impl PowerForm {
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

    /// Render the power form with message constructors
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

    /// Handle text input, validating numeric values
    pub fn handle_text_input(&mut self, new_text: &str) {
        if let Ok(value) = new_text.parse::<u32>() {
            self.input_value = value.to_string();
        } else if new_text.is_empty() {
            self.input_value.clear();
        }
    }

    /// Validate that input is a positive integer
    pub fn validate_input(&self) -> bool {
        let value = self.input_value.parse::<i32>();
        value.is_ok() && value.unwrap_or_default() > 0
    }

    /// Clear the form inputs
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
