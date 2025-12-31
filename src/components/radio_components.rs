//! Generic radio button component system for managing groups of selectable options.
//!
//! This module provides a trait-based system for creating radio button groups with
//! any type of visual component. The [`RadioComponent`] trait defines the interface
//! for individual radio options, while [`RadioComponents`] manages the group state
//! and rendering.
//!
//! # Architecture
//!
//! - [`RadioComponent`] - Trait for individual radio button options
//! - [`RadioComponents`] - Manager for a group of radio options with selection state
//!
//! # Usage
//!
//! Create a `RadioComponents` instance with a vector of items implementing `RadioComponent`
//! (such as `ToggleIconRadio`). Set the `.selected` field to track which option is active.
//! Call `.view(on_select)` to render the group, where `on_select` is a closure mapping
//! the selected index to your message type.
//!
//! For custom radio button styles, implement the `RadioComponent` trait on your type.
//! The trait requires a `view` method that takes an `is_active` flag and selection message,
//! returning an `Element`. See `ToggleIconRadio` for a reference implementation.

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

/// Trait for types that can be used as radio button options.
///
/// Implement this trait to create custom radio button components that can be
/// managed by [`RadioComponents`]. The trait requires types to be cloneable
/// and debuggable for ease of use in application state.
///
/// # Required Methods
///
/// - [`view`](RadioComponent::view) - Render the option with active state and selection handler
///
/// # Type Requirements
///
/// - Must implement [`Clone`] - Radio options are stored in collections
/// - Must implement [`Debug`] - For debugging and development
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::components::radio_components::RadioComponent;
/// use cosmic::{Element, widget::{button, text}};
///
/// #[derive(Debug, Clone)]
/// struct SimpleRadio {
///     label: &'static str,
/// }
///
/// impl RadioComponent for SimpleRadio {
///     fn view<Message>(&self, is_active: bool, on_select: Message) -> Element<'_, Message>
///     where
///         Message: Clone + 'static,
///     {
///         button::text(self.label)
///             .on_press(on_select)
///             .into()
///     }
/// }
/// ```
pub trait RadioComponent: Clone + std::fmt::Debug {
    /// Renders the radio option as an [`Element`].
    ///
    /// This method is called for each option in the radio group to generate
    /// its visual representation. Implementations should handle both active
    /// and inactive states appropriately.
    ///
    /// # Arguments
    ///
    /// - `is_active` - Whether this option is currently selected in the group
    /// - `on_select` - Message to send when this option is clicked/activated
    ///
    /// # Returns
    ///
    /// An [`Element`] representing this radio option.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::components::radio_components::RadioComponent;
    /// use cosmic::{Element, widget::button, theme::Button as ButtonTheme};
    ///
    /// #[derive(Debug, Clone)]
    /// struct MyRadio {
    ///     text: String,
    /// }
    ///
    /// impl RadioComponent for MyRadio {
    ///     fn view<Message>(&self, is_active: bool, on_select: Message) -> Element<'_, Message>
    ///     where
    ///         Message: Clone + 'static,
    ///     {
    ///         let style = if is_active {
    ///             ButtonTheme::Suggested
    ///         } else {
    ///             ButtonTheme::Standard
    ///         };
    ///
    ///         button::text(&self.text)
    ///             .on_press(on_select)
    ///             .class(style)
    ///             .into()
    ///     }
    /// }
    /// ```
    fn view<Message>(&self, is_active: bool, on_select: Message) -> Element<'_, Message>
    where
        Message: Clone + 'static;
}

/// Manager for a group of radio button components.
///
/// `RadioComponents` handles the state and rendering of a radio button group.
/// It stores a collection of options (any type implementing [`RadioComponent`])
/// and tracks which option is currently selected.
///
/// # Fields
///
/// - `options` - Vector of radio button options
/// - `selected` - Index of the currently selected option (None if nothing selected)
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::components::radio_components::RadioComponents;
/// use chronomancer::components::icon_button::ToggleIconRadio;
///
/// let options = vec![
///     ToggleIconRadio::new(0, "option-one"),
///     ToggleIconRadio::new(1, "option-two"),
///     ToggleIconRadio::new(2, "option-three"),
/// ];
///
/// let mut radio_group = RadioComponents::new(options);
///
/// // Set initial selection
/// radio_group.selected = Some(1);
///
/// // Check selection
/// assert_eq!(radio_group.selected, Some(1));
/// ```
#[derive(Debug, Clone)]
pub struct RadioComponents<T: RadioComponent> {
    /// The available radio button options.
    pub options: Vec<T>,

    /// The index of the currently selected option.
    ///
    /// `None` indicates no option is selected.
    pub selected: Option<usize>,
}

impl<T: RadioComponent> RadioComponents<T> {
    /// Creates a new `RadioComponents` group with no selection.
    ///
    /// # Arguments
    ///
    /// - `options` - Vector of radio button options to manage
    ///
    /// # Returns
    ///
    /// A new `RadioComponents` instance with `selected` set to `None`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::components::radio_components::RadioComponents;
    /// use chronomancer::components::icon_button::ToggleIconRadio;
    ///
    /// let options = vec![
    ///     ToggleIconRadio::new(0, "icon-one"),
    ///     ToggleIconRadio::new(1, "icon-two"),
    /// ];
    ///
    /// let radio_group = RadioComponents::new(options);
    /// assert_eq!(radio_group.selected, None);
    /// assert_eq!(radio_group.options.len(), 2);
    /// ```
    #[must_use]
    pub fn new(options: Vec<T>) -> Self {
        Self {
            options,
            selected: None,
        }
    }

    /// Displays options in a row layout with standard spacing (16px).
    ///
    /// # Arguments
    ///
    /// - `on_select` - Function that takes an option index and returns a message
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

    /// Displays options in a row layout with custom spacing.
    ///
    /// # Arguments
    ///
    /// - `spacing` - Space between options in pixels
    /// - `on_select` - Function that takes an option index and returns a message
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
