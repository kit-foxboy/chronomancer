# Copilot Instructions for Chronomancer

## Project Overview

**Chronomancer** is a COSMIC panel applet for comprehensive time management, built with Rust and the libcosmic framework. This applet provides system timer management, recurring reminders, sleep timer overrides, and systemd integration.

## Project Type & Architecture

- **Type:** COSMIC Panel Applet (not a full desktop application)
- **Language:** Rust (Edition 2024)
- **Framework:** libcosmic (git master)
- **Architecture Pattern:** Clean Architecture with three layers:
  1. **UI Layer** - Popup windows with compact, focused interfaces
  2. **Application Layer** - Message routing, state management, background task coordination
  3. **Service Layer** - System integration (systemd, D-Bus, notifications, database)

## Current State

The project is in **early development**. The basic COSMIC application structure exists but needs to be converted to a panel applet architecture. The codebase contains:
- Basic application scaffolding (`main.rs`, `app.rs`)
- Configuration management (`config.rs`)
- Internationalization setup (`i18n.rs`, `i18n/en/chronomancer.ftl`)
- Build tooling (`Cargo.toml`, `justfile`)

## Key Technical Constraints

### Panel Applet Requirements
- **Compact UI:** Limited screen real estate, must be efficient
- **Always-on:** Persistent background process with quick access
- **System Integration:** Deep integration with systemd, D-Bus, and power management
- **Popup Windows:** Main interaction through small popup windows (not full windows)
- **Panel Position:** Must work in all four panel positions (top, bottom, left, right)

### Rust & COSMIC Patterns
- **Message-Driven Architecture (MVU):** All state changes flow through messages
- **Async Tasks:** Use `Task::perform()` for long-running operations
- **Messages Must Be:** `Clone + Debug`
- **Large Data in Messages:** Wrap in `Arc<>` to avoid expensive clones

## Core Features to Implement

### Phase 1: Basic Applet Structure
- Panel icon (shows in system panel, clickable)
- Popup window (opens on click, shows timer list)
- Database persistence (SQLite)
- Quick timer buttons ("5 min", "15 min", "1 hour")

### Phase 2: Reminders
- Create reminders (name, time, recurrence)
- Desktop notifications (via notify-rust)
- Reminder list (view upcoming, mark complete)
- Recurring logic (daily, weekly, custom intervals)

### Phase 3: Sleep Timer Overrides
- Detect current sleep/power settings
- Temporary override ("Don't sleep for X hours")
- Countdown display
- Auto-restore original settings

### Phase 4: systemd Integration
- List user's systemd timers
- Create timers via GUI
- Enable/disable timers
- View logs (last run, next scheduled)

## Critical Technical Guidelines

### Component Organization
Create reusable UI components from day one to avoid repetitive code:

```rust
// src/components/mod.rs
pub mod timer_card;
pub mod reminder_item;
pub mod quick_actions;

// Keep view() methods high-level and readable
fn view(&self) -> Element<Message> {
    components::popup()
        .section(components::active_timers(&self.timers))
        .section(components::quick_actions())
        .footer(components::settings_button())
}
```

### Async Task Pattern
```rust
// User action → Async task → Result message → UI update
Task::perform(
    async move {
        // Long-running operation (systemd call, database query)
        service.create_timer(name, duration).await
    },
    |result| Action::App(Message::TimerCreated(result.into()))
)
```

### Timer Precision
Store absolute end times, not elapsed durations:
```rust
// ✅ DO: Calculate from absolute timestamps
let now = SystemTime::now();
let remaining = end_time.duration_since(now).unwrap_or(Duration::ZERO);

// ❌ DON'T: Rely on elapsed time (drifts on system sleep)
let elapsed = start_time.elapsed();
```

### systemd User Timers
Always use `--user` flag for user-space timers:
```bash
systemctl --user enable my-timer.timer  # ✅ Correct
systemctl enable my-timer.timer         # ❌ Requires root, wrong scope
```

### Time Handling
- **Store:** Always use UTC internally
- **Display:** Convert to local time only in UI
- **Test:** Mock time in tests, don't use real `SystemTime::now()`
- **Handle:** Account for DST changes and time zone switches

