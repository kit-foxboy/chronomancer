//! A form variant of the list header component.
//!
//! This component combines a list header with an embedded form for inline
//! item creation, commonly used in panel applets for space-efficient workflows.
//!
//! # Builder Pattern
//!
//! ListHeaderForm uses the builder pattern for flexible configuration:
//!
//! ```ignore
//! // Minimal - all defaults (App context, Comfortable layout)
//! ListHeaderForm::new("Add Timer");
//!
//! // Applet context with compact layout
//! ListHeaderForm::new("New Timer")
//!     .context(Context::Applet)
//!     .layout(Layout::Compact)
//!     .placeholder("Timer name...");
//!
//! // App context with submit button text
//! ListHeaderForm::new("Create Reminder")
//!     .context(Context::App)
//!     .layout(Layout::Spacious)
//!     .placeholder("Reminder name")
//!     .submit_text("Create");
//! ```
//!
//! See `.journal/component-builder-pattern.md` for the full pattern guide.

use cosmic::{
    Element,
    iced::{Alignment::Center, Length::Fill},
    iced_widget::row,
    theme,
    theme::Button::Icon,
    widget::{button, icon, text_input},
};

use crate::components::{Context, Layout};

/// Messages emitted by the ListHeaderForm component.
#[derive(Debug, Clone)]
pub enum Message {
    /// The text input value changed.
    InputChanged(String),
    /// The submit button was pressed or Enter was hit.
    Submit,
    /// The cancel button was pressed or form was dismissed.
    Cancel,
}

/// A list header with an embedded form for adding new items.
///
/// This component is designed for compact interfaces where showing a separate
/// form would be inefficient. It combines the title and action elements of
/// a standard list header with inline input fields.
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
///     .placeholder("Timer name...")
///     .value(&self.input_value)
///     .view();
/// ```
pub struct ListHeaderForm {
    // Required fields
    title: String,

    // Configuration
    context: Context,
    layout: Layout,

    // Form fields
    placeholder: Option<String>,
    value: String,
    submit_text: Option<String>,

    // Flags
    show_cancel: bool,
}

impl ListHeaderForm {
    /// Creates a new ListHeaderForm with App context and Comfortable layout.
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
            placeholder: None,
            value: String::new(),
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
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the placeholder text for the input field.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .placeholder("Enter timer name...");
    /// ```
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the current value of the input field.
    ///
    /// This should be bound to your component's state.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let form = ListHeaderForm::new("Add Timer")
    ///     .value(&self.timer_name);
    /// ```
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
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
    pub fn show_cancel(mut self, show: bool) -> Self {
        self.show_cancel = show;
        self
    }

    /// Returns layout-specific spacing, padding, and text size from cosmic theme.
    fn layout_values(&self) -> (u16, [u16; 4], u16) {
        let cosmic_spacing = theme::active().cosmic().spacing;

        match self.layout {
            Layout::Compact => {
                let spacing = cosmic_spacing.space_xs;
                let padding = [
                    cosmic_spacing.space_xxs,
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_xxs,
                    cosmic_spacing.space_xs,
                ];
                (spacing, padding, 13)
            }
            Layout::Comfortable => {
                let spacing = cosmic_spacing.space_s;
                let padding = [
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_s,
                ];
                (spacing, padding, 14)
            }
            Layout::Spacious => {
                let spacing = cosmic_spacing.space_m;
                let padding = [
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_m,
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_m,
                ];
                (spacing, padding, 15)
            }
        }
    }

    /// Renders the component as an Element.
    ///
    /// The rendering adapts based on context and configuration:
    /// - Applet → icon-only buttons
    /// - App + submit_text → icon + text submit button
    /// - App (no submit_text) → icon-only submit button
    pub fn view(&self) -> Element<'_, Message> {
        match (self.context, self.submit_text.as_ref()) {
            (Context::Applet, _) => {
                // Applet: always icon-only buttons
                self.view_with_icon_buttons()
            }
            (Context::App, Some(submit_text)) => {
                // App: icon + text submit button when text provided
                self.view_with_text_submit(submit_text.clone())
            }
            (Context::App, None) => {
                // App: icon-only buttons when no text
                self.view_with_icon_buttons()
            }
        }
    }

    /// Renders form with icon-only buttons.
    fn view_with_icon_buttons(&self) -> Element<'_, Message> {
        let (spacing, padding, text_size) = self.layout_values();
        let placeholder = self.placeholder.as_deref().unwrap_or("Enter value...");

        let input = text_input(placeholder, &self.value)
            .size(text_size)
            .on_input(Message::InputChanged)
            .on_submit(|_| Message::Submit)
            .width(Fill);

        let submit_button = button::icon(icon::from_name("emblem-ok-symbolic"))
            .class(Icon)
            .extra_small()
            .on_press(Message::Submit);

        let mut form_row = row![input, submit_button];

        if self.show_cancel {
            let cancel_button = button::icon(icon::from_name("window-close-symbolic"))
                .class(Icon)
                .extra_small()
                .on_press(Message::Cancel);
            form_row = form_row.push(cancel_button);
        }

        form_row
            .align_y(Center)
            .spacing(spacing)
            .padding(padding)
            .into()
    }

    /// Renders form with icon + text submit button (App context).
    fn view_with_text_submit(&self, submit_text: String) -> Element<'_, Message> {
        let (spacing, padding, text_size) = self.layout_values();
        let placeholder = self.placeholder.as_deref().unwrap_or("Enter value...");

        let input = text_input(placeholder, &self.value)
            .size(text_size)
            .on_input(Message::InputChanged)
            .on_submit(|_| Message::Submit)
            .width(Fill);

        let submit_button = button::text(submit_text)
            .leading_icon(icon::from_name("emblem-ok-symbolic"))
            .class(Icon)
            .on_press(Message::Submit);

        let mut form_row = row![input, submit_button];

        if self.show_cancel {
            let cancel_button = button::icon(icon::from_name("window-close-symbolic"))
                .class(Icon)
                .extra_small()
                .on_press(Message::Cancel);
            form_row = form_row.push(cancel_button);
        }

        form_row
            .align_y(Center)
            .spacing(spacing)
            .padding(padding)
            .into()
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
    fn test_placeholder() {
        let form = ListHeaderForm::new("Test").placeholder("Enter name...");
        assert_eq!(form.placeholder, Some("Enter name...".to_string()));
    }

    #[test]
    fn test_value() {
        let form = ListHeaderForm::new("Test").value("My Timer");
        assert_eq!(form.value, "My Timer".to_string());
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
    fn test_builder_chaining_order_independent() {
        // These should produce equivalent results regardless of order
        let form1 = ListHeaderForm::new("Test")
            .context(Context::Applet)
            .placeholder("Name...")
            .layout(Layout::Compact);

        let form2 = ListHeaderForm::new("Test")
            .layout(Layout::Compact)
            .placeholder("Name...")
            .context(Context::Applet);

        assert_eq!(form1.context, form2.context);
        assert_eq!(form1.layout, form2.layout);
        assert_eq!(form1.placeholder, form2.placeholder);
    }
}
