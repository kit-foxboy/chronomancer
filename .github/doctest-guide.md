# Documentation Testing Guide

## Project Policy: Doctests for Logic, Prose for GUI

Chronomancer uses a **selective doctesting strategy** to balance code quality with development velocity. We write executable doctests for pure utility functions, but use prose descriptions for GUI components.

## Current Status

- **53 passing doctests** - Core logic in `utils/` modules
- **0 ignored doctests** - No boilerplate examples in GUI code
- **0 failing doctests** - Clean test suite

## The Policy

### ✅ Write Doctests For

**Pure utility functions** in `utils/` modules:

```rust
/// Filters input to only accept positive integers.
///
/// Returns `Some(normalized)` for valid input, `None` otherwise.
///
/// ```rust
/// use chronomancer::utils::filters::filter_positive_integer;
///
/// assert_eq!(filter_positive_integer("42"), Some("42".to_string()));
/// assert_eq!(filter_positive_integer("007"), Some("7".to_string())); // Normalized
/// assert_eq!(filter_positive_integer("0"), None);
/// ```
#[must_use]
pub fn filter_positive_integer(input: &str) -> Option<String> {
    // ...
}
```

**Applies to:**
- `utils/filters.rs` - Text input validation
- `utils/time.rs` - Time formatting and conversion
- `utils/resources.rs` - System integration helpers
- Any pure function that doesn't involve GUI rendering

**Benefits:**
- Automatic regression testing
- Examples stay current with code
- Clear API contracts
- Fast to write (one-line assertions)

### ❌ Don't Write Doctests For

**UI components and GUI code** in `components/` and `pages/`:

```rust
/// Icon-based radio button that changes style when active.
///
/// Create with `ToggleIconRadio::new(index, icon_name)`, then render with
/// `.view(is_active, message)`. Use with [`RadioComponents`] for automatic
/// selection state management.
pub struct ToggleIconRadio {
    // ...
}
```

**Applies to:**
- `components/**` - All UI components
- `pages/**` - Page implementations
- `utils/ui/**` - UI spacing and layout helpers
- Anything returning `Element<Message>`

**Why not?**
- Requires extensive boilerplate (`Message` enums, imports, widget context)
- Users will look at real implementation code anyway
- Adds friction to development without proportional value
- Panel applet, not a library - working code IS the documentation

## Rationale

### This is a Panel Applet, Not a Library

**Library crates** (like `serde`, `tokio`) need extensive API examples because:
- Users import them into other projects
- Copy-paste examples are the primary way to learn
- Public API is the product

**Panel applets** are different:
- The working application IS the documentation
- Contributors read the actual source code
- Implementation patterns matter more than API examples

### Development Velocity Matters

For a learning project, we optimize for:
- **Fast iteration** - No boilerplate tax on every function
- **Focus on building** - Time writing features, not maintaining examples
- **Learning value** - Understanding code structure beats memorizing APIs

### Test What Actually Helps

**Doctests add value when:**
- ✅ Testing pure function behavior (filters, formatters, converters)
- ✅ Catching regressions in utility logic
- ✅ Documenting edge cases and normalization

**Doctests add friction when:**
- ❌ Requiring `Message` enum boilerplate for every component
- ❌ Needing widget macro imports and context
- ❌ Fighting lifetime issues in examples
- ❌ Maintaining examples that drift from real usage

## Writing Good Doctests

### Keep It Simple

```rust
/// ```rust
/// use chronomancer::utils::time::format_duration;
///
/// assert_eq!(format_duration(3600), "1 hour");
/// assert_eq!(format_duration(7200), "2 hours");
/// ```
```

### Show Edge Cases

```rust
/// ```rust
/// use chronomancer::utils::filters::filter_positive_integer;
///
/// // Valid positive integers
/// assert_eq!(filter_positive_integer("42"), Some("42".to_string()));
///
/// // Empty string is allowed
/// assert_eq!(filter_positive_integer(""), Some("".to_string()));
///
/// // Invalid inputs
/// assert_eq!(filter_positive_integer("0"), None);
/// assert_eq!(filter_positive_integer("-5"), None);
/// ```
```

### Add Explanatory Comments

```rust
/// ```rust
/// use chronomancer::utils::filters::filter_positive_integer;
///
/// assert_eq!(filter_positive_integer("007"), Some("7".to_string())); // Normalized
/// ```
```

## Writing Good Prose Documentation

### Be Concise and Actionable

```rust
/// Icon-based radio button that changes style when active.
///
/// Create with `new(index, icon_name)`, render with `view(is_active, message)`.
```

### Reference Related Types

```rust
/// Radio button group manager with selection state.
///
/// Create with `RadioComponents::new(options)`, render with `.view(on_select)`.
/// Options must implement [`RadioComponent`].
```

### Explain Key Concepts

```rust
/// Padding helpers for consistent container padding.
///
/// Provides methods for generating padding arrays in the format expected by
/// COSMIC widgets: `[top, right, bottom, left]`.
```

### Link to Working Examples

```rust
/// Form component for power management operations.
///
/// See `pages/power_controls.rs` for usage in a complete page.
```

## Running Tests

```bash
# Run all tests (unit + integration + doctests)
cargo test

# Run only doctests
cargo test --doc

# Run doctests for specific module
cargo test --doc utils::filters

# Run only unit tests
cargo test --lib
```

## Adding New Code

### For Utility Functions

1. Write the function
2. Add rustdoc with doctest examples
3. Run `cargo test --doc` to verify

### For Components

1. Write the component
2. Add concise rustdoc explaining usage
3. Reference working code if needed
4. **Don't** add doctest examples

## Migration Notes

This policy was adopted after initially writing comprehensive doctests for all modules. We removed ~200 lines of GUI example boilerplate while keeping 53 passing doctests for utility functions. The result:

- **Cleaner code** - Less noise in documentation
- **Faster development** - No boilerplate tax
- **Same test coverage** - Pure logic still validated
- **Better documentation** - Prose is clearer than forced examples

## See Also

- [Rust Book: Documentation Tests](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)
- [rustdoc Guide](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
- Project: `.github/copilot-instructions.md` - Overall testing strategy