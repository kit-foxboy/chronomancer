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

        form.input_value = "".to_string();
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
}
