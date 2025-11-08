use crate::{
    components::{Component, icon_button, IconButtonForm},
    pages::Page,
    utils::messages::{AppMessage, PageMessage, PowerMessage},
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
}

impl Default for PowerControls {
    fn default() -> Self {
        Self {
            stay_awake_active: false,
            suspend_form: IconButtonForm::new(
                "system-suspend-symbolic",
                ""
            )
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
            // icon_button("system-suspend-symbolic", PowerMessage::SetSuspendTime("".to_string())),
            // icon_button("system-log-out-symbolic", PowerMessage::SetLogoutTime("".to_string())),
            // icon_button("system-shutdown-symbolic", PowerMessage::SetShutdownTime("".to_string())),
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
                    if let Ok(value) = self.suspend_form.input_value.parse::<u32>() {
                        return Task::done(Action::App(AppMessage::PowerMessage(
                            PowerMessage::SetSuspendTime(value),
                        )));
                    }
                }
                Task::done(Action::None)
            }
            // PageMessage::SuspendTextChanged(time) => {
            //     self.suspend_time = time.clone();
            //     Task::none()
            // }
            // PageMessage::LogoutTextChanged(time) => {
            //     self.logout_time = time.clone();
            //     Task::none()
            // }
            // PageMessage::ShutdownTextChanged(time) => {
            //     self.shutdown_time = time.clone();
            //     Task::none()
            // }
            PageMessage::ComponentMessage(msg) => {
                return self.suspend_form.update(msg).map(|action| {
                    match action {
                        Action::App(page_msg) => Action::App(AppMessage::from(page_msg)),
                        _ => Action::None,
                    }
                });
            }
            _ => Task::done(Action::None),
        }
    }
}
