use super::radio_components::RadioComponent;
use crate::utils::{
    messages::ComponentMessage,
    resources,
    ui::{ComponentSize, fill, fixed},
};
use cosmic::{
    Element, theme,
    widget::{button, container},
};

// TODO: add a util file for spacing constants. Magic numbers in UIs drive me crazy.
/// Create a toggle icon button
pub fn toggle_icon_button<Message: Clone + std::fmt::Debug + 'static>(
    name: &str,
    is_active: bool,
    on_press: Message,
) -> Element<'static, Message> {
    let button_style = if is_active {
        theme::Button::Suggested
    } else {
        theme::Button::Text
    };

    button::custom(
        container(resources::system_icon(name, ComponentSize::ICON_SIZE))
            .width(fill())
            .center(fill()),
    )
    .on_press(on_press)
    .width(fill())
    .height(fixed(ComponentSize::ICON_BUTTON_HEIGHT))
    .class(button_style)
    .into()
}

/// Struct representing a toggle-able icon radio button
#[derive(Debug, Clone)]
pub struct ToggleIconRadio {
    pub index: usize,
    pub name: &'static str,
}

impl ToggleIconRadio {
    /// Create a new ToggleIconRadio instance
    pub fn new(index: usize, name: &'static str) -> Self {
        Self { index, name }
    }
}

impl RadioComponent for ToggleIconRadio {
    /// Render the toggle icon radio button
    fn view(&self, is_active: bool) -> Element<'_, ComponentMessage> {
        let button_style = if is_active {
            theme::Button::Suggested
        } else {
            theme::Button::Text
        };

        button::custom(
            container(resources::system_icon(self.name, ComponentSize::ICON_SIZE))
                .width(fill())
                .center(fill()),
        )
        .on_press(ComponentMessage::RadioOptionSelected(self.index))
        .width(fill())
        .height(fixed(ComponentSize::ICON_BUTTON_HEIGHT))
        .class(button_style)
        .into()
    }
}
