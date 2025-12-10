//! Measurement utilities for calculating renderable dimensions.
//!
//! This module provides tools for measuring the width of text and
//! determining optimal dimensions for renderables.

use crate::errors::Result;
use unicode_width::UnicodeWidthStr;

/// Represents the measured dimensions of a renderable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Measurement {
    /// Minimum width required to render without overflow.
    pub minimum: usize,
    /// Maximum/preferred width.
    pub maximum: usize,
}

impl Measurement {
    /// Creates a new measurement with the given minimum and maximum.
    #[inline]
    #[must_use]
    pub const fn new(minimum: usize, maximum: usize) -> Self {
        Self { minimum, maximum }
    }

    /// Creates a measurement where minimum equals maximum.
    #[inline]
    #[must_use]
    pub const fn fixed(width: usize) -> Self {
        Self {
            minimum: width,
            maximum: width,
        }
    }

    /// Returns the span (difference between maximum and minimum).
    #[inline]
    #[must_use]
    pub const fn span(&self) -> usize {
        self.maximum.saturating_sub(self.minimum)
    }

    /// Combines this measurement with another, taking the larger values.
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        let min = if self.minimum > other.minimum {
            self.minimum
        } else {
            other.minimum
        };
        let max = if self.maximum > other.maximum {
            self.maximum
        } else {
            other.maximum
        };
        Self {
            minimum: min,
            maximum: max,
        }
    }

    /// Clamps the measurement to a maximum width.
    #[inline]
    #[must_use]
    pub const fn clamp_max(self, max_width: usize) -> Self {
        let max = if self.maximum > max_width {
            max_width
        } else {
            self.maximum
        };
        let min = if self.minimum > max {
            max
        } else {
            self.minimum
        };
        Self {
            minimum: min,
            maximum: max,
        }
    }

    /// Expands minimum to the given value if it's smaller.
    #[inline]
    #[must_use]
    pub const fn expand_min(self, min_width: usize) -> Self {
        let min = if self.minimum < min_width {
            min_width
        } else {
            self.minimum
        };
        let max = if self.maximum < min {
            min
        } else {
            self.maximum
        };
        Self {
            minimum: min,
            maximum: max,
        }
    }
}

/// Measures the cell width of a string.
///
/// This takes into account Unicode width (e.g., wide CJK characters).
#[inline]
#[must_use]
pub fn cell_len(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Measures the maximum line width in a multi-line string.
#[must_use]
pub fn max_line_width(s: &str) -> usize {
    s.lines().map(cell_len).max().unwrap_or(0)
}

/// Measures the minimum width needed to render text.
///
/// This is the width of the longest word (space-separated).
#[must_use]
pub fn min_width(s: &str) -> usize {
    s.split_whitespace().map(cell_len).max().unwrap_or(0)
}

/// Measures text and returns both minimum and maximum widths.
#[must_use]
pub fn measure_text(s: &str) -> Measurement {
    Measurement {
        minimum: min_width(s),
        maximum: max_line_width(s),
    }
}

/// Options for measuring renderables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct MeasureOptions {
    /// Maximum width available for rendering.
    pub max_width: usize,
    /// Whether to consider ANSI escape codes.
    pub strip_ansi: bool,
}

impl MeasureOptions {
    /// Creates new measure options with the given maximum width.
    #[inline]
    #[must_use]
    pub const fn new(max_width: usize) -> Self {
        Self {
            max_width,
            strip_ansi: true,
        }
    }
}

impl Default for MeasureOptions {
    #[inline]
    fn default() -> Self {
        Self {
            max_width: 80,
            strip_ansi: true,
        }
    }
}

/// Trait for types that can be measured.
pub trait Measurable {
    /// Returns the measurement for this object.
    ///
    /// # Errors
    ///
    /// Returns an error if measurement fails.
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement>;
}

/// Strips ANSI escape codes from a string.
#[must_use]
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_escape = false;

    for ch in s.chars() {
        if in_escape {
            if ch.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if ch == '\x1b' {
            in_escape = true;
        } else {
            result.push(ch);
        }
    }

    result
}

