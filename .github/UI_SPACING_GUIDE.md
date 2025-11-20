# UI Spacing & Sizing Guide

This guide explains the UI spacing and sizing standards for Chronomancer, centralized in `src/utils/ui/spacing.rs`. There's very little documentation on good UI practices in cosmic but in general it follows common design principles and favors being responsive. Fixed numbers are avoided in favor of semantic constants and responsive sizing functions more often than not. I was getting annoyed with extremely verbose use statements and formatting withing components, so this guide is just some personal notes on reducing boilerplate and improving consistency. Not sure this is useful to anyone else but me, but maybe it will be!

## Philosophy

- **No magic numbers**: Use semantic constants instead of hardcoded values
- **Responsive by default**: Adapt to container size when possible
- **COSMIC-first**: Leverage the COSMIC theme system for consistency
- **DRY**: Define once, use everywhere. Rust is not amazing for reuse in the way an OOP language would be (might be a skill issue with a rust noob like me) but good module and function structure can still reduce a lot of duplication.

## Quick Reference

### Component Sizes

```rust
use crate::utils::ui::ComponentSize;

// Icon buttons
ComponentSize::ICON_BUTTON_HEIGHT  // 48.0
ComponentSize::ICON_BUTTON_WIDTH   // 48.0
ComponentSize::ICON_SIZE           // 36

// Quick timer buttons
ComponentSize::QUICK_TIMER_BUTTON_HEIGHT  // 40.0

// Text inputs
ComponentSize::INPUT_MIN_WIDTH     // 60.0

// Typography
ComponentSize::HEADER_TEXT_SIZE    // 24.0
ComponentSize::BODY_TEXT_SIZE      // 14.0
```

### Spacing (Gaps)

```rust
use crate::utils::ui::Gaps;

Gaps::xxs()  // Extra extra small - very tight spacing
Gaps::xs()   // Extra small - related items within a group
Gaps::s()    // Small - grouping related elements
Gaps::m()    // Medium - separating distinct groups
Gaps::l()    // Large - major sections
Gaps::xl()   // Extra large - top-level sections
Gaps::xxl()  // Extra extra large - major visual breaks
```

### Padding

```rust
use crate::utils::ui::Padding;

Padding::none()                     // [0, 0, 0, 0]
Padding::tight()                    // [xxs, xxs, xxs, xxs]
Padding::standard()                 // [xs, xs, xs, xs]
Padding::comfortable()              // [s, s, s, s]
Padding::spacious()                 // [m, m, m, m]

Padding::horizontal(Gaps::s())      // [0, s, 0, s]
Padding::vertical(Gaps::m())        // [m, 0, m, 0]
Padding::custom(1, 2, 3, 4)         // [top, right, bottom, left]
```

### Length Helpers

```rust
use crate::utils::ui::{fill, fixed, shrink};

button.width(fill())           // Length::Fill
button.height(fixed(48.0))     // Length::Fixed(48.0)
text.width(shrink())           // Length::Shrink
```

### Responsive Sizing

```rust
use crate::utils::ui::ResponsiveSize;

// Adapt icon size based on container width
let icon_size = ResponsiveSize::icon_for_width(width);
// 0-200px   -> 24
// 201-300px -> 32
// 301-400px -> 36
// 400+px    -> 40

// Adapt button height based on container width
let button_height = ResponsiveSize::button_height_for_width(width);

// Adapt spacing based on container width
let gap = ResponsiveSize::gap_for_width(width);
```

## Usage Examples

### Before (with magic numbers)

```rust
column![buttons]
    .padding([8, 0])
    .spacing(12)
    .into()

button.height(Length::Fixed(48.0))
```

### After (with spacing helpers)

```rust
use crate::utils::ui::{Gaps, Padding, ComponentSize, fixed};

column![buttons]
    .padding(Padding::vertical(Gaps::xs()))
    .spacing(Gaps::s())
    .into()

button.height(fixed(ComponentSize::ICON_BUTTON_HEIGHT))
```

### Building a Responsive Component

```rust
use crate::utils::ui::{ComponentSize, Gaps, Padding, ResponsiveSize, fill, fixed};

fn view(&self, container_width: f32) -> Element<Message> {
    let icon_size = ResponsiveSize::icon_for_width(container_width);
    let gap = ResponsiveSize::gap_for_width(container_width);
    
    row![
        icon_button("my-icon", icon_size),
        text_input()
    ]
    .spacing(gap)
    .padding(Padding::standard())
    .into()
}
```

## When to Use Each Gap Size
* I'm not a UX expert, but based on common design principles in web and mobile, here are some guidelines for when to use each gap size:
| Gap Size | Use Case | Example |
|----------|----------|---------|
| `xxs()` | Items that must be visually connected | Form label + input |
| `xs()` | Related items in a group | Buttons in a toolbar |
| `s()` | Distinct but related sections | Different control groups |
| `m()` | Major sections | Page sections |
| `l()` | Top-level layout | Header from content |
| `xl()` | Dramatic separation | Modal from background |
| `xxl()` | Rarely needed | Large empty states |

## Component-Specific Guidelines

### Icon Buttons
- Use `ComponentSize::ICON_BUTTON_HEIGHT` for height
- Use `ComponentSize::ICON_SIZE` for icon dimensions
- Space with `Gaps::s()` in toolbars

### Quick Timer Buttons
- Use `ComponentSize::QUICK_TIMER_BUTTON_HEIGHT`
- Space with `Gaps::xs()` for tight groups

### Forms
- Use `ComponentSize::INPUT_MIN_WIDTH` for inputs
- Space form fields with `Gaps::s()`
- Use `Padding::standard()` for form containers

### Text
- Headers: `ComponentSize::HEADER_TEXT_SIZE`
- Body: `ComponentSize::BODY_TEXT_SIZE`
- Space header from content with `Gaps::s()`

## Adding New Standards

When you need to add a new standard:

1. **For sizes**: Add to `ComponentSize` struct
2. **For responsive logic**: Add to `ResponsiveSize` impl
3. **For new gap semantics**: Consider if existing gaps suffice first
4. **Update this guide**: Document the new standard. I'm using AI for writing guide templates when I make a design decision to at least make some attempt at an opinionated and reusable framework for future me and others. Rather than let it write code, this just consolidates my thoughts for both LLMs and other devs.

Example:
```rust
// In src/utils/ui/spacing.rs
impl ComponentSize {
    pub const NEW_COMPONENT_HEIGHT: f32 = 64.0;
}
```

## Testing

Not doing that yet but ideally we would have visual regression tests to ensure spacing consistency across UI changes. However, that type of testing is beyond my general expertise (I'm only experienced in that sort of thing regarding responsive CSS and even then, it's not my strongest area) and current project scope. If implemented, it's suggested to keep it extremely simple and broad as libcosmic is still under heavy beta development and examples of iced testing are sparse to say the least.

## Possible Future Improvements

- Dynamic theme switching (light/dark adjustments)
- Accessibility scaling factors
- Adding layouts that automatically apply these spacing standards (e.g., grids, fractional rows/columns, etc.)
- Better use of iced's built-in layout features (Admittedly I haven't explored iced's layout system in depth yet and am likely missing out on some useful features in favor of simplicity and just getting this functional)
- Using macros to reduce boilerplate in applying spacing/padding
