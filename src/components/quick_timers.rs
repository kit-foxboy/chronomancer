use crate::utils::ui::{ComponentSize, Gaps};
use cosmic::{Element, theme};
use cosmic::{
    iced::Alignment,
    iced_widget::{column, row},
    widget::text,
};

pub fn quick_timers<Message: Clone + std::fmt::Debug + 'static>(
    timers: Vec<(String, Message)>,
) -> Element<'static, Message> {
    let mut buttons = row![];
    for (label, msg) in timers {
        buttons = buttons.push(
            cosmic::widget::button::text(label)
                // .height(fixed(ComponentSize::QUICK_TIMER_BUTTON_HEIGHT))
                .class(theme::Button::Standard)
                .on_press(msg),
        );
    }
    let buttons = buttons.spacing(Gaps::xs());

    let content = column![
        text("Quick Timers:").size(ComponentSize::HEADER_TEXT_SIZE),
        buttons
    ]
    .align_x(Alignment::Center)
    .padding(Gaps::xs())
    .spacing(Gaps::s());

    content.into()
}
