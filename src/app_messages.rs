// SPDX-License-Identifier: MIT

//! Application message types for the Chronomancer panel applet.
//!
//! This module defines the message hierarchy used throughout the application.
//! We follow the Model-View-Update (MVU) architecture in libcosmic's documentation while
//! adapting it tho have a clear separation between UI pages and service layer operations.
//! Messages flow from UI interactions down to service layer operations and back
//! up as results. The main `AppMessage` enum dispatches to specialized message
//! types for logically divided subsystems (database, power management, timers, pages).
//!
//! ## Message Flow Pattern
//!
//! 1. User interacts with UI (button click, text input)
//! 2. Page generates a page-specific message (e.g., `power_controls::Message`)
//! 3. Page converts to `AppMessage` via `From` trait
//! 4. App's `update()` method routes to appropriate handler
//! 5. Handler performs async work, returns result as new message
//! 6. UI updates based on result message
//!
//! ## Examples
//!
//! Page-level messages convert automatically to app-level:
//!
//! ```rust
//! use chronomancer::{app_messages::AppMessage, pages::power_controls};
//!
//! let page_msg = power_controls::Message::ToggleStayAwake;
//! let app_msg: AppMessage = page_msg.into();
//!
//! // Verify the conversion preserves the message type
//! match app_msg {
//!     AppMessage::PowerControlsMessage(power_controls::Message::ToggleStayAwake) => {
//!         // Message was correctly wrapped
//!     }
//!     _ => panic!("Conversion failed"),
//! }
//! ```

use std::{fs::File, sync::Arc};

use crate::{
    config::Config, models::Timer, pages::power_controls, utils::database::SQLiteDatabase,
};

/// Messages related to database operations.
///
/// These messages represent the lifecycle of database initialization and results
/// from database queries. The database is initialized asynchronously on app startup.
#[derive(Debug, Clone)]
pub enum DatabaseMessage {
    /// Database successfully initialized with the given connection
    Initialized(Result<SQLiteDatabase, String>),
    /// Database initialization failed with an error message
    FailedToInitialize(String),
}

/// Messages related to power management operations.
///
/// Handles stay-awake inhibit locks, timed power operations (suspend, logout,
/// shutdown, reboot), and immediate execution of those operations. Inhibit
/// locks prevent the system from sleeping while active without overriding user settings.
#[derive(Debug, Clone)]
pub enum PowerMessage {
    /// Toggle the stay-awake inhibit lock on/off
    ToggleStayAwake,
    /// Result of acquiring a systemd inhibit lock (wrapped in Arc for cheap cloning)
    InhibitAcquired(Arc<Result<File, String>>),
    /// Schedule a suspend operation after the given number of seconds
    SetSuspendTime(i32),
    /// Schedule a logout operation after the given number of seconds
    SetLogoutTime(i32),
    /// Schedule a shutdown operation after the given number of seconds
    SetShutdownTime(i32),
    /// Schedule a reboot operation after the given number of seconds
    SetRebootTime(i32),
    /// Immediately execute a system suspend
    ExecuteSuspend,
    /// Immediately execute a user logout
    ExecuteLogout,
    /// Immediately execute a system shutdown
    ExecuteShutdown,
    /// Immediately execute a system reboot
    ExecuteReboot,
}

/// Messages related to timer operations.
///
/// Represents results from timer creation and retrieval operations. Timers
/// are stored in the database and tracked for countdown display and notifications.
#[derive(Debug, Clone)]
pub enum TimerMessage {
    /// Result of creating a new timer (contains the created Timer on success)
    Created(Result<Timer, String>),
    /// Result of fetching all active timers from the database
    ActiveFetched(Result<Vec<Timer>, String>),
}

/// Top-level application messages that coordinate all subsystems.
///
/// This is the main message type handled by the app's `update()` method. It
/// dispatches to specialized handlers based on message category. Page-specific
/// messages are automatically converted via the `From` trait implementations.
#[derive(Debug, Clone)]
pub enum AppMessage {
    /// Toggle the applet's popup window open/closed
    TogglePopup,
    /// Update the app configuration (triggers save to disk)
    UpdateConfig(Config),
    /// Regular tick for timer countdown updates (fires every second)
    Tick,
    /// Message from the power controls page (auto-converted via From trait)
    PowerControlsMessage(power_controls::Message),
    /// Message from database operations
    DatabaseMessage(DatabaseMessage),
    /// Message from timer operations
    TimerMessage(TimerMessage),
    /// Message from power management operations
    PowerMessage(PowerMessage),
}

/// Automatic conversion from power controls page messages to app messages.
///
/// Allows pages to return their own message types which are seamlessly converted
/// to `AppMessage` when passed up to the app's `update()` method. This keeps
/// page code decoupled from app-level concerns and lets us write messasge agnostic view and update functionsS.
impl From<power_controls::Message> for AppMessage {
    fn from(msg: power_controls::Message) -> Self {
        AppMessage::PowerControlsMessage(msg)
    }
}
