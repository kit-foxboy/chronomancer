use cosmic::Element;

use crate::{
    components::list_header::{ListHeader, Message as ListHeaderMessage},
    fl,
};

/// Page level messages for the timer list page
#[derive(Debug, Clone)]
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
/// It uses the ListHeader component for the header section, the List component for displaying timers,
/// and the ListForm component for adding new timers.
pub struct Page {
    list_header: ListHeader,
}

impl Default for Page {
    fn default() -> Self {
        Self::new(fl!("title-timers"))
    }
}

impl Page {
    /// Creates a new timer list page with the given title
    ///
    /// # Arguments
    /// - `title` - The title of the timer list page
    ///
    /// # Returns
    /// The newly created timer list page
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            list_header: ListHeader::new(title, true),
        }
    }

    /// Renders the timer list page as an Element
    ///
    /// # Returns
    /// An Element representing the timer list page
    pub fn view(&self) -> Element<'_, Message> {
        self.list_header.view().map(Message::ListHeaderMessage)
    }

    // TODO: Add update method to handle messages and update state accordingly
}
