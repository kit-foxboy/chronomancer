use std::fmt::Debug;
use cosmic::{Action, Task, Element};

pub mod button_row;
pub mod quick_timers;
pub mod icon_button;
pub mod power_controls;

pub use button_row::button_row;
pub use quick_timers::quick_timers;
pub use icon_button::icon_button;
pub use power_controls::PowerControls;

// Use if stateful components are needed in the future
pub trait Component<Message: Clone + Debug> {
    fn view(&self) -> Element<'_, Message>;
    fn update(&mut self, message: Message) -> Task<Action<Message>>;
}