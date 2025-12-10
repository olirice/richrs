//! Padding component for adding space around content.
//!
//! Padding adds configurable space around renderable content.

use crate::errors::Result;
use crate::measure::{Measurable, MeasureOptions, Measurement};
use crate::segment::{Segment, Segments};
use crate::text::Text;
use serde::{Deserialize, Serialize};

/// Padding around content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PaddingDimensions {
    /// Top padding (lines).
    pub top: usize,
    /// Right padding (characters).
    pub right: usize,
    /// Bottom padding (lines).
    pub bottom: usize,
    /// Left padding (characters).
    pub left: usize,
}

impl PaddingDimensions {
    /// Creates padding with all sides equal.
    #[inline]
    #[must_use]
    pub const fn all(padding: usize) -> Self {
        Self {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        }
    }

    /// Creates padding with horizontal and vertical values.
    #[inline]
    #[must_use]
    pub const fn symmetric(horizontal: usize, vertical: usize) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Creates padding with individual values.
    #[inline]
    #[must_use]
    pub const fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Returns the total horizontal padding.
    #[inline]
    #[must_use]
    pub const fn horizontal(&self) -> usize {
        self.left.saturating_add(self.right)
    }

    /// Returns the total vertical padding.
    #[inline]
    #[must_use]
    pub const fn vertical(&self) -> usize {
        self.top.saturating_add(self.bottom)
    }
}

impl From<usize> for PaddingDimensions {
    fn from(value: usize) -> Self {
        Self::all(value)
    }
}

impl From<(usize, usize)> for PaddingDimensions {
    fn from((horizontal, vertical): (usize, usize)) -> Self {
        Self::symmetric(horizontal, vertical)
    }
}

impl From<(usize, usize, usize, usize)> for PaddingDimensions {
    fn from((top, right, bottom, left): (usize, usize, usize, usize)) -> Self {
        Self::new(top, right, bottom, left)
    }
}

/// A wrapper that adds padding around content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Padding {
    /// The content to pad.
    content: Text,
    /// Padding dimensions.
    dimensions: PaddingDimensions,
    /// Padding character.
    pad_char: char,
}

impl Padding {
    /// Creates a new padding wrapper.
    #[inline]
    #[must_use]
    pub fn new(content: impl Into<Text>, padding: impl Into<PaddingDimensions>) -> Self {
        Self {
            content: content.into(),
            dimensions: padding.into(),
            pad_char: ' ',
        }
    }

    /// Sets the padding character.
    #[inline]
    #[must_use]
    pub const fn pad_char(mut self, ch: char) -> Self {
        self.pad_char = ch;
        self
    }

    /// Returns the padding dimensions.
    #[inline]
    #[must_use]
    pub const fn dimensions(&self) -> &PaddingDimensions {
        &self.dimensions
    }

    /// Renders the padded content.
    #[must_use]
    pub fn render(&self, max_width: usize) -> Segments {
        let mut segments = Segments::new();
        let content_segments = self.content.to_segments();
        let lines = content_segments.split_lines();

        let content_width = max_width.saturating_sub(self.dimensions.horizontal());

        // Top padding
        for _ in 0..self.dimensions.top {
            segments.push(Segment::new(self.pad_char.to_string().repeat(max_width)));
            segments.push(Segment::newline());
        }

        // Content with left/right padding
        for line in lines {
            // Left padding
            if self.dimensions.left > 0 {
                segments.push(Segment::new(
                    self.pad_char.to_string().repeat(self.dimensions.left),
                ));
            }

            // Content
            let line_width = line.cell_length();
            segments.extend(line);

            // Right padding to fill width
            let remaining = content_width.saturating_sub(line_width);
            if remaining > 0 {
                segments.push(Segment::new(self.pad_char.to_string().repeat(remaining)));
            }

            // Right padding
            if self.dimensions.right > 0 {
                segments.push(Segment::new(
                    self.pad_char.to_string().repeat(self.dimensions.right),
                ));
            }

            segments.push(Segment::newline());
        }

        // Bottom padding
        for _ in 0..self.dimensions.bottom {
            segments.push(Segment::new(self.pad_char.to_string().repeat(max_width)));
            segments.push(Segment::newline());
        }

        segments
    }
}

impl Measurable for Padding {
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        let content_measurement = self.content.measure(options)?;

        let overhead = self.dimensions.horizontal();
        let min = content_measurement.minimum.saturating_add(overhead);
        let max = content_measurement.maximum.saturating_add(overhead);

        Ok(Measurement::new(min, max).clamp_max(options.max_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_dimensions_all() {
        let dims = PaddingDimensions::all(2);
        assert_eq!(dims.top, 2);
        assert_eq!(dims.right, 2);
        assert_eq!(dims.bottom, 2);
        assert_eq!(dims.left, 2);
    }

    #[test]
    fn test_padding_dimensions_symmetric() {
        let dims = PaddingDimensions::symmetric(2, 1);
        assert_eq!(dims.horizontal(), 4);
        assert_eq!(dims.vertical(), 2);
    }

    #[test]
    fn test_padding_dimensions_new() {
        let dims = PaddingDimensions::new(1, 2, 3, 4);
        assert_eq!(dims.top, 1);
        assert_eq!(dims.right, 2);
        assert_eq!(dims.bottom, 3);
        assert_eq!(dims.left, 4);
    }

    #[test]
    fn test_padding_render() {
        let padding = Padding::new("hello", PaddingDimensions::symmetric(2, 1));
        let segments = padding.render(20);
        let text = segments.plain_text();

        // Should have top padding, content, bottom padding
        assert!(text.lines().count() >= 3); // top, content, bottom
    }

    #[test]
    fn test_padding_from_usize() {
        let dims: PaddingDimensions = 3.into();
        assert_eq!(dims, PaddingDimensions::all(3));
    }

    #[test]
    fn test_padding_from_tuple() {
        let dims: PaddingDimensions = (2, 1).into();
        assert_eq!(dims, PaddingDimensions::symmetric(2, 1));

        let dims: PaddingDimensions = (1, 2, 3, 4).into();
        assert_eq!(dims, PaddingDimensions::new(1, 2, 3, 4));
    }

    #[test]
    fn test_padding_measure() {
        let padding = Padding::new("hello", PaddingDimensions::symmetric(2, 1));
        let options = MeasureOptions::new(80);
        let measurement = padding.measure(&options).ok().unwrap_or_default();
        // "hello" is 5 chars + 4 horizontal padding = 9
        assert!(measurement.minimum >= 9);
    }
}
