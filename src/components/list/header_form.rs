//! A form variant of the list header component.
//!
//! This component combines a list header with an embedded form for inline
//! item creation, commonly used in panel applets for space-efficient workflows.
//!
//! # Refactored Architecture
//!
//! `ListHeaderForm` now composes [`TextField`](crate::components::form::TextField),
//! [`NumericField`](crate::components::form::NumericField), and
//! [`ComboField`](crate::components::form::ComboField) from the generic form field
//! system rather than duplicating input/validation/clear logic. The form fields
//! handle their own state, validation, and rendering — `ListHeaderForm` orchestrates
//! them and adds action buttons + layout.
//!
//! # Builder Pattern
//!
//! `ListHeaderForm` uses the builder pattern for flexible configuration:
//!
//! ```ignore
//! // Minimal - all defaults (App context, Comfortable layout)
//! ListHeaderForm::new("Add Timer");
//!
//! // Applet context with compact layout
//! ListHeaderForm::new("New Timer")
//!     .context(Context::Applet)
//!     .layout(Layout::Compact)
//!     .name_placeholder("Timer name...")
//!     .duration_placeholder("Duration...");
//!
//! // App context with submit button text
//! ListHeaderForm::new("Create Reminder")
//!     .context(Context::App)
//!     .layout(Layout::Spacious)
//!     .name_placeholder("Reminder name")
//!     .submit_text("Create");
//! ```
//!
//! See `.journal/component-builder-pattern.md` for the full pattern guide.

use cosmic::{
    Element,
    iced::Alignment::Center,
    iced_widget::{column, row},
    theme,
    theme::Button::Icon,
    widget::{button, icon},
};

use crate::{
    components::{
        Context, Layout,
        form::{ComboField, FormField, NumericField, TextField},
    },
    fl,
    utils::ui::ComponentSize,
    utils::{TimeUnit, ui::Spacing},
};

/// Messages emitted by the `ListHeaderForm` component.
#[derive(Debug, Clone)]
pub enum Message {
    /// The name text input value changed.
    NameInputChanged(String),
    /// The duration text input value changed.
    DurationInputChanged(String),
    /// The time unit selection changed.
    TimeUnitChanged(TimeUnit),
    /// The submit button was pressed or Enter was hit.
    Submit,
    /// The cancel button was pressed or form was dismissed.
    Cancel,
}

/// Data returned from a successful form submission.
///
/// Contains the validated name and computed duration in seconds,
/// ready for timer creation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormSubmission {
    /// The timer name entered by the user.
    pub name: String,
    /// The total duration in seconds (value × time unit multiplier).
    pub duration_seconds: i32,
}

/// A list header with an embedded form for adding new items.
///
/// This component is designed for compact interfaces where showing a separate
/// form would be inefficient. It combines the title and action elements of
/// a standard list header with inline input fields for both a name and a
/// duration with time unit selection.
///
/// Composes [`TextField`] for name input, [`NumericField`] for duration input,
/// and [`ComboField<TimeUnit>`] for unit selection. The composed fields handle
/// their own state, validation, filtering, and rendering.
///
/// Supports both App and Applet contexts with different visual behaviors:
/// - **App**: Can show icon + text buttons, adapts to layout
/// - **Applet**: Always icon-only buttons, compact spacing
///
/// # Example
///
/// ```ignore
/// let form = ListHeaderForm::new("Add Timer")
///     .context(Context::Applet)
///     .name_placeholder("Timer name...")
///     .duration_placeholder("Duration...")
///     .view();
/// ```
pub struct ListHeaderForm {
    // Required fields
    title: String,

    // Configuration
    context: Context,
    layout: Layout,

    // Form fields — composed from generic form field components
    name_field: TextField,
    duration_field: NumericField,
    time_unit_field: ComboField<TimeUnit>,

    // Button configuration
    submit_text: Option<String>,

    // Flags
    show_cancel: bool,
}

