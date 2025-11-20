// SPDX-License-Identifier: MIT

use cosmic::{
    Action, Application, Core, Element, Task, applet,
    cosmic_config::{self, CosmicConfigEntry},
    cosmic_theme::Spacing,
    iced::{
        Limits, Subscription, platform_specific::shell::commands::popup, stream::channel,
        widget::column, window,
    },
    iced_runtime::Appearance,
    theme,
    widget::text,
};
use futures_util::SinkExt;
use notify_rust::{Hint, Notification};
use std::{fs::File, str::FromStr, sync::Arc};

use crate::{
    config::Config,
    models::{Timer, timer::TimerType},
    pages::{Page, PowerControls},
    utils::{
        database::{Repository, SQLiteDatabase},
        messages::{
            AppMessage as Message, DatabaseMessage, PageMessage, PowerMessage, TimerMessage,
        },
        resources,
    },
};

const APP_ID: &str = "com.vulpineinteractive.chronomancer";
// const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
// const APP_ICON: &[u8] = include_bytes!("../resources/icons/hicolor/scalable/apps/hourglass.svg");

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Key bindings for the application
    // will look into this for more keyboard friendly UX as most cosmic apps are very lacking
    // key_binds: HashMap<menu::KeyBind, ()>,
    /// The icon button displayed in the system tray.
    icon_name: String,
    // Configuration data that persists between application runs.
    config: Config,
    /// Popup window
    popup: Option<window::Id>,
    /// Database connection
    // clone when passing to async tasks to add to the pool's reference count
    database: Option<SQLiteDatabase>,
    /// Suspend inhibitor file descriptor. Keep this alive to prevent system sleep.
    suspend_inhibitor: Option<File>,
    /// Active timers
    active_timers: Vec<Timer>,
    /// Power control component
    power_controls: PowerControls,
}

/// Create a COSMIC application from the app model
impl Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: cosmic::Core, _flags: Self::Flags) -> (Self, Task<cosmic::Action<Message>>) {
        // Construct the app model with the runtime's core.
        let app = AppModel {
            core,
            // key_binds: HashMap::new(),
            icon_name: "chronomancer-hourglass".to_string(),
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
            database: None,
            suspend_inhibitor: None,
            active_timers: vec![],
            power_controls: PowerControls::default(),
        };

