use super::radio_components::RadioComponent;
use crate::utils::{resources, ui::ComponentSize};
use cosmic::{
    Element,
    iced::Length,
    theme,
    widget::{button, container},
};

/// Struct representing a toggle-able `ToggleIconRadio` button
#[derive(Debug, Clone)]
pub struct ToggleIconRadio {
    #[allow(dead_code)]
    pub index: usize,
    pub name: &'static str,
}

impl ToggleIconRadio {
    /// Create a new `ToggleIconRadio` instance
    #[must_use]
    pub fn new(index: usize, name: &'static str) -> Self {
        Self { index, name }
    }

    /// Determine the button style based on its active state.
    #[must_use]
    #[allow(clippy::unused_self)]
    pub fn button_style(&self, is_active: bool) -> theme::Button {
        if is_active {
            theme::Button::Suggested
        } else {
            theme::Button::Text
        }
    }
}

impl RadioComponent for ToggleIconRadio {
    /// Render the toggle icon radio button.
    fn view<Message>(&self, is_active: bool, on_select: Message) -> Element<'_, Message>
    where
        Message: Clone + 'static,
    {
        button::custom(
            container(resources::system_icon(self.name, ComponentSize::ICON_SIZE))
                .width(Length::Fill)
                .center(Length::Fill),
        )
        .on_press(on_select)
        .width(Length::Fill)
        .height(ComponentSize::ICON_BUTTON_HEIGHT)
        .class(self.button_style(is_active))
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_icon_radio_creation() {
        let radio = ToggleIconRadio::new(0, "test-icon");
        assert_eq!(radio.index, 0);
        assert_eq!(radio.name, "test-icon");
    }

    #[test]
    fn test_toggle_icon_radio_style_active() {
        let radio = ToggleIconRadio::new(0, "test-icon");
        let style = radio.button_style(true);
        assert!(matches!(style, theme::Button::Suggested));
    }

    #[test]
    fn test_toggle_icon_radio_style_inactive() {
        let radio = ToggleIconRadio::new(1, "test-icon");
        let style = radio.button_style(false);
        assert!(matches!(style, theme::Button::Text));
    }

    #[test]
    fn test_view_compiles() {
        #[derive(Debug, Clone)]
        enum TestMessage {
            Selected,
        }

        let radio = ToggleIconRadio::new(0, "system-suspend-symbolic");

        // Just verify that the view method compiles and returns an Element
        let _element = radio.view(true, TestMessage::Selected);
        let _element = radio.view(false, TestMessage::Selected);
    }
}
