//! UI utilities for consistent visual design.
//!
//! This module provides utilities for sizing and spacing throughout the
//! Chronomancer UI.
//!
//! # Design Philosophy
//!
//! - **Single source of truth** - Fixed dimensions in one place
//! - **Theme integration** - Use cosmic theme values for spacing
//!
//! # Usage
//!
//! Use [`ComponentSize`] constants for fixed dimensions (icon sizes, button
//! heights, font sizes).
//!
//! Use [`Spacing`] to bundle layout values computed from the active cosmic
//! theme.
//!
//! For raw spacing primitives, use cosmic theme values directly:
//!
//! ```rust,ignore
//! use cosmic::theme;
//!
//! let spacing = theme::active().cosmic().spacing;
//! let small_gap = spacing.space_s;
//! let medium_gap = spacing.space_m;
//! ```

pub mod sizing;
pub mod spacing;

pub use sizing::ComponentSize;
pub use spacing::Spacing;
