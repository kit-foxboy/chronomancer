//! Reusable UI components for the Chronomancer application.
//!
//! This module provides a collection of reusable components that can be composed
//! to build the application's user interface. Components follow libcosmic's patterns (at least those that exist...)
//! and are designed to be generic, type-safe, and easy to integrate.
//!
//! # Component Categories
//!
//! ## List Components
//!
//! List-related components are organized in the [`list`] submodule.
//!
//! - [`list::ListHeader`] - Header component for list sections with title and optional action button
//! - [`list::ListHeaderForm`] - Form variant of ListHeader for embedding input fields
//!
//! ## Input Components
//!
//! - [`PowerForm`] - Time duration input form with unit selection and validation
//!
//! For text input filtering and validation helpers, see [`crate::utils::filters`].
//!
//! ## Selection Components
//!
//! - [`RadioComponents`](radio_components::RadioComponents) - Generic radio button group manager
//! - [`ToggleIconRadio`] - Icon-based radio button option
//! - [`RadioComponent`](radio_components::RadioComponent) - Trait for implementing custom radio options
//!
//! ## Power Management
//!
//! - [`PowerOperation`](power_form::PowerOperation) - System power operation types (suspend, shutdown, etc.)
//!
//! # Design Principles
//!
//! All components in this module follow these principles:
//!
//! 1. **Generic over Message types** - Components work with any application message type
//! 2. **Composable** - Components can be nested and combined
//! 3. **Clone-able** - Components can be stored in application state
//! 4. **Documented** - Each component and method includes arguments, return values, and possible errors. Jury is still out on whether or not these will end up being doctests

pub mod icon_button;
pub mod list;
pub mod power_form;
pub mod radio_components;
pub mod types;

pub use icon_button::ToggleIconRadio;
// TODO: Remove this allow and ensure imports are needed
#[allow(unused_imports)]
pub use list::{ListHeader, ListHeaderForm};
pub use power_form::PowerForm;
pub use types::{Context, Layout};
