// SPDX-License-Identifier: MIT

//! Application message types for the Chronomancer panel applet.
//!
//! This module defines the top-level message type used throughout the application.
//! We follow the Model-View-Update (MVU) architecture from libcosmic's documentation,
//! with messages organized in their respective modules (pages, models, utils) and
//! automatically converted to `AppMessage` via the `From` trait.
//!
//! ## Message Flow Pattern
//!
//! 1. User interacts with UI (button click, text input)
//! 2. Component/page generates a module-specific message
//! 3. Message converts to `AppMessage` via `From` trait (automatic with `.map(Into::into)`)
//! 4. App's `update()` method routes to appropriate handler
//! 5. Handler performs work, returns result as new message
//! 6. UI updates based on result message
//!
//! ## Architecture
//!
//! Messages are **decentralized** as per libcosmic examples - each module owns its message types:
//! - **Page messages**: `pages::PowerControlsMessage`, `pages::TimerListMessage`
//! - **Service messages**: `utils::power::PowerMessage`, `utils::database::DatabaseMessage`
//! - **Model messages**: `models::timer::TimerMessage`
//!
//! All convert cleanly to `AppMessage` through a predictable chain:
//! - Page messages: `page::Message` → `PageMessage` → `AppMessage`
//! - Service messages: `service::Message` → `AppMessage`
//!
//! ## Examples
//!
//! ### Automatic Conversion in View Functions
//!
//! ```rust,ignore
//! // Clean syntax using Into trait - no manual wrapping needed
//! let power_view = self.power_controls.view().map(Into::into);
//! let timer_view = self.timer_list.view().map(Into::into);
//! ```
//!
//! ### Message Type Conversions
//!
//! ```rust
//! use chronomancer::{app_messages::AppMessage, pages::{PageMessage, PowerControlsMessage}};
//!
//! // Page message converts through PageMessage wrapper
//! let page_msg = PowerControlsMessage::ToggleStayAwake;
//! let app_msg: AppMessage = page_msg.into();
//!
//! // Verify the conversion chain
//! match app_msg {
//!     AppMessage::Page(PageMessage::PowerControlsMessage(
//!         PowerControlsMessage::ToggleStayAwake
//!     )) => {
//!         // Message was correctly wrapped
//!     }
//!     _ => panic!("Conversion failed"),
//! }
//! ```

use crate::config::Config;

// Re-export message types for convenient importing throughout the app
pub use crate::{
    models::timer::TimerMessage,
    pages::PageMessage,
    utils::{database::DatabaseMessage, power::PowerMessage},
};

/// Top-level application messages that coordinate all subsystems.
///
/// This is the main message type handled by the app's `update()` method. It
/// dispatches to specialized handlers based on message category. All module-specific
/// messages are automatically converted to `AppMessage` via `From` trait implementations.
///
/// # Message Categories
///
/// - **`TogglePopup`**: UI control for showing/hiding the applet popup
/// - **`UpdateConfig`**: Persistence operations for app configuration
/// - **`Tick`**: Regular timer updates (every second via subscription)
/// - **`Page`**: Messages from UI pages (power controls, timer list, etc.)
/// - **`Database`**: Database lifecycle and query results
/// - **`Timer`**: Timer creation and retrieval results
/// - **`Power`**: Power management operations (suspend, shutdown, inhibitors)
#[derive(Debug, Clone)]
pub enum AppMessage {
    /// Toggle the applet's popup window open/closed
    TogglePopup,
    /// Update the app configuration (triggers save to disk)
    UpdateConfig(Config),
    /// Regular tick for timer countdown updates (fires every second)
    Tick,
    /// Message from any page (power controls, timer list, etc.)
    Page(PageMessage),
    /// Message from database operations
    Database(DatabaseMessage),
    /// Message from timer operations
    Timer(TimerMessage),
    /// Message from power management operations
    Power(PowerMessage),
}

/// Automatic conversion from page messages to app messages.
///
/// Allows pages to return their own message types which are seamlessly converted
/// to `AppMessage` when passed up to the app's `update()` method. This keeps
/// page code decoupled from app-level concerns. This keeps us from some ugly ass syntax with highly nested enums.
///
/// # Example
///
/// ```rust,ignore
/// // In a view function, this just works:
/// self.power_controls.view().map(Into::into)
/// // Compiler automatically uses: PageMessage → AppMessage
/// ```
impl From<PageMessage> for AppMessage {
    fn from(msg: PageMessage) -> Self {
        AppMessage::Page(msg)
    }
}

/// Automatic conversion from database messages to app messages.
impl From<DatabaseMessage> for AppMessage {
    fn from(msg: DatabaseMessage) -> Self {
        AppMessage::Database(msg)
    }
}

/// Automatic conversion from timer messages to app messages.
impl From<TimerMessage> for AppMessage {
    fn from(msg: TimerMessage) -> Self {
        AppMessage::Timer(msg)
    }
}

/// Automatic conversion from power messages to app messages.
impl From<PowerMessage> for AppMessage {
    fn from(msg: PowerMessage) -> Self {
        AppMessage::Power(msg)
    }
}
