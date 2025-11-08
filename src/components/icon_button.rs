use cosmic::{
    Element, iced::Length, widget::{button, container}, theme
};
use crate::utils::resources;

pub fn icon_button<Message: Clone + std::fmt::Debug + 'static>(
    name: &str, 
    on_press: Message
) -> Element<'static, Message> {
    button::custom(
        container(resources::system_icon(name, 36))
            .width(Length::Fill)
            .center(Length::Fill),
    )
    .on_press(on_press)
    .width(Length::Fill)
    .height(Length::Fixed(48.0))
    .class(theme::Button::Text)
    .into()
}