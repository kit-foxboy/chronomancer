# Architectural Idioms for Chronomancer

This document outlines key architectural patterns and idioms used throughout the Chronomancer codebase. These patterns maintain consistency, reduce boilerplate, and make the message flow predictable. This is made up on the fly as we identify recurring patterns. Cosmic doesn't have strong opinions on architecture, so we define our own idioms here. Some of these patterns are well defined in web frameworks (my background), and others are more specific to Rust and Cosmic's MVU needs. I disliked how global application messages were handled in previous projects as a large app could have dozens of messages, so I devised a cleaner pattern here... in theory...

---

## Component-to-Page Message Flow

### The Problem

In a layered MVU (Model-View-Update) architecture with Components → Pages → App, we need a clean way for components to emit page-level messages without tightly coupling them to the app's message types or dealing with complex type conversions.

### The Solution: Optional Return with Recursive Update

Components return `Option<PageMessage>` instead of `Task<Action<PageMessage>>`. Pages check the return value and recursively call their own `update` method if a message was emitted.

### Pattern Structure

#### Component Trait
```rust
pub trait Component {
    fn view(&self) -> Element<'_, ComponentMessage>;
    fn update(&mut self, message: ComponentMessage) -> Option<PageMessage>;
}
```

#### Component Implementation
```rust
impl Component for PowerForm {
    fn update(&mut self, message: ComponentMessage) -> Option<PageMessage> {
        match message {
            ComponentMessage::TextChanged(new_text) => {
                self.input_value = new_text;
                None  // Local state change only
            }
            ComponentMessage::SubmitPressed => {
                if self.validate() {
                    Some(PageMessage::PowerFormSubmitted(self.get_value()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
```

#### Page Update Handler
```rust
impl Page for PowerControls {
    fn update(&mut self, message: PageMessage) -> Task<Action<AppMessage>> {
        match message {
            PageMessage::ComponentMessage(msg) => {
                // Check if component emits a page message
                let page_message = self.power_form.update(msg);
                if let Some(page_msg) = page_message {
                    // Recursively handle the page message
                    self.update(page_msg)
                } else {
                    Task::done(Action::None)
                }
            }
            PageMessage::PowerFormSubmitted(value) => {
                // Convert to AppMessage
                Task::done(Action::App(AppMessage::PowerMessage(
                    PowerMessage::SetValue(value)
                )))
            }
        }
    }
}
```

### Benefits

1. **No Type Conversion Gymnastics**: No need for helper functions to convert `Action<PageMessage>` to `Action<AppMessage>`
2. **Components Stay Simple**: Components don't need to know about `Task` or `Action` types
3. **Clear Message Flow**: Easy to trace: Component → (optional) PageMessage → Page → AppMessage
4. **Self-Documenting**: The `Option<PageMessage>` return type makes it obvious that a component *might* emit a page message
5. **Natural Recursion**: Pages naturally handle their own messages through the recursive `self.update()` call

### When to Use This Pattern

- ✅ Component needs to signal the page that something significant happened (form submitted, selection changed, etc.)
- ✅ The component's action requires page-level context (e.g., which radio button is selected)
- ✅ The component should remain reusable across different pages

### When NOT to Use This Pattern

- ❌ Component only changes its own internal state → return `None`
- ❌ Component needs to trigger async operations → consider a different architecture (or emit a PageMessage that the page converts to an async task)
- ❌ Message needs to go directly to the app without page involvement → this pattern adds an unnecessary layer

### Alternative Patterns Considered

1. **Components return `Task<Action<PageMessage>>`**
   - ❌ Requires pages to map actions: `task.map(|a| a.map(|page_msg| AppMessage::from(page_msg)))`
   - ❌ Components need to know about `Task` and `Action` types
   - ❌ More verbose and harder to read

2. **Components return `Task<Action<AppMessage>>`**
   - ❌ Tightly couples components to app-level message types
   - ❌ Makes components non-reusable across different apps or pages
   - ❌ Breaks separation of concerns

3. **Callback closures passed to components**
   - ❌ Not idiomatic in cosmic/iced MVU architecture
   - ❌ Harder to debug message flow
   - ❌ Requires careful lifetime management

### Real-World Example

See the `PowerForm` component and `PowerControls` page interaction:
- `src/components/power_form.rs` - Component implementation
- `src/pages/power_controls.rs` - Page handling with recursive update
- `src/components/mod.rs` - `Component` trait definition

---

## Future Idioms

As the project grows, document new architectural patterns here. I'm sure a lot will change as I write more applications and get feedback from contributors.