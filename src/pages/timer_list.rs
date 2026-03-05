use cosmic::{Element, cosmic_theme::Spacing, iced_widget::column, theme, widget::Space};

use crate::{
    components::list::{
        ListHeader, ListHeaderForm,
        header::Message as ListHeaderMessage,
        header_form::{self, Message as ListHeaderFormMessage},
        item::ListItem,
    },
    models::Timer,
};

/// Page level messages for the timer list page
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    ListHeaderMessage(ListHeaderMessage),
    ListHeaderFormMessage(ListHeaderFormMessage),
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
    list_header_form: ListHeaderForm,
    show_list_header_form: bool,
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
            list_header_form: ListHeaderForm::applet("Add Timer"),
            show_list_header_form: false,
        }
    }

    /// Renders the timer list page as an Element
    ///
    /// # Returns
    /// An Element representing the timer list page
    pub fn view(&self, timers: &[Timer]) -> Element<'_, Message> {
        let Spacing { space_xs, .. } = theme::active().cosmic().spacing;
        let header = self.list_header.view().map(Message::ListHeaderMessage);

        //conditionally show the header form
        let header_form: Element<'_, Message> = if self.show_list_header_form {
            self.list_header_form
                .view()
                .map(Message::ListHeaderFormMessage)
        } else {
            Space::new(0, 0).into()
        };

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

        column![header, header_form]
            .extend(items)
            .spacing(space_xs)
            .into()
    }

    #[allow(clippy::unused_self)]
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ListHeaderMessage(msg) => match msg {
                ListHeaderMessage::AddButtonPressed => {
                    self.show_list_header_form = true;
                }
            },
            Message::ListHeaderFormMessage(msg) => match msg {
                ListHeaderFormMessage::InputChanged(input) => {
                    // Handle input change in the list header form
                }
                ListHeaderFormMessage::Submit => {
                    self.show_list_header_form = false;
                    // Handle form submission, e.g., add a new timer
                }
                ListHeaderFormMessage::Cancel => {
                    self.show_list_header_form = false;
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
