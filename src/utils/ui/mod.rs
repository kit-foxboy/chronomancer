//! UI utilities for consistent styling, spacing, and layout.
//!
//! This module provides utilities that help maintain consistent visual design
//! across the Chronomancer application. It includes helpers for spacing,
//! sizing, and padding that integrate with the COSMIC theme system.
//!
//! # Organization
//!
//! - [`spacing`] - Spacing, sizing, and padding utilities
//!
//! # Key Types
//!
//! - [`ComponentSize`] - Standard component dimensions (icon sizes, button heights)
//! - [`Gaps`] - Semantic spacing helpers that adapt to the COSMIC theme
//! - [`Padding`] - Padding array generators for container widgets
//!
//! # Design Philosophy
//!
//! Rather than using magic numbers or importing theme values everywhere,
//! this module provides:
//! - **Semantic names** - Clear intent in code (`Gaps::s()` instead of raw pixel values)
//! - **Theme integration** - Respects user's COSMIC theme preferences
//! - **Single source of truth** - Centralized dimension constants
//!
//! # Usage
//!
//! Use `Gaps::xs()` and `Gaps::s()` for spacing between UI elements. These return
//! theme-aware spacing values. Use `Padding::standard()`, `Padding::horizontal()`, etc.
//! to generate padding arrays for containers. Use `ComponentSize` constants for
//! fixed dimensions like icon sizes and button heights.

pub mod spacing;

pub use spacing::{ComponentSize, Gaps, Padding};
