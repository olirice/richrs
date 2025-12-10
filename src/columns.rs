//! Columns layout for multi-column display.
//!
//! The [`Columns`] struct provides a way to display multiple items
//! in a column layout that automatically wraps based on terminal width.
//!
//! # Example
//!
//! ```ignore
//! use richrs::prelude::*;
//! use richrs::columns::Columns;
//!
//! let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
//! let columns = Columns::new(items).equal(true);
//! let segments = columns.render(80);
//! ```

use crate::segment::{Segment, Segments};
use crate::text::Text;
use unicode_width::UnicodeWidthStr;

/// Alignment options for column content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ColumnAlign {
    /// Left-align content (default).
    #[default]
    Left,
    /// Center content.
    Center,
    /// Right-align content.
    Right,
}

/// A column layout for displaying multiple items.
///
/// Columns arranges items in a multi-column grid that flows
/// horizontally, wrapping to new rows as needed.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Columns {
    /// The items to display in columns.
    items: Vec<Text>,
    /// Padding between columns (left, top, right, bottom).
    padding: (usize, usize, usize, usize),
    /// Optional fixed width (otherwise uses all available width).
    width: Option<usize>,
    /// Whether to expand to fill available width.
    expand: bool,
    /// Whether all columns should have equal width.
    equal: bool,
    /// Whether to fill columns first (top to bottom) instead of rows.
    column_first: bool,
    /// Whether to render columns right to left.
    right_to_left: bool,
    /// Alignment of content within columns.
    align: ColumnAlign,
    /// Optional title for the column layout.
    title: Option<Text>,
}

impl Default for Columns {
    #[inline]
    fn default() -> Self {
        Self::new(Vec::<&str>::new())
    }
}

