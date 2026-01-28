//! List-related UI components.
//!
//! This module provides components for building list interfaces, including
//! headers, forms, items, and container components.
//!
//!
//! # Components
//!
//! - [`ListHeader`] - Header component for list sections with title and optional action button
//! - [`ListHeaderForm`] - Form variant of ListHeader for embedding input fields
//!
//! # Component Organization
//!
//! List components are grouped in this submodule to keep the codebase organized
//! as the number of list-related components grows. Each component type gets its
//! own file (e.g., `header.rs`, `header_form.rs`).
//!
//! # Builder Pattern with Variants
//!
//! After some experimenting, here's the thing I settled on. Components use **Rust enums** for behavioral variants and the **builder pattern**
//! for configuration. This separates:
//!
//! - **Behavioral context** (`Context::App` vs `Context::Applet`) - Different interaction patterns
//! - **Space constraints** (`Layout::Compact` vs `Layout::Spacious`) - Visual density
//!
//! ## Example Usage
//!
//! ### ListHeader
//!
//! ```ignore
//! use crate::components::{Context, Layout, ListHeader};
//!
//! // Minimal - all defaults (App context, Comfortable layout)
//! ListHeader::new("Timers");
//!
//! // App with spacious layout and text button
//! ListHeader::new("Active Timers")
//!     .layout(Layout::Spacious)
//!     .with_add_button()
//!     .button_text("Add Timer");
//!
//! // Applet context with icon-only button
//! ListHeader::new("Recent")
//!     .context(Context::Applet)
//!     .layout(Layout::Compact)
//!     .with_add_button();
//!
//! // Or use the preset constructors
//! ListHeader::applet_with_add("Timers");
//! ListHeader::app_with_add("Timers", "Add Timer");
//! ```
//!
//! ### ListHeaderForm
//!
//! ```ignore
//! use crate::components::{Context, Layout};
//! use crate::components::list::ListHeaderForm;
//!
//! // Minimal form
//! ListHeaderForm::new("Add Timer");
//!
//! // Applet form with placeholder
//! ListHeaderForm::new("New Timer")
//!     .context(Context::Applet)
//!     .layout(Layout::Compact)
//!     .placeholder("Timer name...")
//!     .value(&self.input_value);
//!
//! // App form with submit button text
//! ListHeaderForm::new("Create Reminder")
//!     .layout(Layout::Spacious)
//!     .placeholder("Reminder name")
//!     .submit_text("Create");
//!
//! // Or use the preset constructors
//! ListHeaderForm::applet("New Timer");
//! ListHeaderForm::app_with_submit("New Timer", "Create");
//! ```
//!
//! ## Key Differences
//!
//! ### Applet Context
//! - Icon-only buttons by default
//! - Inline forms (no navigation)
//! - Popup confirmations
//! - Always space-constrained
//!
//! ### App Context
//! - Icon + text buttons available
//! - Separate screens/dialogs
//! - Can adapt layout to window size
//! - Navigation patterns available
//!
//! ## Benefits (in theory)
//!
//! - **Flexible** - Both behavioral AND visual configuration
//! - **Type-safe** - Enums prevent invalid combinations (no booleans)
//! - **Readable** - `.context(Context::Applet)` is clear and explicit
//! - **Extensible** - Easy to add new contexts or layouts
//!
//! # Design Notes
//!
//! See `.github/component-builder-pattern.md` for detailed implementation guide.

pub mod header;
#[allow(dead_code)]
pub mod header_form;

pub use header::ListHeader;
pub use header_form::ListHeaderForm;
