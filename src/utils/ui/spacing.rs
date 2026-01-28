//! UI sizing utilities for consistent visual design.
//!
//! This module provides constants for component sizing throughout the
//! Chronomancer UI.
//!
//! # Design Philosophy
//!
//! Fixed dimensions for UI components that need consistent sizing across the
//! application. These values are chosen to work well with the COSMIC design
//! system. Magic numbers are illegal in themes and you will be purged with a thunder hammer if you use them you filthy FILTHY heretic!
//!
//! # Usage
//!
//! Use `ComponentSize` constants for fixed dimensions (icon sizes, button heights).
//! For spacing and padding, use cosmic theme values directly:
//!
//! ```rust,ignore
//! use cosmic::theme;
//!
//! let spacing = theme::active().cosmic().spacing;
//! let small_gap = spacing.space_s;
//! let medium_gap = spacing.space_m;
//! ```

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
    ///     icon::from_name("system-suspend-symbolic")
    ///     .size(ComponentSize::ICON_SIZE)
    ///     .icon()
    ///     .into()
    /// # }
    /// ```
    pub const ICON_SIZE: u16 = 36;

    /// Text sizes for labels and inputs
    ///
    /// Use text sizes here so the app UI can be adjusted on the fly without having
    /// to fiddle with components.
    ///
    /// # Examples
    ///```rust,no_run
    /// use chronomancer::utils::ui::ComponentSize;
    /// use cosmic::widget::TextInput;
    ///
    /// # #[derive(Clone)] enum Message {}
    /// # fn example() -> cosmic::Element<'static, Message> {
    ///     TextInput::new(&self.placeholder_text, &self.input_value)
    ///     .size(ComponentSize::FONT_SIZE_DEFAULT);
    /// # }
    /// ```
    ///
    #[allow(dead_code)]
    pub const FONT_SIZE_DEFAULT: u16 = 14;
    #[allow(dead_code)]
    pub const FONT_SIZE_SMALL: u16 = 12;
    #[allow(dead_code)]
    pub const HEADER_FONT_SIZE: u16 = 16;
}
