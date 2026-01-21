# Component Builder Pattern Guide

## Overview

Despite my own distate for factory and builder patterns in many frameworks, it's hard to argue with it for constructing UIs in code. After tinkering with a few approaches, I've decided that Chronomancer components use the **builder pattern** with **Rust enums** to handle configuration for different contexts and constraints. As with the rest of the app, wherever there's a lack of clear opinions in architecture, I look to web and mobile development conventions. This guide provides templates and best practices for implementing this pattern consistently.

### Design Philosophy

Components must work in two (for now) fundamentally different contexts:

1. **App** (default) - Full GUI applications with screen navigation, dialogs, more space
2. **Applet** - Panel applets with inline interactions, popup forms, no navigation

Additionally, both contexts can have space constraints (compact vs. spacious layouts).

## Core Principles

1. **Start simple** - Components begin with sensible App defaults
2. **Separate concerns** - Behavioral context (App/Applet) vs. space constraints (Compact/Spacious)
3. **Use enums** - Pass enum values, not booleans (avoids scaling issues)
4. **Build incrementally** - Each builder method returns `self` for chaining just as we do with view methods
5. **Type-safe** - Use enums and the type system to prevent invalid configurations

## Context and Layout Types

### Context Enum

```rust
/// The context in which a component is being used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    /// Full GUI application - screen navigation, dialogs, icon+text buttons
    App,
    /// Panel applet - inline interactions, popups, icon-only buttons
    Applet,
}

impl Default for Context {
    fn default() -> Self {
        Self::App  // Default to App context
    }
}
```

**Key Differences:**
- **App**: Separate screens, modal dialogs, navigation patterns, icon+text buttons
- **Applet**: Inline forms, popup confirmations, no screen navigation, icon-only buttons

### Layout Enum

```rust
/// Visual layout density for available space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    /// Tight spacing and padding (limited space)
    Compact,
    /// Moderate spacing and padding (default)
    Comfortable,
    /// Generous spacing and padding (ample space)
    Spacious,
}

impl Default for Layout {
    fn default() -> Self {
        Self::Compact  // Default to compact layout
    }
}
```

**When to use:**
- **Compact**: Applets (always), small app windows, mobile-like constraints
- **Comfortable**: Default spacing for most app windows
- **Spacious**: Large app windows, desktop layouts with room to breathe

## Basic Template

```rust
pub use crate::components::{Context, Layout};

pub struct ComponentName {
    // Required fields
    title: String,
    
    // Configuration
    context: Context,
    layout: Layout,
    
    // Optional fields
    show_button: bool,
    button_text: Option<String>,
    
    // Computed from layout
    spacing: u16,
    padding: [u16; 4],
}

impl ComponentName {
    /// Creates a new component with App context and Compact layout.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            context: Context::default(),
            layout: Layout::default(),
            show_button: false,
            button_text: None,
            spacing: 8,
            padding: [8, 12, 8, 12],
        }
    }

    /// Sets the behavioral context.
    pub fn context(mut self, context: Context) -> Self {
        self.context = context;
        self
    }

    /// Sets compact layout (tight spacing, default).
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self.apply_layout_spacing();
        self
    }

    /// Adds an action button to the component.
    pub fn with_add_button(mut self) -> Self {
        self.show_button = true;
        self
    }

    /// Sets custom button text (implies icon+text for App context).
    pub fn button_text(mut self, text: impl Into<String>) -> Self {
        self.button_text = Some(text.into());
        self
    }

    /// Sets custom spacing (overrides layout defaults).
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    fn apply_layout_spacing(&mut self) {
        match self.layout {
            Layout::Compact => {
                self.spacing = 8;
                self.padding = [8, 12, 8, 12];
            }
            Layout::Spacious => {
                self.spacing = 16;
                self.padding = [16, 24, 16, 24];
            }
        }
    }

    /// Renders the component as an Element.
    pub fn view(&self) -> Element<'_, Message> {
        match (self.context, self.show_button, self.button_text.as_ref()) {
            (Context::Applet, true, _) => {
                // Applet: always icon-only button
                self.view_with_icon_button()
            }
            (Context::App, true, Some(text)) => {
                // App: icon + text button when text provided
                self.view_with_text_button(text)
            }
            (Context::App, true, None) => {
                // App: icon-only button when no text
                self.view_with_icon_button()
            }
            _ => {
                // No button
                self.view_title_only()
            }
        }
    }

    fn view_with_icon_button(&self) -> Element<'_, Message> {
        // Implementation for icon-only button
        todo!()
    }

    fn view_with_text_button(&self, text: &str) -> Element<'_, Message> {
        // Implementation for icon + text button
        todo!()
    }

    fn view_title_only(&self) -> Element<'_, Message> {
        // Implementation without button
        todo!()
    }
}
```

## Usage Examples

### App Context (Default)

