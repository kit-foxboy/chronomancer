use crate::{
    components::{
        PowerForm, ToggleIconRadio, power_form::PowerOperation, radio_components::RadioComponents,
    },
    fl,
    utils::{
        TimeUnit,
        ui::{Gaps, Padding},
    },
};
use cosmic::{Action, Element, Task, iced::Alignment, iced_widget::column, widget::Space};

/// Messages for the power controls page
#[derive(Debug, Clone)]
pub enum Message {
    /// Radio button was selected
    RadioOptionSelected(usize),
    /// Text input changed in the power form
    FormTextChanged(String),
    /// Time unit changed in the power form
    FormTimeUnitChanged(TimeUnit),
    /// Form submit button pressed
    FormSubmitPressed,
    /// Clear the form after successful submission
    ClearForm,
    /// Request to toggle stay awake mode
    ToggleStayAwake,
    /// Request to set suspend timer
    SetSuspendTime(i32),
    /// Request to set shutdown timer
    SetShutdownTime(i32),
    /// Request to set logout timer
    SetLogoutTime(i32),
    /// Request to set reboot timer
    SetRebootTime(i32),
}

/// Struct representing the power controls page
#[derive(Debug, Clone)]
pub struct Page {
    pub power_buttons: RadioComponents<ToggleIconRadio>,
    pub power_form: PowerForm,
}

impl Default for Page {
    /// Create a default instance of `Page`
    fn default() -> Self {
        Self {
            power_buttons: RadioComponents::new(vec![
                ToggleIconRadio::new(
                    PowerOperation::StayAwake.index(),
                    PowerOperation::StayAwake.icon_name(),
                ),
                ToggleIconRadio::new(
                    PowerOperation::Suspend.index(),
                    PowerOperation::Suspend.icon_name(),
                ),
                ToggleIconRadio::new(
                    PowerOperation::Logout.index(),
                    PowerOperation::Logout.icon_name(),
                ),
                ToggleIconRadio::new(
                    PowerOperation::Reboot.index(),
                    PowerOperation::Reboot.icon_name(),
                ),
                ToggleIconRadio::new(
                    PowerOperation::Shutdown.index(),
                    PowerOperation::Shutdown.icon_name(),
                ),
            ]),
            power_form: PowerForm::new(fl!("set-time-label", operation = fl!("operation-suspend"))),
        }
    }
}

impl Page {
    /// Render the power controls page
    pub fn view(&self) -> Element<'_, Message> {
        let power_buttons = self.power_buttons.view(Message::RadioOptionSelected);

        // Show power form only if one of the radio buttons is active (not stay-awake)
        let form = if let Some(index) = self.power_buttons.selected
            && index > 0
        {
            self.power_form.view(
                Message::FormTextChanged,
                Message::FormTimeUnitChanged,
                Message::FormSubmitPressed,
            )
        } else {
            Space::new(0, 0).into()
        };

        column![power_buttons, form]
            .align_x(Alignment::Start)
            .padding(Padding::standard())
            .spacing(Gaps::s())
            .into()
    }

    /// Update the power controls page state based on messages
    pub fn update(&mut self, message: Message) -> Task<Action<Message>> {
        match message {
            Message::RadioOptionSelected(new_index) => self.handle_radio_selection(new_index),
            Message::FormTextChanged(new_text) => {
                self.power_form.handle_text_input(&new_text);
                Task::none()
            }
            Message::FormTimeUnitChanged(unit) => {
                self.power_form.time_unit = unit;
                Task::none()
            }
            Message::FormSubmitPressed => self.handle_form_submit(),
            Message::ClearForm => {
                self.power_form.clear();
                Task::none()
            }
            // These messages bubble up to the app level, so we just pass them through
            Message::ToggleStayAwake
            | Message::SetSuspendTime(_)
            | Message::SetShutdownTime(_)
            | Message::SetLogoutTime(_)
            | Message::SetRebootTime(_) => Task::none(),
        }
    }

    /// Handle radio button selection
    fn handle_radio_selection(&mut self, new_index: usize) -> Task<Action<Message>> {
        let previous = self.power_buttons.selected;
        self.power_buttons.selected = Some(new_index);

        // Update form placeholder based on selected action
        let operation = PowerOperation::from_index(new_index);
        self.power_form.placeholder_text = operation.placeholder_text();

        // Toggle stay awake if switching to/from stay awake button
        if operation == PowerOperation::StayAwake || previous == Some(0) {
            Task::done(Action::App(Message::ToggleStayAwake))
        } else {
            Task::none()
        }
    }

    /// Handle form submission
    fn handle_form_submit(&mut self) -> Task<Action<Message>> {
        if !self.power_form.validate_input() {
            self.power_form.clear();
            return Task::none();
        }

        let value = self.power_form.input_value.parse::<i32>().unwrap()
            * self.power_form.time_unit.to_seconds_multiplier();

        if let Some(index) = self.power_buttons.selected {
            let operation = PowerOperation::from_index(index);
            match operation {
                PowerOperation::Suspend => Task::done(Action::App(Message::SetSuspendTime(value))),
                PowerOperation::Shutdown => {
                    Task::done(Action::App(Message::SetShutdownTime(value)))
                }
                PowerOperation::Reboot => Task::done(Action::App(Message::SetRebootTime(value))),
                PowerOperation::Logout => Task::done(Action::App(Message::SetLogoutTime(value))),
                PowerOperation::StayAwake => Task::none(),
            }
        } else {
            Task::none()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_page() -> Page {
        Page::default()
    }

    #[test]
    fn test_create_power_controls() {
        let page = get_test_page();
        assert_eq!(page.power_buttons.options.len(), 5);
        assert_eq!(
            page.power_form.placeholder_text,
            fl!("set-time-label", operation = fl!("operation-suspend"))
        );
    }

    #[test]
    fn test_radio_selection_updates_placeholder() {
        let mut page = get_test_page();

        // Select logout option
        let _ = page.update(Message::RadioOptionSelected(2));
        assert_eq!(
            page.power_form.placeholder_text,
            fl!("set-time-label", operation = fl!("operation-logout"))
        );

        // Select shutdown option
        let _ = page.update(Message::RadioOptionSelected(4));
        assert_eq!(
            page.power_form.placeholder_text,
            fl!("set-time-label", operation = fl!("operation-shutdown"))
        );

        // Select reboot option
        let _ = page.update(Message::RadioOptionSelected(3));
        assert_eq!(
            page.power_form.placeholder_text,
            fl!("set-time-label", operation = fl!("operation-reboot"))
        );

        // Select suspend option
        let _ = page.update(Message::RadioOptionSelected(1));
        assert_eq!(
            page.power_form.placeholder_text,
            fl!("set-time-label", operation = fl!("operation-suspend"))
        );
    }

    #[test]
    fn test_form_text_input() {
        let mut page = get_test_page();
        let _ = page.update(Message::FormTextChanged("15".to_string()));
        assert_eq!(page.power_form.input_value, "15");
    }

    #[test]
    fn test_form_time_unit_change() {
        let mut page = get_test_page();
        let _ = page.update(Message::FormTimeUnitChanged(TimeUnit::Minutes));
        assert_eq!(page.power_form.time_unit, TimeUnit::Minutes);
    }
}
