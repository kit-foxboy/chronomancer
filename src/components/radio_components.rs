use cosmic::{
    Element,
    iced_widget::{column, row},
};

use crate::utils::messages::ComponentMessage;

// Note: This is a simplified radio button component system. If mixed type radio components ends up being desireable, enums with a trait
// object approach can be implemented instead. Rust enums are hot shit! Example:
// pub enum RadioOption {
//     ToggleIconRadio(ToggleIconRadio),
//     AnotherRadioType(AnotherRadioType),
// }
//
// impl RadioComponent for RadioOption {
//     fn view(&self, is_active: bool) -> Element<'_, ComponentMessage> {
//         match self {
//             RadioOption::ToggleIconRadio(option) => option.view(is_active),
//             RadioOption::AnotherRadioType(option) => option.view(is_active),
//         }
//     }
// }

/// Trait for radio button components
pub trait RadioComponent: Clone + std::fmt::Debug {
    fn view(&self, is_active: bool) -> Element<'_, ComponentMessage>;
}

/// Struct to manage a group of radio button components
#[derive(Debug, Clone)]
pub struct RadioComponents<T: RadioComponent> {
    pub options: Vec<T>,
    pub selected: Option<usize>,
}

impl<T: RadioComponent> RadioComponents<T> {
    /// set the active option by index
    fn set_active(&mut self, index: usize) {
        self.selected = Some(index);
    }

    fn deactivate(&mut self) {
        self.selected = None;
    }

    /// create a new RadioComponents instance
    pub fn new(options: Vec<T>) -> Self {
        Self {
            options,
            selected: None,
        }
    }

    /// display options in a column layout
    pub fn column(&self, spacing: u16) -> Element<'_, ComponentMessage> {
        let mut column_elements = vec![];

        for (index, option) in self.options.iter().enumerate() {
            let is_active = match self.selected {
                Some(selected_index) => selected_index == index,
                None => false,
            };
            column_elements.push(option.view(is_active));
        }

        column(column_elements).spacing(spacing).into()
    }

    /// display options in a row layout
    pub fn row(&self, spacing: u16) -> Element<'_, ComponentMessage> {
        let mut row_elements = vec![];

        for (index, option) in self.options.iter().enumerate() {
            let is_active = match self.selected {
                Some(selected_index) => selected_index == index,
                None => false,
            };
            row_elements.push(option.view(is_active));
        }

        row(row_elements).spacing(spacing).into()
    }

    /// TODO: add a more flexible view method that embeds the options in a custom layout/element or something if needed

    /// handle messages to update selected option
    pub fn update(&mut self, message: ComponentMessage) {
        if let ComponentMessage::RadioOptionSelected(index) = message {
            if self.selected == Some(index) {
                self.deactivate();
            } else {
                self.set_active(index);
            }
        }
    }
}