impl ListHeaderForm {
    /// Creates a new `ListHeaderForm` with App context and Comfortable layout.
    ///
    /// # Arguments
    ///
    /// * `title` - The title/label for the form, used as input ID.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            context: Context::default(),
            layout: Layout::default(),
            name_field: TextField::new("header-form-name"),
            duration_field: NumericField::new("header-form-duration"),
            time_unit_field: ComboField::new(
                "header-form-time-unit",
                vec![
                    TimeUnit::Seconds,
                    TimeUnit::Minutes,
                    TimeUnit::Hours,
                    TimeUnit::Days,
                ],
                TimeUnit::Seconds,
            )
            .label(fl!("unit-label")),
            submit_text: None,
            show_cancel: true,
        }
    }

    /// Sets the behavioral context (App or Applet).
    ///
    /// - **App**: Icon + text buttons available, navigation patterns
    /// - **Applet**: Icon-only buttons, inline interactions
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .context(Context::Applet);
    /// ```
    #[must_use]
    pub fn context(mut self, context: Context) -> Self {
        self.context = context;
        self
    }

    /// Sets the visual layout density.
    ///
    /// This affects spacing, padding, and text size at render time.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .layout(Layout::Spacious);
    /// ```
    #[must_use]
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the placeholder text for the name input field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .name_placeholder("Enter timer name...");
    /// ```
    #[must_use]
    pub fn name_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.name_field = self.name_field.placeholder(placeholder);
        self
    }

    /// Sets the placeholder text for the duration input field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .duration_placeholder("Duration...");
    /// ```
    #[must_use]
    pub fn duration_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.duration_field = self.duration_field.placeholder(placeholder);
        self
    }

    /// Sets the submit button text (App context only).
    ///
    /// In App context, this creates an icon + text button.
    /// In Applet context, this value is stored but not displayed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .submit_text("Create");
    /// ```
    #[must_use]
    pub fn submit_text(mut self, text: impl Into<String>) -> Self {
        self.submit_text = Some(text.into());
        self
    }

    /// Controls whether the cancel button is shown.
    ///
    /// Defaults to `true`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .show_cancel(false);
    /// ```
    #[must_use]
    pub fn show_cancel(mut self, show: bool) -> Self {
        self.show_cancel = show;
        self
    }

    /// Returns layout-specific spacing, padding, and text size from cosmic theme.
    fn layout_values(&self) -> Spacing {
        let cosmic_spacing = theme::active().cosmic().spacing;

        match self.layout {
            Layout::Compact => {
                let gap = cosmic_spacing.space_xs;
                let padding = [
                    cosmic_spacing.space_xxs,
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_xxs,
                    cosmic_spacing.space_xs,
                ];
                Spacing {
                    gap,
                    padding,
                    text_size: ComponentSize::FONT_SIZE_SMALL,
                }
            }
            Layout::Comfortable => {
                let gap = cosmic_spacing.space_s;
                let padding = [
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_s,
                ];
                Spacing {
                    gap,
                    padding,
                    text_size: ComponentSize::FONT_SIZE_DEFAULT,
                }
            }
            Layout::Spacious => {
                let gap = cosmic_spacing.space_m;
                let padding = [
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_m,
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_m,
                ];
                Spacing {
                    gap,
                    padding,
                    text_size: ComponentSize::FONT_SIZE_DEFAULT,
                }
            }
        }
    }

    /// Renders the component as an Element.
    ///
    /// The rendering adapts based on context and configuration:
    /// - Applet → icon-only buttons
    /// - App + `submit_text` → icon + text submit button
    /// - App (no `submit_text`) → icon-only submit button
    #[must_use]
    pub fn view(&self) -> Element<'_, Message> {
        let layout_values = self.layout_values();

        // Render form fields using the FormField trait
        let name_input = self.name_field.view(Message::NameInputChanged);
        let duration_input = self.duration_field.view(Message::DurationInputChanged);
        let unit_combo = self.time_unit_field.view_typed(Message::TimeUnitChanged);

        // Build action buttons based on context
        let action_buttons = match (self.context, self.submit_text.as_ref()) {
            (Context::Applet, _) | (Context::App, None) => self.build_action_buttons_icon_only(),
            (Context::App, Some(text)) => self.build_action_buttons_with_text(text.clone()),
        };

        // Layout: name input, then duration + unit side-by-side, then buttons
        let duration_row = row![duration_input, unit_combo]
            .align_y(Center)
            .spacing(layout_values.gap);

        let button_row = row![action_buttons]
            .align_y(Center)
            .spacing(layout_values.gap);

        column![name_input, duration_row, button_row]
            .spacing(layout_values.gap)
            .padding(layout_values.padding)
            .into()
    }

    /// Builds the action buttons row (submit + optional cancel) with icon-only buttons.
    fn build_action_buttons_icon_only(&self) -> Element<'_, Message> {
        let submit_button = button::icon(icon::from_name("emblem-ok-symbolic"))
            .class(Icon)
            .extra_small()
            .on_press(Message::Submit);

        let mut button_row = row![submit_button];

        if self.show_cancel {
            let cancel_button = button::icon(icon::from_name("window-close-symbolic"))
                .class(Icon)
                .extra_small()
                .on_press(Message::Cancel);
            button_row = button_row.push(cancel_button);
        }

        button_row.align_y(Center).spacing(4).into()
    }

    /// Builds the action buttons row with text submit button (App context).
    fn build_action_buttons_with_text(&self, submit_text: String) -> Element<'_, Message> {
        let submit_button = button::text(submit_text)
            .leading_icon(icon::from_name("emblem-ok-symbolic"))
            .class(Icon)
            .on_press(Message::Submit);

        let mut button_row = row![submit_button];

        if self.show_cancel {
            let cancel_button = button::icon(icon::from_name("window-close-symbolic"))
                .class(Icon)
                .extra_small()
                .on_press(Message::Cancel);
            button_row = button_row.push(cancel_button);
        }

        button_row.align_y(Center).spacing(4).into()
    }

    /// Handles changes to the name text input.
    ///
    /// Delegates to [`TextField::update`] which accepts any text.
    ///
    /// # Arguments
    ///
    /// - `new_text` - The new name input value
    pub fn handle_name_input(&mut self, new_text: &str) {
        self.name_field.update(new_text);
    }

    /// Handles changes to the duration text input with numeric validation.
    ///
    /// Delegates to [`NumericField::update`] which uses
    /// [`filters::filter_positive_integer`](crate::utils::filters::filter_positive_integer)
    /// to validate input.
    ///
    /// # Arguments
    ///
    /// - `new_text` - The new duration input value to validate and apply
    pub fn handle_duration_input(&mut self, new_text: &str) {
        self.duration_field.update(new_text);
    }

    /// Sets the time unit for duration calculation.
    ///
    /// Delegates to [`ComboField::update_selection`].
    ///
    /// # Arguments
    ///
    /// - `unit` - The new time unit to use
    pub fn set_time_unit(&mut self, unit: TimeUnit) {
        self.time_unit_field.update_selection(unit);
    }

    /// Handles form submission, returning a `FormSubmission` if all fields are valid.
    ///
    /// Validates that:
    /// - The name is non-empty (after trimming whitespace) — via [`TextField::validate`]
    /// - The duration is a valid positive integer — via [`NumericField::validate`]
    ///
    /// Returns `Some(FormSubmission)` with the name and computed duration in seconds,
    /// or `None` if either field fails validation.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(submission) = header_form.handle_submit() {
    ///     println!("Name: {}, Duration: {}s", submission.name, submission.duration_seconds);
    /// }
    /// ```
    pub fn handle_submit(&self) -> Option<FormSubmission> {
        let name = self.name_field.value()?;
        let duration_value = self.duration_field.value()?;
        let time_unit = self.time_unit_field.value()?;

        let duration_seconds = duration_value * time_unit.to_seconds_multiplier();

        Some(FormSubmission {
            name,
            duration_seconds,
        })
    }

    /// Clears all form input values and resets the time unit to default.
    ///
    /// Delegates to each field's [`FormField::clear`] method.
    ///
    /// # Example
    ///
    /// ```ignore
    /// header_form.clear();
    /// ```
    pub fn clear(&mut self) {
        self.name_field.clear();
        self.duration_field.clear();
        self.time_unit_field.clear();
    }
}

