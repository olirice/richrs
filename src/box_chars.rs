//! Box drawing characters for borders and tables.
//!
//! This module provides various box-drawing character sets for creating
//! borders around panels and tables.

use serde::{Deserialize, Serialize};

/// A set of box-drawing characters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BoxChars {
    /// Top-left corner.
    pub top_left: char,
    /// Top-right corner.
    pub top_right: char,
    /// Bottom-left corner.
    pub bottom_left: char,
    /// Bottom-right corner.
    pub bottom_right: char,
    /// Horizontal line.
    pub horizontal: char,
    /// Vertical line.
    pub vertical: char,
    /// Left T-junction.
    pub vertical_right: char,
    /// Right T-junction.
    pub vertical_left: char,
    /// Top T-junction.
    pub horizontal_down: char,
    /// Bottom T-junction.
    pub horizontal_up: char,
    /// Cross junction.
    pub cross: char,
}

impl BoxChars {
    /// ASCII box characters.
    pub const ASCII: Self = Self {
        top_left: '+',
        top_right: '+',
        bottom_left: '+',
        bottom_right: '+',
        horizontal: '-',
        vertical: '|',
        vertical_right: '+',
        vertical_left: '+',
        horizontal_down: '+',
        horizontal_up: '+',
        cross: '+',
    };

    /// Square box characters (Unicode).
    pub const SQUARE: Self = Self {
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
        horizontal: '─',
        vertical: '│',
        vertical_right: '├',
        vertical_left: '┤',
        horizontal_down: '┬',
        horizontal_up: '┴',
        cross: '┼',
    };

    /// Rounded box characters (Unicode).
    pub const ROUNDED: Self = Self {
        top_left: '╭',
        top_right: '╮',
        bottom_left: '╰',
        bottom_right: '╯',
        horizontal: '─',
        vertical: '│',
        vertical_right: '├',
        vertical_left: '┤',
        horizontal_down: '┬',
        horizontal_up: '┴',
        cross: '┼',
    };

    /// Heavy/bold box characters (Unicode).
    pub const HEAVY: Self = Self {
        top_left: '┏',
        top_right: '┓',
        bottom_left: '┗',
        bottom_right: '┛',
        horizontal: '━',
        vertical: '┃',
        vertical_right: '┣',
        vertical_left: '┫',
        horizontal_down: '┳',
        horizontal_up: '┻',
        cross: '╋',
    };

    /// Double-line box characters (Unicode).
    pub const DOUBLE: Self = Self {
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
        horizontal: '═',
        vertical: '║',
        vertical_right: '╠',
        vertical_left: '╣',
        horizontal_down: '╦',
        horizontal_up: '╩',
        cross: '╬',
    };

    /// Minimal box (just spaces for corners).
    pub const MINIMAL: Self = Self {
        top_left: ' ',
        top_right: ' ',
        bottom_left: ' ',
        bottom_right: ' ',
        horizontal: '─',
        vertical: ' ',
        vertical_right: ' ',
        vertical_left: ' ',
        horizontal_down: '─',
        horizontal_up: '─',
        cross: '─',
    };

    /// No box (all spaces).
    pub const NONE: Self = Self {
        top_left: ' ',
        top_right: ' ',
        bottom_left: ' ',
        bottom_right: ' ',
        horizontal: ' ',
        vertical: ' ',
        vertical_right: ' ',
        vertical_left: ' ',
        horizontal_down: ' ',
        horizontal_up: ' ',
        cross: ' ',
    };

    /// Simple box with simple corners.
    pub const SIMPLE: Self = Self {
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
        horizontal: '─',
        vertical: '│',
        vertical_right: '│',
        vertical_left: '│',
        horizontal_down: '─',
        horizontal_up: '─',
        cross: '─',
    };

    /// Creates a custom box character set.
    #[inline]
    #[must_use]
    pub const fn custom(
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
        horizontal: char,
        vertical: char,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            horizontal,
            vertical,
            vertical_right: vertical,
            vertical_left: vertical,
            horizontal_down: horizontal,
            horizontal_up: horizontal,
            cross: horizontal,
        }
    }

    /// Returns the top border string for a given width.
    #[must_use]
    pub fn top_border(&self, width: usize) -> String {
        let inner_width = width.saturating_sub(2);
        format!(
            "{}{}{}",
            self.top_left,
            self.horizontal.to_string().repeat(inner_width),
            self.top_right
        )
    }

    /// Returns the bottom border string for a given width.
    #[must_use]
    pub fn bottom_border(&self, width: usize) -> String {
        let inner_width = width.saturating_sub(2);
        format!(
            "{}{}{}",
            self.bottom_left,
            self.horizontal.to_string().repeat(inner_width),
            self.bottom_right
        )
    }

    /// Returns a horizontal divider with T-junctions.
    #[must_use]
    pub fn row_divider(&self, width: usize) -> String {
        let inner_width = width.saturating_sub(2);
        format!(
            "{}{}{}",
            self.vertical_right,
            self.horizontal.to_string().repeat(inner_width),
            self.vertical_left
        )
    }
}

impl Default for BoxChars {
    #[inline]
    fn default() -> Self {
        Self::ROUNDED
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_box() {
        let b = BoxChars::ASCII;
        assert_eq!(b.top_left, '+');
        assert_eq!(b.horizontal, '-');
        assert_eq!(b.vertical, '|');
    }

    #[test]
    fn test_rounded_box() {
        let b = BoxChars::ROUNDED;
        assert_eq!(b.top_left, '╭');
        assert_eq!(b.top_right, '╮');
    }

    #[test]
    fn test_top_border() {
        let b = BoxChars::ASCII;
        let border = b.top_border(10);
        assert_eq!(border, "+--------+");
    }

    #[test]
    fn test_bottom_border() {
        let b = BoxChars::ASCII;
        let border = b.bottom_border(10);
        assert_eq!(border, "+--------+");
    }

    #[test]
    fn test_row_divider() {
        let b = BoxChars::SQUARE;
        let divider = b.row_divider(10);
        assert_eq!(divider, "├────────┤");
    }

    #[test]
    fn test_default() {
        let b = BoxChars::default();
        assert_eq!(b.top_left, '╭');
    }

    #[test]
    fn test_custom() {
        let b = BoxChars::custom('*', '*', '*', '*', '=', '!');
        assert_eq!(b.top_left, '*');
        assert_eq!(b.horizontal, '=');
        assert_eq!(b.vertical, '!');
    }
}
