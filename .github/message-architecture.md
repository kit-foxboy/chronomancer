# Message Architecture

This document describes the decentralized message architecture used in Chronomancer, following libcosmic's recommended patterns. This is after a little experimentation with centralized vs decentralized message handling. It's not super important for small apps like this but if we ever want a thriving cosmic ecosystem, it's good to explore best practices early on. I've included how I've divided the messages, conversion chains, and usage examples. The use of the `From` trait for automatic conversions keeps the code clean and maintainable and saves us from a lot of ugly-ass mapping and nested enums.

## Overview

Messages are **decentralized** - each module owns its message types. This provides clear separation of concerns and makes the codebase easier to navigate and maintain.

## Architecture Principles

1. **Messages live with their domain** - Database messages in `utils/database`, power messages in `utils/power`, timer messages in `models/timer`, page messages in `pages/`
2. **Automatic conversion via `From` trait** - No manual wrapping needed in view functions
3. **Single coordination point** - `app_messages.rs` re-exports all message types for convenient importing
4. **Predictable conversion chains** - Clear, traceable message flow throughout the app

## Message Locations

### Service Messages (Business Logic)

- **`utils::database::DatabaseMessage`** - Database initialization and lifecycle
  - `Initialized(Result<SQLiteDatabase, String>)`
  - `FailedToInitialize(String)`

- **`utils::power::PowerMessage`** - Power management operations
  - `ToggleStayAwake` - Suspend inhibitor control
  - `InhibitAcquired(Arc<Result<File, String>>>)` - Inhibitor lock result
  - `SetSuspendTime(i32)` - Schedule suspend
  - `SetLogoutTime(i32)` - Schedule logout
  - `SetShutdownTime(i32)` - Schedule shutdown
  - `SetRebootTime(i32)` - Schedule reboot
  - `ExecuteSuspend` - Immediate suspend
  - `ExecuteLogout` - Immediate logout
  - `ExecuteShutdown` - Immediate shutdown
  - `ExecuteReboot` - Immediate reboot

- **`models::timer::TimerMessage`** - Timer operation results
  - `Created(Result<Timer, String>)` - Timer creation result
  - `ActiveFetched(Result<Vec<Timer>, String>)` - Active timer list

### Page Messages (UI)

- **`pages::PowerControlsMessage`** - Power controls page events
  - `RadioOptionSelected(usize)` - Power operation selected
  - `FormTextChanged(String)` - Time input changed
  - `FormTimeUnitChanged(TimeUnit)` - Unit changed
  - `FormSubmitPressed` - Submit button pressed
  - `ClearForm` - Reset form after submission
  - `ToggleStayAwake` - Stay awake toggle
  - `SetSuspendTime(i32)` - Set suspend timer
  - `SetShutdownTime(i32)` - Set shutdown timer
  - `SetLogoutTime(i32)` - Set logout timer
  - `SetRebootTime(i32)` - Set reboot timer
  - `ClosePopup` - Close popup request

- **`pages::TimerListMessage`** - Timer list page events
  - `ListHeaderMessage(ListHeaderMessage)` - Header interactions
  - `TimerFormSubmitted` - New timer submitted
  - `PauseTimer(usize)` - Pause timer request
  - `ResumeTimer(usize)` - Resume timer request
  - `DeleteTimer(usize)` - Delete timer request
  - `ToggleRecurring(usize)` - Toggle recurring status

- **`pages::PageMessage`** - Wrapper for all page messages
  - `PowerControlsMessage(PowerControlsMessage)`
  - `TimerListMessage(TimerListMessage)`

### App Messages (Coordination)

- **`app_messages::AppMessage`** - Top-level message dispatcher
  - `TogglePopup` - Show/hide applet popup
  - `UpdateConfig(Config)` - Save configuration
  - `Tick` - Regular timer update (1 second interval)
  - `Page(PageMessage)` - UI page events
  - `Database(DatabaseMessage)` - Database operations
  - `Timer(TimerMessage)` - Timer operations
  - `Power(PowerMessage)` - Power management

## Conversion Chains

### Page Messages

```text
power_controls::Message → PageMessage → AppMessage
timer_list::Message → PageMessage → AppMessage
```

**Implementation:**
```rust
// In pages/mod.rs
impl From<PowerControlsMessage> for PageMessage { ... }
impl From<TimerListMessage> for PageMessage { ... }

// In app_messages.rs
impl From<PageMessage> for AppMessage { ... }
```

### Service Messages

