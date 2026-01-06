//! Module for different pages in the Chronomancer application.
//!
//! This module contains the definitions and implementations of various pages (screens or screen areas in applets)
//! used in the Chronomancer application. Each page is responsible for managing
//! its own state and behavior, and they interact with the overall application state.
//! This module re-exports the pages for easier access.
//! Pages represent different screens or screen areas in the application.
//! They sit between the application state and the UI components.
//! Pages are responsible for managing the state and behavior of their respective screens.
//!
//! # Pages
//!
//! - [`PowerControls`] - Page for scheduling system power operations like shutdown and suspend.
//! - [`TimerList`] - Page for displaying and managing a list of timers.
//!
//! # Design Principles
//!
//! All pages in this module follow these principles:
//!
//! 1. **Encapsulated State** - Each page manages its own state and behavior.
//! 2. **Composable** - Pages can be composed of multiple UI components.
//! 3. **Documented** - Each page and method includes arguments, return values, and possible errors. Jury is still out on whether or not these will end up being doctests
//! 4. **Organized** - Pages are organized in a way that reflects their purpose and functionality, grouping reusable componetns together.
//! 5. **Overengineered** - ...probably...
//!

pub mod power_controls;
pub mod timer_list;

pub use power_controls::Message as PowerControlsMessage;
pub use power_controls::Page as PowerControls;
pub use timer_list::Message as TimerListMessage;
pub use timer_list::Page as TimerList;

#[derive(Debug, Clone)]
pub enum PageMessage {
    PowerControlsMessage(PowerControlsMessage),
    TimerListMessage(TimerListMessage),
}

/// Automatic conversion from power controls messages to unified page messages.
///
/// This enables seamless conversion chains: `power_controls::Message` -> `PageMessage` -> `AppMessage`.
/// In view functions, you can now use `.map(Into::into)` instead of explicit wrapping.
impl From<PowerControlsMessage> for PageMessage {
    fn from(msg: PowerControlsMessage) -> Self {
        PageMessage::PowerControlsMessage(msg)
    }
}

/// Automatic conversion from timer list messages to unified page messages.
///
/// This enables seamless conversion chains: `timer_list::Message` -> `PageMessage` -> `AppMessage`.
/// In view functions, you can now use `.map(Into::into)` instead of explicit wrapping.
impl From<TimerListMessage> for PageMessage {
    fn from(msg: TimerListMessage) -> Self {
        PageMessage::TimerListMessage(msg)
    }
}
