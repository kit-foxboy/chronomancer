use crate::{
    components::{Component, IconButtonForm, icon_button},
    pages::Page,
    utils::{TimeUnit, messages::{AppMessage, PageMessage, PowerMessage}},
    fl
};
use cosmic::{
    Action, Element, Task,
    cosmic_theme::Spacing,
    iced::{Alignment},
    iced_widget::{column, row},
    theme,
};

#[derive(Debug, Clone)]
pub struct PowerControls {
    pub stay_awake_active: bool,
    suspend_form: IconButtonForm,
    logout_form: IconButtonForm,
    shutdown_form: IconButtonForm,
}

impl Default for PowerControls {
    fn default() -> Self {
        Self {
            stay_awake_active: false,
            suspend_form: IconButtonForm::new(
                "system-suspend-symbolic",
                fl!("time")
            ),
            logout_form: IconButtonForm::new(
                "system-log-out-symbolic",
                fl!("time")
            ),
            shutdown_form: IconButtonForm::new(
                "system-shutdown-symbolic",
                fl!("time")
            ),
        }
    }
}

impl Page for PowerControls {
    fn view(&self) -> Element<'_, PageMessage> {
        let Spacing {
            space_xs, space_s, ..
        } = theme::active().cosmic().spacing;

        let power_buttons = row![
            // Custom chronomancer icons installed to system theme with chronomancer- prefix
            icon_button(
                "chronomancer-stay-awake",
                PageMessage::StayAwakeButtonPressed,
            ),
            self.suspend_form.view().map(PageMessage::ComponentMessage),
            // self.logout_form.view().map(PageMessage::ComponentMessage),
            self.shutdown_form.view().map(PageMessage::ComponentMessage),
        ]
        .spacing(space_s)
        .padding([0, space_s]);

        column![power_buttons]
            .align_x(Alignment::Start)
            .padding([space_xs, 0])
            .into()
    }

    fn update(&mut self, message: PageMessage) -> Task<Action<AppMessage>> {
        // Handle component state updates and pass message to parent
        match message {
            PageMessage::StayAwakeButtonPressed=> {
                self.stay_awake_active = !self.stay_awake_active;
                Task::done(Action::App(AppMessage::PowerMessage(
                    PowerMessage::ToggleStayAwake,
                )))
            },
            PageMessage::FormSubmitted(id) => {
                if id == self.suspend_form.id {
                    if let Ok(value) = self.suspend_form.input_value.parse::<i32>() {
                        let value = TimeUnit::to_seconds_multiplier(&self.suspend_form.time_unit) * value;
                        return Task::done(Action::App(AppMessage::PowerMessage(
                            PowerMessage::SetSuspendTime(value),
                        )));
                    }
                }

                if id == self.shutdown_form.id {
                    if let Ok(value) = self.shutdown_form.input_value.parse::<i32>() {
                        let value = TimeUnit::to_seconds_multiplier(&self.shutdown_form.time_unit) * value;
                        return Task::done(Action::App(AppMessage::PowerMessage(
                            PowerMessage::SetShutdownTime(value),
                        )));
                    }
                }
                Task::done(Action::None)
            }
            PageMessage::ComponentMessage(msg) => {
                return self.suspend_form.update(msg).map(|action| {
                    match action {
                        Action::App(page_msg) => Action::App(AppMessage::from(page_msg)),
                        _ => Action::None,
                    }
                });
            }
        }
    }
}
