use crate::utils::messages::{AppMessage, PageMessage};
use cosmic::{Action, Element, Task};

pub mod power_controls;
pub use power_controls::PowerControls;

pub trait Page {
    fn view(&self) -> Element<'_, PageMessage>;
    fn update(&mut self, message: PageMessage) -> Task<Action<AppMessage>>;
}
