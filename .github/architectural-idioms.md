# Architectural Idioms for Chronomancer

This document outlines key architectural patterns and idioms used throughout the Chronomancer codebase. These patterns maintain consistency, reduce boilerplate, and make the message flow predictable. This is made up on the fly as we identify recurring patterns. Cosmic doesn't have strong opinions on architecture, so we define our own idioms here. Some of these patterns are well defined in web frameworks (my background), and others are more specific to Rust and Cosmic's MVU needs.

---

## Module-Based Message Pattern (libcosmic Style)

### The Problem

In a layered MVU (Model-View-Update) architecture with Components → Pages → App, we need a clean way to organize messages and handle message flow without creating a single massive message enum or tightly coupling modules.

### The Solution: Self-Contained Module Messages

Following the libcosmic pattern (see [libcosmic book - Modules](https://pop-os.github.io/libcosmic-book/modules.html)), each page and component module defines its own `Message` enum. The app-level message enum uses enum composition to wrap page-specific messages, and `From` implementations handle conversions.

### Pattern Structure

#### Page Module Structure

Each page module is self-contained with its own types:

```rust
// src/pages/power_controls.rs
pub mod power_controls {
    #[derive(Debug, Clone)]
    pub enum Message {
        // Component interactions
        RadioOptionSelected(usize),
        FormTextChanged(String),
        FormTimeUnitChanged(TimeUnit),
        FormSubmitPressed,
        
        // Actions to bubble up to app
        ToggleStayAwake,
        SetSuspendTime(i32),
        SetShutdownTime(i32),
        SetLogoutTime(i32),
    }

    pub struct Page {
        power_buttons: RadioComponents<ToggleIconRadio>,
        power_form: PowerForm,
    }

    impl Page {
        pub fn view(&self) -> Element<'_, Message> {
            // Components accept message constructors
            self.power_form.view(
                Message::FormTextChanged,
                Message::FormTimeUnitChanged,
                Message::FormSubmitPressed,
            )
        }

        pub fn update(&mut self, message: Message) -> Task<Action<Message>> {
            match message {
                Message::FormTextChanged(text) => {
                    self.power_form.handle_text_input(text);
                    Task::none()
                }
                Message::ToggleStayAwake => {
                    // These bubble up to app level, so just return none
                    // App will intercept and handle them
                    Task::none()
                }
                // ... handle other messages
            }
        }
    }
}
```

#### Message-Agnostic Components

Components don't define their own message types. Instead, they accept message constructors as function parameters. This is the best improvement for reusability and I finally get the point and utility of the where clause pattern. It's generics with extra type safety because of lifetimes:

```rust
pub struct PowerForm {
    pub input_value: String,
    pub time_unit: TimeUnit,
    // ... other fields
}

impl PowerForm {
    /// View method accepts message constructors
    pub fn view<Message>(
        &self,
        on_text_input: impl Fn(String) -> Message + 'static,
        on_time_unit: impl Fn(TimeUnit) -> Message + 'static,
        on_submit: Message,
    ) -> Element<'_, Message>
    where
        Message: Clone + 'static,
    {
        column![
            TextInput::new(&self.placeholder_text, &self.input_value)
                .on_input(on_text_input)
                .on_submit(move |_| on_submit.clone()),
            ComboBox::new(
                &self.time_unit_options,
                &fl!("unit-label"),
                Some(&self.time_unit),
                on_time_unit,
            ),
            button::text(fl!("set-button-label"))
                .on_press(on_submit)
        ]
        .into()
    }

    /// Public methods for state manipulation (no messages)
    pub fn handle_text_input(&mut self, new_text: String) {
        if let Ok(value) = new_text.parse::<u32>() {
            self.input_value = value.to_string();
        }
    }
}
```

#### App-Level Message Composition

The app uses enum composition to wrap page messages:

```rust
// src/app_messages.rs
#[derive(Debug, Clone)]
pub enum AppMessage {
    TogglePopup,
    UpdateConfig(Config),
    Tick,
    
    // Page messages wrapped
    PowerControlsMessage(power_controls::Message),
    
    // Service-level messages
    DatabaseMessage(DatabaseMessage),
    TimerMessage(TimerMessage),
    PowerMessage(PowerMessage),
}

// Conversion for convenient .map() usage
impl From<power_controls::Message> for AppMessage {
    fn from(msg: power_controls::Message) -> Self {
        AppMessage::PowerControlsMessage(msg)
    }
}
```

#### App-Level Message Handling

The app intercepts page messages that need app-level handling:

```rust
// src/app.rs
impl Application for AppModel {
    fn update(&mut self, message: AppMessage) -> Task<Action<AppMessage>> {
        match message {
            AppMessage::PowerControlsMessage(msg) => {
                self.handle_power_controls_message(msg)
            }
            // ... other messages
        }
    }
}

impl AppModel {
    fn handle_power_controls_message(
        &mut self,
        msg: power_controls::Message,
    ) -> Task<Action<AppMessage>> {
        // Intercept messages that need app-level handling
        match msg {
            power_controls::Message::ToggleStayAwake => {
                self.handle_power_message(PowerMessage::ToggleStayAwake)
            }
            power_controls::Message::SetSuspendTime(time) => {
                self.handle_power_message(PowerMessage::SetSuspendTime(time))
            }
            // ... other intercepted messages
            
            // Pass through to page for local state updates
            _ => self.power_controls.update(msg).map(|action| match action {
                Action::App(page_msg) => Action::App(AppMessage::PowerControlsMessage(page_msg)),
                Action::None => Action::None,
                Action::Cosmic(cosmic_action) => Action::Cosmic(cosmic_action),
                Action::DbusActivation(dbus_action) => Action::DbusActivation(dbus_action),
            }),
        }
    }
}
```

#### View Mapping

Pages map their view to app-level messages:

```rust
impl Application for AppModel {
    fn view(&self) -> Element<AppMessage> {
        self.power_controls
            .view()
            .map(AppMessage::PowerControlsMessage)
    }
}
```

### Benefits

1. **Module Encapsulation**: Each page/component is self-contained with its own message types
2. **No Global Message Bloat**: App-level messages only contain top-level routing, not every possible UI interaction. This was becoming an obvious problem even in a tiny app like this
3. **Clear Ownership**: Easy to see which module handles which messages
4. **Reusable Components**: Components are message-agnostic and work with any message type
5. **Type Safety**: Compiler ensures message types match at boundaries and lifetimes are respected
6. **Follows libcosmic Conventions**: Better aligns with recommended patterns from the framework

### Component Design Guidelines

**Message-Agnostic Components** (preferred for reusability):
- Accept message constructors as `view()` parameters
- Use trait bounds: `where Message: Clone + 'static`
- Provide public methods for state manipulation (e.g., `handle_text_input()`)
- Don't define their own message types

**Example**:
```rust
pub fn view<Message>(
    &self,
    on_select: impl Fn(usize) -> Message + 'static,
) -> Element<'_, Message>
where
    Message: Clone + 'static,
{
    button::text("Click me").on_press(on_select(self.index))
}
```

### When to Use This Pattern

- ✅ Building pages with multiple components and interactions
- ✅ Need clear separation between page-level and app-level concerns
- ✅ Want reusable components across different pages/apps
- ✅ Following libcosmic's recommended architecture

### When NOT to Use This Pattern

- ❌ Simple single-page app with minimal state → flat message structure is fine
- ❌ Component is page-specific and will never be reused → can use page messages directly
- ❌ Prototyping/experimenting → use whatever is fastest to write

### Key Differences from Previous Pattern

**Old Pattern** (Component trait with `Option<PageMessage>`):
- Components implemented a `Component` trait
- Components returned `Option<PageMessage>`
- Pages handled component messages through recursive `update()` calls
- Required a global `ComponentMessage` and `PageMessage` enum

**New Pattern** (libcosmic module style):
- No `Component` trait needed
- Components are message-agnostic (accept message constructors)
- Each page has its own `Message` enum
- App uses enum composition to wrap page messages
- More aligned with libcosmic conventions

### Real-World Examples

See the following files for reference:
- `src/pages/power_controls.rs` - Page with self-contained `Message` enum
- `src/components/power_form.rs` - Message-agnostic component accepting message constructors
- `src/components/radio_components.rs` - Generic component with message constructor parameters
- `src/app_messages.rs` - App-level message composition
- `src/app.rs` - Message interception and routing

---

## Future Idioms

As the project grows, document new architectural patterns here. I'm sure a lot will change as I write more applications and get feedback from contributors.

**Potential areas to document**:
- Async task patterns (database queries, systemd calls, etc.)
- Service layer patterns (systemd integration, D-Bus communication)
- State management strategies (when to use app state vs page state)
- Testing strategies for MVU architecture
