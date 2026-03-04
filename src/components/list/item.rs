//! A composable list item component with optional description, icon, and action buttons.
//!
//! `ListItem` uses the builder pattern for flexible configuration and is
//! message-agnostic — action buttons are provided as pre-built elements at
//! render time
//!

use cosmic::{
    Element,
    iced::Length::Fill,
    iced_widget::column,
    iced_widget::row,
    theme,
    widget::{icon, text},
};

use crate::{
    components::{Context, Layout},
    utils::ui::{ComponentSize, Spacing},
};

pub struct ListItem {
    title: String,
    context: Context,
    layout: Layout,
    description: Option<String>,
    icon: Option<String>, // Path to icon resource
}

impl ListItem {
    /// Creates a new `ListItem` with App context and Comfortable layout.
    ///
    /// # Arguments
    ///
    /// * `title` - The primary text displayed in the list item.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let item = ListItem::new("My Timer");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            context: Context::App,
            layout: Layout::Comfortable,
            description: None,
            icon: None,
        }
    }

    /// Sets the behavioral context (App or Applet).
    ///
    /// - **App**: More spacious, can show longer descriptions
    /// - **Applet**: Compact, truncates description if needed
    ///
    /// # Example
    ///
    /// ```ignore
    /// let item = ListItem::new("Timer")
    ///     .context(Context::Applet);
    /// ```
    #[must_use]
    pub fn context(mut self, context: Context) -> Self {
        self.context = context;
        self
    }

    /// Sets the visual layout density.
    ///
    /// Affects spacing, padding, and text sizes at render time.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let item = ListItem::new("Timer")
    ///     .layout(Layout::Compact);
    /// ```
    #[must_use]
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the description text shown below the title.
    ///
    /// Use this for secondary information like remaining time, status, or
    /// recurrence pattern.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let item = ListItem::new("Sleep Timer")
    ///     .description("45 minutes remaining");
    /// ```
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the leading icon by system icon name.
    ///
    /// The icon is displayed to the left of the title/description. Use XDG
    /// icon names for best compatibility (e.g., `"alarm-symbolic"`,
    /// `"media-playback-pause-symbolic"`) as few are bundled with Chronomancer itself.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let item = ListItem::new("Sleep Timer")
    ///     .icon("alarm-symbolic");
    /// ```
    #[must_use]
    pub fn icon(mut self, icon_path: impl Into<String>) -> Self {
        self.icon = Some(icon_path.into());
        self
    }

    /// Reads the current layout and context to determine appropriate spacing
    /// and sizing for rendering.
    ///
    /// Returns layout-specific spacing, padding, text size, and description
    /// text size from the cosmic theme.
    fn layout_values(&self) -> Spacing {
        let cosmic_spacing = theme::active().cosmic().spacing;

        match self.layout {
            Layout::Compact => Spacing {
                gap: cosmic_spacing.space_xxs,
                padding: [
                    cosmic_spacing.space_xxs,
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_xxs,
                    cosmic_spacing.space_xs,
                ],
                text_size: ComponentSize::FONT_SIZE_SMALL,
            },
            Layout::Comfortable => Spacing {
                gap: cosmic_spacing.space_xs,
                padding: [
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_xs,
                    cosmic_spacing.space_s,
                ],
                text_size: ComponentSize::FONT_SIZE_DEFAULT,
            },
            Layout::Spacious => Spacing {
                gap: cosmic_spacing.space_s,
                padding: [
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_m,
                    cosmic_spacing.space_s,
                    cosmic_spacing.space_m,
                ],
                text_size: ComponentSize::FONT_SIZE_DEFAULT,
            },
        }
    }

    /// Renders the list item as an `Element`.
    ///
    /// Action buttons are passed as pre-built elements. This keeps `ListItem`
    /// message-agnostic — the caller decides what buttons exist and what
    /// messages they produce.
    ///
    /// # Arguments
    ///
    /// * `actions` - Pre-built action button elements to display on the
    ///   trailing edge of the item. Pass an empty `Vec` for no actions.
    ///
    /// # Layout
    ///
    /// ```text
    /// ┌─────────────────────────────────────────────────┐
    /// │ [icon]  Title                    [btn] [btn]    │
    /// │         Description                             │
    /// └─────────────────────────────────────────────────┘
    /// ```
    ///
    /// # Example
    ///
    /// ```ignore
    /// let edit = button::icon(icon::from_name("edit-symbolic"))
    ///     .extra_small()
    ///     .on_press(Message::EditTimer(id))
    ///     .into();
    ///
    /// let element = ListItem::new("My Timer")
    ///     .description("25 min left")
    ///     .view(vec![edit]);
    /// ```
    #[must_use]
    pub fn view<'a, Message: 'static>(
        &'a self,
        actions: Vec<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        // Get layout-specific values for spacing, padding, and text size
        let layout_values = self.layout_values();

        // Build the text column with title and optional description
        let mut text_column = column![text(&self.title).size(layout_values.text_size)];

        if let Some(desc) = &self.description {
            text_column = text_column.push(
                text(desc).size(layout_values.text_size - 2), // Slightly smaller for description
            );
        }

        let text_content: Element<'a, Message> = text_column
            .spacing(layout_values.gap / 2)
            .width(Fill)
            .into();

        // Build the content row: [icon] [text] [spacer] [actions]
        let mut content_row = row![];

        // Leading icon
        if let Some(icon_name) = &self.icon {
            content_row = content_row.push(icon::from_name(icon_name.as_str()));
        }

        // Add text content
        content_row = content_row.push(text_content);

        // Add action buttons
        for action in actions {
            content_row = content_row.push(action);
        }

        content_row
            .spacing(layout_values.gap)
            .padding(layout_values.padding)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let item = ListItem::new("Test");
        assert_eq!(item.title, "Test");
        assert_eq!(item.context, Context::App);
        assert_eq!(item.layout, Layout::Comfortable);
        assert_eq!(item.description, None);
        assert_eq!(item.icon, None);
    }

    #[test]
    fn test_title_from_string() {
        let item = ListItem::new(String::from("Owned Title"));
        assert_eq!(item.title, "Owned Title");
    }

    #[test]
    fn test_context_app() {
        let item = ListItem::new("Test").context(Context::App);
        assert_eq!(item.context, Context::App);
    }

    #[test]
    fn test_context_applet() {
        let item = ListItem::new("Test").context(Context::Applet);
        assert_eq!(item.context, Context::Applet);
    }

    #[test]
    fn test_layout_compact() {
        let item = ListItem::new("Test").layout(Layout::Compact);
        assert_eq!(item.layout, Layout::Compact);
    }

    #[test]
    fn test_layout_spacious() {
        let item = ListItem::new("Test").layout(Layout::Spacious);
        assert_eq!(item.layout, Layout::Spacious);
    }

    #[test]
    fn test_description() {
        let item = ListItem::new("Timer").description("45 minutes remaining");
        assert_eq!(item.description, Some("45 minutes remaining".to_string()));
    }

    #[test]
    fn test_description_overwrites() {
        let item = ListItem::new("Timer")
            .description("first")
            .description("second");
        assert_eq!(item.description, Some("second".to_string()));
    }

    #[test]
    fn test_icon() {
        let item = ListItem::new("Timer").icon("alarm-symbolic");
        assert_eq!(item.icon, Some("alarm-symbolic".to_string()));
    }

    #[test]
    fn test_icon_overwrites() {
        let item = ListItem::new("Timer")
            .icon("alarm-symbolic")
            .icon("timer-symbolic");
        assert_eq!(item.icon, Some("timer-symbolic".to_string()));
    }

    #[test]
    fn test_full_builder() {
        let item = ListItem::new("Sleep Timer")
            .context(Context::Applet)
            .layout(Layout::Compact)
            .description("30 min left")
            .icon("alarm-symbolic");

        assert_eq!(item.title, "Sleep Timer");
        assert_eq!(item.context, Context::Applet);
        assert_eq!(item.layout, Layout::Compact);
        assert_eq!(item.description, Some("30 min left".to_string()));
        assert_eq!(item.icon, Some("alarm-symbolic".to_string()));
    }

    #[test]
    fn test_builder_chaining_order_independent() {
        let item1 = ListItem::new("Test")
            .context(Context::Applet)
            .layout(Layout::Compact)
            .description("desc")
            .icon("icon-symbolic");

        let item2 = ListItem::new("Test")
            .icon("icon-symbolic")
            .description("desc")
            .layout(Layout::Compact)
            .context(Context::Applet);

        assert_eq!(item1.title, item2.title);
        assert_eq!(item1.context, item2.context);
        assert_eq!(item1.layout, item2.layout);
        assert_eq!(item1.description, item2.description);
        assert_eq!(item1.icon, item2.icon);
    }
}
