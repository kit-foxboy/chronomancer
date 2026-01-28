//! UI utilities for consistent visual design.
//!
//! This module provides utilities for sizing throughout the Chronomancer UI.
//!
//! # Design Philosophy
//!
//! - **Single source of truth** - Fixed dimensions in one place
//! - **Theme integration** - Use cosmic theme values for spacing
//!
//! # Usage
//!
//! Use `ComponentSize` constants for fixed dimensions (icon sizes, button heights).
//!
//! For spacing and padding, use cosmic theme values directly:
//!
//! ```rust,ignore
//! use cosmic::theme;
//!
//! let spacing = theme::active().cosmic().spacing;
//! let small_gap = spacing.space_s;
//! let medium_gap = spacing.space_m;
//! ```

pub mod spacing;

pub use spacing::ComponentSize;
