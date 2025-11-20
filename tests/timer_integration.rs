//! Integration tests for Timer operations.
//!
//! These tests exercise the Timer module through its public API,
//! demonstrating how components work together. Unlike unit tests,
//! these can't access private internals.
//! I kind of hate how Rust idoms say to  put unit tests right in the same file as the code.
//! Integration tests being separated is very sensible at least.
//! Coming from TypeScript, I prefer tests that treat modules as black boxes,
//! focusing on public interfaces rather than implementation details.
use chronomancer::models::timer::{Timer, TimerType};
use std::str::FromStr;

// ============================================================================
// Basic Timer Creation and State
// ============================================================================

#[test]
fn timer_creation_basic() {
    let timer = Timer::new(60, false, &TimerType::Suspend);

    assert_eq!(timer.description, "System Suspend");
    assert!(!timer.is_recurring);
    assert!(timer.is_active(), "Newly created timer should be active");
}

#[test]
fn timer_type_conversions() {
    // Test all built-in types
    let suspend = TimerType::from_str("System Suspend").expect("Failed to parse System Suspend");
    assert!(matches!(suspend, TimerType::Suspend));
    assert_eq!(suspend.as_str(), "System Suspend");

    let logout = TimerType::from_str("System Logout").expect("Failed to parse System Logout");
    assert!(matches!(logout, TimerType::Logout));
    assert_eq!(logout.as_str(), "System Logout");

    let shutdown = TimerType::from_str("System Shutdown").expect("Failed to parse System Shutdown");
    assert!(matches!(shutdown, TimerType::Shutdown));
    assert_eq!(shutdown.as_str(), "System Shutdown");

    // Test custom type
    let custom = TimerType::from_str("My Custom Timer").expect("Failed to parse custom timer");
    match &custom {
        TimerType::UserDefined(name) => assert_eq!(name, "My Custom Timer"),
        _ => panic!("Expected UserDefined variant"),
    }
    assert_eq!(custom.as_str(), "My Custom Timer");
}

#[test]
fn timer_expiry_manual() {
    let mut timer = Timer::new(5, false, &TimerType::UserDefined("Test".into()));
    assert!(timer.is_active());

    // Force expiry by setting ends_at to the past
    timer.ends_at = chrono::Utc::now().timestamp() - 1;
    assert!(
        !timer.is_active(),
        "Timer with past end time should be inactive"
    );
}
