// SPDX-License-Identifier: MIT

use crate::config::Config;
use cosmic::{
    Action, Application, Core, Element, Task, applet,
    cosmic_config::{self, CosmicConfigEntry},
    cosmic_theme::Spacing,
    iced::{Limits, Alignment, platform_specific::shell::commands::popup, stream, window},
    iced_futures::subscription::Subscription,
    iced_runtime::Appearance,
    iced_widget::{column, row},
    theme,
    widget::{button, menu, text},
};
use futures_util::SinkExt;
use std::collections::HashMap;

// const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
// const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/hourglass.svg");

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Key bindings for the application
    // will look into this for more keyboard friendly UX as most cosmic apps are very lacking
    key_binds: HashMap<menu::KeyBind, ()>,
    /// The icon button displayed in the system tray.
    icon_name: String,
    // Configuration data that persists between application runs.
    config: Config,
    /// Popup window
    popup: Option<window::Id>,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    UpdateConfig(Config),
    LaunchUrl(String),
    SubscriptionChannel,
    NewTimer(u32),
}

/// Create a COSMIC application from the app model
impl Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.github.kit-foxboy.chronomancer";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Construct the app model with the runtime's core.
        let app = AppModel {
            core,
            key_binds: HashMap::new(),
            icon_name: "com.github.kit-foxboy.chronomancer".to_string(),
            // Optional configuration file for an application.
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((_errors, config)) => {
                        // for why in errors {
                        //     tracing::error!(%why, "error loading app config");
                        // }

                        config
                    }
                })
                .unwrap_or_default(),
            popup: None,
        };

        (app, Task::none())
    }

    /// Define the view window for the application.
    fn view_window(&self, id: window::Id) -> Element<'_, Self::Message> {
        let Spacing { space_s, .. } = theme::active().cosmic().spacing;
        if matches!(self.popup, Some(p) if p == id) {
            let quick_timers = row![
                button::standard("5 minutes").on_press(Message::NewTimer(300)),
                button::standard("10 minutes").on_press(Message::NewTimer(600)),
                button::standard("15 minutes").on_press(Message::NewTimer(900)),
            ].spacing(space_s);

            let content = column![text("Quick Timers:").size(24), quick_timers]
                .align_x(Alignment::Center)
                .padding([8, 0])
                .spacing(space_s);

            self.core
                .applet
                .popup_container(content)
                .max_height(400.)
                .max_width(500.)
                .into()
        } else {
            text("").into()
        }
    }

    /// Describes the interface based on the current state of the application model.
    fn view(&'_ self) -> cosmic::Element<'_, Message> {
        self.core
            .applet
            .icon_button(&self.icon_name)
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn style(&self) -> Option<Appearance> {
        Some(applet::style())
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;

        Subscription::batch(vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run_with_id(
                std::any::TypeId::of::<MySubscription>(),
                stream::channel(4, move |mut channel| async move {
                    _ = channel.send(Message::SubscriptionChannel).await;

                    futures_util::future::pending().await
                }),
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::SubscriptionChannel => {
                // For example purposes only.
            }

            Message::TogglePopup => {
                let task = self.toggle_popup();
                return task.map(|_| Action::None);
            }

            Message::NewTimer(timer_length) => {
                println!("Starting new timer for {} seconds", timer_length);
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }

            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("failed to open {url:?}: {err}");
                }
            },
        }
        Task::none()
    }
}

impl AppModel {
    /// Toggles the main panel visibility.
    fn toggle_popup(&mut self) -> Task<Message> {
        if let Some(p) = self.popup.take() {
            // Close the popup if it is open.
            popup::destroy_popup::<Message>(p)
        } else {
            // create new popup
            let new_id = window::Id::unique();
            self.popup.replace(new_id);

            // Get the popup settings from the applet
            let mut popup_settings = self.core.applet.get_popup_settings(
                self.core.main_window_id().unwrap(),
                new_id,
                Some((500, 500)),
                None,
                None,
            );

            // Set minimum size limits for the popup
            popup_settings.positioner.size_limits = Limits::NONE.min_width(300.0).min_height(150.0);

            popup::get_popup::<Message>(popup_settings)
        }
    }
}

// TODO: Implement menu actions as needed. Might try to use this for keyboard shortcuts
// impl menu::action::MenuAction for MenuAction {
//     type Message = Message;

//     fn message(&self) -> Self::Message {
//         match self {
//         }
//     }
// }
