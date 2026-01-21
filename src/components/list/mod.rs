//! List-related UI components.
//!
//! This module provides components for building list interfaces, including
//! headers, forms, items, and container components.
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
//! Components use **Rust enums** for behavioral variants and the **builder pattern**
//! for configuration. This separates:
//!
//! - **Behavioral context** (`Context::App` vs `Context::Applet`) - Different interaction patterns
//! - **Space constraints** (`Layout::Compact` vs `Layout::Spacious`) - Visual density
//!
//! ## Example Usage
//!
//! ```ignore
//! // App context (default) with compact layout
//! ListHeader::new("Timers")
//!     .with_add_button();
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
//!     .with_add_button();
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
//! ## Benefits
//!
//! - **Flexible** - Both behavioral AND visual configuration
//! - **Type-safe** - Enums prevent invalid combinations (no booleans)
//! - **Readable** - `.context(Context::Applet)` is clear and explicit
//! - **Extensible** - Easy to add new contexts or layouts
//!
//! # Design Notes
//!
//! List components prioritize:
//! - **Context-aware behavior** - Different patterns for applets vs. apps
//! - **Efficient layouts** - Respect space constraints
//! - **Clear hierarchy** - Visual distinction between headers, items, and actions
//! - **Composability** - Easy to combine with other components
//!
//! See `.github/component-builder-pattern.md` for detailed implementation guide.

pub mod header;
pub mod header_form;

pub use header::ListHeader;
pub use header_form::ListHeaderForm;
