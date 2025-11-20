# Iterator Patterns for Avoiding Cloning and Borrowing Issues

## Overview

This guide covers common iterator patterns in Rust that help you avoid unnecessary cloning and resolve borrowing conflicts, particularly when building up collections or complex data structures.

## Core Concept: Ownership in Iterators

The key insight is that many builder-style APIs (like `iced::Row`, `Vec`, etc.) take ownership of `self` and return `Self`, allowing you to chain operations without borrowing.

```rust
// ❌ This doesn't work - borrowing issue
let mut row = row![];
for item in items {
    &row.push(item); // Error: can't borrow `row` as mutable
}

// ✅ This works - ownership transfer
let mut row = row![];
for item in items {
    row = row.push(item); // Takes ownership, returns new row
}

// ✅ Even better - using fold
let row = items.iter().fold(row![], |row, item| {
    row.push(item)
});
```

## Pattern 1: `fold()` for Building Structures

`fold()` is ideal when you need to build up a structure by consuming and returning it on each iteration.

### Basic Example

```rust
use cosmic::iced_widget::row;
use cosmic::widget::button;

fn build_button_row(labels: &[&str]) -> cosmic::iced::widget::Row<Message> {
    labels
        .iter()
        .fold(row![], |row, &label| {
            row.push(button::standard(label).on_press(Message::Click(label)))
        })
}
```

### With Initial Value

```rust
fn build_column_with_header(items: &[String]) -> cosmic::iced::widget::Column<Message> {
    items
        .iter()
        .fold(
            column![text("Header").size(20)], // Initial accumulator
            |col, item| {
                col.push(text(item))
            }
        )
}
```

### Multiple Transformations

```rust
fn build_styled_list(items: &[(String, bool)]) -> cosmic::iced::widget::Column<Message> {
    items
        .iter()
        .fold(column![], |col, (text, is_active)| {
            let element = if *is_active {
                text(text).style(theme::Text::Accent)
            } else {
                text(text).style(theme::Text::Default)
            };
            col.push(element)
        })
}
```

## Pattern 2: `reduce()` for Homogeneous Collections

`reduce()` is useful when your accumulator and items are the same type.

```rust
fn combine_texts(strings: Vec<String>) -> Option<String> {
    strings
        .into_iter()
        .reduce(|acc, s| format!("{}, {}", acc, s))
}
```

## Pattern 3: Mutable Accumulator with Reassignment

When you need more complex logic, use a mutable variable and reassign on each iteration.

```rust
fn build_conditional_ui(items: &[Item]) -> Element<Message> {
    let mut container = column![].spacing(10);
    
    for item in items {
        if item.should_show {
            container = container.push(text(&item.name));
            
            if item.has_details {
                container = container.push(text(&item.details).size(12));
            }
        }
    }
    
    container.into()
}
```

## Pattern 4: `collect()` for Simple Transformations

When you just need to transform elements without building a custom structure, `collect()` is sufficient.

```rust
fn get_active_names(items: &[Item]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.is_active)
        .map(|item| item.name.clone())
        .collect()
}
```

## Pattern 5: Avoiding Clones with References

### Using Borrowed Data

```rust
// ❌ Unnecessary clones
fn build_ui_cloning(items: &[String]) -> Element<Message> {
    items
        .iter()
        .fold(column![], |col, item| {
            col.push(text(item.clone())) // Unnecessary clone!
        })
        .into()
}

// ✅ Borrow instead
fn build_ui_borrowing(items: &[String]) -> Element<Message> {
    items
        .iter()
        .fold(column![], |col, item| {
            col.push(text(item)) // text() accepts &str
        })
        .into()
}
```

### Using `Cow` for Conditional Ownership

```rust
use std::borrow::Cow;

fn get_display_name(item: &Item) -> Cow<str> {
    if item.has_custom_name() {
        Cow::Owned(format!("{} (custom)", item.name))
    } else {
        Cow::Borrowed(&item.name)
    }
}
```

## Pattern 6: HashMap/BTreeMap Iteration

When iterating over maps to build UI, remember that iteration order matters.

```rust
use std::collections::HashMap;

// ❌ HashMap has undefined order
fn build_from_hashmap(map: &HashMap<String, i32>) -> Element<Message> {
    map.iter()
        .fold(column![], |col, (key, &value)| {
            col.push(text(format!("{}: {}", key, value)))
        })
        .into()
}

// ✅ Sort first for predictable UI
fn build_from_hashmap_sorted(map: &HashMap<String, i32>) -> Element<Message> {
    let mut items: Vec<_> = map.iter().collect();
    items.sort_by_key(|(key, _)| *key);
    
    items
        .into_iter()
        .fold(column![], |col, (key, &value)| {
            col.push(text(format!("{}: {}", key, value)))
        })
        .into()
}
```

## Pattern 7: Chaining with `inspect()` for Debugging