```rust
// Minimal - all defaults (App + Compact)
let header = ListHeader::new("Timers");

// App with spacious layout
let header = ListHeader::new("Active Timers")
    .layout(Layout::Spacious)
    .with_add_button()
    .button_text("Add Timer");

// App with compact layout, icon-only button
let header = ListHeader::new("Recent")
    .layout(Layout::Compact)
    .with_add_button();
```

### Applet Context

```rust
// Applet with icon-only button
let header = ListHeader::new("Timers")
    .context(Context::Applet)
    .with_add_button();

// Explicit applet configuration
let header = ListHeader::new("Timers")
    .context(Context::Applet)
    .layout(Layout::Compact)  // Always compact for applets
    .with_add_button();
```

### Custom Configuration

```rust
// Custom spacing override
let header = ListHeader::new("Quick Actions")
    .context(Context::Applet)
    .spacing(4)  // Extra tight
    .with_add_button();

// App with custom configuration
let header = ListHeader::new("Dashboard")
    .context(Context::App)
    .layout(Layout::Spacious)
    .spacing(20)  // Override spacious default
    .with_add_button()
    .button_text("Add Widget");
```

## Common Builder Methods

### Context & Layout

- `context(Context)` - Set behavioral context (App or Applet)
- `layout(Layout)` - Set visual layout density (Compact or Spacious)
- `spacing(u16)` - Custom spacing override
- `padding([u16; 4])` - Custom padding override

### Button Configuration

- `with_add_button()` - Show add/action button
- `button_text(text)` - Set button text (icon+text for App, ignored for Applet)
- `button_icon(name)` - Custom icon name
- `on_button_press(Message)` - Custom button message

### Visual Style

- `text_size(u16)` - Override default text size
- `heading_level(HeadingLevel)` - Semantic heading level
- `bold()` - Use bold font weight

## Context-Specific Behavior

### App Context

When `context == Context::App`:
- Buttons can have icon + text (via `button_text()`)
- Forms can be separate screens
- Confirmations use dialogs
- Can adapt layout to window size (compact/spacious)
- Screen navigation available

### Applet Context

When `context == Context::Applet`:
- Buttons are always icon-only (button_text ignored)
- Forms appear inline (no navigation)
- Confirmations use popups
- Always space-constrained (usually compact)
- No screen navigation

## Implementation Checklist

When adding builder pattern to a component:

- [ ] Define `context: Context` and `layout: Layout` fields with defaults
- [ ] Implement `context(Context)` method
- [ ] Implement `layout(Layout)` method
- [ ] Ensure all builder methods return `Self` with `mut self`
- [ ] Implement `view()` to handle different context/layout combinations
- [ ] Document behavioral differences between contexts
- [ ] Add usage examples for both App and Applet contexts
- [ ] Update component tests to cover different configurations

## Testing

Test different context and layout combinations:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_app_compact() {
        let header = ListHeader::new("Test");
        assert_eq!(header.context, Context::App);
        assert_eq!(header.layout, Layout::Compact);
    }

    #[test]
    fn test_applet_context() {
        let header = ListHeader::new("Test")
            .context(Context::Applet);
        assert_eq!(header.context, Context::Applet);
    }

    #[test]
    fn test_spacious_layout() {
        let header = ListHeader::new("Test")
            .layout(Layout::Spacious);
        assert_eq!(header.layout, Layout::Spacious);
        assert_eq!(header.spacing, 16);
    }

    #[test]
    fn test_app_with_text_button() {
        let header = ListHeader::new("Test")
            .context(Context::App)
            .with_add_button()
            .button_text("Add");
        assert_eq!(header.context, Context::App);
        assert!(header.show_button);
        assert_eq!(header.button_text, Some("Add".to_string()));
    }

    #[test]
    fn test_applet_icon_only() {
        let header = ListHeader::new("Test")
            .context(Context::Applet)
            .with_add_button()
            .button_text("Add");  // Should be ignored
        assert_eq!(header.context, Context::Applet);
        assert!(header.show_button);
        // Button text stored but not used in Applet context
    }
}
```

## Preset Configurations

For common configurations, provide preset constructors:

```rust
impl ListHeader {
    /// Creates an applet header with icon-only add button.
    pub fn applet_with_add(title: impl Into<String>) -> Self {
        Self::new(title)
            .context(Context::Applet)
            .with_add_button()
    }

    /// Creates an app header with text add button.
    pub fn app_with_add(
        title: impl Into<String>,
        button_text: impl Into<String>,
    ) -> Self {
        Self::new(title)
            .context(Context::App)
            .layout(Layout::Spacious)
            .with_add_button()
            .button_text(button_text)
    }
}
```

## See Also

- `.journal/architectural-idioms.md` - Component-to-page message flow
- `src/components/list/mod.rs` - Builder pattern examples in list components
- `src/components/types.rs` - Shared Context and Layout type definitions
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/type-safety.html