/// Measures the cell width of a string, optionally stripping ANSI codes.
#[must_use]
pub fn cell_len_with_options(s: &str, strip: bool) -> usize {
    if strip {
        cell_len(&strip_ansi(s))
    } else {
        cell_len(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_new() {
        let m = Measurement::new(5, 10);
        assert_eq!(m.minimum, 5);
        assert_eq!(m.maximum, 10);
    }

    #[test]
    fn test_measurement_fixed() {
        let m = Measurement::fixed(7);
        assert_eq!(m.minimum, 7);
        assert_eq!(m.maximum, 7);
    }

    #[test]
    fn test_measurement_span() {
        let m = Measurement::new(5, 10);
        assert_eq!(m.span(), 5);
    }

    #[test]
    fn test_measurement_union() {
        let m1 = Measurement::new(5, 10);
        let m2 = Measurement::new(3, 15);
        let union = m1.union(m2);
        assert_eq!(union.minimum, 5);
        assert_eq!(union.maximum, 15);
    }

    #[test]
    fn test_measurement_clamp_max() {
        let m = Measurement::new(5, 20);
        let clamped = m.clamp_max(10);
        assert_eq!(clamped.minimum, 5);
        assert_eq!(clamped.maximum, 10);

        // Min is also clamped if needed
        let m = Measurement::new(15, 20);
        let clamped = m.clamp_max(10);
        assert_eq!(clamped.minimum, 10);
        assert_eq!(clamped.maximum, 10);
    }

    #[test]
    fn test_measurement_expand_min() {
        let m = Measurement::new(5, 20);
        let expanded = m.expand_min(10);
        assert_eq!(expanded.minimum, 10);
        assert_eq!(expanded.maximum, 20);

        // Max is also expanded if needed
        let m = Measurement::new(5, 8);
        let expanded = m.expand_min(10);
        assert_eq!(expanded.minimum, 10);
        assert_eq!(expanded.maximum, 10);
    }

    #[test]
    fn test_cell_len() {
        assert_eq!(cell_len("hello"), 5);
        assert_eq!(cell_len(""), 0);
        // Wide characters (CJK)
        assert_eq!(cell_len("日本語"), 6);
        // Mixed
        assert_eq!(cell_len("hello日本"), 9);
    }

    #[test]
    fn test_max_line_width() {
        assert_eq!(max_line_width("hello\nworld"), 5);
        assert_eq!(max_line_width("short\nlonger line"), 11);
        assert_eq!(max_line_width("single line"), 11);
        assert_eq!(max_line_width(""), 0);
    }

    #[test]
    fn test_min_width() {
        assert_eq!(min_width("hello world"), 5);
        assert_eq!(min_width("supercalifragilisticexpialidocious"), 34);
        assert_eq!(min_width("a b c"), 1);
        assert_eq!(min_width(""), 0);
    }

    #[test]
    fn test_measure_text() {
        let m = measure_text("hello world");
        assert_eq!(m.minimum, 5); // "hello" or "world"
        assert_eq!(m.maximum, 11); // whole line

        let m = measure_text("short\nmuch longer line");
        assert_eq!(m.minimum, 6); // "longer"
        assert_eq!(m.maximum, 16); // "much longer line"
    }

    #[test]
    fn test_strip_ansi() {
        assert_eq!(strip_ansi("hello"), "hello");
        assert_eq!(strip_ansi("\x1b[31mhello\x1b[0m"), "hello");
        assert_eq!(
            strip_ansi("\x1b[1;31;40mcolored text\x1b[0m"),
            "colored text"
        );
    }

    #[test]
    fn test_cell_len_with_options() {
        assert_eq!(cell_len_with_options("hello", false), 5);
        assert_eq!(cell_len_with_options("hello", true), 5);
        assert_eq!(cell_len_with_options("\x1b[31mhello\x1b[0m", true), 5);
        // Without stripping, ANSI codes add to length (though they're not visible)
        assert!(cell_len_with_options("\x1b[31mhello\x1b[0m", false) > 5);
    }

    #[test]
    fn test_measure_options_default() {
        let opts = MeasureOptions::default();
        assert_eq!(opts.max_width, 80);
        assert!(opts.strip_ansi);
    }

    #[test]
    fn test_measure_options_new() {
        let opts = MeasureOptions::new(120);
        assert_eq!(opts.max_width, 120);
        assert!(opts.strip_ansi);
    }
}
