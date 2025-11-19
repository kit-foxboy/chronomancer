// UI spacing and sizing standards for Chronomancer
//
// This module provides consistent spacing and sizing values across the application,
// leveraging the COSMIC theme system while providing semantic names for component sizes.
// Bascially, I was getting annoyed with the verbosity of theme imports and wanted a cleaner way.
// It's also good to have any fixed numbers in one place in any graphical application

use cosmic::{cosmic_theme::Spacing, iced::Length, theme};

/// Standard component sizing
pub struct ComponentSize;

impl ComponentSize {
    /// Standard height for icon buttons
    pub const ICON_BUTTON_HEIGHT: f32 = 48.0;

    /// Standard width for icon buttons (matches height for square buttons)
    pub const ICON_BUTTON_WIDTH: f32 = 48.0;

    /// Standard icon size within buttons
    pub const ICON_SIZE: u16 = 36;

    /// Quick timer button height
    pub const QUICK_TIMER_BUTTON_HEIGHT: f32 = 40.0;

    /// Minimum width for text input fields
    pub const INPUT_MIN_WIDTH: f32 = 60.0;

    /// Standard text size for headers
    pub const HEADER_TEXT_SIZE: f32 = 24.0;

    /// Standard text size for body content
    pub const BODY_TEXT_SIZE: f32 = 14.0;
}

/// Get the current COSMIC theme spacing values
pub fn cosmic_spacing() -> Spacing {
    theme::active().cosmic().spacing
}

/// Semantic spacing helpers based on COSMIC theme
pub struct Gaps;

impl Gaps {
    /// Extra extra small gap - use for very tight spacing
    pub fn xxs() -> u16 {
        cosmic_spacing().space_xxs
    }

    /// Extra small gap - use for related items within a group
    pub fn xs() -> u16 {
        cosmic_spacing().space_xs
    }

    /// Small gap - use for grouping related elements
    pub fn s() -> u16 {
        cosmic_spacing().space_s
    }

    /// Medium gap - use for separating distinct groups
    pub fn m() -> u16 {
        cosmic_spacing().space_m
    }

    /// Large gap - use for major sections
    pub fn l() -> u16 {
        cosmic_spacing().space_l
    }

    /// Extra large gap - use for top-level sections
    pub fn xl() -> u16 {
        cosmic_spacing().space_xl
    }

    /// Extra extra large gap - use sparingly for major visual breaks
    pub fn xxl() -> u16 {
        cosmic_spacing().space_xxl
    }
}

/// Padding helpers for consistent container padding
pub struct Padding;

impl Padding {
    /// No padding
    pub fn none() -> [u16; 4] {
        [0, 0, 0, 0]
    }

    /// Minimal padding for tight layouts
    pub fn tight() -> [u16; 4] {
        let xxs = Gaps::xxs();
        [xxs, xxs, xxs, xxs]
    }

    /// Standard padding for most components
    pub fn standard() -> [u16; 4] {
        let xs = Gaps::xs();
        [xs, xs, xs, xs]
    }

    /// Comfortable padding for main content areas
    pub fn comfortable() -> [u16; 4] {
        let s = Gaps::s();
        [s, s, s, s]
    }

    /// Spacious padding for top-level containers
    pub fn spacious() -> [u16; 4] {
        let m = Gaps::m();
        [m, m, m, m]
    }

    /// Horizontal padding only
    pub fn horizontal(amount: u16) -> [u16; 4] {
        [0, amount, 0, amount]
    }

    /// Vertical padding only
    pub fn vertical(amount: u16) -> [u16; 4] {
        [amount, 0, amount, 0]
    }

    /// Custom padding [top, right, bottom, left]
    pub fn custom(top: u16, right: u16, bottom: u16, left: u16) -> [u16; 4] {
        [top, right, bottom, left]
    }
}

/// Responsive sizing based on container dimensions
pub struct ResponsiveSize;

impl ResponsiveSize {
    /// Calculate responsive icon size based on available width
    /// For applet: typically 200-400px wide
    pub fn icon_for_width(width: f32) -> u16 {
        match width as u32 {
            0..=200 => 24,   // Compact mode
            201..=300 => 32, // Standard mode
            301..=400 => 36, // Comfortable mode
            _ => 40,         // Spacious mode
        }
    }

    /// Calculate responsive button height based on available space
    pub fn button_height_for_width(width: f32) -> f32 {
        match width as u32 {
            0..=200 => 36.0,   // Compact mode
            201..=300 => 44.0, // Standard mode
            301..=400 => 48.0, // Comfortable mode
            _ => 52.0,         // Spacious mode
        }
    }

    /// Calculate responsive spacing based on available width
    pub fn gap_for_width(width: f32) -> u16 {
        match width as u32 {
            0..=200 => Gaps::xxs(),
            201..=300 => Gaps::xs(),
            301..=400 => Gaps::s(),
            _ => Gaps::m(),
        }
    }
}
