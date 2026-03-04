//! UI spacing utilities for consistent visual design.
//!
//! This module provides the [`Spacing`] struct for grouping layout-related
//! values throughout the Chronomancer UI.
//!
//! # Design Philosophy
//!
//! Rather than passing spacing, padding, and text size around as separate
//! values, [`Spacing`] bundles them together so components can compute their
//! layout values once and use them consistently.
//!
//! For the underlying spacing primitives, use cosmic theme values directly:
//!
//! ```rust,ignore
//! use cosmic::theme;
//!
//! let spacing = theme::active().cosmic().spacing;
//! let small_gap = spacing.space_s;
//! let medium_gap = spacing.space_m;
//! ```

/// Bundled layout values for a component at a given density.
///
/// `Spacing` groups the spacing, padding, and text size that a component
/// needs to render itself consistently. Components typically compute one
/// instance of this struct from the active cosmic theme and then apply
/// all three values in their `view` method.
///
/// # Examples
///
/// ```rust
/// use chronomancer::utils::ui::spacing::Spacing;
///
/// let layout = Spacing {
///     gap: 4,
///     padding: [4, 8, 4, 8],
///     text_size: 14,
/// };
///
/// assert_eq!(layout.gap, 4);
/// assert_eq!(layout.padding, [4, 8, 4, 8]);
/// assert_eq!(layout.text_size, 14);
/// ```
pub struct Spacing {
    /// Gap between child elements in a row or column.
    pub gap: u16,
    /// Padding applied to the component container as `[top, right, bottom, left]`.
    pub padding: [u16; 4],
    /// Font size for the primary text in the component.
    pub text_size: u16,
}
