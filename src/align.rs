//! Text alignment utilities.
//!
//! This module provides tools for aligning text within a given width.

use crate::errors::Result;
use crate::measure::{Measurable, MeasureOptions, Measurement, cell_len};
use crate::segment::{Segment, Segments};
use crate::text::{Justify, Text};
use serde::{Deserialize, Serialize};

/// Vertical alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VerticalAlign {
    /// Align to the top.
    #[default]
    Top,
    /// Align to the middle.
    Middle,
    /// Align to the bottom.
    Bottom,
}

/// A wrapper that aligns its content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Align {
    /// The content to align.
    content: Text,
    /// Horizontal alignment.
    horizontal: Justify,
    /// Vertical alignment.
    vertical: VerticalAlign,
    /// Optional fixed width.
    width: Option<usize>,
    /// Optional fixed height.
    height: Option<usize>,
    /// Padding character.
    pad_char: char,
}

impl Align {
    /// Creates a new alignment wrapper.
    #[inline]
    #[must_use]
    pub fn new(content: impl Into<Text>) -> Self {
        Self {
            content: content.into(),
            horizontal: Justify::Left,
            vertical: VerticalAlign::Top,
            width: None,
            height: None,
            pad_char: ' ',
        }
    }

    /// Creates left-aligned content.
    #[inline]
    #[must_use]
    pub fn left(content: impl Into<Text>) -> Self {
        Self::new(content).horizontal(Justify::Left)
    }

    /// Creates right-aligned content.
    #[inline]
    #[must_use]
    pub fn right(content: impl Into<Text>) -> Self {
        Self::new(content).horizontal(Justify::Right)
    }

    /// Creates center-aligned content.
    #[inline]
    #[must_use]
    pub fn center(content: impl Into<Text>) -> Self {
        Self::new(content).horizontal(Justify::Center)
    }

    /// Sets the horizontal alignment.
    #[inline]
    #[must_use]
    pub const fn horizontal(mut self, justify: Justify) -> Self {
        self.horizontal = justify;
        self
    }

    /// Sets the vertical alignment.
    #[inline]
    #[must_use]
    pub const fn vertical(mut self, align: VerticalAlign) -> Self {
        self.vertical = align;
        self
    }

    /// Sets the width.
    #[inline]
    #[must_use]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height.
    #[inline]
    #[must_use]
    pub const fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the padding character.
    #[inline]
    #[must_use]
    pub const fn pad_char(mut self, ch: char) -> Self {
        self.pad_char = ch;
        self
    }

    /// Renders the aligned content.
    #[must_use]
    pub fn render(&self, max_width: usize) -> Segments {
        let target_width = self.width.unwrap_or(max_width);
        let content_segments = self.content.to_segments();
        let content_width = content_segments.cell_length();

        let mut segments = Segments::new();

        // Calculate padding
        let total_padding = target_width.saturating_sub(content_width);
        let (left_pad, right_pad) = match self.horizontal {
            Justify::Left | Justify::Default => (0, total_padding),
            Justify::Right => (total_padding, 0),
            Justify::Center => {
                let half = total_padding.checked_div(2).unwrap_or(0);
                (half, total_padding.saturating_sub(half))
            }
            Justify::Full => (0, total_padding), // Full justification is more complex
        };

        // Add left padding
        if left_pad > 0 {
            segments.push(Segment::new(self.pad_char.to_string().repeat(left_pad)));
        }

        // Add content
        segments.extend(content_segments);

        // Add right padding
        if right_pad > 0 {
            segments.push(Segment::new(self.pad_char.to_string().repeat(right_pad)));
        }

        segments
    }
}

impl From<&str> for Align {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Align {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl Measurable for Align {
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        let content_measurement = self.content.measure(options)?;

        let width = self
            .width
            .map(Measurement::fixed)
            .unwrap_or(content_measurement);

        Ok(width.clamp_max(options.max_width))
    }
}

/// Pads a string to the left to reach the given width.
#[must_use]
pub fn pad_left(s: &str, width: usize, pad_char: char) -> String {
    let current_width = cell_len(s);
    if current_width >= width {
        return s.to_owned();
    }

    let padding = width.saturating_sub(current_width);
    format!("{}{}", pad_char.to_string().repeat(padding), s)
}

/// Pads a string to the right to reach the given width.
#[must_use]
pub fn pad_right(s: &str, width: usize, pad_char: char) -> String {
    let current_width = cell_len(s);
    if current_width >= width {
        return s.to_owned();
    }

    let padding = width.saturating_sub(current_width);
    format!("{}{}", s, pad_char.to_string().repeat(padding))
}

/// Centers a string within the given width.
#[must_use]
pub fn pad_center(s: &str, width: usize, pad_char: char) -> String {
    let current_width = cell_len(s);
    if current_width >= width {
        return s.to_owned();
    }

    let total_padding = width.saturating_sub(current_width);
    let left_padding = total_padding.checked_div(2).unwrap_or(0);
    let right_padding = total_padding.saturating_sub(left_padding);

    format!(
        "{}{}{}",
        pad_char.to_string().repeat(left_padding),
        s,
        pad_char.to_string().repeat(right_padding)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_left() {
        let align = Align::left("hello");
        let segments = align.render(10);
        let text = segments.plain_text();
        assert_eq!(text, "hello     ");
    }

    #[test]
    fn test_align_right() {
        let align = Align::right("hello");
        let segments = align.render(10);
        let text = segments.plain_text();
        assert_eq!(text, "     hello");
    }

    #[test]
    fn test_align_center() {
        let align = Align::center("hi");
        let segments = align.render(10);
        let text = segments.plain_text();
        assert_eq!(text, "    hi    ");
    }

    #[test]
    fn test_align_fixed_width() {
        let align = Align::left("hello").width(8);
        let segments = align.render(100);
        let text = segments.plain_text();
        assert_eq!(text, "hello   ");
    }

    #[test]
    fn test_align_pad_char() {
        let align = Align::right("hello").pad_char('.');
        let segments = align.render(10);
        let text = segments.plain_text();
        assert_eq!(text, ".....hello");
    }

    #[test]
    fn test_pad_left() {
        assert_eq!(pad_left("hi", 5, ' '), "   hi");
        assert_eq!(pad_left("hello", 3, ' '), "hello");
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(pad_right("hi", 5, ' '), "hi   ");
        assert_eq!(pad_right("hello", 3, ' '), "hello");
    }

    #[test]
    fn test_pad_center() {
        assert_eq!(pad_center("hi", 6, ' '), "  hi  ");
        assert_eq!(pad_center("hi", 7, ' '), "  hi   ");
        assert_eq!(pad_center("hello", 3, ' '), "hello");
    }

    #[test]
    fn test_align_measure() {
        let align = Align::center("hello").width(10);
        let options = MeasureOptions::new(80);
        let measurement = align.measure(&options).ok().unwrap_or_default();
        assert_eq!(measurement.minimum, 10);
        assert_eq!(measurement.maximum, 10);
    }
}
