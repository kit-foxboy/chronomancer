use cosmic::{
    widget,
    Element,
};

/// Load a system icon using icon::from_name
pub fn system_icon<Message: 'static>(name: &str, size: u16) -> Element<'static, Message> {
    widget::icon::from_name(name)
        .size(size)
        .symbolic(true)
        .icon()
        .into()
}