// Preset constructors for common configurations
impl ListHeaderForm {
    /// Creates an applet form with compact layout.
    ///
    /// Equivalent to:
    /// ```ignore
    /// ListHeaderForm::new(title)
    ///     .context(Context::Applet)
    ///     .layout(Layout::Compact)
    /// ```
    pub fn applet(title: impl Into<String>) -> Self {
        Self::new(title)
            .context(Context::Applet)
            .layout(Layout::Compact)
    }

    /// Creates an app form with spacious layout and submit text.
    ///
    /// Equivalent to:
    /// ```ignore
    /// ListHeaderForm::new(title)
    ///     .context(Context::App)
    ///     .layout(Layout::Spacious)
    ///     .submit_text(submit_text)
    /// ```
    pub fn app_with_submit(title: impl Into<String>, submit_text: impl Into<String>) -> Self {
        Self::new(title)
            .context(Context::App)
            .layout(Layout::Spacious)
            .submit_text(submit_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_app_comfortable() {
        let form = ListHeaderForm::new("Test");
        assert_eq!(form.context, Context::App);
        assert_eq!(form.layout, Layout::Comfortable);
    }

    #[test]
    fn test_applet_context() {
        let form = ListHeaderForm::new("Test").context(Context::Applet);
        assert_eq!(form.context, Context::Applet);
    }

    #[test]
    fn test_compact_layout() {
        let form = ListHeaderForm::new("Test").layout(Layout::Compact);
        assert_eq!(form.layout, Layout::Compact);
    }

    #[test]
    fn test_spacious_layout() {
        let form = ListHeaderForm::new("Test").layout(Layout::Spacious);
        assert_eq!(form.layout, Layout::Spacious);
    }

    #[test]
    fn test_submit_text() {
        let form = ListHeaderForm::new("Test").submit_text("Create");
        assert_eq!(form.submit_text, Some("Create".to_string()));
    }

    #[test]
    fn test_show_cancel_default() {
        let form = ListHeaderForm::new("Test");
        assert!(form.show_cancel);
    }

    #[test]
    fn test_show_cancel_false() {
        let form = ListHeaderForm::new("Test").show_cancel(false);
        assert!(!form.show_cancel);
    }

    #[test]
    fn test_applet_preset() {
        let form = ListHeaderForm::applet("Timer");
        assert_eq!(form.context, Context::Applet);
        assert_eq!(form.layout, Layout::Compact);
    }

    #[test]
    fn test_app_with_submit_preset() {
        let form = ListHeaderForm::app_with_submit("Timer", "Create");
        assert_eq!(form.context, Context::App);
        assert_eq!(form.layout, Layout::Spacious);
        assert_eq!(form.submit_text, Some("Create".to_string()));
    }

    #[test]
    fn test_handle_name_input() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("My Timer");
        assert_eq!(form.name_field.raw_value(), "My Timer");
    }

    #[test]
    fn test_handle_name_input_allows_any_text() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("Timer #1 - important!");
        assert_eq!(form.name_field.raw_value(), "Timer #1 - important!");
    }

