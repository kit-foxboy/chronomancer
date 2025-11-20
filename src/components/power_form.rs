use cosmic::{
    Element,
    iced::{Alignment, Length::Fill, widget::column},
    theme::Button,
    widget::{ComboBox, TextInput, button, combo_box},
};

use crate::{
    components::Component,
    fl,
    utils::{
        Padding, TimeUnit,
        messages::{ComponentMessage, PageMessage},
        ui::Gaps,
    },
};

/// Struct representing a power form component
#[derive(Debug, Clone)]
pub struct PowerForm {
    pub input_value: String,
    pub time_unit: TimeUnit,
    pub time_unit_options: combo_box::State<TimeUnit>,
    pub placeholder_text: String,
}

impl Component for PowerForm {
    fn view(&self) -> Element<'_, ComponentMessage> {
        column![
            TextInput::new(&self.placeholder_text, &self.input_value)
                .on_input(ComponentMessage::TextChanged)
                .on_submit(|_| { ComponentMessage::SubmitPressed })
                .width(Fill),
            ComboBox::new(
                &self.time_unit_options,
                &fl!("unit-label"),
                Some(&self.time_unit),
                ComponentMessage::TimeUnitChanged,
            )
            .width(Fill),
            button::text(fl!("set-button-label"))
                .on_press(ComponentMessage::SubmitPressed)
                .class(Button::Suggested)
        ]
        .align_x(Alignment::Center)
        .spacing(Gaps::s())
        .padding(Padding::horizontal(24))
        .into()
    }

    /// Update the power form state based on messages
    fn update(&mut self, message: ComponentMessage) -> Option<PageMessage> {
        match message {
            ComponentMessage::TextChanged(new_text) => {
                if let Ok(value) = new_text.parse::<u32>() {
                    self.input_value = value.to_string();
                }
                None
            }
            ComponentMessage::TimeUnitChanged(unit) => {
                self.time_unit = unit;
                None
            }
            ComponentMessage::SubmitPressed => {
                if self.validate_input() {
                    println!("valid input");
                    let value = self.input_value.parse::<i32>().unwrap()
                        * self.time_unit.to_seconds_multiplier();
                    Some(PageMessage::PowerFormSubmitted(value))
                } else {
                    self.clear();
                    None
                }
            }
            ComponentMessage::RadioOptionSelected(_) => None,
        }
    }
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

    fn validate_input(&self) -> bool {
        let value = self.input_value.parse::<i32>();
        value.is_ok() && value.unwrap_or_default() > 0
    }

    fn clear(&mut self) {
        self.input_value.clear();
        self.time_unit = TimeUnit::Seconds;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_form_creation() {
        let form = PowerForm::new("Enter time");
        assert_eq!(form.input_value, "");
        assert_eq!(form.time_unit, TimeUnit::Seconds);
        assert_eq!(form.placeholder_text, "Enter time");
    }

    #[test]
    fn test_power_form_update_text() {
        let mut form = PowerForm::new("Enter time");
        form.update(ComponentMessage::TextChanged("15".to_string()));
        assert_eq!(form.input_value, "15");
    }

    #[test]
    fn test_power_form_update_time_unit() {
        let mut form = PowerForm::new("Enter time");
        form.update(ComponentMessage::TimeUnitChanged(TimeUnit::Minutes));
        assert_eq!(form.time_unit, TimeUnit::Minutes);
    }

    #[test]
    fn test_power_form_submit_valid() {
        let mut form = PowerForm::new("Enter time");
        form.input_value = "5".to_string();
        form.time_unit = TimeUnit::Minutes;
        let result = form.update(ComponentMessage::SubmitPressed);

        assert!(result.is_some());
        if let PageMessage::PowerFormSubmitted(time) = result.unwrap() {
            assert_eq!(time, 300); // 5 minutes in seconds XwX
        } else {
            panic!("Expected PowerFormSubmitted message");
        }
    }

    #[test]
    fn test_power_form_submit_invalid() {
        let mut form = PowerForm::new("Enter time");
        form.input_value = "potato".to_string();
        let result = form.update(ComponentMessage::SubmitPressed);
        assert!(result.is_none());
        assert_eq!(form.input_value, "");
    }

    #[test]
    fn test_power_form_validation() {
        let mut form = PowerForm::new("Enter time");
        form.input_value = "10".to_string();
        assert!(form.validate_input());
    }
}
