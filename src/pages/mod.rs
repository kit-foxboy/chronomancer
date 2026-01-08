//! Module for different pages in the Chronomancer application.
//!
//! This module contains the definitions and implementations of various pages (screens or screen areas in applets)
//! used in the Chronomancer application. Each page is responsible for managing
//! its own state and behavior, and they interact with the overall application state.
//!
//! Pages represent different screens or screen areas in the application.
//! They sit between the application state and the UI components.
//!
//! # Pages
//!
//! - [`PowerControls`] - Page for scheduling system power operations like shutdown and suspend.
//! - [`TimerList`] - Page for displaying and managing a list of timers.
//!
//! # Design Principles
//!
//! Each page has its own `Message` type for page-specific events. These are automatically
//! converted to `AppMessage` through the `PageMessage` wrapper and `From` trait implementations.
//!
//! 1. **Encapsulated State** - Each page manages its own state and behavior.
//! 2. **Composable** - Pages can be composed of multiple UI components.
//! 3. **Documented** - Each page and method includes arguments, return values, and possible errors. Jury is still out on whether or not these will end up being doctests
//! 4. **Organized** - Pages are organized in a way that reflects their purpose and functionality, grouping reusable componetns together.
//! 5. **Overengineered** - ...probably...
//!
//! This enables clean `.map(Into::into)` syntax in view functions without manual wrapping.

pub mod power_controls;
pub mod timer_list;

pub use power_controls::Message as PowerControlsMessage;
pub use power_controls::Page as PowerControls;
pub use timer_list::Message as TimerListMessage;
pub use timer_list::Page as TimerList;

/// Wrapper enum for all page-specific messages.
///
/// This enum wraps messages from individual pages, enabling automatic conversion
/// to `AppMessage` through the `From` trait. Pages don't need to know about
/// `AppMessage` - they only work with their own message types.
///
/// # Conversion Chain
///
/// ```text
/// power_controls::Message → PageMessage → AppMessage
/// timer_list::Message → PageMessage → AppMessage
/// ```
///
/// # Example
///
/// ```rust,ignore
/// // In a page's view function:
/// self.power_controls.view().map(Into::into)
/// // Compiler chain: Message → PageMessage (via From) → AppMessage (via From)
/// ```
#[derive(Debug, Clone)]
pub enum PageMessage {
    /// Message from the power controls page
    PowerControlsMessage(PowerControlsMessage),
    /// Message from the timer list page
    TimerListMessage(TimerListMessage),
}

/// Automatic conversion from power controls messages to page messages.
///
/// First step in the conversion chain to `AppMessage`.
impl From<PowerControlsMessage> for PageMessage {
    fn from(msg: PowerControlsMessage) -> Self {
        PageMessage::PowerControlsMessage(msg)
    }
}

/// Automatic conversion from timer list messages to page messages.
///
/// First step in the conversion chain to `AppMessage`.
impl From<TimerListMessage> for PageMessage {
    fn from(msg: TimerListMessage) -> Self {
        PageMessage::TimerListMessage(msg)
    }
}
