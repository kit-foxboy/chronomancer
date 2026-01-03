//! Icon-based radio button components for toggle selection interfaces.
//!
//! This module provides [`ToggleIconRadio`], a radio button component that displays
//! an icon and changes visual style based on its active state. It implements the
//! [`RadioComponent`] trait for use in radio button groups.
//!
//! Create with `ToggleIconRadio::new(index, icon_name)`, then render with
//! `.view(is_active, message)`. Use with [`RadioComponents`](super::radio_components::RadioComponents)
//! for automatic selection state management.

use super::radio_components::RadioComponent;
use crate::utils::{resources, ui::ComponentSize};
use cosmic::{
    Element,
    iced::Length,
    theme,
    widget::{button, container},
};

/// A radio button component that displays a system icon.
///
/// `ToggleIconRadio` represents a single option in a radio button group. It displays
/// a system icon and changes its visual style (Suggested or Text theme) based on
/// whether it's the currently selected option.
///
/// # Visual Behavior
///
/// - **Active (selected)**: Uses `Button::Suggested` style (highlighted)
/// - **Inactive**: Uses `Button::Text` style (normal appearance)
#[derive(Debug, Clone)]
pub struct ToggleIconRadio {
    /// The index of this option in its radio group.
    #[allow(dead_code)]
    pub index: usize,

    /// The system icon name to display (e.g., "system-suspend-symbolic").
    ///
    /// This should be a valid icon name from the system icon theme or
    /// a custom icon registered with the application. Use XDG icon names
    /// for best compatibility.
    pub name: &'static str,
}

impl ToggleIconRadio {
    /// Creates a new `ToggleIconRadio` button.
    ///
    /// # Arguments
    ///
    /// - `index` - The position of this option in its radio group
    /// - `name` - The system icon name to display
    ///
    #[must_use]
    pub fn new(index: usize, name: &'static str) -> Self {
        Self { index, name }
    }

    /// Determines the button style based on its active state.
    ///
    /// Returns the appropriate theme for the button:
    /// - Active buttons use `Button::Suggested` (highlighted)
    /// - Inactive buttons use `Button::Text` (default appearance)
    ///
    /// # Arguments
    ///
    /// - `is_active` - Whether this option is currently selected
    ///
    /// # Returns
    ///
    /// A [`theme::Button`] style to apply to the button.
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
    /// Renders the toggle icon radio button as an [`Element`].
    ///
    /// Creates a button containing the system icon, styled according to the
    /// active state. The button fills available width and has a fixed height.
    ///
    /// # Arguments
    ///
    /// - `is_active` - Whether this option is currently selected
    /// - `on_select` - Message to send when this option is clicked
    ///
    /// # Returns
    ///
    /// An [`Element`] that can be added to a view.
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
