//! Shared types for component configuration.
//!
//! This module defines common enums used across components for behavioral
//! context and layout constraints.

/// The context in which a component is being used.
///
/// This determines behavioral patterns like navigation, button styles,
/// and interaction patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    /// Full GUI application context.
    ///
    /// Characteristics:
    /// - Screen navigation available
    /// - Icon + text buttons available
    /// - Modal dialogs for confirmations
    /// - Can adapt to window size
    /// - Forms can be separate screens
    App,

    /// Panel applet context.
    ///
    /// Characteristics:
    /// - Inline interactions (no screen navigation)
    /// - Icon-only buttons by default
    /// - Popup confirmations
    /// - Always space-constrained
    /// - Forms appear inline or in popups
    Applet,
}

impl Default for Context {
    /// Defaults to `App` since that's the primary use case.
    fn default() -> Self {
        Self::App
    }
}

/// Visual layout density based on available space.
///
/// This determines spacing, padding, and visual density independent
/// of the behavioral context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    /// Tight spacing and minimal padding.
    ///
    /// Use when:
    /// - Space is limited (applets, small windows)
    /// - Need to fit more content in less space
    /// - Mobile-like constraints
    Compact,

    /// Moderate spacing and padding
    ///
    /// Default layout for most apps
    /// When in doubt, start here
    Comfortable,

    /// Generous spacing and padding.
    ///
    /// Use when:
    /// - Ample space available (large windows)
    /// - Desktop layouts with room to breathe
    /// - Prioritizing visual comfort over density
    Spacious,
}

impl Default for Layout {
    /// Defaults to `Comfortable` for space efficiency and visual comfort balance.
    fn default() -> Self {
        Self::Comfortable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_default() {
        assert_eq!(Context::default(), Context::App);
    }

    #[test]
    fn test_layout_default() {
        assert_eq!(Layout::default(), Layout::Comfortable);
    }
}