```text
DatabaseMessage → AppMessage
PowerMessage → AppMessage
TimerMessage → AppMessage
```

**Implementation:**
```rust
// In app_messages.rs
impl From<DatabaseMessage> for AppMessage { ... }
impl From<PowerMessage> for AppMessage { ... }
impl From<TimerMessage> for AppMessage { ... }
```

## Usage Examples

### In View Functions

```rust
// Pages automatically convert through PageMessage to AppMessage
fn view_window(&self, id: window::Id) -> Element<'_, Message> {
    let power = self
        .power_controls
        .view()
        .map(PageMessage::PowerControlsMessage)
        .map(Message::Page);

    let timer_list = self
        .timer_list
        .view()
        .map(PageMessage::TimerListMessage)
        .map(Message::Page);
    
    column![power, timer_list].into()
}
```

### In Update Functions

```rust
fn update(&mut self, message: Message) -> Task<Action<Message>> {
    match message {
        // Page messages
        Message::Page(PageMessage::PowerControlsMessage(msg)) => {
            self.handle_power_controls_message(msg)
        }
        Message::Page(PageMessage::TimerListMessage(msg)) => {
            self.timer_list.update(msg);
            Task::none()
        }
        
        // Service messages
        Message::Database(msg) => self.handle_database_message(msg),
        Message::Timer(msg) => self.handle_timer_message(msg),
        Message::Power(msg) => self.handle_power_message(msg),
        
        // App-level messages
        Message::TogglePopup => self.toggle_popup().map(|_| Action::None),
        Message::Tick => self.handle_tick(),
        Message::UpdateConfig(config) => {
            self.config = config;
            Task::none()
        }
    }
}
```

### In Async Tasks

```rust
// Database initialization
Task::perform(
    async move { SQLiteDatabase::new().await.map_err(|e| e.to_string()) },
    |result| match result {
        Ok(db) => Action::App(Message::Database(DatabaseMessage::Initialized(Ok(db)))),
        Err(err) => Action::App(Message::Database(DatabaseMessage::FailedToInitialize(err))),
    },
)

// Timer creation
Task::perform(
    async move {
        Timer::insert(database.pool(), &timer)
            .await
            .map_err(|e| e.to_string())
    },
    |result| Action::App(Message::Timer(TimerMessage::Created(result))),
)

// Power inhibitor acquisition
Task::perform(
    async move {
        resources::acquire_suspend_inhibit("Chronomancer", "Stay awake mode", "block")
            .await
            .map_err(|e| e.to_string())
    },
    |result| Action::App(Message::Power(PowerMessage::InhibitAcquired(Arc::new(result)))),
)
```

## Import Convenience

All message types are re-exported from `app_messages` for convenient importing:

```rust
use crate::app_messages::{
    AppMessage,
    DatabaseMessage,
    PageMessage,
    PowerMessage,
    TimerMessage,
};
```

This avoids deep imports like `crate::utils::database::DatabaseMessage` throughout the codebase while keeping the actual message definitions close to their implementation. It strikes a balance between discoverability and modularity hard won when I was suffering in my import statements.

## Benefits of This Architecture

1. **Locality** - Messages are defined near the code that uses them
2. **Scalability** - Adding new pages/services doesn't bloat a central file
3. **Type Safety** - Compiler prevents mixing concerns across domains
4. **Discoverability** - Easy to find where a message type is defined
5. **Maintainability** - Clear ownership and responsibility for each message type
6. **No Circular Dependencies** - Service modules don't import from app_messages

## Migration Notes

This architecture was adopted in place of a centralized `app_messages.rs` that contained all message enums. I was half-assing the modularity before and this is taking the libcosmic approach more to heart while keeping my desired project structure. The refactoring involved:

1. Moving `DatabaseMessage` to `utils/database/mod.rs`
2. Moving `PowerMessage` to `utils/power.rs` (new module)
3. Moving `TimerMessage` to `models/timer.rs`
4. Keeping `PageMessage` in `pages/mod.rs` (already decentralized)
5. Simplifying `app_messages.rs` to only contain `AppMessage` and re-exports
6. Updating all `From` implementations for clean conversion chains
7. Renaming enum variants in `AppMessage` for clarity:
   - `PageMessage` → `Page`
   - `DatabaseMessage` → `Database`
   - `TimerMessage` → `Timer`
   - `PowerMessage` → `Power`

## References

- [libcosmic Modules Documentation](https://pop-os.github.io/libcosmic-book/modules.html)
- [Project Architectural Idioms](.github/architectural-idioms.md)