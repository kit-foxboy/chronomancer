use cosmic::Element;

use crate::components::list::{ListHeader, header::Message as ListHeaderMessage};

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
    /// Creates a new timer list page with the given title
    ///
    /// # Arguments
    /// - `title` - The title of the timer list page
    ///
    /// # Returns
    /// The newly created timer list page
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            list_header: ListHeader::new(title).with_add_button(),
        }
    }

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
    pub fn view(&self) -> Element<'_, Message> {
        self.list_header.view().map(Message::ListHeaderMessage)
    }

    pub fn update(&self, message: Message) {
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