    #[test]
    fn test_handle_duration_input_valid() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_duration_input("15");
        assert_eq!(form.duration_field.raw_value(), "15");
    }

    #[test]
    fn test_handle_duration_input_invalid() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_duration_input("15");
        form.handle_duration_input("abc");
        assert_eq!(form.duration_field.raw_value(), "15"); // unchanged
    }

    #[test]
    fn test_handle_duration_input_zero_rejected() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_duration_input("0");
        assert_eq!(form.duration_field.raw_value(), ""); // rejected
    }

    #[test]
    fn test_set_time_unit() {
        let mut form = ListHeaderForm::new("Test");
        assert_eq!(form.time_unit_field.value(), Some(TimeUnit::Seconds));
        form.set_time_unit(TimeUnit::Minutes);
        assert_eq!(form.time_unit_field.value(), Some(TimeUnit::Minutes));
    }

    #[test]
    fn test_handle_submit_valid() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("My Timer");
        form.handle_duration_input("5");
        form.set_time_unit(TimeUnit::Minutes);

        let result = form.handle_submit();
        assert!(result.is_some());
        let submission = result.unwrap();
        assert_eq!(submission.name, "My Timer");
        assert_eq!(submission.duration_seconds, 300); // 5 * 60
    }

    #[test]
    fn test_handle_submit_seconds() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("Quick Timer");
        form.handle_duration_input("30");
        // time_unit defaults to Seconds

        let result = form.handle_submit();
        assert!(result.is_some());
        let submission = result.unwrap();
        assert_eq!(submission.name, "Quick Timer");
        assert_eq!(submission.duration_seconds, 30);
    }

    #[test]
    fn test_handle_submit_hours() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("Long Timer");
        form.handle_duration_input("2");
        form.set_time_unit(TimeUnit::Hours);

        let result = form.handle_submit();
        assert!(result.is_some());
        let submission = result.unwrap();
        assert_eq!(submission.duration_seconds, 7200); // 2 * 3600
    }

    #[test]
    fn test_handle_submit_empty_name() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_duration_input("10");
        let result = form.handle_submit();
        assert!(result.is_none());
    }

    #[test]
    fn test_handle_submit_whitespace_name() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("   ");
        form.handle_duration_input("10");
        let result = form.handle_submit();
        assert!(result.is_none());
    }

    #[test]
    fn test_handle_submit_empty_duration() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("My Timer");
        let result = form.handle_submit();
        assert!(result.is_none());
    }

    #[test]
    fn test_handle_submit_trims_name() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("  My Timer  ");
        form.handle_duration_input("10");

        let result = form.handle_submit();
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "My Timer");
    }

    #[test]
    fn test_clear() {
        let mut form = ListHeaderForm::new("Test");
        form.handle_name_input("My Timer");
        form.handle_duration_input("15");
        form.set_time_unit(TimeUnit::Hours);

        form.clear();

        assert_eq!(form.name_field.raw_value(), "");
        assert_eq!(form.duration_field.raw_value(), "");
        assert_eq!(form.time_unit_field.value(), Some(TimeUnit::Seconds));
    }

    #[test]
    fn test_default_time_unit_is_seconds() {
        let form = ListHeaderForm::new("Test");
        assert_eq!(form.time_unit_field.value(), Some(TimeUnit::Seconds));
    }

    #[test]
    fn test_builder_chaining_order_independent() {
        let form1 = ListHeaderForm::new("Test")
            .context(Context::Applet)
            .name_placeholder("Name...")
            .duration_placeholder("Duration...")
            .layout(Layout::Compact);

        let form2 = ListHeaderForm::new("Test")
            .layout(Layout::Compact)
            .duration_placeholder("Duration...")
            .name_placeholder("Name...")
            .context(Context::Applet);

        assert_eq!(form1.context, form2.context);
        assert_eq!(form1.layout, form2.layout);
    }
}
