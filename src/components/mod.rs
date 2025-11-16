use cosmic::{Action, Element, Task};

use crate::utils::messages::{ComponentMessage, PageMessage};

pub mod icon_button;
pub mod power_form;
pub mod quick_timers;
pub mod radio_components;

pub use icon_button::ToggleIconRadio;
pub use power_form::PowerForm;
pub use quick_timers::quick_timers;

/// Trait for UI components
pub trait Component {
    fn view(&self) -> Element<'_, ComponentMessage>;
    fn update(&mut self, message: ComponentMessage) -> Task<Action<PageMessage>>;
}