impl Columns {
    /// Creates a new Columns layout from an iterable of items.
    ///
    /// Items can be strings, Text objects, or any type that implements
    /// `Into<Text>`.
    #[must_use]
    #[inline]
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            padding: (0, 0, 1, 0), // Default: 1 space right padding
            width: None,
            expand: false,
            equal: false,
            column_first: false,
            right_to_left: false,
            align: ColumnAlign::Left,
            title: None,
        }
    }

    /// Adds an item to the columns.
    #[must_use]
    #[inline]
    pub fn add<T: Into<Text>>(mut self, item: T) -> Self {
        self.items.push(item.into());
        self
    }

    /// Sets the padding around each column item.
    ///
    /// Padding is specified as (left, top, right, bottom).
    #[must_use]
    #[inline]
    pub const fn padding(mut self, left: usize, top: usize, right: usize, bottom: usize) -> Self {
        self.padding = (left, top, right, bottom);
        self
    }

    /// Sets horizontal padding (left and right).
    #[must_use]
    #[inline]
    pub const fn padding_horizontal(mut self, padding: usize) -> Self {
        self.padding.0 = padding;
        self.padding.2 = padding;
        self
    }

    /// Sets vertical padding (top and bottom).
    #[must_use]
    #[inline]
    pub const fn padding_vertical(mut self, padding: usize) -> Self {
        self.padding.1 = padding;
        self.padding.3 = padding;
        self
    }

    /// Sets a fixed width for the columns layout.
    #[must_use]
    #[inline]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets whether to expand to fill available width.
    #[must_use]
    #[inline]
    pub const fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Sets whether all columns should have equal width.
    #[must_use]
    #[inline]
    pub const fn equal(mut self, equal: bool) -> Self {
        self.equal = equal;
        self
    }

    /// Sets whether to fill columns first (top to bottom) instead of rows.
    #[must_use]
    #[inline]
    pub const fn column_first(mut self, column_first: bool) -> Self {
        self.column_first = column_first;
        self
    }

    /// Sets whether to render columns right to left.
    #[must_use]
    #[inline]
    pub const fn right_to_left(mut self, rtl: bool) -> Self {
        self.right_to_left = rtl;
        self
    }

    /// Sets the alignment of content within columns.
    #[must_use]
    #[inline]
    pub const fn align(mut self, align: ColumnAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets a title for the columns layout.
    #[must_use]
    #[inline]
    pub fn title<T: Into<Text>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Gets the display width of a Text item.
    fn text_width(text: &Text) -> usize {
        text.plain().width()
    }

    /// Calculates the width of each item including padding.
    fn item_widths(&self) -> Vec<usize> {
        let pad_h = self.padding.0.saturating_add(self.padding.2);
        self.items
            .iter()
            .map(|item| Self::text_width(item).saturating_add(pad_h))
            .collect()
    }

    /// Calculates how many columns fit in the given width.
    fn calculate_columns(&self, max_width: usize) -> usize {
        if self.items.is_empty() {
            return 0;
        }

        let widths = self.item_widths();
        let max_item_width = widths.iter().copied().max().unwrap_or(1);

        if self.equal {
            // All columns same width, calculate how many fit
            max_width.checked_div(max_item_width).unwrap_or(1).max(1)
        } else {
            // Variable width columns - estimate based on average
            let total_width: usize = widths.iter().sum();
            let avg_width = total_width.checked_div(widths.len()).unwrap_or(1).max(1);
            max_width.checked_div(avg_width).unwrap_or(1).max(1)
        }
    }

    /// Renders the columns to segments.
    #[must_use]
    pub fn render(&self, max_width: usize) -> Segments {
        let mut segments = Segments::new();

        if self.items.is_empty() {
            return segments;
        }

        let render_width = self.width.unwrap_or(max_width);
        let num_columns = self.calculate_columns(render_width);
        let widths = self.item_widths();
        let pad_h = self.padding.0.saturating_add(self.padding.2);

        // Calculate column width
        let column_width = if self.equal {
            let max_item_width = widths.iter().copied().max().unwrap_or(1);
            if self.expand {
                render_width
                    .checked_div(num_columns)
                    .unwrap_or(max_item_width)
            } else {
                max_item_width
            }
        } else {
            // For variable widths, use the max as a base
            widths.iter().copied().max().unwrap_or(1)
        };

        // Organize items into rows
        let mut rows: Vec<Vec<usize>> = Vec::new();

        if self.column_first {
            // Fill columns first (top to bottom)
            let num_items = self.items.len();
            let num_rows = num_items
                .saturating_add(num_columns)
                .saturating_sub(1)
                .checked_div(num_columns)
                .unwrap_or(1);

            for row_idx in 0..num_rows {
                let mut row = Vec::new();
                for col_idx in 0..num_columns {
                    let item_idx = col_idx.saturating_mul(num_rows).saturating_add(row_idx);
                    if item_idx < num_items {
                        row.push(item_idx);
                    }
                }
                if !row.is_empty() {
                    rows.push(row);
                }
            }
        } else {
            // Fill rows first (left to right)
            for chunk in self
                .items
                .iter()
                .enumerate()
                .collect::<Vec<_>>()
                .chunks(num_columns)
            {
                rows.push(chunk.iter().map(|(idx, _)| *idx).collect());
            }
        }

        // Render each row
        for (row_idx, row) in rows.iter().enumerate() {
            // Add top padding for first row
            if row_idx == 0 {
                for _ in 0..self.padding.1 {
                    segments.push(Segment::newline());
                }
            }

            // Get items for this row, reversing if RTL
            let row_items: Vec<usize> = if self.right_to_left {
                row.iter().copied().rev().collect()
            } else {
                row.clone()
            };

            // Render items in this row
            for (col_idx, &item_idx) in row_items.iter().enumerate() {
                if let Some(item) = self.items.get(item_idx) {
                    // Left padding
                    if self.padding.0 > 0 {
                        segments.push(Segment::new(" ".repeat(self.padding.0)));
                    }

                    // Content with alignment
                    let content_width = Self::text_width(item);
                    let cell_width = column_width.saturating_sub(pad_h);

                    let (left_pad, right_pad) = match self.align {
                        ColumnAlign::Left => (0, cell_width.saturating_sub(content_width)),
                        ColumnAlign::Right => (cell_width.saturating_sub(content_width), 0),
                        ColumnAlign::Center => {
                            let total_pad = cell_width.saturating_sub(content_width);
                            let left = total_pad.checked_div(2).unwrap_or(0);
                            (left, total_pad.saturating_sub(left))
                        }
                    };

                    if left_pad > 0 {
                        segments.push(Segment::new(" ".repeat(left_pad)));
                    }

                    segments.extend(item.to_segments().into_iter());

                    if right_pad > 0 && col_idx < row_items.len().saturating_sub(1) {
                        segments.push(Segment::new(" ".repeat(right_pad)));
                    }

                    // Right padding
                    if self.padding.2 > 0 && col_idx < row_items.len().saturating_sub(1) {
                        segments.push(Segment::new(" ".repeat(self.padding.2)));
                    }
                }
            }

            // End of row
            segments.push(Segment::newline());

            // Add bottom padding
            for _ in 0..self.padding.3 {
                segments.push(Segment::newline());
            }
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns_new() {
        let items = vec!["a", "b", "c"];
        let columns = Columns::new(items);
        assert_eq!(columns.items.len(), 3);
    }

    #[test]
    fn test_columns_add() {
        let columns = Columns::new(vec!["a", "b"]).add("c");
        assert_eq!(columns.items.len(), 3);
    }

    #[test]
    fn test_columns_equal() {
        let columns = Columns::new(vec!["short", "verylongitem"]).equal(true);
        assert!(columns.equal);
    }

    #[test]
    fn test_columns_render_basic() {
        let items = vec!["Item 1", "Item 2", "Item 3"];
        let columns = Columns::new(items);
        let segments = columns.render(80);
        let output = segments.to_ansi();
        assert!(output.contains("Item 1"));
        assert!(output.contains("Item 2"));
        assert!(output.contains("Item 3"));
    }

    #[test]
    fn test_columns_empty() {
        let columns = Columns::new(Vec::<&str>::new());
        let segments = columns.render(80);
        assert!(segments.to_ansi().is_empty());
    }

    #[test]
    fn test_columns_single_item() {
        let columns = Columns::new(vec!["single"]);
        let segments = columns.render(80);
        assert!(segments.to_ansi().contains("single"));
    }

    #[test]
    fn test_columns_alignment() {
        let columns = Columns::new(vec!["a", "b"])
            .align(ColumnAlign::Center)
            .equal(true);
        assert_eq!(columns.align, ColumnAlign::Center);
    }

    #[test]
    fn test_columns_rtl() {
        let columns = Columns::new(vec!["first", "second"]).right_to_left(true);
        assert!(columns.right_to_left);
    }

    #[test]
    fn test_column_first() {
        let columns = Columns::new(vec!["1", "2", "3", "4"]).column_first(true);
        assert!(columns.column_first);
    }

    #[test]
    fn test_columns_padding() {
        let columns = Columns::new(vec!["a"]).padding(1, 2, 3, 4);
        assert_eq!(columns.padding, (1, 2, 3, 4));
    }

    #[test]
    fn test_columns_width() {
        let columns = Columns::new(vec!["a"]).width(40);
        assert_eq!(columns.width, Some(40));
    }
}