### Error Handling Strategy
- **User-facing errors:** Toast notifications
- **Background errors:** Log silently, retry if safe
- **Critical errors:** Log loudly, show persistent notification
- **systemd/D-Bus:** Expect failures (busy, timeout), design for resilience

## Essential Dependencies

```toml
[dependencies]
# COSMIC framework
libcosmic = { git = "https://github.com/pop-os/libcosmic", features = ["applet", "tokio"] }

# Database
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio"] }
anyhow = "1.0"

# System integration
zbus = "4.0"              # D-Bus communication
notify-rust = "4.11"      # Desktop notifications

# Time handling
chrono = "0.4"            # Date/time parsing and formatting
tokio = { version = "1.0", features = ["time", "rt-multi-thread"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Localization
i18n-embed = { version = "0.16", features = ["fluent-system", "desktop-requester"] }
i18n-embed-fl = "0.10"
rust-embed = "8.8.0"
```

## Database Schema

```sql
-- Timers (one-time or countdown)
CREATE TABLE timers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    duration_seconds INTEGER NOT NULL,
    start_time INTEGER NOT NULL,  -- Unix timestamp
    created_at INTEGER NOT NULL
);

## Project Structure

```
chronomancer/
├── src/
│   ├── main.rs              # Applet entry point
│   ├── applet.rs            # Applet trait implementation
│   ├── config.rs            # User preferences
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   ├── timer_card.rs
│   │   ├── reminder_item.rs
│   │   └── quick_actions.rs
│   ├── services/            # System integration
│   │   ├── mod.rs
│   │   ├── systemd.rs       # systemd timer management
│   │   ├── notifications.rs # Desktop notifications
│   │   └── sleep_timer.rs   # Power management integration
│   ├── database/
│   │   ├── mod.rs           # Database trait
│   │   ├── sqlite.rs        # SQLite implementation
│   │   └── migrations/
│   └── i18n.rs              # Localization
├── resources/
│   └── icons/               # Panel icon (small, monochrome)
├── i18n/
│   └── en/
│       └── chronomancer.ftl
└── migrations/
    └── 0001_initial_schema.sql
```

## Code Style & Best Practices

### Import Organization
```rust
// ✅ DO: Import commonly-used types at module level
use cosmic::{Action, Task, Element};

// ✅ DO: Nest imports for clarity, structure, and brevity
use crate::{
    components::{Component, PowerControls, quick_timers},
    config::Config,
    models::{PowerMessage, Timer, TimerMessage},
    utils::{
        database::{DatabaseMessage, Repository, SQLiteDatabase},
        resources,
    },
};

// ✅ DO: Use type aliases for commonly-used complex types if needed
type DbResult<T> = Result<T, anyhow::Error>;

// ❌ DON'T: Use full paths everywhere
cosmic::Action::App(cosmic::app::Message::...)

// ❌ DON'T: Use functions, structs, or traits more than one item deep.
let element = cosmic::widget::container(...);

// ❌ DON'T: import multiple items from the same module separately
use cosmic::Action;
use cosmic::Task;
```

### Message Design
```rust
// Messages must be Clone + Debug
#[derive(Debug, Clone)]
pub enum Message {
    // Simple data can be cloned directly
    StartTimer(String, Duration),
    
    // Large data wrapped in Arc
    TimerCreated(Arc<Result<Timer, Error>>),
    
    // Async results
    DatabaseQueryComplete(Result<Vec<Reminder>, String>),
}
```

### Component Functions
```rust
// Helper functions for reusable UI elements
fn build_timer_card(timer: &Timer) -> Element<Message> {
    widget::container(
        widget::column()
            .push(widget::text(&timer.name))
            .push(widget::text(format_duration(timer.remaining())))
    )
    .into()
}
```

### Message Passing Strategy

**See `.github/architectural-idioms.md` for the full component-to-page message flow pattern.**

Components return `Option<PageMessage>` to signal page-level events. Pages handle these by recursively calling their own `update` method:

```rust
// Component trait - returns optional page message
pub trait Component {
    fn view(&self) -> Element<'_, ComponentMessage>;
    fn update(&mut self, message: ComponentMessage) -> Option<PageMessage>;
}

