//! Chronomancer library entry point.
//!
//! This file exposes internal modules so they can be used by integration tests
//! (placed in the `tests/` directory). The binary (`main.rs`) drives the COSMIC
//! applet runtime, while this `lib.rs` lets us test the domain & utility layers.

// Core modules
pub mod app_messages;
pub mod config;
pub mod i18n;
pub mod models;
pub mod utils;

// UI layer (for component/message tests)
pub mod components;
pub mod pages;

// Convenient re-exports for common types
pub use config::Config;
pub use models::timer::{Timer, TimerType};
pub use utils::time::TimeUnit;
