//! UI spacing and sizing utilities for consistent visual design.
//!
//! This module provides semantic constants and helpers for spacing, sizing, and
//! padding throughout the Chronomancer UI. It wraps COSMIC theme values with
//! friendly names to reduce verbosity and ensure consistency.
//!
//! # Design Philosophy
//!
//! Rather than using magic numbers or importing theme values directly everywhere,
//! this module provides:
//! - **Semantic names** - `Gaps::s()` instead of `theme::active().cosmic().spacing.space_s`
//! - **Single source of truth** - All fixed dimensions in one place
//! - **Theme integration** - Values derive from COSMIC theme when possible
//!
//! # Usage
//!
//! Use `ComponentSize` constants for fixed dimensions (icon sizes, button heights).
//! Use `Gaps::xs()` and `Gaps::s()` for theme-aware spacing between elements.
//! Use `Padding` methods to generate padding arrays for containers.

use cosmic::{cosmic_theme::Spacing, theme};

/// Standard component sizing constants.
///
/// Provides fixed dimensions for UI components that need consistent sizing
/// across the application. These values are chosen to work well with the
/// COSMIC design system.
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::ui::ComponentSize;
///
/// // Get standard icon button height
/// let height = ComponentSize::ICON_BUTTON_HEIGHT;
/// assert_eq!(height, 48.0);
///
/// // Get standard icon size
/// let icon_size = ComponentSize::ICON_SIZE;
/// assert_eq!(icon_size, 36);
/// ```
pub struct ComponentSize;

impl ComponentSize {
    /// Standard height for icon buttons in pixels.
    ///
    /// This provides enough space for comfortable touch targets and
    /// visual balance with COSMIC's design language.
    pub const ICON_BUTTON_HEIGHT: f32 = 48.0;

    /// Standard size for icons within buttons in pixels.
    ///
    /// This size works well with the button height and provides good
    /// visual hierarchy.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::utils::ui::ComponentSize;
    /// use cosmic::widget::icon;
    ///
    /// # #[derive(Clone)] enum Message {}
    /// # fn example() -> cosmic::Element<'static, Message> {
    /// icon::from_name("system-suspend-symbolic")
    ///     .size(ComponentSize::ICON_SIZE)
    ///     .icon()
    ///     .into()
    /// # }
    /// ```
    pub const ICON_SIZE: u16 = 36;
}

/// Retrieves the current COSMIC theme spacing values.
///
/// Accesses the active COSMIC theme to get spacing values that respect
/// the user's theme preferences. Returns the [`Spacing`] configuration.
#[must_use]
pub fn cosmic_spacing() -> Spacing {
    theme::active().cosmic().spacing
}

/// Semantic spacing helpers based on COSMIC theme values.
///
/// Provides friendly names for spacing values from the COSMIC theme. Methods
/// return spacing values that automatically adapt to the user's theme preferences.
///
/// - `xs()` - Extra small spacing for tightly related items
/// - `s()` - Small spacing for grouping related elements
pub struct Gaps;

impl Gaps {
    /// Extra small gap for tightly related items within a group.
    ///
    /// Use for items that are closely related but need some visual separation,
    /// such as label/value pairs or closely related controls.
    #[must_use]
    pub fn xs() -> u16 {
        cosmic_spacing().space_xs
    }

    /// Small gap for grouping related elements.
    ///
    /// Use for standard spacing between elements in the same logical group,
    /// such as form fields or menu items.
    #[must_use]
    pub fn s() -> u16 {
        cosmic_spacing().space_s
    }
}

/// Padding helpers for consistent container padding.
///
/// Provides methods for generating padding arrays in the format expected by
/// COSMIC widgets: `[top, right, bottom, left]`. All methods return `[u16; 4]`
/// arrays where indices represent: `[0]` top, `[1]` right, `[2]` bottom, `[3]` left.
pub struct Padding;

impl Padding {
    /// Standard padding for most components.
    ///
    /// Applies equal padding on all four sides using the extra small spacing
    /// value from the COSMIC theme.
    ///
    /// # Returns
    ///
    /// Padding array: `[xs, xs, xs, xs]`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::utils::ui::Padding;
    /// use cosmic::widget::container;
    ///
    /// # #[derive(Clone)] enum Message {}
    /// # fn example() -> cosmic::Element<'static, Message> {
    /// container("content")
    ///     .padding(Padding::standard())
    ///     .into()
    /// # }
    /// ```
    #[must_use]
    #[allow(dead_code)]
    pub fn standard() -> [u16; 4] {
        let xs = Gaps::xs();
        [xs, xs, xs, xs]
    }

    /// Horizontal padding only (left and right sides).
    ///
    /// Useful for containers that should have padding on the sides but
    /// extend to the full height vertically.
    ///
    /// # Arguments
    ///
    /// - `amount` - Padding amount in pixels for left and right sides
    ///
    /// # Returns
    ///
    /// Padding array: `[0, amount, 0, amount]`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::utils::ui::Padding;
    /// use cosmic::widget::container;
    ///
    /// # #[derive(Clone)] enum Message {}
    /// # fn example() -> cosmic::Element<'static, Message> {
    /// // 24px padding on left and right, none on top/bottom
    /// container("content")
    ///     .padding(Padding::horizontal(24))
    ///     .into()
    /// # }
    /// ```
    #[must_use]
    pub fn horizontal(amount: u16) -> [u16; 4] {
        [0, amount, 0, amount]
    }

    /// Standard padding without bottom padding.
    ///
    /// Applies extra small spacing on top, left, and right sides, but no
    /// bottom padding. Useful for popup containers that should extend to
    /// the bottom edge.
    ///
    /// # Returns
    ///
    /// Padding array: `[xs, xs, 0, xs]`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use chronomancer::utils::ui::Padding;
    /// use cosmic::widget::container;
    ///
    /// # #[derive(Clone)] enum Message {}
    /// # fn example() -> cosmic::Element<'static, Message> {
    /// // For popup windows that extend to bottom edge
    /// container("popup content")
    ///     .padding(Padding::no_bottom())
    ///     .into()
    /// # }
    /// ```
    #[must_use]
    pub fn no_bottom() -> [u16; 4] {
        let xs = Gaps::xs();
        [xs, xs, 0, xs]
    }
}
