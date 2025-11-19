use crate::{
    components::{Component, PowerForm, ToggleIconRadio, radio_components::RadioComponents},
    fl,
    pages::Page,
    utils::{
        messages::{AppMessage, ComponentMessage, PageMessage, PowerMessage},
        ui::{Gaps, Padding},
    },
};
use cosmic::{Action, Element, Task, iced::Alignment, iced_widget::column, widget::Space};

/// Struct representing the power controls page
#[derive(Debug, Clone)]
pub struct PowerControls {
    pub power_buttons: RadioComponents<ToggleIconRadio>,
    pub power_form: PowerForm,
}

impl Default for PowerControls {
    /// Create a default instance of PowerControls
    fn default() -> Self {
        Self {
            power_buttons: RadioComponents::new(vec![
                ToggleIconRadio::new(0, "chronomancer-stay-awake"),
                ToggleIconRadio::new(1, "system-suspend-symbolic"),
                ToggleIconRadio::new(2, "system-shutdown-symbolic"),
                ToggleIconRadio::new(3, "system-logout-symbolic"),
            ]),
            power_form: PowerForm::new(fl!("set-time-label", operation = fl!("operation-suspend"))),
        }
    }
}

impl Page for PowerControls {
    /// Render the power controls page
    fn view(&self) -> Element<'_, PageMessage> {
        let power_buttons = self
            .power_buttons
            .row(Gaps::s())
            .map(PageMessage::ComponentMessage);

        // Show power form only if one of the radio buttons is active (not stay-awake)
        let form = if let Some(index) = self.power_buttons.selected {
            if index > 0 {
                self.power_form.view().map(PageMessage::ComponentMessage)
            } else {
                Space::new(0, 0).into()
            }
        } else {
            Space::new(0, 0).into()
        };

        column![power_buttons, form]
            .align_x(Alignment::Start)
            .padding(Padding::vertical(Gaps::xs()))
            .spacing(Gaps::s())
            .into()
    }

    /// Update the power controls page state based on messages
    fn update(&mut self, message: PageMessage) -> Task<Action<AppMessage>> {
        match message {
            PageMessage::ComponentMessage(msg) => {
                if let ComponentMessage::RadioOptionSelected(new_index) = msg.clone() {
                    self.handle_radio_selection(new_index, msg)
                } else {
                    Task::done(Action::None)
                }
            }
            PageMessage::PowerFormSubmitted(time) => {
                if let Some(index) = self.power_buttons.selected {
                    match index {
                        1 => Task::done(Action::App(AppMessage::PowerMessage(
                            PowerMessage::SetSuspendTime(time),
                        ))),
                        2 => Task::done(Action::App(AppMessage::PowerMessage(
                            PowerMessage::SetShutdownTime(time),
                        ))),
                        3 => Task::done(Action::App(AppMessage::PowerMessage(
                            PowerMessage::SetLogoutTime(time),
                        ))),
                        _ => Task::done(Action::None),
                    }
                } else {
                    Task::done(Action::None)
                }
            }
        }
    }
}

impl PowerControls {
    /// handle radio button selection
    fn handle_radio_selection(
        &mut self,
        new_index: usize,
        msg: ComponentMessage,
    ) -> Task<Action<AppMessage>> {
        let previous = self.power_buttons.selected;
        self.power_buttons.update(msg);
        self.power_form.placeholder_text = match new_index {
            1 => fl!("set-time-label", operation = fl!("operation-suspend")),
            2 => fl!("set-time-label", operation = fl!("operation-shutdown")),
            3 => fl!("set-time-label", operation = fl!("operation-logout")),
            _ => fl!("set-time-label", operation = fl!("operation-suspend")),
        };
        if new_index == 0 || previous == Some(0) {
            Task::done(Action::App(AppMessage::PowerMessage(
                PowerMessage::ToggleStayAwake,
            )))
        } else {
            Task::done(Action::None)
        }
    }
}
