use cosmic::{Element, Task, Action};

use crate::utils::messages::{PageMessage, ComponentMessage};

pub mod button_row;
pub mod quick_timers;
pub mod icon_button;
pub mod icon_button_form;

pub use button_row::button_row;
pub use quick_timers::quick_timers;
pub use icon_button_form::IconButtonForm;
pub use icon_button::icon_button;


/// Trait for UI components
pub trait Component {
    fn view(&self) -> Element<'_, ComponentMessage>;
    fn update(&mut self, message: ComponentMessage) -> Task<Action<PageMessage>>;
}