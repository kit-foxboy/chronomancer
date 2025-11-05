use crate::components::{Component, icon_button};
use crate::models::PowerMessage as PowerMsg; // Rename to avoid conflict
use cosmic::{
    Action, Element, Task,
    cosmic_theme::Spacing,
    iced::Alignment,
    iced_widget::{column, row},
    theme,
};

#[derive(Debug, Clone)]
pub struct PowerControls {
    pub stay_awake_active: bool,
    pub suspend_time: String,
    pub logout_time: String,
    pub shutdown_time: String,
}

impl Default for PowerControls {
    fn default() -> Self {
        Self {
            stay_awake_active: false,
            suspend_time: String::new(),
            logout_time: String::new(),
            shutdown_time: String::new(),
        }
    }
}

impl Component<PowerMsg> for PowerControls {
    fn view(&self) -> Element<'_, PowerMsg> {
        let Spacing {
            space_xs, space_m, ..
        } = theme::active().cosmic().spacing;

        let power_buttons = row![
            // Custom chronomancer icons installed to system theme with chronomancer- prefix
            icon_button("chronomancer-stay-awake", PowerMsg::ToggleStayAwake),
            // icon_button("system-suspend-symbolic", PowerMsg::SetSuspendTime("".to_string())),
            // icon_button("system-log-out-symbolic", PowerMsg::SetLogoutTime("".to_string())),
            // icon_button("system-shutdown-symbolic", PowerMsg::SetShutdownTime("".to_string())),
        ]
        .spacing(space_m)
        .padding([0, space_m]);

        column![power_buttons]
            .align_x(Alignment::Start)
            .padding([space_xs, 0])
            .into()
    }

    fn update(&mut self, message: &PowerMsg) -> Task<Action<PowerMsg>> {
        match message {
            // PowerMsg::InhibitAcquired(result) => {
            //     // if let Ok(manager) = result.as_ref() {
            //     //     self.stay_awake_active = manager.is_active();
            //     // }
            // }
            _ => {
                // Handle other power messages
            }
        }
        Task::none()
    }
}
