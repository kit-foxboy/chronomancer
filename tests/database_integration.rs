// ============================================================================
// Database Integration Tests (Async)
// ============================================================================
//
// These are commented out skeletons. To enable:
// 1. Add tokio to dev-dependencies in Cargo.toml:
//    [dev-dependencies]
//    tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
// 2. Create a test helper to set up an isolated test database
// 3. Uncomment and adjust these tests
//
// Key concept: Integration tests should use an isolated database (in-memory
// or temporary file) so they don't pollute your actual app data.

use chronomancer::utils::database::repository::Repository;
use chronomancer::utils::database::sqlite::SQLiteDatabase;
use chronomancer::{Timer, TimerType};

/// Helper to create a test database (in-memory or temp file)
async fn setup_test_db() -> SQLiteDatabase {
    SQLiteDatabase::new_in_memory()
        .await
        .expect("Test DB setup failed")
}

#[tokio::test]
async fn timer_insert_and_fetch() {
    let db = setup_test_db().await;
    let timer_type = TimerType::Suspend;
    let timer = Timer::new(120, false, &timer_type);

    // Insert timer
    let saved = Timer::insert(db.pool(), &timer)
        .await
        .expect("Failed to insert timer");

    assert!(saved.id > 0, "Saved timer should have a valid ID");
    assert_eq!(saved.description, timer.description);

    // Fetch by ID
    let fetched = Timer::get_by_id(db.pool(), &saved.id)
        .await
        .expect("Failed to fetch timer")
        .expect("Timer not found");

    assert_eq!(fetched.id, saved.id);
    assert_eq!(fetched.description, saved.description);
    assert_eq!(fetched.ends_at, saved.ends_at);
}

#[tokio::test]
async fn timer_get_all_active() {
    let db = setup_test_db().await;

    // Create and insert multiple timers
    let timer1 = Timer::new(60, false, &TimerType::Suspend);
    let timer2 = Timer::new(120, false, &TimerType::Shutdown);

    Timer::insert(db.pool(), &timer1)
        .await
        .expect("Insert failed");
    Timer::insert(db.pool(), &timer2)
        .await
        .expect("Insert failed");

    // All timers should be active
    let active = Timer::get_all_active(db.pool())
        .await
        .expect("Failed to fetch active timers");

    assert_eq!(active.len(), 2, "Should have 2 active timers");
}

#[tokio::test]
async fn timer_delete() {
    let db = setup_test_db().await;
    let timer = Timer::new(60, false, &TimerType::Logout);

    let saved = Timer::insert(db.pool(), &timer)
        .await
        .expect("Insert failed");

    Timer::delete_by_id(db.pool(), &saved.id)
        .await
        .expect("Delete failed");

    // Verify it's gone
    let result = Timer::get_by_id(db.pool(), &saved.id)
        .await
        .expect("Fetch failed");

    assert!(result.is_none(), "Timer should be deleted");
}

#[tokio::test]
async fn timer_expired_not_in_active_list() {
    let db = setup_test_db().await;

    // Create timer that's already expired
    let mut expired_timer = Timer::new(1, false, &TimerType::UserDefined("Test".into()));
    expired_timer.ends_at = chrono::Utc::now().timestamp() - 100;

    let saved = Timer::insert(db.pool(), &expired_timer)
        .await
        .expect("Insert failed");

    // get_all_active should NOT include expired timers
    let active = Timer::get_all_active(db.pool())
        .await
        .expect("Fetch failed");

    assert!(
        !active.iter().any(|t| t.id == saved.id),
        "Expired timer should not appear in active list"
    );
}
