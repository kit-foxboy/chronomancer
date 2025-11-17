use crate::{
    components::Component,
    utils::{
        TimeUnit,
        messages::{ComponentMessage, PageMessage},
        ui::{ComponentSize, Gaps, fixed},
    },
};
use cosmic::{
    Action, Element, Task,
    iced::{
        Alignment,
        widget::{column, row},
    },
    theme::Button,
    widget::{ComboBox, TextInput, button, combo_box},
};

/// Struct representing a power form component
#[derive(Debug, Clone)]
pub struct PowerForm {
    pub input_value: String,
    pub time_unit: TimeUnit,
    pub time_unit_options: combo_box::State<TimeUnit>,
    placeholder_text: String,
}

impl Component for PowerForm {
    fn view(&self) -> Element<'_, ComponentMessage> {
        column![
            row![
                TextInput::new(&self.placeholder_text, &self.input_value)
                    .on_input(|text| ComponentMessage::TextChanged(text))
                    .on_submit(|_| { ComponentMessage::SubmitPressed })
                    .width(fixed(ComponentSize::INPUT_MIN_WIDTH)),
                ComboBox::new(
                    &self.time_unit_options,
                    "",
                    Some(&self.time_unit),
                    ComponentMessage::TimeUnitChanged,
                )
            ]
            .spacing(Gaps::xs())
            .align_y(Alignment::Center),
            button::text("Set {} Time")
                .on_press(ComponentMessage::SubmitPressed)
                .class(Button::Suggested)
        ]
        .into()
    }

    /// Update the power form state based on messages
    fn update(&mut self, message: ComponentMessage) -> Task<Action<PageMessage>> {
        match message {
            ComponentMessage::TextChanged(new_text) => {
                if let Ok(value) = new_text.parse::<u32>() {
                    self.input_value = value.to_string();
                }
                Task::none()
            }
            ComponentMessage::TimeUnitChanged(unit) => {
                self.time_unit = unit;
                Task::none()
            }
            ComponentMessage::SubmitPressed => {
                if self.validate_input() {
                    let value = self.input_value.parse::<i32>().unwrap()
                        * self.time_unit.to_seconds_multiplier();
                    Task::done(Action::App(PageMessage::PowerFormSubmitted(value)))
                } else {
                    self.clear();
                    Task::none()
                }
            }
            _ => Task::none(),
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
