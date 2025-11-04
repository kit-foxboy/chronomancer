// use cosmic::{Action, Task, Element};

pub mod button_row;
pub mod quick_timers;

pub use button_row::button_row;
pub use quick_timers::quick_timers;

// Use if stateful components are needed in the future
// pub trait Component<Message: Clone + std::fmt::Debug> {
//     fn view(&self) -> Element<'_, Message>;
//     fn update(&mut self, message: Message) -> Task<Action<Message>>;
// }