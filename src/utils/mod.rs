//! Utility modules for non-UI helper functions and system integration.
//!
//! This module contains reusable utilities that don't involve rendering UI
//! or handling messages. These are pure functions, data structures, and
//! system integration helpers used throughout Chronomancer.
//!
//! # Organization
//!
//! - [`database`] - Database abstractions and `SQLite` implementation
//! - [`filters`] - Text input validation and filtering functions
//! - [`resources`] - System icons and power management (D-Bus integration)
//! - [`time`] - Time unit conversion and duration formatting
//! - [`ui`] - UI spacing, sizing, and padding constants
//!
//! # Module Philosophy
//!
//! Utils are distinct from components in that they:
//! - **Don't render UI** - No `view()` methods or `Element` returns
//! - **Don't handle messages** - No message types or MVU patterns
//! - **Are reusable** - Can be used anywhere in the application
//! - **Are testable** - Pure functions or simple abstractions
//!
//! See `.github/architectural-idioms.md` for detailed guidelines on
//! when to use utils vs components.
//!
//! # Examples
//!
//! ## Text input validation
//!
//! ```rust
//! use chronomancer::utils::filters;
//!
//! // Validate positive integer input
//! if let Some(filtered) = filters::filter_positive_integer("42") {
//!     println!("Valid input: {}", filtered);
//! }
//! ```
//!
//! ## Time formatting
//!
//! ```rust
//! use chronomancer::utils::time;
//!
//! let seconds = 3600;
//! let formatted = time::format_duration(seconds);
//! assert_eq!(formatted, "1 hour");
//! ```
//!
//! ## System power operations
//!
//! ```rust,no_run
//! use chronomancer::utils::resources;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Suspend the system
//! resources::execute_system_suspend().await?;
//! # Ok(())
//! # }
//! ```
//!

pub mod database;
pub mod filters;
pub mod power;
pub mod resources;
pub mod time;
pub mod ui;

pub use time::{TimeUnit, format_duration};
#[allow(dead_code)]
pub use ui::Padding;
