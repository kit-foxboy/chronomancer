//! Generic form field components using the Strategy pattern.
//!
//! This module provides a trait-based system for creating reusable, type-safe
//! form fields. Each field type is a "strategy" that knows how to capture,
//! validate, and extract one piece of typed input. The consuming page or component is responsible for layout and message mapping.
//! I know this isn't OOP, but the principle is sound and it fits Rust's trait system well.
//!
//! # Architecture
//!
//! - [`FormField`] - Trait defining the interface for all form fields
//! - [`TextField`] - Freeform text input field
//! - [`NumericField`] - Positive integer input with filtering
//! - [`ComboField`] - Dropdown selection from a list of options
//!
//! # Design Philosophy (We'll dignify this on the fly experiment as such X3)
//!
//! Form fields are **purely functional data components**. They own their state,
//! validate their input, and render their widget — but they don't own layout.
//! Layout is the page's responsibility. Fields provide `Element`s via `view()`,
//! and the page arranges them however it wants (rows, columns, grids, etc.).
//!
//! This follows libcosmic's pattern where widgets take message-producing
//! closures rather than owning message types. Each field's `view()` method
//! accepts a closure that wraps the field's raw input into the consumer's
//! message type. It also keeps our message handoffs from being too wacky.
//!
//! # Strategy Pattern
//!
//! Smarter people than me have good ideas on ways to structure code. This is a rusty implementation of the OOP strategy pattern.
//! Each concrete field type *is* a strategy for "how to capture and validate
//! one piece of input." The [`FormField`] trait defines the contract:
//!
//! - **Associated `Value` type** — compile-time guarantee of what each field produces because rust WILL yell at us if we try to do this on the fly with downcasting or something hacky like that
//! - **`view()`** — renders the widget, takes a closure for message mapping
//! - **`update()`** — handles raw input with validation/filtering
//! - **`validate()`** — checks if current state is submittable
//! - **`value()`** — extracts the typed value (None if invalid, I love Some/None SO much as a built in null object pattern)
//! - **`clear()`** — resets to default state
//!
//! # Usage
//!
//! Fields are composed into page-level or component-level structs. The consumer
//! wires up messages and layout:
//!
//! ```ignore
//! use crate::components::form::{TextField, NumericField, ComboField};
//!
//! struct MyForm {
//!     name: TextField,
//!     duration: NumericField,
//!     unit: ComboField<TimeUnit>,
//! }
//!
//! impl MyForm {
//!     fn view(&self) -> Element<'_, MyMessage> {
//!         let name = self.name.view(MyMessage::NameChanged);
//!         let duration = self.duration.view(MyMessage::DurationChanged);
//!         let unit = self.unit.view(MyMessage::UnitChanged);
//!
//!         // Page controls layout
//!         column![name, row![duration, unit]].into()
//!     }
//! }
//! ```

pub mod combo_field;
pub mod numeric_field;
pub mod text_field;

pub use combo_field::ComboField;
pub use numeric_field::NumericField;
pub use text_field::TextField;

use cosmic::Element;

/// Trait defining the interface for a form field component.
///
/// Each implementor is a strategy for capturing, validating, and extracting
/// one piece of typed input. The associated `Value` type provides compile-time
/// safety — `TextField::value()` returns `Option<String>`, while
/// `NumericField::value()` returns `Option<i32>`, with no downcasting needed.
///
/// # Associated Types
///
/// - `Value` — the typed output this field produces on successful extraction
///
/// # Lifecycle
///
/// 1. **Render**: `view()` produces an `Element` wired to the consumer's message type
/// 2. **Update**: `update()` is called with raw input (e.g., from `on_input` callback)
/// 3. **Validate**: `validate()` checks if the field is ready for submission
/// 4. **Extract**: `value()` returns `Some(Value)` if valid, `None` otherwise
/// 5. **Reset**: `clear()` returns the field to its default state
///
/// # Layout Responsibility
///
/// Fields do **not** handle layout. They produce a single `Element` from `view()`.
/// The consuming page or component arranges fields into rows, columns, or any
/// other layout structure. This keeps fields reusable across different visual
/// contexts (applet popups, full app dialogs, settings pages, etc.).
///
/// # Message Mapping
///
/// Following libcosmic's pattern, `view()` takes a closure that maps the field's
/// raw input type to the consumer's message type. This avoids fields needing to
/// know about application-level message enums:
///
/// ```ignore
/// // The field produces String from on_input
/// // The closure wraps it into the page's message
/// let element = text_field.view(MyPageMessage::NameChanged);
/// ```
///
/// # Examples
///
/// ```ignore
/// use crate::components::form::{FormField, TextField};
///
/// let mut field = TextField::new("Name").placeholder("Enter name...");
///
/// // Update with user input
/// field.update("Hello");
///
/// // Check validity
/// assert!(field.validate());
///
/// // Extract typed value
/// assert_eq!(field.value(), Some("Hello".to_string()));
///
/// // Reset
/// field.clear();
/// assert_eq!(field.value(), None);
/// ```
pub trait FormField {
    /// The typed value this field produces on successful extraction.
    type Value;

    /// Renders the field as an [`Element`], mapping input events via the closure.
    ///
    /// The `on_input` closure converts the field's raw input (always `String`
    /// from text-based widgets) into the consumer's message type. This follows
    /// the same pattern as `cosmic::widget::TextInput::on_input`.
    ///
    /// I used M for the generic because it stands for Message, and it's a common convention in Rust to use single-letter generics for this kind of thing I've noticed.
    ///
    /// # Arguments
    ///
    /// - `on_input` - Closure mapping raw input `String` to consumer's message type
    fn view<M: Clone + 'static>(&self, on_input: impl Fn(String) -> M + 'static) -> Element<'_, M>;

    /// Updates the field state with new raw input.
    ///
    /// Implementations apply any filtering or transformation (e.g., rejecting
    /// non-numeric characters). Invalid input may be silently ignored.
    /// This isn't the web where we have to worry about user input being malicious, so we can be a little more relaxed about it. If the input is invalid, the field just won't update its internal state, and `validate()` will return false until valid input is provided.
    ///
    /// # Arguments
    ///
    /// - `input` - Raw input string from the widget callback
    fn update(&mut self, input: &str);

    /// Returns whether the field's current state is valid for submission.
    ///
    /// This is a quick check used by form-level submit logic to determine
    /// if all fields are ready. This may not be strictly necessary since input validation is handled in `update()`, but it could be useful if we later want to add more complex validation rules that depend on multiple fields or external state.
    /// For now, it can just check if the internal state is in a valid configuration.
    fn validate(&self) -> bool;

    /// Extracts the typed value if the field is valid.
    ///
    /// Returns `Some(Self::Value)` when the field contains valid, extractable
    /// data, or `None` if the field is empty or invalid.
    fn value(&self) -> Option<Self::Value>;

    /// Resets the field to its default/empty state.
    fn clear(&mut self);
}