```rust
fn build_debug_ui(items: &[Item]) -> Element<Message> {
    items
        .iter()
        .inspect(|item| println!("Processing: {}", item.name)) // Debug
        .filter(|item| item.is_visible)
        .fold(column![], |col, item| {
            col.push(text(&item.name))
        })
        .into()
}
```

## Pattern 8: Early Return with `try_fold()`

When you need to handle errors during iteration without collecting first.

```rust
fn build_validated_ui(items: &[Item]) -> Result<Element<Message>, Error> {
    items
        .iter()
        .try_fold(column![], |col, item| {
            let validated = item.validate()?;
            Ok(col.push(text(validated.name())))
        })
        .map(|col| col.into())
}
```

## Pattern 9: Partition for Conditional Processing

```rust
fn build_two_column_layout(items: &[Item]) -> Element<Message> {
    let (active, inactive): (Vec<_>, Vec<_>) = items
        .iter()
        .partition(|item| item.is_active);
    
    row![
        active.iter().fold(column![], |col, item| {
            col.push(text(&item.name))
        }),
        inactive.iter().fold(column![], |col, item| {
            col.push(text(&item.name).style(theme::Text::Disabled))
        })
    ]
    .into()
}
```

## Pattern 10: Enumerate for Indexed Operations

```rust
fn build_numbered_list(items: &[String]) -> Element<Message> {
    items
        .iter()
        .enumerate()
        .fold(column![], |col, (idx, item)| {
            col.push(text(format!("{}. {}", idx + 1, item)))
        })
        .into()
}
```

## Common Pitfalls

### Pitfall 1: Forgetting to Reassign

```rust
// ❌ This does nothing - result is dropped
let mut row = row![];
for item in items {
    row.push(item); // Returns new row but doesn't reassign!
}

// ✅ Reassign the result
let mut row = row![];
for item in items {
    row = row.push(item);
}
```

### Pitfall 2: Unnecessary `collect()` Before `fold()`

```rust
// ❌ Unnecessary intermediate allocation
let items: Vec<_> = data.iter().filter(|x| x.active).collect();
let ui = items.iter().fold(column![], |col, item| col.push(text(item)));

// ✅ Chain directly
let ui = data
    .iter()
    .filter(|x| x.active)
    .fold(column![], |col, item| col.push(text(item)));
```

### Pitfall 3: Cloning in Closures

```rust
// ❌ Unnecessary clone in closure
let ui = items.iter().fold(column![], |col, item| {
    col.push(make_widget(item.clone())) // Clone might not be needed
});

// ✅ Check if the function accepts references
let ui = items.iter().fold(column![], |col, item| {
    col.push(make_widget(item)) // If make_widget accepts &Item
});
```

## Real-World Example: Button Row Component

Here's a complete example from the Chronomancer project:

```rust
use std::collections::HashMap;
use cosmic::{Element, iced_widget::row, widget::button};

pub struct ButtonRow {
    pub buttons: HashMap<String, i32>,
    pub spacing: u16,
}

impl ButtonRow {
    pub fn view(&self) -> Element<Message> {
        self.buttons
            .iter()
            .fold(row![].spacing(self.spacing), |row, (label, &seconds)| {
                row.push(
                    button::standard(label)
                        .on_press(Message::StartTimer(seconds))
                )
            })
            .into()
    }
}
```

**Key points:**
- No cloning needed - `label` is `&String` which can be converted to `&str`
- `fold()` handles the ownership chain elegantly
- `spacing()` is called on the initial accumulator
- Each iteration consumes the row and returns a new one

## Performance Considerations

1. **Lazy evaluation**: Iterators are lazy - transformations don't happen until consumed
2. **Zero-cost abstractions**: Well-written iterator chains compile to the same assembly as manual loops
3. **Avoid collecting unnecessarily**: Chain operations directly when possible
4. **Consider `Vec::with_capacity()`**: If you know the size and must collect

## Exercise Ideas

1. Convert a `for` loop building a UI to use `fold()`
2. Refactor code that clones unnecessarily to use references
3. Build a complex nested UI structure using only iterator chains
4. Handle errors in UI building with `try_fold()`
5. Create a sorted, filtered, transformed UI from unsorted data
6. Build conditional layouts using `partition()`
7. Optimize a function that collects intermediate results unnecessarily

## Summary

- **Use `fold()`** when building structures that consume and return `self`
- **Reassign the result** when using `for` loops with builder patterns
- **Avoid clones** by checking if functions accept references
- **Chain iterators** directly instead of collecting intermediate results
- **Consider iterator order** when building UI from maps
- **Use `try_fold()`** for error handling during iteration
- **Remember**: Iterator chains are zero-cost abstractions in release builds

## Additional Resources

- [Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Iterator Trait Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Effective Rust - Use Iterators](https://www.lurklurk.org/effective-rust/iter.html)
