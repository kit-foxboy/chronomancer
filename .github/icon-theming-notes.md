# Icon Theming in COSMIC/libcosmic

## Key Finding: Custom SVG Widgets Don't Support Theme Colors

### The Problem

When embedding custom SVG icons using `include_bytes!()` and `svg::Handle`, the `currentColor` attribute **does not respect COSMIC's theme system**. The SVG renders as-is with literal colors.

### Why This Happens

1. **`icon::from_name()`** - Uses COSMIC's `Icon` widget which:
   - Looks up icons through the system icon theme
   - Applies theme colors (accent, text colors) automatically
   - Works with symbolic icons that use `currentColor`

2. **`svg::Handle::from_memory()`** - Uses iced's raw SVG widget which:
   - Renders SVG exactly as provided
   - Does NOT apply theme transformations
   - Treats `currentColor` as literal black (default color)

### The Solution

**Use system icon names** that exist in the icon theme instead of embedding custom SVGs:

```rust
// ✅ This works - accent color applied automatically
icon::from_name("system-shutdown-symbolic")
    .size(40)
    .into()

// ❌ This doesn't theme - renders black regardless of theme
let svg_data = include_bytes!("custom-icon.svg");
let handle = svg::Handle::from_memory(svg_data);
widget::svg(handle).width(40).height(40).into()
```

### Alternative: Install Custom Icons to System Theme

If you really want custom icons with theming:

1. **Create proper symbolic icons** (monochrome, `fill="currentColor"`)
2. **Install to icon theme directory:**
   ```bash
   ~/.local/share/icons/YourTheme/scalable/actions/your-icon-symbolic.svg
   ```
3. **Use with `icon::from_name()`:**
   ```rust
   icon::from_name("your-icon-symbolic").size(40)
   ```

### Icon Name Conventions

System icon names follow freedesktop.org standards:
- **Symbolic suffix:** `-symbolic` for monochrome theme-adaptive icons
- **Categories:** `system-*`, `weather-*`, `media-*`, etc.
- **Standard names:**
  - `system-shutdown-symbolic`
  - `system-suspend-symbolic`
  - `system-log-out-symbolic`
  - `system-lock-screen-symbolic`
  - `weather-clear-night-symbolic` (good for "stay awake" metaphor)

### Theme Button Classes

When using icon buttons, apply the correct theme class:

```rust
button::custom(icon::from_name("system-shutdown-symbolic").size(40))
    .class(theme::Button::Icon)  // ✅ Applies accent color to Icon widgets
    .into()
```

The `Button::Icon` class tells COSMIC to colorize the icon with the accent color, but this only works with `Icon` widgets, not raw `svg` widgets.

## Verification from BeautyVulpine Theme

The BeautyVulpine custom theme icons use **hardcoded gradients**, not `currentColor`:

```xml
<linearGradient id="paint0">
  <stop stop-color="#DC18FC"/>
  <stop offset="1" stop-color="#8E1EFF"/>
</linearGradient>
```

This means even theme-installed icons don't automatically adapt to the active theme unless they're designed as symbolic icons with `currentColor`.

## Recommendation

For the Chronomancer applet:
1. Use standard system icon names for power controls
2. Document which icons represent which actions
3. Consider contributing symbolic icon designs to COSMIC if standard names don't fit the use case

Standard icons are:
- **Predictable** - Users recognize them across COSMIC apps
- **Theme-consistent** - Always match the active accent color
- **Maintained** - Updated by COSMIC/freedesktop.org icon theme maintainers

## Chronomancer Solution: Install Custom Icons During Build

We've configured the justfile to automatically install custom icons with the `chronomancer-` prefix:

### Workflow

1. **Create custom icons** in `resources/icons/hicolor/scalable/apps/`
   - Design as symbolic icons with `fill="currentColor"`
   - Name descriptively (e.g., `stay-awake.svg`)

2. **Install icons** with `just install`
   - Icons are automatically copied to system theme directory
   - Prefixed with `chronomancer-` (e.g., `chronomancer-stay-awake.svg`)

3. **Use in code** with `icon::from_name()`
   ```rust
   icon::from_name("chronomancer-stay-awake").size(40)
   ```

### Justfile Configuration

```makefile
# Install all custom icons with chronomancer- prefix
install:
    @for icon in resources/icons/hicolor/scalable/apps/*.svg; do \
        basename="$$(basename "$$icon")"; \
        if [ "$$basename" != "hourglass.svg" ]; then \
            install -Dm0644 "$$icon" /usr/share/icons/hicolor/scalable/apps/chronomancer-"$$basename"; \
        fi \
    done
```

This approach:
- ✅ Icons get proper theme adaptation via `Icon` widget
- ✅ Namespaced with `chronomancer-` prefix (no conflicts)
- ✅ Automatically installed during `just install`
- ✅ Works with COSMIC's accent color system
- ✅ No code changes needed for new icons
