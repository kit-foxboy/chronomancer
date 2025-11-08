use cosmic::{
    Action, Element, Task,
    cosmic_theme::Spacing,
    theme,
    iced::{Alignment, widget::{column, row}},
    widget::{ComboBox, Id, TextInput, combo_box}
};
use crate::{
    components::{Component, icon_button},
    utils::{messages::{PageMessage, ComponentMessage}, TimeUnit},
};

#[derive(Debug, Clone)]
pub struct IconButtonForm {
    pub id: Id,
    pub input_value: String,
    pub time_unit: TimeUnit,
    pub time_unit_options: combo_box::State<TimeUnit>,
    icon_name: String,
    placeholder_text: String,
}

impl Component for IconButtonForm {
    fn view(&self) -> Element<'_, ComponentMessage> {
        let Spacing { space_xxs, .. } = theme::active().cosmic().spacing;
        // show a text widget with an icon button that submits the form
        column![
            icon_button(&self.icon_name, ComponentMessage::SubmitPressed()),
            row![
                TextInput::new(&self.placeholder_text, &self.input_value)
                    .on_input(|text| ComponentMessage::TextChanged(text))
                    .on_submit(|_| ComponentMessage::SubmitPressed()),
                ComboBox::new(
                    &self.time_unit_options,
                    "",
                    Some(&self.time_unit),
                    ComponentMessage::TimeUnitChanged,
                )
            ]
            .spacing(space_xxs)
            .align_y(Alignment::Center),
        ]
        .spacing(space_xxs)
        .into()
    }

    fn update(&mut self, message: ComponentMessage) -> Task<Action<PageMessage>> {
        match message {
            ComponentMessage::TextChanged(new_text) => {
                if let Ok(value) = new_text.parse::<u32>() {
                    self.input_value = value.to_string();
                }
                Task::none()
            },
            ComponentMessage::TimeUnitChanged(unit) => {
                self.time_unit = unit;
                Task::none()
            },
            ComponentMessage::SubmitPressed() => {
                Task::done(Action::App(PageMessage::FormSubmitted(self.id.clone())))
            }
        }
    }
}

impl IconButtonForm {
    pub fn new(icon_name: impl Into<String>, placeholder_text: impl Into<String>) -> Self {
        Self {
            id: Id::unique(),
            input_value: String::new(),
            time_unit: TimeUnit::Seconds, // Default to seconds
            time_unit_options: combo_box::State::new(vec![
                TimeUnit::Seconds,
                TimeUnit::Minutes,
                TimeUnit::Hours,
                TimeUnit::Days,
            ]),
            icon_name: icon_name.into(),
            placeholder_text: placeholder_text.into(),
        }
    }
}
