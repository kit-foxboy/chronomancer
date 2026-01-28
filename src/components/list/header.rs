//! A header component for lists with an optional add button.
//!
//! This component displays a title and, if specified, an add button that
//! can trigger an action when pressed. It is designed to be used as a header
//! for list sections in the UI.
//!
//! In applet context, the add button is typically represented by a "+" icon only.
//! In full GUI apps, this button can include text alongside the icon.
//!
//! # Builder Pattern
//!
//! ListHeader uses the builder pattern for flexible configuration:
//!
//! ```ignore
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
//!     .with_add_button();
//! ```
//!
//! See `.journal/component-builder-pattern.md` for the full pattern guide.

use cosmic::{
    Element,
    iced::{Alignment::Center, Length::Fill},
    iced_widget::row,
    theme,
    theme::Button::Icon,
    widget::{Space, button, icon, text},
};

use crate::components::{Context, Layout};

/// Messages emitted by the ListHeader component.
#[derive(Debug, Clone)]
pub enum Message {
    /// The add button was pressed.
    AddButtonPressed,
}

/// A header component for list sections with title and optional action button.
///
/// Supports both App and Applet contexts with different visual behaviors:
/// - **App**: Can show icon + text buttons, adapts to layout
/// - **Applet**: Always icon-only buttons, compact spacing
///
/// # Example
///
/// ```ignore
/// let header = ListHeader::new("Timers")
///     .context(Context::Applet)
///     .with_add_button()
///     .view();
/// ```
pub struct ListHeader {
    // Required fields
    title: String,

    // Configuration
    context: Context,
    layout: Layout,

    // Optional fields
    show_button: bool,
    button_text: Option<String>,
}

impl ListHeader {
    /// Creates a new ListHeader with App context and Comfortable layout.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display in the header.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let header = ListHeader::new("My Timers");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            context: Context::default(),
            layout: Layout::default(),
            show_button: false,
            button_text: None,
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
    /// let header = ListHeader::new("Timers")
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
    /// let header = ListHeader::new("Timers")
    ///     .layout(Layout::Spacious);
    /// ```
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Enables the add button on this header.
    ///
    /// The button style depends on context:
    /// - **App**: Icon + text if `button_text()` is set, otherwise icon-only
    /// - **Applet**: Always icon-only (button_text ignored)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let header = ListHeader::new("Timers")
    ///     .with_add_button();
    /// ```
    pub fn with_add_button(mut self) -> Self {
        self.show_button = true;
        self
    }

    /// Sets the button text (App context only).
    ///
    /// In App context, this creates an icon + text button.
    /// In Applet context, this value is stored but not displayed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let header = ListHeader::new("Timers")
    ///     .with_add_button()
    ///     .button_text("Add Timer");
    /// ```
    pub fn button_text(mut self, text: impl Into<String>) -> Self {
        self.button_text = Some(text.into());
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
    /// - Applet + button → icon-only button
    /// - App + button + text → icon + text button
    /// - App + button (no text) → icon-only button
    /// - No button → title only
    pub fn view(&self) -> Element<'_, Message> {
        match (self.context, self.show_button, self.button_text.as_ref()) {
            // Applet: always icon-only button
            (Context::Applet, true, _) => self.view_with_icon_button(),
            // App: icon + text button when text provided
            (Context::App, true, Some(btn_text)) => self.view_with_text_button(btn_text.clone()),
            // App: icon-only button when no text
            (Context::App, true, None) => self.view_with_icon_button(),
            _ => {
                // No button
                self.view_title_only()
            }
        }
    }

    /// Renders header with icon-only add button.
    fn view_with_icon_button(&self) -> Element<'_, Message> {
        let (spacing, padding, text_size) = self.layout_values();
        let title = text(&self.title).size(text_size);

        let add_button = button::icon(icon::from_name("list-add-symbolic"))
            .class(Icon)
            .extra_small()
            .on_press(Message::AddButtonPressed);

