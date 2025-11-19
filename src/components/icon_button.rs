use super::radio_components::RadioComponent;
use crate::utils::{messages::ComponentMessage, resources, ui::ComponentSize};
use cosmic::{
    Element,
    iced::Length,
    theme,
    widget::{button, container},
};

/// Struct representing a toggle-able `ToggleIconRadio` button
#[derive(Debug, Clone)]
pub struct ToggleIconRadio {
    pub index: usize,
    pub name: &'static str,
}

impl ToggleIconRadio {
    /// Create a new `ToggleIconRadio` instance
    pub fn new(index: usize, name: &'static str) -> Self {
        Self { index, name }
    }
}

impl RadioComponent for ToggleIconRadio {
    /// Render the toggle icon radio button.
    fn view(&self, is_active: bool) -> Element<'_, ComponentMessage> {
        let button_style = if is_active {
            theme::Button::Suggested
        } else {
            theme::Button::Text
        };

        button::custom(
            container(resources::system_icon(self.name, ComponentSize::ICON_SIZE))
                .width(Length::Fill)
                .center(Length::Fill),
        )
        .on_press(ComponentMessage::RadioOptionSelected(self.index))
        .width(Length::Fill)
        .height(ComponentSize::ICON_BUTTON_HEIGHT)
        .class(button_style)
        .into()
    }
}
