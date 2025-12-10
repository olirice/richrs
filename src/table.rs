//! Table component for tabular data display.
//!
//! Tables render data in rows and columns with optional borders,
//! headers, and footers.

use crate::box_chars::BoxChars;
use crate::errors::Result;
use crate::measure::{Measurable, MeasureOptions, Measurement, cell_len};
use crate::segment::{Segment, Segments};
use crate::style::Style;
use crate::text::{Justify, Text};
use serde::{Deserialize, Serialize};

/// Vertical alignment options for table cells.
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

/// A table column definition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Column {
    /// Column header text.
    pub header: Option<Text>,
    /// Column footer text.
    pub footer: Option<Text>,
    /// Header style.
    pub header_style: Option<Style>,
    /// Footer style.
    pub footer_style: Option<Style>,
    /// Cell style.
    pub style: Option<Style>,
    /// Horizontal justification.
    pub justify: Justify,
    /// Vertical alignment.
    pub vertical: VerticalAlign,
    /// Fixed width.
    pub width: Option<usize>,
    /// Minimum width.
    pub min_width: Option<usize>,
    /// Maximum width.
    pub max_width: Option<usize>,
    /// Ratio for proportional sizing.
    pub ratio: Option<f32>,
    /// Disable word wrapping.
    pub no_wrap: bool,
    /// Enable highlighting.
    pub highlight: bool,
}

impl Column {
    /// Creates a new column with a header.
    #[inline]
    #[must_use]
    pub fn new(header: impl Into<Text>) -> Self {
        Self {
            header: Some(header.into()),
            ..Self::default()
        }
    }

    /// Creates a column without a header.
    #[inline]
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            header: None,
            footer: None,
            header_style: None,
            footer_style: None,
            style: None,
            justify: Justify::Left,
            vertical: VerticalAlign::Top,
            width: None,
            min_width: None,
            max_width: None,
            ratio: None,
            no_wrap: false,
            highlight: false,
        }
    }

    /// Sets the header style.
    #[inline]
    #[must_use]
    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = Some(style);
        self
    }

    /// Sets the footer.
    #[inline]
    #[must_use]
    pub fn footer(mut self, footer: impl Into<Text>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Sets the footer style.
    #[inline]
    #[must_use]
    pub fn footer_style(mut self, style: Style) -> Self {
        self.footer_style = Some(style);
        self
    }

    /// Sets the cell style.
    #[inline]
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Sets horizontal justification.
    #[inline]
    #[must_use]
    pub const fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    /// Sets vertical alignment.
    #[inline]
    #[must_use]
    pub const fn vertical(mut self, vertical: VerticalAlign) -> Self {
        self.vertical = vertical;
        self
    }

    /// Sets fixed width.
    #[inline]
    #[must_use]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets minimum width.
    #[inline]
    #[must_use]
    pub const fn min_width(mut self, min_width: usize) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// Sets maximum width.
    #[inline]
    #[must_use]
    pub const fn max_width(mut self, max_width: usize) -> Self {
        self.max_width = Some(max_width);
        self
    }

    /// Sets ratio for proportional sizing.
    #[inline]
    #[must_use]
    pub const fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = Some(ratio);
        self
    }

    /// Sets no-wrap mode.
    #[inline]
    #[must_use]
    pub const fn no_wrap(mut self, no_wrap: bool) -> Self {
        self.no_wrap = no_wrap;
        self
    }
}

/// A table row.
#[derive(Debug, Clone, Default)]
pub struct Row {
    /// Cells in this row.
    pub cells: Vec<Text>,
    /// Row style.
    pub style: Option<Style>,
    /// End section after this row.
    pub end_section: bool,
}

impl Row {
    /// Creates a new row with the given cells.
    #[must_use]
    pub fn new<I, T>(cells: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            style: None,
            end_section: false,
        }
    }

    /// Sets the row style.
    #[inline]
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

