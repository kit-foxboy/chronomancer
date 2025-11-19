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
