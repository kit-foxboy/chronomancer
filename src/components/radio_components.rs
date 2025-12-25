use cosmic::{Element, iced_widget::row};

// Note: This is a simplified radio button component system. If mixed type radio components ends up being desireable, enums with a trait
// object approach can be implemented instead. Rust enums are hot shit! Example:
// pub enum RadioOption {
//     ToggleIconRadio(ToggleIconRadio),
//     AnotherRadioType(AnotherRadioType),
// }
//
// impl RadioComponent for RadioOption {
//     fn view<Message>(&self, is_active: bool, on_select: Message) -> Element<'_, Message> {
//         match self {
//             RadioOption::ToggleIconRadio(option) => option.view(is_active, on_select),
//             RadioOption::AnotherRadioType(option) => option.view(is_active, on_select),
//         }
//     }
// }

/// Trait for radio button components
pub trait RadioComponent: Clone + std::fmt::Debug {
    fn view<Message>(&self, is_active: bool, on_select: Message) -> Element<'_, Message>
    where
        Message: Clone + 'static;
}

/// Struct to manage a group of radio button components
#[derive(Debug, Clone)]
pub struct RadioComponents<T: RadioComponent> {
    pub options: Vec<T>,
    pub selected: Option<usize>,
}

impl<T: RadioComponent> RadioComponents<T> {
    /// create a new `RadioComponents` instance
    #[must_use]
    pub fn new(options: Vec<T>) -> Self {
        Self {
            options,
            selected: None,
        }
    }

    /// display options in a row layout with message constructor
    #[must_use]
    pub fn view<Message>(
        &self,
        on_select: impl Fn(usize) -> Message + 'static,
    ) -> Element<'_, Message>
    where
        Message: Clone + 'static,
    {
        self.row_with_spacing(16, on_select)
    }

    /// display options in a row layout with custom spacing
    #[must_use]
    pub fn row_with_spacing<Message>(
        &self,
        spacing: u16,
        on_select: impl Fn(usize) -> Message + 'static,
    ) -> Element<'_, Message>
    where
        Message: Clone + 'static,
    {
        let mut row_elements = vec![];

        for (index, option) in self.options.iter().enumerate() {
            let is_active = self.selected == Some(index);
            row_elements.push(option.view(is_active, on_select(index)));
        }

        row(row_elements).spacing(spacing).into()
    }
}

#[cfg(test)]
mod tests {
    use cosmic::widget::Space;

    use super::*;

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    enum TestMessage {
        RadioSelected(usize),
    }

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct MockRadioOption {
        pub index: usize,
    }

    impl RadioComponent for MockRadioOption {
        fn view<Message>(&self, _is_active: bool, on_select: Message) -> Element<'_, Message>
        where
            Message: Clone + 'static,
        {
            // Simplified view for testing - would normally be a button with on_press(on_select)
            let _ = on_select; // Acknowledge we received it
            Space::new(10, 10).into()
        }
    }

    impl MockRadioOption {
        pub fn new(index: usize) -> Self {
            Self { index }
        }
    }

    #[test]
    fn test_radio_components_creation() {
        let options = vec![
            MockRadioOption::new(0),
            MockRadioOption::new(1),
            MockRadioOption::new(2),
        ];
        let radio_components = RadioComponents::new(options);

        // Initially, no option should be selected
        assert_eq!(radio_components.selected, None);
        assert_eq!(radio_components.options.len(), 3);
    }

    #[test]
    fn test_radio_components_manual_selection() {
        let options = vec![
            MockRadioOption::new(0),
            MockRadioOption::new(1),
            MockRadioOption::new(2),
        ];
        let mut radio_components = RadioComponents::new(options);

        // Manually set selection
        radio_components.selected = Some(0);
        assert_eq!(radio_components.selected, Some(0));

        // Change selection
        radio_components.selected = Some(1);
        assert_eq!(radio_components.selected, Some(1));

        // Deselect
        radio_components.selected = None;
        assert_eq!(radio_components.selected, None);
    }

    #[test]
    fn test_view_compiles() {
        let options = vec![MockRadioOption::new(0), MockRadioOption::new(1)];
        let radio_components = RadioComponents::new(options);

        // Just verify that the view method compiles and returns an Element
        let _element = radio_components.view(TestMessage::RadioSelected);
    }
}