/// A data table for displaying tabular information.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
#[allow(dead_code)]
pub struct Table {
    /// Column definitions.
    columns: Vec<Column>,
    /// Data rows.
    rows: Vec<Row>,
    /// Box drawing characters.
    box_chars: Option<BoxChars>,
    /// Table title.
    title: Option<Text>,
    /// Table caption.
    caption: Option<Text>,
    /// Title style.
    title_style: Option<Style>,
    /// Caption style.
    caption_style: Option<Style>,
    /// Border style.
    border_style: Option<Style>,
    /// Header style.
    header_style: Option<Style>,
    /// Footer style.
    footer_style: Option<Style>,
    /// Row styles (alternating).
    row_styles: Vec<Style>,
    /// Table style.
    style: Option<Style>,
    /// Title justification.
    title_justify: Justify,
    /// Caption justification.
    caption_justify: Justify,
    /// Whether to show header.
    show_header: bool,
    /// Whether to show footer.
    show_footer: bool,
    /// Whether to show edge borders.
    show_edge: bool,
    /// Whether to show lines between rows.
    show_lines: bool,
    /// Whether to expand to full width.
    expand: bool,
    /// Cell padding.
    padding: (usize, usize),
    /// Collapse cell padding.
    collapse_padding: bool,
    /// Pad edge cells.
    pad_edge: bool,
    /// Leading (space between rows).
    leading: usize,
    /// Minimum width.
    min_width: Option<usize>,
    /// Fixed width.
    width: Option<usize>,
    /// Use ASCII-safe box characters.
    safe_box: bool,
    /// Enable highlighting.
    highlight: bool,
}

