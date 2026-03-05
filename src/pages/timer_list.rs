use cosmic::{Element, cosmic_theme::Spacing, iced_widget::column, theme};

use crate::{
    components::list::{ListHeader, header::Message as ListHeaderMessage, item::ListItem},
    models::Timer,
};

/// Page level messages for the timer list page
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    ListHeaderMessage(ListHeaderMessage),
    TimerFormSubmitted,
    PauseTimer(usize),
    ResumeTimer(usize),
    DeleteTimer(usize),
    ToggleRecurring(usize),
}

/// Struct representing the timer list page
///
/// This page displays a list of timers with options to add, pause, resume, delete, and toggle recurring status.
/// It uses the `ListHeader` component for the header section, the `List` component for displaying timers,
/// and the `ListForm` component for adding new timers.
pub struct Page {
    list_header: ListHeader,
}

impl Page {
    /// Creates a new timer list page configured for applet context
    ///
    /// # Arguments
    /// - `title` - The title of the timer list page
    ///
    /// # Returns
    /// The newly created timer list page with applet configuration
    pub fn applet(title: impl Into<String>) -> Self {
        Self {
            list_header: ListHeader::applet_with_add(title),
        }
    }

    /// Renders the timer list page as an Element
    ///
    /// # Returns
    /// An Element representing the timer list page
    pub fn view(&self, timers: &[Timer]) -> Element<'_, Message> {
        let Spacing { space_xs, .. } = theme::active().cosmic().spacing;
        let header = self.list_header.view().map(Message::ListHeaderMessage);
        let items: Vec<Element<'_, Message>> = timers
            .iter()
            .map(|timer| {
                let actions: Vec<Element<'_, Message>> = vec![]; // TODO: add actions for each timer
                ListItem::new(&timer.description)
                    // TODO: decide how to display the remaining time and whether to include it in the description or as a separate field
                    // .description()
                    .icon("alarm-symbolic")
                    .view(actions)
            })
            .collect();
        column![header].extend(items).spacing(space_xs).into()
    }

    #[allow(clippy::unused_self)]
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ListHeaderMessage(msg) => match msg {
                ListHeaderMessage::AddButtonPressed => {
                    // Handle add button pressed
                    println!("Add button pressed");
                }
            },
            Message::TimerFormSubmitted => {
                // Handle timer form submission
            }
            Message::PauseTimer(_index) => {
                // Handle pausing timer at index
            }
            Message::ResumeTimer(_index) => {
                // Handle resuming timer at index
            }
            Message::DeleteTimer(_index) => {
                // Handle deleting timer at index
            }
            Message::ToggleRecurring(_index) => {
                // Handle toggling recurring status of timer at index
            }
        }
    }
}
