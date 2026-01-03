# Documentation Testing Guide

## Overview

Chronomancer uses Rust's built-in **documentation testing** (doctests) to validate code examples in our API documentation. When you run `cargo test`, Rust automatically compiles and runs code blocks in `///` doc comments.
Is this immense overkill for a project this small? Almost certainly! But I saw that doctests are a thing and really wanted to check it out. And because I'm in charge, NONE SHALL STOP MY CAPRICIOUS WHIMS! >:3

## Current Status (as of writing)

- **53 passing doctests** - Core logic and utilities are fully tested
- **18 ignored doctests** - UI examples marked as illustrative only
- **0 failing doctests** - All examples either pass or are explicitly ignored

## Doctest Annotations

Rust supports three doctest modes looks like:

### ````rust` - Compile and Run

Use for **pure functions** and **testable logic**:

```rust
/// Filters input to only accept positive integers.
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::filters::filter_positive_integer;
///
/// assert_eq!(filter_positive_integer("42"), Some("42".to_string()));
/// assert_eq!(filter_positive_integer("0"), None);
/// ```
```

✅ **Use when:**
- Testing utility functions (filters, formatters, converters)
- Examples can be self-contained
- Logic is deterministic and doesn't require external state

✅ **Benefits:**
- Free integration tests
- Examples stay up-to-date with code
- API contracts are validated automatically

### ````rust,no_run` - Compile Only

Use for **examples that compile but shouldn't execute**:

```rust
/// Creates a new toggle radio button.
///
/// # Examples
///
/// ```rust,no_run
/// use chronomancer::components::icon_button::ToggleIconRadio;
///
/// fn example() {
///     let button = ToggleIconRadio::new(0, "system-suspend-symbolic");
///     let _element = button.view(true, Message::Selected);
/// }
/// ```
```

✅ **Use when:**
- Example compiles but needs runtime context to execute
- Demonstrating API usage patterns
- Type signatures need validation

❌ **Avoid when:**
- Example has lifetime issues or requires complex setup
- Better to use `ignore` and simplify the example

### ````rust,ignore` - Don't Compile

Use for **illustrative pseudocode** and **complex UI examples**:

```rust
/// Displays radio buttons with custom spacing.
///
/// # Examples
///
/// ```rust,ignore
/// use chronomancer::components::radio_components::RadioComponents;
///
/// let options = vec![
///     ToggleIconRadio::new(0, "icon-one"),
///     ToggleIconRadio::new(1, "icon-two"),
/// ];
///
/// let radio_group = RadioComponents::new(options);
/// radio_group.view(|index| Message::Selected(index))
/// ```
```

✅ **Use when:**
- UI components that need full cosmic/libcosmic context
- Widget examples with macro complexity (e.g., `column!`, `row!`)
- Lifetime issues would require extensive boilerplate
- Focus is on showing usage patterns, not testing correctness

## Categories in Chronomancer

### Always Test (````rust`)

These modules have **real, executable doctests** because they are nice and boring with none of that graphical interface and multithreading bullshit:

- `utils/filters.rs` - Text input validation
- `utils/time.rs` - Time formatting and conversion
- `utils/resources.rs` - System integration helpers

### Always Ignore (````rust,ignore`)

These modules use **illustrative examples only** and basically exist for reading the nifty HTML that cargo generates for me in the target/doc folder:

- `components/**` - UI components (need widget context)
- `utils/ui/spacing.rs` - Layout helpers (need widget macros)

## Best Practices

### ✅ DO

1. **Write real tests for pure functions**
   ```rust
   /// ```rust
   /// assert_eq!(my_function(5), 10);
   /// ```
   ```

2. **Keep examples focused and minimal**
   ```rust
   /// ```rust,ignore
   /// // Just show the key API call
   /// let result = do_thing(arg);
   /// ```
   ```

3. **Use `ignore` for complex UI code**
   ```rust
   /// ```rust,ignore
   /// column![]
   ///     .spacing(Gaps::s())
   /// ```
   ```

4. **Add context comments when needed**
   ```rust
   /// ```rust
   /// let filtered = filter_positive_integer("007");
   /// assert_eq!(filtered, Some("7".to_string())); // Normalized
   /// ```
   ```

### ❌ DON'T

1. **Don't write complex setup just to make it compile**
   - If it needs more than 3 lines of boilerplate, use `ignore`

2. **Don't ignore tests that could easily pass**
   - Pure functions should always be tested

3. **Don't return borrowed data in examples**
   - Causes lifetime issues; assign to `_variable` instead

4. **Don't use `no_run` when `ignore` is clearer**
   - If it won't compile, use `ignore` not `no_run`

## Running Doctests

```bash
# Run all tests (including doctests)
cargo test

# Run only doctests
cargo test --doc

# Run doctests for specific module
cargo test --doc utils::filters

# Show ignored doctests
cargo test --doc -- --ignored
```

## Maintenance

When adding new documentation:

1. **Start with ````rust`** for pure functions
2. **Use ````rust,ignore`** for UI/component examples
3. **Run `cargo test --doc`** to verify
4. **Fix failures by:**
   - Fixing the example (if trivial)
   - Adding `ignore` (if complex)

## Philosophy

> **Documentation examples should teach, not test UI wiring.**

- **Teach:** Show how to call the function
- **Don't test:** Widget macro syntax and cosmic internals

I'll be completely honest: While this is really neat because it makes your documentation both accurate and functional, holy moly is it ever a f#@*ton of overhead! I also kind of hate how it clogs files and makes it more annoying to just modify your code. I'm trying really hard to learn as much about Rust as I can, so I'm exploring all the features and working with their idioms. While I think this is important, I'm likely going to create a branch every release that has the doctests so the main codebase is cleaner. We'll see based on how annoyed I get lmao. One other thing of note: I don't let AI write code for me AND NEITHER SHOULD YOU, but I do let it help me write documentation and test. Doctests involve repetitive boilerplate that follows a strict pattern, so it's a natural fit.

## See Also

- [Rust Book: Documentation Tests](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)
- [rustdoc Guide: Documentation Tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html)
