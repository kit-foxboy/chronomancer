# Integration Tests

This directory contains integration tests for Chronomancer. These tests verify that different modules work together correctly through the public API. Rust provides a robust testing framework that makes it easy to write and run these tests even if I'm not the biggest fan of how Unit tests are organized. These at least live in a separate directory which is nice.

## Rust Testing Philosophy

Rust has two types of tests:

### Unit Tests
- **Location:** Inside source files in `#[cfg(test)] mod tests { ... }` blocks
- **Purpose:** Test individual functions and internal logic in isolation
- **Access:** Can test private functions and implementation details
- **Examples:** See `src/models/timer.rs`, `src/components/icon_button.rs`, etc.

### Integration Tests
- **Location:** This directory (`tests/`)
- **Purpose:** Test how modules work together through public APIs
- **Access:** Only public interfaces (like an external consumer)
- **Examples:** `timer_integration.rs`

## Running Tests

```bash
# Run all tests (unit + integration)
cargo test

# Run only integration tests
cargo test --test '*'

# Run a specific integration test file
cargo test --test timer_integration

# Run with verbose output
cargo test -- --nocapture

# Run tests matching a pattern
cargo test timer_creation
```

## Writing Integration Tests

Each `.rs` file in this directory becomes a separate test binary. Follow these patterns:

### Basic Structure
```rust
// Import from the crate's public API
use chronomancer::models::timer::{Timer, TimerType};

#[test]
fn my_integration_test() {
    // Arrange: Set up test data
    let timer = Timer::new(60, false, &TimerType::Suspend);
    
    // Act: Perform operations
    let is_active = timer.is_active();
    
    // Assert: Verify results
    assert!(is_active, "Timer should be active");
}
```

### Async Tests (for database operations)
```rust
// Add to Cargo.toml [dev-dependencies]:
// tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

#[tokio::test]
async fn database_integration_test() {
    // Test database operations here
    // See commented examples in timer_integration.rs
}
```

## Current Test Files

- **`timer_integration.rs`**: Tests for Timer creation, type conversions, and expiry logic
  - Includes commented-out examples for database integration tests
  - Database tests require setup of isolated test database (see TODOs in file)

## Best Practices

1. **Test behavior, not implementation**: Focus on what the code does, not how it does it
2. **Use descriptive test names**: `timer_creation_basic` not `test1`
3. **One assertion per test** (when practical): Makes failures easier to diagnose
4. **Arrange-Act-Assert pattern**: Clearly separate setup, execution, and verification
5. **Avoid test interdependence**: Each test should be independently runnable
6. **Use helper functions**: Reduce boilerplate (e.g., `setup_test_db()`)

## Integration Test Ideas

As you develop features, consider adding tests for:

- Database operations (insert, fetch, delete)
- Timer expiry and notification flow
- Multiple timers interacting
- Power management integration
- Configuration loading and persistence
- Error handling and recovery

## Debugging Failed Tests

```bash
# Show test output even on success
cargo test -- --show-output

# Run a specific test with backtrace
RUST_BACKTRACE=1 cargo test test_name

# Run tests serially (not in parallel)
cargo test -- --test-threads=1
```

## CI Integration

These tests run automatically in GitHub Actions (see `.github/workflows/ci.yml`). Tests must pass before PRs can be merged.