// Component implementation
impl Component for PowerForm {
    fn update(&mut self, message: ComponentMessage) -> Option<PageMessage> {
        match message {
            ComponentMessage::SubmitPressed => {
                if self.validate() {
                    Some(PageMessage::PowerFormSubmitted(self.value()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// Page handles component messages with recursive update
impl Page for PowerControls {
    fn update(&mut self, message: PageMessage) -> Task<Action<AppMessage>> {
        match message {
            PageMessage::ComponentMessage(msg) => {
                let page_message = self.component.update(msg);
                if let Some(page_msg) = page_message {
                    self.update(page_msg)  // Recursive call
                } else {
                    Task::done(Action::None)
                }
            }
            PageMessage::PowerFormSubmitted(value) => {
                Task::done(Action::App(AppMessage::PowerMessage(
                    PowerMessage::SetValue(value)
                )))
            }
        }
    }
}
```

**Key Benefits:**
- No type conversion helpers needed
- Components don't need `Task` or `Action` types
- Clear, traceable message flow
- Self-documenting with `Option<PageMessage>`
```
## Learning & Development Approach

This project is a **learning exercise**. When assisting:
- **Guide, don't railroad:** Explain options and trade-offs
- **Focus on learning:** Help understand *why*, not just *what*
- **Encourage experimentation:** It's okay to try things and iterate
- **Promote agency:** Support the developer's decisions and ideas
- **Start small, iterate:** Get one feature working before moving to the next
- **Test early:** Validate each feature as it's built

## Common Gotchas

1. **Background Timer Drift:** System sleep breaks elapsed time calculations
2. **User vs System Timers:** Must use `--user` flag with systemctl
3. **Notification Persistence:** Store notification history in database
4. **Panel Geometry:** Popup position must adapt to panel location
5. **Resource Cleanup:** Background timers can leak on crash
6. **Time Zone Changes:** User travels, reminders fire at wrong time

## Success Criteria

- ✅ Applet runs persistently, no crashes for 24+ hours
- ✅ Timer accuracy within ±1 second
- ✅ Zero compiler warnings
- ✅ Code is readable (view() methods < 50 lines with components)
- ✅ Works across all four panel positions
- ✅ Database migrations apply cleanly
- ✅ systemd timers created via GUI actually run
- ✅ Sleep override successfully prevents suspend

## Resources

### Project Documentation
- **Architectural Idioms:** `.github/architectural-idioms.md` - Component-to-page message flow and other patterns
- **UI Spacing Guide:** `.github/UI_SPACING_GUIDE.md`
- **Iterator Patterns:** `.github/iterator-patterns.md`
- **Icon Theming Notes:** `.github/icon-theming-notes.md`
- **Macro Explanations:** `.github/macro-explanations.md`

### External Documentation
- **libcosmic Applets:** https://pop-os.github.io/libcosmic-book/panel-applets.html
- **libcosmic API:** https://pop-os.github.io/libcosmic/cosmic/
- **systemd D-Bus API:** https://www.freedesktop.org/wiki/Software/systemd/dbus/
- **XDG Base Directory:** https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
- **Tokio Time:** https://tokio.rs/tokio/topics/time
- **zbus Tutorial:** https://dbus2.github.io/zbus/

## Build & Run

```bash
# Build debug version
just build

# Build release version
just

# Run the applet
just run

# Check for errors
just check

# Install system-wide
just install
```

## Testing

- **Unit Tests:** Test services independently with mocked dependencies
- **Integration Tests:** Mock systemd D-Bus interface
- **Manual Testing:** All four panel positions, light/dark themes, system sleep/wake cycles

## Data Storage

Follow XDG Base Directory specification:
- **Config:** `~/.config/com.vulpineinteractive.chronomancer/`
- **Data:** `~/.local/share/com.vulpineinteractive.chronomancer/`
- **Database:** `~/.local/share/com.vulpineinteractive.chronomancer/chronomancer.db`


## License

MIT License (see LICENSE file)