impl Table {
    /// Creates a new empty table.
    #[must_use]
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            box_chars: Some(BoxChars::SQUARE),
            title: None,
            caption: None,
            title_style: None,
            caption_style: None,
            border_style: None,
            header_style: None,
            footer_style: None,
            row_styles: Vec::new(),
            style: None,
            title_justify: Justify::Center,
            caption_justify: Justify::Center,
            show_header: true,
            show_footer: true,
            show_edge: true,
            show_lines: false,
            expand: false,
            padding: (1, 1),
            collapse_padding: false,
            pad_edge: true,
            leading: 0,
            min_width: None,
            width: None,
            safe_box: false,
            highlight: false,
        }
    }

    /// Creates a grid (table without borders, for layout).
    #[must_use]
    pub fn grid() -> Self {
        Self {
            box_chars: None,
            show_header: false,
            show_edge: false,
            ..Self::new()
        }
    }

    /// Adds a column to the table.
    #[inline]
    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    /// Adds a column with just a header string.
    #[inline]
    pub fn add_column_str(&mut self, header: &str) {
        self.columns.push(Column::new(header));
    }

    /// Adds a row to the table.
    #[inline]
    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    /// Adds a row from an iterator of cell values.
    pub fn add_row_cells<I, T>(&mut self, cells: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<Text>,
    {
        self.rows.push(Row::new(cells));
    }

    /// Adds a section separator after the current row.
    pub fn add_section(&mut self) {
        if let Some(last) = self.rows.last_mut() {
            last.end_section = true;
        }
    }

    /// Sets the box characters.
    #[inline]
    #[must_use]
    pub fn box_chars(mut self, box_chars: Option<BoxChars>) -> Self {
        self.box_chars = box_chars;
        self
    }

    /// Sets the title.
    #[inline]
    #[must_use]
    pub fn title(mut self, title: impl Into<Text>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the caption.
    #[inline]
    #[must_use]
    pub fn caption(mut self, caption: impl Into<Text>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Sets the border style.
    #[inline]
    #[must_use]
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = Some(style);
        self
    }

    /// Sets the header style.
    #[inline]
    #[must_use]
    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = Some(style);
        self
    }

    /// Sets whether to show the header.
    #[inline]
    #[must_use]
    pub const fn show_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    /// Sets whether to show the footer.
    #[inline]
    #[must_use]
    pub const fn show_footer(mut self, show: bool) -> Self {
        self.show_footer = show;
        self
    }

    /// Sets whether to show edge borders.
    #[inline]
    #[must_use]
    pub const fn show_edge(mut self, show: bool) -> Self {
        self.show_edge = show;
        self
    }

    /// Sets whether to show lines between rows.
    #[inline]
    #[must_use]
    pub const fn show_lines(mut self, show: bool) -> Self {
        self.show_lines = show;
        self
    }

    /// Sets whether to expand to full width.
    #[inline]
    #[must_use]
    pub const fn expand(mut self, expand: bool) -> Self {
        self.expand = expand;
        self
    }

    /// Sets the cell padding.
    #[inline]
    #[must_use]
    pub const fn padding(mut self, horizontal: usize, vertical: usize) -> Self {
        self.padding = (horizontal, vertical);
        self
    }

    /// Sets ASCII-safe mode.
    #[inline]
    #[must_use]
    pub fn safe_box(mut self, safe: bool) -> Self {
        self.safe_box = safe;
        if safe {
            self.box_chars = Some(BoxChars::ASCII);
        }
        self
    }

    /// Sets the width.
    #[inline]
    #[must_use]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the minimum width.
    #[inline]
    #[must_use]
    pub const fn min_width(mut self, min_width: usize) -> Self {
        self.min_width = Some(min_width);
        self
    }

    /// Returns the number of columns.
    #[inline]
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Returns the number of rows.
    #[inline]
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Calculates column widths.
    fn calculate_column_widths(&self, max_width: usize) -> Vec<usize> {
        if self.columns.is_empty() {
            return Vec::new();
        }

        let col_count = self.columns.len();
        let mut widths = vec![0usize; col_count];

        // Calculate minimum widths from content
        for (idx, col) in self.columns.iter().enumerate() {
            // Header width
            if let Some(ref header) = col.header {
                widths[idx] = widths
                    .get(idx)
                    .copied()
                    .unwrap_or(0)
                    .max(cell_len(header.plain()));
            }

            // Fixed width overrides
            if let Some(w) = col.width {
                widths[idx] = w;
            } else if let Some(min_w) = col.min_width {
                widths[idx] = widths.get(idx).copied().unwrap_or(0).max(min_w);
            }
        }

        // Measure row content
        for row in &self.rows {
            for (idx, cell) in row.cells.iter().enumerate() {
                if idx < col_count {
                    let cell_width = cell_len(cell.plain());
                    widths[idx] = widths.get(idx).copied().unwrap_or(0).max(cell_width);
                }
            }
        }

        // Apply maximum widths
        for (idx, col) in self.columns.iter().enumerate() {
            if let Some(max_w) = col.max_width {
                if let Some(w) = widths.get_mut(idx) {
                    *w = (*w).min(max_w);
                }
            }
        }

        // Ensure we don't exceed max_width
        let border_overhead = if self.box_chars.is_some() {
            col_count.saturating_add(1)
        } else {
            0
        };
        let padding_overhead = col_count.saturating_mul(self.padding.0.saturating_mul(2));
        let available = max_width
            .saturating_sub(border_overhead)
            .saturating_sub(padding_overhead);

        let total: usize = widths.iter().sum();
        if total > available && !widths.is_empty() {
            // Scale down proportionally
            for w in &mut widths {
                *w = (*w)
                    .saturating_mul(available)
                    .checked_div(total)
                    .unwrap_or(*w)
                    .max(1);
            }
        }

        widths
    }

    /// Renders the table to segments.
    #[must_use]
    pub fn render(&self, max_width: usize) -> Segments {
        let mut segments = Segments::new();

        if self.columns.is_empty() {
            return segments;
        }

        let widths = self.calculate_column_widths(max_width);
        let box_chars = self.box_chars.as_ref();

        // Render title
        if let Some(ref title) = self.title {
            self.render_title(&mut segments, title, &widths);
        }

        // Render top border
        if let Some(bc) = box_chars {
            if self.show_edge {
                self.render_top_border(&mut segments, bc, &widths);
            }
        }

        // Render header
        if self.show_header {
            self.render_header(&mut segments, &widths, box_chars);
        }

        // Render rows
        for (row_idx, row) in self.rows.iter().enumerate() {
            self.render_row(&mut segments, row, row_idx, &widths, box_chars);

            // Section separator
            if row.end_section && row_idx < self.rows.len().saturating_sub(1) {
                if let Some(bc) = box_chars {
                    self.render_row_separator(&mut segments, bc, &widths);
                }
            }
        }

        // Render bottom border
        if let Some(bc) = box_chars {
            if self.show_edge {
                self.render_bottom_border(&mut segments, bc, &widths);
            }
        }

        // Render caption
        if let Some(ref caption) = self.caption {
            self.render_caption(&mut segments, caption, &widths);
        }

        segments
    }

    /// Renders the title.
    fn render_title(&self, segments: &mut Segments, title: &Text, widths: &[usize]) {
        let total_width = self.calculate_total_width(widths);
        let title_text = title.plain();
        let title_width = cell_len(title_text);
        let padding = total_width.saturating_sub(title_width);
        let left_pad = padding.checked_div(2).unwrap_or(0);
        let right_pad = padding.saturating_sub(left_pad);

        segments.push(Segment::new(" ".repeat(left_pad)));
        segments.extend(title.to_segments());
        segments.push(Segment::new(" ".repeat(right_pad)));
        segments.push(Segment::newline());
    }

    /// Renders the caption.
    fn render_caption(&self, segments: &mut Segments, caption: &Text, widths: &[usize]) {
        let total_width = self.calculate_total_width(widths);
        let caption_text = caption.plain();
        let caption_width = cell_len(caption_text);
        let padding = total_width.saturating_sub(caption_width);
        let left_pad = padding.checked_div(2).unwrap_or(0);
        let right_pad = padding.saturating_sub(left_pad);

        segments.push(Segment::new(" ".repeat(left_pad)));
        segments.extend(caption.to_segments());
        segments.push(Segment::new(" ".repeat(right_pad)));
        segments.push(Segment::newline());
    }

    /// Calculates total table width.
    fn calculate_total_width(&self, widths: &[usize]) -> usize {
        let content: usize = widths.iter().sum();
        let padding = widths
            .len()
            .saturating_mul(self.padding.0.saturating_mul(2));
        let borders = if self.box_chars.is_some() {
            widths.len().saturating_add(1)
        } else {
            0
        };
        content.saturating_add(padding).saturating_add(borders)
    }

    /// Renders the top border.
    fn render_top_border(&self, segments: &mut Segments, bc: &BoxChars, widths: &[usize]) {
        let style = self.border_style.clone();

        let corner = bc.top_left.to_string();
        if let Some(ref s) = style {
            segments.push(Segment::styled(corner, s.clone()));
        } else {
            segments.push(Segment::new(corner));
        }

        for (idx, width) in widths.iter().enumerate() {
            let line_width = width.saturating_add(self.padding.0.saturating_mul(2));
            let line = bc.horizontal.to_string().repeat(line_width);
            if let Some(ref s) = style {
                segments.push(Segment::styled(line, s.clone()));
            } else {
                segments.push(Segment::new(line));
            }

            if idx < widths.len().saturating_sub(1) {
                let junction = bc.horizontal_down.to_string();
                if let Some(ref s) = style {
                    segments.push(Segment::styled(junction, s.clone()));
                } else {
                    segments.push(Segment::new(junction));
                }
            }
        }

        let corner = bc.top_right.to_string();
        if let Some(ref s) = style {
            segments.push(Segment::styled(corner, s.clone()));
        } else {
            segments.push(Segment::new(corner));
        }

        segments.push(Segment::newline());
    }

    /// Renders the bottom border.
    fn render_bottom_border(&self, segments: &mut Segments, bc: &BoxChars, widths: &[usize]) {
        let style = self.border_style.clone();

        let corner = bc.bottom_left.to_string();
        if let Some(ref s) = style {
            segments.push(Segment::styled(corner, s.clone()));
        } else {
            segments.push(Segment::new(corner));
        }

        for (idx, width) in widths.iter().enumerate() {
            let line_width = width.saturating_add(self.padding.0.saturating_mul(2));
            let line = bc.horizontal.to_string().repeat(line_width);
            if let Some(ref s) = style {
                segments.push(Segment::styled(line, s.clone()));
            } else {
                segments.push(Segment::new(line));
            }

            if idx < widths.len().saturating_sub(1) {
                let junction = bc.horizontal_up.to_string();
                if let Some(ref s) = style {
                    segments.push(Segment::styled(junction, s.clone()));
                } else {
                    segments.push(Segment::new(junction));
                }
            }
        }

        let corner = bc.bottom_right.to_string();
        if let Some(ref s) = style {
            segments.push(Segment::styled(corner, s.clone()));
        } else {
            segments.push(Segment::new(corner));
        }

        segments.push(Segment::newline());
    }

    /// Renders a row separator.
    fn render_row_separator(&self, segments: &mut Segments, bc: &BoxChars, widths: &[usize]) {
        let style = self.border_style.clone();

        let junction = bc.vertical_right.to_string();
        if let Some(ref s) = style {
            segments.push(Segment::styled(junction, s.clone()));
        } else {
            segments.push(Segment::new(junction));
        }

        for (idx, width) in widths.iter().enumerate() {
            let line_width = width.saturating_add(self.padding.0.saturating_mul(2));
            let line = bc.horizontal.to_string().repeat(line_width);
            if let Some(ref s) = style {
                segments.push(Segment::styled(line, s.clone()));
            } else {
                segments.push(Segment::new(line));
            }

            if idx < widths.len().saturating_sub(1) {
                let junction = bc.cross.to_string();
                if let Some(ref s) = style {
                    segments.push(Segment::styled(junction, s.clone()));
                } else {
                    segments.push(Segment::new(junction));
                }
            }
        }

        let junction = bc.vertical_left.to_string();
        if let Some(ref s) = style {
            segments.push(Segment::styled(junction, s.clone()));
        } else {
            segments.push(Segment::new(junction));
        }

        segments.push(Segment::newline());
    }

    /// Renders the header row.
    fn render_header(
        &self,
        segments: &mut Segments,
        widths: &[usize],
        box_chars: Option<&BoxChars>,
    ) {
        let border_style = self.border_style.clone();
        let header_style = self.header_style.clone();

        // Left border
        if let Some(bc) = box_chars {
            let vertical = bc.vertical.to_string();
            if let Some(ref s) = border_style {
                segments.push(Segment::styled(vertical, s.clone()));
            } else {
                segments.push(Segment::new(vertical));
            }
        }

        for (idx, col) in self.columns.iter().enumerate() {
            let width = widths.get(idx).copied().unwrap_or(0);

            // Left padding
            segments.push(Segment::new(" ".repeat(self.padding.0)));

            // Header content
            if let Some(ref header) = col.header {
                let header_text = header.plain();
                let header_len = cell_len(header_text);
                let remaining = width.saturating_sub(header_len);

                let header_segs = header.to_segments();
                let style = col.header_style.clone().or_else(|| header_style.clone());

                if let Some(s) = style {
                    for seg in header_segs.iter() {
                        let combined = seg
                            .style
                            .clone()
                            .map(|ss| ss.combine(&s))
                            .unwrap_or_else(|| s.clone());
                        segments.push(Segment::styled(seg.text.clone(), combined));
                    }
                } else {
                    segments.extend(header_segs);
                }

                segments.push(Segment::new(" ".repeat(remaining)));
            } else {
                segments.push(Segment::new(" ".repeat(width)));
            }

            // Right padding
            segments.push(Segment::new(" ".repeat(self.padding.0)));

            // Column separator
            if let Some(bc) = box_chars {
                if idx < widths.len().saturating_sub(1) {
                    let vertical = bc.vertical.to_string();
                    if let Some(ref s) = border_style {
                        segments.push(Segment::styled(vertical, s.clone()));
                    } else {
                        segments.push(Segment::new(vertical));
                    }
                }
            }
        }

        // Right border
        if let Some(bc) = box_chars {
            let vertical = bc.vertical.to_string();
            if let Some(ref s) = border_style {
                segments.push(Segment::styled(vertical, s.clone()));
            } else {
                segments.push(Segment::new(vertical));
            }
        }

        segments.push(Segment::newline());

        // Header separator
        if let Some(bc) = box_chars {
            self.render_row_separator(segments, bc, widths);
        }
    }

    /// Renders a data row.
    fn render_row(
        &self,
        segments: &mut Segments,
        row: &Row,
        _row_idx: usize,
        widths: &[usize],
        box_chars: Option<&BoxChars>,
    ) {
        let border_style = self.border_style.clone();

        // Left border
        if let Some(bc) = box_chars {
            let vertical = bc.vertical.to_string();
            if let Some(ref s) = border_style {
                segments.push(Segment::styled(vertical, s.clone()));
            } else {
                segments.push(Segment::new(vertical));
            }
        }

        for (idx, width) in widths.iter().enumerate() {
            // Left padding
            segments.push(Segment::new(" ".repeat(self.padding.0)));

            // Cell content
            if let Some(cell) = row.cells.get(idx) {
                let cell_text = cell.plain();
                let cell_len = cell_len(cell_text);
                let remaining = width.saturating_sub(cell_len);

                let cell_segs = cell.to_segments();
                let style = row
                    .style
                    .clone()
                    .or_else(|| self.columns.get(idx).and_then(|c| c.style.clone()));

                if let Some(s) = style {
                    for seg in cell_segs.iter() {
                        let combined = seg
                            .style
                            .clone()
                            .map(|ss| ss.combine(&s))
                            .unwrap_or_else(|| s.clone());
                        segments.push(Segment::styled(seg.text.clone(), combined));
                    }
                } else {
                    segments.extend(cell_segs);
                }

                segments.push(Segment::new(" ".repeat(remaining)));
            } else {
                segments.push(Segment::new(" ".repeat(*width)));
            }

            // Right padding
            segments.push(Segment::new(" ".repeat(self.padding.0)));

            // Column separator
            if let Some(bc) = box_chars {
                if idx < widths.len().saturating_sub(1) {
                    let vertical = bc.vertical.to_string();
                    if let Some(ref s) = border_style {
                        segments.push(Segment::styled(vertical, s.clone()));
                    } else {
                        segments.push(Segment::new(vertical));
                    }
                }
            }
        }

        // Right border
        if let Some(bc) = box_chars {
            let vertical = bc.vertical.to_string();
            if let Some(ref s) = border_style {
                segments.push(Segment::styled(vertical, s.clone()));
            } else {
                segments.push(Segment::new(vertical));
            }
        }

        segments.push(Segment::newline());
    }
}

