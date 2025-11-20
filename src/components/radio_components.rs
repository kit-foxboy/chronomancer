use cosmic::{Element, iced_widget::row};

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

    /// create a new `RadioComponents` instance
    pub fn new(options: Vec<T>) -> Self {
        Self {
            options,
            selected: None,
        }
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
    pub fn update(&mut self, message: &ComponentMessage) {
        if let ComponentMessage::RadioOptionSelected(index) = *message {
            if self.selected == Some(index) {
                self.deactivate();
            } else {
                self.set_active(index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmic::widget::Space;

    use super::*;

    #[derive(Debug, Clone)]
    struct MockRadioOption {
        #[allow(dead_code)]
        pub index: usize,
    }

    impl RadioComponent for MockRadioOption {
        fn view(&self, _is_active: bool) -> Element<'_, ComponentMessage> {
            // Simplified view for testing
            Space::new(10, 10).into()
        }
    }

    impl MockRadioOption {
        pub fn new(index: usize) -> Self {
            Self { index }
        }
    }

    #[test]
    fn test_radio_components_selection() {
        let options = vec![
            MockRadioOption::new(0),
            MockRadioOption::new(1),
            MockRadioOption::new(2),
        ];
        let mut radio_components = RadioComponents::new(options);

        // Initially, no option should be selected
        assert_eq!(radio_components.selected, None);

        // Select the first option
        radio_components.update(&ComponentMessage::RadioOptionSelected(0));
        assert_eq!(radio_components.selected, Some(0));

        // Select the second option
        radio_components.update(&ComponentMessage::RadioOptionSelected(1));
        assert_eq!(radio_components.selected, Some(1));

        // Deselect the second option
        radio_components.update(&ComponentMessage::RadioOptionSelected(1));
        assert_eq!(radio_components.selected, None);
    }
}
