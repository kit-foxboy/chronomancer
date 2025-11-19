// UI spacing and sizing standards for Chronomancer
//
// This module provides consistent spacing and sizing values across the application,
// leveraging the COSMIC theme system while providing semantic names for component sizes.
// Bascially, I was getting annoyed with the verbosity of theme imports and wanted a cleaner way.
// It's also good to have any fixed numbers in one place in any graphical application

use cosmic::{cosmic_theme::Spacing, theme};

/// Standard component sizing
pub struct ComponentSize;

impl ComponentSize {
    /// Standard height for icon buttons
    pub const ICON_BUTTON_HEIGHT: f32 = 48.0;

    /// Standard icon size within buttons
    pub const ICON_SIZE: u16 = 36;
}

/// Get the current COSMIC theme spacing values
pub fn cosmic_spacing() -> Spacing {
    theme::active().cosmic().spacing
}

/// Semantic spacing helpers based on COSMIC theme
pub struct Gaps;

impl Gaps {
    /// Extra small gap - use for related items within a group
    pub fn xs() -> u16 {
        cosmic_spacing().space_xs
    }

    /// Small gap - use for grouping related elements
    pub fn s() -> u16 {
        cosmic_spacing().space_s
    }
}

/// Padding helpers for consistent container padding
pub struct Padding;

impl Padding {
    /// Standard padding for most components
    pub fn standard() -> [u16; 4] {
        let xs = Gaps::xs();
        [xs, xs, xs, xs]
    }

    /// Horizontal padding only
    pub fn horizontal(amount: u16) -> [u16; 4] {
        [0, amount, 0, amount]
    }
}