        row![title, Space::with_width(Fill), add_button]
            .align_y(Center)
            .spacing(spacing)
            .padding(padding)
            .into()
    }

    /// Renders header with icon + text add button (App context).
    fn view_with_text_button(&self, btn_text: String) -> Element<'_, Message> {
        let (spacing, padding, text_size) = self.layout_values();
        let title = text(&self.title).size(text_size);

        let add_button = button::text(btn_text)
            .leading_icon(icon::from_name("list-add-symbolic"))
            .class(Icon)
            .on_press(Message::AddButtonPressed);

        row![title, Space::with_width(Fill), add_button]
            .align_y(Center)
            .spacing(spacing)
            .padding(padding)
            .into()
    }

    /// Renders header with title only (no button).
    fn view_title_only(&self) -> Element<'_, Message> {
        let (spacing, padding, text_size) = self.layout_values();
        let title = text(&self.title).size(text_size);

        row![title]
            .align_y(Center)
            .spacing(spacing)
            .padding(padding)
            .into()
    }
}

// Preset constructors for common configurations

impl ListHeader {
    /// Creates an applet header with icon-only add button.
    ///
    /// Equivalent to:
    /// ```ignore
    /// ListHeader::new(title)
    ///     .context(Context::Applet)
    ///     .layout(Layout::Compact)
    ///     .with_add_button()
    /// ```
    pub fn applet_with_add(title: impl Into<String>) -> Self {
        Self::new(title)
            .context(Context::Applet)
            .layout(Layout::Compact)
            .with_add_button()
    }

    /// Creates an app header with text add button.
    ///
    /// Equivalent to:
    /// ```ignore
    /// ListHeader::new(title)
    ///     .context(Context::App)
    ///     .layout(Layout::Spacious)
    ///     .with_add_button()
    ///     .button_text(button_text)
    /// ```
    pub fn app_with_add(title: impl Into<String>, button_text: impl Into<String>) -> Self {
        Self::new(title)
            .context(Context::App)
            .layout(Layout::Spacious)
            .with_add_button()
            .button_text(button_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let header = ListHeader::new("Test");
        assert_eq!(header.context, Context::App);
        assert_eq!(header.layout, Layout::Comfortable);
    }

    #[test]
    fn test_app_context() {
        let header = ListHeader::app_with_add("Title", "click me");
        assert_eq!(header.context, Context::App);
    }

    #[test]
    fn test_applet_context() {
        let header = ListHeader::new("Test").context(Context::Applet);
        assert_eq!(header.context, Context::Applet);
    }

    #[test]
    fn test_compact_layout() {
        let header = ListHeader::new("Test").layout(Layout::Compact);
        assert_eq!(header.layout, Layout::Compact);
    }

    #[test]
    fn test_spacious_layout() {
        let header = ListHeader::new("Test").layout(Layout::Spacious);
        assert_eq!(header.layout, Layout::Spacious);
    }

    #[test]
    fn test_with_add_button() {
        let header = ListHeader::new("Test").with_add_button();
        assert!(header.show_button);
    }

    #[test]
    fn test_button_text() {
        let header = ListHeader::new("Test")
            .with_add_button()
            .button_text("Add Item");
        assert_eq!(header.button_text, Some("Add Item".to_string()));
    }

    #[test]
    fn test_applet_with_add_preset() {
        let header = ListHeader::applet_with_add("Timers");
        assert_eq!(header.context, Context::Applet);
        assert_eq!(header.layout, Layout::Compact);
        assert!(header.show_button);
    }

    #[test]
    fn test_app_with_add_preset() {
        let header = ListHeader::app_with_add("Timers", "Add Timer");
        assert_eq!(header.context, Context::App);
        assert_eq!(header.layout, Layout::Spacious);
        assert!(header.show_button);
        assert_eq!(header.button_text, Some("Add Timer".to_string()));
    }

    #[test]
    fn test_builder_chaining_order_independent() {
        // These should produce equivalent results regardless of order
        let header1 = ListHeader::new("Test")
            .context(Context::Applet)
            .with_add_button()
            .layout(Layout::Compact);

        let header2 = ListHeader::new("Test")
            .layout(Layout::Compact)
            .with_add_button()
            .context(Context::Applet);

        assert_eq!(header1.context, header2.context);
        assert_eq!(header1.layout, header2.layout);
        assert_eq!(header1.show_button, header2.show_button);
    }
}