impl Measurable for Table {
    fn measure(&self, options: &MeasureOptions) -> Result<Measurement> {
        let widths = self.calculate_column_widths(options.max_width);
        let total = self.calculate_total_width(&widths);
        Ok(Measurement::fixed(total).clamp_max(options.max_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_new() {
        let table = Table::new();
        assert_eq!(table.column_count(), 0);
        assert_eq!(table.row_count(), 0);
    }

    #[test]
    fn test_table_add_column() {
        let mut table = Table::new();
        table.add_column(Column::new("Name"));
        table.add_column(Column::new("Value"));
        assert_eq!(table.column_count(), 2);
    }

    #[test]
    fn test_table_add_row() {
        let mut table = Table::new();
        table.add_column(Column::new("Name"));
        table.add_column(Column::new("Value"));
        table.add_row_cells(["Alice", "100"]);
        table.add_row_cells(["Bob", "200"]);
        assert_eq!(table.row_count(), 2);
    }

    #[test]
    fn test_table_render() {
        let mut table = Table::new();
        table.add_column(Column::new("Name"));
        table.add_column(Column::new("Value"));
        table.add_row_cells(["Alice", "100"]);

        let segments = table.render(80);
        let text = segments.plain_text();
        assert!(text.contains("Name"));
        assert!(text.contains("Value"));
        assert!(text.contains("Alice"));
        assert!(text.contains("100"));
    }

    #[test]
    fn test_table_grid() {
        let table = Table::grid();
        assert!(!table.show_header);
        assert!(!table.show_edge);
        assert!(table.box_chars.is_none());
    }

    #[test]
    fn test_column_builder() {
        let col = Column::new("Test")
            .justify(Justify::Right)
            .width(20)
            .no_wrap(true);

        assert!(col.header.is_some());
        assert_eq!(col.justify, Justify::Right);
        assert_eq!(col.width, Some(20));
        assert!(col.no_wrap);
    }

    #[test]
    fn test_row_builder() {
        let style = Style::new().bold();
        let row = Row::new(["a", "b", "c"]).style(style.clone());
        assert_eq!(row.cells.len(), 3);
        assert!(row.style.is_some());
    }

    #[test]
    fn test_table_safe_box() {
        let table = Table::new().safe_box(true);
        assert!(table.safe_box);
        assert_eq!(table.box_chars, Some(BoxChars::ASCII));
    }

    #[test]
    fn test_table_measure() {
        let mut table = Table::new();
        table.add_column(Column::new("Name"));
        table.add_row_cells(["Test"]);

        let options = MeasureOptions::new(80);
        let measurement = table.measure(&options).ok().unwrap_or_default();
        assert!(measurement.minimum > 0);
    }
}
