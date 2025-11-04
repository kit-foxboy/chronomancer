use cosmic::{
    Element,
    iced_widget::row,
    widget::button,
};

pub fn button_row<Message: Clone + std::fmt::Debug + 'static>(
    buttons: Vec<(String, Message)>,
    spacing: u16,
) -> Element<'static, Message> {
    buttons
        .into_iter()
        .fold(row![].spacing(spacing), |row, (label, message)| {
            row.push(button::standard(label).on_press(message))
        })
        .into()
}