//! A header component for lists with an optional add button.
//!
//! This component displays a title and, if specified, an add button that
//! can trigger an action when pressed. It is designed to be used as a header for list sections in the UI.
//! In the case of applets, the add button is typically represented by a "+" icon
//! and triggers the showing of a form to add a new item to the list. In full GUI apps, this button will more likely be used for naviation
//! to a different screen or dialog for adding items.
//!

use cosmic::Element;

/// Messages emitted by the `ListHeader` component.
#[derive(Debug, Clone)]
pub enum Message {
    AddButtonPressed,
}

/// A struct representing a list header with a title and optional add button.
///
/// A view method is used to return the visual representation of the component as an Element.
pub struct ListHeader {
    title: String,
    has_add_button: bool,
}

impl ListHeader {
    /// Creates a new `ListHeader` with the specified title and add button option.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display in the header.
    /// * `has_add_button` - Whether to include an add button in the header.
    pub fn new(title: impl Into<String>, has_add_button: bool) -> Self {
        Self {
            title: title.into(),
            has_add_button,
        }
    }

    /// Returns the visual representation of the `ListHeader` as an Element.
    ///
    /// # Returns
    ///
    /// An Element representing the `ListHeader` component.
    #[must_use]
    pub fn view(&self) -> Element<'_, Message> {
        use crate::utils::ui::Padding;
        use cosmic::{
            iced::{Alignment::Center, Length::Fill},
            iced_widget::row,
            theme::Button::Icon,
            widget::{Space, button, icon, text},
        };

        // TODO: move text size to a constant in ui
        let title = text(&self.title).size(14);
        let mut header = row![title];

        if self.has_add_button {
            header = header
                .push(Space::with_width(Fill)) // Push button to right
                .push(
                    button::icon(icon::from_name("list-add-symbolic"))
                        .class(Icon)
                        .extra_small() // Compact size for applet
                        .on_press(Message::AddButtonPressed),
                );
        }

        header
            .align_y(Center)
            .spacing(8)
            .padding(Padding::standard())
            .into()
    }
}