        (
            app,
            Task::perform(
                async move { SQLiteDatabase::new().await.map_err(|e| e.to_string()) },
                |result| match result {
                    Ok(db) => Action::App(Message::DatabaseMessage(DatabaseMessage::Initialized(
                        Ok(db),
                    ))),
                    Err(err) => Action::App(Message::DatabaseMessage(
                        DatabaseMessage::FailedToInitialize(err),
                    )),
                },
            ),
        )
    }

    /// Define the view window for the application.
    fn view_window(&self, id: window::Id) -> Element<'_, Message> {
        if matches!(self.popup, Some(p) if p == id) {
            let Spacing { space_m, .. } = theme::active().cosmic().spacing;

            let power = self.power_controls.view().map(Message::PageMessage);
            let content = column![power].spacing(space_m);

            self.core
                .applet
                .popup_container(content)
                .max_height(400.)
                .max_width(800.)
                .into()
        } else {
            text("").into()
        }
    }

    /// Describes the interface based on the current state of the application model.
    fn view(&'_ self) -> Element<'_, Message> {
        self.core
            .applet
            .icon_button(&self.icon_name)
            .class(if self.suspend_inhibitor.is_some() {
                theme::Button::Suggested
            } else {
                theme::Button::AppletIcon
            })
            .on_press_down(Message::TogglePopup)
            .into()
    }

    fn style(&self) -> Option<Appearance> {
        Some(applet::style())
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        let task: Task<Action<Message>> = match message {
            Message::TogglePopup => {
                let t = self.toggle_popup();
                t.map(|_| Action::<Message>::None)
            }

            Message::PageMessage(msg) => self.handle_page_message(msg),

            Message::DatabaseMessage(msg) => self.handle_database_message(msg),

            Message::TimerMessage(msg) => self.handle_timer_message(msg),

            Message::PowerMessage(msg) => self.handle_power_message(msg),

            Message::Tick => self.handle_tick(),

            Message::UpdateConfig(config) => {
                self.config = config;
                Task::none()
            }
        };

        task
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    /// Good example uses are to watch for configuration file changes or keyboard events.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct TimerSubscription;

        Subscription::batch(vec![
            // Timer tick subscription - fires every second
            Subscription::run_with_id(
                std::any::TypeId::of::<TimerSubscription>(),
                channel(4, move |mut channel| async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

                    loop {
                        interval.tick().await;
                        if channel.send(Message::Tick).await.is_err() {
                            // Channel closed, exit the subscription
                            break;
                        }
                    }
                }),
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config)),
        ])
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

            // Get the main window id; in test contexts this may be None. If absent, we still
            // record that a popup is "open" but return no window task.
            let Some(main_id) = self.core.main_window_id() else {
                // No main window; treat as opened logically without creating actual popup window
                return Task::none();
            };

            let mut popup_settings =
                self.core
                    .applet
                    .get_popup_settings(main_id, new_id, Some((500, 500)), None, None);

            // Set minimum size limits for the popup
            popup_settings.positioner.size_limits = Limits::NONE.min_width(300.0).min_height(150.0);
            popup::get_popup::<Message>(popup_settings)
        }
    }

    fn handle_tick(&mut self) -> Task<Action<Message>> {
        let mut tasks: Vec<Task<Action<Message>>> = vec![];

        for timer in self.active_timers.clone() {
            if !timer.is_active() {
                match TimerType::from_str(&timer.description) {
                    Ok(TimerType::Suspend) => {
                        // Execute system suspend
                        tasks.push(Task::done(Action::App(Message::PowerMessage(
                            PowerMessage::ExecuteSuspend,
                        ))));
                    }
                    Ok(TimerType::Logout) => {
                        // Execute system logout
                        tasks.push(Task::done(Action::App(Message::PowerMessage(
                            PowerMessage::ExecuteLogout,
                        ))));
                    }
                    Ok(TimerType::Shutdown) => {
                        tasks.push(Task::done(Action::App(Message::PowerMessage(
                            PowerMessage::ExecuteShutdown,
                        ))));
                    }
                    Ok(TimerType::UserDefined(ref description)) => {
                        if let Err(e) = Notification::new()
                            .summary("Timer Finished")
                            .body(description.as_str())
                            .icon("alarm")
                            .hint(Hint::Category("alarm".to_owned()))
                            .hint(Hint::Resident(true))
                            .timeout(0)
                            .show()
                        {
                            eprintln!("Failed to send notification: {e}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to parse timer type: {e}");
                    }
                }

                // Capture the id before mutating the vector
                let timer_id = timer.id;

                // Remove finished timer from active timers
                self.active_timers.retain(|t| t.id != timer_id);

                if let Some(database) = self.database.clone() {
                    // Schedule an async task to delete the timer from the DB.
                    tasks.push(Task::perform(
                        async move {
                            Timer::delete_by_id(database.pool(), &timer_id)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |_result| Action::<Message>::None,
                    ));
                    // We only schedule one deletion per tick; break to avoid multiple concurrent deletes
                    // We could easily batch them, but 1 second and some local db calls aren't going to be a big deal here
                    // More noting this to acknowledge potential optimizations XP
                    break;
                }
            }
        }

        Task::batch(tasks)
    }

    /// Handle messages from pages
    // Todo: If necessary, expand to route to multiple pages
    // applets work a little differently than full apps with multiple pages so unsure if this is problem
    // attempting to define opinionated architecture around pages/components even in applets though
    fn handle_page_message(&mut self, msg: PageMessage) -> Task<Action<Message>> {
        self.power_controls.update(msg)
    }

    fn handle_database_message(&mut self, msg: DatabaseMessage) -> Task<Action<Message>> {
        match msg {
            DatabaseMessage::Initialized(result) => {
                if let Ok(db) = result {
                    println!("Database initialized successfully: {db:?}");
                    self.database = Some(db);

                    // Fetch active timers from the database
                    if let Some(database) = self.database.clone() {
                        return Task::perform(
                            async move {
                                Timer::get_all_active(database.pool())
                                    .await
                                    .map_err(|e| e.to_string())
                            },
                            |result| {
                                Action::App(Message::TimerMessage(TimerMessage::ActiveFetched(
                                    result,
                                )))
                            },
                        );
                    }
                }
            }
            DatabaseMessage::FailedToInitialize(err) => {
                eprintln!("Failed to initialize database: {err}");
                // todo: figure out how tf to notify user appropriately in applets
            }
        }
        Task::none()
    }

    fn handle_timer_message(&mut self, msg: TimerMessage) -> Task<Action<Message>> {
        match msg {
            TimerMessage::Created(result) => match result {
                Ok(timer) => {
                    self.active_timers.push(timer);
                    println!("Created timer: {:#?}", &self.active_timers.last());
                }
                Err(err) => {
                    eprintln!("Failed to create timer: {err}");
                }
            },
            TimerMessage::ActiveFetched(result) => match result {
                Ok(timers) => {
                    self.active_timers = timers;
                }
                Err(err) => {
                    eprintln!("Failed to fetch active timers: {err}");
                }
            },
        }
        Task::none()
    }

    fn handle_power_message(&mut self, msg: PowerMessage) -> Task<Action<Message>> {
        // let _ = self.power_controls.update(&msg);
        match msg {
            PowerMessage::ToggleStayAwake => {
                if let Some(inhibitor) = self.suspend_inhibitor.take() {
                    resources::release_suspend_inhibit(inhibitor);
                    println!("Released suspend inhibitor");
                } else {
                    return AppModel::get_suspend_inhibitor();
                }
            }
            PowerMessage::InhibitAcquired(result) => {
                match Arc::try_unwrap(result) {
                    Ok(Ok(file)) => {
                        // Successfully unwrapped the Arc and got the File
                        // Double okay is a bit silly but matches the async task return type
                        // Also makes the arc unwrap safe
                        self.suspend_inhibitor = Some(file);
                        println!("Successfully acquired suspend inhibitor");
                    }
                    Ok(Err(err)) => {
                        eprintln!("Failed to acquire inhibit: {err}");
                    }
                    Err(arc) => {
                        // Multiple Arc references exist - this shouldn't happen in normal flow
                        // but handle it gracefully anyway as things that shouldn't happen have a habit of happening
                        eprintln!(
                            "Cannot take ownership: Arc has multiple references (count: {})",
                            Arc::strong_count(&arc)
                        );
                    }
                }
            }
            PowerMessage::SetSuspendTime(time) => {
                // We create a suspend inhibitor when setting a suspend timer so the timer overrides system settings
                let _inhibitor_task = AppModel::get_suspend_inhibitor();

                let suspend_timer = Timer::new(time, false, &TimerType::Suspend);
                if let Some(database) = self.database.clone() {
                    return Task::perform(
                        async move {
                            Timer::insert(database.pool(), &suspend_timer)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |result| Action::App(Message::TimerMessage(TimerMessage::Created(result))),
                    );
                }
                eprintln!("Database not yet available");
            }
            PowerMessage::SetShutdownTime(time) => {
                // We create a suspend inhibitor when setting a shutdown timer so the timer overrides system settings
                // Otherwise the system might suspend before shutting down and never complete until it wakes up and immedately shuts down
                let _inhibitor_task = AppModel::get_suspend_inhibitor();

                let shutdown_timer = Timer::new(time, false, &TimerType::Shutdown);
                if let Some(database) = self.database.clone() {
                    return Task::perform(
                        async move {
                            Timer::insert(database.pool(), &shutdown_timer)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |result| Action::App(Message::TimerMessage(TimerMessage::Created(result))),
                    );
                }
                eprintln!("Database not yet available");
            }
            PowerMessage::SetLogoutTime(time) => {
                // We create a suspend inhibitor when setting a logout timer so the timer overrides system settings
                // Otherwise the system might suspend before logging out and never complete until it wakes up and immedately logs out
                let _inhibitor_task = AppModel::get_suspend_inhibitor();

                let logout_timer = Timer::new(time, false, &TimerType::Logout);
                if let Some(database) = self.database.clone() {
                    return Task::perform(
                        async move {
                            Timer::insert(database.pool(), &logout_timer)
                                .await
                                .map_err(|e| e.to_string())
                        },
                        |result| Action::App(Message::TimerMessage(TimerMessage::Created(result))),
                    );
                }
                eprintln!("Database not yet available");
            }
            PowerMessage::ExecuteSuspend => {
                if let Err(e) = resources::execute_system_suspend() {
                    eprintln!("Failed to suspend system: {e}");
                }
            }
            PowerMessage::ExecuteShutdown => {
                if let Err(e) = resources::execute_system_shutdown() {
                    eprintln!("Failed to shutdown system: {e}");
                }
            }
            PowerMessage::ExecuteLogout => {
                println!("Executing system logout");
                if let Err(e) = resources::execute_system_logout() {
                    eprintln!("Failed to logout system: {e}");
                }
            }
        }
        Task::none()
    }

    fn get_suspend_inhibitor() -> Task<Action<Message>> {
        Task::perform(
            async move {
                resources::acquire_suspend_inhibit(
                    "Chronomancer",
                    "User requested stay-awake mode",
                    "block",
                )
                .await
                .map_err(|e| e.to_string())
            },
            |result| {
                Action::<Message>::App(Message::PowerMessage(PowerMessage::InhibitAcquired(
                    Arc::new(result),
                )))
            },
        )
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

#[cfg(test)]
mod tests {
    use crate::utils::messages::ComponentMessage;

    use super::*;

    fn get_test_app() -> AppModel {
        AppModel::init(Core::default(), ()).0
    }

    #[test]
    fn test_app_initialization() {
        let app = get_test_app();
        assert_eq!(app.icon_name, "chronomancer-hourglass");
        assert!(
            app.popup.is_none(),
            "Popup should be None on initialization"
        );
        assert!(
            app.database.is_none(),
            "Database should be None on initialization"
        );
        assert!(
            app.suspend_inhibitor.is_none(),
            "Suspend inhibitor should be None on initialization"
        );
        assert!(
            app.active_timers.is_empty(),
            "Active timers should be empty on initialization"
        );
    }

    #[test]
    fn test_popup() {
        let mut app = get_test_app();

        // Initially no popup
        assert!(app.popup.is_none(), "Popup should be closed by default");

        // Toggle open (may be a no-op window task if no main window id yet, but popup Option gets set)
        let _task_open = app.toggle_popup();
        assert!(app.popup.is_some(), "Popup should be opened after toggle");

        // Toggle closed
        let _task_close = app.toggle_popup();
        assert!(
            app.popup.is_none(),
            "Popup should be closed after second toggle"
        );
    }

    #[test]
    fn test_tick_with_active_timer() {
        let mut app = get_test_app();

        // To ensure handle_tick removes it immediately, set ends_at to a past second value.
        let now_sec = chrono::Utc::now().timestamp();
        let expired_timer = Timer {
            id: 1,
            is_recurring: false,
            description: TimerType::UserDefined("Renamon exists reminder".to_string())
                .as_str()
                .to_string(),
            paused_at: 0,
            created_at: now_sec - 5, // created slightly in the past
            ends_at: now_sec - 1,    // already expired
        };
        app.active_timers.push(expired_timer);

        // Handle tick; should remove the expired timer
        let task = app.handle_tick();

        // Active timers list should now be empty
        assert!(
            app.active_timers.is_empty(),
            "Expired timer should be removed after tick"
        );

        // Consume task (cannot inspect internals here)
        let _ = task;
    }

    #[test]
    fn test_handle_tick_no_timers() {
        let mut app = get_test_app();

        // Ensure no active timers
        app.active_timers.clear();

        // Handle tick; should not schedule any follow-up actions since there are no timers.
        let _task = app.handle_tick();

        // State should remain unchanged (still no active timers).
        assert!(
            app.active_timers.is_empty(),
            "Active timers should remain empty after tick"
        );
    }

    #[test]
    fn test_handle_page_message_power_controls() {
        let mut app = get_test_app();

        // Create a sample PageMessage for PowerControls
        let msg = PageMessage::ComponentMessage(ComponentMessage::SubmitPressed);

        // Handle the message
        let task = app.handle_page_message(msg);

        // Since PowerControls does not handle SubmitPressed, no task should be scheduled.
        let _ = task.map(|_action| {
            unreachable!("Uh-oh, spaghettios, SubmitPressed should not produce any action here!");
        });
    }

    // Verify that the tick subscription produces Tick messages at 1-second intervals.
    // We spawn the stream and verify it sends messages, then cancel it.
    #[tokio::test]
    async fn test_tick_subscription_interval() {
        use cosmic::iced::futures::StreamExt;
        use cosmic::iced::stream::channel;
        use tokio::time::{Duration, timeout};

        // Use the same channel function signature as in the actual subscription
        let stream = channel(4, |mut output| async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                if output.send(Message::Tick).await.is_err() {
                    break;
                }
            }
        });

        tokio::pin!(stream);

        // Verify we can receive multiple Tick messages
        for i in 1..=3 {
            let result = timeout(Duration::from_secs(2), stream.next()).await;
            match result {
                Ok(Some(msg)) => {
                    assert!(
                        matches!(msg, Message::Tick),
                        "Expected Tick message, got something else"
                    );
                    println!("Successfully received tick {i}");
                }
                Ok(None) => panic!("Stream closed unexpectedly after {i} ticks"),
                Err(err) => panic!("Timeout waiting for tick {i}. Error: {err}"),
            }
        }
    }

    #[test]
    fn test_update_toggle_popup() {
        let mut app = get_test_app();

        // Initially no popup
        assert!(app.popup.is_none());

        // Send TogglePopup message
        let _task = app.update(Message::TogglePopup);

        // Popup should now be open
        assert!(app.popup.is_some());

        // Toggle again
        let _task = app.update(Message::TogglePopup);

        // Popup should be closed
        assert!(app.popup.is_none());
    }

    #[test]
    fn test_update_config() {
        let mut app = get_test_app();

        let new_config = Config::default();

        // Update config
        let _task = app.update(Message::UpdateConfig(new_config.clone()));

        // Config should be updated (compare equality)
        assert_eq!(app.config, new_config);
    }

    #[test]
    fn test_update_tick_message() {
        let mut app = get_test_app();

        // Create an expired timer
        let now_sec = chrono::Utc::now().timestamp();
        let expired_timer = Timer {
            id: 1,
            is_recurring: false,
            description: TimerType::UserDefined("Test".to_string())
                .as_str()
                .to_string(),
            paused_at: 0,
            created_at: now_sec - 5,
            ends_at: now_sec - 1,
        };
        app.active_timers.push(expired_timer);

        // Send Tick message
        let _task = app.update(Message::Tick);

        // Timer should be removed
        assert!(app.active_timers.is_empty());
    }

    #[test]
    fn test_handle_timer_message_created_success() {
        let mut app = get_test_app();

        let timer = Timer {
            id: 1,
            is_recurring: false,
            description: "Test Timer".to_string(),
            paused_at: 0,
            created_at: chrono::Utc::now().timestamp(),
            ends_at: chrono::Utc::now().timestamp() + 3600,
        };

        let msg = TimerMessage::Created(Ok(timer.clone()));
        let _task = app.update(Message::TimerMessage(msg));

        // Timer should be added to active timers
        assert_eq!(app.active_timers.len(), 1);
        assert_eq!(app.active_timers[0].id, timer.id);
    }

    #[test]
    fn test_handle_timer_message_created_failure() {
        let mut app = get_test_app();

        let msg = TimerMessage::Created(Err("Database error".to_string()));
        let _task = app.update(Message::TimerMessage(msg));

        // No timers should be added
        assert!(app.active_timers.is_empty());
    }

    #[test]
    fn test_handle_timer_message_active_fetched_success() {
        let mut app = get_test_app();

        let first_timer = Timer {
            id: 1,
            is_recurring: false,
            description: "Timer 1".to_string(),
            paused_at: 0,
            created_at: chrono::Utc::now().timestamp(),
            ends_at: chrono::Utc::now().timestamp() + 3600,
        };

        let second_timer = Timer {
            id: 2,
            is_recurring: false,
            description: "Timer 2".to_string(),
            paused_at: 0,
            created_at: chrono::Utc::now().timestamp(),
            ends_at: chrono::Utc::now().timestamp() + 7200,
        };

        let timers = vec![first_timer.clone(), second_timer.clone()];
        let msg = TimerMessage::ActiveFetched(Ok(timers));
        let _task = app.update(Message::TimerMessage(msg));

        // Active timers should be populated
        assert_eq!(app.active_timers.len(), 2);
        assert_eq!(app.active_timers[0].id, first_timer.id);
        assert_eq!(app.active_timers[1].id, second_timer.id);
    }

    #[test]
    fn test_handle_timer_message_active_fetched_failure() {
        let mut app = get_test_app();

        // Add a timer first
        app.active_timers.push(Timer {
            id: 1,
            is_recurring: false,
            description: "Existing Timer".to_string(),
            paused_at: 0,
            created_at: chrono::Utc::now().timestamp(),
            ends_at: chrono::Utc::now().timestamp() + 3600,
        });

        let msg = TimerMessage::ActiveFetched(Err("Fetch failed".to_string()));
        let _task = app.update(Message::TimerMessage(msg));

        // Existing timers should remain unchanged
        assert_eq!(app.active_timers.len(), 1);
    }

    #[test]
    fn test_handle_database_message_failed_to_initialize() {
        let mut app = get_test_app();

        let msg = DatabaseMessage::FailedToInitialize("Connection error".to_string());
        let _task = app.update(Message::DatabaseMessage(msg));

        // Database should remain None
        assert!(app.database.is_none());
    }

    #[test]
    fn test_handle_power_message_toggle_stay_awake_acquire() {
        let mut app = get_test_app();

        // Initially no inhibitor
        assert!(app.suspend_inhibitor.is_none());

        // Send ToggleStayAwake message (should acquire inhibitor via task)
        let _task = app.update(Message::PowerMessage(PowerMessage::ToggleStayAwake));

        // Note: The actual inhibitor acquisition happens in the async task,
        // so we can only verify the task is created, not that inhibitor is set
        // The inhibitor will be None until the task completes
        assert!(app.suspend_inhibitor.is_none());
    }

    #[test]
    fn test_handle_power_message_toggle_stay_awake_release() {
        let mut app = get_test_app();

        // Create a fake file descriptor for testing
        // Note: This is a simplified test; in production we'd mock the resource module
        let temp_file = std::env::temp_dir().join("chronomancer_test_inhibitor");
        let file = std::fs::File::create(&temp_file).unwrap();
        app.suspend_inhibitor = Some(file);

        assert!(app.suspend_inhibitor.is_some());

        // Send ToggleStayAwake message (should release inhibitor)
        let _task = app.update(Message::PowerMessage(PowerMessage::ToggleStayAwake));

        // Inhibitor should be released
        assert!(app.suspend_inhibitor.is_none());

        // Cleanup
        let _ = std::fs::remove_file(&temp_file);
    }

    #[test]
    fn test_handle_power_message_inhibit_acquired_success() {
        let mut app = get_test_app();

        // Create a temporary file to simulate the inhibitor
        let temp_file = std::env::temp_dir().join("chronomancer_test_inhibitor2");
        let file = std::fs::File::create(&temp_file).unwrap();

        let msg = PowerMessage::InhibitAcquired(Arc::new(Ok(file)));
        let _task = app.update(Message::PowerMessage(msg));

        // Inhibitor should be set
        assert!(app.suspend_inhibitor.is_some());

        // Cleanup
        app.suspend_inhibitor = None;
        let _ = std::fs::remove_file(&temp_file);
    }

    #[test]
    fn test_handle_power_message_inhibit_acquired_failure() {
        let mut app = get_test_app();

        let msg = PowerMessage::InhibitAcquired(Arc::new(Err("Failed to acquire".to_string())));
        let _task = app.update(Message::PowerMessage(msg));

        // Inhibitor should remain None
        assert!(app.suspend_inhibitor.is_none());
    }

    #[test]
    fn test_update_page_message() {
        let mut app = get_test_app();

        let msg = PageMessage::ComponentMessage(ComponentMessage::SubmitPressed);
        let _task = app.update(Message::PageMessage(msg));

        // The message is forwarded to power_controls.update()
        // We can't easily verify internal state without exposing it,
        // but we can verify no panic occurs
        // and trust the damn thing to test itself in its own tests
    }

    #[test]
    fn test_multiple_timer_creation_and_removal() {
        let mut app = get_test_app();
        let now = chrono::Utc::now().timestamp();

        // Create multiple timers
        for i in 1..=3 {
            let timer = Timer {
                id: i,
                is_recurring: false,
                description: format!("Timer {i}"),
                paused_at: 0,
                created_at: now,
                ends_at: now + 3600 + (i * 100),
            };
            let msg = TimerMessage::Created(Ok(timer));
            let _task = app.update(Message::TimerMessage(msg));
        }

        assert_eq!(app.active_timers.len(), 3);

        // Create an expired timer
        let expired = Timer {
            id: 4,
            is_recurring: false,
            description: "Expired".to_string(),
            paused_at: 0,
            created_at: now - 10,
            ends_at: now - 1,
        };
        let msg = TimerMessage::Created(Ok(expired));
        let _task = app.update(Message::TimerMessage(msg));

        assert_eq!(app.active_timers.len(), 4);

        // Tick should remove expired timer
        let _task = app.update(Message::Tick);

        assert_eq!(app.active_timers.len(), 3);
    }
}
