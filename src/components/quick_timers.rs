use crate::components::button_row;
use cosmic::{Element, theme};
use cosmic::{cosmic_theme::Spacing, iced::Alignment, iced_widget::column, widget::text};

pub fn quick_timers<Message: Clone + std::fmt::Debug + 'static>(
    timers: Vec<(String, Message)>,
) -> Element<'static, Message> {
    let Spacing {
        space_xxs, space_xs, ..
    } = theme::active().cosmic().spacing;

    let content = column![text("Quick Timers:").size(24), button_row(timers, space_xs)]
        .align_x(Alignment::Center)
        .padding(space_xxs)
        .spacing(space_xs);

    content.into()
}
