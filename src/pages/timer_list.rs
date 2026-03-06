use cosmic::{
    Action, Element, Task, cosmic_theme::Spacing, iced_widget::column, theme, widget::Space,
};

use crate::{
    components::list::{
        ListHeader, ListHeaderForm, header::Message as ListHeaderMessage,
        header_form::Message as ListHeaderFormMessage, item::ListItem,
    },
    models::Timer,
    utils::time,
};

/// Page level messages for the timer list page
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Message {
    ListHeaderMessage(ListHeaderMessage),
    ListHeaderFormMessage(ListHeaderFormMessage),
    TimerFormSubmitted { name: String, duration_seconds: i32 },
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
            list_header_form: ListHeaderForm::applet("Add Timer")
                .name_placeholder("Timer name...")
                .duration_placeholder("Duration..."),
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
                let end_time = timer.ends_at.clone();
                ListItem::new(&timer.description)
                    .description(time::format_duration(time::timestamp_diff_seconds(
                        end_time, None,
                    )))
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
    pub fn update(&mut self, message: Message) -> Task<Action<Message>> {
        match message {
            Message::ListHeaderMessage(msg) => match msg {
                ListHeaderMessage::AddButtonPressed => {
                    self.show_list_header_form = true;
                    Task::none()
                }
            },
            Message::ListHeaderFormMessage(msg) => match msg {
                ListHeaderFormMessage::NameInputChanged(input) => {
                    self.list_header_form.handle_name_input(&input);
                    Task::none()
                }
                ListHeaderFormMessage::DurationInputChanged(input) => {
                    self.list_header_form.handle_duration_input(&input);
                    Task::none()
                }
                ListHeaderFormMessage::TimeUnitChanged(unit) => {
                    self.list_header_form.set_time_unit(unit);
                    Task::none()
                }
                ListHeaderFormMessage::Submit => {
                    if let Some(submission) = self.list_header_form.handle_submit() {
                        let name = submission.name.clone();
                        let duration_seconds = submission.duration_seconds;
                        self.list_header_form.clear();
                        self.show_list_header_form = false;

                        // Emit a page-level message so the app can create the timer
                        self.update(Message::TimerFormSubmitted {
                            name,
                            duration_seconds,
                        })
                    } else {
                        Task::none()
                    }
                }
                ListHeaderFormMessage::Cancel => {
                    self.list_header_form.clear();
                    self.show_list_header_form = false;

                    Task::none()
                }
            },
            Message::TimerFormSubmitted {
                name,
                duration_seconds,
            } => {
                // TODO: create the timer via the app-level handler
                println!("New timer submitted: name='{name}', duration={duration_seconds}s");
                Task::done(Action::App(Message::TimerFormSubmitted {
                    name,
                    duration_seconds,
                }))
            }
            Message::PauseTimer(_index) => {
                // Handle pausing timer at index
                Task::none()
            }
            Message::ResumeTimer(_index) => {
                // Handle resuming timer at index
                Task::none()
            }
            Message::DeleteTimer(_index) => {
                // Handle deleting timer at index
                Task::none()
            }
            Message::ToggleRecurring(_index) => {
                // Handle toggling recurring status of timer at index
                Task::none()
            }
        }
    }
}
